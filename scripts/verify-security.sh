#!/bin/bash
# verify-security.sh - Verify security hardening is complete
#
# Run this script to verify all security fixes have been applied

set -e

echo "🔒 Rust Learning Ground - Security Verification"
echo "================================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0

# Function to check if a file exists and is not tracked by git
check_untracked() {
    local file=$1
    local desc=$2
    
    if git check-ignore -q "$file" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} $desc is properly ignored by git"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $desc is NOT ignored by git (CRITICAL)"
        ((FAIL++))
    fi
}

# Function to check if a file contains a specific pattern
check_pattern() {
    local file=$1
    local pattern=$2
    local desc=$3
    
    if grep -q "$pattern" "$file" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} $desc"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $desc NOT FOUND"
        ((FAIL++))
    fi
}

echo "1. Checking Secret Files are Ignored..."
echo "----------------------------------------"
check_untracked "server/.env" ".env file"
check_untracked "server/data/keys/master.key" "master.key"
check_untracked "secrets/prod.yaml" "production secrets"
echo ""

echo "2. Checking Security Fixes Applied..."
echo "--------------------------------------"

# Check password reset token hashing
check_pattern "server/app/email_service.py" "hashlib.sha256(token.encode()).hexdigest()" "Password reset token hashing (C-04)"

# Check SQL injection fix
check_pattern "server/app/routers/admin.py" "safe_search = search.replace" "SQL injection fix (C-05)"

# Check subprocess fallback gating
check_pattern "server/app/code_runner.py" "Subprocess fallback attempted in production" "Subprocess fallback gating (C-05)"

# Check nsjail socket syscalls removed
check_pattern "deploy/nsjail/rust_exec.cfg" "DENY.*socket" "Socket syscalls denied (C-14)"

# Check open redirect fix
check_pattern "server/app/security.py" "url.startswith\('//'\)" "Open redirect fix (H-04)"

# Check CSRF rate limiting
check_pattern "server/app/main.py" "limiter.limit.*30/minute" "CSRF rate limiting (H-05)"

# Check detailed health auth
check_pattern "server/app/main.py" "admin: User = Depends(require_admin)" "Detailed health auth (H-01)"

# Check WebSocket stats auth
check_pattern "server/app/websocket_manager.py" "admin: User = Depends(require_admin)" "WebSocket stats auth (H-02)"

# Check pickle removed
check_pattern "server/app/cache_service.py" "SECURITY FIX.*pickle" "Pickle removed (H-06)"

# Check MD5→SHA-256
check_pattern "server/app/cache_service.py" "sha256.*key_parts" "MD5→SHA-256 (H-07)"

# Check XSS fix in editor
check_pattern "web/editor.html" "escapeHtml.*text" "XSS fix in editor (C-04)"

echo ""

echo "3. Checking License..."
echo "----------------------"
if grep -q "AGPL-3.0" LICENSE 2>/dev/null; then
    echo -e "${GREEN}✓${NC} License is AGPL-3.0"
    ((PASS++))
else
    echo -e "${RED}✗${NC} License should be AGPL-3.0"
    ((FAIL++))
fi
echo ""

echo "4. Checking Deployment Files..."
echo "--------------------------------"
if [ -f "compose.prod.yml" ]; then
    echo -e "${GREEN}✓${NC} compose.prod.yml exists"
    ((PASS++))
else
    echo -e "${RED}✗${NC} compose.prod.yml missing"
    ((FAIL++))
fi

if [ -f "Caddyfile" ]; then
    echo -e "${GREEN}✓${NC} Caddyfile exists"
    ((PASS++))
else
    echo -e "${RED}✗${NC} Caddyfile missing"
    ((FAIL++))
fi

if [ -f "deploy/nsjail/rust_exec.cfg" ]; then
    echo -e "${GREEN}✓${NC} nsjail config exists"
    ((PASS++))
else
    echo -e "${RED}✗${NC} nsjail config missing"
    ((FAIL++))
fi

if [ -f ".sops.yaml" ]; then
    echo -e "${GREEN}✓${NC} SOPS config exists"
    ((PASS++))
else
    echo -e "${RED}✗${NC} SOPS config missing"
    ((FAIL++))
fi
echo ""

echo "5. Secret Rotation Status..."
echo "-----------------------------"
if [ -f "server/.env.backup.*" ]; then
    echo -e "${YELLOW}!${NC} Backup of old .env exists - secrets were rotated"
    echo "   Make sure to remove old secrets from git history if they were ever committed"
else
    echo -e "${YELLOW}!${NC} No backup found - ensure secrets have been rotated manually"
fi
echo ""

# Summary
echo "================================================"
echo "Verification Complete"
echo "================================================"
echo -e "${GREEN}Passed: $PASS${NC}"
echo -e "${RED}Failed: $FAIL${NC}"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}✓ All security checks passed!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Review DEPLOYMENT_CHECKLIST.md"
    echo "  2. Install nsjail on production servers"
    echo "  3. Set up PostgreSQL database"
    echo "  4. Configure TLS certificates"
    echo "  5. Deploy using: podman-compose -f compose.prod.yml up -d"
    exit 0
else
    echo -e "${RED}✗ Some security checks failed!${NC}"
    echo "Please review the failures above and fix them before deploying."
    exit 1
fi
