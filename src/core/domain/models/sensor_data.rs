//! Sensor data domain models for the Digital Twin Desktop.
//!
//! This module defines entities for handling sensor readings, data streams,
//! and real-time telemetry from connected devices and systems.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a collection of sensor data from a specific source.
///
/// SensorData encapsulates time-series data from physical or virtual sensors,
/// including metadata about the sensor and its readings over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    /// Unique identifier for this sensor data collection
    pub id: Uuid,
    
    /// The digital twin this data belongs to
    pub twin_id: Uuid,
    
    /// The specific sensor generating this data
    pub sensor: SensorInfo,
    
    /// Collection of readings over time
    pub readings: Vec<SensorReading>,
    
    /// Data quality metrics
    pub quality_metrics: DataQualityMetrics,
    
    /// Aggregated statistics for this data set
    pub statistics: SensorStatistics,
    
    /// Data processing pipeline configuration
    pub processing_config: ProcessingConfig,
    
    /// Metadata for this sensor data
    pub metadata: SensorDataMetadata,
    
    /// Timestamp when this data collection was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when this data was last updated
    pub updated_at: DateTime<Utc>,
}

/// Information about a sensor device or data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorInfo {
    /// Unique identifier for the sensor
    pub sensor_id: String,
    
    /// Human-readable name of the sensor
    pub name: String,
    
    /// Type of sensor
    pub sensor_type: SensorType,
    
    /// Physical or virtual location of the sensor
    pub location: Option<SensorLocation>,
    
    /// Sensor specifications
    pub specifications: SensorSpecifications,
    
    /// Current operational status
    pub status: SensorStatus,
    
    /// Calibration information
    pub calibration: Option<CalibrationInfo>,
}

/// Represents a single reading from a sensor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    /// Unique identifier for this reading
    pub id: Uuid,
    
    /// The actual measured value
    pub value: SensorValue,
    
    /// Timestamp when the reading was taken
    pub timestamp: DateTime<Utc>,
    
    /// Quality indicator for this reading
    pub quality: ReadingQuality,
    
    /// Additional context for this reading
    pub context: Option<ReadingContext>,
    
    /// Any alerts or warnings associated with this reading
    pub alerts: Vec<SensorAlert>,
}

/// Types of sensors supported by the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SensorType {
    /// Temperature sensor
    Temperature { unit: TemperatureUnit },
    
    /// Pressure sensor
    Pressure { unit: PressureUnit },
    
    /// Humidity sensor
    Humidity,
    
    /// Motion/acceleration sensor
    Motion { axes: u8 },
    
    /// Location/GPS sensor
    Location,
    
    /// Flow rate sensor
    Flow { medium: String, unit: FlowUnit },
    
    /// Energy/power sensor
    Energy { measurement_type: EnergyMeasurementType },
    
    /// Light/luminosity sensor
    Light { spectrum: LightSpectrum },
    
    /// Sound/acoustic sensor
    Sound { frequency_range: Option<FrequencyRange> },
    
    /// Chemical/gas sensor
    Chemical { target_substance: String },
    
    /// Biometric sensor
    Biometric { biometric_type: String },
    
    /// Custom sensor type
    Custom { 
        category: String,
        measurement_unit: String,
    },
}

/// Possible values from a sensor reading.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SensorValue {
    /// Single numeric value
    Numeric(f64),
    
    /// Boolean on/off state
    Boolean(bool),
    
    /// Text-based value
    String(String),
    
    /// Multi-dimensional numeric value (e.g., 3D coordinates)
    Vector(Vec<f64>),
    
    /// Complex structured data
    Json(serde_json::Value),
    
    /// Binary data (base64 encoded)
    Binary(String),
}

