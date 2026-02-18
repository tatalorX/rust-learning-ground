#!/bin/bash
# deploy.sh — One-command deployment for Rust Learning Ground
# Usage: sudo bash deploy.sh
# 
# This script will:
# 1. Check for required configuration
# 2. Install dependencies if missing
# 3. Clone/update the repository
# 4. Build and start all services

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# CONFIGURATION - These can be set via environment variables
SERVER_IP="${SERVER_IP:-}"
DOMAIN="${DOMAIN:-}"
EMAIL="${EMAIL:-admin@example.com}"

# Deployment directory
DEPLOY_DIR="${DEPLOY_DIR:-/opt/rustground}"
REPO_URL="${REPO_URL:-https://github.com/tatalorX/rust-learning-ground.git}"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo " Rust Learning Ground — Automated Deploy Script"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# ── 0. Check configuration ────────────────────────────────────────────────────
if [ -z "$DOMAIN" ]; then
    echo -e "${YELLOW}⚠️  DOMAIN not set. Please set it:${NC}"
    echo "   export DOMAIN=yourdomain.com"
    echo "   sudo -E bash deploy.sh"
    exit 1
fi

if [ -z "$SERVER_IP" ]; then
    # Try to auto-detect
    SERVER_IP=$(hostname -I | awk '{print $1}')
    echo -e "${YELLOW}⚠️  SERVER_IP not set. Using detected IP: $SERVER_IP${NC}"
    echo "   To override: export SERVER_IP=your.ip.address.here"
fi

echo ""
echo "Configuration:"
echo "  Domain: $DOMAIN"
echo "  Server IP: $SERVER_IP"
echo "  Email: $EMAIL"
echo "  Deploy Dir: $DEPLOY_DIR"
echo ""

# ── 1. Install dependencies ───────────────────────────────────────────────────
echo "[1/8] Checking dependencies..."

# Check for Podman
if ! command -v podman &> /dev/null; then
    echo "  Installing Podman..."
    if command -v apt-get &> /dev/null; then
        apt-get update
        apt-get install -y podman podman-compose
    elif command -v yum &> /dev/null; then
        yum install -y podman podman-docker
    elif command -v dnf &> /dev/null; then
        dnf install -y podman podman-compose
    else
        echo -e "${RED}❌ Could not install Podman automatically${NC}"
        echo "   Please install Podman manually"
        exit 1
    fi
fi

# Check for Caddy
if ! command -v caddy &> /dev/null; then
    echo "  Installing Caddy..."
    apt-get install -y debian-keyring debian-archive-keyring apt-transport-https
    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
    curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list
    apt-get update
    apt-get install -y caddy
fi

# Check for nsjail
if ! command -v nsjail &> /dev/null; then
    echo "  Installing nsjail..."
    if command -v apt-get &> /dev/null; then
        apt-get install -y nsjail
    else
        echo -e "${YELLOW}⚠️  nsjail not available in repos. Building from source...${NC}"
        # Fallback: build nsjail
        apt-get install -y build-essential git protobuf-compiler libprotobuf-dev
        git clone https://github.com/google/nsjail.git /tmp/nsjail
        cd /tmp/nsjail
        make
        cp nsjail /usr/local/bin/
    fi
fi

# Check for git
if ! command -v git &> /dev/null; then
    apt-get install -y git
fi

echo -e "${GREEN}  ✓ All dependencies installed${NC}"

# ── 2. Clone or update repository ─────────────────────────────────────────────
echo "[2/8] Setting up repository..."
if [ -d "$DEPLOY_DIR/.git" ]; then
    echo "  Repository exists, updating..."
    cd $DEPLOY_DIR
    git pull origin main
else
    echo "  Cloning repository..."
    mkdir -p $DEPLOY_DIR
    git clone $REPO_URL $DEPLOY_DIR
    cd $DEPLOY_DIR
fi
echo -e "${GREEN}  ✓ Repository ready${NC}"

# ── 3. Setup nsjail configuration ────────────────────────────────────────────
echo "[3/8] Setting up nsjail..."
if [ ! -f /etc/nsjail/rust_exec.cfg ]; then
    mkdir -p /etc/nsjail
    cp $DEPLOY_DIR/deploy/nsjail/rust_exec.cfg /etc/nsjail/
    echo -e "${GREEN}  ✓ nsjail config installed${NC}"
else
    echo "  nsjail config already exists"
fi

# ── 4. Create .env file ──────────────────────────────────────────────────────
echo "[4/8] Creating environment configuration..."
if [ ! -f $DEPLOY_DIR/.env ]; then
    JWT_SECRET=$(openssl rand -hex 32)
    SECRET_KEY=$(openssl rand -hex 32)
    POSTGRES_PASSWORD=$(openssl rand -hex 16)
    GRAFANA_PASSWORD=$(openssl rand -hex 16)
    
    cat > $DEPLOY_DIR/.env << EOF
# Rust Learning Ground - Production Environment
# Generated on $(date)
# DO NOT COMMIT THIS FILE

