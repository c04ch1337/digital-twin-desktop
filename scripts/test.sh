#!/bin/bash
set -e

# Digital Twin Desktop Test Runner Script

echo "Running Digital Twin Desktop tests..."

# Parse command line arguments
TEST_TYPE="all"
TEST_FILTER=""
COVERAGE=false

# Process command line arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --unit)
            TEST_TYPE="unit"
            shift
            ;;
        --integration)
            TEST_TYPE="integration"
            shift
            ;;
        --e2e)
            TEST_TYPE="e2e"
            shift
            ;;
        --filter)
            TEST_FILTER="$2"
            shift
            shift
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --help)
            echo "Usage: ./scripts/test.sh [options]"
            echo ""
            echo "Options:"
            echo "  --unit                 Run only unit tests"
            echo "  --integration          Run only integration tests"
            echo "  --e2e                  Run only end-to-end tests"
            echo "  --filter <pattern>     Run tests matching pattern"
            echo "  --coverage             Generate test coverage report"
            echo "  --help                 Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $key"
            echo "Use --help for usage information."
            exit 1
            ;;
    esac
done

# Set up test environment
echo "Setting up test environment..."

# Check if .env.local exists, if not create a test version
if [ ! -f .env.local ]; then
    echo "Creating test environment configuration..."
    cp .env.local.example .env.test
    # Override with test-specific values
    echo "DB_PATH=./data/test.db" >> .env.test
    echo "LOG_LEVEL=debug" >> .env.test
    export $(grep -v '^#' .env.test | xargs)
else
    export $(grep -v '^#' .env.local | xargs)
fi

# Create test database directory if it doesn't exist
mkdir -p data

# Run backend tests
run_backend_tests() {
    local test_args=""
    
    # Set test type filter
    case $TEST_TYPE in
        unit)
            echo "Running unit tests..."
            test_args="--test unit"
            ;;
        integration)
            echo "Running integration tests..."
            test_args="--test integration"
            ;;
        e2e)
            echo "Running end-to-end tests..."
            test_args="--test e2e"
            ;;
        all)
            echo "Running all tests..."
            test_args=""
            ;;
    esac
    
    # Add test filter if specified
    if [ -n "$TEST_FILTER" ]; then
        test_args="$test_args -- $TEST_FILTER"
    fi
    
    # Run tests with or without coverage
    if [ "$COVERAGE" = true ]; then
        echo "Generating test coverage report..."
        # Install cargo-tarpaulin if not already installed
        if ! command -v cargo-tarpaulin &> /dev/null; then
            echo "Installing cargo-tarpaulin..."
            cargo install cargo-tarpaulin
        fi
        
        cargo tarpaulin --out Html $test_args
        echo "Coverage report generated in tarpaulin-report.html"
    else
        cargo test $test_args
    fi
}

# Run frontend tests
run_frontend_tests() {
    echo "Running frontend tests..."
    cd ui
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        npm install
    fi
    
    # Run tests based on type
    case $TEST_TYPE in
        unit)
            echo "Running frontend unit tests..."
            npm test -- --testPathPattern=src/__tests__
            ;;
        e2e)
            echo "Running frontend E2E tests..."
            npm run cypress:run
            ;;
        all)
            echo "Running all frontend tests..."
            npm test
            npm run cypress:run
            ;;
    esac
    
    cd ..
}

# Run the appropriate tests
if [[ "$TEST_TYPE" == "e2e" ]]; then
    # E2E tests require both backend and frontend
    run_frontend_tests
elif [[ "$TEST_TYPE" == "unit" || "$TEST_TYPE" == "integration" ]]; then
    # Run backend tests for unit and integration
    run_backend_tests
else
    # Run all tests
    run_backend_tests
    run_frontend_tests
fi

echo "All tests completed!"