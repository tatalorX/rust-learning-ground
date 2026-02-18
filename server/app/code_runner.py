# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 The Book of Rust Contributors
#
"""
code_runner.py -- Qubes-inspired disposable execution engine.

Each run = ephemeral jail, born and destroyed in milliseconds.
No shared state. No network. No persistence.

Execution Modes (auto-detected, highest security first):
  1. nsjail   -- Full namespace + seccomp isolation (~5ms startup)
  2. Docker   -- Container sandbox with --network=none
  3. Subprocess -- Restricted fallback for development only

In production, ONLY nsjail or Docker modes should be used.
"""

import asyncio
import hashlib
import logging
import os
import re
import shutil
import tempfile
import uuid
from dataclasses import dataclass
from pathlib import Path
from typing import Optional, Dict, Any, Tuple

logger = logging.getLogger(__name__)

# -- Constants -----------------------------------------------------------------
NSJAIL_BIN = "/usr/sbin/nsjail"
NSJAIL_CFG = "/etc/nsjail/rust_exec.cfg"
RUSTC_PATH = "/rust/bin/rustc"  # read-only mount inside jail
CARGO_PATH = "/rust/bin/cargo"

COMPILE_TIMEOUT = 12  # seconds
RUN_TIMEOUT = 5  # seconds
MAX_CODE_BYTES = 65_536  # 64KB source limit
MAX_OUTPUT_BYTES = 65_536  # 64KB output limit
MAX_CONCURRENT = 8  # semaphore: max parallel executions
MAX_CODE_SIZE = 100_000  # 100KB for validation

# Sandbox base -- should be tmpfs-backed in production
SANDBOX_BASE = Path(os.environ.get("SANDBOX_BASE", "/var/sandboxes"))

# Docker fallback
DOCKER_IMAGE = os.environ.get("SANDBOX_DOCKER_IMAGE", "rust-sandbox:latest")

# Prohibited patterns for code validation (defense in depth)
PROHIBITED_PATTERNS = [
    r"std\s*::\s*process\s*::\s*Command",
    r"std\s*::\s*fs\s*::",
    r"std\s*::\s*net\s*::",
    r"std\s*::\s*os\s*::",
    r"unsafe\s*\{",
    r"#\s*!\s*\[\s*feature",
    r"include\s*!",
    r"include_str\s*!",
    r"include_bytes\s*!",
    r"env\s*!",
    r"option_env\s*!",
    r"#![\s]*no_std",
    r"#![\s]*no_main",
    r"extern\s+crate",
    r"mod\s+[^;]+;",
    r"use\s+std::process",
    r"use\s+std::fs",
    r"use\s+std::net",
    r"File::open",
    r"File::create",
    r"File::remove",
    r"fs::remove_file",
    r"fs::remove_dir",
    r"Command::new",
    r"libc::",
    r"winapi::",
]

# Path sanitization for error messages
PATH_PATTERN = re.compile(r"/[\w/]+/")
USER_PATTERN = re.compile(r"home/\w+/")


@dataclass
class ExecutionResult:
    """Result of a sandboxed code execution."""

    success: bool
    stdout: str
    stderr: str
    compile_time_ms: int
    run_time_ms: int
    exit_code: int
    sandbox_id: str
    # Legacy compatibility fields
    output: str = ""
    error: Optional[str] = None
    execution_time_ms: int = 0
    compilation_error: bool = False
    timed_out: bool = False
    memory_exceeded: bool = False
    security_violation: bool = False


def validate_code(code: str) -> Tuple[bool, Optional[str]]:
    """
    Validate code before execution (defense in depth).
    The sandbox itself is the primary security boundary.
    """
    if len(code.encode()) > MAX_CODE_BYTES:
        return False, "Code exceeds 64KB limit."

    if len(code) > MAX_CODE_SIZE:
        return False, f"Code exceeds maximum size of {MAX_CODE_SIZE} bytes"

    for pattern in PROHIBITED_PATTERNS:
        if re.search(pattern, code, re.IGNORECASE | re.MULTILINE):
            return False, "Security violation: Prohibited pattern detected"

    if "fn main()" not in code and "fn main ()" not in code:
        return False, "Code must contain a main function"

    return True, None


def sanitize_error_message(error: str) -> str:
    """Sanitize error messages to prevent information disclosure."""
    if not error:
        return error
    error = PATH_PATTERN.sub("[PATH]/", error)
    error = USER_PATTERN.sub("home/[USER]/", error)
    error = re.sub(r"tmp[\w]+", "[TEMP]", error, flags=re.IGNORECASE)
    return error


