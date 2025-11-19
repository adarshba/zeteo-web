# Neurolink + otel-mcp-server Integration Guide

**The Ultimate AI-Powered Observability Platform**

This guide explains how to combine **Juspay Neurolink** (universal AI SDK with 12+ providers and 100+ models) with **otel-mcp-server** (OpenTelemetry MCP server for log exploration) to create an incredibly powerful observability platform.

## 🚀 What This Combination Unlocks

### Single Setup: Two Powerful Capabilities

1. **otel-mcp-server**: Direct AI access to your Elasticsearch/Kibana/OpenObserve logs
2. **Neurolink SDK**: Build custom AI applications powered by your observability data

Together, they enable:
- 🤖 **Natural language log queries** through AI assistants
- 🏗️ **Custom AI applications** that analyze your logs programmatically
- 🔄 **Multi-provider AI access** (switch between OpenAI, Anthropic, Google, etc.)
- 📊 **Automated log analysis** with intelligent pattern detection
- 🎯 **Cost optimization** by choosing the right AI model for each task
- 🔗 **Integration possibilities** limited only by your imagination

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Your Applications                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────┐         ┌──────────────────────┐     │
│  │ AI Assistant     │         │  Custom Neurolink    │     │
│  │ (Claude, etc.)   │         │  Applications        │     │
│  └────────┬─────────┘         └──────────┬───────────┘     │
│           │                              │                  │
│           │ MCP Protocol                 │ TypeScript SDK   │
│           │                              │                  │
│  ┌────────▼─────────┐         ┌─────────▼──────────┐       │
│  │ otel-mcp-server  │◄────────┤  Neurolink SDK     │       │
│  │ (MCP Server)     │   Uses  │  (Multi-Provider)  │       │
│  └────────┬─────────┘         └─────────┬──────────┘       │
│           │                              │                  │
│           │                              │                  │
└───────────┼──────────────────────────────┼──────────────────┘
            │                              │
            │    Both connect to:          │
            │                              │
     ┌──────▼──────────────────────────────▼──────┐
     │        Elasticsearch / OpenObserve         │
     │              (Your Logs)                   │
     └────────────────────────────────────────────┘
```

## 🎯 Powerful Use Cases

### 1. Interactive Debugging with AI Assistants

**Setup**: Configure otel-mcp-server in Claude Desktop/Cline
**Use**: Natural language queries to your logs

```
You: "Show me all errors in the payment service from the last hour"
AI: [Queries via otel-mcp-server] Here are 23 errors...

You: "What's the common pattern?"
AI: [Analyzes patterns] 85% are database timeout errors...

You: "Show me the first occurrence"
AI: [Retrieves specific log] It started at 14:32:15...
```

**Benefits**:
- Zero code required
- Conversational debugging
- Instant insights

---

### 2. Automated Log Analysis Dashboard

**Setup**: Build a TypeScript/Node.js app with Neurolink SDK
**Use**: Scheduled analysis of production logs

```typescript
import { NeuroLink } from "@juspay/neurolink";
import { Client } from "@elastic/elasticsearch";

const neurolink = new NeuroLink({
  provider: "google-ai", // Free tier available!
});

const es = new Client({ node: "http://localhost:9200" });

async function analyzeProductionLogs() {
  // Get recent errors from Elasticsearch
  const errors = await es.search({
    index: "logs-*",
    body: {
      query: {
        bool: {
          must: [
            { term: { level: "ERROR" } },
            { range: { "@timestamp": { gte: "now-1h" } } },
          ],
        },
      },
      size: 100,
    },
  });

  // Use AI to analyze patterns
  const analysis = await neurolink.generate({
    input: {
      text: `Analyze these production errors and identify:
1. Common patterns or root causes
2. Services most affected
3. Recommended actions
4. Severity assessment

Errors: ${JSON.stringify(errors.hits.hits, null, 2)}`,
    },
    temperature: 0.3, // More focused analysis
  });

  console.log("AI Analysis:", analysis.content);

  // Could send to Slack, email, or save to database
  return analysis;
}

// Run every hour
setInterval(analyzeProductionLogs, 3600000);
```

**Benefits**:
- Automated monitoring
- Proactive issue detection
- Scheduled reports

---

### 3. Smart Alert Triage System

**Setup**: Webhook receiver + Neurolink + otel-mcp-server
**Use**: AI-powered alert classification and response

```typescript
import express from "express";
import { NeuroLink } from "@juspay/neurolink";