/// Physical or virtual location of a sensor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorLocation {
    /// Latitude for geographic location
    pub latitude: Option<f64>,
    
    /// Longitude for geographic location
    pub longitude: Option<f64>,
    
    /// Altitude in meters
    pub altitude: Option<f64>,
    
    /// Building or facility name
    pub building: Option<String>,
    
    /// Floor or level
    pub floor: Option<String>,
    
    /// Room or area identifier
    pub room: Option<String>,
    
    /// Specific position description
    pub position: Option<String>,
    
    /// Coordinate system used (if not geographic)
    pub coordinate_system: Option<String>,
}

/// Technical specifications of a sensor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorSpecifications {
    /// Measurement range (min, max)
    pub range: Option<(f64, f64)>,
    
    /// Measurement accuracy/precision
    pub accuracy: Option<f64>,
    
    /// Resolution of measurements
    pub resolution: Option<f64>,
    
    /// Sampling rate in Hz
    pub sampling_rate: Option<f64>,
    
    /// Response time in milliseconds
    pub response_time_ms: Option<u32>,
    
    /// Operating temperature range
    pub operating_temp_range: Option<(f64, f64)>,
    
    /// Power consumption in watts
    pub power_consumption: Option<f64>,
    
    /// Communication protocol
    pub protocol: Option<String>,
    
    /// Manufacturer information
    pub manufacturer: Option<ManufacturerInfo>,
}

/// Manufacturer information for a sensor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturerInfo {
    /// Company name
    pub company: String,
    
    /// Model number
    pub model: String,
    
    /// Serial number
    pub serial_number: Option<String>,
    
    /// Firmware version
    pub firmware_version: Option<String>,
    
    /// Manufacturing date
    pub manufacture_date: Option<DateTime<Utc>>,
}

/// Current operational status of a sensor.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SensorStatus {
    /// Sensor is operating normally
    Online,
    
    /// Sensor is offline or disconnected
    Offline,
    
    /// Sensor is generating warnings
    Warning,
    
    /// Sensor has failed
    Failed,
    
    /// Sensor is in maintenance mode
    Maintenance,
    
    /// Sensor is starting up
    Initializing,
    
    /// Unknown status
    Unknown,
}

/// Calibration information for a sensor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationInfo {
    /// Last calibration date
    pub last_calibrated: DateTime<Utc>,
    
    /// Next scheduled calibration
    pub next_calibration: Option<DateTime<Utc>>,
    
    /// Calibration method used
    pub method: String,
    
    /// Calibration coefficients or parameters
    pub parameters: HashMap<String, f64>,
    
    /// Who performed the calibration
    pub calibrated_by: Option<String>,
    
    /// Calibration certificate reference
    pub certificate_ref: Option<String>,
}

/// Quality assessment for a sensor reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingQuality {
    /// Overall quality score (0.0 to 1.0)
    pub score: f32,
    
    /// Specific quality indicators
    pub indicators: QualityIndicators,
    
    /// Any quality issues detected
    pub issues: Vec<QualityIssue>,
}

/// Specific quality indicators for readings.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QualityIndicators {
    /// Signal strength (0.0 to 1.0)
    pub signal_strength: Option<f32>,
    
    /// Data completeness (0.0 to 1.0)
    pub completeness: f32,
    
    /// Reading within expected range
    pub within_range: bool,
    
    /// Noise level assessment
    pub noise_level: NoiseLevel,
    
    /// Anomaly detection result
    pub anomaly_score: Option<f32>,
}

/// Noise level categories.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NoiseLevel {
    Low,
    Medium,
    High,
    Excessive,
}

/// Quality issues that may affect readings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// Type of issue
    pub issue_type: QualityIssueType,
    
    /// Severity of the issue
    pub severity: IssueSeverity,
    
    /// Description of the issue
    pub description: String,
    
    /// Suggested remediation
    pub remediation: Option<String>,
}

/// Types of quality issues.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QualityIssueType {
    OutOfRange,
    SignalLoss,
    Interference,
    Drift,
    Spike,
    MissingData,
    CalibrationNeeded,
}

