# Agent API

## Overview

The Agent API allows you to manage AI agents in the Digital Twin Desktop application. Agents are intelligent assistants that can interact with users, digital twins, and external systems to provide insights, run simulations, and perform various tasks.

## Endpoints

### List Agents

```
GET /api/agents
```

Retrieves a list of agents.

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 20, max: 100) |
| sort      | string | Field to sort by (e.g., created_at, -name)       |
| status    | string | Filter by status (active, disabled)              |
| type      | string | Filter by agent type (general, twin_specialist, etc.) |
| search    | string | Search term to filter agents                     |

#### Response

```json
{
  "data": [
    {
      "id": "agent-789",
      "name": "Digital Twin Assistant",
      "description": "General-purpose assistant for digital twin management",
      "type": "general",
      "status": "active",
      "llm_provider": "anthropic",
      "llm_model": "claude-3-sonnet-20240229",
      "created_at": "2025-10-01T12:00:00Z",
      "updated_at": "2025-10-15T09:30:00Z"
    },
    {
      "id": "agent-790",
      "name": "Pump Specialist",
      "description": "Specialized assistant for pump systems",
      "type": "twin_specialist",
      "status": "active",
      "llm_provider": "anthropic",
      "llm_model": "claude-3-opus-20240229",
      "created_at": "2025-10-05T14:15:00Z",
      "updated_at": "2025-10-15T10:45:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total_items": 8,
      "total_pages": 1,
      "current_page": 1,
      "per_page": 20
    }
  },
  "links": {
    "self": "/api/agents?page=1&per_page=20"
  }
}
```

### Get Agent

```
GET /api/agents/{agent_id}
```

Retrieves a specific agent by ID.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Response

```json
{
  "data": {
    "id": "agent-789",
    "name": "Digital Twin Assistant",
    "description": "General-purpose assistant for digital twin management",
    "type": "general",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-sonnet-20240229",
    "system_prompt": "You are a Digital Twin Assistant, designed to help users monitor, simulate, and interact with digital twins of physical systems...",
    "tools": ["twin_query", "simulation", "sensor_data", "web_search", "calculator"],
    "capabilities": ["twin_monitoring", "simulation", "anomaly_detection", "maintenance_planning"],
    "metadata": {
      "version": "1.2.0",
      "created_by": "admin"
    },
    "created_at": "2025-10-01T12:00:00Z",
    "updated_at": "2025-10-15T09:30:00Z"
  }
}
```

### Create Agent

```
POST /api/agents
```

Creates a new agent.

#### Request Body

```json
{
  "name": "Heat Exchanger Specialist",
  "description": "Specialized assistant for heat exchanger systems",
  "type": "twin_specialist",
  "llm_provider": "anthropic",
  "llm_model": "claude-3-sonnet-20240229",
  "system_prompt": "You are a Heat Exchanger Specialist, an AI assistant with expertise in heat exchanger systems...",
  "tools": ["twin_query", "simulation", "sensor_data", "calculator"],
  "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning"],
  "metadata": {
    "version": "1.0.0",
    "created_by": "admin"
  }
}
```

#### Required Fields

| Field        | Type   | Description                                |
|--------------|--------|--------------------------------------------|
| name         | string | The name of the agent                      |
| description  | string | A description of the agent's purpose       |
| llm_provider | string | The LLM provider (anthropic, openai)       |
| llm_model    | string | The LLM model to use                       |

#### Optional Fields

| Field        | Type     | Description                                |
|--------------|----------|--------------------------------------------|
| type         | string   | The agent type (default: "general")        |
| system_prompt| string   | Custom system prompt for the agent         |
| tools        | string[] | List of tools the agent can use            |
| capabilities | string[] | List of agent capabilities                 |
| metadata     | object   | Additional metadata for the agent          |

#### Response

