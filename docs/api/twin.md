# Twin API

## Overview

The Twin API allows you to create, manage, and interact with digital twins in the Digital Twin Desktop application. Digital twins are virtual representations of physical systems that enable monitoring, simulation, and analysis.

## Endpoints

### List Twins

```
GET /api/twins
```

Retrieves a list of digital twins.

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 20, max: 100) |
| sort      | string | Field to sort by (e.g., created_at, -name)       |
| type      | string | Filter by twin type (equipment, system, etc.)    |
| status    | string | Filter by status (active, inactive, archived)    |
| search    | string | Search term to filter twins                      |

#### Response

```json
{
  "data": [
    {
      "id": "pump-101",
      "name": "Main Cooling Pump",
      "description": "Primary cooling pump for reactor system",
      "type": "equipment",
      "status": "active",
      "sensor_count": 3,
      "actuator_count": 2,
      "created_at": "2025-10-01T12:00:00Z",
      "updated_at": "2025-10-15T09:30:00Z"
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
    "self": "/api/twins?page=1&per_page=20",
    "next": "/api/twins?page=2&per_page=20"
  }
}
```

### Get Twin

```
GET /api/twins/{twin_id}
```

Retrieves a specific digital twin by ID.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |

#### Response

```json
{
  "data": {
    "id": "pump-101",
    "name": "Main Cooling Pump",
    "description": "Primary cooling pump for reactor system",
    "type": "equipment",
    "status": "active",
    "sensors": [
      {
        "id": "temperature",
        "name": "Temperature",
        "unit": "°C",
        "current_value": 65.5,
        "min_value": -10.0,
        "max_value": 100.0,
        "last_updated": "2025-11-17T03:57:00Z"
      },
      {
        "id": "pressure",
        "name": "Pressure",
        "unit": "bar",
        "current_value": 2.5,
        "min_value": 0.0,
        "max_value": 10.0,
        "last_updated": "2025-11-17T03:57:00Z"
      }
    ],
    "actuators": [
      {
        "id": "power",
        "name": "Power Switch",
        "type": "binary",
        "current_state": true,
        "last_updated": "2025-11-17T03:57:00Z"
      }
    ],
    "metadata": {
      "manufacturer": "FlowTech Industries",
      "model": "FT-3000",
      "installation_date": "2024-05-15"
    },
    "created_at": "2025-10-01T12:00:00Z",
    "updated_at": "2025-10-15T09:30:00Z"
  }
}
```

### Create Twin

```
POST /api/twins
```

Creates a new digital twin.

#### Request Body

```json
{
  "id": "heat-exchanger-1",
  "name": "Primary Heat Exchanger",
  "description": "Main heat exchanger for cooling system",
  "type": "equipment",
  "metadata": {
    "manufacturer": "HeatTech Inc",
    "model": "HT-5000",
    "installation_date": "2024-06-01"
  }
}
```

#### Required Fields

| Field | Type   | Description                                |
|-------|--------|--------------------------------------------|
| id    | string | Unique identifier for the twin             |
| name  | string | Display name of the twin                   |

#### Optional Fields

| Field       | Type   | Description                                |
|-------------|--------|-------------------------------------------|
| description | string | Description of the twin's purpose          |
| type        | string | Type of twin (equipment, system, etc.)     |
| metadata    | object | Additional metadata for the twin           |

#### Response

```json
{
  "data": {
    "id": "heat-exchanger-1",
    "name": "Primary Heat Exchanger",
    "description": "Main heat exchanger for cooling system",
    "type": "equipment",
    "status": "active",
    "sensor_count": 0,
    "actuator_count": 0,
    "sensors": [],
    "actuators": [],
    "metadata": {
      "manufacturer": "HeatTech Inc",
      "model": "HT-5000",
      "installation_date": "2024-06-01"
    },
    "created_at": "2025-11-17T03:58:00Z",
    "updated_at": "2025-11-17T03:58:00Z"
  }
}
```

### Update Twin

```
PATCH /api/twins/{twin_id}
```

Updates an existing digital twin.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |

#### Request Body