/// Severity levels for issues.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Additional context for a sensor reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingContext {
    /// Environmental conditions during reading
    pub environment: Option<EnvironmentalContext>,
    
    /// Related readings from other sensors
    pub correlated_sensors: Vec<String>,
    
    /// Events occurring during this reading
    pub events: Vec<String>,
    
    /// Custom context data
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Environmental context during a reading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalContext {
    /// Ambient temperature
    pub temperature: Option<f64>,
    
    /// Humidity percentage
    pub humidity: Option<f64>,
    
    /// Atmospheric pressure
    pub pressure: Option<f64>,
    
    /// Vibration level
    pub vibration: Option<f64>,
}

/// Alert generated from sensor readings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorAlert {
    /// Alert identifier
    pub id: Uuid,
    
    /// Type of alert
    pub alert_type: AlertType,
    
    /// Alert severity
    pub severity: AlertSeverity,
    
    /// Alert message
    pub message: String,
    
    /// Threshold that triggered the alert
    pub threshold: Option<ThresholdInfo>,
    
    /// When the alert was triggered
    pub triggered_at: DateTime<Utc>,
    
    /// Whether the alert has been acknowledged
    pub acknowledged: bool,
}

/// Types of sensor alerts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertType {
    ThresholdExceeded,
    RateOfChange,
    DataQuality,
    SensorFailure,
    Predictive,
    Custom(String),
}

/// Alert severity levels.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Threshold information for alerts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdInfo {
    /// Threshold value
    pub value: f64,
    
    /// Type of threshold
    pub threshold_type: ThresholdType,
    
    /// Direction of threshold crossing
    pub direction: ThresholdDirection,
}

/// Types of thresholds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThresholdType {
    Absolute,
    Percentage,
    StandardDeviation,
    RateOfChange,
}

/// Direction of threshold crossing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThresholdDirection {
    Above,
    Below,
    Either,
}

/// Quality metrics for a collection of sensor data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataQualityMetrics {
    /// Overall data quality score (0.0 to 1.0)
    pub overall_score: f32,
    
    /// Percentage of valid readings
    pub validity_percentage: f32,
    
    /// Data completeness percentage
    pub completeness_percentage: f32,
    
    /// Number of missing readings
    pub missing_count: u32,
    
    /// Number of error readings
    pub error_count: u32,
    
    /// Average signal quality
    pub avg_signal_quality: Option<f32>,
}

/// Statistical analysis of sensor data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SensorStatistics {
    /// Total number of readings
    pub count: u64,
    
    /// Minimum value recorded
    pub min: Option<f64>,
    
    /// Maximum value recorded
    pub max: Option<f64>,
    
    /// Mean/average value
    pub mean: Option<f64>,
    
    /// Median value
    pub median: Option<f64>,
    
    /// Standard deviation
    pub std_dev: Option<f64>,
    
    /// Percentile values
    pub percentiles: HashMap<u8, f64>,
    
    /// Time-based statistics
    pub time_stats: TimeStatistics,
}

/// Time-based statistics for sensor data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeStatistics {
    /// First reading timestamp
    pub first_reading: Option<DateTime<Utc>>,
    
    /// Last reading timestamp
    pub last_reading: Option<DateTime<Utc>>,
    
    /// Average time between readings
    pub avg_interval_ms: Option<u64>,
    
    /// Longest gap between readings
    pub max_gap_ms: Option<u64>,
}

/// Configuration for data processing pipelines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    /// Filtering configuration
    pub filters: Vec<FilterConfig>,
    
    /// Aggregation settings
    pub aggregation: Option<AggregationConfig>,
    
    /// Anomaly detection settings
    pub anomaly_detection: Option<AnomalyDetectionConfig>,
    
    /// Data transformation rules
    pub transformations: Vec<TransformationRule>,
}

/// Filter configuration for sensor data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    /// Filter type
    pub filter_type: FilterType,
    
    /// Filter parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Whether the filter is enabled
    pub enabled: bool,
}

