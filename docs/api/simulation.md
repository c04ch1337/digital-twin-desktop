# Simulation API

## Overview

The Simulation API allows you to create, run, and manage simulations on digital twins in the Digital Twin Desktop application. Simulations enable you to test scenarios, predict system behavior, and optimize parameters without affecting physical systems.

## Endpoints

### List Simulations

```
GET /api/simulations
```

Retrieves a list of simulations.

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| page      | number | Page number for pagination (default: 1)          |
| per_page  | number | Number of items per page (default: 20, max: 100) |
| sort      | string | Field to sort by (e.g., created_at, -name)       |
| twin_id   | string | Filter by associated digital twin ID             |
| status    | string | Filter by status (draft, running, completed, failed) |
| search    | string | Search term to filter simulations                |

#### Response

```json
{
  "data": [
    {
      "id": "sim-001",
      "name": "Pump Failure Scenario",
      "description": "Simulates a gradual pump failure due to overheating",
      "twin_id": "pump-101",
      "status": "completed",
      "duration": 3600,
      "created_at": "2025-10-15T14:30:00Z",
      "updated_at": "2025-10-15T15:45:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total_items": 12,
      "total_pages": 1,
      "current_page": 1,
      "per_page": 20
    }
  }
}
```

### Get Simulation

```
GET /api/simulations/{simulation_id}
```

Retrieves a specific simulation by ID.

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Response

```json
{
  "data": {
    "id": "sim-001",
    "name": "Pump Failure Scenario",
    "description": "Simulates a gradual pump failure due to overheating",
    "twin_id": "pump-101",
    "status": "completed",
    "duration": 3600,
    "time_step": 1,
    "real_time_factor": 60.0,
    "scenario": "overheating",
    "parameters": {
      "initial_temperature": 25.0,
      "target_temperature": 95.0,
      "ramp_duration": 2700
    },
    "results": {
      "total_steps": 3600,
      "completed_steps": 3600,
      "final_state": {
        "temperature": 95.0,
        "pressure": 0.5,
        "flow_rate": 0.0
      },
      "alerts": [
        {
          "step": 1800,
          "severity": "warning",
          "message": "Temperature exceeding normal range"
        },
        {
          "step": 2700,
          "severity": "critical",
          "message": "Pump failure imminent"
        }
      ]
    },
    "created_at": "2025-10-15T14:30:00Z",
    "updated_at": "2025-10-15T15:45:00Z"
  }
}
```

### Create Simulation

```
POST /api/simulations
```

Creates a new simulation.

#### Request Body

```json
{
  "name": "Heat Exchanger Efficiency Test",
  "description": "Tests heat exchanger efficiency under various load conditions",
  "twin_id": "heat-exchanger-1",
  "duration": 7200,
  "time_step": 10,
  "real_time_factor": 120.0,
  "scenario": "load_variation",
  "parameters": {
    "initial_load": 0.5,
    "load_steps": [0.6, 0.7, 0.8, 0.9, 1.0],
    "step_duration": 1200
  }
}
```

#### Required Fields

| Field   | Type   | Description                                |
|---------|--------|-------------------------------------------|
| name    | string | Name of the simulation                     |
| twin_id | string | ID of the digital twin to simulate         |

#### Optional Fields

| Field            | Type   | Description                                |
|------------------|--------|-------------------------------------------|
| description      | string | Description of the simulation              |
| duration         | number | Simulation duration in seconds             |
| time_step        | number | Time step in seconds (default: 1)          |
| real_time_factor | number | Speed multiplier (default: 1.0)            |
| scenario         | string | Predefined scenario name                   |
| parameters       | object | Custom simulation parameters               |

#### Response

```json
{
  "data": {
    "id": "sim-002",
    "name": "Heat Exchanger Efficiency Test",
    "description": "Tests heat exchanger efficiency under various load conditions",
    "twin_id": "heat-exchanger-1",
    "status": "draft",
    "duration": 7200,
    "time_step": 10,
    "real_time_factor": 120.0,
    "scenario": "load_variation",
    "parameters": {
      "initial_load": 0.5,
      "load_steps": [0.6, 0.7, 0.8, 0.9, 1.0],
      "step_duration": 1200
    },
    "created_at": "2025-11-17T04:00:00Z",
    "updated_at": "2025-11-17T04:00:00Z"
  }
}
```

### Update Simulation

```
PATCH /api/simulations/{simulation_id}
```

Updates an existing simulation (only for draft simulations).

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Request Body

```json
{
  "name": "Heat Exchanger Efficiency Test (Updated)",
  "description": "Tests heat exchanger efficiency under various load conditions with enhanced monitoring",
  "duration": 9000,
  "parameters": {
    "initial_load": 0.5,
    "load_steps": [0.6, 0.7, 0.8, 0.9, 1.0],
    "step_duration": 1500
  }
}
```

#### Response

