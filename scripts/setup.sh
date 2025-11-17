#!/bin/bash
set -e

# Digital Twin Desktop Development Environment Setup Script

echo "Setting up Digital Twin Desktop development environment..."

# Create necessary directories
mkdir -p .vscode
mkdir -p .github/workflows
mkdir -p data

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed. Updating..."
    rustup update
fi

# Install Rust components
rustup component add rustfmt clippy

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Node.js not found. Please install Node.js (v18 or later) manually."
    echo "Visit https://nodejs.org/ for installation instructions."
    exit 1
else
    NODE_VERSION=$(node -v | cut -d 'v' -f 2)
    NODE_MAJOR_VERSION=$(echo "$NODE_VERSION" | cut -d '.' -f 1)
    
    if [ "$NODE_MAJOR_VERSION" -lt 18 ]; then
        echo "Node.js version $NODE_VERSION is too old. Please upgrade to v18 or later."
        exit 1
    else
        echo "Node.js v$NODE_VERSION is installed."
    fi
fi

# Install Tauri CLI
echo "Installing Tauri CLI..."
cargo install tauri-cli

# Install frontend dependencies
echo "Installing frontend dependencies..."
cd ui
npm install
cd ..

# Create .env.local if it doesn't exist
if [ ! -f .env.local ]; then
    echo "Creating .env.local from example..."
    cp .env.local.example .env.local
    echo "Please edit .env.local to add your API keys and configuration."
fi

# Create SQLite database directory
echo "Setting up database..."
mkdir -p data
touch data/digital_twin.db

# Install development tools
echo "Installing additional development tools..."
cargo install cargo-watch
cargo install cargo-audit

# Check for Docker
if command -v docker &> /dev/null; then
    echo "Docker is installed. You can use the Docker development environment."
    echo "Run 'docker-compose up -d' to start the containerized environment."
else
    echo "Docker not found. If you want to use the containerized development environment,"
    echo "please install Docker and Docker Compose."
fi

echo "Development environment setup complete!"
echo "To start the development server, run: ./scripts/dev.sh"