# Security
JWT_SECRET=$JWT_SECRET
SECRET_KEY=$SECRET_KEY

# Database
DATABASE_URL=postgresql://rustground:$POSTGRES_PASSWORD@postgres:5432/rustground
POSTGRES_DB=rustground
POSTGRES_USER=rustground
POSTGRES_PASSWORD=$POSTGRES_PASSWORD

# Monitoring
GRAFANA_PASSWORD=$GRAFANA_PASSWORD

# Application
ENVIRONMENT=production
DEBUG=false
APP_NAME="Rust Learning Ground"

# Sandbox
SANDBOX_BASE=/var/sandboxes

# Server
DOMAIN=$DOMAIN
SERVER_IP=$SERVER_IP
EOF
    chmod 600 $DEPLOY_DIR/.env
    echo -e "${GREEN}  ✓ .env created with secure secrets${NC}"
    echo -e "${YELLOW}  ⚠️  IMPORTANT: Save these credentials:${NC}"
    echo ""
    cat $DEPLOY_DIR/.env
    echo ""
else
    echo "  .env already exists, loading..."
    source $DEPLOY_DIR/.env
fi

# ── 5. Create Caddyfile ──────────────────────────────────────────────────────
echo "[5/8] Creating Caddyfile..."
cat > $DEPLOY_DIR/Caddyfile << EOF
{
    email $EMAIL
    auto_https off
}

$DOMAIN {
    header {
        Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
        X-Content-Type-Options "nosniff"
        X-Frame-Options "DENY"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "no-referrer"
        Permissions-Policy "geolocation=(), camera=(), microphone=()"
        Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"
        -Server
        -X-Powered-By
    }

    handle /api/* {
        reverse_proxy api:8000 {
            health_uri /api/v4/health
            health_interval 30s
            header_up X-Real-IP {remote_host}
            header_up X-Forwarded-Proto {scheme}
        }
    }

    handle {
        reverse_proxy api:8000 {
            header_up X-Real-IP {remote_host}
            header_up X-Forwarded-Proto {scheme}
        }
    }

    log {
        output file /var/log/caddy/access.log {
            roll_size 100mb
            roll_keep 5
        }
        format json
    }
}
EOF
echo -e "${GREEN}  ✓ Caddyfile created${NC}"

# ── 6. Prepare directories ───────────────────────────────────────────────────
echo "[6/8] Preparing directories..."
mkdir -p /var/log/caddy /var/log/rustground /var/sandboxes

# Mount sandbox as tmpfs
if ! mountpoint -q /var/sandboxes; then
    mount -t tmpfs -o size=2G,noexec,nosuid,nodev tmpfs /var/sandboxes || true
    echo -e "${GREEN}  ✓ /var/sandboxes mounted as tmpfs${NC}"
fi

# ── 7. Build and start containers ────────────────────────────────────────────
echo "[7/8] Building and starting containers..."
cd $DEPLOY_DIR

# Stop any existing containers
podman-compose -f compose.prod.yml down 2>/dev/null || true

# Build API container
echo "  Building API container (this may take 5-10 minutes)..."
podman-compose -f compose.prod.yml build --no-cache api

# Start all services
echo "  Starting all services..."
podman-compose -f compose.prod.yml up -d

# Wait for API to be healthy
echo "  Waiting for API to be healthy..."
for i in {1..60}; do
    if curl -sf http://localhost:8000/api/v4/health > /dev/null 2>&1; then
        echo -e "${GREEN}  ✓ API is healthy${NC}"
        break
    fi
    if [ $i -eq 60 ]; then
        echo -e "${RED}  ✗ API failed to start${NC}"
        echo "   Check logs: podman-compose -f compose.prod.yml logs api"
        exit 1
    fi
    echo "    ... attempt $i/60"
    sleep 2
done

# ── 8. Start Caddy ───────────────────────────────────────────────────────────
echo "[8/8] Starting Caddy..."
caddy validate --config $DEPLOY_DIR/Caddyfile
caddy stop 2>/dev/null || true
caddy start --config $DEPLOY_DIR/Caddyfile

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✅ DEPLOYMENT COMPLETE!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Your Rust Learning Ground instance is running at:"
echo "  https://$DOMAIN"
echo ""
echo "Health check:"
curl -sf http://localhost:8000/api/v4/health && echo -e "${GREEN}✓ API responding${NC}" || echo -e "${RED}✗ API not responding${NC}"
echo ""
echo "Useful commands:"
echo "  View logs:    podman-compose -f $DEPLOY_DIR/compose.prod.yml logs -f"
echo "  Stop:         podman-compose -f $DEPLOY_DIR/compose.prod.yml down"
echo "  Restart:      sudo bash $DEPLOY_DIR/deploy.sh"
echo ""
echo "Next steps:"
echo "  1. Configure DNS to point $DOMAIN to $SERVER_IP"
echo "  2. Wait 5-10 minutes for DNS propagation"
echo "  3. Visit https://$DOMAIN"
echo ""
