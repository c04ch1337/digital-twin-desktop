# Quick Start - Testing Guide

**Get started with testing in 3 steps**

---

## 🚀 Quick Start (3 Steps)

### Step 1: Install Backend Dependencies
```bash
cd digital-twin-desktop
cargo fetch
```

### Step 2: Verify Setup
```bash
./scripts/pre-test-check.sh
```

### Step 3: Run Tests
```bash
# All tests
./scripts/test.sh

# Or start with unit tests (fastest)
./scripts/test.sh --unit
```

---

## ✅ What You Have

- ✅ Rust installed (1.91.1)
- ✅ Node.js installed (v20.19.5)
- ✅ Frontend dependencies installed
- ✅ Test structure in place
- ✅ Test scripts ready

## ⚠️ What You Need

- ⚠️ Backend dependencies: Run `cargo fetch`
- ⚠️ Environment file: `.env.local` or will auto-create `.env.test`

---

## 📋 Complete Checklist

Run this to verify everything:

```bash
./scripts/pre-test-check.sh
```

This will check:
- ✅ Rust & Cargo installed
- ✅ Node.js & npm installed
- ✅ Backend dependencies
- ✅ Frontend dependencies
- ✅ Environment configuration
- ✅ Backend compilation
- ✅ Test script availability

---

## 🧪 Test Commands

### Run All Tests
```bash
./scripts/test.sh
```

### Run Specific Test Types
```bash
# Unit tests (fast, no external deps)
./scripts/test.sh --unit

# Integration tests (requires database)
./scripts/test.sh --integration

# E2E tests (requires full stack)
./scripts/test.sh --e2e
```

### With Coverage
```bash
./scripts/test.sh --coverage
```

### Filter Tests
```bash
./scripts/test.sh --filter twin
```

---

## 🔧 If Something Fails

### Backend Won't Compile
```bash
cargo clean
cargo build
```

### Frontend Tests Fail
```bash
cd ui
npm install
npm test
```

### Database Issues
```bash
# Remove test database
rm -f data/test.db
mkdir -p data
```

---

## 📊 Expected Test Structure

```
tests/
├── unit/              # Fast, isolated tests
│   ├── domain/        # Domain model tests
│   ├── application/   # Service tests
│   └── infrastructure/
├── integration/       # Component interaction tests
│   ├── api/          # API endpoint tests
│   ├── db/           # Database tests
│   └── llm/          # LLM integration tests
└── e2e/              # Full workflow tests
```

---

## 🎯 Recommended First Run

1. **Start with unit tests** (fastest, most reliable):
   ```bash
   ./scripts/test.sh --unit
   ```

2. **Then integration tests**:
   ```bash
   ./scripts/test.sh --integration
   ```

3. **Finally E2E tests** (if needed):
   ```bash
   ./scripts/test.sh --e2e
   ```

---

## 📝 Notes

- **Unit tests** use mocks - no API keys needed
- **Integration tests** may need real API keys (check test files)
- **E2E tests** require full application running
- Test database is auto-created and cleaned
- Coverage reports go to `tarpaulin-report.html`

---

## ✅ Ready?

Run the verification:
```bash
./scripts/pre-test-check.sh
```

If all checks pass, you're ready to test! 🎉

---

**Last Updated:** 2024  
**Version:** 0.1.0

