# logs-explorer

AI-powered log exploration and analysis platform with Neurolink integration. Supercharge your log searching with natural language queries and intelligent insights powered by 100+ AI models across 12+ providers.

## 🎯 What's New

**✨ Neurolink Integration**: Now powered by Juspay Neurolink SDK for multi-provider AI operations!

This **monorepo** contains everything you need for AI-powered log analysis:

### 1️⃣ Full-Stack Web Application (Vercel-Ready)
**Location:** `frontend/` and `api/`
- 🚀 Vercel serverless functions with Neurolink AI
- 💬 Beautiful Svelte web interface
- 🤖 Multi-provider AI support (OpenAI, Anthropic, Google AI, etc.)
- ⚙️ Settings UI for Elasticsearch/OpenObserve configuration
- 🔍 Natural language log querying
- 📦 One-click Vercel deployment

**[→ Deployment Guide](VERCEL-DEPLOYMENT.md)**

### 2️⃣ MCP Server Integration
**Location:** Documentation files in `docs/`
- 📚 Complete otel-mcp-server setup guides
- 🔌 Works with Claude Desktop, Cline, etc.
- 💡 No code required - just configuration
- 🎨 Natural language queries in AI assistants

**[→ Quick Start](docs/QUICKSTART.md)** | **[→ Setup Guide](docs/otel-mcp-server-setup.md)**

### 3️⃣ Neurolink SDK Integration
**Location:** `api/` and `docs/`
- 🧠 Advanced AI SDK with multi-provider support (INTEGRATED!)
- 🏗️ Build custom log analysis applications
- 💰 Cost optimization across 12+ AI providers
- 📊 100+ models available

**[→ Integration Guide](docs/NEUROLINK-INTEGRATION.md)** | **[→ Usage Guide](docs/NEUROLINK-USAGE.md)**

### 4️⃣ Rust Backend (Legacy - For Local Use)
**Location:** `backend/`
- 🦀 High-performance Rust API server (OpenAI only)
- 💻 For local development and self-hosting
- ⚠️ Note: Not used in Vercel deployment

**[→ Setup Guide](docs/AI-BOT-SETUP.md)** | **[→ Rust Explained](docs/RUST-BACKEND-EXPLAINED.md)**

## 🚀 Quick Start

### Path A: Deploy to Vercel (Recommended - Production Ready!)

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https%3A%2F%2Fgithub.com%2Fadarshba%2Flogs-explorer)

```bash
# 1. One-click deploy or use Vercel CLI
npm i -g vercel
vercel login
vercel

# 2. Configure environment variables in Vercel Dashboard
AI_PROVIDER=openai
OPENAI_API_KEY=your_key_here
AI_MODEL=gpt-4o-mini

# 3. Done! Your app is live at https://your-app.vercel.app
```
**Best for:** Production deployments, teams, easy hosting

**[→ Deployment Guide](docs/VERCEL-DEPLOYMENT.md)**

### Path B: Local Development
```bash
# 1. Clone and install
git clone https://github.com/adarshba/logs-explorer.git
cd logs-explorer

# 2. Install dependencies
npm install

# 3. Configure environment
cp .env.example .env
# Edit .env with your AI provider settings

# 4. Run development server
npm run dev

# Frontend: http://localhost:5173
# API: Runs as serverless functions
```
**Best for:** Local development and testing

**[→ Local Setup Guide](docs/AI-BOT-SETUP.md)**

### Path C: AI Assistant (Claude Desktop, Cline)
```bash
# Add to your MCP client config (e.g., Claude Desktop)
# See docs/QUICKSTART.md for detailed instructions
```
**Best for:** Individual developers, ad-hoc debugging

**[→ Quick Start](docs/QUICKSTART.md)** | **[→ Setup Guide](docs/otel-mcp-server-setup.md)**

### Path D: Build Custom Apps with Neurolink
```bash
npm install @juspay/neurolink
# See docs/NEUROLINK-INTEGRATION.md for examples
```
**Best for:** Custom integrations, automation, SaaS products

**[→ Integration Guide](docs/NEUROLINK-INTEGRATION.md)** | **[→ Usage Examples](docs/NEUROLINK-USAGE.md)**

## 📁 Repository Structure