```json
{
  "name": "Primary Heat Exchanger (Updated)",
  "description": "Main heat exchanger for cooling system with enhanced monitoring",
  "status": "active",
  "metadata": {
    "manufacturer": "HeatTech Inc",
    "model": "HT-5000",
    "installation_date": "2024-06-01",
    "last_maintenance": "2025-11-01"
  }
}
```

#### Response

```json
{
  "data": {
    "id": "heat-exchanger-1",
    "name": "Primary Heat Exchanger (Updated)",
    "description": "Main heat exchanger for cooling system with enhanced monitoring",
    "type": "equipment",
    "status": "active",
    "sensor_count": 2,
    "actuator_count": 1,
    "metadata": {
      "manufacturer": "HeatTech Inc",
      "model": "HT-5000",
      "installation_date": "2024-06-01",
      "last_maintenance": "2025-11-01"
    },
    "created_at": "2025-11-17T03:58:00Z",
    "updated_at": "2025-11-17T04:00:00Z"
  }
}
```

### Delete Twin

```
DELETE /api/twins/{twin_id}
```

Deletes a digital twin.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |

#### Response

```
204 No Content
```

### Add Sensor

```
POST /api/twins/{twin_id}/sensors
```

Adds a sensor to a digital twin.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |

#### Request Body

```json
{
  "id": "flow_rate",
  "name": "Flow Rate",
  "description": "Pump flow rate measurement",
  "unit": "L/min",
  "current_value": 150.0,
  "min_value": 0.0,
  "max_value": 500.0
}
```

#### Required Fields

| Field | Type   | Description                                |
|-------|--------|--------------------------------------------|
| id    | string | Unique identifier for the sensor           |
| name  | string | Display name of the sensor                 |
| unit  | string | Unit of measurement                        |

#### Optional Fields

| Field         | Type   | Description                                |
|---------------|--------|-------------------------------------------|
| description   | string | Description of the sensor                  |
| current_value | number | Current sensor value                       |
| min_value     | number | Minimum expected value                     |
| max_value     | number | Maximum expected value                     |

#### Response

```json
{
  "data": {
    "id": "flow_rate",
    "name": "Flow Rate",
    "description": "Pump flow rate measurement",
    "unit": "L/min",
    "current_value": 150.0,
    "min_value": 0.0,
    "max_value": 500.0,
    "last_updated": "2025-11-17T04:00:00Z"
  }
}
```

### Update Sensor

```
PATCH /api/twins/{twin_id}/sensors/{sensor_id}
```

Updates a sensor on a digital twin.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |
| sensor_id | string | The sensor ID |

#### Request Body

```json
{
  "current_value": 155.5,
  "min_value": 0.0,
  "max_value": 600.0
}
```

#### Response

```json
{
  "data": {
    "id": "flow_rate",
    "name": "Flow Rate",
    "description": "Pump flow rate measurement",
    "unit": "L/min",
    "current_value": 155.5,
    "min_value": 0.0,
    "max_value": 600.0,
    "last_updated": "2025-11-17T04:01:00Z"
  }
}
```

### Remove Sensor

```
DELETE /api/twins/{twin_id}/sensors/{sensor_id}
```

Removes a sensor from a digital twin.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |
| sensor_id | string | The sensor ID |

#### Response

```
204 No Content
```

### Add Actuator

```
POST /api/twins/{twin_id}/actuators
```

Adds an actuator to a digital twin.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |

#### Request Body

```json
{
  "id": "speed_control",
  "name": "Speed Control",
  "description": "Pump speed control",
  "type": "continuous",
  "current_state": 0.75
}
```

#### Required Fields

| Field | Type   | Description                                |
|-------|--------|--------------------------------------------|
| id    | string | Unique identifier for the actuator         |
| name  | string | Display name of the actuator               |
| type  | string | Type of actuator (binary, continuous)      |

#### Optional Fields

| Field         | Type   | Description                                |
|---------------|--------|-------------------------------------------|
| description   | string | Description of the actuator                |
| current_state | number | Current actuator state                     |

#### Response

```json
{
  "data": {
    "id": "speed_control",
    "name": "Speed Control",
    "description": "Pump speed control",
    "type": "continuous",
    "current_state": 0.75,
    "last_updated": "2025-11-17T04:02:00Z"
  }
}
```

