//! Tauri commands for simulation control
//!
//! This module provides Tauri commands for creating, running,
//! and analyzing simulations of digital twins.

use tauri::{State, Window};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::sync::Arc;

use crate::core::application::services::{
    SimulationService, SimulationConfig, ScenarioConfig,
    BatchSimulationRequest, BatchSimulationResult
};
use crate::core::domain::models::{Simulation, SimulationStatus};
use crate::api::dto::{
    ApiResponse, CreateSimulationRequest, SimulationStatusDto
};
use crate::api::error::{ApiResult, map_result};

/// Create and start a new simulation
#[tauri::command]
pub async fn create_simulation(
    request: CreateSimulationRequest,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<SimulationStatusDto> {
    // Create simulation configuration
    let config = SimulationConfig {
        name: request.name,
        parameters: request.parameters.clone(),
        duration_seconds: request.duration_seconds.unwrap_or(3600),
        twin_id: request.twin_id,
    };
    
    // Create and start the simulation
    let result = simulation_service.create_simulation(config).await;
    let simulation = map_result(result)?;
    
    // Convert to DTO
    Ok(SimulationStatusDto {
        id: simulation.id,
        twin_id: simulation.twin_id,
        name: simulation.name,
        status: format!("{:?}", simulation.status),
        progress: simulation.progress,
        started_at: simulation.started_at,
        completed_at: simulation.completed_at,
    })
}

/// Get simulation status by ID
#[tauri::command]
pub async fn get_simulation_status(
    simulation_id: String,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<SimulationStatusDto> {
    let id = Uuid::parse_str(&simulation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = simulation_service.get_simulation_status(id).await;
    let simulation = map_result(result)?;
    
    // Convert to DTO
    Ok(SimulationStatusDto {
        id: simulation.id,
        twin_id: simulation.twin_id,
        name: simulation.name,
        status: format!("{:?}", simulation.status),
        progress: simulation.progress,
        started_at: simulation.started_at,
        completed_at: simulation.completed_at,
    })
}

/// List all simulations for a digital twin
#[tauri::command]
pub async fn list_simulations(
    twin_id: Option<String>,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<Vec<SimulationStatusDto>> {
    // Parse twin ID if provided
    let twin_uuid = match twin_id {
        Some(id) => Some(Uuid::parse_str(&id)
            .map_err(|e| crate::api::error::to_api_error(e))?),
        None => None,
    };
    
    // Get simulations
    let result = match twin_uuid {
        Some(id) => simulation_service.list_simulations_for_twin(id).await,
        None => simulation_service.list_all_simulations().await,
    };
    
    let simulations = map_result(result)?;
    
    // Convert to DTOs
    let dtos = simulations.iter()
        .map(|sim| SimulationStatusDto {
            id: sim.id,
            twin_id: sim.twin_id,
            name: sim.name.clone(),
            status: format!("{:?}", sim.status),
            progress: sim.progress,
            started_at: sim.started_at,
            completed_at: sim.completed_at,
        })
        .collect();
    
    Ok(dtos)
}

/// Stop a running simulation
#[tauri::command]
pub async fn stop_simulation(
    simulation_id: String,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&simulation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = simulation_service.stop_simulation(id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Pause a running simulation
#[tauri::command]
pub async fn pause_simulation(
    simulation_id: String,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&simulation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = simulation_service.pause_simulation(id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Resume a paused simulation
#[tauri::command]
pub async fn resume_simulation(
    simulation_id: String,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&simulation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = simulation_service.resume_simulation(id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Get simulation results
#[tauri::command]
pub async fn get_simulation_results(
    simulation_id: String,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&simulation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = simulation_service.get_simulation_results(id).await;
    let results = map_result(result)?;
    
    Ok(results)
}

/// Stream simulation updates
///
/// This command supports streaming responses for real-time updates
#[tauri::command]
pub async fn stream_simulation_updates(
    simulation_id: String,
    window: Window,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<()> {
    let id = Uuid::parse_str(&simulation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Set up simulation update streaming
    let mut receiver = simulation_service.subscribe_to_updates(id).await?;
    
    // Spawn a task to handle the streaming
    tauri::async_runtime::spawn(async move {
        while let Some(update) = receiver.recv().await {
            // Convert to DTO
            let update_dto = SimulationStatusDto {
                id: update.id,
                twin_id: update.twin_id,
                name: update.name,
                status: format!("{:?}", update.status),
                progress: update.progress,
                started_at: update.started_at,
                completed_at: update.completed_at,
            };
            
            // Emit event to the frontend
            let _ = window.emit("simulation:status_update", update_dto);
            
            // If simulation is complete, also send results
            if update.status == SimulationStatus::Completed {
                if let Ok(results) = simulation_service.get_simulation_results(update.id).await {
                    let _ = window.emit("simulation:results", results);
                }
            }
        }
    });
    
    Ok(())
}

/// Run a batch of simulations with different parameters
#[tauri::command]
pub async fn run_batch_simulation(
    twin_id: String,
    base_parameters: Value,
    variations: Vec<Value>,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<String> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Create batch simulation request
    let request = BatchSimulationRequest {
        twin_id: id,
        base_parameters,
        variations,
        parallel: true,
    };
    
    // Run batch simulation
    let result = simulation_service.run_batch_simulation(request).await;
    let batch_id = map_result(result)?;
    
    Ok(batch_id.to_string())
}

/// Get batch simulation results
#[tauri::command]
pub async fn get_batch_simulation_results(
    batch_id: String,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&batch_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = simulation_service.get_batch_results(id).await;
    let batch_results = map_result(result)?;
    
    // Convert to JSON response
    let simulations = batch_results.simulations.iter()
        .map(|sim| {
            serde_json::json!({
                "id": sim.id,
                "parameters": sim.parameters,
                "status": format!("{:?}", sim.status),
                "progress": sim.progress,
                "results": sim.results,
            })
        })
        .collect::<Vec<_>>();
    
    let response = serde_json::json!({
        "batch_id": batch_results.batch_id,
        "twin_id": batch_results.twin_id,
        "total_simulations": batch_results.total_simulations,
        "completed_simulations": batch_results.completed_simulations,
        "started_at": batch_results.started_at,
        "completed_at": batch_results.completed_at,
        "simulations": simulations,
    });
    
    Ok(response)
}

/// Create a simulation scenario
#[tauri::command]
pub async fn create_simulation_scenario(
    twin_id: String,
    name: String,
    description: String,
    parameters: Value,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&twin_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Create scenario configuration
    let config = ScenarioConfig {
        name,
        description,
        parameters,
        twin_id: id,
    };
    
    // Create the scenario
    let result = simulation_service.create_scenario(config).await;
    let scenario = map_result(result)?;
    
    // Convert to JSON response
    let response = serde_json::json!({
        "id": scenario.id,
        "name": scenario.name,
        "description": scenario.description,
        "twin_id": scenario.twin_id,
        "created_at": scenario.created_at,
    });
    
    Ok(response)
}

/// List simulation scenarios
#[tauri::command]
pub async fn list_simulation_scenarios(
    twin_id: Option<String>,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<Vec<Value>> {
    // Parse twin ID if provided
    let twin_uuid = match twin_id {
        Some(id) => Some(Uuid::parse_str(&id)
            .map_err(|e| crate::api::error::to_api_error(e))?),
        None => None,
    };
    
    // Get scenarios
    let result = match twin_uuid {
        Some(id) => simulation_service.list_scenarios_for_twin(id).await,
        None => simulation_service.list_all_scenarios().await,
    };
    
    let scenarios = map_result(result)?;
    
    // Convert to JSON responses
    let responses = scenarios.iter()
        .map(|scenario| {
            serde_json::json!({
                "id": scenario.id,
                "name": scenario.name,
                "description": scenario.description,
                "twin_id": scenario.twin_id,
                "created_at": scenario.created_at,
            })
        })
        .collect();
    
    Ok(responses)
}

/// Run a simulation from a saved scenario
#[tauri::command]
pub async fn run_simulation_scenario(
    scenario_id: String,
    override_parameters: Option<Value>,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<SimulationStatusDto> {
    let id = Uuid::parse_str(&scenario_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Run the scenario
    let result = simulation_service.run_scenario(id, override_parameters).await;
    let simulation = map_result(result)?;
    
    // Convert to DTO
    Ok(SimulationStatusDto {
        id: simulation.id,
        twin_id: simulation.twin_id,
        name: simulation.name,
        status: format!("{:?}", simulation.status),
        progress: simulation.progress,
        started_at: simulation.started_at,
        completed_at: simulation.completed_at,
    })
}

/// Compare simulation results
#[tauri::command]
pub async fn compare_simulations(
    simulation_ids: Vec<String>,
    metrics: Vec<String>,
    simulation_service: State<'_, Arc<SimulationService>>,
) -> ApiResult<Value> {
    // Parse simulation IDs
    let ids = simulation_ids.iter()
        .map(|id| Uuid::parse_str(id)
            .map_err(|e| crate::api::error::to_api_error(e)))
        .collect::<Result<Vec<_>, _>>()?;
    
    // Compare simulations
    let result = simulation_service.compare_simulations(ids, metrics).await;
    let comparison = map_result(result)?;
    
    Ok(comparison)
}