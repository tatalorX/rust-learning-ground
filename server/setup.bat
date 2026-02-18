@echo off
REM Setup script for Rust Learning Ground Authentication Server (Windows)

echo 🦀 Setting up Rust Learning Ground Authentication Server...

REM Check Python version
python --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Python is not installed or not in PATH
    exit /b 1
)

REM Create virtual environment if it doesn't exist
if not exist "venv" (
    echo 📦 Creating virtual environment...
    python -m venv venv
)

REM Activate virtual environment
echo 🔄 Activating virtual environment...
call venv\Scripts\activate.bat

REM Upgrade pip
echo ⬆️  Upgrading pip...
pip install --upgrade pip

REM Install dependencies
echo 📥 Installing dependencies...
pip install -r requirements.txt

REM Create data directory
if not exist "data" mkdir data

REM Check if .env exists
if not exist ".env" (
    echo ⚙️  Creating .env file from template...
    copy .env.example .env
    
    echo 🔑 Please update .env with your settings
    echo 📝 Set strong SECRET_KEY and JWT_SECRET_KEY for production
)

echo.
echo ✅ Setup complete!
echo.
echo To start the server:
echo    venv\Scripts\activate.bat
echo    python run.py
echo.
echo For production with Docker:
echo    docker-compose up -d
echo.

pause
