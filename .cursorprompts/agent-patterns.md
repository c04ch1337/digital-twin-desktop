# Digital Twin Desktop Agent Patterns

## Overview
This prompt provides guidance for implementing and extending the agent system in the Digital Twin Desktop application. Use this when working on agent-related features, improving agent capabilities, or debugging agent behavior.

## Agent Architecture

The Digital Twin Desktop uses a sophisticated agent architecture with the following components:

1. **Orchestrator**
   - Central coordinator of agent activities
   - Manages the agent's state machine
   - Handles message routing and tool execution

2. **State Machine**
   - Explicitly models agent states
   - Manages transitions between states
   - Ensures consistent behavior

3. **Context Manager**
   - Builds and maintains the agent's context
   - Handles token counting and context window management
   - Implements intelligent context truncation

4. **Reasoning Engine**
   - Implements planning capabilities
   - Handles task decomposition
   - Manages self-reflection and learning

5. **Tool System**
   - Provides a registry of available tools
   - Handles tool discovery and registration
   - Manages tool execution and sandboxing

## Agent State Machine

The agent follows a state machine pattern with these primary states:

```
┌─────────┐      ┌──────────┐      ┌───────────────┐
│  Idle   │─────▶│ Thinking │─────▶│ ExecutingTool │
└─────────┘      └──────────┘      └───────────────┘
     ▲                │                    │
     │                │                    │
     │                ▼                    │
     │          ┌──────────┐              │
     └──────────│ Responding│◀─────────────┘
                └──────────┘
```

State transitions should be explicit and logged. Each state has specific responsibilities:

- **Idle**: Waiting for user input
- **Thinking**: Processing user input, planning next actions
- **ExecutingTool**: Running a specific tool with provided inputs
- **Responding**: Generating and sending response to the user

## Agent Prompt Engineering

### System Prompt Structure

The system prompt should include:

1. **Role Definition**: Clear description of the agent's purpose
2. **Capabilities**: What the agent can and cannot do
3. **Tool Descriptions**: Detailed information about available tools
4. **Reasoning Framework**: How the agent should approach problems
5. **Output Format**: Expected structure of responses

Example system prompt template:

```
You are a Digital Twin Assistant, designed to help users monitor, simulate, and interact with digital twins of physical systems.

# Capabilities
- Monitor sensor data from digital twins
- Run simulations on digital twin models
- Execute tools to interact with physical systems
- Analyze historical data and identify patterns
- Provide recommendations based on system state

# Available Tools
{{tool_descriptions}}

# Reasoning Process
1. Understand the user's request
2. Identify relevant digital twin components
3. Determine necessary tools or data sources
4. Execute tools or analyze data
5. Synthesize results into a clear response

# Response Format
Always structure your responses as follows:
- Summary of understanding
- Actions taken (if any)
- Results and analysis
- Recommendations (if applicable)
```

### Context Management

Implement these strategies for effective context management:

1. **Token Counting**: Accurately count tokens to stay within model limits
2. **Intelligent Truncation**: Remove less relevant messages first
3. **Summarization**: Summarize older conversation history
4. **Relevance Filtering**: Include only relevant tool outputs and file contents

## Tool System Design

### Tool Interface

All tools should implement the `Tool` trait:

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameter_schema(&self) -> Option<serde_json::Value>;
    async fn execute(&self, input: &str) -> Result<String, ToolError>;
}
```

### Tool Registry

The tool registry manages tool discovery and access:

```rust
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.insert(tool.name().to_string(), Box::new(tool));
    }
    
    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }
    
    pub fn list_descriptions(&self) -> Vec<ToolDescription> {
        self.tools.values()
            .map(|tool| ToolDescription {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                parameter_schema: tool.parameter_schema(),
            })
            .collect()
    }
}
```

### Tool Safety

Implement these safety measures for all tools:

1. **Sandboxing**: Run tools in isolated environments
2. **Timeouts**: Set execution time limits
3. **Permission Checks**: Verify the agent has necessary permissions
4. **Input Validation**: Validate all inputs before execution
5. **Rate Limiting**: Prevent excessive tool usage

## Agent Reasoning Patterns

### Planning

Implement a planning system that:

1. Breaks down complex tasks into steps
2. Identifies required tools for each step
3. Handles dependencies between steps
4. Adapts the plan based on new information

Example planning structure:

```rust
pub struct Plan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
}

pub struct PlanStep {
    pub description: String,
    pub tool: Option<String>,
    pub tool_input: Option<String>,
    pub completed: bool,
}
```

### Reflection

Implement self-reflection capabilities:

1. Evaluate the success of actions
2. Identify errors or inefficiencies
3. Learn from past interactions
4. Improve future behavior

## Digital Twin Integration

The agent should integrate with digital twins through:

1. **Twin Query Tools**: Tools to query twin state
2. **Simulation Tools**: Tools to run simulations
3. **Sensor Data Analysis**: Tools to analyze sensor data
4. **Twin Modification**: Tools to update twin configuration

## Implementation Best Practices

1. **Explicit State Transitions**
   ```rust
   match self.state {
       AgentState::Idle => {
           // Process user input
           self.state = AgentState::Thinking { context };
           self.process_thinking().await?;
       }
       // Other states...
   }
   ```

2. **Dependency Injection for Testing**
   ```rust
   pub struct Agent<L: LlmClient, R: ToolRegistry> {
       llm: L,
       tools: R,
       // ...
   }
   ```

3. **Comprehensive Logging**
   ```rust
   tracing::info!("Agent transitioning from {:?} to {:?}", old_state, new_state);
   ```

4. **Error Handling**
   ```rust
   match tool.execute(input).await {
       Ok(result) => {
           // Process result
       }
       Err(e) => {
           tracing::error!("Tool execution failed: {}", e);
           self.state = AgentState::Error { error: e.into() };
       }
   }
   ```

## Testing Agent Behavior

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test agent with mock LLM and tools
3. **Scenario Tests**: Test end-to-end workflows
4. **Regression Tests**: Ensure fixed bugs don't reappear

Example test:

```rust
#[tokio::test]
async fn test_agent_executes_tool_correctly() {
    // Setup
    let mock_llm = MockLlmClient::new()
        .with_response("I'll use the calculator tool: {{\"tool\": \"calculator\", \"input\": \"2+2\"}}");
    let mock_tool = MockTool::new()
        .with_name("calculator")
        .with_result("4");
    let mut registry = ToolRegistry::new();
    registry.register(mock_tool);
    
    let agent = Agent::new(mock_llm, registry);
    
    // Execute
    let response = agent.process_message("What is 2+2?").await.unwrap();
    
    // Verify
    assert!(response.contains("4"));
    assert_eq!(agent.state, AgentState::Idle);
}
```

## Debugging Agent Issues

When debugging agent issues:

1. Enable verbose logging to trace state transitions
2. Inspect the full context sent to the LLM
3. Check tool inputs and outputs
4. Verify state transitions are happening correctly
5. Test with simplified prompts to isolate issues

## Performance Considerations

1. **Minimize Context Size**: Only include relevant information
2. **Batch Operations**: Group related operations
3. **Cache Results**: Store and reuse expensive computations
4. **Stream Responses**: Use streaming for better UX
5. **Optimize Tool Execution**: Make tools efficient