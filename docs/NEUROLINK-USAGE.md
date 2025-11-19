# How We Use Neurolink in Logs Explorer

## Overview

The Logs Explorer application uses **Juspay Neurolink SDK** as the AI backbone for all intelligent operations. Neurolink replaces the previous direct OpenAI integration, providing multi-provider support and advanced AI capabilities.

## Architecture

```
User Query (Natural Language)
        ↓
Frontend (Svelte)
        ↓
API Endpoint (Vercel Serverless Function)
        ↓
Neurolink SDK ← Configured with provider (OpenAI/Anthropic/Google AI/etc.)
        ↓
AI Model (gpt-4o-mini, claude-3-sonnet, gemini-pro, etc.)
        ↓
Structured Response
        ↓
Elasticsearch/OpenObserve Query
        ↓
Log Results
```

## Neurolink Integration Points

### 1. **Query Parsing** (`api/query.ts`)

**What it does:** Converts natural language queries into structured Elasticsearch queries.

**Implementation:**
```typescript
import { NeuroLink } from '@juspay/neurolink';

const getNeurolink = () => {
  return new NeuroLink({
    provider: process.env.AI_PROVIDER || 'openai',    // openai, anthropic, google-ai, etc.
    apiKey: process.env.OPENAI_API_KEY || process.env.AI_API_KEY,
    model: process.env.AI_MODEL || 'gpt-4o-mini',
  });
};

async function parseQueryWithAI(query: string): Promise<SearchParams> {
  const neurolink = getNeurolink();
  
  const result = await neurolink.generate({
    input: {
      text: query,
      systemPrompt: "You are a log query parser...",
    },
    temperature: 0.3,           // Low temperature for consistent parsing
    output: { format: 'json' }, // Structured JSON output
  });
  
  return parsedSearchParams;
}
```

**Example:**
- **User Input:** "Show errors in payment service last hour"
- **Neurolink Output:** 
  ```json
  {
    "query": "error",
    "time_range": "1h",
    "filters": {
      "service": "payment",
      "level": "ERROR"
    }
  }
  ```
- **Result:** Converts to Elasticsearch query that searches for errors in payment service from the last hour

**Benefits of Neurolink:**
- Can switch between OpenAI, Anthropic, Google AI without code changes
- Structured JSON output ensures consistency
- Lower temperature (0.3) for reliable parsing

---

### 2. **Log Analysis** (`api/analyze.ts`)

**What it does:** Analyzes log entries and answers questions about patterns, issues, and insights.

**Implementation:**
```typescript
const neurolink = getNeurolink();

const result = await neurolink.generate({
  input: {
    text: `Question: ${question}\n\nLogs:\n${JSON.stringify(logs)}`,
    systemPrompt: "You are a log analysis expert...",
  },
  temperature: 0.5,      // Medium temperature for balanced analysis
  maxTokens: 1500,       // Limit response length
});

return result.content; // Natural language analysis
```

**Example:**
- **User Input:** "What's causing these errors?"
- **Logs:** 50 error log entries
- **Neurolink Output:** 
  ```
  Analysis: The errors are caused by database connection timeouts. 
  85% of errors show "connection refused" at 2:30 PM. This correlates 
  with a database maintenance window. Recommend:
  1. Implement retry logic with exponential backoff
  2. Add circuit breaker pattern
  3. Schedule maintenance during off-peak hours
  ```

**Benefits of Neurolink:**
- More powerful models (like Claude) excel at analysis
- Can use cost-effective models for simple queries
- Consistent analysis across different AI providers

---

### 3. **Debug Assistance** (`api/debug.ts`)

**What it does:** AI-powered debugging that creates optimal search queries and analyzes results to find root causes.

**Implementation:**

**Step 1: Create Debug Query**
```typescript
const neurolink = getNeurolink();

// AI creates the best search query for the issue
const result = await neurolink.generate({
  input: {
    text: `Issue: ${issue}\nContext: ${context}`,
    systemPrompt: "You are a debugging expert...",
  },
  temperature: 0.3,
  output: { format: 'json' },
});

// Returns: { query: "...", time_range: "...", filters: {...} }
```

**Step 2: Analyze Retrieved Logs**
```typescript
// After fetching logs, analyze them
const analysis = await neurolink.generate({
  input: {
    text: `Issue: ${issue}\n\nLogs:\n${logs}`,
    systemPrompt: "You are an expert SRE...",
  },
  temperature: 0.4,
  maxTokens: 2000,
  output: { format: 'json' },
});

// Returns: {
//   analysis: "detailed analysis",
//   root_cause: "identified cause",
//   recommendations: ["step 1", "step 2", ...]
// }
```

