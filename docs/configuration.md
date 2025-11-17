# Digital Twin Desktop - Configuration Guide

This document provides comprehensive information about configuring the Digital Twin Desktop application through environment variables.

## Table of Contents

1. [Overview](#overview)
2. [Environment Files](#environment-files)
3. [LLM Provider Configuration](#llm-provider-configuration)
4. [Configuration Sections](#configuration-sections)
5. [Agent Configuration](#agent-configuration)
6. [Environment-Specific Examples](#environment-specific-examples)
7. [Security Best Practices](#security-best-practices)
8. [Troubleshooting](#troubleshooting)

## Overview

The Digital Twin Desktop application uses environment variables for configuration management. Configuration is loaded in the following order:

1. Default values defined in code
2. Environment-specific configuration files (`.env.development`, `.env.production`, `.env.test`)
3. Environment variables (with `APP_` prefix)

This hierarchical approach allows for flexible configuration across different deployment environments.

## Environment Files

### `.env.local.example`

Template file showing all available configuration options. Copy this file to create environment-specific configurations:

```bash
cp .env.local.example .env.development
cp .env.local.example .env.production
cp .env.local.example .env.test
```

### `.env.development`

Development environment configuration with:
- Relaxed security settings for easier testing
- Verbose logging (debug level)
- Higher rate limits
- Disabled sandbox for tool execution
- In-memory or local file-based database

### `.env.production`

Production environment configuration with:
- Strict security settings
- Minimal logging (warn level)
- Lower rate limits
- Enabled sandbox for tool execution
- Remote database paths
- Secure credential references

### `.env.test`

Test environment configuration with:
- Minimal logging (error level)
- In-memory database for fast test execution
- Disabled long-term memory
- Mock API keys
- Relaxed rate limits for testing

## LLM Provider Configuration

The Digital Twin Desktop supports multiple LLM providers for flexibility and cost optimization. You can configure and switch between different providers based on your needs.

### Supported Providers

1. **OpenAI** - GPT-4, GPT-3.5-turbo models
2. **Anthropic** - Claude models
3. **OpenRouter** - Aggregated access to multiple models
4. **Google Gemini** - Google's Gemini models
5. **HuggingFace** - Open-source models via Inference API
6. **Ollama** - Local LLM execution
7. **LMStudio** - Local LLM execution with GUI

### Provider Selection

Set the default provider using:

```env
LLM_DEFAULT_PROVIDER=openai
```

Available values: `openai`, `anthropic`, `openrouter`, `gemini`, `huggingface`, `ollama`, `lmstudio`

### OpenAI Configuration

```env
LLM_OPENAI_API_KEY=your_openai_api_key_here
LLM_OPENAI_DEFAULT_MODEL=gpt-4
LLM_OPENAI_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LLM_OPENAI_API_KEY` | String | Required | OpenAI API key |
| `LLM_OPENAI_DEFAULT_MODEL` | String | `gpt-4` | Default model to use |
| `LLM_OPENAI_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |

### Anthropic Configuration

```env
LLM_ANTHROPIC_API_KEY=your_anthropic_api_key_here
LLM_ANTHROPIC_DEFAULT_MODEL=claude-2
LLM_ANTHROPIC_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LLM_ANTHROPIC_API_KEY` | String | Required | Anthropic API key |
| `LLM_ANTHROPIC_DEFAULT_MODEL` | String | `claude-2` | Default model to use |
| `LLM_ANTHROPIC_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |

### OpenRouter Configuration

```env
LLM_OPENROUTER_API_KEY=your_openrouter_api_key_here
LLM_OPENROUTER_DEFAULT_MODEL=openrouter/auto
LLM_OPENROUTER_SITE_URL=https://your-site.com
LLM_OPENROUTER_SITE_NAME=Your Site Name
LLM_OPENROUTER_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LLM_OPENROUTER_API_KEY` | String | Required | OpenRouter API key |
| `LLM_OPENROUTER_DEFAULT_MODEL` | String | `openrouter/auto` | Default model to use |
| `LLM_OPENROUTER_SITE_URL` | String | Optional | Your site URL for OpenRouter |
| `LLM_OPENROUTER_SITE_NAME` | String | Optional | Your site name for OpenRouter |
| `LLM_OPENROUTER_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |

### Google Gemini Configuration

```env
LLM_GEMINI_API_KEY=your_gemini_api_key_here
LLM_GEMINI_DEFAULT_MODEL=gemini-pro
LLM_GEMINI_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LLM_GEMINI_API_KEY` | String | Required | Google Gemini API key |
| `LLM_GEMINI_DEFAULT_MODEL` | String | `gemini-pro` | Default model to use |
| `LLM_GEMINI_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |

### HuggingFace Configuration

```env
LLM_HUGGINGFACE_API_KEY=your_huggingface_api_key_here
LLM_HUGGINGFACE_DEFAULT_MODEL=gpt2
LLM_HUGGINGFACE_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LLM_HUGGINGFACE_API_KEY` | String | Required | HuggingFace API key |
| `LLM_HUGGINGFACE_DEFAULT_MODEL` | String | `gpt2` | Default model to use |
| `LLM_HUGGINGFACE_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |

### Ollama Configuration (Local)

```env
LLM_OLLAMA_BASE_URL=http://localhost:11434
LLM_OLLAMA_DEFAULT_MODEL=llama2
LLM_OLLAMA_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LLM_OLLAMA_BASE_URL` | String | `http://localhost:11434` | Ollama server URL |
| `LLM_OLLAMA_DEFAULT_MODEL` | String | `llama2` | Default model to use |
| `LLM_OLLAMA_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |

**Setup Instructions**:
1. Download Ollama from https://ollama.ai
2. Run `ollama serve` to start the server
3. Pull a model: `ollama pull llama2`
4. Configure the base URL and model name

### LMStudio Configuration (Local)

```env
LLM_LMSTUDIO_BASE_URL=http://localhost:1234
LLM_LMSTUDIO_DEFAULT_MODEL=local-model
LLM_LMSTUDIO_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LLM_LMSTUDIO_BASE_URL` | String | `http://localhost:1234` | LMStudio server URL |
| `LLM_LMSTUDIO_DEFAULT_MODEL` | String | `local-model` | Default model to use |
| `LLM_LMSTUDIO_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |

**Setup Instructions**:
1. Download LMStudio from https://lmstudio.ai
2. Launch LMStudio and load a model
3. Start the local server (default port 1234)
4. Configure the base URL and model name

## Configuration Sections

### API Keys for LLM Services

```env
OPENAI_API_KEY=your_openai_api_key_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here
OPENROUTER_API_KEY=your_openrouter_api_key_here
GEMINI_API_KEY=your_gemini_api_key_here
HUGGINGFACE_API_KEY=your_huggingface_api_key_here
```

**Description**: API keys for various LLM services. See [LLM Provider Configuration](#llm-provider-configuration) for detailed setup instructions.

**Security**: Store these securely using environment variables in production. Never commit real keys to version control.

### Database Configuration

```env
DB_PATH=./data/digital_twin.db
DB_MAX_CONNECTIONS=5
DB_FOREIGN_KEYS=true
DB_WAL_MODE=true
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `DB_PATH` | String | `./data/digital_twin.db` | Path to SQLite database file |
| `DB_MAX_CONNECTIONS` | Integer | `5` | Maximum database connection pool size |
| `DB_FOREIGN_KEYS` | Boolean | `true` | Enable foreign key constraints |
| `DB_WAL_MODE` | Boolean | `true` | Enable Write-Ahead Logging for better concurrency |

### Logging Configuration

```env
LOG_LEVEL=info
LOG_FILE_PATH=./logs/digital_twin.log
LOG_JSON_FORMAT=false
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LOG_LEVEL` | String | `info` | Log level: `debug`, `info`, `warn`, `error` |
| `LOG_FILE_PATH` | String | Optional | Path to log file (optional) |
| `LOG_JSON_FORMAT` | Boolean | `false` | Enable JSON formatted logging |

### Security Configuration

```env
ENCRYPTION_KEY=your_32_character_encryption_key
AUTH_SECRET=your_auth_secret_here
TOKEN_EXPIRATION=3600
CORS_ORIGINS=http://localhost:5173,http://localhost:1420
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `ENCRYPTION_KEY` | String | Required | 32-character encryption key for data encryption |
| `AUTH_SECRET` | String | Required | Secret key for JWT token generation |
| `TOKEN_EXPIRATION` | Integer | `3600` | JWT token expiration time in seconds |
| `CORS_ORIGINS` | String | `*` | Comma-separated list of allowed CORS origins |

## Agent Configuration

The agent configuration controls the behavior of AI agents in the system.

### Basic Agent Settings

```env
AGENT_SYSTEM_PROMPT_FILE=./config/system_prompt.txt
AGENT_DEFAULT_MODEL=gpt-4
AGENT_TEMPERATURE=0.7
AGENT_MAX_TOKENS=2048
AGENT_TOP_P=0.9
AGENT_FREQUENCY_PENALTY=0.0
AGENT_PRESENCE_PENALTY=0.0
```

| Variable | Type | Default | Range | Description |
|----------|------|---------|-------|-------------|
| `AGENT_SYSTEM_PROMPT_FILE` | String | `./config/system_prompt.txt` | - | Path to system prompt file |
| `AGENT_DEFAULT_MODEL` | String | `gpt-4` | - | Default LLM model for agents |
| `AGENT_TEMPERATURE` | Float | `0.7` | 0.0-2.0 | Controls randomness of responses |
| `AGENT_MAX_TOKENS` | Integer | `2048` | 1-4096 | Maximum tokens per response |
| `AGENT_TOP_P` | Float | `0.9` | 0.0-1.0 | Nucleus sampling parameter |
| `AGENT_FREQUENCY_PENALTY` | Float | `0.0` | -2.0-2.0 | Penalizes repeated tokens |
| `AGENT_PRESENCE_PENALTY` | Float | `0.0` | -2.0-2.0 | Penalizes new tokens |

### Agent Memory Configuration

```env
AGENT_MEMORY_ENABLED=true
AGENT_MEMORY_MAX_MESSAGES=100
AGENT_MEMORY_RETENTION_DAYS=30
AGENT_MEMORY_COMPRESSION_ENABLED=true
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `AGENT_MEMORY_ENABLED` | Boolean | `true` | Enable short-term conversation memory |
| `AGENT_MEMORY_MAX_MESSAGES` | Integer | `100` | Maximum messages to keep in memory |
| `AGENT_MEMORY_RETENTION_DAYS` | Integer | `30` | Days to retain messages before cleanup |
| `AGENT_MEMORY_COMPRESSION_ENABLED` | Boolean | `true` | Enable memory compression for efficiency |

### Agent Long-Term Memory Configuration

```env
AGENT_LONG_TERM_MEMORY_ENABLED=true
AGENT_LONG_TERM_MEMORY_EMBEDDING_MODEL=text-embedding-3-small
AGENT_LONG_TERM_MEMORY_SIMILARITY_THRESHOLD=0.7
AGENT_LONG_TERM_MEMORY_MAX_ENTRIES=10000
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `AGENT_LONG_TERM_MEMORY_ENABLED` | Boolean | `true` | Enable long-term memory with embeddings |
| `AGENT_LONG_TERM_MEMORY_EMBEDDING_MODEL` | String | `text-embedding-3-small` | Embedding model for similarity search |
| `AGENT_LONG_TERM_MEMORY_SIMILARITY_THRESHOLD` | Float | `0.7` | Minimum similarity score for retrieval |
| `AGENT_LONG_TERM_MEMORY_MAX_ENTRIES` | Integer | `10000` | Maximum entries in long-term memory |

### Agent Rate Limiting

```env
AGENT_RATE_LIMIT_REQUESTS=100
AGENT_RATE_LIMIT_WINDOW_SECONDS=60
AGENT_RATE_LIMIT_BURST_SIZE=10
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `AGENT_RATE_LIMIT_REQUESTS` | Integer | `100` | Number of requests allowed per window |
| `AGENT_RATE_LIMIT_WINDOW_SECONDS` | Integer | `60` | Time window in seconds |
| `AGENT_RATE_LIMIT_BURST_SIZE` | Integer | `10` | Maximum burst size for rate limiting |

### MQTT Configuration

```env
MQTT_BROKER_URL=mqtt://localhost
MQTT_BROKER_PORT=1883
MQTT_CLIENT_ID=digital-twin
MQTT_USERNAME=your_mqtt_username
MQTT_PASSWORD=your_mqtt_password
MQTT_TIMEOUT_SECONDS=30
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `MQTT_BROKER_URL` | String | `mqtt://localhost` | MQTT broker URL |
| `MQTT_BROKER_PORT` | Integer | `1883` | MQTT broker port |
| `MQTT_CLIENT_ID` | String | `digital-twin` | MQTT client identifier |
| `MQTT_USERNAME` | String | Optional | MQTT authentication username |
| `MQTT_PASSWORD` | String | Optional | MQTT authentication password |
| `MQTT_TIMEOUT_SECONDS` | Integer | `30` | Connection timeout in seconds |

### Modbus Configuration

```env
MODBUS_HOST=localhost
MODBUS_PORT=502
MODBUS_TIMEOUT_SECONDS=5
MODBUS_MAX_RETRIES=3
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `MODBUS_HOST` | String | `localhost` | Modbus server host |
| `MODBUS_PORT` | Integer | `502` | Modbus server port |
| `MODBUS_TIMEOUT_SECONDS` | Integer | `5` | Request timeout in seconds |
| `MODBUS_MAX_RETRIES` | Integer | `3` | Maximum retry attempts |

### Web Tool Configuration

```env
WEB_TOOL_MAX_RESPONSE_SIZE=1048576
WEB_TOOL_TIMEOUT_SECONDS=30
WEB_TOOL_ALLOWED_DOMAINS=example.com,api.example.com
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `WEB_TOOL_MAX_RESPONSE_SIZE` | Integer | `1048576` | Maximum response size in bytes |
| `WEB_TOOL_TIMEOUT_SECONDS` | Integer | `30` | Request timeout in seconds |
| `WEB_TOOL_ALLOWED_DOMAINS` | String | - | Comma-separated list of allowed domains |

### File Tool Configuration

```env
FILE_TOOL_BASE_PATH=./data/files
FILE_TOOL_MAX_FILE_SIZE=10485760
FILE_TOOL_ALLOWED_EXTENSIONS=txt,json,csv,xml,pdf
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `FILE_TOOL_BASE_PATH` | String | `./data/files` | Base directory for file operations |
| `FILE_TOOL_MAX_FILE_SIZE` | Integer | `10485760` | Maximum file size in bytes |
| `FILE_TOOL_ALLOWED_EXTENSIONS` | String | - | Comma-separated list of allowed extensions |

### Sandbox Configuration

```env
SANDBOX_ENABLED=true
SANDBOX_MAX_EXECUTION_TIME=30
SANDBOX_MAX_MEMORY_MB=256
SANDBOX_ALLOW_NETWORK=false
SANDBOX_ALLOW_FILESYSTEM=false
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `SANDBOX_ENABLED` | Boolean | `true` | Enable sandbox for tool execution |
| `SANDBOX_MAX_EXECUTION_TIME` | Integer | `30` | Maximum execution time in seconds |
| `SANDBOX_MAX_MEMORY_MB` | Integer | `256` | Maximum memory usage in MB |
| `SANDBOX_ALLOW_NETWORK` | Boolean | `false` | Allow network access in sandbox |
| `SANDBOX_ALLOW_FILESYSTEM` | Boolean | `false` | Allow filesystem access in sandbox |

### API Key Configuration

```env
API_KEY_AUTH_ENABLED=true
API_KEY_EXPIRATION=2592000
API_KEY_MAX_PER_USER=5
```

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `API_KEY_AUTH_ENABLED` | Boolean | `true` | Enable API key authentication |
| `API_KEY_EXPIRATION` | Integer | `2592000` | API key expiration time in seconds (30 days) |
| `API_KEY_MAX_PER_USER` | Integer | `5` | Maximum API keys per user |

## Environment-Specific Examples

### Development Environment

For local development with relaxed constraints:

```env
APP_ENV=development
LOG_LEVEL=debug
AGENT_TEMPERATURE=0.8
AGENT_MEMORY_MAX_MESSAGES=200
SANDBOX_ENABLED=false
SANDBOX_ALLOW_NETWORK=true
SANDBOX_ALLOW_FILESYSTEM=true
API_KEY_AUTH_ENABLED=false
```

### Production Environment

For production deployment with strict security:

```env
APP_ENV=production
LOG_LEVEL=warn
LOG_JSON_FORMAT=true
AGENT_TEMPERATURE=0.5
AGENT_MEMORY_MAX_MESSAGES=50
AGENT_RATE_LIMIT_REQUESTS=50
SANDBOX_ENABLED=true
SANDBOX_ALLOW_NETWORK=false
SANDBOX_ALLOW_FILESYSTEM=false
API_KEY_AUTH_ENABLED=true
```

### Testing Environment

For automated testing:

```env
APP_ENV=test
LOG_LEVEL=error
DB_PATH=:memory:
AGENT_TEMPERATURE=0.0
AGENT_LONG_TERM_MEMORY_ENABLED=false
AGENT_RATE_LIMIT_REQUESTS=10000
SANDBOX_ENABLED=false
API_KEY_AUTH_ENABLED=false
```

## Security Best Practices

### 1. Credential Management

- **Never commit credentials** to version control
- Use environment variables for sensitive data in production
- Rotate API keys regularly
- Use different keys for different environments

### 2. Encryption

- Use strong, randomly generated encryption keys
- Store encryption keys securely (e.g., AWS Secrets Manager, HashiCorp Vault)
- Rotate encryption keys periodically

### 3. CORS Configuration

- Restrict CORS origins to known domains in production
- Avoid using wildcard (`*`) in production
- Use HTTPS for all origins

### 4. Rate Limiting

- Set appropriate rate limits based on expected usage
- Use lower limits in production
- Monitor rate limit violations

### 5. Sandbox Configuration

- Enable sandbox in production for tool execution
- Disable network and filesystem access unless necessary
- Set reasonable execution time and memory limits

### 6. Logging

- Use appropriate log levels (warn/error in production)
- Enable JSON formatting for log aggregation
- Store logs securely and rotate them regularly

## Troubleshooting

### Configuration Not Loading

**Problem**: Configuration values are not being loaded from environment variables.

**Solution**:
1. Verify environment variables are set correctly
2. Check that variable names use the correct prefix (`APP_`)
3. Ensure environment variables are exported (not just set)
4. Check application logs for configuration loading errors

### Agent Not Responding

**Problem**: Agent is not responding to requests.

**Solution**:
1. Verify `AGENT_SYSTEM_PROMPT_FILE` path is correct
2. Check that the system prompt file exists and is readable
3. Verify LLM API keys are set correctly
4. Check rate limiting configuration
5. Review application logs for errors

### Database Connection Issues

**Problem**: Cannot connect to database.

**Solution**:
1. Verify `DB_PATH` is correct and writable
2. Check database file permissions
3. Ensure `DB_MAX_CONNECTIONS` is appropriate
4. Review database logs for errors

### Memory Issues

**Problem**: Application running out of memory.

**Solution**:
1. Reduce `AGENT_MEMORY_MAX_MESSAGES`
2. Reduce `AGENT_LONG_TERM_MEMORY_MAX_ENTRIES`
3. Enable `AGENT_MEMORY_COMPRESSION_ENABLED`
4. Increase available system memory

### Rate Limiting Issues

**Problem**: Requests being rate limited unexpectedly.

**Solution**:
1. Increase `AGENT_RATE_LIMIT_REQUESTS`
2. Increase `AGENT_RATE_LIMIT_WINDOW_SECONDS`
3. Increase `AGENT_RATE_LIMIT_BURST_SIZE`
4. Check for concurrent requests exceeding limits

## Loading Configuration at Runtime

The application loads configuration during startup:

```rust
// Load configuration from files and environment
let config = infrastructure::config::AppConfig::load()
    .expect("Failed to load configuration");

// Access agent configuration
let agent_config = config.agent();
println!("Agent model: {}", agent_config.default_model);
println!("Temperature: {}", agent_config.temperature);
```

Configuration is loaded in this order:
1. Default values from code
2. Environment-specific files (`.env.development`, `.env.production`, `.env.test`)
3. Environment variables with `APP_` prefix

Later sources override earlier ones, allowing for flexible configuration management.

## System Prompt File

The system prompt file (`./config/system_prompt.txt`) defines the default behavior and instructions for agents. This file is loaded at startup and can be customized for different use cases.

### Example System Prompt

```
You are an intelligent Digital Twin Agent designed to help users understand, 
simulate, and optimize complex systems through digital representations.

Your core responsibilities:
1. Understand and analyze digital twin models
2. Execute simulations and provide insights
3. Help users make data-driven decisions
4. Monitor system health and alert to issues
5. Provide recommendations for improvements
```

Modify this file to customize agent behavior for your specific use case.
