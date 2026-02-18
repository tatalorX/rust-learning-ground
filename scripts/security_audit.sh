#!/bin/bash
# Security Audit Script
# Run this script locally to check for security issues

set -e

echo "🔒 Running Security Audit..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

cd "$(dirname "$0")/.."

# Check if virtual environment is activated
if [ -z "$VIRTUAL_ENV" ]; then
    echo -e "${YELLOW}⚠️  Warning: Virtual environment not activated${NC}"
    echo "   Activate with: source venv/bin/activate"
    echo ""
fi

# Install pip-audit if not present
echo "📦 Checking pip-audit..."
if ! command -v pip-audit &> /dev/null; then
    pip install pip-audit
fi

# Run pip-audit
echo ""
echo "🔍 Scanning Python dependencies..."
if pip-audit -r requirements.txt --severity-level high --desc; then
    echo -e "${GREEN}✅ No high-severity vulnerabilities found${NC}"
else
    echo -e "${YELLOW}⚠️  Some vulnerabilities detected - review above${NC}"
fi

# Check for secrets in code
echo ""
echo "🔍 Checking for hardcoded secrets..."
if command -v git &> /dev/null && git rev-parse --git-dir &> /dev/null; then
    # Check for common secret patterns
    PATTERNS="password|secret|key|token"
    MATCHES=$(git grep -i -E "$PATTERNS" -- '*.py' '*.js' '*.html' '*.md' | grep -v "example\|template\|test\|README" | head -20 || true)
    
    if [ -n "$MATCHES" ]; then
        echo -e "${YELLOW}⚠️  Potential secrets found (review manually):${NC}"
        echo "$MATCHES"
    else
        echo -e "${GREEN}✅ No obvious secrets found${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Not a git repository, skipping secret scan${NC}"
fi

# Check for TODO/FIXME comments
echo ""
echo "🔍 Checking for TODO/FIXME comments..."
if command -v git &> /dev/null && git rev-parse --git-dir &> /dev/null; then
    TODOS=$(git grep -i -E "TODO|FIXME|XXX|HACK" -- '*.py' '*.js' | head -10 || true)
    if [ -n "$TODOS" ]; then
        echo -e "${YELLOW}⚠️  TODO/FIXME comments found (may indicate incomplete security work):${NC}"
        echo "$TODOS"
    else
        echo -e "${GREEN}✅ No TODO/FIXME comments found${NC}"
    fi
fi

# Check file permissions
echo ""
echo "🔍 Checking file permissions..."
SENSITIVE_FILES="app/config.py app/security.py app/auth_service.py"
for file in $SENSITIVE_FILES; do
    if [ -f "$file" ]; then
        PERMS=$(stat -c "%a" "$file" 2>/dev/null || stat -f "%A" "$file" 2>/dev/null || echo "unknown")
        if [ "$PERMS" = "644" ] || [ "$PERMS" = "600" ] || [ "$PERMS" = "644" ]; then
            echo -e "${GREEN}✅ $file permissions OK ($PERMS)${NC}"
        else
            echo -e "${YELLOW}⚠️  $file permissions: $PERMS (expected 644)${NC}"
        fi
    fi
done

echo ""
echo "========================================"
echo "✅ Security audit complete!"
echo ""
echo "Additional manual checks:"
echo "  - Review .env files are in .gitignore"
echo "  - Verify CORS origins are restricted in production"
echo "  - Check SECRET_KEY is properly set"
echo "  - Ensure DEBUG=false in production"
echo "========================================"