/// Types of filters available.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    MovingAverage,
    MedianFilter,
    KalmanFilter,
    Custom(String),
}

/// Aggregation configuration for data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationConfig {
    /// Time window for aggregation
    pub window_size: String,
    
    /// Aggregation method
    pub method: AggregationMethod,
    
    /// Whether to keep raw data
    pub preserve_raw: bool,
}

/// Aggregation methods.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AggregationMethod {
    Mean,
    Sum,
    Min,
    Max,
    Count,
    StandardDeviation,
}

/// Anomaly detection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionConfig {
    /// Algorithm to use
    pub algorithm: AnomalyAlgorithm,
    
    /// Sensitivity setting (0.0 to 1.0)
    pub sensitivity: f32,
    
    /// Training window size
    pub training_window: Option<String>,
    
    /// Algorithm-specific parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Anomaly detection algorithms.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnomalyAlgorithm {
    ZScore,
    IsolationForest,
    LSTM,
    StatisticalProcess,
    Custom(String),
}

/// Data transformation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationRule {
    /// Transformation type
    pub transform_type: TransformationType,
    
    /// Input field
    pub input_field: String,
    
    /// Output field
    pub output_field: String,
    
    /// Transformation parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Types of transformations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransformationType {
    Scale,
    Offset,
    Normalize,
    Derivative,
    Integral,
    Custom(String),
}

/// Metadata for sensor data collections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorDataMetadata {
    /// Data source identifier
    pub source_id: String,
    
    /// Collection method
    pub collection_method: String,
    
    /// Storage location or reference
    pub storage_ref: Option<String>,
    
    /// Retention period
    pub retention_days: Option<u32>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Custom metadata
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Temperature units.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Pressure units.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PressureUnit {
    Pascal,
    Bar,
    PSI,
    ATM,
}

/// Flow units.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FlowUnit {
    LitersPerMinute,
    GallonsPerMinute,
    CubicMetersPerHour,
}

/// Energy measurement types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnergyMeasurementType {
    Power,
    Voltage,
    Current,
    Energy,
    PowerFactor,
}

/// Light spectrum types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LightSpectrum {
    Visible,
    Infrared,
    Ultraviolet,
    FullSpectrum,
}

/// Frequency range for sound sensors.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FrequencyRange {
    /// Lower frequency limit in Hz
    pub min_hz: f64,
    
    /// Upper frequency limit in Hz
    pub max_hz: f64,
}

impl SensorData {
    /// Creates a new sensor data collection.
    pub fn new(twin_id: Uuid, sensor: SensorInfo) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            twin_id,
            sensor,
            readings: Vec::new(),
            quality_metrics: DataQualityMetrics::default(),
            statistics: SensorStatistics::default(),
            processing_config: ProcessingConfig::default(),
            metadata: SensorDataMetadata::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Adds a new reading to the collection.
    pub fn add_reading(&mut self, reading: SensorReading) {
        self.readings.push(reading);
        self.updated_at = Utc::now();
        self.update_statistics();
    }
    
    /// Updates statistics based on current readings.
    fn update_statistics(&mut self) {
        // This is a placeholder - actual implementation would calculate real statistics
        self.statistics.count = self.readings.len() as u64;
        
        if let Some(last_reading) = self.readings.last() {
            self.statistics.time_stats.last_reading = Some(last_reading.timestamp);
        }
        
        if let Some(first_reading) = self.readings.first() {
            self.statistics.time_stats.first_reading = Some(first_reading.timestamp);
        }
    }
    
    /// Gets readings within a time range.
    pub fn readings_in_range(
        &self, 
        start: DateTime<Utc>, 
        end: DateTime<Utc>
    ) -> Vec<&SensorReading> {
        self.readings
            .iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .collect()
    }
    
