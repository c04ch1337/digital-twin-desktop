# Digital Twin Guide

## Overview

This guide provides comprehensive information on creating, configuring, and working with digital twins in the Digital Twin Desktop application. Digital twins are virtual representations of physical systems that allow for monitoring, simulation, and analysis.

## What is a Digital Twin?

A digital twin is a virtual representation of a physical object, system, or process that serves as a real-time digital counterpart. In the Digital Twin Desktop application, digital twins can represent:

- Industrial equipment and machinery
- IoT devices and sensor networks
- Manufacturing processes
- Building systems (HVAC, lighting, etc.)
- Energy systems
- And more

Digital twins enable:

1. **Real-time monitoring** of physical systems
2. **Simulation and prediction** of system behavior
3. **Anomaly detection** and preventive maintenance
4. **Optimization** of system parameters
5. **What-if analysis** for decision support

## Digital Twin Architecture

### Core Components

Each digital twin in the application consists of:

1. **Twin Model**: The core representation of the twin's structure and behavior
2. **Sensors**: Data points that reflect the state of the physical system
3. **Actuators**: Control points that can modify the physical system
4. **Simulation Engine**: For running simulations on the twin
5. **Historical Data**: Time-series data of past states and events
6. **Metadata**: Additional information about the twin

### Data Model

The digital twin data model is defined as:

```rust
pub struct DigitalTwin {
    id: String,
    name: String,
    description: Option<String>,
    twin_type: TwinType,
    sensors: HashMap<String, Sensor>,
    actuators: HashMap<String, Actuator>,
    metadata: HashMap<String, String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct Sensor {
    id: String,
    name: String,
    description: Option<String>,
    unit: String,
    current_value: f64,
    min_value: Option<f64>,
    max_value: Option<f64>,
    update_frequency: Option<Duration>,
    last_updated: DateTime<Utc>,
}

pub struct Actuator {
    id: String,
    name: String,
    description: Option<String>,
    actuator_type: ActuatorType,
    current_state: ActuatorState,
    last_updated: DateTime<Utc>,
}
```

## Creating a Digital Twin

### Using the UI

To create a digital twin using the UI:

1. Navigate to the "Twins" section in the sidebar
2. Click the "Create Twin" button
3. Fill in the required information:
   - Twin ID (unique identifier)
   - Twin Name (display name)
   - Twin Type (category of twin)
   - Description (optional)
4. Click "Create" to create the basic twin
5. Add sensors and actuators as needed

### Using the API

To create a digital twin programmatically:

```rust
// Create a new twin
let mut twin = DigitalTwin::new("pump-101", "Main Cooling Pump");
twin.set_description("Primary cooling pump for reactor system");
twin.set_twin_type(TwinType::Equipment);

// Add metadata
twin.add_metadata("manufacturer", "FlowTech Industries");
twin.add_metadata("model", "FT-3000");
twin.add_metadata("installation_date", "2024-05-15");

// Add sensors
twin.add_sensor(
    "temperature",
    "Temperature",
    "°C",
    25.0,
    Some(-10.0),
    Some(100.0)
).unwrap();

twin.add_sensor(
    "pressure",
    "Pressure",
    "bar",
    2.5,
    Some(0.0),
    Some(10.0)
).unwrap();

twin.add_sensor(
    "flow_rate",
    "Flow Rate",
    "L/min",
    150.0,
    Some(0.0),
    Some(500.0)
).unwrap();

// Add actuators
twin.add_actuator(
    "power",
    "Power Switch",
    ActuatorType::Binary,
    ActuatorState::Binary(true)
).unwrap();

twin.add_actuator(
    "speed",
    "Speed Control",
    ActuatorType::Continuous,
    ActuatorState::Continuous(0.75) // 75% speed
).unwrap();

// Save the twin
twin_service.save_twin(&twin).await?;
```

## Connecting to Physical Systems

Digital twins can be connected to physical systems through various protocols:

### Modbus Integration

For industrial equipment supporting Modbus:

