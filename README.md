# Digital Twin Desktop

A cross-platform desktop application for creating, managing, and interacting with digital twins of physical systems and devices.

## Overview

Digital Twin Desktop is a Tauri-based application that combines a Rust backend with a React frontend to provide a powerful environment for digital twin simulation and management. The application enables users to:

- Create digital representations of physical systems
- Connect to real-world devices via industrial protocols (Modbus, MQTT)
- Simulate system behavior using AI-powered agents
- Visualize sensor data and system states
- Interact with digital twins through natural language conversations

## Features

- **Digital Twin Management**: Create, configure, and manage digital twins of physical systems
- **Real-time Data Integration**: Connect to physical devices via Modbus and MQTT protocols
- **AI-powered Simulation**: Leverage OpenAI and Anthropic models for intelligent simulation
- **Interactive Visualization**: Monitor and visualize system states and sensor data
- **Conversational Interface**: Interact with digital twins using natural language
- **Secure Environment**: End-to-end encryption and sandboxed tool execution

## Technology Stack

### Backend
- **Tauri**: Cross-platform application framework
- **Rust**: Systems programming language for performance and safety
- **SQLite**: Embedded database for local data storage
- **Tokio**: Asynchronous runtime for Rust

### Frontend
- **React**: UI library for building interactive interfaces
- **TypeScript**: Type-safe JavaScript for frontend development
- **Vite**: Next-generation frontend tooling

### AI/LLM Integration
- **OpenAI API**: Integration with GPT models
- **Anthropic API**: Integration with Claude models

### Industrial Protocols
- **Modbus**: Communication with industrial devices
- **MQTT**: Lightweight messaging protocol for IoT devices

## Getting Started

### Prerequisites

- Rust (1.75.0 or later)
- Node.js (18.0.0 or later)
- npm (9.0.0 or later)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-organization/digital-twin-desktop.git
   cd digital-twin-desktop
   ```

2. Set up the development environment:
   ```bash
   ./scripts/setup.sh
   ```

3. Create a `.env.local` file based on the example:
   ```bash
   cp .env.local.example .env.local
   ```
   Then edit the file to add your API keys and configuration.

### Development

Start the development server:
```bash
./scripts/dev.sh
```

This will launch both the Tauri backend and the React frontend in development mode.

### Building for Production

Build the application for production:
```bash
./scripts/build.sh
```

The built application will be available in the `target/release` directory.

### Testing

Run the test suite:
```bash
./scripts/test.sh
```

## Docker Development Environment

A Docker environment is provided for consistent development across different machines:

```bash
docker-compose up -d
```

This will start a containerized development environment with all dependencies pre-installed.

## Project Structure

```
digital-twin-desktop/
├── src/                    # Rust backend code
│   ├── api/                # API layer (Tauri commands)
│   ├── core/               # Core domain and application logic
│   └── infrastructure/     # External services integration
├── ui/                     # React frontend
│   ├── src/                # Frontend source code
│   ├── public/             # Static assets
│   └── index.html          # HTML entry point
├── scripts/                # Development and build scripts
├── tests/                  # Test suites
└── .github/                # GitHub workflows and templates
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to this project.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each release.