    /// Gets the latest reading if available.
    pub fn latest_reading(&self) -> Option<&SensorReading> {
        self.readings.last()
    }
}

impl SensorReading {
    /// Creates a new sensor reading with a numeric value.
    pub fn numeric(value: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            value: SensorValue::Numeric(value),
            timestamp: Utc::now(),
            quality: ReadingQuality::default(),
            context: None,
            alerts: Vec::new(),
        }
    }
    
    /// Checks if the reading has any alerts.
    pub fn has_alerts(&self) -> bool {
        !self.alerts.is_empty()
    }
    
    /// Gets the numeric value if available.
    pub fn as_numeric(&self) -> Option<f64> {
        match &self.value {
            SensorValue::Numeric(v) => Some(*v),
            _ => None,
        }
    }
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            filters: Vec::new(),
            aggregation: None,
            anomaly_detection: None,
            transformations: Vec::new(),
        }
    }
}

impl Default for SensorDataMetadata {
    fn default() -> Self {
        Self {
            source_id: String::new(),
            collection_method: "direct".to_string(),
            storage_ref: None,
            retention_days: Some(30),
            tags: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }
}

impl Default for ReadingQuality {
    fn default() -> Self {
        Self {
            score: 1.0,
            indicators: QualityIndicators {
                signal_strength: Some(1.0),
                completeness: 1.0,
                within_range: true,
                noise_level: NoiseLevel::Low,
                anomaly_score: None,
            },
            issues: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sensor_data_creation() {
        let sensor_info = SensorInfo {
            sensor_id: "TEMP001".to_string(),
            name: "Temperature Sensor 1".to_string(),
            sensor_type: SensorType::Temperature { 
                unit: TemperatureUnit::Celsius 
            },
            location: None,
            specifications: SensorSpecifications { 
                range: Some((-50.0, 150.0)),
                accuracy: Some(0.1),
                resolution: Some(0.01),
                sampling_rate: Some(1.0),
                response_time_ms: Some(100),
                operating_temp_range: None,
                power_consumption: None,
                protocol: Some("MQTT".to_string()),
                manufacturer: None,
            },
            status: SensorStatus::Online,
            calibration: None,
        };
        
        let twin_id = Uuid::new_v4();
        let sensor_data = SensorData::new(twin_id, sensor_info);
        
        assert_eq!(sensor_data.twin_id, twin_id);
        assert_eq!(sensor_data.sensor.name, "Temperature Sensor 1");
        assert!(sensor_data.readings.is_empty());
    }
    
    #[test]
    fn test_add_reading() {
        let sensor_info = SensorInfo {
            sensor_id: "TEST001".to_string(),
            name: "Test Sensor".to_string(),
            sensor_type: SensorType::Custom {
                category: "test".to_string(),
                measurement_unit: "units".to_string(),
            },
            location: None,
            specifications: SensorSpecifications {
                range: None,
                accuracy: None,
                resolution: None,
                sampling_rate: None,
                response_time_ms: None,
                operating_temp_range: None,
                power_consumption: None,
                protocol: None,
                manufacturer: None,
            },
            status: SensorStatus::Online,
            calibration: None,
        };
        
        let mut sensor_data = SensorData::new(Uuid::new_v4(), sensor_info);
        let reading = SensorReading::numeric(42.5);
        
        sensor_data.add_reading(reading);
        
        assert_eq!(sensor_data.readings.len(), 1);
        assert_eq!(sensor_data.statistics.count, 1);
        assert!(sensor_data.statistics.time_stats.last_reading.is_some());
    }
    
    #[test]
    fn test_sensor_value_numeric() {
        let reading = SensorReading::numeric(25.5);
        
        assert_eq!(reading.as_numeric(), Some(25.5));
        assert!(!reading.has_alerts());
        
        match reading.value {
            SensorValue::Numeric(v) => assert_eq!(v, 25.5),
            _ => panic!("Expected numeric value"),
        }
    }
}