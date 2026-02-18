#!/usr/bin/env python3
"""
Startup script for the Rust Learning Ground server.
"""

import os
import sys
import secrets
import argparse
from pathlib import Path


def generate_secret():
    """Generate a secure random secret."""
    return secrets.token_urlsafe(32)


def setup_environment():
    """Ensure environment is properly configured."""
    env_file = Path(__file__).parent / ".env"
    env_example = Path(__file__).parent / ".env.example"

    if not env_file.exists():
        print("⚠️  .env file not found. Creating from .env.example...")
        if env_example.exists():
            with open(env_example, "r") as f:
                content = f.read()

            # Generate random secrets
            content = content.replace(
                "SECRET_KEY=change-me-in-production-generate-a-64-char-random-string",
                f"SECRET_KEY={generate_secret()}",
            )
            content = content.replace(
                "JWT_SECRET_KEY=change-me-too-generate-another-64-char-random-string",
                f"JWT_SECRET_KEY={generate_secret()}",
            )

            # SECURITY FIX: Set restrictive permissions before writing
            import os as os_module

            fd = os_module.open(env_file, os_module.O_WRONLY | os_module.O_CREAT, 0o600)
            with os_module.fdopen(fd, "w") as f:
                f.write(content)
            print(f"✅ Created .env file with generated secrets at {env_file}")
            print("📝 Please review and update the configuration as needed.")
        else:
            print("❌ .env.example not found!")
            sys.exit(1)


def check_database():
    """Ensure data directory exists."""
    data_dir = Path(__file__).parent / "data"
    data_dir.mkdir(exist_ok=True)


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Rust Learning Ground Server")
    parser.add_argument("--host", default="0.0.0.0", help="Host to bind to")
    parser.add_argument("--port", type=int, default=8000, help="Port to bind to")
    parser.add_argument(
        "--reload", action="store_true", help="Enable auto-reload (dev only)"
    )
    parser.add_argument("--workers", type=int, default=1, help="Number of workers")
    args = parser.parse_args()

    # Setup
    setup_environment()
    check_database()

    # Import after environment setup
    from dotenv import load_dotenv

    load_dotenv()

    # Check for production safety
    secret_key = os.getenv("SECRET_KEY", "")
    if "change-me" in secret_key.lower() or len(secret_key) < 32:
        print("⚠️  WARNING: Using weak SECRET_KEY! Please set a strong secret in .env")
        if os.getenv("ENVIRONMENT") == "production":
            print("❌ Cannot start in production with weak secrets!")
            sys.exit(1)

    print(f"🦀 Starting Rust Learning Ground Server...")
    print(f"   Environment: {os.getenv('ENVIRONMENT', 'development')}")
    print(
        f"   Database: {os.getenv('DATABASE_URL', 'sqlite+aiosqlite:///./data/app.db')}"
    )
    print(f"   Listening on: http://{args.host}:{args.port}")

    if args.reload:
        print("   ⚡ Auto-reload enabled")

    # Start server
    import uvicorn

    uvicorn.run(
        "app.main:app",
        host=args.host,
        port=args.port,
        reload=args.reload,
        workers=args.workers if not args.reload else 1,
        access_log=True,
    )


if __name__ == "__main__":
    main()
