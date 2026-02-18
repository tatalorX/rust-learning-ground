#!/usr/bin/env python3
"""
Rust Learning Ground - SECURE Main Runner Script
================================================
This is a hardened version with security fixes for:
- Path traversal vulnerabilities (CVE-style)
- Code execution sandboxing (Firejail/Docker)
- Authentication requirements
- Input validation
- CORS restrictions
- Security headers

SECURITY WARNING: Even with these fixes, running untrusted code
is inherently dangerous. For production with untrusted users,
use isolated containers or VMs.
"""

import os
import sys
import json
import subprocess
import http.server
import socketserver
import threading
import platform

# Cross-platform file locking
try:
    import fcntl

    HAS_FCNTL = True
except ImportError:
    HAS_FCNTL = False
    fcntl = None  # type: ignore
import tempfile
import shutil
import re
import hmac
import hashlib
import secrets
import time
from pathlib import Path
from urllib.parse import urlparse, parse_qs
from typing import Optional, Tuple, Dict, Any

# =============================================================================
# SECURITY CONFIGURATION
# =============================================================================

ROOT = Path(__file__).parent.resolve()
PORT = int(os.getenv("RUST_LEARNING_PORT", "54321"))
PROFILE_PATH = ROOT / "profile" / "user_stats.json"
PROBLEMS_DIR = ROOT / "problems"
TOTAL_PROBLEMS = 320

# Security settings - THESE MUST BE SET IN PRODUCTION
MAX_CODE_SIZE = 100_000  # 100KB max code size
ALLOWED_ORIGINS = os.getenv(
    "ALLOWED_ORIGINS",
    "http://localhost:3000,http://localhost:54321,http://127.0.0.1:3000,http://127.0.0.1:54321",
).split(",")
AUTH_TOKEN = os.getenv("RUST_LEARNING_AUTH_TOKEN")
SANDBOX_MODE = os.getenv("SANDBOX_MODE", "auto")  # 'auto', 'firejail', 'docker', 'none'

# In production, require authentication
REQUIRE_AUTH = os.getenv("REQUIRE_AUTH", "false").lower() == "true"

# Rate limiting storage
_rate_limit_store: Dict[str, list] = {}
_rate_limit_lock = threading.Lock()
_rate_limit_last_cleanup = 0.0
RATE_LIMIT_CLEANUP_INTERVAL = 300  # Cleanup every 5 minutes


def _cleanup_rate_limit_store(now: float, window: int):
    """Remove old entries from rate limit store to prevent memory leak."""
    global _rate_limit_last_cleanup
    # Only cleanup periodically to avoid performance impact
    if now - _rate_limit_last_cleanup < RATE_LIMIT_CLEANUP_INTERVAL:
        return

    # Remove keys with empty lists (all entries expired)
    keys_to_remove = []
    for key, timestamps in _rate_limit_store.items():
        # Filter out old timestamps
        valid_timestamps = [t for t in timestamps if now - t < window]
        if valid_timestamps:
            _rate_limit_store[key] = valid_timestamps
        else:
            keys_to_remove.append(key)

    for key in keys_to_remove:
        del _rate_limit_store[key]

    _rate_limit_last_cleanup = now


# Dangerous patterns in Rust code
DANGEROUS_PATTERNS = [
    (r"process::Command", "Process execution"),
    (r"std::process::", "Process execution"),
    (r"Command::new", "Process execution"),
    (r"fs::remove", "File deletion"),
    (r"fs::rename", "File rename"),
    (r"std::fs::remove", "File deletion"),
    (r"unsafe\s*\{", "Unsafe block"),
    (r'include_str!\s*\(\s*["\']/(etc|proc|sys|home|root|var)', "File inclusion"),
    (r'include_bytes!\s*\(\s*["\']/(etc|proc|sys|home|root|var)', "File inclusion"),
    (r"net::Tcp", "Network access"),
    (r"net::Udp", "Network access"),
    (r"httpx?", "HTTP client"),
    (r"reqwest", "HTTP client"),
]

# Valid problem IDs
VALID_PIDS = set(range(1, TOTAL_PROBLEMS + 1))


# =============================================================================
# SECURITY UTILITIES
# =============================================================================


class SecurityError(Exception):
    """Security violation detected."""

    pass


