# Tool API

## Overview

The Tool API allows you to discover, configure, and execute tools in the Digital Twin Desktop application. Tools are specialized functions that agents can use to interact with digital twins, external systems, and perform various operations.

## Endpoints

### List Tools

```
GET /api/tools
```

Retrieves a list of available tools.

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 20, max: 100) |
| sort      | string | Field to sort by (e.g., name, -created_at)       |
| category  | string | Filter by tool category                          |
| status    | string | Filter by status (available, disabled)           |
| search    | string | Search term to filter tools                      |

#### Response

```json
{
  "data": [
    {
      "id": "twin_query",
      "name": "Twin Query",
      "description": "Queries the current state of a digital twin",
      "category": "twin_management",
      "status": "available",
      "version": "1.0.0",
      "created_at": "2025-10-01T12:00:00Z",
      "updated_at": "2025-10-15T09:30:00Z"
    },
    {
      "id": "simulation",
      "name": "Simulation",
      "description": "Runs a simulation on a digital twin",
      "category": "simulation",
      "status": "available",
      "version": "1.0.0",
      "created_at": "2025-10-01T12:00:00Z",
      "updated_at": "2025-10-15T09:30:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total_items": 15,
      "total_pages": 1,
      "current_page": 1,
      "per_page": 20
    }
  }
}
```

### Get Tool

```
GET /api/tools/{tool_id}
```

Retrieves a specific tool by ID.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| tool_id   | string | The tool ID  |

#### Response

```json
{
  "data": {
    "id": "twin_query",
    "name": "Twin Query",
    "description": "Queries the current state of a digital twin",
    "long_description": "This tool allows agents to query the current state of a digital twin, including sensor values, actuator states, and system health. It supports filtering by specific fields and can return aggregated data.",
    "category": "twin_management",
    "status": "available",
    "version": "1.0.0",
    "parameter_schema": {
      "type": "object",
      "properties": {
        "twin_id": {
          "type": "string",
          "description": "The ID of the digital twin to query"
        },
        "fields": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Specific fields to query (optional)"
        },
        "include_history": {
          "type": "boolean",
          "description": "Include historical data (optional, default: false)"
        }
      },
      "required": ["twin_id"]
    },
    "return_schema": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string",
          "description": "Twin ID"
        },
        "status": {
          "type": "string",
          "description": "Twin status"
        },
        "sensors": {
          "type": "array",
          "description": "Sensor data"
        },
        "actuators": {
          "type": "array",
          "description": "Actuator states"
        }
      }
    },
    "examples": [
      {
        "input": {
          "twin_id": "pump-101"
        },
        "output": {
          "id": "pump-101",
          "status": "active",
          "sensors": [
            {
              "id": "temperature",
              "value": 65.5,
              "unit": "°C"
            }
          ]
        }
      }
    ],
    "permissions": ["read:twins"],
    "rate_limit": {
      "requests_per_minute": 60,
      "requests_per_hour": 1000
    },
    "timeout": 30,
    "created_at": "2025-10-01T12:00:00Z",
    "updated_at": "2025-10-15T09:30:00Z"
  }
}
```

### List Tool Categories

```
GET /api/tools/categories
```

Retrieves available tool categories.

#### Response

```json
{
  "data": [
    {
      "id": "twin_management",
      "name": "Twin Management",
      "description": "Tools for managing digital twins",
      "tool_count": 5
    },
    {
      "id": "simulation",
      "name": "Simulation",
      "description": "Tools for running simulations",
      "tool_count": 3
    },
    {
      "id": "data_analysis",
      "name": "Data Analysis",
      "description": "Tools for analyzing sensor and historical data",
      "tool_count": 4
    },
    {
      "id": "external_integration",
      "name": "External Integration",
      "description": "Tools for integrating with external systems",
      "tool_count": 3
    }
  ]
}
```

### Execute Tool

```
POST /api/tools/{tool_id}/execute
```

