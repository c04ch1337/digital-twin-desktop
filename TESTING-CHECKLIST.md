# Testing Checklist - Pre-Flight Check

**Before starting tests, verify these items:**

---

## ✅ Prerequisites Checklist

### 1. Environment Setup

- [ ] **Rust Toolchain Installed**
  ```bash
  rustc --version  # Should show 1.70+
  cargo --version
  ```

- [ ] **Node.js Installed**
  ```bash
  node --version   # Should show 18+
  npm --version
  ```

- [ ] **Environment Configuration**
  - [ ] `.env.local` exists OR `.env.local.example` exists to copy
  - [ ] Test database path configured
  - [ ] API keys set (if testing LLM integration)
  - [ ] Log level set appropriately

### 2. Dependencies Installed

- [ ] **Backend Dependencies**
  ```bash
  cd digital-twin-desktop
  cargo fetch
  ```

- [ ] **Frontend Dependencies**
  ```bash
  cd ui
  npm install
  ```

- [ ] **Test Tools (Optional but Recommended)**
  ```bash
  # For coverage reports
  cargo install cargo-tarpaulin
  
  # For test watching
  cargo install cargo-watch
  ```

### 3. Test Environment Files

- [ ] **Test Database Directory**
  ```bash
  mkdir -p data
  # Script will create this, but good to verify
  ```

- [ ] **Test Configuration**
  - [ ] `.env.test` will be auto-created if `.env.local` doesn't exist
  - [ ] Or manually create `.env.local` with test settings

### 4. Code Compilation

- [ ] **Backend Compiles**
  ```bash
  cargo build
  ```

- [ ] **Frontend Compiles**
  ```bash
  cd ui
  npm run build
  cd ..
  ```

---

## 🧪 Test Execution Options

### Quick Test Run
```bash
./scripts/test.sh
```

### Specific Test Types
```bash
# Unit tests only
./scripts/test.sh --unit

# Integration tests only
./scripts/test.sh --integration

# E2E tests only
./scripts/test.sh --e2e

# With coverage
./scripts/test.sh --coverage

# Filter specific tests
./scripts/test.sh --filter twin
```

### Manual Test Execution

**Backend Tests:**
```bash
# All tests
cargo test

# Unit tests only
cargo test --test unit

# Integration tests only
cargo test --test integration

# Specific test
cargo test test_name
```

**Frontend Tests:**
```bash
cd ui
npm test
npm run cypress:run  # E2E tests
```

---

## 🔍 What to Check Before Testing

### 1. Test Database
- Tests use a separate test database (`./data/test.db`)
- Ensure write permissions in `data/` directory
- Old test databases will be overwritten

### 2. API Keys (If Testing LLM)
- For unit tests: Mocks are used (no API keys needed)
- For integration tests: May need real API keys
- Check test files to see which require real APIs

### 3. External Services
- **Modbus**: Only needed if testing Modbus tool
- **MQTT**: Only needed if testing MQTT tool
- **LLM APIs**: Only needed for integration tests

### 4. Test Data
- Fixtures are in `tests/fixtures/`
- Mocks are in `tests/mocks/`
- Test helpers in `tests/helpers/`

---

## 🚨 Common Issues & Solutions

### Issue: "No .env.local found"
**Solution:** Script auto-creates `.env.test`, or create `.env.local` manually

### Issue: "Database locked"
**Solution:** Close any running instances, delete `data/test.db`

### Issue: "Module not found" (Frontend)
**Solution:** Run `cd ui && npm install`

### Issue: "Cargo test fails"
**Solution:** 
- Run `cargo clean && cargo build` first
- Check Rust version: `rustc --version`

### Issue: "Port already in use" (E2E tests)
**Solution:** Stop any running dev servers

---

## 📊 Test Coverage Goals

- **Unit Tests**: Core domain logic (80%+ coverage)
- **Integration Tests**: API endpoints, database operations
- **E2E Tests**: Critical user workflows

---

## 🎯 Recommended Test Sequence

1. **Start with Unit Tests**
   ```bash
   ./scripts/test.sh --unit
   ```
   - Fastest
   - No external dependencies
   - Validates core logic

2. **Then Integration Tests**
   ```bash
   ./scripts/test.sh --integration
   ```
   - Requires database
   - May need API keys
   - Tests component interactions

3. **Finally E2E Tests**
   ```bash
   ./scripts/test.sh --e2e
   ```
   - Requires full stack running
   - Slowest but most comprehensive
   - Tests real user scenarios

---

## ✅ Ready to Test?

If all items above are checked, you're ready! Run:

```bash
./scripts/test.sh
```

Or start with unit tests:

```bash
./scripts/test.sh --unit
```

---

## 📝 Notes

- Test database is automatically cleaned between runs
- Mocks are used for external services in unit tests
- Integration tests may require real services
- E2E tests require the full application running

---

**Last Updated:** 2024  
**Version:** 0.1.0

