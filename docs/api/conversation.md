# Conversation API

## Overview

The Conversation API allows you to manage conversations between users and AI agents in the Digital Twin Desktop application. Conversations provide a natural language interface for interacting with digital twins, running simulations, and analyzing data.

## Endpoints

### List Conversations

```
GET /api/conversations
```

Retrieves a list of conversations.

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 20, max: 100) |
| sort      | string | Field to sort by (e.g., created_at, -updated_at) |
| agent_id  | string | Filter by agent ID                               |
| twin_id   | string | Filter by associated digital twin ID             |
| status    | string | Filter by status (active, archived)              |
| search    | string | Search term to filter conversations              |

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
      "total_items": 45,
      "total_pages": 3,
      "current_page": 1,
      "per_page": 20
    }
  },
  "links": {
    "self": "/api/conversations?page=1&per_page=20",
    "next": "/api/conversations?page=2&per_page=20",
    "last": "/api/conversations?page=3&per_page=20"
  }
}
```

### Get Conversation

```
GET /api/conversations/{conversation_id}
```

Retrieves a specific conversation by ID.

#### Path Parameters

| Parameter       | Type   | Description        |
|-----------------|--------|--------------------|
| conversation_id | string | The conversation ID |

#### Response

```json
{
  "data": {
    "id": "conv-123456",
    "title": "Pump Maintenance Discussion",
    "agent_id": "agent-789",
    "twin_id": "pump-101",
    "status": "active",
    "message_count": 24,
    "created_at": "2025-10-15T14:30:00Z",
    "updated_at": "2025-10-15T15:45:00Z",
    "metadata": {
      "context": "maintenance planning",
      "priority": "high"
    }
  }
}
```

### Create Conversation

```
POST /api/conversations
```

Creates a new conversation.

#### Request Body

```json
{
  "title": "Heat Exchanger Efficiency Analysis",
  "agent_id": "agent-789",
  "twin_id": "heat-exchanger-1",
  "metadata": {
    "context": "efficiency optimization",
    "priority": "medium"
  }
}
```

#### Required Fields

| Field    | Type   | Description                                |
|----------|--------|--------------------------------------------|
| title    | string | The title of the conversation              |
| agent_id | string | The ID of the agent for this conversation  |

#### Optional Fields

| Field    | Type   | Description                                |
|----------|--------|--------------------------------------------|
| twin_id  | string | The ID of the associated digital twin      |
| metadata | object | Additional metadata for the conversation   |

#### Response

```json
{
  "data": {
    "id": "conv-123458",
    "title": "Heat Exchanger Efficiency Analysis",
    "agent_id": "agent-789",
    "twin_id": "heat-exchanger-1",
    "status": "active",
    "message_count": 0,
    "created_at": "2025-11-17T03:46:00Z",
    "updated_at": "2025-11-17T03:46:00Z",
    "metadata": {
      "context": "efficiency optimization",
      "priority": "medium"
    }
  }
}
```

### Update Conversation

```
PATCH /api/conversations/{conversation_id}
```

Updates an existing conversation.

#### Path Parameters

| Parameter       | Type   | Description        |
|-----------------|--------|--------------------|
| conversation_id | string | The conversation ID |

#### Request Body

```json
{
  "title": "Heat Exchanger Efficiency Optimization",
  "status": "archived",
  "metadata": {
    "context": "efficiency optimization",
    "priority": "high",
    "outcome": "successful"
  }
}
```

#### Response

```json
{
  "data": {
    "id": "conv-123458",
    "title": "Heat Exchanger Efficiency Optimization",
    "agent_id": "agent-789",
    "twin_id": "heat-exchanger-1",
    "status": "archived",
    "message_count": 15,
    "created_at": "2025-11-17T03:46:00Z",
    "updated_at": "2025-11-17T04:30:00Z",
    "metadata": {
      "context": "efficiency optimization",
      "priority": "high",
      "outcome": "successful"
    }
  }
}
```

### Delete Conversation

```
DELETE /api/conversations/{conversation_id}
```

Deletes a conversation.

#### Path Parameters

| Parameter       | Type   | Description        |
|-----------------|--------|--------------------|
| conversation_id | string | The conversation ID |

#### Response

```
204 No Content
```

### List Messages

```
GET /api/conversations/{conversation_id}/messages
```

Retrieves messages from a conversation.

#### Path Parameters

| Parameter       | Type   | Description        |
|-----------------|--------|--------------------|
| conversation_id | string | The conversation ID |

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 50, max: 100) |
| before    | string | Get messages before this timestamp               |
| after     | string | Get messages after this timestamp                |

#### Response

```json
{
  "data": [
    {
      "id": "msg-456789",
      "conversation_id": "conv-123456",
      "role": "user",
      "content": "What's the current status of the pump?",
      "created_at": "2025-10-15T14:30:00Z"
    },
    {
      "id": "msg-456790",
      "conversation_id": "conv-123456",
      "role": "assistant",
      "content": "The pump is currently operating at 75% capacity with a temperature of 65°C. All parameters are within normal operating ranges. The last maintenance was performed 45 days ago.",
      "created_at": "2025-10-15T14:30:15Z",
      "tool_calls": [
        {
          "tool": "twin_query",
          "input": {
            "twin_id": "pump-101",
            "fields": ["status", "temperature", "capacity", "last_maintenance"]
          },
          "output": {
            "status": "operational",
            "temperature": 65,
            "capacity": 0.75,
            "last_maintenance": "2025-09-01T10:00:00Z"
          }
        }
      ]
    }
  ],
  "meta": {
    "pagination": {
      "total_items": 24,
      "total_pages": 1,
      "current_page": 1,
      "per_page": 50
    }
  },
  "links": {
    "self": "/api/conversations/conv-123456/messages?page=1&per_page=50"
  }
}
```

### Send Message

```
POST /api/conversations/{conversation_id}/messages
```

Sends a new message to the conversation.

#### Path Parameters

| Parameter       | Type   | Description        |
|-----------------|--------|--------------------|
| conversation_id | string | The conversation ID |

#### Request Body

```json
{
  "content": "What would happen if the temperature increased to 80°C?",
  "role": "user"
}
```

#### Required Fields

| Field   | Type   | Description                  |
|---------|--------|------------------------------|
| content | string | The content of the message   |
| role    | string | The role (user or system)    |

#### Response

```json
{
  "data": {
    "id": "msg-456791",
    "conversation_id": "conv-123456",
    "role": "user",
    "content": "What would happen if the temperature increased to 80°C?",
    "created_at": "2025-11-17T03:47:00Z"
  }
}
```

### Stream Response

```
GET /api/conversations/{conversation_id}/stream
```

Establishes a Server-Sent Events (SSE) connection to stream the assistant's response in real-time.

#### Path Parameters

| Parameter       | Type   | Description        |
|-----------------|--------|--------------------|
| conversation_id | string | The conversation ID |

#### Query Parameters

| Parameter | Type   | Description                                |
|-----------|--------|--------------------------------------------|
| message_id| string | The message ID to respond to               |

#### Response

The response is a stream of Server-Sent Events (SSE) with the following event types:

- `message_start`: Indicates the start of a message
- `message_chunk`: Contains a chunk of the message content
- `tool_call_start`: Indicates the start of a tool call
- `tool_call_update`: Contains an update about a tool call
- `tool_call_end`: Indicates the end of a tool call
- `message_end`: Indicates the end of the message

Example:

```
event: message_start
data: {"message_id":"msg-456792","conversation_id":"conv-123456","role":"assistant"}