def check_rate_limit(
    client_ip: str, endpoint: str, max_requests: int = 10, window: int = 60
) -> bool:
    """Check if client has exceeded rate limit."""
    key = f"{client_ip}:{endpoint}"
    now = time.time()

    with _rate_limit_lock:
        # Periodic cleanup to prevent memory leak
        _cleanup_rate_limit_store(now, window)

        if key not in _rate_limit_store:
            _rate_limit_store[key] = []

        # Remove old entries outside the window
        _rate_limit_store[key] = [t for t in _rate_limit_store[key] if now - t < window]

        # Check if limit exceeded
        if len(_rate_limit_store[key]) >= max_requests:
            return False

        # Add current request
        _rate_limit_store[key].append(now)
        return True


def verify_auth_token(handler) -> bool:
    """Verify authentication token from request."""
    if not REQUIRE_AUTH and not AUTH_TOKEN:
        return True  # Dev mode - no auth required

    if not AUTH_TOKEN:
        # Auth required but not configured
        return False

    # Check Authorization header
    auth_header = handler.headers.get("Authorization", "")
    if auth_header.startswith("Bearer "):
        token = auth_header[7:].strip()
        return hmac.compare_digest(token, AUTH_TOKEN)

    # Check cookie
    cookie = handler.headers.get("Cookie", "")
    for part in cookie.split(";"):
        part = part.strip()
        if part.startswith("auth_token="):
            token = part.split("=", 1)[1]
            return hmac.compare_digest(token, AUTH_TOKEN)

    return False


def validate_pid(pid: int) -> int:
    """Validate problem ID is in allowed range."""
    if pid not in VALID_PIDS:
        raise SecurityError(f"Invalid problem ID: {pid}")
    return pid


def sanitize_path(folder: Path) -> Path:
    """Ensure path is within allowed directory."""
    root = PROBLEMS_DIR.resolve()
    try:
        target = folder.resolve()
        # Ensure the path starts with the root path
        root_str = str(root)
        target_str = str(target)
        if not target_str.startswith(root_str):
            raise SecurityError(
                f"Path traversal detected: {target_str} not in {root_str}"
            )
        return target
    except (OSError, ValueError) as e:
        raise SecurityError(f"Invalid path: {e}")


def validate_code_content(code: str) -> Tuple[bool, str]:
    """Validate code for dangerous patterns."""
    if len(code) > MAX_CODE_SIZE:
        return False, f"Code too large: {len(code)} > {MAX_CODE_SIZE} bytes"

    # Check for null bytes
    if "\x00" in code:
        return False, "Null bytes not allowed"

    for pattern, description in DANGEROUS_PATTERNS:
        if re.search(pattern, code, re.IGNORECASE):
            return False, f"Potentially unsafe code pattern detected: {description}"

    return True, "OK"


def get_client_ip(handler) -> str:
    """Get client IP address safely."""
    # Check for forwarded header (when behind proxy)
    forwarded = handler.headers.get("X-Forwarded-For")
    if forwarded:
        # Take the first IP in the chain
        return forwarded.split(",")[0].strip()[:45]  # IPv6 max length

    if handler.client_address:
        return handler.client_address[0][:45]
    return "unknown"


def get_safe_problem_folder(pid: int) -> Optional[Path]:
    """Get validated problem folder."""
    try:
        pid = validate_pid(pid)
        # Only accept specific patterns
        folder = PROBLEMS_DIR / f"{pid:03d}_exercise"
        if not folder.exists():
            folder = PROBLEMS_DIR / f"{pid:03d}_project"

        if not folder.exists():
            return None

        return sanitize_path(folder)
    except SecurityError:
        return None


def escape_json_string(s: str) -> str:
    """Escape string for safe JSON output."""
    return json.dumps(s)


def _truncate_unicode(s: str, max_bytes: int) -> str:
    """Truncate string to fit within max_bytes when UTF-8 encoded.

    This properly handles multibyte characters by truncating at character
    boundaries to avoid breaking UTF-8 sequences.
    """
    if len(s.encode("utf-8")) <= max_bytes:
        return s

    # Binary search for the right truncation point
    left, right = 0, len(s)
    while left < right:
        mid = (left + right + 1) // 2
        if len(s[:mid].encode("utf-8")) <= max_bytes:
            left = mid
        else:
            right = mid - 1

    return s[:left]


# =============================================================================
# SANDBOXED CODE EXECUTION
# =============================================================================


