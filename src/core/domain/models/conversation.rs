//! Conversation and Message domain models for the Digital Twin Desktop.
//! 
//! This module defines the core conversation entities that represent
//! interactions between users and digital twin agents.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a conversation between a user and one or more digital twin agents.
///
/// A conversation maintains the full context and history of interactions,
/// allowing for coherent multi-turn dialogues with digital twins.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    /// Unique identifier for the conversation
    pub id: Uuid,
    
    /// Title or subject of the conversation
    pub title: String,
    
    /// Optional description providing more context about the conversation
    pub description: Option<String>,
    
    /// List of digital twin agent IDs participating in this conversation
    pub participant_agent_ids: Vec<Uuid>,
    
    /// All messages in this conversation, ordered chronologically
    pub messages: Vec<Message>,
    
    /// Current state of the conversation
    pub state: ConversationState,
    
    /// Metadata for additional conversation properties
    pub metadata: ConversationMetadata,
    
    /// Timestamp when the conversation was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the conversation was last updated
    pub updated_at: DateTime<Utc>,
}

/// Represents a single message within a conversation.
///
/// Messages can be from users, digital twin agents, or system notifications.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    /// Unique identifier for the message
    pub id: Uuid,
    
    /// The conversation this message belongs to
    pub conversation_id: Uuid,
    
    /// The sender of this message
    pub sender: MessageSender,
    
    /// The actual content of the message
    pub content: String,
    
    /// Type of message content
    pub content_type: ContentType,
    
    /// Optional attachments or references
    pub attachments: Vec<Attachment>,
    
    /// Metadata specific to this message
    pub metadata: MessageMetadata,
    
    /// Timestamp when the message was created
    pub created_at: DateTime<Utc>,
}

/// Identifies who sent a message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageSender {
    /// Message from the human user
    User { user_id: String },
    
    /// Message from a digital twin agent
    Agent { agent_id: Uuid },
    
    /// System-generated message
    System,
}

/// Represents the current state of a conversation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationState {
    /// Conversation is active and can receive new messages
    Active,
    
    /// Conversation is temporarily paused
    Paused,
    
    /// Conversation has been archived but can be reopened
    Archived,
    
    /// Conversation has been closed and cannot receive new messages
    Closed,
}

/// Describes the type of content in a message.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContentType {
    /// Plain text message
    Text,
    
    /// Markdown-formatted text
    Markdown,
    
    /// Code snippet with optional language hint
    Code,
    
    /// JSON data
    Json,
    
    /// Tool execution result
    ToolResult,
    
    /// Error message
    Error,
}

/// Represents an attachment or external reference in a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachment {
    /// Unique identifier for the attachment
    pub id: Uuid,
    
    /// Type of attachment
    pub attachment_type: AttachmentType,
    
    /// Name or title of the attachment
    pub name: String,
    
    /// Size in bytes (if applicable)
    pub size: Option<u64>,
    
    /// MIME type
    pub mime_type: Option<String>,
    
    /// URL or path to the attachment
    pub url: Option<String>,
}

/// Describes the type of attachment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttachmentType {
    /// File attachment
    File,
    
    /// Image attachment
    Image,
    
    /// Link to external resource
    Link,
    
    /// Reference to another conversation
    ConversationReference { conversation_id: Uuid },
    
    /// Reference to sensor data
    SensorDataReference { sensor_data_id: Uuid },
}

/// Metadata associated with a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationMetadata {
    /// Tags for categorizing conversations
    pub tags: Vec<String>,
    
    /// Priority level for the conversation
    pub priority: ConversationPriority,
    
    /// Language of the conversation
    pub language: String,
    
    /// Custom key-value pairs for extensibility
    pub custom_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// Priority levels for conversations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConversationPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Metadata associated with a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageMetadata {
    /// Whether the message has been edited
    pub edited: bool,
    
    /// Timestamp of last edit (if any)
    pub edited_at: Option<DateTime<Utc>>,
    
    /// Tokens consumed by this message (for LLM interactions)
    pub token_count: Option<u32>,
    
    /// Model used to generate this message (for agent messages)
    pub model: Option<String>,
    
    /// Confidence score for agent-generated messages
    pub confidence: Option<f32>,
    
    /// Custom metadata fields
    pub custom_fields: std::collections::HashMap<String, serde_json::Value>,
}

