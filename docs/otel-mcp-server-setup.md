# OpenTelemetry MCP Server Setup Guide

This guide will help you integrate the `otel-mcp-server` with your existing logs infrastructure (OpenObserve, Kibana, and Elasticsearch) to supercharge your log searching with AI-powered insights.

## What is otel-mcp-server?

The OpenTelemetry MCP (Model Context Protocol) Server enables AI assistants like Claude to interact with your observability data directly. It allows you to:
- Query logs using natural language
- Analyze patterns and anomalies
- Get AI-powered insights from your log data
- Create complex queries without remembering Elasticsearch DSL syntax

## Prerequisites

- Node.js (v18 or higher)
- Access to your Elasticsearch instance (used by Kibana/OpenObserve)
- MCP-compatible client (Claude Desktop, Cline, or other MCP clients)

## Configuration

### Step 1: MCP Client Configuration

Add the following configuration to your MCP client's settings file:

**For Claude Desktop:**
Add to `claude_desktop_config.json` (usually at `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS or `%APPDATA%/Claude/claude_desktop_config.json` on Windows):

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

**For Cline (VS Code Extension):**
Add to your Cline MCP settings:

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

### Step 2: Environment Variables

Update the environment variables according to your setup:

| Variable | Description | Example |
|----------|-------------|---------|
| `ELASTICSEARCH_URL` | Your Elasticsearch endpoint URL | `http://localhost:9200` or `https://your-cluster.es.io:9200` |
| `ELASTICSEARCH_USERNAME` | Elasticsearch username | `elastic` |
| `ELASTICSEARCH_PASSWORD` | Elasticsearch password | Your secure password |
| `SERVER_NAME` | Name identifier for this server | `otel-mcp-server` |
| `LOGLEVEL` | Logging verbosity (OFF, ERROR, WARN, INFO, DEBUG) | `OFF` or `INFO` |

### Step 3: For OpenObserve Users

If you're using OpenObserve, you'll need to connect to its underlying storage:

```json
{
  "mcpServers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:5080",
        "ELASTICSEARCH_USERNAME": "root@example.com",
        "ELASTICSEARCH_PASSWORD": "your-openobserve-password",
        "SERVER_NAME": "openobserve-mcp",
        "LOGLEVEL": "INFO"
      }
    }
  }
}
```

**Note:** OpenObserve uses a different API structure. You may need to use the OpenObserve API endpoint and credentials.

## Usage Examples

Once configured, you can interact with your logs using natural language in your MCP client:

### Basic Queries

1. **Search recent errors:**
   ```
   Show me all ERROR level logs from the last hour
   ```

2. **Find specific service logs:**
   ```
   Get logs for the payment-service from the last 24 hours
   ```

3. **Analyze patterns:**
   ```
   What are the most common error messages in the last 6 hours?
   ```

### Advanced Queries

1. **Time-based analysis:**
   ```
   Show me the error rate trend for the API gateway over the last week
   ```

2. **Correlation:**
   ```
   Find all logs related to request ID abc-123-def
   ```

3. **Anomaly detection:**
   ```
   Are there any unusual patterns in the application logs today?
   ```

## Benefits Over Traditional Log Exploration

### Traditional Approach (Kibana/OpenObserve Dashboard)
- Manual query construction
- Need to know Elasticsearch DSL or specific query syntax
- Point-and-click interface
- Requires switching between UI and code

### AI-Powered Approach (with otel-mcp-server)
- Natural language queries
- AI understands context and intent
- Direct integration in your development environment
- Can combine multiple data sources
- Automated pattern recognition
- Proactive anomaly detection

## Integration with Your Current Setup

You can use otel-mcp-server **alongside** your existing tools:

1. **Keep using Kibana/OpenObserve** for:
   - Dashboard visualization
   - Long-term monitoring
   - Team-wide dashboards
   - Alert configuration

2. **Use otel-mcp-server** for:
   - Quick ad-hoc queries during development
   - Debugging specific issues
   - AI-assisted log analysis
   - Complex correlation queries
   - Learning about your system behavior

## Troubleshooting

### Connection Issues

1. **Cannot connect to Elasticsearch:**
   ```bash
   # Test your Elasticsearch connection
   curl -u elastic:changeme http://localhost:9200
   ```

2. **Authentication errors:**
   - Verify your username and password
   - Check if your user has the required permissions
   - For Elastic Cloud, use the Cloud ID format

3. **MCP Server not appearing:**
   - Restart your MCP client (Claude Desktop, etc.)
   - Check the logs in the client's developer tools
   - Verify Node.js is installed: `node --version`

### Performance Tips

1. **Use time ranges:** Always specify time ranges for better performance
2. **Index patterns:** Use specific index patterns when possible
3. **Limit results:** For initial exploration, limit result sets
4. **Adjust LOGLEVEL:** Set to `OFF` for production, `INFO` for debugging

## Security Best Practices

1. **Never commit credentials:** Use environment-specific configuration files
2. **Use read-only users:** Create a dedicated read-only Elasticsearch user
3. **Network security:** Use HTTPS for remote Elasticsearch connections
4. **Credential management:** Consider using secret management tools
5. **Access control:** Restrict index access to only what's needed

## Next Steps

1. Configure the MCP server with your Elasticsearch credentials
2. Restart your MCP client
3. Try a simple query: "Show me recent logs"
4. Explore more complex queries as you become familiar
5. Share useful queries with your team

## Resources

- [OpenTelemetry MCP Server Article](https://dev.to/shiftyp/supercharge-your-observability-how-otel-mcp-server-unlocks-ai-powered-insights-5dii)
- [Model Context Protocol Documentation](https://modelcontextprotocol.io/)
- [Elasticsearch Query DSL](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html)
- [OpenObserve Documentation](https://openobserve.ai/docs/)

## Support

For issues or questions:
- Check the [otel-mcp-server GitHub repository](https://github.com/modelcontextprotocol/servers)
- Review Elasticsearch connection settings
- Enable debug logging: Set `LOGLEVEL` to `DEBUG`
