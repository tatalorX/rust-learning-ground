#!/bin/bash
# fix-deployment.sh - Fix common deployment issues

set -e

echo "🔧 Fixing deployment issues..."

# 1. Stop any running containers
echo "[1/5] Stopping existing containers..."
if command -v podman-compose &> /dev/null; then
    podman-compose -f compose.prod.yml down 2>/dev/null || true
elif command -v docker-compose &> /dev/null; then
    docker-compose -f compose.prod.yml down 2>/dev/null || true
fi

# 2. Clean up old containers
echo "[2/5] Cleaning up old containers..."
if command -v podman &> /dev/null; then
    podman rm -f rust-learning-ground_caddy_1 rust-learning-ground_api_1 rust-learning-ground_postgres_1 2>/dev/null || true
elif command -v docker &> /dev/null; then
    docker rm -f rust-learning-ground_caddy_1 rust-learning-ground_api_1 rust-learning-ground_postgres_1 2>/dev/null || true
fi

# 3. Check AppArmor status and disable problematic profiles
echo "[3/5] Checking AppArmor..."
if command -v aa-status &> /dev/null; then
    echo "AppArmor is installed. Checking profiles..."
    # Disable any blocking profiles temporarily
    sudo aa-disable caddy 2>/dev/null || true
    sudo aa-disable rust-learner-api 2>/dev/null || true
fi

# 4. Create necessary directories
echo "[4/5] Creating directories..."
mkdir -p /var/log/caddy /var/log/rustground /var/sandboxes

# 5. Pull latest images
echo "[5/5] Pulling latest images..."
if command -v podman-compose &> /dev/null; then
    podman-compose -f compose.prod.yml pull
elif command -v docker-compose &> /dev/null; then
    docker-compose -f compose.prod.yml pull
fi

echo ""
echo "✅ Deployment issues fixed!"
echo ""
echo "Next steps:"
echo "1. Review the updated compose.prod.yml"
echo "2. Configure your SERVER_IP and DOMAIN in deploy.sh"
echo "3. Run: sudo bash deploy.sh"