class DisposableExecutor:
    """
    Each instance represents one ephemeral execution domain.
    Inspired by Qubes DisposableVMs -- created on demand, destroyed on exit.
    """

    _semaphore = asyncio.Semaphore(MAX_CONCURRENT)

    def __init__(self):
        self.sandbox_id = str(uuid.uuid4())[:8]
        self.sandbox_dir = SANDBOX_BASE / self.sandbox_id

    async def execute(self, code: str, exercise_id: str = "") -> ExecutionResult:
        """Run untrusted code in a disposable sandbox. Returns result."""
        # Validate first (defense in depth)
        is_valid, validation_error = validate_code(code)
        if not is_valid:
            return ExecutionResult(
                success=False,
                stdout="",
                stderr=validation_error or "Validation failed",
                compile_time_ms=0,
                run_time_ms=0,
                exit_code=1,
                sandbox_id=self.sandbox_id,
                output="",
                error=validation_error,
                execution_time_ms=0,
                security_violation=True,
            )

        async with self._semaphore:
            try:
                return await self._run_sandboxed(code, exercise_id)
            finally:
                self._cleanup()

    async def _run_sandboxed(self, code: str, exercise_id: str) -> ExecutionResult:
        """Execute in the best available sandbox."""
        # Try nsjail first (sovereign mode)
        if os.path.isfile(NSJAIL_BIN) and os.path.isfile(NSJAIL_CFG):
            return await self._run_nsjail(code, exercise_id)

        # Try Docker second
        if await self._check_docker():
            logger.warning("nsjail not available, falling back to Docker sandbox")
            return await self._run_docker(code, exercise_id)

        # Fallback to restricted subprocess (development only)
        logger.warning(
            "No sandbox available! Using restricted subprocess (UNSAFE for production)"
        )
        return await self._run_subprocess(code, exercise_id)

    # ── nsjail execution (primary, sovereign mode) ──────────────────────────

    async def _run_nsjail(self, code: str, exercise_id: str) -> ExecutionResult:
        """Full nsjail-based disposable execution."""
        import time

        self.sandbox_dir.mkdir(parents=True, exist_ok=True)
        src_path = self.sandbox_dir / "main.rs"
        bin_path = self.sandbox_dir / "main"
        src_path.write_text(code)

        # Phase 1: Compile inside nsjail
        compile_start = time.monotonic()
        compile_result = await self._nsjail_run(
            cmd=[
                RUSTC_PATH,
                "/home/user/main.rs",
                "-o",
                "/home/user/main",
                "--edition",
                "2021",
                "-C",
                "opt-level=0",
            ],
            bind_src=str(src_path),
            bind_dst="/home/user/main.rs",
            output_bind=str(bin_path),
            timeout=COMPILE_TIMEOUT,
        )
        compile_ms = int((time.monotonic() - compile_start) * 1000)

        if compile_result["exit_code"] != 0:
            stderr = sanitize_error_message(compile_result["stderr"][:MAX_OUTPUT_BYTES])
            return ExecutionResult(
                success=False,
                stdout="",
                stderr=stderr,
                compile_time_ms=compile_ms,
                run_time_ms=0,
                exit_code=compile_result["exit_code"],
                sandbox_id=self.sandbox_id,
                output="",
                error=f"Compilation error:\n{stderr}",
                execution_time_ms=compile_ms,
                compilation_error=True,
            )

        # Phase 2: Execute compiled binary inside fresh nsjail
        run_start = time.monotonic()
        run_result = await self._nsjail_run(
            cmd=["/home/user/main"],
            bind_src=str(bin_path),
            bind_dst="/home/user/main",
            timeout=RUN_TIMEOUT,
        )
        run_ms = int((time.monotonic() - run_start) * 1000)

        stdout = run_result["stdout"][:MAX_OUTPUT_BYTES]
        stderr = sanitize_error_message(run_result["stderr"][:MAX_OUTPUT_BYTES])
        total_ms = compile_ms + run_ms

        return ExecutionResult(
            success=run_result["exit_code"] == 0,
            stdout=stdout,
            stderr=stderr,
            compile_time_ms=compile_ms,
            run_time_ms=run_ms,
            exit_code=run_result["exit_code"],
            sandbox_id=self.sandbox_id,
            output=stdout,
            error=stderr if stderr else None,
            execution_time_ms=total_ms,
            timed_out=run_result["exit_code"] == 124,
        )

    async def _nsjail_run(
        self,
        cmd: list,
        bind_src: str,
        bind_dst: str,
        timeout: int,
        output_bind: Optional[str] = None,
    ) -> dict:
        """Invoke nsjail with config file + dynamic bind mounts."""
        nsjail_cmd = [
            NSJAIL_BIN,
            "--config",
            NSJAIL_CFG,
            "--log_fd",
            "3",
            "--bindmount_ro",
            f"{bind_src}:{bind_dst}",
        ]

        if output_bind:
            Path(output_bind).touch()
            nsjail_cmd += ["--bindmount", f"{output_bind}:/home/user/main"]

        nsjail_cmd += ["--", *cmd]

        try:
            proc = await asyncio.wait_for(
                asyncio.create_subprocess_exec(
                    *nsjail_cmd,
                    stdout=asyncio.subprocess.PIPE,
                    stderr=asyncio.subprocess.PIPE,
                    pass_fds=(3,),
                ),
                timeout=timeout + 2,
            )

            stdout, stderr = await asyncio.wait_for(proc.communicate(), timeout=timeout)

            return {
                "stdout": stdout.decode(errors="replace"),
                "stderr": stderr.decode(errors="replace"),
                "exit_code": proc.returncode or 0,
            }
        except asyncio.TimeoutError:
            try:
                proc.kill()  # type: ignore[possibly-undefined]
            except Exception:
                pass
            return {
                "stdout": "",
                "stderr": f"Execution timed out after {timeout}s",
                "exit_code": 124,
            }

    # ── Docker execution (secondary fallback) ────────────────────────────────

    async def _check_docker(self) -> bool:
        """Check if Docker/Podman is available."""
        try:
            proc = await asyncio.create_subprocess_exec(
                "docker",
                "info",
                stdout=asyncio.subprocess.DEVNULL,
                stderr=asyncio.subprocess.DEVNULL,
            )
            await proc.wait()
            return proc.returncode == 0
        except FileNotFoundError:
            return False

    async def _run_docker(self, code: str, exercise_id: str) -> ExecutionResult:
        """Docker/Podman sandbox execution."""
        import time

        start_time = time.monotonic()
        self.sandbox_dir.mkdir(parents=True, exist_ok=True)
        source_file = self.sandbox_dir / "main.rs"
        source_file.write_text(code)

        docker_cmd = [
            "docker",
            "run",
            "--rm",
            "--network=none",
            "--read-only",
            "--memory=64m",
            "--memory-swap=64m",
            "--cpus=0.5",
            "--pids-limit=10",
            "--security-opt=no-new-privileges:true",
            "--cap-drop=ALL",
            "-v",
            f"{self.sandbox_dir}:/sandbox:ro",
            DOCKER_IMAGE,
        ]

        try:
            proc = await asyncio.create_subprocess_exec(
                *docker_cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
            )

            stdout_bytes, stderr_bytes = await asyncio.wait_for(
                proc.communicate(), timeout=COMPILE_TIMEOUT + RUN_TIMEOUT + 5
            )

            elapsed_ms = int((time.monotonic() - start_time) * 1000)
            stdout = stdout_bytes.decode("utf-8", errors="replace").strip()
            stderr = sanitize_error_message(
                stderr_bytes.decode("utf-8", errors="replace").strip()
            )

            is_compile_error = "COMPILATION_FAILED" in stderr
            is_timeout = "EXECUTION_TIMEOUT" in stderr

            return ExecutionResult(
                success=proc.returncode == 0
                and not is_compile_error
                and not is_timeout,
                stdout=stdout,
                stderr=stderr,
                compile_time_ms=0,
                run_time_ms=elapsed_ms,
                exit_code=proc.returncode or 0,
                sandbox_id=self.sandbox_id,
                output=stdout,
                error=stderr if stderr else None,
                execution_time_ms=elapsed_ms,
                compilation_error=is_compile_error,
                timed_out=is_timeout,
            )
        except asyncio.TimeoutError:
            try:
                proc.kill()  # type: ignore[possibly-undefined]
            except Exception:
                pass
            elapsed_ms = int((time.monotonic() - start_time) * 1000)
            return ExecutionResult(
                success=False,
                stdout="",
                stderr="Execution timed out (sandbox)",
                compile_time_ms=0,
                run_time_ms=elapsed_ms,
                exit_code=124,
                sandbox_id=self.sandbox_id,
                output="",
                error="Execution timed out (sandbox)",
                execution_time_ms=elapsed_ms,
                timed_out=True,
            )
        except Exception as e:
            logger.error(f"Docker execution error: {e}")
            elapsed_ms = int((time.monotonic() - start_time) * 1000)
            return ExecutionResult(
                success=False,
                stdout="",
                stderr="Sandbox execution failed",
                compile_time_ms=0,
                run_time_ms=elapsed_ms,
                exit_code=1,
                sandbox_id=self.sandbox_id,
                output="",
                error="Sandbox execution failed",
                execution_time_ms=elapsed_ms,
            )

    # ── Subprocess execution (DEVELOPMENT ONLY - NOT FOR PRODUCTION) ─────────

    async def _run_subprocess(self, code: str, exercise_id: str) -> ExecutionResult:
        """
        Restricted subprocess fallback for development ONLY.

        SECURITY WARNING: This method executes code without sandboxing.
        It is gated behind an explicit environment check and will fail closed
        in production environments.
        """
        import time
        import os

        # SECURITY FIX: Gate subprocess fallback behind explicit environment check (C-05)
        env = os.environ.get("ENVIRONMENT", "production").lower()
        debug = os.environ.get("DEBUG", "false").lower() == "true"

        if env == "production" or (env != "development" and not debug):
            logger.error(
                "Subprocess fallback attempted in production environment - BLOCKED"
            )
            return ExecutionResult(
                success=False,
                stdout="",
                stderr="Sandbox not available. Code execution disabled in production.",
                compile_time_ms=0,
                run_time_ms=0,
                exit_code=1,
                sandbox_id=self.sandbox_id,
                output="",
                error="Sandbox not available. Code execution disabled in production.",
                execution_time_ms=0,
                security_violation=True,
            )

        start_time = time.monotonic()

        # Use system temp if sandbox base doesn't exist
        base_dir = (
            SANDBOX_BASE if SANDBOX_BASE.exists() else Path(tempfile.gettempdir())
        )
        with tempfile.TemporaryDirectory(dir=base_dir) as temp_dir:
            temp_path = Path(temp_dir)
            source_file = temp_path / "main.rs"
            binary_file = temp_path / "main"

            try:
                source_file.write_text(code)
            except Exception as e:
                elapsed_ms = int((time.monotonic() - start_time) * 1000)
                return ExecutionResult(
                    success=False,
                    stdout="",
                    stderr=f"Failed to write code: {e}",
                    compile_time_ms=0,
                    run_time_ms=0,
                    exit_code=1,
                    sandbox_id=self.sandbox_id,
                    output="",
                    error=f"Failed to write code: {e}",
                    execution_time_ms=elapsed_ms,
                )

            # Compile
            compile_start = time.monotonic()
            try:
                proc = await asyncio.create_subprocess_exec(
                    "rustc",
                    str(source_file),
                    "-o",
                    str(binary_file),
                    "--edition",
                    "2021",
                    "-C",
                    "opt-level=0",
                    stderr=asyncio.subprocess.PIPE,
                    stdout=asyncio.subprocess.PIPE,
                )
                await asyncio.wait_for(proc.wait(), timeout=COMPILE_TIMEOUT)
            except asyncio.TimeoutError:
                proc.kill()  # type: ignore
                compile_ms = int((time.monotonic() - compile_start) * 1000)
                return ExecutionResult(
                    success=False,
                    stdout="",
                    stderr=f"Compilation timed out after {COMPILE_TIMEOUT}s",
                    compile_time_ms=compile_ms,
                    run_time_ms=0,
                    exit_code=1,
                    sandbox_id=self.sandbox_id,
                    output="",
                    error=f"Compilation timed out after {COMPILE_TIMEOUT}s",
                    execution_time_ms=compile_ms,
                    compilation_error=True,
                    timed_out=True,
                )
            except FileNotFoundError:
                compile_ms = int((time.monotonic() - compile_start) * 1000)
                return ExecutionResult(
                    success=False,
                    stdout="",
                    stderr="Rust compiler (rustc) not found",
                    compile_time_ms=compile_ms,
                    run_time_ms=0,
                    exit_code=1,
                    sandbox_id=self.sandbox_id,
                    output="",
                    error="Rust compiler (rustc) not found",
                    execution_time_ms=compile_ms,
                    compilation_error=True,
                )

            compile_ms = int((time.monotonic() - compile_start) * 1000)

            if proc.returncode != 0:
                stderr_bytes = await proc.stderr.read() if proc.stderr else b""
                stderr_str = sanitize_error_message(
                    stderr_bytes.decode("utf-8", errors="replace")
                )
                return ExecutionResult(
                    success=False,
                    stdout="",
                    stderr=stderr_str,
                    compile_time_ms=compile_ms,
                    run_time_ms=0,
                    exit_code=proc.returncode or 1,
                    sandbox_id=self.sandbox_id,
                    output="",
                    error=f"Compilation error:\n{stderr_str}",
                    execution_time_ms=compile_ms,
                    compilation_error=True,
                )

            # Execute
            run_start = time.monotonic()
            try:
                exec_proc = await asyncio.create_subprocess_exec(
                    str(binary_file),
                    stderr=asyncio.subprocess.PIPE,
                    stdout=asyncio.subprocess.PIPE,
                )
                stdout_bytes, stderr_bytes = await asyncio.wait_for(
                    exec_proc.communicate(), timeout=RUN_TIMEOUT
                )
            except asyncio.TimeoutError:
                exec_proc.kill()  # type: ignore
                run_ms = int((time.monotonic() - run_start) * 1000)
                total_ms = compile_ms + run_ms
                return ExecutionResult(
                    success=False,
                    stdout="",
                    stderr=f"Execution timed out after {RUN_TIMEOUT}s",
                    compile_time_ms=compile_ms,
                    run_time_ms=run_ms,
                    exit_code=124,
                    sandbox_id=self.sandbox_id,
                    output="",
                    error=f"Execution timed out after {RUN_TIMEOUT}s",
                    execution_time_ms=total_ms,
                    timed_out=True,
                )

            run_ms = int((time.monotonic() - run_start) * 1000)
            total_ms = compile_ms + run_ms

            stdout = stdout_bytes.decode("utf-8", errors="replace")[:MAX_OUTPUT_BYTES]
            stderr_str = sanitize_error_message(
                stderr_bytes.decode("utf-8", errors="replace")[:MAX_OUTPUT_BYTES]
            )

            if exec_proc.returncode != 0:
                return ExecutionResult(
                    success=False,
                    stdout=stdout,
                    stderr=stderr_str,
                    compile_time_ms=compile_ms,
                    run_time_ms=run_ms,
                    exit_code=exec_proc.returncode or 1,
                    sandbox_id=self.sandbox_id,
                    output=stdout,
                    error=f"Runtime error (exit code {exec_proc.returncode}):\n{stderr_str}",
                    execution_time_ms=total_ms,
                )

            return ExecutionResult(
                success=True,
                stdout=stdout,
                stderr=stderr_str,
                compile_time_ms=compile_ms,
                run_time_ms=run_ms,
                exit_code=0,
                sandbox_id=self.sandbox_id,
                output=stdout,
                error=None,
                execution_time_ms=total_ms,
            )

    def _cleanup(self):
        """Destroy the ephemeral sandbox. No trace left."""
        try:
            if self.sandbox_dir.exists():
                shutil.rmtree(self.sandbox_dir, ignore_errors=True)
        except Exception:
            pass


