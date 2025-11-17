# Digital Twin Desktop Development Dockerfile
FROM rust:1.75-slim-bullseye as rust-base

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    curl \
    wget \
    git \
    build-essential \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 18.x
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Tauri CLI
RUN cargo install tauri-cli

# Set up working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src/ ./src/
COPY build.rs ./
COPY tauri.conf.json ./

# Copy UI files
COPY ui/ ./ui/

# Copy scripts
COPY scripts/ ./scripts/
RUN chmod +x ./scripts/*.sh

# Copy environment files
COPY .env.local.example ./

# Install frontend dependencies
WORKDIR /app/ui
RUN npm install

# Return to app directory
WORKDIR /app

# Create data directory
RUN mkdir -p data

# Set up environment
ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

# Expose ports
# Tauri dev server
EXPOSE 1420
# Vite dev server
EXPOSE 5173

# Default command
CMD ["./scripts/dev.sh"]