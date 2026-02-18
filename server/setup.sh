#!/bin/bash
# Setup script for Rust Learning Ground Authentication Server

set -e

echo "🦀 Setting up Rust Learning Ground Authentication Server..."

# Check Python version
python_version=$(python3 --version 2>&1 | grep -oP '\d+\.\d+')
required_version="3.9"

if [ "$(printf '%s\n' "$required_version" "$python_version" | sort -V | head -n1)" != "$required_version" ]; then 
    echo "❌ Python 3.9 or higher is required. Found: $python_version"
    exit 1
fi

echo "✅ Python version: $python_version"

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "📦 Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
echo "🔄 Activating virtual environment..."
source venv/bin/activate

# Upgrade pip
echo "⬆️  Upgrading pip..."
pip install --upgrade pip

# Install dependencies
echo "📥 Installing dependencies..."
pip install -r requirements.txt

# Create data directory
mkdir -p data

# Check if .env exists
if [ ! -f ".env" ]; then
    echo "⚙️  Creating .env file from template..."
    cp .env.example .env
    
    # Generate random secrets
    secret_key=$(python3 -c "import secrets; print(secrets.token_urlsafe(64))")
    jwt_secret=$(python3 -c "import secrets; print(secrets.token_urlsafe(64))")
    
    # Update .env with generated secrets
    sed -i "s|SECRET_KEY=.*|SECRET_KEY=$secret_key|" .env
    sed -i "s|JWT_SECRET_KEY=.*|JWT_SECRET_KEY=$jwt_secret|" .env
    
    echo "🔑 Generated secure secrets in .env"
    echo ""
    echo "📝 IMPORTANT: Review and update .env file with your settings:"
    echo "   - Set strong SECRET_KEY and JWT_SECRET_KEY for production"
    echo "   - Configure GITHUB_CLIENT_ID and GITHUB_CLIENT_SECRET for OAuth"
    echo "   - Update CORS_ORIGINS for your domain"
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "To start the server:"
echo "   source venv/bin/activate"
echo "   python run.py"
echo ""
echo "For production with Docker:"
echo "   docker-compose up -d"
echo ""
