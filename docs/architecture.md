# Digital Twin Desktop Architecture

## Overview

Digital Twin Desktop is a cross-platform desktop application built with Rust and Tauri, designed to create, monitor, and interact with digital twins of physical systems. The application follows Clean Architecture principles to ensure maintainability, testability, and flexibility.

## Architectural Principles

### Clean Architecture

The application is structured according to Clean Architecture (also known as Onion or Hexagonal Architecture), with the following layers:

1. **Core Domain Layer** (Innermost)
   - Contains pure business logic and domain models
   - Has no external dependencies
   - Defines interfaces that outer layers implement

2. **Application Layer**
   - Contains use cases that orchestrate domain logic
   - Depends only on the Core Domain layer
   - Implements business workflows

3. **Infrastructure Layer**
   - Contains implementations of interfaces defined in Core
   - Handles external integrations (LLM APIs, database, file system)
   - Adapts external libraries to work with our domain

4. **API Layer**
   - Contains Tauri commands for frontend-backend communication
   - Depends on the Application layer
   - Handles serialization/deserialization of data

### Dependency Rule

The fundamental rule of Clean Architecture is that dependencies can only point inward:

- Inner layers never depend on outer layers
- Outer layers depend on inner layers through interfaces (traits)
- Domain models are pure and don't contain infrastructure concerns

This ensures that the core business logic remains isolated from external concerns and can be tested independently.

## Component Structure

```
src/
├── core/               # Domain and Application Layers
│   ├── domain/         # Domain models and business logic
│   │   ├── models/     # Entity definitions
│   │   ├── traits/     # Core interfaces
│   │   ├── errors.rs   # Domain-specific errors
│   │   └── value_objects.rs
│   └── application/    # Application Layer
│       ├── use_cases/  # Business workflows
│       ├── services/   # Domain services
│       ├── commands.rs # Command objects
│       ├── queries.rs  # Query objects
│       ├── dtos.rs     # Data Transfer Objects
│       └── events.rs   # Domain events
├── infrastructure/     # Infrastructure Layer
│   ├── llm/            # LLM provider implementations
│   ├── db/             # Database implementations
│   │   ├── repositories/ # Repository implementations
│   │   └── migrations/   # Database migrations
│   ├── tools/          # Tool implementations
│   ├── security/       # Security implementations
│   ├── config.rs       # Configuration management
│   └── logging.rs      # Logging infrastructure
└── api/                # API Layer (Tauri Commands)
    ├── commands/       # Command handlers
    ├── middleware/     # API middleware
    ├── dto.rs          # API-specific DTOs
    └── error.rs        # API error handling
```

## Key Components

### Domain Models

Domain models represent the core business entities and their behavior:

- **Agent**: Represents an AI agent that can process user requests and execute tools
- **Conversation**: Represents a conversation between a user and an agent
- **DigitalTwin**: Represents a digital twin of a physical system
- **SensorData**: Represents data from sensors in the physical system
- **Tool**: Represents a tool that an agent can use

Domain models encapsulate business rules and validation logic, ensuring that the system maintains a valid state.

### Use Cases

Use cases implement specific business operations:

- **CreateConversation**: Creates a new conversation
- **SendMessage**: Sends a message in a conversation
- **CreateTwin**: Creates a new digital twin
- **SyncTwin**: Synchronizes a digital twin with its physical counterpart
- **RunSimulation**: Runs a simulation on a digital twin
- **ExecuteTool**: Executes a tool on behalf of an agent

Each use case follows the Single Responsibility Principle and orchestrates domain models and repositories to accomplish a specific task.

### Repositories

Repositories provide an abstraction over data storage:

- **ConversationRepository**: Stores and retrieves conversations
- **TwinRepository**: Stores and retrieves digital twins
- **SensorDataRepository**: Stores and retrieves sensor data
- **AgentRepository**: Stores and retrieves agent configurations
- **ToolRepository**: Stores and retrieves tool configurations

Repositories are defined as traits in the domain layer and implemented in the infrastructure layer, allowing the domain to remain independent of storage concerns.

### Services

Services implement domain logic that doesn't naturally fit within a single entity:

- **ConversationService**: Manages conversation state and history
- **TwinService**: Manages digital twin operations
- **AgentService**: Manages agent behavior and tool execution
- **SimulationService**: Manages simulation execution
- **ToolService**: Manages tool registration and execution

