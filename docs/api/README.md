# Digital Twin Desktop API Documentation

## Overview

This directory contains comprehensive documentation for the Digital Twin Desktop API. The API provides programmatic access to the Digital Twin Desktop application, allowing developers to integrate with and extend the functionality of the platform.

## API Design Principles

The Digital Twin Desktop API follows these design principles:

1. **RESTful Design**: The API follows REST principles with resource-based URLs, appropriate HTTP methods, and standard status codes.
2. **Consistent Structure**: All API endpoints follow a consistent structure and naming convention.
3. **Comprehensive Documentation**: Each endpoint is thoroughly documented with examples and schema definitions.
4. **Versioning**: The API is versioned to ensure backward compatibility.
5. **Security**: The API implements authentication, authorization, and rate limiting.

## Authentication

The API supports JWT-based authentication:

```bash
# Request a token
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"your-username","password":"your-password"}'

# Response
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2025-11-18T03:45:19Z"
}

# Use the token in subsequent requests
curl -X GET http://localhost:3000/api/twins \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

## API Sections

The API is organized into the following sections:

1. [**Conversation API**](./conversation.md): Manage conversations with AI agents
2. [**Agent API**](./agent.md): Configure and interact with AI agents
3. [**Twin API**](./twin.md): Create and manage digital twins
4. [**Simulation API**](./simulation.md): Run and manage simulations
5. [**Tool API**](./tool.md): Access and execute tools

## Common Patterns

### Request Format

All request bodies should be sent as JSON with the appropriate `Content-Type` header:

```
Content-Type: application/json
```

### Response Format

All responses follow a consistent format:

```json
{
  "data": { ... },  // The response data (object or array)
  "meta": { ... },  // Metadata about the response (pagination, etc.)
  "links": { ... }  // HATEOAS links for navigation
}
```

### Error Format

Errors follow a consistent format:

```json
{
  "error": {
    "code": "RESOURCE_NOT_FOUND",
    "message": "The requested resource was not found",
    "details": { ... }  // Additional error details
  }
}
```

### Pagination

List endpoints support pagination using the following query parameters:

- `page`: The page number (1-based)
- `per_page`: The number of items per page

Example:

```
GET /api/twins?page=2&per_page=10
```

Response includes pagination metadata:

```json
{
  "data": [ ... ],
  "meta": {
    "pagination": {
      "total_items": 45,
      "total_pages": 5,
      "current_page": 2,
      "per_page": 10
    }
  },
  "links": {
    "self": "/api/twins?page=2&per_page=10",
    "first": "/api/twins?page=1&per_page=10",
    "prev": "/api/twins?page=1&per_page=10",
    "next": "/api/twins?page=3&per_page=10",
    "last": "/api/twins?page=5&per_page=10"
  }
}
```

### Filtering

List endpoints support filtering using query parameters:

```
GET /api/twins?type=equipment&status=active
```

### Sorting

List endpoints support sorting using the `sort` query parameter:

```
GET /api/twins?sort=name  # Sort by name ascending
GET /api/twins?sort=-created_at  # Sort by creation date descending
```

### Field Selection

Endpoints support field selection using the `fields` query parameter:

```
GET /api/twins?fields=id,name,status
```

## Status Codes

The API uses standard HTTP status codes:

- `200 OK`: The request was successful
- `201 Created`: A resource was successfully created
- `204 No Content`: The request was successful but there is no content to return
- `400 Bad Request`: The request was malformed or invalid
- `401 Unauthorized`: Authentication is required or failed
- `403 Forbidden`: The authenticated user doesn't have permission
- `404 Not Found`: The requested resource was not found
- `409 Conflict`: The request conflicts with the current state
- `422 Unprocessable Entity`: Validation errors
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: An unexpected error occurred

## Rate Limiting

The API implements rate limiting to prevent abuse. Rate limit information is included in the response headers:

```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 59
X-RateLimit-Reset: 1605060000
```

When the rate limit is exceeded, the API returns a `429 Too Many Requests` response.

## Versioning

The API is versioned to ensure backward compatibility. The version is specified in the URL:

```
/api/v1/twins
```

## WebSocket API

In addition to the REST API, the Digital Twin Desktop provides a WebSocket API for real-time updates:

```javascript
// Connect to the WebSocket API
const socket = new WebSocket('ws://localhost:3000/api/ws');

// Listen for messages
socket.addEventListener('message', (event) => {
  const data = JSON.parse(event.data);
  console.log('Received message:', data);
});

// Subscribe to twin updates
socket.send(JSON.stringify({
  type: 'subscribe',
  channel: 'twin:123'
}));
```

## API Client Libraries

The Digital Twin Desktop provides client libraries for easy integration:

- [JavaScript/TypeScript Client](https://github.com/your-org/digital-twin-desktop-js)
- [Python Client](https://github.com/your-org/digital-twin-desktop-python)
- [Rust Client](https://github.com/your-org/digital-twin-desktop-rust)

## Examples

### Creating a Digital Twin

```bash
curl -X POST http://localhost:3000/api/twins \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "pump-101",
    "name": "Main Cooling Pump",
    "description": "Primary cooling pump for reactor system",
    "twin_type": "equipment",
    "metadata": {
      "manufacturer": "FlowTech Industries",
      "model": "FT-3000",
      "installation_date": "2024-05-15"
    }
  }'
```

### Adding a Sensor to a Twin

```bash
curl -X POST http://localhost:3000/api/twins/pump-101/sensors \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "temperature",
    "name": "Temperature",
    "description": "Pump motor temperature",
    "unit": "°C",
    "current_value": 25.0,
    "min_value": -10.0,
    "max_value": 100.0
  }'
```

### Starting a Simulation

```bash
curl -X POST http://localhost:3000/api/simulations \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "twin_id": "pump-101",
    "name": "Pump Failure Scenario",
    "description": "Simulates a gradual pump failure due to overheating",
    "duration": 3600,
    "time_step": 1,
    "real_time_factor": 60.0,
    "scenario": "overheating"
  }'
```

## API Reference

For detailed information about each API endpoint, refer to the following documentation:

- [Conversation API](./conversation.md)
- [Agent API](./agent.md)
- [Twin API](./twin.md)
- [Simulation API](./simulation.md)
- [Tool API](./tool.md)

## OpenAPI Specification

The complete OpenAPI specification for the Digital Twin Desktop API is available at:

```
http://localhost:3000/api/docs/openapi.json
```

A Swagger UI for interactive exploration is available at:

```
http://localhost:3000/api/docs
```

## Postman Collection

A Postman collection for the Digital Twin Desktop API is available for download:

[Download Postman Collection](https://example.com/digital-twin-desktop-postman.json)

## Support

For API support, please contact:

- Email: api-support@example.com
- Developer Forum: https://forum.example.com/digital-twin-desktop-api
- GitHub Issues: https://github.com/your-org/digital-twin-desktop/issues