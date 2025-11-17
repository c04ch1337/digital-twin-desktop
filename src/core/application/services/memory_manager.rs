//! Memory Manager Service for context windowing and token counting.
//!
//! This service manages conversation memory with support for:
//! - Context windowing with token counting
//! - Sliding window memory strategy
//! - Token estimation for different models

use crate::core::domain::models::conversation::{Message, MessageSender};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for memory management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum tokens allowed in context window
    pub max_context_tokens: u32,
    /// Maximum number of messages to retain
    pub max_messages: usize,
    /// Strategy for memory management
    pub strategy: MemoryStrategy,
    /// Token estimation model
    pub token_model: TokenModel,
}

/// Memory management strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryStrategy {
    /// Keep most recent messages up to token limit
    SlidingWindow,
    /// Summarize older messages to save tokens
    Summarization,
    /// Retrieve relevant messages based on similarity
    RAG,
    /// Hybrid approach combining multiple strategies
    Hybrid,
}

/// Token estimation models
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenModel {
    /// OpenAI GPT models (cl100k_base encoding)
    GPT,
    /// Claude models
    Claude,
    /// Generic estimation (4 chars per token)
    Generic,
}

/// Represents a message with token metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub id: Uuid,
    pub sender: String,
    pub content: String,
    pub token_count: u32,
    pub created_at: DateTime<Utc>,
    pub model: Option<String>,
}

/// Context window result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWindow {
    pub messages: Vec<ContextMessage>,
    pub total_tokens: u32,
    pub window_start_index: usize,
    pub window_end_index: usize,
}

/// Memory manager for handling conversation context
pub struct MemoryManager {
    config: MemoryConfig,
}

impl MemoryManager {
    /// Creates a new memory manager with the given configuration
    pub fn new(config: MemoryConfig) -> Self {
        Self { config }
    }

    /// Estimates token count for a given text
    pub fn estimate_tokens(&self, text: &str) -> u32 {
        match self.config.token_model {
            TokenModel::GPT => self.estimate_gpt_tokens(text),
            TokenModel::Claude => self.estimate_claude_tokens(text),
            TokenModel::Generic => self.estimate_generic_tokens(text),
        }
    }

    /// Estimates tokens using GPT tokenization (cl100k_base)
    fn estimate_gpt_tokens(&self, text: &str) -> u32 {
        // Rough estimation: ~4 characters per token for GPT models
        // More accurate would require actual tokenizer
        let char_count = text.len() as f32;
        ((char_count / 4.0).ceil()) as u32
    }

    /// Estimates tokens using Claude tokenization
    fn estimate_claude_tokens(&self, text: &str) -> u32 {
        // Claude uses similar tokenization to GPT
        // Rough estimation: ~3.5 characters per token
        let char_count = text.len() as f32;
        ((char_count / 3.5).ceil()) as u32
    }

    /// Generic token estimation
    fn estimate_generic_tokens(&self, text: &str) -> u32 {
        // Generic: ~4 characters per token
        let char_count = text.len() as f32;
        ((char_count / 4.0).ceil()) as u32
    }

    /// Builds a context window from messages using sliding window strategy
    pub fn build_context_window(&self, messages: &[Message]) -> ContextWindow {
        match self.config.strategy {
            MemoryStrategy::SlidingWindow => self.sliding_window_strategy(messages),
            MemoryStrategy::Summarization => self.summarization_strategy(messages),
            MemoryStrategy::RAG => self.rag_strategy(messages),
            MemoryStrategy::Hybrid => self.hybrid_strategy(messages),
        }
    }

    /// Sliding window strategy: keep most recent messages within token limit
    fn sliding_window_strategy(&self, messages: &[Message]) -> ContextWindow {
        let mut context_messages = Vec::new();
        let mut total_tokens = 0u32;
        let mut window_start_index = 0;

        // Process messages in reverse order (most recent first)
        for (idx, message) in messages.iter().enumerate().rev() {
            let token_count = self.estimate_tokens(&message.content);

            // Check if adding this message would exceed token limit
            if total_tokens + token_count > self.config.max_context_tokens
                && !context_messages.is_empty()
            {
                window_start_index = idx + 1;
                break;
            }

            // Check message count limit
            if context_messages.len() >= self.config.max_messages {
                window_start_index = idx + 1;
                break;
            }

            let sender = match &message.sender {
                MessageSender::User { user_id } => format!("user:{}", user_id),
                MessageSender::Agent { agent_id } => format!("agent:{}", agent_id),
                MessageSender::System => "system".to_string(),
            };

            context_messages.push(ContextMessage {
                id: message.id,
                sender,
                content: message.content.clone(),
                token_count,
                created_at: message.created_at,
                model: message.metadata.model.clone(),
            });

            total_tokens += token_count;
        }

        // Reverse to get chronological order
        context_messages.reverse();
        let window_end_index = messages.len();

        ContextWindow {
            messages: context_messages,
            total_tokens,
            window_start_index,
            window_end_index,
        }
    }

