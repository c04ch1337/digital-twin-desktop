-- Initial database schema for Digital Twin Desktop

-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

-- Conversations table
CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT -- JSON
);

-- Messages table
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY NOT NULL,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT, -- JSON
    FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
);

-- Agents table
CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL,
    capabilities TEXT NOT NULL, -- JSON array
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT -- JSON
);

-- Digital Twins table
CREATE TABLE IF NOT EXISTS digital_twins (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    twin_type TEXT NOT NULL,
    state TEXT NOT NULL,
    properties TEXT NOT NULL, -- JSON
    agent_id TEXT,
    last_sync DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT, -- JSON
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE SET NULL
);

-- Sensor Data table
CREATE TABLE IF NOT EXISTS sensor_data (
    id TEXT PRIMARY KEY NOT NULL,
    twin_id TEXT NOT NULL,
    sensor_type TEXT NOT NULL,
    unit TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT, -- JSON
    FOREIGN KEY (twin_id) REFERENCES digital_twins(id) ON DELETE CASCADE
);

-- Sensor Readings table
CREATE TABLE IF NOT EXISTS sensor_readings (
    id TEXT PRIMARY KEY NOT NULL,
    sensor_data_id TEXT NOT NULL,
    value REAL NOT NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT, -- JSON
    FOREIGN KEY (sensor_data_id) REFERENCES sensor_data(id) ON DELETE CASCADE
);

-- Tools table
CREATE TABLE IF NOT EXISTS tools (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    tool_type TEXT NOT NULL,
    config TEXT NOT NULL, -- JSON
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT -- JSON
);

-- Tool Executions table
CREATE TABLE IF NOT EXISTS tool_executions (
    id TEXT PRIMARY KEY NOT NULL,
    tool_id TEXT NOT NULL,
    status TEXT NOT NULL,
    parameters TEXT NOT NULL, -- JSON
    result TEXT, -- JSON
    started_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    error TEXT,
    metrics TEXT, -- JSON
    FOREIGN KEY (tool_id) REFERENCES tools(id) ON DELETE CASCADE
);

-- Indexes
CREATE INDEX idx_conversations_state ON conversations(state);
CREATE INDEX idx_messages_conversation_id ON messages(conversation_id);
CREATE INDEX idx_agents_state ON agents(state);
CREATE INDEX idx_digital_twins_agent_id ON digital_twins(agent_id);
CREATE INDEX idx_digital_twins_state ON digital_twins(state);
CREATE INDEX idx_sensor_data_twin_id ON sensor_data(twin_id);
CREATE INDEX idx_sensor_readings_sensor_data_id ON sensor_readings(sensor_data_id);
CREATE INDEX idx_sensor_readings_timestamp ON sensor_readings(timestamp);
CREATE INDEX idx_tools_type ON tools(tool_type);
CREATE INDEX idx_tool_executions_tool_id ON tool_executions(tool_id);
CREATE INDEX idx_tool_executions_status ON tool_executions(status);