# -- Public API ----------------------------------------------------------------


async def run_code(code: str, exercise_id: str = "") -> ExecutionResult:
    """
    Entry point. Creates a disposable executor, runs code, destroys everything.
    Caller never interacts with the sandbox directly.
    """
    executor = DisposableExecutor()
    return await executor.execute(code, exercise_id)


class CodeRunner:
    """
    Backward-compatible wrapper around DisposableExecutor.
    Maintains the same interface as the original code_runner.py.
    """

    @staticmethod
    async def run_code(code: str, exercise_id: Optional[int] = None) -> ExecutionResult:
        """Run code in the best available sandbox."""
        executor = DisposableExecutor()
        return await executor.execute(code, str(exercise_id or ""))

    @staticmethod
    async def verify_solution(
        code: str, expected_output: str, exercise_id: Optional[int] = None
    ) -> Dict[str, Any]:
        """Verify solution against expected output."""
        result = await CodeRunner.run_code(code, exercise_id)

        actual_output = (result.stdout or result.output or "").strip()
        success = result.success and actual_output == expected_output.strip()

        return {
            "success": success,
            "output": actual_output,
            "expected": expected_output,
            "error": result.error,
            "execution_time_ms": result.execution_time_ms,
            "compilation_error": result.compilation_error,
            "timed_out": result.timed_out,
            "security_violation": result.security_violation,
            "sandbox_id": result.sandbox_id,
        }


def get_code_runner() -> CodeRunner:
    """Get code runner instance."""
    return CodeRunner()