```json
{
  "data": {
    "id": "agent-791",
    "name": "Heat Exchanger Specialist",
    "description": "Specialized assistant for heat exchanger systems",
    "type": "twin_specialist",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-sonnet-20240229",
    "system_prompt": "You are a Heat Exchanger Specialist, an AI assistant with expertise in heat exchanger systems...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning"],
    "metadata": {
      "version": "1.0.0",
      "created_by": "admin"
    },
    "created_at": "2025-11-17T03:48:00Z",
    "updated_at": "2025-11-17T03:48:00Z"
  }
}
```

### Update Agent

```
PATCH /api/agents/{agent_id}
```

Updates an existing agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Request Body

```json
{
  "name": "Heat Exchanger Expert",
  "description": "Advanced specialist for heat exchanger systems with simulation capabilities",
  "status": "active",
  "llm_model": "claude-3-opus-20240229",
  "system_prompt": "You are a Heat Exchanger Expert, an AI assistant with advanced expertise in heat exchanger systems...",
  "tools": ["twin_query", "simulation", "sensor_data", "calculator", "efficiency_analyzer"],
  "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning", "anomaly_detection"],
  "metadata": {
    "version": "1.1.0",
    "created_by": "admin",
    "updated_by": "admin"
  }
}
```

#### Response

```json
{
  "data": {
    "id": "agent-791",
    "name": "Heat Exchanger Expert",
    "description": "Advanced specialist for heat exchanger systems with simulation capabilities",
    "type": "twin_specialist",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-opus-20240229",
    "system_prompt": "You are a Heat Exchanger Expert, an AI assistant with advanced expertise in heat exchanger systems...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator", "efficiency_analyzer"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning", "anomaly_detection"],
    "metadata": {
      "version": "1.1.0",
      "created_by": "admin",
      "updated_by": "admin"
    },
    "created_at": "2025-11-17T03:48:00Z",
    "updated_at": "2025-11-17T04:15:00Z"
  }
}
```

### Delete Agent

```
DELETE /api/agents/{agent_id}
```

Deletes an agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Response

```
204 No Content
```

### Get Agent Tools

```
GET /api/agents/{agent_id}/tools
```

Retrieves the tools available to an agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Response

```json
{
  "data": [
    {
      "name": "twin_query",
      "description": "Queries the current state of a digital twin",
      "parameter_schema": {
        "type": "object",
        "properties": {
          "twin_id": {
            "type": "string",
            "description": "The ID of the digital twin"
          },
          "fields": {
            "type": "array",
            "items": {
              "type": "string"
            },
            "description": "The fields to query (optional)"
          }
        },
        "required": ["twin_id"]
      }
    },
    {
      "name": "simulation",
      "description": "Runs a simulation on a digital twin",
      "parameter_schema": {
        "type": "object",
        "properties": {
          "twin_id": {
            "type": "string",
            "description": "The ID of the digital twin"
          },
          "parameters": {
            "type": "object",
            "description": "Simulation parameters"
          },
          "duration": {
            "type": "number",
            "description": "Simulation duration in seconds (optional)"
          }
        },
        "required": ["twin_id", "parameters"]
      }
    }
  ]
}
```

### Add Tool to Agent

```
POST /api/agents/{agent_id}/tools
```

Adds a tool to an agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Request Body

```json
{
  "tool_name": "efficiency_analyzer"
}
```

#### Required Fields

| Field     | Type   | Description                |
|-----------|--------|----------------------------|
| tool_name | string | The name of the tool to add |

#### Response

```json
{
  "data": {
    "name": "efficiency_analyzer",
    "description": "Analyzes the efficiency of a system based on input and output parameters",
    "parameter_schema": {
      "type": "object",
      "properties": {
        "twin_id": {
          "type": "string",
          "description": "The ID of the digital twin"
        },
        "input_parameters": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Input parameter names"
        },
        "output_parameters": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Output parameter names"
        },
        "time_range": {
          "type": "object",
          "properties": {
            "start": {
              "type": "string",
              "format": "date-time"
            },
            "end": {
              "type": "string",
              "format": "date-time"
            }
          },
          "description": "Time range for analysis (optional)"
        }
      },
      "required": ["twin_id", "input_parameters", "output_parameters"]
    }
  }
}
```