Executes a tool with the provided input.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| tool_id   | string | The tool ID  |

#### Request Body

```json
{
  "input": {
    "twin_id": "pump-101",
    "fields": ["temperature", "pressure", "status"]
  },
  "timeout": 30,
  "async": false
}
```

#### Required Fields

| Field | Type   | Description                                |
|-------|--------|-------------------------------------------|
| input | object | Input parameters for the tool              |

#### Optional Fields

| Field   | Type    | Description                                |
|---------|---------|-------------------------------------------|
| timeout | number  | Execution timeout in seconds               |
| async   | boolean | Execute asynchronously (default: false)    |

#### Response

```json
{
  "data": {
    "execution_id": "exec-001",
    "tool_id": "twin_query",
    "status": "completed",
    "input": {
      "twin_id": "pump-101",
      "fields": ["temperature", "pressure", "status"]
    },
    "output": {
      "id": "pump-101",
      "status": "active",
      "temperature": 65.5,
      "pressure": 2.5
    },
    "execution_time": 0.125,
    "started_at": "2025-11-17T04:00:00Z",
    "completed_at": "2025-11-17T04:00:00.125Z"
  }
}
```

### Get Execution Status

```
GET /api/tools/executions/{execution_id}
```

Retrieves the status of a tool execution.

#### Path Parameters

| Parameter    | Type   | Description        |
|--------------|--------|--------------------|
| execution_id | string | The execution ID   |

#### Response

```json
{
  "data": {
    "execution_id": "exec-001",
    "tool_id": "twin_query",
    "status": "completed",
    "progress": {
      "percentage": 100,
      "message": "Execution completed successfully"
    },
    "input": {
      "twin_id": "pump-101",
      "fields": ["temperature", "pressure", "status"]
    },
    "output": {
      "id": "pump-101",
      "status": "active",
      "temperature": 65.5,
      "pressure": 2.5
    },
    "execution_time": 0.125,
    "started_at": "2025-11-17T04:00:00Z",
    "completed_at": "2025-11-17T04:00:00.125Z"
  }
}
```

### Cancel Execution

```
POST /api/tools/executions/{execution_id}/cancel
```

Cancels a running tool execution.

#### Path Parameters

| Parameter    | Type   | Description        |
|--------------|--------|--------------------|
| execution_id | string | The execution ID   |

#### Response

```json
{
  "data": {
    "execution_id": "exec-001",
    "tool_id": "twin_query",
    "status": "cancelled",
    "cancelled_at": "2025-11-17T04:00:05Z"
  }
}
```

### Get Tool Execution History

```
GET /api/tools/{tool_id}/executions
```

Retrieves execution history for a tool.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| tool_id   | string | The tool ID  |

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 20, max: 100) |
| status    | string | Filter by execution status                       |
| start_date| string | Filter by start date (ISO 8601)                  |
| end_date  | string | Filter by end date (ISO 8601)                    |

#### Response

```json
{
  "data": [
    {
      "execution_id": "exec-001",
      "status": "completed",
      "execution_time": 0.125,
      "started_at": "2025-11-17T04:00:00Z",
      "completed_at": "2025-11-17T04:00:00.125Z"
    },
    {
      "execution_id": "exec-002",
      "status": "completed",
      "execution_time": 0.098,
      "started_at": "2025-11-17T04:01:00Z",
      "completed_at": "2025-11-17T04:01:00.098Z"
    }
  ],
  "meta": {
    "pagination": {
      "total_items": 156,
      "total_pages": 8,
      "current_page": 1,
      "per_page": 20
    }
  }
}
```

### Get Tool Statistics

```
GET /api/tools/{tool_id}/statistics
```

Retrieves usage statistics for a tool.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| tool_id   | string | The tool ID  |

#### Query Parameters

| Parameter  | Type   | Description                                      |
|------------|--------|--------------------------------------------------|
| start_date | string | Start date for statistics (ISO 8601)             |
| end_date   | string | End date for statistics (ISO 8601)               |