```
logs-explorer/                    # 🏠 Monorepo root
├── api/                         # ⚡ Vercel serverless functions
│   ├── query.ts                # Query parsing with Neurolink
│   ├── analyze.ts              # Log analysis (streaming)
│   ├── debug.ts                # AI-powered debugging
│   ├── health.ts               # Health check
│   └── package.json            # API dependencies
│
├── frontend/                    # 🎨 Svelte web UI
│   ├── src/
│   │   ├── routes/+page.svelte  # Main page
│   │   └── lib/                 # Components
│   │       ├── Settings.svelte      # Settings modal
│   │       ├── QueryInterface.svelte # Query input
│   │       └── LogResults.svelte    # Results display
│   ├── package.json             # Node dependencies
│   └── vite.config.ts           # Vite config
│
├── backend/                     # 🦀 Rust API server (optional)
│   ├── src/
│   │   ├── main.rs              # API endpoints
│   │   ├── ai.rs                # OpenAI integration
│   │   ├── elasticsearch.rs     # Elasticsearch client
│   │   └── openobserve.rs       # OpenObserve client
│   ├── Cargo.toml               # Rust dependencies
│   └── .env.example             # Environment template
│
├── docs/                        # 📚 Documentation
│   ├── VERCEL-DEPLOYMENT.md         # Vercel deployment guide
│   ├── NEUROLINK-USAGE.md           # How we use Neurolink
│   ├── NEUROLINK-INTEGRATION.md     # Neurolink integration guide
│   ├── STREAMING.md                 # Streaming implementation
│   ├── RUST-BACKEND-EXPLAINED.md    # Rust vs TypeScript
│   ├── AI-BOT-SETUP.md              # Bot setup guide
│   ├── QUICKSTART.md                # 5-minute quick start
│   ├── otel-mcp-server-setup.md     # MCP server guide
│   ├── config-examples.md           # Configuration examples
│   ├── query-examples.md            # Query examples
│   ├── COMPARISON.md                # Tool comparison
│   └── PROJECT-STRUCTURE.md         # Project structure
│
├── .env.example                 # Environment variables template
├── vercel.json                  # Vercel configuration
├── package.json                 # Root package config
└── README.md                    # 📖 This file
```

## 🤔 Why a Monorepo?

✅ **Single source of truth** - All code and docs in one place  
✅ **Easier setup** - Clone once, use everything  
✅ **Consistent versions** - Backend and frontend always in sync  
✅ **Simpler deployment** - Deploy as a unit  
✅ **Better DX** - Jump between backend and frontend easily  
✅ **Shared documentation** - All guides accessible together  

**When to split into separate repos:**
- If backend and frontend are deployed independently by different teams
- If you want different release cycles
- If repos become very large (>100k LOC each)

For this project, a **monorepo is perfect** and recommended! 🎉

## Overview

This repository provides comprehensive guides and configurations for integrating AI-powered log analysis into your existing observability stack (Kibana, OpenObserve, Elasticsearch).

### What You Get

- 🤖 **Natural Language Queries**: Ask questions in plain English instead of writing complex Elasticsearch DSL
- 🔍 **Intelligent Pattern Detection**: AI-powered anomaly detection and trend analysis
- ⚡ **Faster Debugging**: Get insights quickly without switching between multiple tools
- 🔗 **Seamless Integration**: Works alongside your existing Kibana and OpenObserve dashboards
- 🎯 **Context-Aware Analysis**: AI understands relationships between logs across services
- 🦀 **High Performance**: Rust-powered backend for speed and reliability
- 🎨 **Beautiful UI**: Modern Svelte frontend with settings and query interface

## 🏗️ Architecture Options

You can use this project in three ways:

### 1. Complete Web Application (Vercel Deployment)
**What:** Full-stack web app with AI-powered log querying  
**Location:** `api/` + `frontend/`  
**Best for:** Production deployments, teams, easy hosting  
**Setup time:** 5 minutes  
**[→ Setup Guide](docs/VERCEL-DEPLOYMENT.md)**

### 2. MCP Integration (AI Assistants)
**What:** Connect Claude Desktop or Cline to your logs  
**Location:** Configuration only (otel-mcp-server)  
**Best for:** Individual developers, quick debugging  
**Setup time:** 5 minutes  
**[→ Quick Start](docs/QUICKSTART.md)**