### Remove Tool from Agent

```
DELETE /api/agents/{agent_id}/tools/{tool_name}
```

Removes a tool from an agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |
| tool_name | string | The tool name |

#### Response

```
204 No Content
```

### Get Agent Conversations

```
GET /api/agents/{agent_id}/conversations
```

Retrieves conversations associated with an agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 20, max: 100) |
| sort      | string | Field to sort by (e.g., created_at, -updated_at) |
| status    | string | Filter by status (active, archived)              |
| twin_id   | string | Filter by associated digital twin ID             |

#### Response

```json
{
  "data": [
    {
      "id": "conv-123456",
      "title": "Pump Maintenance Discussion",
      "agent_id": "agent-789",
      "twin_id": "pump-101",
      "status": "active",
      "message_count": 24,
      "created_at": "2025-10-15T14:30:00Z",
      "updated_at": "2025-10-15T15:45:00Z"
    },
    {
      "id": "conv-123457",
      "title": "Temperature Anomaly Analysis",
      "agent_id": "agent-789",
      "twin_id": "heat-exchanger-1",
      "status": "active",
      "message_count": 18,
      "created_at": "2025-10-14T09:15:00Z",
      "updated_at": "2025-10-14T10:30:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total_items": 12,
      "total_pages": 1,
      "current_page": 1,
      "per_page": 20
    }
  },
  "links": {
    "self": "/api/agents/agent-789/conversations?page=1&per_page=20"
  }
}
```

### Get Agent Performance Metrics

```
GET /api/agents/{agent_id}/metrics
```

Retrieves performance metrics for an agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Query Parameters

| Parameter  | Type   | Description                                      |
|------------|--------|--------------------------------------------------|
| start_date | string | Start date for metrics (ISO 8601)                |
| end_date   | string | End date for metrics (ISO 8601)                  |
| metrics    | string | Comma-separated list of metrics to include       |

#### Response

```json
{
  "data": {
    "time_range": {
      "start": "2025-10-01T00:00:00Z",
      "end": "2025-10-31T23:59:59Z"
    },
    "conversation_metrics": {
      "total_conversations": 45,
      "average_messages_per_conversation": 12.3,
      "average_conversation_duration": 15.7
    },
    "response_metrics": {
      "average_response_time": 2.5,
      "average_tokens_per_response": 350.2
    },
    "tool_metrics": {
      "total_tool_calls": 156,
      "tool_usage": {
        "twin_query": 78,
        "simulation": 42,
        "sensor_data": 25,
        "calculator": 11
      },
      "tool_success_rate": 0.97
    },
    "user_satisfaction": {
      "average_rating": 4.8,
      "total_ratings": 32
    }
  }
}
```

### Clone Agent

```
POST /api/agents/{agent_id}/clone
```

Creates a clone of an existing agent.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Request Body

```json
{
  "name": "Heat Exchanger Expert (Test)",
  "description": "Test version of the Heat Exchanger Expert agent",
  "metadata": {
    "version": "1.1.0-test",
    "created_by": "admin",
    "purpose": "testing"
  }
}
```

#### Required Fields

| Field       | Type   | Description                                |
|-------------|--------|--------------------------------------------|
| name        | string | The name of the cloned agent               |

#### Optional Fields

| Field       | Type   | Description                                |
|-------------|--------|--------------------------------------------|
| description | string | A description of the cloned agent's purpose |
| metadata    | object | Additional metadata for the cloned agent    |

#### Response

```json
{
  "data": {
    "id": "agent-792",
    "name": "Heat Exchanger Expert (Test)",
    "description": "Test version of the Heat Exchanger Expert agent",
    "type": "twin_specialist",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-opus-20240229",
    "system_prompt": "You are a Heat Exchanger Expert, an AI assistant with advanced expertise in heat exchanger systems...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator", "efficiency_analyzer"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning", "anomaly_detection"],
    "metadata": {
      "version": "1.1.0-test",
      "created_by": "admin",
      "purpose": "testing",
      "cloned_from": "agent-791"
    },
    "created_at": "2025-11-17T04:30:00Z",
    "updated_at": "2025-11-17T04:30:00Z"
  }
}
```