impl Conversation {
    /// Creates a new conversation with the given title.
    pub fn new(title: String, participant_agent_ids: Vec<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            participant_agent_ids,
            messages: Vec::new(),
            state: ConversationState::Active,
            metadata: ConversationMetadata::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Adds a new message to the conversation.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }
    
    /// Gets the most recent message in the conversation.
    pub fn last_message(&self) -> Option<&Message> {
        self.messages.last()
    }
    
    /// Counts messages by sender type.
    pub fn message_count_by_sender(&self) -> (usize, usize, usize) {
        let mut user_count = 0;
        let mut agent_count = 0;
        let mut system_count = 0;
        
        for message in &self.messages {
            match &message.sender {
                MessageSender::User { .. } => user_count += 1,
                MessageSender::Agent { .. } => agent_count += 1,
                MessageSender::System => system_count += 1,
            }
        }
        
        (user_count, agent_count, system_count)
    }
    
    /// Checks if the conversation can accept new messages.
    pub fn is_active(&self) -> bool {
        matches!(self.state, ConversationState::Active)
    }
}

impl Message {
    /// Creates a new message from a user.
    pub fn from_user(conversation_id: Uuid, user_id: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            conversation_id,
            sender: MessageSender::User { user_id },
            content,
            content_type: ContentType::Text,
            attachments: Vec::new(),
            metadata: MessageMetadata::default(),
            created_at: Utc::now(),
        }
    }
    
    /// Creates a new message from an agent.
    pub fn from_agent(
        conversation_id: Uuid, 
        agent_id: Uuid, 
        content: String,
        model: Option<String>,
    ) -> Self {
        let mut metadata = MessageMetadata::default();
        metadata.model = model;
        
        Self {
            id: Uuid::new_v4(),
            conversation_id,
            sender: MessageSender::Agent { agent_id },
            content,
            content_type: ContentType::Text,
            attachments: Vec::new(),
            metadata,
            created_at: Utc::now(),
        }
    }
    
    /// Creates a system message.
    pub fn system(conversation_id: Uuid, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            conversation_id,
            sender: MessageSender::System,
            content,
            content_type: ContentType::Text,
            attachments: Vec::new(),
            metadata: MessageMetadata::default(),
            created_at: Utc::now(),
        }
    }
}

impl Default for ConversationMetadata {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            priority: ConversationPriority::Normal,
            language: "en".to_string(),
            custom_fields: std::collections::HashMap::new(),
        }
    }
}

impl Default for MessageMetadata {
    fn default() -> Self {
        Self {
            edited: false,
            edited_at: None,
            token_count: None,
            model: None,
            confidence: None,
            custom_fields: std::collections::HashMap::new(),
        }
    }
}

impl Default for ConversationPriority {
    fn default() -> Self {
        Self::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversation_creation() {
        let agent_id = Uuid::new_v4();
        let conversation = Conversation::new(
            "Test Conversation".to_string(),
            vec![agent_id],
        );
        
        assert_eq!(conversation.title, "Test Conversation");
        assert_eq!(conversation.participant_agent_ids.len(), 1);
        assert_eq!(conversation.state, ConversationState::Active);
        assert!(conversation.messages.is_empty());
    }
    
    #[test]
    fn test_message_creation() {
        let conv_id = Uuid::new_v4();
        let user_msg = Message::from_user(
            conv_id,
            "user123".to_string(),
            "Hello, world!".to_string(),
        );
        
        assert_eq!(user_msg.conversation_id, conv_id);
        assert!(matches!(user_msg.sender, MessageSender::User { .. }));
        assert_eq!(user_msg.content, "Hello, world!");
        assert_eq!(user_msg.content_type, ContentType::Text);
    }
    
    #[test]
    fn test_add_message_to_conversation() {
        let agent_id = Uuid::new_v4();
        let mut conversation = Conversation::new(
            "Test".to_string(),
            vec![agent_id],
        );
        
        let message = Message::from_user(
            conversation.id,
            "user123".to_string(),
            "Test message".to_string(),
        );
        
        let original_updated = conversation.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        conversation.add_message(message);
        
        assert_eq!(conversation.messages.len(), 1);
        assert!(conversation.updated_at > original_updated);
    }
}