class CodeSandbox:
    """Sandboxed code execution environment."""

    @staticmethod
    def check_sandbox_available() -> str:
        """Check which sandbox is available."""
        if SANDBOX_MODE != "auto":
            return SANDBOX_MODE

        # Check for firejail
        try:
            result = subprocess.run(
                ["which", "firejail"], capture_output=True, timeout=5
            )
            if result.returncode == 0:
                return "firejail"
        except:
            pass

        # Check for docker
        try:
            result = subprocess.run(["which", "docker"], capture_output=True, timeout=5)
            if result.returncode == 0:
                # Check if we can run docker
                result = subprocess.run(
                    ["docker", "info"], capture_output=True, timeout=5
                )
                if result.returncode == 0:
                    return "docker"
        except:
            pass

        return "none"

    @staticmethod
    def run_with_firejail(src: Path, timeout: int = 10) -> Tuple[bool, str]:
        """Run code with firejail sandbox."""
        tmpdir = tempfile.mkdtemp(prefix="rust_sandbox_")
        try:
            temp_src = Path(tmpdir) / "main.rs"
            temp_exe = Path(tmpdir) / "output"

            # Copy source
            shutil.copy2(src, temp_src)

            # Compile with restrictions
            compile_cmd = [
                "firejail",
                "--quiet",
                "--noprofile",
                "--net=none",  # No network access
                "--private-tmp",  # Private /tmp
                "--private-dev",  # Private /dev
                "--seccomp",  # System call filtering
                "--rlimit-cpu=30",  # CPU seconds
                "--rlimit-as=268435456",  # 256MB RAM
                "--rlimit-fsize=10485760",  # 10MB file size
                "--rlimit-nofile=64",  # Max 64 file descriptors
                "--rlimit-nproc=10",  # Max 10 processes
                "rustc",
                "-O",
                str(temp_src),
                "-o",
                str(temp_exe),
            ]

            compile_result = subprocess.run(
                compile_cmd, capture_output=True, text=True, timeout=60
            )

            if compile_result.returncode != 0:
                return False, f"Compilation failed:\n{compile_result.stderr}"

            # Run
            run_cmd = [
                "firejail",
                "--quiet",
                "--noprofile",
                "--net=none",
                "--private-tmp",
                "--private-dev",
                "--seccomp",
                "--rlimit-cpu=10",
                "--rlimit-as=134217728",  # 128MB
                "--rlimit-fsize=1048576",  # 1MB output
                "--rlimit-nofile=32",
                "--rlimit-nproc=5",
                str(temp_exe),
            ]

            run_result = subprocess.run(
                run_cmd, capture_output=True, text=True, timeout=timeout
            )

            output = run_result.stdout
            if run_result.stderr:
                output += "\n" + run_result.stderr

            return run_result.returncode == 0, output

        finally:
            shutil.rmtree(tmpdir, ignore_errors=True)

    @staticmethod
    def run_with_docker(src: Path, timeout: int = 10) -> Tuple[bool, str]:
        """Run code with Docker sandbox."""
        tmpdir = tempfile.mkdtemp(prefix="rust_sandbox_")
        container_name = f"rust_sandbox_{secrets.token_hex(8)}"
        image_built = False

        try:
            temp_src = Path(tmpdir) / "main.rs"
            temp_exe = Path(tmpdir) / "output"
            shutil.copy2(src, temp_src)

            # Create Dockerfile
            dockerfile = Path(tmpdir) / "Dockerfile"
            dockerfile.write_text("""
FROM rust:1.75-slim
WORKDIR /sandbox
COPY main.rs .
RUN rustc -O main.rs -o output
CMD ["./output"]
""")

            # Build and run
            build_result = subprocess.run(
                ["docker", "build", "-q", "-t", container_name, tmpdir],
                capture_output=True,
                text=True,
                timeout=120,
            )

            if build_result.returncode != 0:
                return False, f"Docker build failed:\n{build_result.stderr}"

            image_built = True

            run_result = subprocess.run(
                [
                    "docker",
                    "run",
                    "--rm",
                    "--network",
                    "none",
                    "--memory",
                    "128m",
                    "--cpus",
                    "0.5",
                    "--timeout",
                    str(timeout),
                    container_name,
                ],
                capture_output=True,
                text=True,
                timeout=timeout + 5,
            )

            output = run_result.stdout
            if run_result.stderr:
                output += "\n" + run_result.stderr

            return run_result.returncode == 0, output

        except Exception as e:
            return False, f"Docker execution error: {e}"
        finally:
            # Ensure cleanup always runs even if exceptions occur
            shutil.rmtree(tmpdir, ignore_errors=True)
            # Cleanup image if it was built
            if image_built:
                subprocess.run(
                    ["docker", "rmi", "-f", container_name],
                    capture_output=True,
                    timeout=30,
                )
            # Cleanup container if still exists
            subprocess.run(
                ["docker", "rm", "-f", container_name], capture_output=True, timeout=10
            )

    @staticmethod
    def run_basic(src: Path, exe: Path, timeout: int = 5) -> Tuple[bool, str]:
        """Basic execution WITHOUT sandbox (fallback - less secure)."""
        # Clean up any existing binary
        if exe.exists():
            exe.unlink()

        # Compile
        compile_result = subprocess.run(
            ["rustc", "-O", str(src), "-o", str(exe)],
            capture_output=True,
            text=True,
            timeout=30,
        )

        if compile_result.returncode != 0:
            return False, f"Compilation failed:\n{compile_result.stderr}"

        if not exe.exists():
            return False, "Binary not created"

        # Run with timeout
        try:
            run_result = subprocess.run(
                [str(exe)], capture_output=True, text=True, timeout=timeout
            )

            # Clean up
            exe.unlink()

            output = run_result.stdout
            if run_result.stderr:
                output += "\n" + run_result.stderr

            return run_result.returncode == 0, output

        except subprocess.TimeoutExpired:
            exe.unlink() if exe.exists() else None
            return False, "Program timed out (infinite loop?)"
        except Exception as e:
            exe.unlink() if exe.exists() else None
            return False, f"Runtime error: {e}"


