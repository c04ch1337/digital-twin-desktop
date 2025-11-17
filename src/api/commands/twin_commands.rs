//! Tauri commands for digital twin management
//!
//! This module provides Tauri commands for creating, configuring,
//! and interacting with digital twins.

use tauri::State;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::sync::Arc;

use crate::core::application::services::TwinService;
use crate::core::domain::models::{
    DigitalTwin, TwinType, DataSource, DataSourceType, 
    ConnectionConfig, SyncConfiguration, SyncMode, RetentionPolicy
};
use crate::api::dto::{
    ApiResponse, TwinSummary, CreateTwinRequest
};
use crate::api::error::{ApiResult, map_result};

/// Create a new digital twin
#[tauri::command]
pub async fn create_digital_twin(
    request: CreateTwinRequest,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<TwinSummary> {
    // Parse twin type
    let twin_type = match request.twin_type.as_str() {
        "industrial" => TwinType::Industrial,
        "infrastructure" => TwinType::Infrastructure,
        "process" => TwinType::Process,
        custom => TwinType::Custom(custom.to_string()),
    };
    
    // Create the digital twin
    let result = twin_service.create_twin(
        request.name,
        twin_type,
        request.configuration.unwrap_or_else(|| serde_json::json!({})),
    ).await;
    
    let twin = map_result(result)?;
    
    // Convert to DTO
    Ok(crate::api::dto::converters::twin_to_summary(&twin))
}

/// Get digital twin by ID
#[tauri::command]
pub async fn get_digital_twin(
    twin_id: String,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = twin_service.get_twin(id).await;
    let twin = map_result(result)?;
    
    // Convert to detailed JSON response
    let data_sources = twin.data_sources.iter()
        .map(|ds| {
            serde_json::json!({
                "id": ds.id,
                "name": ds.name,
                "type": format!("{:?}", ds.source_type),
                "connection": {
                    "url": ds.connection.url,
                    "credentials": ds.connection.credentials.is_some(),
                    "parameters": ds.connection.parameters,
                },
                "enabled": ds.enabled,
            })
        })
        .collect::<Vec<_>>();
    
    let response = serde_json::json!({
        "id": twin.id,
        "name": twin.metadata.name,
        "description": twin.metadata.description,
        "twin_type": format!("{:?}", twin.twin_type),
        "created_at": twin.metadata.created_at,
        "updated_at": twin.metadata.updated_at,
        "state": format!("{:?}", twin.state),
        "configuration": twin.configuration,
        "data_sources": data_sources,
        "properties": twin.properties,
        "sync_configuration": {
            "mode": format!("{:?}", twin.sync_configuration.mode),
            "interval_seconds": twin.sync_configuration.interval_seconds,
            "enabled": twin.sync_configuration.enabled,
        },
    });
    
    Ok(response)
}

/// List all digital twins
#[tauri::command]
pub async fn list_digital_twins(
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<Vec<TwinSummary>> {
    let result = twin_service.list_twins().await;
    let twins = map_result(result)?;
    
    // Convert to DTOs
    let summaries = twins.iter()
        .map(|twin| crate::api::dto::converters::twin_to_summary(twin))
        .collect();
    
    Ok(summaries)
}

/// Update digital twin configuration
#[tauri::command]
pub async fn update_digital_twin(
    twin_id: String,
    name: Option<String>,
    description: Option<String>,
    configuration: Option<Value>,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<TwinSummary> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = twin_service.update_twin(id, name, description, configuration).await;
    let twin = map_result(result)?;
    
    // Convert to DTO
    Ok(crate::api::dto::converters::twin_to_summary(&twin))
}

/// Delete a digital twin
#[tauri::command]
pub async fn delete_digital_twin(
    twin_id: String,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = twin_service.delete_twin(id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Add data source to digital twin
#[tauri::command]
pub async fn add_data_source(
    twin_id: String,
    name: String,
    source_type: String,
    connection_url: String,
    parameters: Value,
    credentials: Option<Value>,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Parse source type
    let source_type = match source_type.as_str() {
        "mqtt" => DataSourceType::MQTT,
        "modbus" => DataSourceType::Modbus,
        "opc_ua" => DataSourceType::OpcUa,
        "rest" => DataSourceType::Rest,
        "database" => DataSourceType::Database,
        "file" => DataSourceType::File,
        custom => DataSourceType::Custom(custom.to_string()),
    };
    
    // Create connection config
    let connection = ConnectionConfig {
        url: connection_url,
        credentials,
        parameters,
    };
    
    // Add the data source
    let result = twin_service.add_data_source(
        id, name, source_type, connection
    ).await;
    
    let data_source = map_result(result)?;
    
    // Convert to JSON response
    let response = serde_json::json!({
        "id": data_source.id,
        "name": data_source.name,
        "type": format!("{:?}", data_source.source_type),
        "connection": {
            "url": data_source.connection.url,
            "credentials": data_source.connection.credentials.is_some(),
            "parameters": data_source.connection.parameters,
        },
        "enabled": data_source.enabled,
    });
    
    Ok(response)
}

/// Remove data source from digital twin
#[tauri::command]
pub async fn remove_data_source(
    twin_id: String,
    data_source_id: String,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<bool> {
    let twin_id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let data_source_id = Uuid::parse_str(&data_source_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = twin_service.remove_data_source(twin_id, data_source_id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Configure twin synchronization
#[tauri::command]
pub async fn configure_twin_sync(
    twin_id: String,
    mode: String,
    interval_seconds: Option<u64>,
    enabled: bool,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Parse sync mode
    let sync_mode = match mode.as_str() {
        "real_time" => SyncMode::RealTime,
        "interval" => SyncMode::Interval,
        "manual" => SyncMode::Manual,
        "event_driven" => SyncMode::EventDriven,
        _ => return Err(crate::api::error::ApiError {
            code: "INVALID_SYNC_MODE".to_string(),
            message: "Invalid sync mode".to_string(),
            details: Some(format!("Supported modes: real_time, interval, manual, event_driven. Received: {}", mode)),
        }),
    };
    
    // Create sync configuration
    let sync_config = SyncConfiguration {
        mode: sync_mode,
        interval_seconds: interval_seconds.unwrap_or(60),
        enabled,
        retention_policy: RetentionPolicy::KeepAll,
    };
    
    // Update the sync configuration
    let result = twin_service.configure_sync(id, sync_config).await;
    map_result(result)?;
    
    Ok(true)
}

/// Manually sync digital twin with data sources
#[tauri::command]
pub async fn sync_digital_twin(
    twin_id: String,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = twin_service.sync_twin(id).await;
    let sync_result = map_result(result)?;
    
    // Convert to JSON response
    let response = serde_json::json!({
        "success": sync_result.success,
        "timestamp": sync_result.timestamp,
        "data_points_synced": sync_result.data_points_synced,
        "errors": sync_result.errors,
        "duration_ms": sync_result.duration_ms,
    });
    
    Ok(response)
}

/// Get twin properties
#[tauri::command]
pub async fn get_twin_properties(
    twin_id: String,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = twin_service.get_twin_properties(id).await;
    let properties = map_result(result)?;
    
    Ok(properties.into())
}

/// Update twin property
#[tauri::command]
pub async fn update_twin_property(
    twin_id: String,
    property_path: String,
    value: Value,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = twin_service.update_property(id, property_path, value).await;
    map_result(result)?;
    
    Ok(true)
}

/// Export digital twin model
#[tauri::command]
pub async fn export_twin_model(
    twin_id: String,
    format: String,
    twin_service: State<'_, Arc<TwinService>>,
) -> ApiResult<String> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = match format.to_lowercase().as_str() {
        "json" => twin_service.export_twin_model_json(id).await,
        "xml" => twin_service.export_twin_model_xml(id).await,
        _ => return Err(crate::api::error::ApiError {
            code: "INVALID_FORMAT".to_string(),
            message: "Unsupported export format".to_string(),
            details: Some(format!("Supported formats: json, xml. Received: {}", format)),
        }),
    };
    
    map_result(result)
}