const app = express();
const neurolink = new NeuroLink({
  provider: "anthropic", // Claude for complex reasoning
});

app.post("/webhook/alert", async (req, res) => {
  const alert = req.body;

  // Use AI to analyze the alert
  const triage = await neurolink.generate({
    input: {
      text: `Alert received:
Service: ${alert.service}
Message: ${alert.message}
Time: ${alert.timestamp}

Tasks:
1. Assess severity (Critical/High/Medium/Low)
2. Identify likely root cause
3. Suggest immediate actions
4. Determine if this needs immediate escalation
5. Check if similar patterns exist in recent logs`,
    },
    tools: [
      {
        name: "queryLogs",
        description: "Query Elasticsearch for related logs",
        parameters: {
          type: "object",
          properties: {
            query: { type: "string" },
            timeRange: { type: "string" },
          },
        },
        handler: async ({ query, timeRange }) => {
          // Query via Elasticsearch
          // ... implementation
        },
      },
    ],
  });

  // AI can automatically call queryLogs tool to get more context
  console.log("Triage Result:", triage.content);

  // Route based on severity
  if (triage.content.includes("Critical")) {
    await notifyOnCall(triage);
  }

  res.json({ status: "processed", analysis: triage.content });
});
```

**Benefits**:
- Intelligent alert routing
- Reduced alert fatigue
- Faster incident response

---

### 4. Developer Onboarding Assistant

**Setup**: VS Code extension or CLI tool with Neurolink
**Use**: Help new developers understand the codebase through logs

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink({
  provider: "openai",
  model: "gpt-4o", // Latest model for best results
});

async function explainService(serviceName: string) {
  const result = await neurolink.generate({
    input: {
      text: `I'm a new developer trying to understand the ${serviceName} service.
      
Please analyze recent logs and explain:
1. What does this service do?
2. What are its main dependencies?
3. What are the most common operations?
4. What are the typical error patterns?
5. How does it interact with other services?

Be specific and use actual examples from the logs.`,
    },
    tools: [
      {
        name: "searchLogs",
        description: "Search logs for the service",
        // ... implementation connects to Elasticsearch
      },
    ],
  });

  return result.content;
}

// Use in CLI
// $ node onboarding-assistant.js payment-service
```

**Benefits**:
- Faster onboarding
- Self-service documentation
- Real-world examples

---

### 5. Cost Optimization: Smart Model Selection

**Setup**: Neurolink with cost-aware routing
**Use**: Use cheap models for simple queries, powerful models for complex analysis

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink();

// Simple query - use cheap/free model
async function simpleLogQuery(query: string) {
  return await neurolink.generate({
    input: { text: query },
    provider: "google-ai", // Free tier
    model: "gemini-2.0-flash-exp", // Fast and free
  });
}

// Complex analysis - use powerful model
async function deepAnalysis(logs: any[]) {
  return await neurolink.generate({
    input: {
      text: `Perform deep root cause analysis...`,
      // ... complex prompt
    },
    provider: "anthropic",
    model: "claude-3-7-sonnet", // Most capable
  });
}

// Neurolink's auto cost optimization
async function smartQuery(query: string, complexity: "simple" | "complex") {
  return await neurolink.generate({
    input: { text: query },
    optimizeCost: complexity === "simple", // Automatic model selection
  });
}
```

**Benefits**:
- Minimize AI costs
- Use right tool for the job
- Scale economically

---

### 6. Multi-Tenant Log Analysis SaaS

**Setup**: Web application with Neurolink + Elasticsearch per tenant
**Use**: Offer AI-powered log analysis as a service

```typescript
import { NeuroLink } from "@juspay/neurolink";
import { Client } from "@elastic/elasticsearch";

class LogAnalysisSaaS {
  private neurolinks: Map<string, NeuroLink> = new Map();
  private esClients: Map<string, Client> = new Map();

  async analyzeForTenant(
    tenantId: string,
    query: string
  ): Promise<string> {
    // Get or create tenant-specific connections
    const neurolink = this.getNeurolink(tenantId);
    const es = this.getElasticsearch(tenantId);

    // Query logs
    const logs = await es.search({
      /* tenant-specific query */
    });

    // Analyze with AI
    const analysis = await neurolink.generate({
      input: {
        text: `${query}\n\nLogs: ${JSON.stringify(logs)}`,
      },
      conversationMemory: {
        enabled: true,
        sessionId: `tenant-${tenantId}`,
        store: "redis", // Shared Redis for all tenants
      },
    });

    return analysis.content;
  }

