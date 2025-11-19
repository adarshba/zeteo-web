# AI Log Query Bot - Setup Guide

Complete implementation of an AI-powered log analysis bot with Rust backend and Svelte frontend.

## 🏗️ Architecture

```
┌──────────────────────────────────────────────────────────┐
│                  Svelte Frontend (Port 5173)              │
│  - Natural language query interface                       │
│  - Settings for Elasticsearch/OpenObserve                 │
│  - Beautiful log display and analysis                     │
└────────────────────┬─────────────────────────────────────┘
                     │ HTTP API
┌────────────────────▼─────────────────────────────────────┐
│                  Rust Backend (Port 3001)                 │
│  - High-performance API server                            │
│  - AI query parsing with OpenAI                           │
│  - Elasticsearch/OpenObserve integration                  │
└────────────────────┬─────────────────────────────────────┘
                     │
        ┌────────────┴───────────┐
        │                        │
┌───────▼──────┐        ┌────────▼─────────┐
│ Elasticsearch │        │   OpenObserve    │
│  / Kibana     │        │                  │
└───────────────┘        └──────────────────┘
```

## 📋 Prerequisites

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** v18+ and pnpm
   ```bash
   curl -fsSL https://get.pnpm.io/install.sh | sh -
   ```

3. **OpenAI API Key**
   - Get one from https://platform.openai.com/api-keys

4. **Log Source** (one of):
   - Elasticsearch (with Kibana)
   - OpenObserve

## 🚀 Quick Start

### Step 1: Configure Environment

Create `.env` file in `backend/`:

```bash
cd backend
cat > .env << 'EOF'
# OpenAI Configuration (Required)
OPENAI_API_KEY=sk-your-key-here
OPENAI_MODEL=gpt-4o-mini

# Optional: Pre-configure log source (can also use frontend settings)
# ELASTICSEARCH_URL=http://localhost:9200
# ELASTICSEARCH_USERNAME=elastic
# ELASTICSEARCH_PASSWORD=changeme

# or for OpenObserve:
# OPENOBSERVE_URL=http://localhost:5080
# OPENOBSERVE_USERNAME=root@example.com
# OPENOBSERVE_PASSWORD=yourpassword
# OPENOBSERVE_ORG=default
EOF
```

### Step 2: Start Backend

```bash
# In backend/ directory
cargo run --release
```

The backend will start on `http://localhost:3001`

### Step 3: Start Frontend

```bash
# In frontend/ directory
pnpm install
pnpm run dev
```

The frontend will start on `http://localhost:5173`

### Step 4: Configure Log Source

1. Open `http://localhost:5173` in your browser
2. Click the **⚙️ Settings** button
3. Enter your log source details:
   - **For Elasticsearch/Kibana:**
     - URL: `http://localhost:9200`
     - Username: `elastic`
     - Password: your password
   
   - **For OpenObserve:**
     - URL: `http://localhost:5080`
     - Username: `root@example.com`
     - Password: your password
     - Organization: `default`

4. Click **Save Settings**

### Step 5: Start Querying!

Try these natural language queries:
- "Show recent errors"
- "Database timeouts in the last hour"
- "Payment service errors"
- "Show me all WARNING and ERROR logs from today"

## 📚 API Documentation

### Health Check

```bash
curl http://localhost:3001/api/health
```

Response:
```json
{
  "status": "healthy",
  "ai_enabled": true
}
```

### Query Logs

```bash
curl -X POST http://localhost:3001/api/query \
  -H "Content-Type: application/json" \
  -d '{
    "query": "show recent errors",
    "source": "elasticsearch",
    "config": {
      "url": "http://localhost:9200",
      "username": "elastic",
      "password": "changeme"
    }
  }'
```

### Debug Issue

```bash
curl -X POST http://localhost:3001/api/debug \
  -H "Content-Type: application/json" \
  -d '{
    "issue_description": "Users cannot log in",
    "context": {
      "service": "auth-service",
      "time_range": "1h"
    },
    "config": {
      "source": "elasticsearch",
      "url": "http://localhost:9200",
      "username": "elastic",
      "password": "changeme"
    }
  }'
```

## 🎯 Features

### Frontend Features
- ✅ Natural language query interface
- ✅ Settings page for log source configuration
- ✅ Beautiful log display with syntax highlighting
- ✅ Expandable log entries
- ✅ Copy log JSON to clipboard
- ✅ Multiple log source support (Elasticsearch & OpenObserve)
- ✅ Real-time AI-powered analysis
- ✅ Example queries for quick testing