    /// Summarization strategy: summarize older messages to save tokens
    fn summarization_strategy(&self, messages: &[Message]) -> ContextWindow {
        // For now, fall back to sliding window
        // In production, this would call a summarization service
        self.sliding_window_strategy(messages)
    }

    /// RAG strategy: retrieve relevant messages based on similarity
    fn rag_strategy(&self, messages: &[Message]) -> ContextWindow {
        // For now, fall back to sliding window
        // In production, this would use semantic search
        self.sliding_window_strategy(messages)
    }

    /// Hybrid strategy: combine multiple approaches
    fn hybrid_strategy(&self, messages: &[Message]) -> ContextWindow {
        // For now, fall back to sliding window
        // In production, this would intelligently combine strategies
        self.sliding_window_strategy(messages)
    }

    /// Calculates token statistics for a set of messages
    pub fn calculate_token_stats(&self, messages: &[Message]) -> TokenStats {
        let mut total_tokens = 0u32;
        let mut token_counts = HashMap::new();
        let mut model_counts = HashMap::new();

        for message in messages {
            let token_count = self.estimate_tokens(&message.content);
            total_tokens += token_count;

            let sender = match &message.sender {
                MessageSender::User { .. } => "user",
                MessageSender::Agent { .. } => "agent",
                MessageSender::System => "system",
            };

            *token_counts.entry(sender.to_string()).or_insert(0u32) += token_count;

            if let Some(model) = &message.metadata.model {
                *model_counts.entry(model.clone()).or_insert(0u32) += token_count;
            }
        }

        TokenStats {
            total_tokens,
            message_count: messages.len(),
            tokens_by_sender: token_counts,
            tokens_by_model: model_counts,
            avg_tokens_per_message: if messages.is_empty() {
                0u32
            } else {
                total_tokens / messages.len() as u32
            },
        }
    }

    /// Checks if adding a message would exceed token limit
    pub fn would_exceed_limit(&self, current_tokens: u32, new_message_tokens: u32) -> bool {
        current_tokens + new_message_tokens > self.config.max_context_tokens
    }

    /// Gets the remaining token capacity
    pub fn remaining_capacity(&self, current_tokens: u32) -> u32 {
        if current_tokens >= self.config.max_context_tokens {
            0
        } else {
            self.config.max_context_tokens - current_tokens
        }
    }
}

/// Token statistics for a set of messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStats {
    pub total_tokens: u32,
    pub message_count: usize,
    pub tokens_by_sender: HashMap<String, u32>,
    pub tokens_by_model: HashMap<String, u32>,
    pub avg_tokens_per_message: u32,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_context_tokens: 4096,
            max_messages: 50,
            strategy: MemoryStrategy::SlidingWindow,
            token_model: TokenModel::GPT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_estimation() {
        let config = MemoryConfig::default();
        let manager = MemoryManager::new(config);

        let text = "Hello, this is a test message.";
        let tokens = manager.estimate_tokens(text);
        assert!(tokens > 0);
    }

    #[test]
    fn test_gpt_token_estimation() {
        let config = MemoryConfig {
            token_model: TokenModel::GPT,
            ..Default::default()
        };
        let manager = MemoryManager::new(config);

        let text = "a".repeat(100);
        let tokens = manager.estimate_gpt_tokens(&text);
        assert_eq!(tokens, 25); // 100 / 4 = 25
    }

    #[test]
    fn test_remaining_capacity() {
        let config = MemoryConfig {
            max_context_tokens: 1000,
            ..Default::default()
        };
        let manager = MemoryManager::new(config);

        assert_eq!(manager.remaining_capacity(500), 500);
        assert_eq!(manager.remaining_capacity(1000), 0);
        assert_eq!(manager.remaining_capacity(1500), 0);
    }

    #[test]
    fn test_would_exceed_limit() {
        let config = MemoryConfig {
            max_context_tokens: 1000,
            ..Default::default()
        };
        let manager = MemoryManager::new(config);

        assert!(!manager.would_exceed_limit(500, 400));
        assert!(manager.would_exceed_limit(500, 600));
        assert!(manager.would_exceed_limit(1000, 1));
    }
}