  private getNeurolink(tenantId: string): NeuroLink {
    if (!this.neurolinks.has(tenantId)) {
      this.neurolinks.set(
        tenantId,
        new NeuroLink({
          // Tenant-specific config
          conversationMemory: {
            enabled: true,
            sessionId: `tenant-${tenantId}`,
          },
        })
      );
    }
    return this.neurolinks.get(tenantId)!;
  }
}
```

**Benefits**:
- Build SaaS products
- Per-tenant customization
- Scalable architecture

---

### 7. Compliance & Audit Assistant

**Setup**: Neurolink with structured output + Elasticsearch
**Use**: Automated compliance checking and audit report generation

```typescript
import { NeuroLink } from "@juspay/neurolink";
import { z } from "zod";

const ComplianceReportSchema = z.object({
  summary: z.string(),
  violations: z.array(
    z.object({
      severity: z.enum(["critical", "high", "medium", "low"]),
      description: z.string(),
      affectedServices: z.array(z.string()),
      recommendation: z.string(),
    })
  ),
  complianceScore: z.number(),
  requiresAction: z.boolean(),
});

const neurolink = new NeuroLink({
  provider: "anthropic",
});

async function generateComplianceReport(timeRange: string) {
  const result = await neurolink.generate({
    input: {
      text: `Review logs from ${timeRange} for compliance violations:
      
1. Check for unauthorized access attempts
2. Verify audit trail completeness
3. Identify data access patterns
4. Check for encryption usage
5. Verify authentication methods

Generate a structured compliance report.`,
    },
    schema: ComplianceReportSchema,
    output: { format: "json" },
  });

  // Type-safe result
  const report = result.content as z.infer<typeof ComplianceReportSchema>;

  if (report.requiresAction) {
    await alertComplianceTeam(report);
  }

  return report;
}
```

**Benefits**:
- Automated compliance
- Structured reports
- Audit trail generation

---

### 8. Predictive Analytics & Anomaly Detection

**Setup**: Scheduled Neurolink analysis with historical data
**Use**: Predict future issues before they occur

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink({
  provider: "openai",
  model: "gpt-4o",
});

async function predictiveAnalysis() {
  // Get historical patterns (last 30 days)
  const historicalLogs = await getHistoricalData(30);

  // Get recent patterns (last 24 hours)
  const recentLogs = await getHistoricalData(1);

  const prediction = await neurolink.generate({
    input: {
      text: `Analyze these log patterns:

Historical baseline (30 days):
${JSON.stringify(historicalLogs.summary)}

Recent activity (24 hours):
${JSON.stringify(recentLogs.summary)}

Tasks:
1. Identify any anomalies in recent activity
2. Predict potential issues in next 24-48 hours
3. Assess risk levels
4. Recommend preventive actions
5. Highlight services that need attention`,
    },
    temperature: 0.4,
    enableEvaluation: true,
  });

  return {
    prediction: prediction.content,
    confidence: prediction.evaluation?.overallScore,
  };
}

// Run daily
setInterval(predictiveAnalysis, 86400000);
```

**Benefits**:
- Proactive monitoring
- Prevent outages
- Capacity planning

---

## 🔧 Setup Instructions

### Prerequisites

1. **Node.js** v18 or higher
2. **Elasticsearch** (via Kibana, OpenObserve, or direct)
3. **At least one AI provider** API key (OpenAI, Anthropic, Google, etc.)

### Step 1: Install Neurolink

```bash
npm install @juspay/neurolink
# or
pnpm add @juspay/neurolink
```

### Step 2: Configure Neurolink

Run the interactive setup wizard:

```bash
npx @juspay/neurolink setup
```