# =============================================================================
# PROFILE MANAGEMENT
# =============================================================================


class CrossPlatformFileLock:
    """Cross-platform file locking using fcntl on Unix, simple lock file on Windows."""

    def __init__(self, lock_path: Path):
        self.lock_path = lock_path
        self._fd: Optional[int] = None

    def acquire_shared(self):
        """Acquire shared (read) lock."""
        if HAS_FCNTL:
            self._fd = os.open(str(self.lock_path), os.O_CREAT | os.O_RDWR)
            fcntl.flock(self._fd, fcntl.LOCK_SH)
        # On Windows without fcntl, rely on thread lock only

    def acquire_exclusive(self):
        """Acquire exclusive (write) lock."""
        if HAS_FCNTL:
            self._fd = os.open(str(self.lock_path), os.O_CREAT | os.O_RDWR)
            fcntl.flock(self._fd, fcntl.LOCK_EX)
        # On Windows without fcntl, rely on thread lock only

    def release(self):
        """Release the lock."""
        if HAS_FCNTL and self._fd is not None:
            fcntl.flock(self._fd, fcntl.LOCK_UN)
            os.close(self._fd)
            self._fd = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.release()
        return False


class ProfileManager:
    """Thread-safe profile manager with file locking."""

    def __init__(self, path: Path):
        self.path = path
        self._lock = threading.Lock()
        self._lock_file = path.parent / (path.name + ".lock")
        self._ensure_exists()

    def _ensure_exists(self):
        """Ensure the profile file exists with default structure."""
        if not self.path.exists():
            self.path.parent.mkdir(parents=True, exist_ok=True)
            self.save({"xp": 0, "solved": []})

    def load(self) -> dict:
        """Load profile with file locking for thread safety."""
        try:
            with self._lock:  # Thread-level lock
                with CrossPlatformFileLock(self._lock_file) as file_lock:
                    file_lock.acquire_shared()
                    with open(self.path, "r") as f:
                        data = json.load(f)
                    return data
        except (json.JSONDecodeError, FileNotFoundError):
            return {"xp": 0, "solved": []}

    def save(self, data: dict):
        """Save profile with file locking for thread safety."""
        with self._lock:  # Thread-level lock
            with CrossPlatformFileLock(self._lock_file) as file_lock:
                file_lock.acquire_exclusive()
                with open(self.path, "w") as f:
                    json.dump(data, f, indent=4)

    def mark_solved(self, pid: int) -> bool:
        """Mark a problem as solved. Returns True if it was newly solved."""
        data = self.load()
        if pid not in data["solved"]:
            data["solved"].append(pid)
            data["solved"].sort()
            data["xp"] += 10
            self.save(data)
            return True
        return False

    def reset(self):
        """Reset all progress."""
        self.save({"xp": 0, "solved": []})


profile = ProfileManager(PROFILE_PATH)


# =============================================================================
# SECURE HTTP HANDLER
# =============================================================================


