//! Digital Twin core domain models.
//!
//! This module defines the primary Digital Twin entity that represents
//! a virtual replica of a real-world entity, process, or system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a Digital Twin - a virtual replica of a real-world entity.
///
/// A Digital Twin maintains synchronized state with its physical counterpart,
/// processes sensor data, and provides AI-powered insights and interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwin {
    /// Unique identifier for the digital twin
    pub id: Uuid,
    
    /// Human-readable name of the digital twin
    pub name: String,
    
    /// Description of what this twin represents
    pub description: String,
    
    /// Type/category of the digital twin
    pub twin_type: TwinType,
    
    /// Current state of the digital twin
    pub state: TwinState,
    
    /// Associated AI agents for this twin
    pub agent_ids: Vec<Uuid>,
    
    /// Data source configurations
    pub data_sources: Vec<DataSource>,
    
    /// Current properties and measurements
    pub properties: TwinProperties,
    
    /// Synchronization settings
    pub sync_config: SyncConfiguration,
    
    /// Visualization and UI settings
    pub visualization_config: VisualizationConfig,
    
    /// Metadata for additional twin properties
    pub metadata: TwinMetadata,
    
    /// Timestamp when the twin was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the twin was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Last synchronization timestamp
    pub last_sync_at: Option<DateTime<Utc>>,
}

/// Represents the current operational state of a digital twin.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TwinState {
    /// Twin is active and synchronized
    Active,
    
    /// Twin is active but not receiving updates
    Idle,
    
    /// Twin is synchronizing with data sources
    Syncing,
    
    /// Twin has lost connection to data sources
    Disconnected,
    
    /// Twin encountered an error
    Error,
    
    /// Twin is paused by user
    Paused,
    
    /// Twin is archived (read-only)
    Archived,
}

/// Categories of digital twins based on what they represent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TwinType {
    /// Physical device or machine
    Device { 
        device_type: String,
        manufacturer: Option<String>,
        model: Option<String>,
    },
    
    /// Business or industrial process
    Process { 
        process_type: String,
        domain: String,
    },
    
    /// System or infrastructure
    System { 
        system_type: String,
        components: Vec<String>,
    },
    
    /// Person or user profile
    Person { 
        role: String,
        department: Option<String>,
    },
    
    /// Environment or space
    Environment { 
        environment_type: String,
        location: Option<String>,
    },
    
    /// Custom twin type
    Custom { 
        category: String,
        attributes: HashMap<String, String>,
    },
}

/// Configuration for a data source feeding the digital twin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// Unique identifier for the data source
    pub id: Uuid,
    
    /// Name of the data source
    pub name: String,
    
    /// Type of data source
    pub source_type: DataSourceType,
    
    /// Connection configuration
    pub connection_config: ConnectionConfig,
    
    /// Data mapping rules
    pub mappings: Vec<DataMapping>,
    
    /// Whether this source is currently active
    pub active: bool,
    
    /// Last successful connection timestamp
    pub last_connected: Option<DateTime<Utc>>,
}

/// Types of data sources that can feed a digital twin.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataSourceType {
    /// IoT sensor data
    Sensor { 
        protocol: String,
        sensor_type: String,
    },
    
    /// Database connection
    Database { 
        db_type: String,
        read_only: bool,
    },
    
    /// REST API endpoint
    RestApi { 
        base_url: String,
        auth_type: Option<String>,
    },
    
    /// Message queue subscriber
    MessageQueue { 
        queue_type: String,
        topic: String,
    },
    
    /// File system watcher
    FileSystem { 
        path: String,
        file_pattern: Option<String>,
    },
    
    /// Custom data source
    Custom { 
        source_id: String,
    },
}

/// Connection configuration for data sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Connection string or endpoint
    pub endpoint: String,
    
    /// Authentication credentials (encrypted)
    pub credentials: Option<HashMap<String, String>>,
    
    /// Connection timeout in seconds
    pub timeout_seconds: u32,
    
    /// Retry configuration
    pub retry_config: RetryConfig,
    
    /// Custom connection parameters
    pub custom_params: HashMap<String, serde_json::Value>,
}

/// Retry configuration for failed connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
    
    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f32,
    
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
}

/// Defines how data from a source maps to twin properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMapping {
    /// Source field or path
    pub source_field: String,
    
    /// Target property in the twin
    pub target_property: String,
    
    /// Data transformation rules
    pub transform: Option<TransformRule>,
    
    /// Data type of the mapped value
    pub data_type: DataType,
}

/// Data transformation rules for mappings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformRule {
    /// Simple mathematical operation
    Math { operation: String },
    
    /// Unit conversion
    UnitConvert { from_unit: String, to_unit: String },
    
    /// Custom script transformation
    Script { language: String, code: String },
    
    /// Value mapping lookup
    Lookup { map: HashMap<String, serde_json::Value> },
}

/// Supported data types for properties.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Json,
}

