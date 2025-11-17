mod api;
mod core;
mod infrastructure;

use std::sync::Arc;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize tracing
            tracing_subscriber::fmt::init();
            
            // Initialize database
            let app_handle = app.handle();
            let app_dir = app_handle.path().app_data_dir()
                .expect("Failed to get app data directory");
            
            // Ensure directory exists
            std::fs::create_dir_all(&app_dir)
                .expect("Failed to create app data directory");
            
            // Load configuration
            let config = infrastructure::config::AppConfig::load()
                .expect("Failed to load configuration");
            
            // Initialize security components
            infrastructure::security::init()
                .expect("Failed to initialize security module");
            
            // Initialize middleware
            let (auth_middleware, rate_limit_middleware, validation_middleware) =
                api::middleware::init(&config.security);
            
            // Initialize services
            let conversation_service = Arc::new(core::application::services::ConversationService::new());
            let agent_service = Arc::new(core::application::services::AgentService::new());
            let twin_service = Arc::new(core::application::services::TwinService::new());
            let simulation_service = Arc::new(core::application::services::SimulationService::new());
            let tool_service = Arc::new(core::application::services::ToolService::new());
            
            // Register services and middleware as state
            app.manage(conversation_service);
            app.manage(agent_service);
            app.manage(twin_service);
            app.manage(simulation_service);
            app.manage(tool_service);
            app.manage(auth_middleware);
            app.manage(rate_limit_middleware);
            app.manage(validation_middleware);
            app.manage(config);
            
            tracing::info!("Digital Twin Desktop started");
            
            Ok(())
        })
        .plugin(tauri::plugin::shell::init())
        .invoke_handler(tauri::generate_handler![
            // Authentication middleware commands
            api::middleware::authenticate,
            api::middleware::require_permission,
            api::middleware::validate_input,
            api::middleware::rate_limit,
            
            // Legacy commands (for backward compatibility)
            api::commands::create_digital_twin,
            api::commands::list_digital_twins,
            api::commands::start_simulation,
            api::commands::generate_with_ai,
            
            // Conversation commands
            api::commands::conversation_commands::create_conversation,
            api::commands::conversation_commands::get_conversation,
            api::commands::conversation_commands::list_conversations,
            api::commands::conversation_commands::send_message,
            api::commands::conversation_commands::stream_conversation_messages,
            api::commands::conversation_commands::delete_conversation,
            api::commands::conversation_commands::update_conversation,
            api::commands::conversation_commands::export_conversation,
            
            // Agent commands
            api::commands::agent_commands::create_agent,
            api::commands::agent_commands::get_agent,
            api::commands::agent_commands::list_agents,
            api::commands::agent_commands::update_agent_configuration,
            api::commands::agent_commands::set_agent_capability,
            api::commands::agent_commands::delete_agent,
            api::commands::agent_commands::assign_agent_to_conversation,
            api::commands::agent_commands::get_agent_metrics,
            
            // Twin commands
            api::commands::twin_commands::create_digital_twin,
            api::commands::twin_commands::get_digital_twin,
            api::commands::twin_commands::list_digital_twins,
            api::commands::twin_commands::update_digital_twin,
            api::commands::twin_commands::delete_digital_twin,
            api::commands::twin_commands::add_data_source,
            api::commands::twin_commands::remove_data_source,
            api::commands::twin_commands::configure_twin_sync,
            api::commands::twin_commands::sync_digital_twin,
            api::commands::twin_commands::get_twin_properties,
            api::commands::twin_commands::update_twin_property,
            api::commands::twin_commands::export_twin_model,
            
            // Simulation commands
            api::commands::simulation_commands::create_simulation,
            api::commands::simulation_commands::get_simulation_status,
            api::commands::simulation_commands::list_simulations,
            api::commands::simulation_commands::stop_simulation,
            api::commands::simulation_commands::pause_simulation,
            api::commands::simulation_commands::resume_simulation,
            api::commands::simulation_commands::get_simulation_results,
            api::commands::simulation_commands::stream_simulation_updates,
            api::commands::simulation_commands::run_batch_simulation,
            api::commands::simulation_commands::get_batch_simulation_results,
            api::commands::simulation_commands::create_simulation_scenario,
            api::commands::simulation_commands::list_simulation_scenarios,
            api::commands::simulation_commands::run_simulation_scenario,
            api::commands::simulation_commands::compare_simulations,
            
            // Tool commands
            api::commands::tool_commands::register_tool,
            api::commands::tool_commands::get_tool,
            api::commands::tool_commands::list_tools,
            api::commands::tool_commands::execute_tool,
            api::commands::tool_commands::stream_tool_execution,
            api::commands::tool_commands::cancel_tool_execution,
            api::commands::tool_commands::get_execution_status,
            api::commands::tool_commands::validate_tool_parameters,
            api::commands::tool_commands::update_tool_configuration,
            api::commands::tool_commands::delete_tool,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}