class SecureHandler(http.server.SimpleHTTPRequestHandler):
    """Secure HTTP request handler with authentication and validation."""

    def log_message(self, format, *args):
        """Suppress default logging for security."""
        pass

    def send_security_headers(self):
        """Add security headers to all responses."""
        self.send_header("X-Content-Type-Options", "nosniff")
        self.send_header("X-Frame-Options", "DENY")
        self.send_header("X-XSS-Protection", "1; mode=block")
        self.send_header("Referrer-Policy", "strict-origin-when-cross-origin")
        self.send_header(
            "Content-Security-Policy",
            "default-src 'self'; "
            "script-src 'self' 'unsafe-inline' https://cdn.jsdelivr.net; "
            "style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; "
            "font-src 'self' https://fonts.gstatic.com; "
            "img-src 'self' data: https://avatars.githubusercontent.com; "
            "connect-src 'self' http://localhost:8000 http://127.0.0.1:8000 https://*.ngrok-free.app https://*.ngrok.io ws: wss:;",
        )

    def send_cors_headers(self):
        """Add CORS headers for allowed origins only."""
        origin = self.headers.get("Origin", "")

        # Check if origin is allowed
        allowed = False
        if origin in ALLOWED_ORIGINS:
            allowed = True
        else:
            # Check for ngrok patterns
            import re

            ngrok_pattern = re.compile(r"https://[a-z0-9-]+\.ngrok-(free\.app|io)")
            if ngrok_pattern.match(origin):
                allowed = True
            # Allow localhost origins in development
            if origin.startswith("http://localhost:") or origin.startswith(
                "http://127.0.0.1:"
            ):
                allowed = True

        if allowed:
            # SECURITY FIX: Sanitize origin to prevent HTTP response splitting
            # Only allow valid URL characters
            import re

            sanitized_origin = re.sub(r"[^\w\-\.:/]", "", origin)
            self.send_header("Access-Control-Allow-Origin", sanitized_origin)
            self.send_header("Access-Control-Allow-Credentials", "true")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type, Authorization")

    def verify_authentication(self) -> bool:
        """Verify request is authenticated."""
        return verify_auth_token(self)

    def send_error_json(self, code: int, message: str):
        """Send JSON error response."""
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_security_headers()
        self.end_headers()
        self.wfile.write(json.dumps({"success": False, "error": message}).encode())

    def do_OPTIONS(self):
        """Handle OPTIONS requests for CORS."""
        self.send_response(200)
        self.send_cors_headers()
        self.send_security_headers()
        self.end_headers()

    def do_GET(self):
        """Handle GET requests securely."""
        client_ip = get_client_ip(self)

        # Verify authentication for API endpoints
        if self.path.startswith("/api/"):
            # Config endpoint doesn't require auth
            if self.path != "/api/config" and not self.verify_authentication():
                self.send_error_json(401, "Authentication required")
                return

            # Rate limiting for sensitive endpoints
            if self.path.startswith("/api/run/"):
                if not check_rate_limit(client_ip, "run", max_requests=5, window=60):
                    self.send_error_json(429, "Rate limit exceeded")
                    return

        try:
            if self.path == "/api/config":
                self._handle_api_config()
            elif self.path == "/api/stats":
                self._handle_get_stats()
            elif self.path.startswith("/api/code/"):
                self._handle_get_code()
            elif self.path.startswith("/api/run/"):
                self._handle_run()
            elif self.path.startswith("/api/open/"):
                self._handle_open()
            elif self.path == "/exercise_data.json":
                self._handle_exercise_data()
            else:
                super().do_GET()
        except SecurityError as e:
            self.send_error_json(403, str(e))
        except Exception as e:
            # Log error but don't expose details
            print(f"Error handling GET {self.path}: {e}")
            self.send_error_json(500, "Internal server error")

    def do_POST(self):
        """Handle POST requests securely."""
        client_ip = get_client_ip(self)

        # All POST endpoints require authentication
        if not self.verify_authentication():
            self.send_error_json(401, "Authentication required")
            return

        # Rate limiting for POST
        if not check_rate_limit(client_ip, "post", max_requests=30, window=60):
            self.send_error_json(429, "Rate limit exceeded")
            return

        try:
            if self.path.startswith("/api/code/"):
                self._handle_save_code()
            else:
                self.send_error_json(404, "Not found")
        except SecurityError as e:
            self.send_error_json(403, str(e))
        except Exception as e:
            print(f"Error handling POST {self.path}: {e}")
            self.send_error_json(500, "Internal server error")

    def _handle_api_config(self):
        """Serve API configuration for frontend."""
        # Get API base URL from environment or use default
        import os

        api_base = os.getenv("API_BASE_URL", "http://localhost:8000")

        config = {
            "api_base": api_base,
            "version": "3.0",
            "features": {"auth": True, "github_oauth": False, "sandbox": True},
        }

        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Access-Control-Allow-Origin", "*")  # Public endpoint
        self.send_header("Access-Control-Allow-Methods", "GET, OPTIONS")
        self.send_security_headers()
        self.end_headers()
        self.wfile.write(json.dumps(config).encode())

    def _handle_get_stats(self):
        """Get user stats."""
        data = profile.load()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_cors_headers()
        self.send_security_headers()
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())

    def _handle_get_code(self):
        """Get code for a problem (validated)."""
        parts = self.path.split("/")
        if len(parts) < 4:
            self.send_error_json(400, "Invalid request")
            return

        try:
            pid = int(parts[3])
            folder = get_safe_problem_folder(pid)
            if not folder:
                self.send_error_json(404, "Problem not found")
                return

            # Determine target file
            if (folder / "Cargo.toml").exists():
                target_file = folder / "src" / "main.rs"
            else:
                target_file = folder / "template.rs"

            if not target_file.exists():
                self.send_error_json(404, "File not found")
                return

            with open(target_file, "r") as f:
                code = f.read()

            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_cors_headers()
            self.send_security_headers()
            self.end_headers()
            self.wfile.write(
                json.dumps(
                    {
                        "code": code,
                        "file_path": str(target_file.relative_to(ROOT)),
                        "is_cargo": (folder / "Cargo.toml").exists(),
                    }
                ).encode()
            )

        except ValueError:
            self.send_error_json(400, "Invalid problem ID")

    def _handle_run(self):
        """Run a problem with sandboxing."""
        parts = self.path.split("/")
        if len(parts) < 4:
            self.send_error_json(400, "Invalid request")
            return

        try:
            pid = int(parts[3])
            folder = get_safe_problem_folder(pid)
            if not folder:
                self.send_error_json(404, "Problem not found")
                return

            is_cargo = (folder / "Cargo.toml").exists()

            if is_cargo:
                # For cargo projects, run tests with limits
                result = subprocess.run(
                    ["cargo", "test", "--quiet"],
                    cwd=folder,
                    capture_output=True,
                    text=True,
                    timeout=60,
                )
                success = result.returncode == 0
                output = result.stdout + result.stderr
            else:
                # For exercises, use sandbox
                src = folder / "template.rs"
                exe = folder / "runner_bin"

                if not src.exists():
                    self.send_error_json(404, "Source file not found")
                    return

                # First validate the code
                with open(src, "r") as f:
                    code = f.read()

                is_valid, error_msg = validate_code_content(code)
                if not is_valid:
                    self.send_error_json(400, f"Code validation failed: {error_msg}")
                    return

                sandbox = CodeSandbox()
                sandbox_type = sandbox.check_sandbox_available()

                if sandbox_type == "firejail":
                    success, output = sandbox.run_with_firejail(src)
                elif sandbox_type == "docker":
                    success, output = sandbox.run_with_docker(src)
                else:
                    # Fallback to basic (warn user)
                    print(
                        "WARNING: Running without sandbox! Install firejail for better security."
                    )
                    success, output = sandbox.run_basic(src, exe)

            if success:
                is_new = profile.mark_solved(pid)
            else:
                is_new = False

            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_cors_headers()
            self.send_security_headers()
            self.end_headers()
            self.wfile.write(
                json.dumps(
                    {
                        "success": success,
                        "output": _truncate_unicode(
                            output, 10000
                        ),  # Limit output size safely
                        "newly_solved": success and is_new,
                    }
                ).encode()
            )

        except ValueError:
            self.send_error_json(400, "Invalid problem ID")
        except subprocess.TimeoutExpired:
            self.send_error_json(500, "Execution timed out")

    def _handle_save_code(self):
        """Save code with validation."""
        parts = self.path.split("/")
        if len(parts) < 4:
            self.send_error_json(400, "Invalid request")
            return

        try:
            pid = int(parts[3])
            folder = get_safe_problem_folder(pid)
            if not folder:
                self.send_error_json(404, "Problem not found")
                return

            # Read and validate request body
            content_length_str = self.headers.get("Content-Length", "0")
            try:
                content_length = int(content_length_str)
            except ValueError:
                self.send_error_json(400, "Invalid Content-Length")
                return

            if content_length > MAX_CODE_SIZE:
                self.send_error_json(413, "Code too large")
                return

            post_data = self.rfile.read(content_length)

            try:
                data = json.loads(post_data.decode("utf-8"))
            except json.JSONDecodeError:
                self.send_error_json(400, "Invalid JSON")
                return

            code = data.get("code", "")
            if not isinstance(code, str):
                self.send_error_json(400, "Code must be a string")
                return

            # Validate code content
            is_valid, error_msg = validate_code_content(code)
            if not is_valid:
                self.send_error_json(400, f"Code validation failed: {error_msg}")
                return

            # Determine target file
            if (folder / "Cargo.toml").exists():
                target_file = folder / "src" / "main.rs"
            else:
                target_file = folder / "template.rs"

            # Validate path one more time
            target_file = sanitize_path(target_file)

            with open(target_file, "w") as f:
                f.write(code)

            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_cors_headers()
            self.send_security_headers()
            self.end_headers()
            self.wfile.write(json.dumps({"success": True, "status": "saved"}).encode())

        except ValueError:
            self.send_error_json(400, "Invalid problem ID")

    def _handle_open(self):
        """Open file in external editor."""
        parts = self.path.split("/")
        if len(parts) < 4:
            self.send_error_json(400, "Invalid request")
            return

        try:
            pid = int(parts[3])
            folder = get_safe_problem_folder(pid)
            if not folder:
                self.send_error_json(404, "Problem not found")
                return

            if (folder / "Cargo.toml").exists():
                target_file = folder / "src" / "main.rs"
            else:
                target_file = folder / "template.rs"

            # Try VS Code first
            try:
                subprocess.Popen(
                    ["code", str(target_file)],
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                )
                status = "success"
            except FileNotFoundError:
                subprocess.Popen(
                    ["xdg-open", str(folder)],
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                )
                status = "opened_folder"

            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_cors_headers()
            self.send_security_headers()
            self.end_headers()
            self.wfile.write(json.dumps({"status": status}).encode())

        except ValueError:
            self.send_error_json(400, "Invalid problem ID")

    def _handle_exercise_data(self):
        """Serve exercise metadata."""
        data_file = ROOT / "web" / "exercise_data.json"
        if data_file.exists():
            try:
                with open(data_file, "r") as f:
                    # Validate JSON before serving
                    data = json.load(f)

                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_cors_headers()
                self.send_security_headers()
                self.end_headers()
                self.wfile.write(json.dumps(data).encode())
            except json.JSONDecodeError as e:
                self.send_error_json(500, f"Invalid exercise data format: {e}")
            except Exception as e:
                self.send_error_json(500, f"Error reading exercise data: {e}")
        else:
            self.send_error_json(404, "Exercise data not found")

    def end_headers(self):
        """Override to add security headers."""
        self.send_security_headers()
        super().end_headers()


