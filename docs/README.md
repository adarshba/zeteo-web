# Documentation Index

Welcome to the Logs Explorer documentation! This guide will help you navigate all available documentation.

## 🚀 Getting Started

### Quick Start Guides
- **[QUICKSTART.md](QUICKSTART.md)** - Get up and running in 5 minutes
- **[VERCEL-DEPLOYMENT.md](VERCEL-DEPLOYMENT.md)** - Deploy to Vercel (recommended)
- **[AI-BOT-SETUP.md](AI-BOT-SETUP.md)** - Set up the Rust backend locally

## 🧠 Neurolink AI Integration

### Core Documentation
- **[NEUROLINK-USAGE.md](NEUROLINK-USAGE.md)** - How we use Neurolink in this project
- **[NEUROLINK-INTEGRATION.md](NEUROLINK-INTEGRATION.md)** - Complete Neurolink integration guide
- **[STREAMING.md](STREAMING.md)** - Real-time streaming implementation (much cooler!)

## 🏗️ Architecture

### Understanding the System
- **[RUST-BACKEND-EXPLAINED.md](RUST-BACKEND-EXPLAINED.md)** - Rust vs TypeScript architecture explained
- **[PROJECT-STRUCTURE.md](PROJECT-STRUCTURE.md)** - Project organization and structure
- **[PROJECT-COMPLETE.md](PROJECT-COMPLETE.md)** - Complete project overview

## 📖 Configuration & Usage

### Configuration
- **[config-examples.md](config-examples.md)** - Sample configurations for various scenarios
- **[otel-mcp-server-setup.md](otel-mcp-server-setup.md)** - MCP server setup guide

### Usage Examples
- **[query-examples.md](query-examples.md)** - Natural language query examples
- **[COMPARISON.md](COMPARISON.md)** - Tool comparison and use cases

## 📚 Documentation by Use Case

### I want to deploy to production
1. Read [VERCEL-DEPLOYMENT.md](VERCEL-DEPLOYMENT.md)
2. Review [NEUROLINK-USAGE.md](NEUROLINK-USAGE.md)
3. Check [config-examples.md](config-examples.md)

### I want to develop locally
1. Start with [QUICKSTART.md](QUICKSTART.md)
2. Follow [AI-BOT-SETUP.md](AI-BOT-SETUP.md) for Rust backend
3. Or use TypeScript API from [RUST-BACKEND-EXPLAINED.md](RUST-BACKEND-EXPLAINED.md)

### I want to understand the AI integration
1. Read [NEUROLINK-USAGE.md](NEUROLINK-USAGE.md)
2. Explore [STREAMING.md](STREAMING.md)
3. See examples in [NEUROLINK-INTEGRATION.md](NEUROLINK-INTEGRATION.md)

### I want to build custom integrations
1. Start with [NEUROLINK-INTEGRATION.md](NEUROLINK-INTEGRATION.md)
2. Review [query-examples.md](query-examples.md)
3. Check [config-examples.md](config-examples.md)

### I want to use MCP (Claude Desktop, Cline)
1. Follow [QUICKSTART.md](QUICKSTART.md)
2. Read [otel-mcp-server-setup.md](otel-mcp-server-setup.md)
3. See [query-examples.md](query-examples.md)

## 🔍 Quick Reference

### Deployment Options
| Option | Documentation | Best For |
|--------|--------------|----------|
| **Vercel** | [VERCEL-DEPLOYMENT.md](VERCEL-DEPLOYMENT.md) | Production, teams |
| **Local Development** | [AI-BOT-SETUP.md](AI-BOT-SETUP.md) | Development |
| **MCP Integration** | [otel-mcp-server-setup.md](otel-mcp-server-setup.md) | Individual use |

### AI Providers
All AI providers supported by Neurolink work with this project:
- OpenAI (including Azure OpenAI)
- Anthropic Claude
- Google AI (Gemini)
- Cohere, Mistral, Groq, and more

See [NEUROLINK-USAGE.md](NEUROLINK-USAGE.md) for details.

### Architecture Options
| Architecture | Documentation | When to Use |
|--------------|---------------|-------------|
| **TypeScript + Neurolink** | [VERCEL-DEPLOYMENT.md](VERCEL-DEPLOYMENT.md) | Vercel, multi-provider AI |
| **Rust Backend** | [RUST-BACKEND-EXPLAINED.md](RUST-BACKEND-EXPLAINED.md) | Self-hosting, performance |

## 💡 Features

### Real-Time Streaming
Learn how we implement real-time AI responses:
- [STREAMING.md](STREAMING.md)

### Multi-Provider AI
Switch between OpenAI, Anthropic, Google AI, and more:
- [NEUROLINK-USAGE.md](NEUROLINK-USAGE.md)

### Natural Language Queries
Query logs in plain English:
- [query-examples.md](query-examples.md)

## 🆘 Getting Help

### Common Questions

**Q: Should I use Rust or TypeScript backend?**
A: See [RUST-BACKEND-EXPLAINED.md](RUST-BACKEND-EXPLAINED.md)

**Q: How do I deploy to Vercel?**
A: Follow [VERCEL-DEPLOYMENT.md](VERCEL-DEPLOYMENT.md)

**Q: How does Neurolink work?**
A: Read [NEUROLINK-USAGE.md](NEUROLINK-USAGE.md)

**Q: What's streaming and why is it cool?**
A: Check out [STREAMING.md](STREAMING.md)

**Q: Can I use Azure OpenAI?**
A: Yes! See [VERCEL-DEPLOYMENT.md](VERCEL-DEPLOYMENT.md#azure-openai-configuration)

## 📝 Contributing

Found an issue or want to contribute?
- Open an issue on GitHub
- Submit a pull request
- Check [PROJECT-STRUCTURE.md](PROJECT-STRUCTURE.md) for code organization

## 🔗 External Resources

- [Neurolink SDK](https://github.com/juspay/neurolink)
- [OpenTelemetry MCP Server](https://dev.to/shiftyp/supercharge-your-observability-how-otel-mcp-server-unlocks-ai-powered-insights-5dii)
- [Vercel Documentation](https://vercel.com/docs)
- [SvelteKit Documentation](https://kit.svelte.dev)

---

**Need help?** Start with [QUICKSTART.md](QUICKSTART.md) or [VERCEL-DEPLOYMENT.md](VERCEL-DEPLOYMENT.md)!