/// Current properties and state of the digital twin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinProperties {
    /// Static attributes that rarely change
    pub attributes: HashMap<String, serde_json::Value>,
    
    /// Dynamic measurements that update frequently
    pub measurements: HashMap<String, Measurement>,
    
    /// Computed/derived properties
    pub computed: HashMap<String, serde_json::Value>,
    
    /// Historical trends and statistics
    pub analytics: TwinAnalytics,
}

/// A measurement value with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    /// The current value
    pub value: serde_json::Value,
    
    /// Unit of measurement
    pub unit: Option<String>,
    
    /// Quality/confidence score (0.0 to 1.0)
    pub quality: f32,
    
    /// Timestamp of the measurement
    pub timestamp: DateTime<Utc>,
    
    /// Source that provided this measurement
    pub source_id: Option<Uuid>,
}

/// Analytics and statistics for the twin.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TwinAnalytics {
    /// Total number of updates received
    pub total_updates: u64,
    
    /// Health score (0.0 to 1.0)
    pub health_score: f32,
    
    /// Anomaly detection results
    pub anomalies: Vec<Anomaly>,
    
    /// Key performance indicators
    pub kpis: HashMap<String, f64>,
}

/// Represents a detected anomaly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Type of anomaly detected
    pub anomaly_type: String,
    
    /// Severity level (0.0 to 1.0)
    pub severity: f32,
    
    /// Description of the anomaly
    pub description: String,
    
    /// When the anomaly was detected
    pub detected_at: DateTime<Utc>,
    
    /// Related property or measurement
    pub related_property: Option<String>,
}

/// Configuration for twin synchronization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfiguration {
    /// Synchronization mode
    pub mode: SyncMode,
    
    /// Update interval in seconds (for periodic mode)
    pub interval_seconds: Option<u32>,
    
    /// Whether to sync on startup
    pub sync_on_startup: bool,
    
    /// Conflict resolution strategy
    pub conflict_strategy: ConflictStrategy,
    
    /// Data retention policy
    pub retention_policy: RetentionPolicy,
}

/// Synchronization modes for digital twins.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncMode {
    /// Real-time synchronization
    RealTime,
    
    /// Periodic batch updates
    Periodic,
    
    /// On-demand synchronization
    OnDemand,
    
    /// Event-driven updates
    EventDriven,
}

/// Strategy for resolving conflicts in data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictStrategy {
    /// Most recent update wins
    LastWriteWins,
    
    /// Source with highest priority wins
    PriorityBased,
    
    /// Manual resolution required
    Manual,
    
    /// Merge all updates
    Merge,
}

/// Data retention configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Keep raw data for this many days
    pub raw_data_days: Option<u32>,
    
    /// Keep aggregated data for this many days
    pub aggregated_data_days: Option<u32>,
    
    /// Aggregation intervals
    pub aggregation_intervals: Vec<String>,
}

/// Visualization configuration for the digital twin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    /// Default view type
    pub default_view: ViewType,
    
    /// Available dashboard layouts
    pub dashboard_layouts: Vec<DashboardLayout>,
    
    /// 3D model configuration (if applicable)
    pub model_3d: Option<Model3DConfig>,
    
    /// Chart and graph preferences
    pub chart_config: ChartConfig,
}

/// Types of views available for visualization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViewType {
    Dashboard,
    Model3D,
    Schematic,
    Timeline,
    Analytics,
}

/// Dashboard layout configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLayout {
    /// Layout identifier
    pub id: String,
    
    /// Layout name
    pub name: String,
    
    /// Widget configurations
    pub widgets: Vec<WidgetConfig>,
}

/// Widget configuration for dashboards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    /// Widget type
    pub widget_type: String,
    
    /// Position and size
    pub layout: WidgetLayout,
    
    /// Data bindings
    pub data_bindings: HashMap<String, String>,
    
    /// Widget-specific settings
    pub settings: HashMap<String, serde_json::Value>,
}

/// Widget layout information.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WidgetLayout {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// 3D model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model3DConfig {
    /// Model file URL or path
    pub model_url: String,
    
    /// Model format (GLTF, OBJ, etc.)
    pub format: String,
    
    /// Property-to-visual mappings
    pub property_mappings: HashMap<String, String>,
}

/// Chart configuration preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    /// Default time range for charts
    pub default_time_range: String,
    
    /// Preferred chart types for different data
    pub chart_preferences: HashMap<String, String>,
    
    /// Color scheme
    pub color_scheme: String,
}

/// Metadata for the digital twin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinMetadata {
    /// Version of the twin configuration
    pub version: String,
    
    /// Owner or responsible party
    pub owner: String,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Related documentation links
    pub documentation: Vec<String>,
    
    /// Custom metadata fields
    pub custom_fields: HashMap<String, serde_json::Value>,
}