# =============================================================================
# MAIN ENTRY POINT
# =============================================================================


def print_security_banner():
    """Print security configuration on startup."""
    print("=" * 70)
    print("🔒 RUST LEARNING GROUND - SECURE MODE")
    print("=" * 70)

    if REQUIRE_AUTH or AUTH_TOKEN:
        print("✅ Authentication: Enabled")
    else:
        print("⚠️  Authentication: DISABLED (set RUST_LEARNING_AUTH_TOKEN to enable)")

    sandbox = CodeSandbox.check_sandbox_available()
    if sandbox == "firejail":
        print("✅ Sandbox: Firejail (recommended)")
    elif sandbox == "docker":
        print("✅ Sandbox: Docker")
    else:
        print("⚠️  Sandbox: None (install firejail or docker for better security)")

    print(f"✅ Max code size: {MAX_CODE_SIZE:,} bytes")
    print(f"✅ Allowed origins: {', '.join(ALLOWED_ORIGINS)}")
    print(f"✅ Rate limiting: Enabled")
    print("=" * 70)


def run_dashboard():
    """Start the secure web dashboard."""
    web_dir = ROOT / "web"
    if not web_dir.exists():
        print("❌ Web directory not found.")
        return

    print_security_banner()

    os.chdir(web_dir)
    socketserver.TCPServer.allow_reuse_address = True

    try:
        with socketserver.TCPServer(("", PORT), SecureHandler) as httpd:
            auth_status = (
                "with auth" if (REQUIRE_AUTH or AUTH_TOKEN) else "WITHOUT AUTH"
            )
            print(f"🌐 Server running on http://localhost:{PORT} ({auth_status})")
            print(f"   Press Ctrl+C to stop")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n👋 Server stopped.")
    except OSError as e:
        print(f"❌ Could not start server: {e}")