```json
{
  "data": {
    "id": "sim-002",
    "name": "Heat Exchanger Efficiency Test (Updated)",
    "description": "Tests heat exchanger efficiency under various load conditions with enhanced monitoring",
    "twin_id": "heat-exchanger-1",
    "status": "draft",
    "duration": 9000,
    "time_step": 10,
    "real_time_factor": 120.0,
    "scenario": "load_variation",
    "parameters": {
      "initial_load": 0.5,
      "load_steps": [0.6, 0.7, 0.8, 0.9, 1.0],
      "step_duration": 1500
    },
    "created_at": "2025-11-17T04:00:00Z",
    "updated_at": "2025-11-17T04:05:00Z"
  }
}
```

### Delete Simulation

```
DELETE /api/simulations/{simulation_id}
```

Deletes a simulation.

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Response

```
204 No Content
```

### Start Simulation

```
POST /api/simulations/{simulation_id}/start
```

Starts a simulation run.

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Request Body

```json
{
  "scenario": "load_variation",
  "parameters": {
    "initial_load": 0.5,
    "load_steps": [0.6, 0.7, 0.8, 0.9, 1.0],
    "step_duration": 1200
  }
}
```

#### Response

```json
{
  "data": {
    "id": "sim-002",
    "name": "Heat Exchanger Efficiency Test",
    "twin_id": "heat-exchanger-1",
    "status": "running",
    "run_id": "run-001",
    "progress": {
      "total_steps": 900,
      "completed_steps": 0,
      "percentage": 0
    },
    "started_at": "2025-11-17T04:10:00Z"
  }
}
```

### Stop Simulation

```
POST /api/simulations/{simulation_id}/stop
```

Stops a running simulation.

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Response

```json
{
  "data": {
    "id": "sim-002",
    "name": "Heat Exchanger Efficiency Test",
    "twin_id": "heat-exchanger-1",
    "status": "stopped",
    "run_id": "run-001",
    "progress": {
      "total_steps": 900,
      "completed_steps": 450,
      "percentage": 50
    },
    "stopped_at": "2025-11-17T04:15:00Z"
  }
}
```

### Get Simulation Progress

```
GET /api/simulations/{simulation_id}/progress
```

Retrieves the progress of a running simulation.

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Response

```json
{
  "data": {
    "id": "sim-002",
    "status": "running",
    "run_id": "run-001",
    "progress": {
      "total_steps": 900,
      "completed_steps": 450,
      "percentage": 50,
      "estimated_time_remaining": 450
    },
    "current_state": {
      "timestamp": "2025-11-17T04:12:30Z",
      "temperature": 65.0,
      "pressure": 5.5,
      "flow_rate": 200.0
    }
  }
}
```

### Get Simulation Results

```
GET /api/simulations/{simulation_id}/results
```

Retrieves the results of a completed simulation.

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| format    | string | Result format (json, csv, html)                  |
| include   | string | Comma-separated list of result sections to include |

#### Response

```json
{
  "data": {
    "id": "sim-002",
    "name": "Heat Exchanger Efficiency Test",
    "twin_id": "heat-exchanger-1",
    "status": "completed",
    "run_id": "run-001",
    "summary": {
      "total_steps": 900,
      "completed_steps": 900,
      "duration": 7200,
      "start_time": "2025-11-17T04:10:00Z",
      "end_time": "2025-11-17T06:10:00Z"
    },
    "sensor_data": [
      {
        "sensor_id": "temperature",
        "min_value": 25.0,
        "max_value": 85.0,
        "average_value": 55.0,
        "final_value": 85.0
      },
      {
        "sensor_id": "pressure",
        "min_value": 2.0,
        "max_value": 9.5,
        "average_value": 5.75,
        "final_value": 9.5
      }
    ],
    "alerts": [
      {
        "step": 450,
        "timestamp": "2025-11-17T05:10:00Z",
        "severity": "warning",
        "message": "Temperature exceeding normal range"
      },
      {
        "step": 750,
        "timestamp": "2025-11-17T05:50:00Z",
        "severity": "critical",
        "message": "System efficiency degrading"
      }
    ],
    "performance_metrics": {
      "average_efficiency": 0.87,
      "peak_efficiency": 0.95,
      "minimum_efficiency": 0.72,
      "efficiency_trend": "declining"
    }
  }
}
```

### Export Simulation Results

```
GET /api/simulations/{simulation_id}/export
```

Exports simulation results in the specified format.

#### Path Parameters

| Parameter      | Type   | Description        |
|----------------|--------|--------------------|
| simulation_id  | string | The simulation ID  |

#### Query Parameters

| Parameter | Type   | Description                                      |
|-----------|--------|--------------------------------------------------|
| format    | string | Export format (json, csv, pdf, html)             |

#### Response

For JSON format, returns the simulation results as JSON. For other formats, returns a file download.

### List Simulation Scenarios

```
GET /api/simulations/scenarios
```

Retrieves available predefined simulation scenarios.

#### Response