### Update Actuator

```
PATCH /api/twins/{twin_id}/actuators/{actuator_id}
```

Updates an actuator on a digital twin.

#### Path Parameters

| Parameter    | Type   | Description    |
|--------------|--------|-----------------|
| twin_id      | string | The twin ID    |
| actuator_id  | string | The actuator ID |

#### Request Body

```json
{
  "current_state": 0.85
}
```

#### Response

```json
{
  "data": {
    "id": "speed_control",
    "name": "Speed Control",
    "description": "Pump speed control",
    "type": "continuous",
    "current_state": 0.85,
    "last_updated": "2025-11-17T04:03:00Z"
  }
}
```

### Remove Actuator

```
DELETE /api/twins/{twin_id}/actuators/{actuator_id}
```

Removes an actuator from a digital twin.

#### Path Parameters

| Parameter    | Type   | Description    |
|--------------|--------|-----------------|
| twin_id      | string | The twin ID    |
| actuator_id  | string | The actuator ID |

#### Response

```
204 No Content
```

### Get Sensor History

```
GET /api/twins/{twin_id}/sensors/{sensor_id}/history
```

Retrieves historical data for a sensor.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |
| sensor_id | string | The sensor ID |

#### Query Parameters

| Parameter  | Type   | Description                                      |
|------------|--------|--------------------------------------------------|
| start_date | string | Start date for history (ISO 8601)                |
| end_date   | string | End date for history (ISO 8601)                  |
| interval   | string | Aggregation interval (1m, 5m, 1h, 1d)           |
| page       | number | Page number for pagination (default: 1)          |
| per_page   | number | Number of items per page (default: 100, max: 1000) |

#### Response

```json
{
  "data": [
    {
      "timestamp": "2025-11-17T04:00:00Z",
      "value": 150.0
    },
    {
      "timestamp": "2025-11-17T04:01:00Z",
      "value": 152.5
    },
    {
      "timestamp": "2025-11-17T04:02:00Z",
      "value": 155.5
    }
  ],
  "meta": {
    "pagination": {
      "total_items": 60,
      "total_pages": 1,
      "current_page": 1,
      "per_page": 100
    },
    "aggregation": {
      "interval": "1m",
      "start_date": "2025-11-17T04:00:00Z",
      "end_date": "2025-11-17T05:00:00Z"
    }
  }
}
```

### Get Twin Status

```
GET /api/twins/{twin_id}/status
```

Retrieves the current status of a digital twin.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |

#### Response

```json
{
  "data": {
    "id": "pump-101",
    "name": "Main Cooling Pump",
    "status": "active",
    "health": "good",
    "last_sync": "2025-11-17T03:57:00Z",
    "sensors": {
      "total": 3,
      "healthy": 3,
      "warning": 0,
      "error": 0
    },
    "actuators": {
      "total": 2,
      "operational": 2,
      "error": 0
    },
    "alerts": [
      {
        "id": "alert-123",
        "severity": "info",
        "message": "Routine maintenance scheduled for 2025-11-20",
        "created_at": "2025-11-17T03:00:00Z"
      }
    ]
  }
}
```

### Sync Twin

```
POST /api/twins/{twin_id}/sync
```

Synchronizes a digital twin with its physical counterpart.

#### Path Parameters

| Parameter | Type   | Description  |
|-----------|--------|--------------|
| twin_id   | string | The twin ID  |

#### Request Body

```json
{
  "force": false,
  "timeout": 30
}
```

#### Optional Fields

| Field   | Type    | Description                                |
|---------|---------|-------------------------------------------|
| force   | boolean | Force synchronization even if recently synced |
| timeout | number  | Synchronization timeout in seconds         |

#### Response

```json
{
  "data": {
    "id": "pump-101",
    "sync_status": "completed",
    "sync_time": "2025-11-17T04:05:00Z",
    "sensors_updated": 3,
    "actuators_updated": 2,
    "changes": {
      "temperature": {
        "old_value": 65.5,
        "new_value": 66.2
      },
      "pressure": {
        "old_value": 2.5,
        "new_value": 2.6
      }
    }
  }
}
```

