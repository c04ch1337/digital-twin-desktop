# Deployment Guide

## Overview

This guide provides comprehensive instructions for deploying the Digital Twin Desktop application in various environments. It covers both desktop deployment for end-users and headless server deployment for enterprise scenarios.

## System Requirements

### Desktop Deployment

- **Operating Systems**:
  - Windows 10/11 (64-bit)
  - macOS 11.0+ (Big Sur or newer)
  - Ubuntu 20.04+ or other modern Linux distributions

- **Hardware Requirements**:
  - CPU: 4+ cores, 2.0 GHz or faster
  - RAM: 8 GB minimum, 16 GB recommended
  - Storage: 1 GB for application, plus additional space for twin data
  - GPU: OpenGL 3.3+ compatible graphics (for 3D visualization)

- **Software Dependencies**:
  - SQLite 3.35.0+
  - OpenSSL 1.1.1+

### Headless Server Deployment

- **Operating Systems**:
  - Ubuntu Server 20.04+ LTS
  - Debian 11+
  - RHEL/CentOS 8+
  - Windows Server 2019+

- **Hardware Requirements**:
  - CPU: 8+ cores, 2.5 GHz or faster
  - RAM: 16 GB minimum, 32 GB recommended
  - Storage: 2 GB for application, plus additional space for twin data
  - Network: 1 Gbps Ethernet

- **Software Dependencies**:
  - Docker 20.10+ and Docker Compose 2.0+ (for containerized deployment)
  - SQLite 3.35.0+ or PostgreSQL 13+ (for database)
  - Nginx or Apache (for reverse proxy)
  - Let's Encrypt Certbot (for SSL certificates)

## Desktop Deployment

### Installation

#### Windows

1. Download the latest Windows installer (`digital-twin-desktop-setup-x.y.z.exe`) from the releases page
2. Run the installer with administrator privileges
3. Follow the installation wizard:
   - Choose installation directory
   - Select components to install
   - Configure start menu and desktop shortcuts
4. Launch the application from the Start menu or desktop shortcut

#### macOS

1. Download the latest macOS package (`digital-twin-desktop-x.y.z.dmg`) from the releases page
2. Open the DMG file
3. Drag the Digital Twin Desktop application to the Applications folder
4. Right-click the application and select "Open" to bypass Gatekeeper on first launch
5. Follow the setup wizard to complete installation

#### Linux

##### Using AppImage

1. Download the latest AppImage (`digital-twin-desktop-x.y.z.AppImage`) from the releases page
2. Make the AppImage executable:
   ```bash
   chmod +x digital-twin-desktop-x.y.z.AppImage
   ```
3. Run the AppImage:
   ```bash
   ./digital-twin-desktop-x.y.z.AppImage
   ```

##### Using Debian/Ubuntu Package

1. Download the latest DEB package (`digital-twin-desktop-x.y.z.deb`) from the releases page
2. Install the package:
   ```bash
   sudo apt install ./digital-twin-desktop-x.y.z.deb
   ```
3. Launch the application from the application menu or command line:
   ```bash
   digital-twin-desktop
   ```

### Configuration

The desktop application stores its configuration in the following locations:

- **Windows**: `%APPDATA%\digital-twin-desktop\config.json`
- **macOS**: `~/Library/Application Support/digital-twin-desktop/config.json`
- **Linux**: `~/.config/digital-twin-desktop/config.json`

The configuration file can be edited manually or through the application's settings interface.

#### Example Configuration

```json
{
  "app": {
    "theme": "dark",
    "language": "en",
    "auto_update": true,
    "start_minimized": false,
    "hardware_acceleration": true
  },
  "database": {
    "path": "default",
    "backup_enabled": true,
    "backup_interval_hours": 24,
    "max_backups": 7
  },
  "llm": {
    "provider": "anthropic",
    "api_key": "encrypted:AES256:...",
    "model": "claude-3-sonnet-20240229",
    "temperature": 0.7,
    "max_tokens": 4096
  },
  "twin": {
    "sync_interval_seconds": 5,
    "realtime_updates": true,
    "simulation_timestep_ms": 100
  },
  "security": {
    "encrypt_api_keys": true,
    "encrypt_conversations": false,
    "require_confirmation": true
  },
  "logging": {
    "level": "info",
    "file_enabled": true,
    "max_log_files": 10,
    "max_file_size_mb": 10
  }
}
```