event: message_chunk
data: {"message_id":"msg-456792","chunk":"If the temperature increased to 80°C, "}

event: message_chunk
data: {"message_id":"msg-456792","chunk":"the pump would be operating outside its normal range. "}

event: tool_call_start
data: {"message_id":"msg-456792","tool_call_id":"tc-123","tool":"simulation","input":{"twin_id":"pump-101","parameters":{"temperature":80}}}

event: tool_call_update
data: {"message_id":"msg-456792","tool_call_id":"tc-123","status":"running"}

event: tool_call_update
data: {"message_id":"msg-456792","tool_call_id":"tc-123","status":"completed","output":{"warning_threshold_exceeded":true,"estimated_time_to_failure":"4.5 hours","recommended_action":"reduce_load"}}

event: message_chunk
data: {"message_id":"msg-456792","chunk":"Based on the simulation, at 80°C the pump would exceed its warning threshold. "}

event: message_chunk
data: {"message_id":"msg-456792","chunk":"The estimated time to failure would be approximately 4.5 hours if no action is taken. "}

event: message_chunk
data: {"message_id":"msg-456792","chunk":"I recommend reducing the load on the pump to bring the temperature back to normal operating range."}

event: message_end
data: {"message_id":"msg-456792"}
```

### Export Conversation

```
GET /api/conversations/{conversation_id}/export
```

Exports a conversation in the specified format.

#### Path Parameters

| Parameter       | Type   | Description        |
|-----------------|--------|--------------------|
| conversation_id | string | The conversation ID |

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| format    | string | Export format (json, markdown, html, pdf)        |

#### Response

For JSON format:

```json
{
  "data": {
    "id": "conv-123456",
    "title": "Pump Maintenance Discussion",
    "created_at": "2025-10-15T14:30:00Z",
    "updated_at": "2025-10-15T15:45:00Z",
    "messages": [
      {
        "id": "msg-456789",
        "role": "user",
        "content": "What's the current status of the pump?",
        "created_at": "2025-10-15T14:30:00Z"
      },
      {
        "id": "msg-456790",
        "role": "assistant",
        "content": "The pump is currently operating at 75% capacity with a temperature of 65°C. All parameters are within normal operating ranges. The last maintenance was performed 45 days ago.",
        "created_at": "2025-10-15T14:30:15Z",
        "tool_calls": [
          {
            "tool": "twin_query",
            "input": {
              "twin_id": "pump-101",
              "fields": ["status", "temperature", "capacity", "last_maintenance"]
            },
            "output": {
              "status": "operational",
              "temperature": 65,
              "capacity": 0.75,
              "last_maintenance": "2025-09-01T10:00:00Z"
            }
          }
        ]
      }
    ]
  }
}
```

For other formats, the response will be a file download with the appropriate Content-Type header.

## Models

### Conversation

| Field         | Type     | Description                                   |
|---------------|----------|-----------------------------------------------|
| id            | string   | Unique identifier for the conversation        |
| title         | string   | Title of the conversation                     |
| agent_id      | string   | ID of the agent for this conversation         |
| twin_id       | string   | ID of the associated digital twin (optional)  |
| status        | string   | Status of the conversation (active, archived) |
| message_count | number   | Number of messages in the conversation        |
| created_at    | string   | Creation timestamp (ISO 8601)                 |
| updated_at    | string   | Last update timestamp (ISO 8601)              |
| metadata      | object   | Additional metadata (optional)                |

### Message

| Field           | Type     | Description                                   |
|-----------------|----------|-----------------------------------------------|
| id              | string   | Unique identifier for the message             |
| conversation_id | string   | ID of the conversation                        |
| role            | string   | Role of the message sender (user, assistant, system) |
| content         | string   | Content of the message                        |
| created_at      | string   | Creation timestamp (ISO 8601)                 |
| tool_calls      | array    | Tool calls made during message generation (optional) |

### Tool Call

| Field    | Type     | Description                                   |
|----------|----------|-----------------------------------------------|
| tool     | string   | Name of the tool                              |
| input    | object   | Input parameters for the tool                 |
| output   | object   | Output from the tool execution                |

## Error Codes

| Code                    | Description                                   |
|-------------------------|-----------------------------------------------|
| CONVERSATION_NOT_FOUND  | The specified conversation was not found      |
| MESSAGE_NOT_FOUND       | The specified message was not found           |
| INVALID_MESSAGE_ROLE    | The message role is invalid                   |
| CONVERSATION_ARCHIVED   | Cannot add messages to an archived conversation |
| EXPORT_FORMAT_INVALID   | The specified export format is not supported  |

## Examples

### Creating a New Conversation and Sending a Message

```bash
# Create a new conversation
curl -X POST http://localhost:3000/api/conversations \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Pump Efficiency Analysis",
    "agent_id": "agent-789",
    "twin_id": "pump-101"
  }'

