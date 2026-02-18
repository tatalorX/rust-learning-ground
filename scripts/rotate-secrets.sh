#!/bin/bash
# rotate-secrets.sh - Rotate all secrets and remove from git history
# 
# WARNING: This script will:
# 1. Generate new random secrets
# 2. Update the .env file
# 3. Provide instructions for removing old secrets from git history
#
# Run this script after fixing security vulnerabilities

set -e

echo "🔐 Rotating secrets for Rust Learning Ground..."
echo ""

# Generate new secrets
NEW_SECRET_KEY=$(openssl rand -base64 48 | tr -d '\n')
NEW_JWT_SECRET=$(openssl rand -base64 48 | tr -d '\n')
NEW_AGE_KEY=$(age-keygen 2>&1 | grep "Public key" | cut -d: -f2 | tr -d ' ')

echo "✅ Generated new secrets"
echo ""

# Backup current .env
cp server/.env server/.env.backup.$(date +%Y%m%d_%H%M%S)
echo "📁 Backed up current .env file"

# Update .env file with new secrets
sed -i "s/^SECRET_KEY=.*/SECRET_KEY=${NEW_SECRET_KEY}/" server/.env
sed -i "s/^JWT_SECRET_KEY=.*/JWT_SECRET_KEY=${NEW_JWT_SECRET}/" server/.env

echo "✅ Updated server/.env with new secrets"
echo ""

# Generate new age keypair
echo "📝 Generating new age keypair..."
age-keygen -o server/data/keys/master.key.new 2>/dev/null || echo "Note: age-keygen not found. Install age to generate encryption keys."
if [ -f server/data/keys/master.key.new ]; then
    mv server/data/keys/master.key.new server/data/keys/master.key
    echo "✅ Generated new age master key"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "⚠️  IMPORTANT: Remove old secrets from git history"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "The old secrets are still in git history. To completely remove them:"
echo ""
echo "1. Install git-filter-repo (if not already installed):"
echo "   pip install git-filter-repo"
echo ""
echo "2. Remove secret files from history:"
echo "   git filter-repo --force --path server/.env --invert-paths"
echo "   git filter-repo --force --path server/data/keys/master.key --invert-paths"
echo ""
echo "3. Or use BFG Repo-Cleaner (alternative):"
echo "   java -jar bfg.jar --delete-files .env"
echo "   java -jar bfg.jar --delete-files master.key"
echo ""
echo "4. Force push to remote (DANGEROUS - coordinate with team):"
echo "   git push origin --force --all"
echo "   git push origin --force --tags"
echo ""
echo "5. Update any deployed instances with the new secrets from server/.env"
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "📋 Summary of Changes"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "✅ Rotated SECRET_KEY"
echo "✅ Rotated JWT_SECRET_KEY"
echo "✅ Generated new age master key (if age installed)"
echo "✅ Backed up old .env file"
echo ""
echo "⚠️  NEXT STEPS:"
echo "   1. Review the new secrets in server/.env"
echo "   2. Remove old secrets from git history (see above)"
echo "   3. Update production deployments"
echo "   4. Revoke any leaked API keys (GitHub, ngrok, etc.)"
echo ""