### Updates

The application includes an auto-update mechanism that can be configured in the settings:

1. Open the application
2. Go to Settings > General
3. Enable or disable automatic updates
4. Set update check frequency
5. Choose update channel (stable, beta, nightly)

To manually update:

1. Download the latest version from the releases page
2. Install over the existing installation
3. Your data and settings will be preserved

## Headless Server Deployment

The Digital Twin Desktop can be deployed as a headless server for enterprise scenarios, providing API access and multi-user support.

### Docker Deployment

#### Prerequisites

- Docker 20.10+ and Docker Compose 2.0+
- 16+ GB RAM
- 4+ CPU cores
- 10+ GB free disk space

#### Deployment Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/digital-twin-desktop.git
   cd digital-twin-desktop
   ```

2. Create a `.env` file based on the example:
   ```bash
   cp .env.local.example .env.local
   ```

3. Edit the `.env.local` file to configure your deployment:
   ```bash
   # Required settings
   ENVIRONMENT="production"
   APP_DATA_DIR="/data"
   
   # Database settings
   DATABASE_NAME="digital_twin.db"
   
   # LLM settings
   ANTHROPIC_API_KEY="your-api-key"
   ANTHROPIC_MODEL="claude-3-sonnet-20240229"
   
   # Security settings
   SECURITY_MASTER_KEY="generate-a-secure-random-key"
   SECURITY_ENCRYPT_API_KEYS="true"
   
   # API settings
   API_ENABLE_AUTH="true"
   API_JWT_SECRET="generate-a-secure-random-key"
   API_RATE_LIMIT_ENABLED="true"
   ```

4. Build and start the containers:
   ```bash
   docker-compose up -d
   ```

5. Verify the deployment:
   ```bash
   docker-compose ps
   ```

#### Docker Compose Configuration

```yaml
version: '3.8'

services:
  app:
    image: digital-twin-desktop:latest
    build:
      context: .
      dockerfile: Dockerfile
    restart: unless-stopped
    ports:
      - "3000:3000"  # API port
    volumes:
      - ./data:/data
      - ./logs:/logs
    env_file:
      - .env.local
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  nginx:
    image: nginx:alpine
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/conf.d:/etc/nginx/conf.d
      - ./nginx/ssl:/etc/nginx/ssl
      - ./nginx/www:/var/www/html
    depends_on:
      - app
```

### Kubernetes Deployment

For larger deployments, Kubernetes is recommended:

1. Create a namespace:
   ```bash
   kubectl create namespace digital-twin
   ```

2. Create a ConfigMap for configuration:
   ```bash
   kubectl create configmap digital-twin-config --from-file=config.json -n digital-twin
   ```

3. Create Secrets for sensitive data:
   ```bash
   kubectl create secret generic digital-twin-secrets \
     --from-literal=ANTHROPIC_API_KEY=your-api-key \
     --from-literal=SECURITY_MASTER_KEY=your-master-key \
     --from-literal=API_JWT_SECRET=your-jwt-secret \
     -n digital-twin
   ```

4. Apply the deployment manifest:
   ```bash
   kubectl apply -f kubernetes/deployment.yaml -n digital-twin
   ```

5. Apply the service manifest:
   ```bash
   kubectl apply -f kubernetes/service.yaml -n digital-twin
   ```

6. Apply the ingress manifest:
   ```bash
   kubectl apply -f kubernetes/ingress.yaml -n digital-twin
   ```

7. Verify the deployment:
   ```bash
   kubectl get pods -n digital-twin
   ```

### Manual Deployment

For environments without Docker or Kubernetes:

1. Install Rust and required dependencies:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustup default stable
   
   # Install system dependencies
   sudo apt update
   sudo apt install -y build-essential libssl-dev pkg-config
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/your-org/digital-twin-desktop.git
   cd digital-twin-desktop
   ```

3. Build the application in release mode:
   ```bash
   cargo build --release --features headless
   ```

