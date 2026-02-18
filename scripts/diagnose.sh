#!/bin/bash
# =============================================================================
# Rust Learning Ground - Network Diagnostics Script
# =============================================================================
# This script helps diagnose network/CORS issues with the Rust Learning Ground
# =============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Rust Learning Ground - Network Diagnostics                  ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if commands exist
check_command() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✓${NC} $1 is installed"
        return 0
    else
        echo -e "${RED}✗${NC} $1 is NOT installed"
        return 1
    fi
}

# Test HTTP endpoint
test_endpoint() {
    local url=$1
    local name=$2
    
    echo -e "\n${BLUE}Testing $name${NC}"
    echo "  URL: $url"
    
    if response=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 "$url" 2>/dev/null); then
        if [ "$response" = "200" ]; then
            echo -e "  Status: ${GREEN}✓ OK (200)${NC}"
            return 0
        else
            echo -e "  Status: ${YELLOW}⚠ HTTP $response${NC}"
            return 1
        fi
    else
        echo -e "  Status: ${RED}✗ Connection failed${NC}"
        return 1
    fi
}

echo "${BLUE}System Information:${NC}"
echo "  OS: $(uname -s)"
echo "  Hostname: $(hostname)"
echo "  User: $(whoami)"
echo ""

echo "${BLUE}Checking Dependencies:${NC}"
check_command python3
check_command curl
check_command ngrok || echo "  (ngrok optional - for public access)"
echo ""

echo "${BLUE}Checking Server Status:${NC}"

# Test backend
test_endpoint "http://localhost:8000/api/health" "Backend (port 8000)" || {
    echo -e "  ${YELLOW}Tip:${NC} Start the backend with:"
    echo "    cd server && source venv/bin/activate && python run.py"
}

# Test frontend
test_endpoint "http://localhost:54321" "Frontend (port 54321)" || {
    echo -e "  ${YELLOW}Tip:${NC} Start the frontend with:"
    echo "    ./r.py dashboard"
}

echo ""
echo "${BLUE}Checking CORS Configuration:${NC}"

# Test CORS headers
echo "  Testing CORS headers from backend..."
if response=$(curl -s -I -X OPTIONS -H "Origin: http://localhost:54321" \
    -H "Access-Control-Request-Method: POST" \
    http://localhost:8000/api/auth/login 2>/dev/null); then
    
    if echo "$response" | grep -q "Access-Control-Allow-Origin"; then
        echo -e "  ${GREEN}✓ CORS headers present${NC}"
        allow_origin=$(echo "$response" | grep -i "Access-Control-Allow-Origin" | tr -d '\r')
        echo "    $allow_origin"
    else
        echo -e "  ${YELLOW}⚠ CORS headers not found${NC}"
        echo "  Response headers:"
        echo "$response" | grep -E "^HTTP|^[A-Za-z-]+:" | head -10
    fi
else
    echo -e "  ${RED}✗ Failed to test CORS${NC}"
fi

echo ""
echo "${BLUE}Checking Environment Configuration:${NC}"

if [ -f "server/.env" ]; then
    echo -e "  ${GREEN}✓ server/.env exists${NC}"
    
    # Show CORS_ORIGINS
    cors_origins=$(grep "CORS_ORIGINS" server/.env | cut -d= -f2 | head -1)
    if [ -n "$cors_origins" ]; then
        echo "  CORS_ORIGINS: $cors_origins"
    fi
    
    # Show ALLOWED_HOSTS
    allowed_hosts=$(grep "ALLOWED_HOSTS" server/.env | cut -d= -f2 | head -1)
    if [ -n "$allowed_hosts" ]; then
        echo "  ALLOWED_HOSTS: $allowed_hosts"
    fi
else
    echo -e "  ${YELLOW}⚠ server/.env not found${NC}"
    echo "  Run: cp .env.example server/.env"
fi

echo ""
echo "${BLUE}Checking ngrok Status:${NC}"

if command -v ngrok &> /dev/null; then
    if curl -s http://127.0.0.1:4040/api/tunnels &> /dev/null; then
        echo -e "  ${GREEN}✓ ngrok is running${NC}"
        ngrok_url=$(curl -s http://127.0.0.1:4040/api/tunnels | grep -o '"public_url":"https://[^"]*' | grep -o 'https://[^"]*' | head -1)
        if [ -n "$ngrok_url" ]; then
            echo "  Public URL: $ngrok_url"
            echo ""
            echo -e "${YELLOW}To use this URL:${NC}"
            echo "  1. Set in browser console: localStorage.setItem('api_base', '$ngrok_url')"
            echo "  2. Or add to server/.env: CORS_ORIGINS=...,$ngrok_url"
            echo "  3. Reload the page"
        fi
    else
        echo -e "  ${YELLOW}⚠ ngrok is installed but not running${NC}"
        echo "  To start: ngrok http 8000"
    fi
else
    echo -e "  ${YELLOW}⚠ ngrok not installed${NC}"
    echo "  Install: sudo snap install ngrok"
    echo "  Then sign up at https://ngrok.com"
fi

echo ""
echo "${BLUE}═══════════════════════════════════════════════════════════════${NC}"

# Summary
echo ""
echo "${BLUE}Summary & Recommendations:${NC}"

backend_ok=false
frontend_ok=false

if curl -s http://localhost:8000/api/health &> /dev/null; then
    backend_ok=true
fi

if curl -s http://localhost:54321 &> /dev/null; then
    frontend_ok=true
fi

if [ "$backend_ok" = true ] && [ "$frontend_ok" = true ]; then
    echo -e "  ${GREEN}✓ Both servers are running${NC}"
    echo ""
    echo "  If you're still seeing network errors:"
    echo "  1. Check browser console (F12) for CORS errors"
    echo "  2. Try: localStorage.clear() then reload"
    echo "  3. Read: PUBLIC_ACCESS_GUIDE.md"
else
    echo -e "  ${YELLOW}⚠ Some servers are not running${NC}"
    echo ""
    if [ "$backend_ok" = false ]; then
        echo "  Start backend:"
        echo "    cd server && source venv/bin/activate && python run.py"
    fi
    if [ "$frontend_ok" = false ]; then
        echo "  Start frontend:"
        echo "    ./r.py dashboard"
    fi
fi

echo ""
echo "${BLUE}Quick Links:${NC}"
echo "  Frontend: http://localhost:54321"
echo "  Backend:  http://localhost:8000/api/health"
echo "  ngrok:    http://localhost:4040 (if running)"
echo ""
