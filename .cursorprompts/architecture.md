# Digital Twin Desktop Architecture Prompt

## Overview
This prompt provides guidance for maintaining and extending the Digital Twin Desktop application architecture. Use this when working on architectural changes, adding new components, or refactoring existing code.

## Clean Architecture Principles

The Digital Twin Desktop follows a Clean Architecture (Onion/Hexagonal) approach with the following layers:

1. **Core Domain Layer** (Innermost)
   - Contains pure business logic and domain models
   - No external dependencies
   - Defines interfaces (traits) that outer layers implement

2. **Application Layer**
   - Contains use cases that orchestrate domain logic
   - Depends only on the Core Domain layer
   - Implements business workflows and application logic

3. **Infrastructure Layer**
   - Contains implementations of interfaces defined in Core
   - Handles external integrations (LLM APIs, database, file system)
   - Adapts external libraries to work with our domain

4. **API Layer**
   - Contains Tauri commands for frontend-backend communication
   - Depends on the Application layer
   - Handles serialization/deserialization of data

## Dependency Rules

1. Inner layers NEVER depend on outer layers
2. Outer layers depend on inner layers through interfaces (traits)
3. Use dependency injection via trait objects or generics
4. Domain models should be pure and not contain infrastructure concerns

## Component Organization

```
src/
├── core/               # Domain Layer
│   ├── domain/         # Domain models and business logic
│   │   ├── models/     # Entity definitions
│   │   └── traits/     # Core interfaces
│   └── application/    # Application Layer
│       ├── use_cases/  # Business workflows
│       └── services/   # Domain services
├── infrastructure/     # Infrastructure Layer
│   ├── llm/            # LLM provider implementations
│   ├── db/             # Database implementations
│   ├── tools/          # Tool implementations
│   └── security/       # Security implementations
└── api/                # API Layer (Tauri Commands)
    └── commands/       # Command handlers
```

## Design Patterns to Apply

1. **Repository Pattern**
   - Abstract data access behind repository interfaces
   - Define repositories in core, implement in infrastructure

2. **Use Case Pattern**
   - Each business operation is a separate use case class
   - Use cases orchestrate domain logic and repositories

3. **Dependency Injection**
   - Pass dependencies through constructors
   - Use trait objects for polymorphism

4. **Factory Pattern**
   - Create factories for complex object creation
   - Keep construction logic separate from business logic

5. **State Pattern**
   - Model agent and twin states explicitly
   - Use enums for state representation

## Architecture Decision Guidelines

When making architectural decisions, consider:

1. **Maintainability**
   - Will this be easy to understand and modify?
   - Does it follow established patterns in the codebase?

2. **Testability**
   - Can components be tested in isolation?
   - Are dependencies easily mockable?

3. **Performance**
   - Are there potential bottlenecks?
   - Is resource usage efficient?

4. **Security**
   - Are security concerns properly addressed?
   - Is user data protected?

5. **Scalability**
   - Will this solution scale with increasing data/users?
   - Are there potential concurrency issues?

## Example: Adding a New Feature

When adding a new feature:

1. Start by defining domain models and interfaces in the Core layer
2. Implement use cases in the Application layer
3. Create infrastructure implementations
4. Add Tauri commands in the API layer
5. Connect to the frontend

Example for adding a new "Export Conversation" feature:

```rust
// 1. Core Domain (src/core/domain/models/export.rs)
pub enum ExportFormat {
    Markdown,
    JSON,
    HTML,
}

// 2. Core Interface (src/core/domain/traits/exporter.rs)
pub trait ConversationExporter {
    fn export(&self, conversation_id: &ConversationId, format: ExportFormat) -> Result<Vec<u8>, ExportError>;
}

// 3. Application Layer (src/core/application/use_cases/export_conversation.rs)
pub struct ExportConversationUseCase<E: ConversationExporter, R: ConversationRepository> {
    exporter: E,
    repository: R,
}

impl<E: ConversationExporter, R: ConversationRepository> ExportConversationUseCase<E, R> {
    pub fn new(exporter: E, repository: R) -> Self {
        Self { exporter, repository }
    }
    
    pub fn execute(&self, conversation_id: &str, format: ExportFormat) -> Result<Vec<u8>, ApplicationError> {
        let conv_id = ConversationId::from_string(conversation_id)?;
        Ok(self.exporter.export(&conv_id, format)?)
    }
}

// 4. Infrastructure Layer (src/infrastructure/exporters/markdown_exporter.rs)
pub struct MarkdownExporter<R: ConversationRepository> {
    repository: R,
}

impl<R: ConversationRepository> ConversationExporter for MarkdownExporter<R> {
    fn export(&self, conversation_id: &ConversationId, format: ExportFormat) -> Result<Vec<u8>, ExportError> {
        // Implementation
    }
}

// 5. API Layer (src/api/commands/conversation.rs)
#[tauri::command]
pub async fn export_conversation(
    state: State<'_, AppState>,
    conversation_id: String,
    format: String,
) -> Result<Vec<u8>, String> {
    // Implementation
}
```

## Architecture Review Checklist

When reviewing architectural changes, check:

- [ ] Does it follow the dependency rules?
- [ ] Are concerns properly separated?
- [ ] Are interfaces defined in the appropriate layers?
- [ ] Is error handling consistent?
- [ ] Are components testable in isolation?
- [ ] Is the design flexible enough for future changes?
- [ ] Is performance considered?
- [ ] Are security concerns addressed?