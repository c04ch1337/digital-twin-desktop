# Contributing to Digital Twin Desktop

Thank you for your interest in contributing to Digital Twin Desktop! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please read it before contributing.

## Getting Started

### Development Environment Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/digital-twin-desktop.git
   cd digital-twin-desktop
   ```
3. Set up the development environment:
   ```bash
   ./scripts/setup.sh
   ```
4. Create a `.env.local` file based on the example:
   ```bash
   cp .env.local.example .env.local
   ```
   Then edit the file to add your API keys and configuration.

### Development Workflow

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
   or
   ```bash
   git checkout -b fix/issue-you-are-fixing
   ```

2. Make your changes, following the coding standards and guidelines below.

3. Run the tests to ensure your changes don't break existing functionality:
   ```bash
   ./scripts/test.sh
   ```

4. Commit your changes with a descriptive commit message:
   ```bash
   git commit -m "Add feature: your feature description"
   ```

5. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

6. Open a pull request against the main repository.

## Coding Standards

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` to format your code
- Run `clippy` to catch common mistakes and improve your code
- Write documentation for public APIs
- Include unit tests for new functionality

### TypeScript/React Code

- Follow the [TypeScript Coding Guidelines](https://github.com/microsoft/TypeScript/wiki/Coding-guidelines)
- Use ESLint and Prettier for code formatting
- Use functional components with hooks
- Write meaningful component and function names
- Include unit tests for components and utilities

### Commit Messages

- Use the imperative mood ("Add feature" not "Added feature")
- Start with a capital letter
- Keep the first line under 72 characters
- Reference issues and pull requests where appropriate

## Pull Request Process

1. Update the README.md or documentation with details of changes if appropriate
2. Update the CHANGELOG.md with details of changes
3. The PR should work on the main development branches
4. Include tests that cover your changes
5. Get approval from at least one maintainer

## Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting a pull request
- Include integration tests for complex features
- Test on multiple platforms if possible (Windows, macOS, Linux)

## Documentation

- Update documentation for any changed functionality
- Document new features thoroughly
- Use clear, concise language
- Include examples where appropriate

## Issue Reporting

- Use the issue tracker to report bugs
- Include detailed steps to reproduce the issue
- Mention your operating system and application version
- Attach screenshots or error logs if available

## Feature Requests

- Use the issue tracker to suggest features
- Clearly describe the problem the feature would solve
- Suggest a solution if possible
- Discuss the feature with maintainers before implementing

## Code Review

All submissions require review. We use GitHub pull requests for this purpose.

## License

By contributing to Digital Twin Desktop, you agree that your contributions will be licensed under the project's MIT License.