#### Response

```json
{
  "data": {
    "tool_id": "twin_query",
    "time_range": {
      "start": "2025-11-10T00:00:00Z",
      "end": "2025-11-17T00:00:00Z"
    },
    "usage": {
      "total_executions": 1250,
      "successful_executions": 1235,
      "failed_executions": 15,
      "success_rate": 0.988
    },
    "performance": {
      "average_execution_time": 0.145,
      "min_execution_time": 0.025,
      "max_execution_time": 2.5,
      "p95_execution_time": 0.35,
      "p99_execution_time": 0.85
    },
    "errors": [
      {
        "error_type": "timeout",
        "count": 8,
        "percentage": 53.3
      },
      {
        "error_type": "invalid_input",
        "count": 5,
        "percentage": 33.3
      },
      {
        "error_type": "twin_not_found",
        "count": 2,
        "percentage": 13.3
      }
    ]
  }
}
```

### Create Custom Tool

```
POST /api/tools
```

Creates a new custom tool.

#### Request Body

```json
{
  "name": "Custom Efficiency Calculator",
  "description": "Calculates system efficiency based on input and output parameters",
  "category": "data_analysis",
  "parameter_schema": {
    "type": "object",
    "properties": {
      "twin_id": {
        "type": "string",
        "description": "The ID of the digital twin"
      },
      "input_power": {
        "type": "number",
        "description": "Input power in kW"
      },
      "output_power": {
        "type": "number",
        "description": "Output power in kW"
      }
    },
    "required": ["twin_id", "input_power", "output_power"]
  },
  "implementation": {
    "type": "script",
    "language": "python",
    "code": "def execute(twin_id, input_power, output_power):\n    efficiency = (output_power / input_power) * 100\n    return {'efficiency': efficiency, 'unit': '%'}"
  }
}
```

#### Required Fields

| Field       | Type   | Description                                |
|-------------|--------|-------------------------------------------|
| name        | string | Name of the tool                           |
| description | string | Description of the tool                    |
| category    | string | Tool category                              |

#### Optional Fields

| Field              | Type   | Description                                |
|--------------------|--------|-------------------------------------------|
| parameter_schema   | object | JSON Schema for tool parameters            |
| return_schema      | object | JSON Schema for tool return value          |
| implementation     | object | Tool implementation details                |
| permissions        | string[]| Required permissions                       |
| rate_limit         | object | Rate limiting configuration                |
| timeout            | number | Execution timeout in seconds               |

#### Response

```json
{
  "data": {
    "id": "custom_efficiency_calculator",
    "name": "Custom Efficiency Calculator",
    "description": "Calculates system efficiency based on input and output parameters",
    "category": "data_analysis",
    "status": "available",
    "version": "1.0.0",
    "parameter_schema": {
      "type": "object",
      "properties": {
        "twin_id": {
          "type": "string",
          "description": "The ID of the digital twin"
        },
        "input_power": {
          "type": "number",
          "description": "Input power in kW"
        },
        "output_power": {
          "type": "number",
          "description": "Output power in kW"
        }
      },
      "required": ["twin_id", "input_power", "output_power"]
    },
    "created_at": "2025-11-17T04:05:00Z",
    "updated_at": "2025-11-17T04:05:00Z"
  }
}
```

### Update Tool

```
PATCH /api/tools/{tool_id}
```

Updates an existing tool.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| tool_id   | string | The tool ID  |

#### Request Body

```json
{
  "description": "Updated description of the tool",
  "status": "available",
  "timeout": 45
}
```

#### Response

```json
{
  "data": {
    "id": "custom_efficiency_calculator",
    "name": "Custom Efficiency Calculator",
    "description": "Updated description of the tool",
    "category": "data_analysis",
    "status": "available",
    "version": "1.0.1",
    "timeout": 45,
    "created_at": "2025-11-17T04:05:00Z",
    "updated_at": "2025-11-17T04:10:00Z"
  }
}
```