4. Create configuration directory:
   ```bash
   sudo mkdir -p /etc/digital-twin-desktop
   sudo cp .env.local.example /etc/digital-twin-desktop/.env
   ```

5. Edit the configuration:
   ```bash
   sudo nano /etc/digital-twin-desktop/.env
   ```

6. Create a systemd service:
   ```bash
   sudo nano /etc/systemd/system/digital-twin.service
   ```

   With the following content:
   ```
   [Unit]
   Description=Digital Twin Desktop Headless Server
   After=network.target
   
   [Service]
   Type=simple
   User=digital-twin
   Group=digital-twin
   WorkingDirectory=/opt/digital-twin-desktop
   ExecStart=/opt/digital-twin-desktop/target/release/digital-twin-desktop --headless
   Restart=on-failure
   Environment="CONFIG_PATH=/etc/digital-twin-desktop/.env"
   
   [Install]
   WantedBy=multi-user.target
   ```

7. Create user and group:
   ```bash
   sudo useradd -r -s /bin/false digital-twin
   ```

8. Install the application:
   ```bash
   sudo mkdir -p /opt/digital-twin-desktop
   sudo cp -r target/release /opt/digital-twin-desktop/
   sudo cp -r scripts /opt/digital-twin-desktop/
   sudo chown -R digital-twin:digital-twin /opt/digital-twin-desktop
   ```

9. Enable and start the service:
   ```bash
   sudo systemctl enable digital-twin
   sudo systemctl start digital-twin
   ```

10. Check the service status:
    ```bash
    sudo systemctl status digital-twin
    ```

## API Configuration

The headless server exposes a REST API for integration with other systems.

### Authentication

The API supports JWT-based authentication:

1. Enable authentication in the configuration:
   ```
   API_ENABLE_AUTH=true
   API_JWT_SECRET=your-secure-secret
   ```

2. Create API users:
   ```bash
   digital-twin-desktop --headless --create-user username password
   ```

3. Authenticate to get a token:
   ```bash
   curl -X POST http://localhost:3000/api/auth/login \
     -H "Content-Type: application/json" \
     -d '{"username":"username","password":"password"}'
   ```

4. Use the token in subsequent requests:
   ```bash
   curl -X GET http://localhost:3000/api/twins \
     -H "Authorization: Bearer your-token"
   ```

### Rate Limiting

Configure rate limiting to prevent API abuse:

```
API_RATE_LIMIT_ENABLED=true
API_RATE_LIMIT_MAX_REQUESTS=60
API_RATE_LIMIT_WINDOW_SECONDS=60
```

### CORS Configuration

Configure CORS for web clients:

```
API_CORS_ENABLED=true
API_CORS_ALLOWED_ORIGINS=https://example.com,https://app.example.com
API_CORS_ALLOWED_METHODS=GET,POST,PUT,DELETE
API_CORS_ALLOWED_HEADERS=Content-Type,Authorization
API_CORS_MAX_AGE=86400
```

## Database Configuration

### SQLite (Default)

The application uses SQLite by default, which is suitable for most deployments:

```
DATABASE_NAME=digital_twin.db
DATABASE_WAL_MODE=true
DATABASE_AUTO_VACUUM=incremental
DATABASE_FOREIGN_KEYS=true
```

### PostgreSQL (Optional)

For larger deployments, PostgreSQL can be used:

1. Install PostgreSQL:
   ```bash
   sudo apt install postgresql postgresql-contrib
   ```

2. Create a database and user:
   ```bash
   sudo -u postgres psql
   ```
   ```sql
   CREATE DATABASE digital_twin;
   CREATE USER digital_twin WITH ENCRYPTED PASSWORD 'your-password';
   GRANT ALL PRIVILEGES ON DATABASE digital_twin TO digital_twin;
   \q
   ```

3. Configure the application to use PostgreSQL:
   ```
   DATABASE_TYPE=postgres
   DATABASE_URL=postgres://digital_twin:your-password@localhost/digital_twin
   DATABASE_POOL_SIZE=10
   ```

## Security Considerations

### API Key Encryption

API keys for external services (like LLM providers) are encrypted at rest:

```
SECURITY_ENCRYPT_API_KEYS=true
SECURITY_MASTER_KEY=your-secure-master-key
```

Generate a secure master key:
```bash
openssl rand -hex 32
```

### SSL/TLS Configuration

For production deployments, configure SSL/TLS:

1. Obtain SSL certificates (e.g., using Let's Encrypt)
2. Configure Nginx or Apache as a reverse proxy

Example Nginx configuration:

```nginx
server {
    listen 80;
    server_name digital-twin.example.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name digital-twin.example.com;

    ssl_certificate /etc/letsencrypt/live/digital-twin.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/digital-twin.example.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### Firewall Configuration

Configure firewall rules to restrict access:

```bash
# Allow SSH
sudo ufw allow 22/tcp

# Allow HTTP/HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Block direct access to the API port
sudo ufw deny 3000/tcp

# Enable the firewall
sudo ufw enable
```

## Monitoring and Logging

### Log Configuration

Configure logging in the `.env` file:

```
LOG_LEVEL=info
LOG_FILE_ENABLED=true
LOG_FILE_PATH=/var/log/digital-twin-desktop/app.log
LOG_MAX_FILES=10
LOG_MAX_FILE_SIZE_MB=10
```

### Health Checks

The application provides a health check endpoint:

```
GET /health
```

Response:
```json
{
  "status": "ok",
  "version": "1.0.0",
  "uptime": 3600,
  "database": "connected",
  "llm_service": "connected"
}
```

### Prometheus Metrics

Enable Prometheus metrics for monitoring:

```
METRICS_ENABLED=true
METRICS_PATH=/metrics
```

Access metrics at:
```
GET /metrics
```

### Grafana Dashboard

A sample Grafana dashboard is provided in the `monitoring` directory:

1. Import the dashboard JSON into Grafana
2. Configure a Prometheus data source
3. Monitor application performance, API usage, and system resources

## Backup and Recovery

### Automated Backups

Configure automated backups:

```
BACKUP_ENABLED=true
BACKUP_INTERVAL_HOURS=24
BACKUP_MAX_FILES=7
BACKUP_DIRECTORY=/var/backups/digital-twin-desktop
```

### Manual Backup

Perform a manual backup:

```bash
digital-twin-desktop --headless --backup
```

### Restore from Backup

Restore from a backup:

```bash
digital-twin-desktop --headless --restore /path/to/backup.zip
```

## Troubleshooting

### Common Issues

#### Application Won't Start

1. Check the logs:
   ```bash
   sudo journalctl -u digital-twin
   ```

2. Verify the configuration:
   ```bash
   digital-twin-desktop --headless --verify-config
   ```

3. Check file permissions:
   ```bash
   sudo chown -R digital-twin:digital-twin /opt/digital-twin-desktop
   sudo chmod -R 750 /opt/digital-twin-desktop
   ```

#### Database Connection Issues

1. Verify the database exists:
   ```bash
   ls -la /data/digital_twin.db
   ```

2. Check database permissions:
   ```bash
   sudo chown digital-twin:digital-twin /data/digital_twin.db
   sudo chmod 640 /data/digital_twin.db
   ```

3. For PostgreSQL, test the connection:
   ```bash
   psql -U digital_twin -h localhost -d digital_twin
   ```

#### API Connection Refused

1. Verify the service is running:
   ```bash
   sudo systemctl status digital-twin
   ```

2. Check if the port is open:
   ```bash
   sudo netstat -tuln | grep 3000
   ```

3. Check firewall rules:
   ```bash
   sudo ufw status
   ```

### Diagnostic Tools

The application includes diagnostic tools:

```bash
# Check system requirements
digital-twin-desktop --headless --check-system

# Verify configuration
digital-twin-desktop --headless --verify-config

# Test database connection
digital-twin-desktop --headless --test-db

# Test LLM connection
digital-twin-desktop --headless --test-llm

# Generate diagnostic report
digital-twin-desktop --headless --diagnostics > diagnostics.json
```

## Conclusion

This deployment guide covers the essential steps for deploying the Digital Twin Desktop application in both desktop and server environments. For additional support, refer to the project documentation or contact the support team.