### Export Agent Configuration

```
GET /api/agents/{agent_id}/export
```

Exports an agent's configuration.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| agent_id  | string | The agent ID |

#### Response

```json
{
  "data": {
    "id": "agent-791",
    "name": "Heat Exchanger Expert",
    "description": "Advanced specialist for heat exchanger systems with simulation capabilities",
    "type": "twin_specialist",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-opus-20240229",
    "system_prompt": "You are a Heat Exchanger Expert, an AI assistant with advanced expertise in heat exchanger systems...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator", "efficiency_analyzer"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning", "anomaly_detection"],
    "metadata": {
      "version": "1.1.0",
      "created_by": "admin",
      "updated_by": "admin"
    },
    "created_at": "2025-11-17T03:48:00Z",
    "updated_at": "2025-11-17T04:15:00Z"
  }
}
```

### Import Agent Configuration

```
POST /api/agents/import
```

Imports an agent configuration.

#### Request Body

```json
{
  "name": "Imported Heat Exchanger Expert",
  "description": "Imported agent configuration for heat exchanger systems",
  "type": "twin_specialist",
  "llm_provider": "anthropic",
  "llm_model": "claude-3-opus-20240229",
  "system_prompt": "You are a Heat Exchanger Expert, an AI assistant with advanced expertise in heat exchanger systems...",
  "tools": ["twin_query", "simulation", "sensor_data", "calculator", "efficiency_analyzer"],
  "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning", "anomaly_detection"],
  "metadata": {
    "version": "1.1.0",
    "imported_at": "2025-11-17T05:00:00Z",
    "imported_by": "admin"
  }
}
```

#### Response

```json
{
  "data": {
    "id": "agent-793",
    "name": "Imported Heat Exchanger Expert",
    "description": "Imported agent configuration for heat exchanger systems",
    "type": "twin_specialist",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-opus-20240229",
    "system_prompt": "You are a Heat Exchanger Expert, an AI assistant with advanced expertise in heat exchanger systems...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator", "efficiency_analyzer"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning", "anomaly_detection"],
    "metadata": {
      "version": "1.1.0",
      "imported_at": "2025-11-17T05:00:00Z",
      "imported_by": "admin"
    },
    "created_at": "2025-11-17T05:00:00Z",
    "updated_at": "2025-11-17T05:00:00Z"
  }
}
```

## Models

### Agent

| Field         | Type     | Description                                   |
|---------------|----------|-----------------------------------------------|
| id            | string   | Unique identifier for the agent               |
| name          | string   | Name of the agent                             |
| description   | string   | Description of the agent's purpose            |
| type          | string   | Type of agent (general, twin_specialist, etc.)|
| status        | string   | Status of the agent (active, disabled)        |
| llm_provider  | string   | LLM provider (anthropic, openai)              |
| llm_model     | string   | LLM model used by the agent                   |
| system_prompt | string   | System prompt for the agent                   |
| tools         | string[] | List of tools the agent can use               |
| capabilities  | string[] | List of agent capabilities                    |
| metadata      | object   | Additional metadata                           |
| created_at    | string   | Creation timestamp (ISO 8601)                 |
| updated_at    | string   | Last update timestamp (ISO 8601)              |

### Tool

| Field            | Type   | Description                                   |
|------------------|--------|-----------------------------------------------|
| name             | string | Name of the tool                              |
| description      | string | Description of what the tool does             |
| parameter_schema | object | JSON Schema for the tool's parameters         |

## Error Codes

| Code                | Description                                   |
|---------------------|-----------------------------------------------|
| AGENT_NOT_FOUND     | The specified agent was not found             |
| TOOL_NOT_FOUND      | The specified tool was not found              |
| INVALID_AGENT_TYPE  | The agent type is invalid                     |
| INVALID_LLM_PROVIDER| The LLM provider is invalid                   |
| INVALID_LLM_MODEL   | The LLM model is invalid                      |
| TOOL_ALREADY_ADDED  | The tool is already added to the agent        |

