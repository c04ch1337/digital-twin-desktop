//! Tauri commands for conversation management
//!
//! This module provides Tauri commands for creating, retrieving,
//! and interacting with conversations.

use tauri::State;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::sync::Arc;

use crate::core::application::services::ConversationService;
use crate::core::domain::models::{Message, MessageSender, ContentType};
use crate::api::dto::{
    ApiResponse, ConversationSummary, CreateConversationRequest,
    MessageDto, SendMessageRequest
};
use crate::api::error::{ApiResult, map_result};

/// Create a new conversation
#[tauri::command]
pub async fn create_conversation(
    request: CreateConversationRequest,
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<ConversationSummary> {
    let result = conversation_service.create_conversation(
        request.title,
        request.description,
        request.agent_id,
    ).await;
    
    let conversation = map_result(result)?;
    
    // If initial message is provided, send it
    if let Some(initial_message) = request.initial_message {
        conversation_service.send_message(
            conversation.id,
            MessageSender::User,
            initial_message,
            ContentType::Text,
            vec![],
        ).await?;
    }
    
    // Convert to DTO
    Ok(ConversationSummary {
        id: conversation.id,
        title: conversation.metadata.title,
        last_activity: conversation.metadata.updated_at,
        message_count: conversation.messages.len(),
        state: format!("{:?}", conversation.state),
    })
}

/// Get conversation by ID
#[tauri::command]
pub async fn get_conversation(
    conversation_id: String,
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<Value> {
    let id = Uuid::parse_str(&conversation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = conversation_service.get_conversation(id).await;
    let conversation = map_result(result)?;
    
    // Convert to DTO with messages
    let messages: Vec<MessageDto> = conversation.messages.iter()
        .map(|msg| crate::api::dto::converters::message_to_dto(msg))
        .collect();
    
    let response = serde_json::json!({
        "id": conversation.id,
        "title": conversation.metadata.title,
        "description": conversation.metadata.description,
        "created_at": conversation.metadata.created_at,
        "updated_at": conversation.metadata.updated_at,
        "state": format!("{:?}", conversation.state),
        "messages": messages,
        "agent_id": conversation.agent_id,
    });
    
    Ok(response)
}

/// List all conversations
#[tauri::command]
pub async fn list_conversations(
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<Vec<ConversationSummary>> {
    let result = conversation_service.list_conversations().await;
    let conversations = map_result(result)?;
    
    // Convert to DTOs
    let summaries = conversations.iter()
        .map(|conv| ConversationSummary {
            id: conv.id,
            title: conv.metadata.title.clone(),
            last_activity: conv.metadata.updated_at,
            message_count: conv.messages.len(),
            state: format!("{:?}", conv.state),
        })
        .collect();
    
    Ok(summaries)
}

/// Send a message to a conversation
#[tauri::command]
pub async fn send_message(
    request: SendMessageRequest,
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<MessageDto> {
    // Convert attachments if present
    let attachments = request.attachments
        .unwrap_or_default()
        .iter()
        .map(|att| crate::core::domain::models::Attachment {
            name: att.name.clone(),
            content_type: ContentType::from_mime_type(&att.mime_type),
            size: att.size,
            url: att.url.clone(),
        })
        .collect();
    
    // Send the message
    let result = conversation_service.send_message(
        request.conversation_id,
        MessageSender::User,
        request.content,
        ContentType::Text,
        attachments,
    ).await;
    
    let message = map_result(result)?;
    
    // Convert to DTO
    Ok(crate::api::dto::converters::message_to_dto(&message))
}

/// Stream messages from a conversation
///
/// This command supports streaming responses for real-time updates
#[tauri::command]
pub async fn stream_conversation_messages(
    conversation_id: String,
    window: tauri::Window,
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<()> {
    let id = Uuid::parse_str(&conversation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    // Set up message streaming
    let mut receiver = conversation_service.subscribe_to_messages(id).await?;
    
    // Spawn a task to handle the streaming
    tauri::async_runtime::spawn(async move {
        while let Some(message) = receiver.recv().await {
            // Convert to DTO
            let message_dto = crate::api::dto::converters::message_to_dto(&message);
            
            // Emit event to the frontend
            let _ = window.emit("conversation:new_message", message_dto);
        }
    });
    
    Ok(())
}

/// Delete a conversation
#[tauri::command]
pub async fn delete_conversation(
    conversation_id: String,
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<bool> {
    let id = Uuid::parse_str(&conversation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = conversation_service.delete_conversation(id).await;
    map_result(result)?;
    
    Ok(true)
}

/// Update conversation metadata
#[tauri::command]
pub async fn update_conversation(
    conversation_id: String,
    title: Option<String>,
    description: Option<String>,
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<ConversationSummary> {
    let id = Uuid::parse_str(&conversation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = conversation_service.update_conversation(id, title, description).await;
    let conversation = map_result(result)?;
    
    // Convert to DTO
    Ok(ConversationSummary {
        id: conversation.id,
        title: conversation.metadata.title,
        last_activity: conversation.metadata.updated_at,
        message_count: conversation.messages.len(),
        state: format!("{:?}", conversation.state),
    })
}

/// Export conversation to file
#[tauri::command]
pub async fn export_conversation(
    conversation_id: String,
    format: String,
    conversation_service: State<'_, Arc<ConversationService>>,
) -> ApiResult<String> {
    let id = Uuid::parse_str(&conversation_id)
        .map_err(|e| crate::api::error::to_api_error(e))?;
    
    let result = match format.to_lowercase().as_str() {
        "json" => conversation_service.export_conversation_json(id).await,
        "markdown" | "md" => conversation_service.export_conversation_markdown(id).await,
        "txt" | "text" => conversation_service.export_conversation_text(id).await,
        _ => return Err(crate::api::error::ApiError {
            code: "INVALID_FORMAT".to_string(),
            message: "Unsupported export format".to_string(),
            details: Some(format!("Supported formats: json, markdown, txt. Received: {}", format)),
        }),
    };
    
    map_result(result)
}