def run_problem(pid: int):
    """Run a problem by ID (CLI mode)."""
    if not 1 <= pid <= TOTAL_PROBLEMS:
        print(f"❌ Invalid problem ID: {pid}. Must be between 1 and {TOTAL_PROBLEMS}.")
        return

    folder = get_safe_problem_folder(pid)
    if not folder:
        print(f"❌ Problem {pid:03d} not found.")
        return

    is_cargo = (folder / "Cargo.toml").exists()

    print(f"🦀 Running problem {pid:03d}...")

    if is_cargo:
        result = subprocess.run(
            ["cargo", "test", "--quiet"],
            cwd=folder,
            capture_output=True,
            text=True,
            timeout=60,
        )
        success = result.returncode == 0
        if not success:
            print(f"❌ Tests failed:\n{result.stderr or result.stdout}")
    else:
        src = folder / "template.rs"
        exe = folder / "runner_bin"

        sandbox = CodeSandbox()
        sandbox_type = sandbox.check_sandbox_available()

        if sandbox_type == "firejail":
            success, output = sandbox.run_with_firejail(src)
        else:
            success, output = sandbox.run_basic(src, exe)

        if not success:
            print(f"❌ {output}")

    if success:
        is_new = profile.mark_solved(pid)
        pid_full = f"{pid:03d}"

        if is_new:
            print(f"✅ PROBLEM {pid_full} SOLVED! (+10 XP)")
        else:
            print(f"✅ PROBLEM {pid_full} SOLVED! (already counted)")

        current_stats = profile.load()
        print(
            f"   Total XP: {current_stats['xp']} | Solved: {len(current_stats['solved'])}/{TOTAL_PROBLEMS}"
        )