Or configure manually in your code:

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink({
  provider: "google-ai", // or "openai", "anthropic", etc.
  apiKey: process.env.GOOGLE_AI_API_KEY,
});
```

### Step 3: Configure otel-mcp-server

Add to your MCP client config (Claude Desktop, Cline, etc.):

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

### Step 4: Start Building!

Choose your approach:

**A) Interactive AI Assistant** (via otel-mcp-server)
- Use Claude Desktop/Cline with natural language
- Zero code required
- Perfect for: Debugging, exploration, ad-hoc queries

**B) Custom Applications** (via Neurolink SDK)
- Build TypeScript/Node.js apps
- Programmatic access to AI + logs
- Perfect for: Automation, SaaS products, custom workflows

**C) Hybrid Approach** (Both!)
- Use AI assistant for debugging
- Build custom apps for automation
- Best of both worlds!

---

## 💡 Integration Patterns

### Pattern 1: Neurolink Calls otel-mcp-server

Your Neurolink app can programmatically use otel-mcp-server:

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink({
  provider: "anthropic",
});

// Add otel-mcp-server as an external MCP server
await neurolink.addExternalMCPServer("otel", {
  command: "npx",
  args: ["-y", "otel-mcp-server"],
  transport: "stdio",
  env: {
    ELASTICSEARCH_URL: process.env.ELASTICSEARCH_URL!,
    ELASTICSEARCH_USERNAME: process.env.ELASTICSEARCH_USERNAME!,
    ELASTICSEARCH_PASSWORD: process.env.ELASTICSEARCH_PASSWORD!,
  },
});

// Now AI can use otel-mcp-server tools automatically!
const result = await neurolink.generate({
  input: {
    text: "Analyze errors in payment-service from last hour",
  },
});

console.log(result.content);
```

### Pattern 2: Direct Elasticsearch + Neurolink

For more control, query Elasticsearch directly:

```typescript
import { NeuroLink } from "@juspay/neurolink";
import { Client } from "@elastic/elasticsearch";

const neurolink = new NeuroLink();
const es = new Client({ node: "http://localhost:9200" });

// Query logs directly
const logs = await es.search({
  index: "logs-*",
  body: {
    query: { match: { level: "ERROR" } },
  },
});

// Analyze with AI
const analysis = await neurolink.generate({
  input: {
    text: `Analyze these errors: ${JSON.stringify(logs.hits.hits)}`,
  },
});
```

### Pattern 3: Streaming Analysis

Real-time log analysis with streaming:

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink();

async function streamingAnalysis(logs: any[]) {
  const stream = await neurolink.generate({
    input: {
      text: `Analyze these logs and provide insights:\n${JSON.stringify(logs)}`,
    },
    stream: true,
  });

  for await (const chunk of stream) {
    process.stdout.write(chunk.content);
  }
}
```

---

## 🎨 Example Applications

### Example 1: CLI Log Analyzer

```bash
# Create a new project
mkdir log-analyzer && cd log-analyzer
npm init -y
npm install @juspay/neurolink @elastic/elasticsearch commander

# Create index.ts
```

```typescript
#!/usr/bin/env node
import { Command } from "commander";
import { NeuroLink } from "@juspay/neurolink";
import { Client } from "@elastic/elasticsearch";

const program = new Command();
const neurolink = new NeuroLink();
const es = new Client({ node: process.env.ELASTICSEARCH_URL });

program
  .name("log-analyzer")
  .description("AI-powered log analysis CLI")
  .version("1.0.0");

program
  .command("analyze")
  .description("Analyze recent logs")
  .option("-s, --service <name>", "Service name")
  .option("-t, --time <range>", "Time range", "1h")
  .action(async (options) => {
    console.log("Fetching logs...");

    const logs = await es.search({
      index: "logs-*",
      body: {
        query: {
          bool: {
            must: [
              options.service && { term: { service: options.service } },
              { range: { "@timestamp": { gte: `now-${options.time}` } } },
            ].filter(Boolean),
          },
        },
      },
    });

    console.log("Analyzing with AI...");

    const analysis = await neurolink.generate({
      input: {
        text: `Analyze these logs:\n${JSON.stringify(logs.hits.hits, null, 2)}`,
      },
    });

    console.log("\nAnalysis:");
    console.log(analysis.content);
  });

program.parse();
```

Usage:
```bash
npx tsx index.ts analyze --service payment-service --time 2h
```

### Example 2: Slack Bot

```typescript
import { App } from "@slack/bolt";
import { NeuroLink } from "@juspay/neurolink";

const app = new App({
  token: process.env.SLACK_BOT_TOKEN,
  signingSecret: process.env.SLACK_SIGNING_SECRET,
});

const neurolink = new NeuroLink();

app.message(/logs/i, async ({ message, say }) => {
  const query = message.text.replace(/logs/i, "").trim();

  const analysis = await neurolink.generate({
    input: {
      text: `Query: ${query}\nSearch logs and provide insights.`,
    },
  });

  await say(analysis.content);
});

await app.start();
console.log("⚡️ Log analysis Slack bot is running!");
```

---

## 🚦 Best Practices

### 1. Choose the Right Provider for the Task

```typescript
// Simple queries - use free/cheap models
const simpleQuery = await neurolink.generate({
  input: { text: "Show recent errors" },
  provider: "google-ai", // Free tier
});

