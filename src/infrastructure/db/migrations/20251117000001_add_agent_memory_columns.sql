-- Add agent memory and prompt management columns

-- Add columns to agents table for prompt management
ALTER TABLE agents ADD COLUMN system_prompt TEXT;
ALTER TABLE agents ADD COLUMN instructions TEXT;
ALTER TABLE agents ADD COLUMN prompt_version TEXT DEFAULT '1.0.0';

-- Add agent_id to conversations table for direct agent relationship
ALTER TABLE conversations ADD COLUMN agent_id TEXT;
ALTER TABLE conversations ADD FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE SET NULL;

-- Add metadata columns to messages table for token counting and model tracking
ALTER TABLE messages ADD COLUMN agent_id TEXT;
ALTER TABLE messages ADD COLUMN token_count INTEGER;
ALTER TABLE messages ADD COLUMN model TEXT;
ALTER TABLE messages ADD COLUMN content_type TEXT DEFAULT 'text';

-- Create prompts table for versioning and storage
CREATE TABLE IF NOT EXISTS prompts (
    id TEXT PRIMARY KEY NOT NULL,
    agent_id TEXT NOT NULL,
    version TEXT NOT NULL,
    system_prompt TEXT NOT NULL,
    instructions TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT, -- JSON
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE CASCADE,
    UNIQUE(agent_id, version)
);

-- Create message_context table for memory management
CREATE TABLE IF NOT EXISTS message_context (
    id TEXT PRIMARY KEY NOT NULL,
    conversation_id TEXT NOT NULL,
    message_id TEXT NOT NULL,
    context_window_start INTEGER,
    context_window_end INTEGER,
    token_count INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE,
    FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
);

-- Create indexes for performance
CREATE INDEX idx_conversations_agent_id ON conversations(agent_id);
CREATE INDEX idx_messages_agent_id ON messages(agent_id);
CREATE INDEX idx_messages_token_count ON messages(token_count);
CREATE INDEX idx_messages_model ON messages(model);
CREATE INDEX idx_prompts_agent_id ON prompts(agent_id);
CREATE INDEX idx_prompts_version ON prompts(version);
CREATE INDEX idx_message_context_conversation_id ON message_context(conversation_id);
CREATE INDEX idx_message_context_message_id ON message_context(message_id);
CREATE INDEX idx_message_context_token_count ON message_context(token_count);