### 3. SDK Integration (Build Your Own)
**What:** Use Neurolink SDK to build custom apps  
**Location:** Your custom code + docs  
**Best for:** Custom tools, automation, SaaS  
**Setup time:** Varies  
**[→ Integration Guide](docs/NEUROLINK-INTEGRATION.md)**

## 📚 Documentation

- **[Vercel Deployment Guide](docs/VERCEL-DEPLOYMENT.md)** - Deploy to Vercel in one click
- **[Neurolink Usage](docs/NEUROLINK-USAGE.md)** - How we use Neurolink for AI
- **[Streaming Implementation](docs/STREAMING.md)** - Real-time streaming responses
- **[Rust Backend Explained](docs/RUST-BACKEND-EXPLAINED.md)** - Rust vs TypeScript architecture
- **[Setup Guide](docs/otel-mcp-server-setup.md)** - Complete installation and configuration
- **[Configuration Examples](docs/config-examples.md)** - Sample configurations
- **[Query Examples](docs/query-examples.md)** - Natural language query examples

## Example Usage

Instead of writing complex Elasticsearch queries, simply ask:

```
Show me all ERROR logs from the payment-service in the last hour
```

```
What caused the spike in errors at 2:30 PM?
```

```
Find all logs related to request ID abc-123-def
```

```
Are there any unusual patterns in today's logs?
```

## How It Works

```
┌─────────────────┐
│   AI Assistant  │ (Claude Desktop, Cline, etc.)
│  (MCP Client)   │
└────────┬────────┘
         │ Natural Language Queries
         │
┌────────▼────────┐
│ otel-mcp-server │ Translates queries to Elasticsearch DSL
└────────┬────────┘
         │
         ├──────────────┬─────────────┬─────────────┐
         │              │             │             │
    ┌────▼────┐   ┌────▼─────┐  ┌───▼──────┐ ┌────▼────────┐
    │Elasticsearch│ │  Kibana  │  │OpenObserve│ │Your Logs   │
    └────────────┘ └──────────┘  └───────────┘ └─────────────┘
```

## Use Cases

### Development
- Debug issues in real-time with conversational queries
- Understand log patterns during development
- Trace requests across microservices

### DevOps
- Investigate production incidents faster
- Identify anomalies and trends
- Monitor deployment impacts

### Team Collaboration
- Share natural language queries instead of complex DSL
- Document common debugging patterns
- Onboard team members faster

## Benefits Over Traditional Approaches

| Traditional (Kibana/OpenObserve) | AI-Powered (otel-mcp-server) |
|----------------------------------|------------------------------|
| Manual query construction | Natural language queries |
| Need to know query syntax | Conversational interface |
| Switch between tools | Integrated in development environment |
| Point-and-click exploration | AI-guided investigation |
| Manual pattern recognition | Automated insights |

## Compatibility

- ✅ Elasticsearch (any version)
- ✅ Kibana
- ✅ OpenObserve
- ✅ Elastic Cloud
- ✅ Self-hosted Elasticsearch clusters

## Requirements

- Node.js v18 or higher
- Access to Elasticsearch (via Kibana, OpenObserve, or direct)
- MCP-compatible client (Claude Desktop, Cline, etc.)

## Getting Help

- Check the [Setup Guide](otel-mcp-server-setup.md) for troubleshooting
- Review [Configuration Examples](config-examples.md) for your specific setup
- Try [Query Examples](query-examples.md) to learn effective querying

## Resources

- [OpenTelemetry MCP Server Article](https://dev.to/shiftyp/supercharge-your-observability-how-otel-mcp-server-unlocks-ai-powered-insights-5dii) - Deep dive into AI-powered observability
- [Model Context Protocol](https://modelcontextprotocol.io/) - Learn about MCP
- [Claude Desktop](https://claude.ai/desktop) - Popular MCP client
- [Elasticsearch Documentation](https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html)

## Contributing

This project was created using GitHub Copilot. Contributions, suggestions, and improvements are welcome!

## License

MIT

---

**Ready to supercharge your log searching?** Start with the [Setup Guide](otel-mcp-server-setup.md)!