// Complex analysis - use powerful models
const deepAnalysis = await neurolink.generate({
  input: { text: "Perform root cause analysis..." },
  provider: "anthropic",
  model: "claude-3-7-sonnet",
});

// Let Neurolink choose automatically
const autoQuery = await neurolink.generate({
  input: { text: "Analyze logs" },
  optimizeCost: true, // Automatically picks cheapest suitable model
});
```

### 2. Use Conversation Memory for Context

```typescript
const neurolink = new NeuroLink({
  conversationMemory: {
    enabled: true,
    sessionId: "debug-session-123",
    store: "redis", // Persist across restarts
  },
});

// First query
await neurolink.generate({
  input: { text: "Show payment service errors" },
});

// Follow-up - AI remembers context!
await neurolink.generate({
  input: { text: "What's the pattern?" },
});
```

### 3. Structure Output for Automation

```typescript
import { z } from "zod";

const ErrorSummarySchema = z.object({
  totalErrors: z.number(),
  criticalErrors: z.number(),
  services: z.array(z.string()),
  recommendedActions: z.array(z.string()),
});

const result = await neurolink.generate({
  input: { text: "Summarize errors from last hour" },
  schema: ErrorSummarySchema,
  output: { format: "json" },
});

// Type-safe!
const summary = result.content as z.infer<typeof ErrorSummarySchema>;
console.log(`Total errors: ${summary.totalErrors}`);
```

### 4. Implement Error Handling & Fallbacks

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink({
  providers: [
    { name: "google-ai", priority: 1 },
    { name: "openai", priority: 2 },
    { name: "anthropic", priority: 3 },
  ],
  fallbackOnError: true, // Automatic failover
});

try {
  const result = await neurolink.generate({
    input: { text: "Analyze logs" },
  });
} catch (error) {
  console.error("All providers failed:", error);
  // Fallback to basic analysis without AI
}
```

### 5. Monitor Costs & Usage

```typescript
import { NeuroLink } from "@juspay/neurolink";

const neurolink = new NeuroLink({
  enableAnalytics: true,
});

const result = await neurolink.generate({
  input: { text: "Analyze logs" },
  enableAnalytics: true,
});

console.log("Token usage:", result.metadata?.usage);
console.log("Cost estimate:", result.metadata?.cost);
```

---

## 📊 Comparison: otel-mcp-server vs Neurolink Direct

| Aspect             | otel-mcp-server               | Neurolink Direct          | Best For                  |
| ------------------ | ----------------------------- | ------------------------- | ------------------------- |
| **Setup**          | Configure MCP client          | npm install + code        | otel: Quick start         |
| **Usage**          | Natural language in AI client | Programmatic TypeScript   | otel: Ad-hoc queries      |
| **Customization**  | Limited to MCP capabilities   | Full control              | Neurolink: Custom apps    |
| **Automation**     | Manual queries                | Fully automated           | Neurolink: Automation     |
| **Cost Control**   | Depends on AI client          | Fine-grained control      | Neurolink: Cost sensitive |
| **Integration**    | Standalone                    | Embed in apps             | Neurolink: SaaS products  |
| **Learning Curve** | Zero code                     | TypeScript knowledge      | otel: Beginners           |
| **Scalability**    | Single user                   | Multi-tenant, production  | Neurolink: Scale          |

**Recommendation**: Use **both**!
- otel-mcp-server for interactive debugging
- Neurolink SDK for automation and custom applications

---

## 🔗 Resources

### Documentation
- [Neurolink Documentation](https://github.com/juspay/neurolink)
- [otel-mcp-server Setup](otel-mcp-server-setup.md)
- [Query Examples](query-examples.md)
- [Configuration Examples](config-examples.md)

### Example Code
- See [examples/](../examples/) directory for complete applications
- Check out the Juspay/Lumos project for real-world AI debugging

### Community
- [Neurolink GitHub Issues](https://github.com/juspay/neurolink/issues)
- [Model Context Protocol Docs](https://modelcontextprotocol.io/)

---

## 🎯 Next Steps

1. **Try otel-mcp-server** - Get natural language log querying working
2. **Install Neurolink** - Set up the SDK in a test project
3. **Build something simple** - Start with a basic log analyzer
4. **Experiment** - Try different providers and models
5. **Scale up** - Build production applications

The combination of Neurolink SDK and otel-mcp-server gives you the most powerful and flexible AI-powered observability platform available. Start small, iterate fast, and build amazing things! 🚀