## Models

### Digital Twin

| Field         | Type     | Description                                   |
|---------------|----------|-----------------------------------------------|
| id            | string   | Unique identifier for the twin                |
| name          | string   | Display name of the twin                      |
| description   | string   | Description of the twin's purpose             |
| type          | string   | Type of twin (equipment, system, etc.)        |
| status        | string   | Status of the twin (active, inactive, archived) |
| sensor_count  | number   | Number of sensors on the twin                 |
| actuator_count| number   | Number of actuators on the twin               |
| sensors       | array    | List of sensors                               |
| actuators     | array    | List of actuators                             |
| metadata      | object   | Additional metadata                           |
| created_at    | string   | Creation timestamp (ISO 8601)                 |
| updated_at    | string   | Last update timestamp (ISO 8601)              |

### Sensor

| Field         | Type   | Description                                   |
|---------------|--------|-----------------------------------------------|
| id            | string | Unique identifier for the sensor              |
| name          | string | Display name of the sensor                    |
| description   | string | Description of the sensor                     |
| unit          | string | Unit of measurement                           |
| current_value | number | Current sensor value                          |
| min_value     | number | Minimum expected value                        |
| max_value     | number | Maximum expected value                        |
| last_updated  | string | Last update timestamp (ISO 8601)              |

### Actuator

| Field         | Type   | Description                                   |
|---------------|--------|-----------------------------------------------|
| id            | string | Unique identifier for the actuator            |
| name          | string | Display name of the actuator                  |
| description   | string | Description of the actuator                   |
| type          | string | Type of actuator (binary, continuous)         |
| current_state | number | Current actuator state                        |
| last_updated  | string | Last update timestamp (ISO 8601)              |

## Error Codes

| Code              | Description                                   |
|-------------------|-----------------------------------------------|
| TWIN_NOT_FOUND    | The specified twin was not found              |
| SENSOR_NOT_FOUND  | The specified sensor was not found            |
| ACTUATOR_NOT_FOUND| The specified actuator was not found          |
| INVALID_TWIN_TYPE | The twin type is invalid                      |
| SENSOR_EXISTS     | A sensor with this ID already exists          |
| ACTUATOR_EXISTS   | An actuator with this ID already exists       |
| SYNC_FAILED       | Twin synchronization failed                   |

## Examples

### Creating a Digital Twin with Sensors

```bash
# Create a new twin
curl -X POST http://localhost:3000/api/twins \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "compressor-1",
    "name": "Air Compressor Unit 1",
    "description": "Main air compressor for production line",
    "type": "equipment",
    "metadata": {
      "manufacturer": "CompressAir Inc",
      "model": "CA-2000",
      "installation_date": "2024-03-15"
    }
  }'

# Add temperature sensor
curl -X POST http://localhost:3000/api/twins/compressor-1/sensors \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "temperature",
    "name": "Temperature",
    "unit": "°C",
    "current_value": 45.0,
    "min_value": 0.0,
    "max_value": 80.0
  }'

# Add pressure sensor
curl -X POST http://localhost:3000/api/twins/compressor-1/sensors \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "id": "pressure",
    "name": "Pressure",
    "unit": "bar",
    "current_value": 8.5,
    "min_value": 0.0,
    "max_value": 12.0
  }'
```

### Retrieving Sensor History

```bash
# Get temperature history for the last 24 hours
curl -X GET "http://localhost:3000/api/twins/compressor-1/sensors/temperature/history?start_date=2025-11-16T04:05:00Z&end_date=2025-11-17T04:05:00Z&interval=1h" \
  -H "Authorization: Bearer your-token"
```

### Synchronizing a Twin

```bash
# Synchronize the twin with its physical counterpart
curl -X POST http://localhost:3000/api/twins/compressor-1/sync \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "force": false,
    "timeout": 30
  }'
```

## Related Resources

- [Conversation API](./conversation.md): Manage conversations with agents
- [Agent API](./agent.md): Manage AI agents
- [Simulation API](./simulation.md): Run simulations on twins
- [Tool API](./tool.md): Access and execute tools