```json
{
  "data": [
    {
      "id": "overheating",
      "name": "Overheating Scenario",
      "description": "Simulates a gradual temperature increase leading to system failure",
      "parameters": {
        "initial_temperature": 25.0,
        "target_temperature": 95.0,
        "ramp_duration": 2700
      },
      "applicable_twin_types": ["equipment", "system"]
    },
    {
      "id": "load_variation",
      "name": "Load Variation Scenario",
      "description": "Tests system behavior under varying load conditions",
      "parameters": {
        "initial_load": 0.5,
        "load_steps": [0.6, 0.7, 0.8, 0.9, 1.0],
        "step_duration": 1200
      },
      "applicable_twin_types": ["equipment", "system"]
    },
    {
      "id": "component_failure",
      "name": "Component Failure Scenario",
      "description": "Simulates the failure of a critical component",
      "parameters": {
        "failure_time": 1800,
        "failure_mode": "gradual"
      },
      "applicable_twin_types": ["equipment"]
    }
  ]
}
```

### Get Simulation Scenario

```
GET /api/simulations/scenarios/{scenario_id}
```

Retrieves details about a specific simulation scenario.

#### Path Parameters

| Parameter    | Type   | Description      |
|--------------|--------|------------------|
| scenario_id  | string | The scenario ID  |

#### Response

```json
{
  "data": {
    "id": "overheating",
    "name": "Overheating Scenario",
    "description": "Simulates a gradual temperature increase leading to system failure",
    "long_description": "This scenario simulates a realistic overheating failure mode where the system gradually increases in temperature due to reduced cooling capacity or increased load. The scenario includes realistic sensor readings and system responses.",
    "parameters": {
      "initial_temperature": {
        "type": "number",
        "description": "Starting temperature in °C",
        "default": 25.0,
        "min": 0.0,
        "max": 50.0
      },
      "target_temperature": {
        "type": "number",
        "description": "Target temperature in °C",
        "default": 95.0,
        "min": 50.0,
        "max": 150.0
      },
      "ramp_duration": {
        "type": "number",
        "description": "Duration of temperature ramp in seconds",
        "default": 2700,
        "min": 600,
        "max": 10800
      }
    },
    "applicable_twin_types": ["equipment", "system"],
    "expected_outcomes": [
      "Temperature exceeds warning threshold",
      "System efficiency decreases",
      "Potential component failure"
    ]
  }
}
```

## Models

### Simulation

| Field            | Type     | Description                                   |
|------------------|----------|-----------------------------------------------|
| id               | string   | Unique identifier for the simulation          |
| name             | string   | Name of the simulation                        |
| description      | string   | Description of the simulation                 |
| twin_id          | string   | ID of the digital twin being simulated        |
| status           | string   | Status (draft, running, completed, failed)    |
| duration         | number   | Simulation duration in seconds                |
| time_step        | number   | Time step in seconds                          |
| real_time_factor | number   | Speed multiplier relative to real time        |
| scenario         | string   | Predefined scenario name                      |
| parameters       | object   | Custom simulation parameters                  |
| results          | object   | Simulation results (if completed)             |
| created_at       | string   | Creation timestamp (ISO 8601)                 |
| updated_at       | string   | Last update timestamp (ISO 8601)              |

### Simulation Scenario

| Field                  | Type     | Description                                   |
|------------------------|----------|-----------------------------------------------|
| id                     | string   | Unique identifier for the scenario            |
| name                   | string   | Name of the scenario                          |
| description            | string   | Short description of the scenario             |
| long_description       | string   | Detailed description of the scenario          |
| parameters             | object   | Parameter definitions for the scenario        |
| applicable_twin_types  | string[] | Types of twins this scenario applies to       |
| expected_outcomes      | string[] | Expected outcomes of the scenario             |

## Error Codes

| Code                    | Description                                   |
|-------------------------|-----------------------------------------------|
| SIMULATION_NOT_FOUND    | The specified simulation was not found        |
| TWIN_NOT_FOUND          | The specified twin was not found              |
| SCENARIO_NOT_FOUND      | The specified scenario was not found          |
| SIMULATION_RUNNING      | Cannot modify a running simulation            |
| SIMULATION_NOT_RUNNING  | Cannot stop a simulation that is not running  |
| INVALID_PARAMETERS      | The simulation parameters are invalid         |

## Examples

### Creating and Running a Simulation

```bash
# Create a new simulation
curl -X POST http://localhost:3000/api/simulations \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Pump Efficiency Test",
    "description": "Tests pump efficiency under various operating conditions",
    "twin_id": "pump-101",
    "duration": 3600,
    "time_step": 1,
    "real_time_factor": 60.0,
    "scenario": "load_variation",
    "parameters": {
      "initial_load": 0.5,
      "load_steps": [0.6, 0.7, 0.8, 0.9, 1.0],
      "step_duration": 600
    }
  }'

# Start the simulation
curl -X POST http://localhost:3000/api/simulations/sim-003/start \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{
    "scenario": "load_variation"
  }'

# Check progress
curl -X GET http://localhost:3000/api/simulations/sim-003/progress \
  -H "Authorization: Bearer your-token"

# Get results when completed
curl -X GET http://localhost:3000/api/simulations/sim-003/results \
  -H "Authorization: Bearer your-token"
```

## Related Resources

- [Twin API](./twin.md): Manage digital twins
- [Conversation API](./conversation.md): Discuss simulations with agents
- [Agent API](./agent.md): Use agents to analyze simulation results