### Delete Tool

```
DELETE /api/tools/{tool_id}
```

Deletes a custom tool.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| tool_id   | string | The tool ID  |

#### Response

```
204 No Content
```

## Models

### Tool

| Field            | Type     | Description                                   |
|------------------|----------|-----------------------------------------------|
| id               | string   | Unique identifier for the tool                |
| name             | string   | Display name of the tool                      |
| description      | string   | Description of the tool's purpose             |
| long_description | string   | Detailed description of the tool              |
| category         | string   | Tool category                                 |
| status           | string   | Status (available, disabled)                  |
| version          | string   | Tool version                                  |
| parameter_schema | object   | JSON Schema for tool parameters               |
| return_schema    | object   | JSON Schema for tool return value             |
| examples         | array    | Usage examples                                |
| permissions      | string[] | Required permissions                          |
| rate_limit       | object   | Rate limiting configuration                   |
| timeout          | number   | Execution timeout in seconds                  |
| created_at       | string   | Creation timestamp (ISO 8601)                 |
| updated_at       | string   | Last update timestamp (ISO 8601)              |

### Tool Execution

| Field          | Type   | Description                                   |
|----------------|--------|-----------------------------------------------|
| execution_id   | string | Unique identifier for the execution           |
| tool_id        | string | ID of the executed tool                       |
| status         | string | Status (running, completed, failed, cancelled)|
| input          | object | Input parameters provided to the tool         |
| output         | object | Output returned by the tool                   |
| error          | object | Error details (if failed)                     |
| execution_time | number | Execution time in seconds                     |
| started_at     | string | Start timestamp (ISO 8601)                    |
| completed_at   | string | Completion timestamp (ISO 8601)               |

## Error Codes

| Code                | Description                                   |
|---------------------|-----------------------------------------------|
| TOOL_NOT_FOUND      | The specified tool was not found              |
| EXECUTION_NOT_FOUND | The specified execution was not found         |
| INVALID_INPUT       | The tool input is invalid                     |
| EXECUTION_TIMEOUT   | The tool execution timed out                  |
| EXECUTION_FAILED    | The tool execution failed                     |
| TOOL_DISABLED       | The tool is currently disabled                |
| RATE_LIMIT_EXCEEDED | Rate limit for the tool has been exceeded     |

## Examples

### Executing a Tool

```bash
# Execute the twin_query tool
curl -X POST http://localhost:3000/api/tools/twin_query/execute \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "input": {
      "twin_id": "pump-101",
      "fields": ["temperature", "pressure", "status"]
    },
    "timeout": 30
  }'

# Response
{
  "data": {
    "execution_id": "exec-001",
    "tool_id": "twin_query",
    "status": "completed",
    "output": {
      "id": "pump-101",
      "status": "active",
      "temperature": 65.5,
      "pressure": 2.5
    },
    "execution_time": 0.125,
    "completed_at": "2025-11-17T04:00:00.125Z"
  }
}
```

### Getting Tool Statistics

```bash
# Get statistics for the twin_query tool
curl -X GET "http://localhost:3000/api/tools/twin_query/statistics?start_date=2025-11-10T00:00:00Z&end_date=2025-11-17T00:00:00Z" \
  -H "Authorization: Bearer your-token"
```

### Creating a Custom Tool

```bash
# Create a custom tool
curl -X POST http://localhost:3000/api/tools \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "System Health Analyzer",
    "description": "Analyzes the overall health of a system based on multiple parameters",
    "category": "data_analysis",
    "parameter_schema": {
      "type": "object",
      "properties": {
        "twin_id": {
          "type": "string",
          "description": "The ID of the digital twin"
        }
      },
      "required": ["twin_id"]
    }
  }'
```

## Related Resources

- [Twin API](./twin.md): Manage digital twins
- [Agent API](./agent.md): Manage AI agents that use tools
- [Conversation API](./conversation.md): Discuss tool execution results