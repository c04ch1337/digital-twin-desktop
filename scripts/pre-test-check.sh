#!/bin/bash
# Pre-Test Verification Script
# Checks all prerequisites before running tests

set -e

echo "🔍 Pre-Test Verification Checklist"
echo "=================================="
echo ""

ERRORS=0
WARNINGS=0

# Check Rust
echo -n "Checking Rust... "
if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    echo "✅ Rust $RUST_VERSION installed"
else
    echo "❌ Rust not found. Install from https://rustup.rs/"
    ((ERRORS++))
fi

# Check Node.js
echo -n "Checking Node.js... "
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo "✅ Node.js $NODE_VERSION installed"
else
    echo "❌ Node.js not found. Install from https://nodejs.org/"
    ((ERRORS++))
fi

# Check npm
echo -n "Checking npm... "
if command -v npm &> /dev/null; then
    NPM_VERSION=$(npm --version)
    echo "✅ npm $NPM_VERSION installed"
else
    echo "❌ npm not found"
    ((ERRORS++))
fi

# Check backend dependencies
echo -n "Checking backend dependencies... "
if [ -f "Cargo.lock" ]; then
    echo "✅ Cargo.lock found"
else
    echo "⚠️  Cargo.lock not found. Run 'cargo fetch' to install dependencies"
    ((WARNINGS++))
fi

# Check frontend dependencies
echo -n "Checking frontend dependencies... "
if [ -d "ui/node_modules" ]; then
    echo "✅ node_modules found"
else
    echo "⚠️  node_modules not found. Run 'cd ui && npm install'"
    ((WARNINGS++))
fi

# Check environment file
echo -n "Checking environment configuration... "
if [ -f ".env.local" ]; then
    echo "✅ .env.local found"
elif [ -f ".env.local.example" ]; then
    echo "⚠️  .env.local not found, but .env.local.example exists"
    echo "   Test script will auto-create .env.test"
    ((WARNINGS++))
else
    echo "⚠️  No environment file found"
    ((WARNINGS++))
fi

# Check test database directory
echo -n "Checking test data directory... "
if [ -d "data" ]; then
    echo "✅ data/ directory exists"
else
    echo "⚠️  data/ directory will be created by test script"
    ((WARNINGS++))
fi

# Check if backend compiles
echo -n "Checking backend compilation... "
if cargo build --quiet 2>/dev/null; then
    echo "✅ Backend compiles successfully"
else
    echo "❌ Backend compilation failed. Run 'cargo build' to see errors"
    ((ERRORS++))
fi

# Check test script
echo -n "Checking test script... "
if [ -f "scripts/test.sh" ] && [ -x "scripts/test.sh" ]; then
    echo "✅ test.sh exists and is executable"
else
    echo "⚠️  test.sh not found or not executable"
    ((WARNINGS++))
fi

echo ""
echo "=================================="
echo "Summary:"
echo "  Errors: $ERRORS"
echo "  Warnings: $WARNINGS"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo "❌ Please fix errors before running tests"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo "⚠️  Some warnings found, but tests may still run"
    echo "   Review warnings above"
    exit 0
else
    echo "✅ All checks passed! Ready to run tests."
    echo ""
    echo "Run tests with:"
    echo "  ./scripts/test.sh              # All tests"
    echo "  ./scripts/test.sh --unit       # Unit tests only"
    echo "  ./scripts/test.sh --integration # Integration tests only"
    exit 0
fi

