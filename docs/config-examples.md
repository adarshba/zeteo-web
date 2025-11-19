# Configuration Examples for otel-mcp-server

This file contains various configuration examples for different scenarios and setups.

## Basic Configurations

### Local Elasticsearch (Default Kibana Setup)

```json
{
  "servers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "changeme",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

### Local OpenObserve

```json
{
  "servers": {
    "openobserve-mcp": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:5080",
        "ELASTICSEARCH_USERNAME": "root@example.com",
        "ELASTICSEARCH_PASSWORD": "Complexpass#123",
        "SERVER_NAME": "openobserve-mcp",
        "LOGLEVEL": "INFO"
      }
    }
  }
}
```

## Production Configurations

### Elastic Cloud

```json
{
  "servers": {
    "elastic-cloud-mcp": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "https://my-deployment.es.us-central1.gcp.cloud.es.io:9243",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "your-secure-password",
        "SERVER_NAME": "elastic-cloud-mcp",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

### Self-Hosted with TLS

```json
{
  "servers": {
    "secure-es-mcp": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "https://elasticsearch.company.com:9200",
        "ELASTICSEARCH_USERNAME": "log_viewer",
        "ELASTICSEARCH_PASSWORD": "read-only-password",
        "SERVER_NAME": "company-es-mcp",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

## Multi-Environment Setup

You can configure multiple MCP servers to connect to different environments:

```json
{
  "servers": {
    "production-logs": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "https://prod-es.company.com:9200",
        "ELASTICSEARCH_USERNAME": "prod_reader",
        "ELASTICSEARCH_PASSWORD": "prod-password",
        "SERVER_NAME": "production-logs",
        "LOGLEVEL": "OFF"
      }
    },
    "staging-logs": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "https://staging-es.company.com:9200",
        "ELASTICSEARCH_USERNAME": "staging_reader",
        "ELASTICSEARCH_PASSWORD": "staging-password",
        "SERVER_NAME": "staging-logs",
        "LOGLEVEL": "INFO"
      }
    },
    "development-logs": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "dev-password",
        "SERVER_NAME": "development-logs",
        "LOGLEVEL": "DEBUG"
      }
    }
  }
}
```

## Using Environment Variables

For better security, you can use environment variables:

```json
{
  "servers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "${ELASTICSEARCH_URL}",
        "ELASTICSEARCH_USERNAME": "${ELASTICSEARCH_USERNAME}",
        "ELASTICSEARCH_PASSWORD": "${ELASTICSEARCH_PASSWORD}",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

Then set your environment variables:
```bash
export ELASTICSEARCH_URL="http://localhost:9200"
export ELASTICSEARCH_USERNAME="elastic"
export ELASTICSEARCH_PASSWORD="changeme"
```

## Platform-Specific Configurations

### Claude Desktop (macOS)

Configuration file location: `~/Library/Application Support/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "changeme",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

### Claude Desktop (Windows)

Configuration file location: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "changeme",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

### Cline (VS Code)

Add to Cline's MCP settings in VS Code:

```json
{
  "servers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "changeme",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

## Debugging Configuration

For troubleshooting connection or query issues:

```json
{
  "servers": {
    "otel-mcp-server-debug": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "changeme",
        "SERVER_NAME": "otel-mcp-server-debug",
        "LOGLEVEL": "DEBUG"
      }
    }
  }
}
```

## Docker-Based Elasticsearch

If running Elasticsearch in Docker:

```json
{
  "servers": {
    "docker-es-mcp": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "docker-password",
        "SERVER_NAME": "docker-es-mcp",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

Docker Compose example for testing:

```yaml
version: '3.8'
services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.11.0
    environment:
      - discovery.type=single-node
      - ELASTIC_PASSWORD=changeme
      - xpack.security.enabled=true
    ports:
      - "9200:9200"
    volumes:
      - es-data:/usr/share/elasticsearch/data

volumes:
  es-data:
    driver: local
```

## Read-Only User Configuration

For security, create a read-only Elasticsearch user:

```bash
# Using Elasticsearch Security API
POST /_security/role/logs_reader
{
  "cluster": ["monitor"],
  "indices": [
    {
      "names": ["logs-*", "filebeat-*", "logstash-*"],
      "privileges": ["read", "view_index_metadata"]
    }
  ]
}

POST /_security/user/log_viewer
{
  "password": "secure-read-only-password",
  "roles": ["logs_reader"],
  "full_name": "Log Viewer",
  "email": "logviewer@company.com"
}
```

Then use in configuration:

```json
{
  "servers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:9200",
        "ELASTICSEARCH_USERNAME": "log_viewer",
        "ELASTICSEARCH_PASSWORD": "secure-read-only-password",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

## Notes

- Always use HTTPS in production environments
- Never commit credentials to version control
- Use read-only users when possible
- Test connectivity before configuring in your MCP client
- Adjust `LOGLEVEL` based on your needs (OFF for production, DEBUG for troubleshooting)