```rust
// Create a Modbus connection configuration
let modbus_config = ModbusConfig {
    connection_type: ModbusConnectionType::Tcp,
    ip_address: "192.168.1.100".to_string(),
    port: 502,
    slave_id: 1,
    timeout: Duration::from_secs(2),
};

// Create sensor mappings
let sensor_mappings = vec![
    SensorMapping {
        sensor_id: "temperature".to_string(),
        register_type: RegisterType::HoldingRegister,
        register_address: 100,
        data_type: DataType::Float32,
        scaling_factor: 0.1,
    },
    SensorMapping {
        sensor_id: "pressure".to_string(),
        register_type: RegisterType::HoldingRegister,
        register_address: 102,
        data_type: DataType::Float32,
        scaling_factor: 0.01,
    },
];

// Create actuator mappings
let actuator_mappings = vec![
    ActuatorMapping {
        actuator_id: "power".to_string(),
        register_type: RegisterType::Coil,
        register_address: 1,
        data_type: DataType::Boolean,
    },
    ActuatorMapping {
        actuator_id: "speed".to_string(),
        register_type: RegisterType::HoldingRegister,
        register_address: 200,
        data_type: DataType::Uint16,
        scaling_factor: 0.01,
    },
];

// Create the connection
let connection = ModbusConnection::new(
    "pump-101-modbus",
    "Pump 101 Modbus Connection",
    modbus_config,
    sensor_mappings,
    actuator_mappings,
);

// Associate with the twin
twin_service.add_connection(&twin.id, connection).await?;
```

### MQTT Integration

For IoT devices using MQTT:

```rust
// Create an MQTT connection configuration
let mqtt_config = MqttConfig {
    broker_url: "mqtt://iot.example.com".to_string(),
    port: 1883,
    client_id: "digital-twin-desktop".to_string(),
    username: Some("user".to_string()),
    password: Some("password".to_string()),
    use_tls: false,
    qos: QosLevel::AtLeastOnce,
};

// Create sensor topic mappings
let sensor_mappings = vec![
    MqttSensorMapping {
        sensor_id: "temperature".to_string(),
        topic: "sensors/pump-101/temperature".to_string(),
        message_format: MessageFormat::Json,
        json_path: Some("$.value".to_string()),
    },
    MqttSensorMapping {
        sensor_id: "pressure".to_string(),
        topic: "sensors/pump-101/pressure".to_string(),
        message_format: MessageFormat::Json,
        json_path: Some("$.value".to_string()),
    },
];

// Create actuator topic mappings
let actuator_mappings = vec![
    MqttActuatorMapping {
        actuator_id: "power".to_string(),
        topic: "actuators/pump-101/power".to_string(),
        message_format: MessageFormat::Json,
        message_template: r#"{"command": "set_power", "value": {{value}}}"#.to_string(),
    },
];

// Create the connection
let connection = MqttConnection::new(
    "pump-101-mqtt",
    "Pump 101 MQTT Connection",
    mqtt_config,
    sensor_mappings,
    actuator_mappings,
);

// Associate with the twin
twin_service.add_connection(&twin.id, connection).await?;
```

### REST API Integration

For systems with REST APIs:

```rust
// Create a REST API connection configuration
let rest_config = RestApiConfig {
    base_url: "https://api.example.com/v1".to_string(),
    auth_type: AuthType::Bearer,
    auth_token: Some("your-api-token".to_string()),
    headers: HashMap::from([
        ("Content-Type".to_string(), "application/json".to_string()),
    ]),
    timeout: Duration::from_secs(5),
};

// Create sensor endpoint mappings
let sensor_mappings = vec![
    RestSensorMapping {
        sensor_id: "temperature".to_string(),
        http_method: HttpMethod::Get,
        endpoint: "/devices/pump-101/sensors/temperature".to_string(),
        polling_interval: Duration::from_secs(60),
        json_path: "$.value".to_string(),
    },
];

// Create actuator endpoint mappings
let actuator_mappings = vec![
    RestActuatorMapping {
        actuator_id: "power".to_string(),
        http_method: HttpMethod::Post,
        endpoint: "/devices/pump-101/actuators/power".to_string(),
        body_template: r#"{"value": {{value}}}"#.to_string(),
    },
];

// Create the connection
let connection = RestApiConnection::new(
    "pump-101-rest",
    "Pump 101 REST API Connection",
    rest_config,
    sensor_mappings,
    actuator_mappings,
);

// Associate with the twin
twin_service.add_connection(&twin.id, connection).await?;
```