## Examples

### Creating a Specialized Agent

```bash
# Create a new specialized agent
curl -X POST http://localhost:3000/api/agents \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "HVAC System Specialist",
    "description": "Specialized assistant for HVAC systems",
    "type": "twin_specialist",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-sonnet-20240229",
    "system_prompt": "You are an HVAC System Specialist, an AI assistant with expertise in heating, ventilation, and air conditioning systems. You help users monitor, analyze, and optimize HVAC systems through digital twins...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning"],
    "metadata": {
      "version": "1.0.0",
      "created_by": "admin",
      "specialization": "hvac"
    }
  }'

# Response
{
  "data": {
    "id": "agent-794",
    "name": "HVAC System Specialist",
    "description": "Specialized assistant for HVAC systems",
    "type": "twin_specialist",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-sonnet-20240229",
    "system_prompt": "You are an HVAC System Specialist, an AI assistant with expertise in heating, ventilation, and air conditioning systems. You help users monitor, analyze, and optimize HVAC systems through digital twins...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning"],
    "metadata": {
      "version": "1.0.0",
      "created_by": "admin",
      "specialization": "hvac"
    },
    "created_at": "2025-11-17T05:15:00Z",
    "updated_at": "2025-11-17T05:15:00Z"
  }
}
```

### Adding a Custom Tool to an Agent

```bash
# Add a custom tool to an agent
curl -X POST http://localhost:3000/api/agents/agent-794/tools \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "tool_name": "hvac_efficiency_calculator"
  }'

# Response
{
  "data": {
    "name": "hvac_efficiency_calculator",
    "description": "Calculates HVAC system efficiency based on input and output parameters",
    "parameter_schema": {
      "type": "object",
      "properties": {
        "twin_id": {
          "type": "string",
          "description": "The ID of the HVAC digital twin"
        },
        "input_power": {
          "type": "number",
          "description": "Input power in kW"
        },
        "output_cooling": {
          "type": "number",
          "description": "Output cooling capacity in kW"
        },
        "ambient_temperature": {
          "type": "number",
          "description": "Ambient temperature in °C"
        }
      },
      "required": ["twin_id", "input_power", "output_cooling"]
    }
  }
}
```

### Cloning an Agent for Testing

```bash
# Clone an agent for testing
curl -X POST http://localhost:3000/api/agents/agent-794/clone \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "HVAC System Specialist (Test)",
    "description": "Test version of the HVAC System Specialist agent",
    "metadata": {
      "version": "1.0.0-test",
      "created_by": "admin",
      "purpose": "testing",
      "test_scenario": "efficiency_optimization"
    }
  }'

# Response
{
  "data": {
    "id": "agent-795",
    "name": "HVAC System Specialist (Test)",
    "description": "Test version of the HVAC System Specialist agent",
    "type": "twin_specialist",
    "status": "active",
    "llm_provider": "anthropic",
    "llm_model": "claude-3-sonnet-20240229",
    "system_prompt": "You are an HVAC System Specialist, an AI assistant with expertise in heating, ventilation, and air conditioning systems. You help users monitor, analyze, and optimize HVAC systems through digital twins...",
    "tools": ["twin_query", "simulation", "sensor_data", "calculator", "hvac_efficiency_calculator"],
    "capabilities": ["twin_monitoring", "simulation", "efficiency_analysis", "maintenance_planning"],
    "metadata": {
      "version": "1.0.0-test",
      "created_by": "admin",
      "purpose": "testing",
      "test_scenario": "efficiency_optimization",
      "cloned_from": "agent-794"
    },
    "created_at": "2025-11-17T05:30:00Z",
    "updated_at": "2025-11-17T05:30:00Z"
  }
}
```

## Related Resources

- [Conversation API](./conversation.md): Manage conversations with agents
- [Twin API](./twin.md): Manage digital twins
- [Tool API](./tool.md): Access and execute tools