impl DigitalTwin {
    /// Creates a new digital twin with basic configuration.
    pub fn new(name: String, description: String, twin_type: TwinType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            twin_type,
            state: TwinState::Idle,
            agent_ids: Vec::new(),
            data_sources: Vec::new(),
            properties: TwinProperties::default(),
            sync_config: SyncConfiguration::default(),
            visualization_config: VisualizationConfig::default(),
            metadata: TwinMetadata::default(),
            created_at: now,
            updated_at: now,
            last_sync_at: None,
        }
    }
    
    /// Adds an agent to this digital twin.
    pub fn add_agent(&mut self, agent_id: Uuid) {
        if !self.agent_ids.contains(&agent_id) {
            self.agent_ids.push(agent_id);
            self.updated_at = Utc::now();
        }
    }
    
    /// Adds a data source to this digital twin.
    pub fn add_data_source(&mut self, data_source: DataSource) {
        self.data_sources.push(data_source);
        self.updated_at = Utc::now();
    }
    
    /// Updates the twin's state.
    pub fn set_state(&mut self, state: TwinState) {
        self.state = state;
        self.updated_at = Utc::now();
    }
    
    /// Marks the twin as synchronized.
    pub fn mark_synchronized(&mut self) {
        self.last_sync_at = Some(Utc::now());
        self.updated_at = Utc::now();
        if self.state == TwinState::Syncing {
            self.state = TwinState::Active;
        }
    }
    
    /// Checks if the twin needs synchronization based on its configuration.
    pub fn needs_sync(&self) -> bool {
        match self.sync_config.mode {
            SyncMode::RealTime => false, // Always kept in sync
            SyncMode::OnDemand => false, // Only sync when requested
            SyncMode::Periodic => {
                if let (Some(interval), Some(last_sync)) = 
                    (self.sync_config.interval_seconds, self.last_sync_at) {
                    let elapsed = Utc::now().signed_duration_since(last_sync);
                    elapsed.num_seconds() as u32 >= interval
                } else {
                    true // Never synced
                }
            },
            SyncMode::EventDriven => false, // Sync triggered by events
        }
    }
    
    /// Gets active data sources.
    pub fn active_data_sources(&self) -> Vec<&DataSource> {
        self.data_sources
            .iter()
            .filter(|ds| ds.active)
            .collect()
    }
}

impl Default for TwinProperties {
    fn default() -> Self {
        Self {
            attributes: HashMap::new(),
            measurements: HashMap::new(),
            computed: HashMap::new(),
            analytics: TwinAnalytics::default(),
        }
    }
}

impl Default for SyncConfiguration {
    fn default() -> Self {
        Self {
            mode: SyncMode::Periodic,
            interval_seconds: Some(300), // 5 minutes
            sync_on_startup: true,
            conflict_strategy: ConflictStrategy::LastWriteWins,
            retention_policy: RetentionPolicy::default(),
        }
    }
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            raw_data_days: Some(30),
            aggregated_data_days: Some(365),
            aggregation_intervals: vec![
                "1h".to_string(),
                "1d".to_string(),
                "1w".to_string(),
            ],
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: 30000,
        }
    }
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            default_view: ViewType::Dashboard,
            dashboard_layouts: Vec::new(),
            model_3d: None,
            chart_config: ChartConfig::default(),
        }
    }
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            default_time_range: "24h".to_string(),
            chart_preferences: HashMap::new(),
            color_scheme: "default".to_string(),
        }
    }
}

impl Default for TwinMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            owner: "system".to_string(),
            tags: Vec::new(),
            documentation: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_digital_twin_creation() {
        let twin = DigitalTwin::new(
            "Test Device".to_string(),
            "A test IoT device".to_string(),
            TwinType::Device {
                device_type: "sensor".to_string(),
                manufacturer: Some("ACME".to_string()),
                model: Some("S1000".to_string()),
            },
        );
        
        assert_eq!(twin.name, "Test Device");
        assert_eq!(twin.state, TwinState::Idle);
        assert!(twin.agent_ids.is_empty());
        assert!(twin.data_sources.is_empty());
    }
    
    #[test]
    fn test_add_agent_to_twin() {
        let mut twin = DigitalTwin::new(
            "Test Twin".to_string(),
            "Description".to_string(),
            TwinType::System {
                system_type: "test".to_string(),
                components: vec![],
            },
        );
        
        let agent_id = Uuid::new_v4();
        twin.add_agent(agent_id);
        
        assert_eq!(twin.agent_ids.len(), 1);
        assert!(twin.agent_ids.contains(&agent_id));
        
        // Test duplicate prevention
        twin.add_agent(agent_id);
        assert_eq!(twin.agent_ids.len(), 1);
    }
    
    #[test]
    fn test_sync_needed() {
        let mut twin = DigitalTwin::new(
            "Test".to_string(),
            "Description".to_string(),
            TwinType::Custom {
                category: "test".to_string(),
                attributes: HashMap::new(),
            },
        );
        
        // Periodic sync mode with no last sync
        twin.sync_config.mode = SyncMode::Periodic;
        twin.sync_config.interval_seconds = Some(60);
        assert!(twin.needs_sync());
        
        // After marking synchronized
        twin.mark_synchronized();
        assert!(!twin.needs_sync());
        
        // Real-time mode never needs sync
        twin.sync_config.mode = SyncMode::RealTime;
        assert!(!twin.needs_sync());
    }
}