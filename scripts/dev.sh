#!/bin/bash
set -e

# Digital Twin Desktop Development Server Script

echo "Starting Digital Twin Desktop development server..."

# Check if .env.local exists
if [ ! -f .env.local ]; then
    echo "Error: .env.local file not found."
    echo "Please run ./scripts/setup.sh first or create .env.local manually."
    exit 1
fi

# Load environment variables
export $(grep -v '^#' .env.local | xargs)

# Check if the data directory exists
if [ ! -d data ]; then
    echo "Creating data directory..."
    mkdir -p data
fi

# Check if the database exists
if [ ! -f data/digital_twin.db ]; then
    echo "Initializing database..."
    touch data/digital_twin.db
fi

# Check if we're in a Docker environment
if [ -f /.dockerenv ]; then
    echo "Running in Docker environment..."
    # Docker-specific configurations
    export TAURI_DEV_SERVER_PORT=${TAURI_DEV_SERVER_PORT:-1420}
    export VITE_DEV_SERVER_PORT=${VITE_DEV_SERVER_PORT:-5173}
fi

# Start the development server with cargo-watch for auto-reloading
echo "Starting Tauri development server..."

# Check if we should run in debug mode
if [ "$1" == "--debug" ]; then
    echo "Running in debug mode..."
    cargo watch -x "tauri dev -- --features=debug"
else
    # Start the development server
    cargo tauri dev
fi

# Note: The script will not reach this point unless the development server is stopped
echo "Development server stopped."