### Backend Features
- ✅ **Rust-powered**: High performance and memory safety
- ✅ **AI Integration**: OpenAI GPT-4o-mini for query parsing
- ✅ **Multi-source support**: Works with Elasticsearch and OpenObserve
- ✅ **Smart query parsing**: Natural language to search parameters
- ✅ **Debug assistance**: AI-powered root cause analysis
- ✅ **CORS enabled**: Works with any frontend
- ✅ **Type-safe**: Strongly typed throughout

## 🔧 Development

### Backend Development

```bash
cd backend

# Run with hot reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Run tests
cargo test

# Build for production
cargo build --release
```

### Frontend Development

```bash
cd frontend

# Install dependencies
pnpm install

# Run dev server with hot reload
pnpm run dev

# Build for production
pnpm run build

# Preview production build
pnpm run preview
```

## 🚢 Production Deployment

### Backend (Rust)

```bash
cd backend

# Build optimized binary
cargo build --release

# The binary will be at: target/release/logs-explorer-backend

# Run in production
./target/release/logs-explorer-backend
```

### Frontend (Svelte)

```bash
cd frontend

# Build static files
pnpm run build

# Serve with any static file server
# The built files are in: build/
```

Example with nginx:
```nginx
server {
    listen 80;
    server_name logs.example.com;

    root /path/to/frontend/build;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api {
        proxy_pass http://localhost:3001;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 🐳 Docker Deployment

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  backend:
    build: ./backend
    ports:
      - "3001:3001"
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - OPENAI_MODEL=gpt-4o-mini
    restart: unless-stopped

  frontend:
    build: ./frontend
    ports:
      - "80:80"
    depends_on:
      - backend
    restart: unless-stopped
```

## 📝 Configuration Options

### Backend Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `OPENAI_API_KEY` | Yes | - | OpenAI API key |
| `OPENAI_MODEL` | No | `gpt-4o-mini` | OpenAI model to use |
| `RUST_LOG` | No | `info` | Log level (trace, debug, info, warn, error) |

### Frontend Configuration

All configuration is done through the Settings UI and stored in browser localStorage:
- Log source (Elasticsearch or OpenObserve)
- Connection URLs and credentials
- Organization (for OpenObserve)

## 🔒 Security

### Best Practices

1. **HTTPS in Production**: Always use HTTPS
2. **Read-only Credentials**: Use read-only database users
3. **API Key Protection**: Never expose OpenAI API key in frontend
4. **CORS Configuration**: Restrict in production
5. **Rate Limiting**: Add rate limiting for API endpoints

### Example Read-Only Elasticsearch User

```bash
# Create read-only role
POST /_security/role/logs_reader
{
  "cluster": ["monitor"],
  "indices": [
    {
      "names": ["logs-*", "filebeat-*"],
      "privileges": ["read", "view_index_metadata"]
    }
  ]
}

# Create user
POST /_security/user/log_viewer
{
  "password": "secure-password",
  "roles": ["logs_reader"]
}
```

## 🐛 Troubleshooting

### Backend Issues

**Rust compilation errors:**
```bash
# Update Rust
rustup update

# Clear cache and rebuild
cargo clean
cargo build
```

**Can't connect to log source:**
- Check URL format (include http:// or https://)
- Verify credentials
- Check network connectivity
- Review backend logs

### Frontend Issues

**Build errors:**
```bash
# Clear node modules and reinstall
rm -rf node_modules
pnpm install
```

**API connection errors:**
- Ensure backend is running on port 3001
- Check CORS configuration
- Review browser console for errors

## 📊 Performance Tips

1. **Use connection pooling** for databases
2. **Enable gzip compression** for API responses
3. **Cache AI responses** for common queries
4. **Limit result size** (default: 100 logs)
5. **Use time ranges** to reduce search scope

## 🎓 Example Queries

### Simple Queries
- "Recent logs"
- "Show errors"
- "What happened in the last hour?"

### Specific Searches
- "Database connection errors"
- "Payment service timeouts"
- "Authentication failures today"

### Complex Queries
- "Show ERROR and WARN logs from the payment-service in the last 2 hours"
- "Find all database timeout errors in the production environment"
- "What caused the spike in errors at 3pm?"

## 📈 Monitoring

Monitor your deployment:

```bash
# Backend health
curl http://localhost:3001/api/health

# Frontend (should return 200)
curl http://localhost:5173
```

## 🤝 Contributing

Contributions welcome! Areas for improvement:
- Add more AI providers (Anthropic, Google, etc.)
- Support more log backends
- Add caching layer
- Implement WebSocket for streaming
- Add user authentication
- Create CLI interface

## 📄 License

MIT License - see LICENSE file for details

## 🙏 Acknowledgments

- Built with Rust, Svelte, and OpenAI
- Inspired by modern observability tools
- Thanks to the open-source community

---

**Need Help?** Open an issue on GitHub or check the documentation in the `/docs` folder.