## Monitoring Digital Twins

### Real-time Dashboard

The Digital Twin Desktop provides a real-time dashboard for monitoring twins:

1. Navigate to the "Twins" section
2. Select a twin from the list
3. View the dashboard showing:
   - Current sensor values
   - Sensor trends over time
   - Actuator states
   - Alerts and notifications
   - Key performance indicators

### Historical Data Analysis

To analyze historical data:

1. From the twin dashboard, click "Historical Data"
2. Select the sensors to analyze
3. Choose the time range
4. Apply filters or aggregations as needed
5. View the resulting charts and statistics

### Alerts and Notifications

Configure alerts for important events:

```rust
// Create an alert rule
let alert_rule = AlertRule::new(
    "high-temperature",
    "High Temperature Alert",
    "temperature",
    AlertCondition::GreaterThan(80.0),
    AlertSeverity::Warning,
    Some(Duration::from_secs(60)), // Minimum duration before triggering
);

// Add the alert rule to the twin
twin_service.add_alert_rule(&twin.id, alert_rule).await?;
```

## Simulating Digital Twins

### Creating a Simulation

To create a simulation:

```rust
// Create a simulation configuration
let sim_config = SimulationConfig {
    name: "Pump Failure Scenario".to_string(),
    description: Some("Simulates a gradual pump failure due to overheating".to_string()),
    duration: Duration::from_secs(3600), // 1 hour simulation
    time_step: Duration::from_secs(1),   // 1 second steps
    real_time_factor: 60.0,              // Run 60x faster than real-time
};

// Create simulation scenarios
let mut scenario = SimulationScenario::new("overheating");

// Add sensor behavior models
scenario.add_sensor_model(
    "temperature",
    SensorBehaviorModel::LinearRamp {
        initial_value: 25.0,
        final_value: 95.0,
        duration: Duration::from_secs(2700), // Ramp up over 45 minutes
    }
);

scenario.add_sensor_model(
    "flow_rate",
    SensorBehaviorModel::StepChange {
        initial_value: 150.0,
        changes: vec![
            StepChange {
                time_offset: Duration::from_secs(1800), // After 30 minutes
                new_value: 100.0,
            },
            StepChange {
                time_offset: Duration::from_secs(2700), // After 45 minutes
                new_value: 50.0,
            },
            StepChange {
                time_offset: Duration::from_secs(3300), // After 55 minutes
                new_value: 0.0,
            },
        ],
    }
);

// Create the simulation
let simulation = Simulation::new(
    "pump-101-failure-sim",
    twin.id.clone(),
    sim_config,
    vec![scenario],
);

// Save the simulation
simulation_service.save_simulation(&simulation).await?;
```

### Running a Simulation

To run a simulation:

```rust
// Run the simulation
let sim_run = simulation_service.start_simulation(
    &simulation.id,
    Some("overheating"), // Scenario name
    None,                // Use default configuration
).await?;

// Get simulation results
let results = simulation_service.get_simulation_results(&sim_run.id).await?;
```

### Analyzing Simulation Results

After running a simulation:

1. Navigate to the "Simulations" section
2. Select the completed simulation run
3. View the results:
   - Sensor value charts
   - Event timeline
   - KPI summary
   - Alert occurrences
4. Export results for further analysis

## Advanced Features

### Twin Templates

Create reusable templates for common twin types:

```rust
// Create a pump template
let template = TwinTemplate::new(
    "industrial-pump",
    "Industrial Pump Template",
    Some("Template for standard industrial pumps"),
);

// Define standard sensors
template.add_sensor_template(
    "temperature",
    "Temperature",
    "°C",
    25.0,
    Some(-10.0),
    Some(100.0),
);

template.add_sensor_template(
    "pressure",
    "Pressure",
    "bar",
    2.5,
    Some(0.0),
    Some(10.0),
);

template.add_sensor_template(
    "flow_rate",
    "Flow Rate",
    "L/min",
    150.0,
    Some(0.0),
    Some(500.0),
);

// Define standard actuators
template.add_actuator_template(
    "power",
    "Power Switch",
    ActuatorType::Binary,
);

template.add_actuator_template(
    "speed",
    "Speed Control",
    ActuatorType::Continuous,
);

// Save the template
twin_service.save_template(&template).await?;

// Create a twin from the template
let twin = twin_service.create_from_template(
    "pump-102",
    "Secondary Cooling Pump",
    "industrial-pump",
).await?;
```