**Example Flow:**
1. **User:** "Payment API is returning 500 errors"
2. **Neurolink (Query Creation):** Creates optimal search: `query: "500 error", filters: {service: "payment-api"}, time_range: "2h"`
3. **System:** Fetches 127 relevant logs from Elasticsearch
4. **Neurolink (Analysis):** Analyzes logs and returns:
   ```json
   {
     "analysis": "The payment API is failing due to Redis connection pool exhaustion...",
     "root_cause": "Redis max connections (100) exceeded during peak traffic",
     "recommendations": [
       "Increase Redis max connections to 200",
       "Implement connection pooling with better limits",
       "Add Redis connection monitoring alerts"
     ]
   }
   ```

**Benefits of Neurolink:**
- Two-stage AI process: query creation + analysis
- Structured output for easy UI display
- Can use different models for different stages (cheap for queries, powerful for analysis)

---

## Configuration & Provider Selection

### Environment Variables

```bash
# Choose your AI provider
AI_PROVIDER=openai          # or anthropic, google-ai, cohere, etc.

# API Key
OPENAI_API_KEY=sk-...       # For OpenAI
AI_API_KEY=...              # For other providers

# Model selection
AI_MODEL=gpt-4o-mini        # or claude-3-sonnet, gemini-pro, etc.
```

### Supported Providers (via Neurolink)

1. **OpenAI** - GPT-4o, GPT-4o-mini, GPT-4, GPT-3.5-turbo
2. **Anthropic** - Claude 3.7 Sonnet, Claude 3.5 Sonnet, Claude 3 Opus
3. **Google AI** - Gemini 2.0 Flash, Gemini Pro, Gemini Pro Vision
4. **Cohere** - Command R+, Command R
5. **Mistral AI** - Mistral Large, Mistral Medium
6. **Groq** - Llama 3, Mixtral
7. **Perplexity** - Sonar models
8. **Together AI** - Various open source models
9. **OpenRouter** - Access to 100+ models
10. **Azure OpenAI** - Enterprise OpenAI deployment
11. **AWS Bedrock** - Claude, Llama, etc. on AWS
12. **Vertex AI** - Google Cloud AI models

### Switching Providers

Just change environment variables:

```bash
# Use Claude (great for analysis)
AI_PROVIDER=anthropic
AI_API_KEY=sk-ant-...
AI_MODEL=claude-3-sonnet-20240229

# Use Google AI (free tier available!)
AI_PROVIDER=google-ai
AI_API_KEY=...
AI_MODEL=gemini-2.0-flash-exp

# Use OpenAI (default)
AI_PROVIDER=openai
OPENAI_API_KEY=sk-...
AI_MODEL=gpt-4o-mini
```

No code changes needed!

---

## Key Advantages of Using Neurolink

### 1. **Multi-Provider Support**
- Not locked into one AI provider
- Can switch providers based on cost, performance, or availability
- If one provider is down, switch to another instantly

### 2. **Cost Optimization**
Different operations can use different models:
- **Query Parsing**: Use cheap models (gpt-4o-mini, gemini-flash) - simple task
- **Deep Analysis**: Use powerful models (claude-3-sonnet, gpt-4) - complex reasoning
- **Bulk Operations**: Use free tier models (google-ai) - high volume

```typescript
// Example: Smart cost optimization
const cheapNeurolink = new NeuroLink({
  provider: 'google-ai',
  model: 'gemini-2.0-flash-exp', // Free tier
});

const powerfulNeurolink = new NeuroLink({
  provider: 'anthropic',
  model: 'claude-3-7-sonnet', // Best for complex analysis
});

// Use cheap for parsing
const params = await cheapNeurolink.generate({ input: query });

// Use powerful for analysis
const analysis = await powerfulNeurolink.generate({ input: complexIssue });
```

### 3. **Consistent API**
Same code works across all providers:
```typescript
// Works with any provider!
const result = await neurolink.generate({
  input: { text: "..." },
  temperature: 0.3,
  maxTokens: 1000,
});
```

### 4. **Advanced Features**
- **Structured Output**: JSON mode for consistent parsing
- **Streaming**: Real-time responses (can be added)
- **Conversation Memory**: Multi-turn conversations (can be added)
- **Function Calling**: Tools and actions (can be added)
- **Cost Tracking**: Built-in usage metrics
- **Provider Fallback**: Automatic failover

### 5. **Production Ready**
- **Type Safety**: Full TypeScript support
- **Error Handling**: Graceful degradation
- **Retry Logic**: Built-in retry mechanisms
- **Rate Limiting**: Automatic rate limit handling