def list_problems():
    """List all problems with their status."""
    data = profile.load()
    solved_set = set(data["solved"])

    print(f"\n🦀 Rust Learning Ground - Problem List")
    print(f"   XP: {data['xp']} | Solved: {len(solved_set)}/{TOTAL_PROBLEMS}")
    print("-" * 60)

    for pid in range(1, TOTAL_PROBLEMS + 1):
        folder = get_safe_problem_folder(pid)
        status = "✅" if pid in solved_set else "⬜"
        ptype = "📦" if folder and (folder / "Cargo.toml").exists() else "📝"
        folder_name = folder.name if folder else "MISSING"
        print(f"{status} {ptype} {pid:03d}: {folder_name}")
    print()


def reset_progress():
    """Reset all progress."""
    confirm = input("⚠️  Are you sure you want to reset all progress? (yes/no): ")
    if confirm.lower() == "yes":
        profile.reset()
        print("✅ Progress reset successfully.")
    else:
        print("Cancelled.")


def show_help():
    """Show help message."""
    print(__doc__)
    print("\nCommands:")
    print("  ./r.py run <id>       Run problem/exercise by ID")
    print("  ./r.py dashboard      Start the web dashboard")
    print("  ./r.py list           List all problems with status")
    print("  ./r.py reset          Reset user progress")
    print("  ./r.py help           Show this help message")
    print("\nEnvironment Variables:")
    print("  RUST_LEARNING_AUTH_TOKEN  - Set to require authentication")
    print("  RUST_LEARNING_PORT        - Port to listen on (default: 54321)")
    print("  ALLOWED_ORIGINS           - Comma-separated allowed origins")
    print("  SANDBOX_MODE              - 'firejail', 'docker', 'none', or 'auto'")
    print("  REQUIRE_AUTH              - 'true' to force authentication")


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        show_help()
        return

    cmd = sys.argv[1].lower()

    if cmd == "run":
        if len(sys.argv) < 3:
            print("❌ Please provide a problem ID: ./r.py run <id>")
            return
        try:
            pid = int(sys.argv[2])
            run_problem(pid)
        except ValueError:
            print("❌ Invalid problem ID. Must be a number.")
    elif cmd == "dashboard":
        run_dashboard()
    elif cmd == "list":
        list_problems()
    elif cmd == "reset":
        reset_progress()
    elif cmd in ("help", "-h", "--help"):
        show_help()
    else:
        print(f"❌ Unknown command: {cmd}")
        print("Use './r.py help' for usage information.")


if __name__ == "__main__":
    main()