### Twin Hierarchies

Create hierarchical relationships between twins:

```rust
// Create a parent twin for a cooling system
let mut cooling_system = DigitalTwin::new(
    "cooling-system-1",
    "Primary Cooling System",
);

// Add child twins
twin_service.add_child_twin(
    &cooling_system.id,
    "pump-101", // Reference to existing twin
    "primary_pump",
).await?;

twin_service.add_child_twin(
    &cooling_system.id,
    "pump-102", // Reference to existing twin
    "secondary_pump",
).await?;

twin_service.add_child_twin(
    &cooling_system.id,
    "heat-exchanger-1", // Reference to existing twin
    "main_exchanger",
).await?;

// Query the hierarchy
let system_with_children = twin_service.get_twin_with_children(
    &cooling_system.id,
).await?;
```

### Custom Behaviors

Define custom behaviors for twins:

```rust
// Define a custom behavior
let behavior = TwinBehavior::new(
    "temperature_control",
    "Temperature Control Logic",
);

// Add behavior logic using a Rust script
behavior.set_script(r#"
// This script controls the pump speed based on temperature
fn update(twin: &mut DigitalTwin) -> Result<(), String> {
    // Get current temperature
    let temp = twin.get_sensor_value("temperature")?;
    
    // Calculate desired speed based on temperature
    let desired_speed = if temp < 40.0 {
        // Normal operation
        0.75
    } else if temp < 60.0 {
        // Increased cooling
        0.9
    } else if temp < 80.0 {
        // Maximum cooling
        1.0
    } else {
        // Emergency shutdown
        0.0
    };
    
    // Update the speed actuator
    twin.set_actuator_value("speed", ActuatorState::Continuous(desired_speed))?;
    
    Ok(())
}
"#);

// Add the behavior to the twin
twin_service.add_behavior(&twin.id, behavior).await?;
```

## Best Practices

### Twin Design

1. **Use meaningful IDs**: Choose IDs that reflect the physical asset's identification
2. **Standardize naming**: Use consistent naming conventions for sensors and actuators
3. **Include metadata**: Add relevant metadata for better searchability and context
4. **Set value ranges**: Define min/max values for sensors to enable validation
5. **Group related twins**: Use hierarchies to represent system relationships

### Data Management

1. **Set appropriate update frequencies**: Balance between data resolution and storage
2. **Configure data retention**: Define how long historical data should be kept
3. **Use data aggregation**: Aggregate older data to reduce storage requirements
4. **Export important data**: Regularly export critical data for backup
5. **Validate incoming data**: Implement validation rules for sensor data

### Simulation

1. **Start simple**: Begin with simple simulations and add complexity incrementally
2. **Validate models**: Compare simulation results with real-world data
3. **Use realistic scenarios**: Create scenarios based on real operational conditions
4. **Document assumptions**: Document all assumptions made in simulation models
5. **Review results critically**: Analyze simulation results for validity

## Troubleshooting

### Connection Issues

If a twin is not receiving data from the physical system:

1. Check the connection configuration (IP address, credentials, etc.)
2. Verify that the physical system is online and accessible
3. Check network connectivity and firewall settings
4. Inspect the connection logs for error messages
5. Test the connection using diagnostic tools

### Data Quality Issues

If sensor data appears incorrect:

1. Check sensor mappings and scaling factors
2. Verify the sensor's physical installation and calibration
3. Look for interference or environmental factors
4. Check for data conversion or precision issues
5. Compare with alternative measurement sources

### Performance Issues

If the application is slow when working with twins:

1. Reduce the number of active connections
2. Decrease data polling frequencies
3. Optimize database queries and indexes
4. Limit the amount of historical data loaded
5. Close unused simulations and visualizations

## Conclusion

Digital twins provide a powerful way to monitor, simulate, and optimize physical systems. By following this guide, you can create effective digital twins that provide valuable insights and enable better decision-making.