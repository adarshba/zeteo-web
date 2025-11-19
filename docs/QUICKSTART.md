# Quick Start Guide

Get up and running with otel-mcp-server in 5 minutes.

## Prerequisites Check

Before starting, verify you have:

```bash
# Check Node.js version (need v18+)
node --version

# Test Elasticsearch connection
curl -u elastic:changeme http://localhost:9200
```

## Step 1: Identify Your Setup

Choose your scenario:

- **A) I use Kibana with local Elasticsearch**
- **B) I use OpenObserve**
- **C) I use Elastic Cloud**
- **D) I have a custom Elasticsearch setup**

## Step 2: Configure MCP Client

### For Claude Desktop

1. **Find your config file:**
   - macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - Windows: `%APPDATA%\Claude\claude_desktop_config.json`
   - Linux: `~/.config/Claude/claude_desktop_config.json`

2. **Add this configuration:**

**Option A - Local Kibana/Elasticsearch:**
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

**Option B - OpenObserve:**
```json
{
  "mcpServers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "http://localhost:5080",
        "ELASTICSEARCH_USERNAME": "root@example.com",
        "ELASTICSEARCH_PASSWORD": "your-password",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

**Option C - Elastic Cloud:**
```json
{
  "mcpServers": {
    "otel-mcp-server": {
      "command": "npx",
      "args": ["-y", "otel-mcp-server"],
      "env": {
        "ELASTICSEARCH_URL": "https://your-deployment.es.region.cloud.es.io:9243",
        "ELASTICSEARCH_USERNAME": "elastic",
        "ELASTICSEARCH_PASSWORD": "your-cloud-password",
        "SERVER_NAME": "otel-mcp-server",
        "LOGLEVEL": "OFF"
      }
    }
  }
}
```

3. **Update the credentials** with your actual values
4. **Save the file**
5. **Restart Claude Desktop completely** (quit and reopen)

### For Cline (VS Code)

1. Open VS Code
2. Go to Cline settings
3. Add MCP server configuration:

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

4. Reload VS Code window

## Step 3: Verify Connection

1. **Open your MCP client** (Claude Desktop or Cline)

2. **Test with a simple query:**
   ```
   Can you connect to my Elasticsearch cluster?
   ```

3. **If connected, try a real query:**
   ```
   Show me the latest 5 logs
   ```

## Step 4: Start Exploring

Try these queries to get familiar:

```
Show me all logs from the last hour
```

```
What are the most recent ERROR logs?
```

```
How many logs do we have from today?
```

```
Show me logs from [your-service-name]
```

## Troubleshooting

### "Cannot connect to Elasticsearch"

1. **Verify Elasticsearch is running:**
   ```bash
   curl http://localhost:9200
   ```

2. **Check credentials:**
   ```bash
   curl -u elastic:changeme http://localhost:9200
   ```

3. **Check if using HTTPS:**
   - Update `ELASTICSEARCH_URL` to use `https://` if needed

### "MCP server not showing up"

1. **Verify Node.js installation:**
   ```bash
   node --version  # Should be v18+
   ```

2. **Check config file location** - make sure you edited the right file

3. **Completely restart** your MCP client (don't just reload)

4. **Check for errors** in your MCP client's logs/developer tools

### "Authentication failed"

1. **Verify username and password** are correct
2. **For OpenObserve:** Use your OpenObserve login email and password
3. **For Elastic Cloud:** Use the elastic user or create a dedicated user

### "Getting empty results"

1. **Check your indices:**
   ```
   What indices are available?
   ```

2. **Use specific index patterns:**
   ```
   Show me logs from logstash-* index
   ```

3. **Verify time range:**
   ```
   Show me logs from the last 24 hours
   ```

## Next Steps

✅ **You're ready!** Your AI assistant can now query your logs.

**Learn More:**
- [Query Examples](query-examples.md) - Learn powerful query patterns
- [Configuration Examples](config-examples.md) - Advanced configurations
- [Full Setup Guide](otel-mcp-server-setup.md) - Complete documentation

**Pro Tips:**
1. Start with time-bound queries for better performance
2. Use specific service names when available
3. Ask follow-up questions to drill down
4. Try "Show me common error patterns" for insights

## Common First Queries

Copy and try these:

```
What logs do we have from the last 10 minutes?
```

```
Show me all ERROR and WARN level logs from today
```

```
What are the most active services in our logs?
```

```
Are there any errors in the payment service?
```

```
Show me the distribution of log levels over the last hour
```

## Need Help?

- **Configuration issues?** See [Configuration Examples](config-examples.md)
- **Query not working?** Check [Query Examples](query-examples.md)
- **Connection problems?** Review [Setup Guide](otel-mcp-server-setup.md) troubleshooting section

---

**Stuck?** Set `LOGLEVEL` to `DEBUG` in your configuration to see detailed logs, then restart your MCP client.