---

## Code Comparison: Before vs After

### Before (Direct OpenAI)
```typescript
import { Client } from 'async-openai';

const client = new Client({ apiKey: process.env.OPENAI_API_KEY });

const response = await client.chat.completions.create({
  model: 'gpt-4o-mini',
  messages: [
    { role: 'system', content: systemPrompt },
    { role: 'user', content: userPrompt }
  ],
  temperature: 0.3,
});

const content = response.choices[0].message.content;
```

**Issues:**
- Locked into OpenAI only
- Manual message formatting
- No structured output
- Custom error handling needed
- Provider-specific code

### After (with Neurolink)
```typescript
import { NeuroLink } from '@juspay/neurolink';

const neurolink = new NeuroLink({
  provider: process.env.AI_PROVIDER || 'openai', // Any provider!
  apiKey: process.env.OPENAI_API_KEY,
  model: process.env.AI_MODEL || 'gpt-4o-mini',
});

const result = await neurolink.generate({
  input: {
    text: userPrompt,
    systemPrompt: systemPrompt,
  },
  temperature: 0.3,
  output: { format: 'json' }, // Structured output
});

const content = result.content;
```

**Benefits:**
- Works with 12+ providers
- Cleaner API
- Built-in structured output
- Automatic error handling
- Provider-agnostic code

---

## Example Use Cases

### Use Case 1: Cost Optimization

```typescript
// Morning routine: Use free Google AI
const morningNeurolink = new NeuroLink({
  provider: 'google-ai',
  model: 'gemini-2.0-flash-exp', // FREE!
});

// Critical incident: Use powerful Claude
const incidentNeurolink = new NeuroLink({
  provider: 'anthropic',
  model: 'claude-3-7-sonnet', // Most capable
});
```

**Result:** Save ~90% on AI costs for routine queries while using best models for critical issues.

### Use Case 2: Provider Fallback

```typescript
const providers = ['openai', 'anthropic', 'google-ai'];

for (const provider of providers) {
  try {
    const neurolink = new NeuroLink({ provider });
    return await neurolink.generate({ input: query });
  } catch (error) {
    console.log(`${provider} failed, trying next...`);
  }
}
```

**Result:** 99.9% uptime even if one provider is down.

### Use Case 3: A/B Testing Models

```typescript
// Test which model is better for your logs
const models = [
  { provider: 'openai', model: 'gpt-4o-mini' },
  { provider: 'anthropic', model: 'claude-3-haiku' },
  { provider: 'google-ai', model: 'gemini-flash' },
];

// Compare accuracy, cost, speed
const results = await Promise.all(
  models.map(config => 
    new NeuroLink(config).generate({ input: query })
  )
);
```

**Result:** Data-driven model selection for your specific use case.

---

## Future Enhancements (Possible with Neurolink)

### 1. Streaming Responses
```typescript
const stream = await neurolink.generate({
  input: { text: query },
  stream: true,
});

for await (const chunk of stream) {
  process.stdout.write(chunk.content);
}
```

### 2. Conversation Memory
```typescript
const neurolink = new NeuroLink({
  conversationMemory: {
    enabled: true,
    sessionId: 'user-123',
  },
});

// First query
await neurolink.generate({ input: { text: "Show errors" }});

// Follow-up - AI remembers context!
await neurolink.generate({ input: { text: "What's the pattern?" }});
```

### 3. Function Calling
```typescript
const result = await neurolink.generate({
  input: { text: "Find related logs" },
  tools: [
    {
      name: 'searchLogs',
      description: 'Search Elasticsearch',
      handler: async (params) => {
        // Custom log search logic
      },
    },
  ],
});
```

---

## Summary

**Neurolink is used in 3 critical places:**

1. **Query Parsing** (`/api/query`) - Converts natural language → Elasticsearch DSL
2. **Log Analysis** (`/api/analyze`) - Analyzes logs and provides insights
3. **Debug Assistance** (`/api/debug`) - Creates queries + analyzes results for root cause

**Why Neurolink over direct OpenAI?**

✅ **Multi-provider support** - Not locked to OpenAI  
✅ **Cost optimization** - Use cheap/free models when possible  
✅ **Better reliability** - Provider fallback  
✅ **Cleaner code** - Unified API  
✅ **Production ready** - Type safety, error handling  
✅ **Future proof** - Easy to add new features  

**Configuration is simple:**
- Set `AI_PROVIDER` environment variable
- Set API key
- Set model name
- Done! Code works with any provider

This makes the Logs Explorer truly **vendor-agnostic** and **future-proof**! 🚀