# Response
{
  "data": {
    "id": "conv-123459",
    "title": "Pump Efficiency Analysis",
    "agent_id": "agent-789",
    "twin_id": "pump-101",
    "status": "active",
    "message_count": 0,
    "created_at": "2025-11-17T03:48:00Z",
    "updated_at": "2025-11-17T03:48:00Z"
  }
}

# Send a message to the conversation
curl -X POST http://localhost:3000/api/conversations/conv-123459/messages \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Analyze the efficiency of the pump over the last 30 days",
    "role": "user"
  }'

# Response
{
  "data": {
    "id": "msg-456793",
    "conversation_id": "conv-123459",
    "role": "user",
    "content": "Analyze the efficiency of the pump over the last 30 days",
    "created_at": "2025-11-17T03:48:30Z"
  }
}
```

### Streaming the Assistant's Response

```javascript
// Establish SSE connection
const eventSource = new EventSource(
  'http://localhost:3000/api/conversations/conv-123459/stream?message_id=msg-456793',
  { headers: { 'Authorization': 'Bearer your-token' } }
);

// Handle events
eventSource.addEventListener('message_start', (event) => {
  const data = JSON.parse(event.data);
  console.log('Assistant started responding with message ID:', data.message_id);
});

eventSource.addEventListener('message_chunk', (event) => {
  const data = JSON.parse(event.data);
  console.log('Received chunk:', data.chunk);
});

eventSource.addEventListener('tool_call_start', (event) => {
  const data = JSON.parse(event.data);
  console.log('Tool call started:', data.tool);
});

eventSource.addEventListener('tool_call_update', (event) => {
  const data = JSON.parse(event.data);
  console.log('Tool call update:', data.status);
  if (data.output) {
    console.log('Tool output:', data.output);
  }
});

eventSource.addEventListener('message_end', (event) => {
  const data = JSON.parse(event.data);
  console.log('Assistant finished responding with message ID:', data.message_id);
  eventSource.close();
});

eventSource.onerror = (error) => {
  console.error('SSE error:', error);
  eventSource.close();
};
```

### Exporting a Conversation

```bash
# Export conversation as JSON
curl -X GET http://localhost:3000/api/conversations/conv-123456/export?format=json \
  -H "Authorization: Bearer your-token"

# Export conversation as Markdown
curl -X GET http://localhost:3000/api/conversations/conv-123456/export?format=markdown \
  -H "Authorization: Bearer your-token" \
  -o conversation.md

# Export conversation as PDF
curl -X GET http://localhost:3000/api/conversations/conv-123456/export?format=pdf \
  -H "Authorization: Bearer your-token" \
  -o conversation.pdf
```

## Related Resources

- [Agent API](./agent.md): Manage AI agents
- [Twin API](./twin.md): Manage digital twins
- [Tool API](./tool.md): Access and execute tools