Services coordinate between multiple domain models and repositories to implement complex business logic.

### LLM Integration

The application integrates with Large Language Models (LLMs) to power the agent system:

- **LlmClient**: Trait defining the interface for LLM providers
- **AnthropicClient**: Implementation for Anthropic's Claude API
- **OpenAIClient**: Implementation for OpenAI's GPT API

The LLM integration follows the Adapter pattern, allowing the application to switch between different LLM providers without changing the core logic.

### Tool System

The tool system allows agents to interact with the outside world:

- **Tool**: Trait defining the interface for tools
- **ToolRegistry**: Manages tool registration and discovery
- **ToolExecutor**: Executes tools with proper sandboxing and error handling

Tools are implemented in the infrastructure layer but defined in the domain layer, allowing the domain to specify what tools can do without depending on specific implementations.

### Database

The application uses SQLite for local data storage:

- **SqliteConnection**: Manages database connections
- **Migrations**: Handles database schema evolution
- **Repositories**: Implement data access logic

The database is accessed through repository interfaces, allowing the application to switch to a different database technology if needed.

### API Layer

The API layer exposes functionality to the frontend through Tauri commands:

- **ConversationCommands**: Commands for conversation management
- **TwinCommands**: Commands for digital twin management
- **AgentCommands**: Commands for agent configuration
- **SimulationCommands**: Commands for simulation control
- **ToolCommands**: Commands for tool management

Commands validate inputs, delegate to use cases, and transform outputs into a format suitable for the frontend.

## Communication Flow

1. User interacts with the React frontend
2. Frontend calls Tauri commands
3. Commands validate inputs and call appropriate use cases
4. Use cases orchestrate domain models and repositories
5. Repositories interact with the database or external services
6. Results flow back through the same layers
7. Frontend updates based on the results

## Error Handling

The application uses a layered approach to error handling:

1. **Domain Errors**: Defined in the domain layer, represent business rule violations
2. **Application Errors**: Wrap domain errors and add application-specific context
3. **Infrastructure Errors**: Wrap external errors (e.g., database errors) and map them to domain concepts
4. **API Errors**: Map all errors to user-friendly messages for the frontend

Each layer has its own error types and handles errors appropriate to its level of abstraction.

## Configuration Management

Configuration is managed through:

1. **Environment Variables**: For deployment-specific settings
2. **Configuration Files**: For user-configurable settings
3. **Database Settings**: For persistent user preferences

The configuration system follows the Options pattern, with sensible defaults that can be overridden.

## Security Considerations

The application implements several security measures:

1. **API Key Encryption**: Encrypts API keys at rest
2. **Sandboxed Tool Execution**: Limits what tools can do
3. **Input Validation**: Validates all inputs before processing
4. **Permission System**: Controls access to sensitive operations
5. **Rate Limiting**: Prevents abuse of external APIs

## Performance Considerations

Performance is optimized through:

1. **Connection Pooling**: Reuses database connections
2. **Caching**: Caches expensive operations
3. **Async Processing**: Uses Tokio for asynchronous operations
4. **Streaming Responses**: Streams LLM responses for better UX
5. **Efficient Data Structures**: Uses appropriate data structures for different operations

## Testing Strategy

The application is tested at multiple levels:

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test components working together
3. **End-to-End Tests**: Test complete user workflows
4. **Property-Based Tests**: Test invariants across random inputs

Tests are organized to mirror the production code structure, making it easy to find tests for specific components.

## Deployment Architecture

The application can be deployed in two modes:

1. **Desktop Application**: Runs as a standalone desktop application
2. **Headless Server**: Runs as a server without a UI, accessible via API

Both modes share the same core logic but have different entry points and configuration options.

## Future Extensibility

The architecture is designed for extensibility:

1. **Plugin System**: Allows adding new tools and capabilities
2. **Multiple LLM Support**: Can switch between different LLM providers
3. **Custom Twin Types**: Can define custom digital twin types
4. **Alternative Storage**: Can use different storage backends
5. **Cloud Synchronization**: Can synchronize data with cloud services

## Architectural Decision Records

Major architectural decisions are documented in ADRs (Architectural Decision Records) in the `docs/adr` directory, providing context and rationale for significant architectural choices.