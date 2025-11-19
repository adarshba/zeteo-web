# Streaming with Neurolink - Much Cooler! 🔥

## Why Streaming is Much Cooler

Traditional API calls make users wait for the entire response before showing anything. With streaming, users see the AI "thinking" in real-time, character by character, just like ChatGPT!

### Before (Without Streaming)
```
User: "Analyze these 100 logs"
[5 seconds of blank screen]
AI: [Complete response appears all at once]
```

### After (With Streaming) ✨
```
User: "Analyze these 100 logs"
AI: "Analysis: The errors are caused by...
     [text appears in real-time]
     database connection timeouts...
     [more text appears]
     85% of errors show..."
     [continues smoothly]
```

**Result:** Users see progress immediately, feels much more responsive and interactive!

## How We Implemented Streaming

### 1. Analyze Endpoint (`/api/analyze`) - Streaming Enabled ✨

This endpoint now streams the analysis response in real-time:

```typescript
// Set headers for Server-Sent Events (SSE)
res.setHeader('Content-Type', 'text/event-stream');
res.setHeader('Cache-Control', 'no-cache');
res.setHeader('Connection', 'keep-alive');

// Enable streaming in Neurolink
const stream = await neurolink.generate({
  input: {
    text: userPrompt,
    systemPrompt: systemPrompt,
  },
  temperature: 0.5,
  maxTokens: 1500,
  stream: true, // 🔥 The magic happens here!
});

let fullContent = '';

// Stream each chunk as it arrives
for await (const chunk of stream) {
  const content = typeof chunk.content === 'string' ? chunk.content : '';
  fullContent += content;
  
  // Send chunk to frontend immediately
  res.write(`data: ${JSON.stringify({ content, done: false })}\n\n`);
}

// Send final message
res.write(`data: ${JSON.stringify({ 
  content: fullContent, 
  done: true,
  log_count: requestBody.logs.length 
})}\n\n`);

res.end();
```

**Why it's cool:**
- ✨ Real-time feedback - users see results immediately
- 🚀 Perceived performance - feels much faster
- 💬 ChatGPT-like experience - text appears naturally
- 🎯 Better UX - no blank loading screens

### 2. Query Endpoint (`/api/query`) - Non-Streaming

This endpoint parses queries into JSON, so we keep it non-streaming for reliability:

```typescript
const result = await neurolink.generate({
  input: {
    text: query,
    systemPrompt: systemPrompt,
  },
  temperature: 0.3,
  output: { format: 'json' },
  stream: false, // Keep non-streaming for JSON parsing
});
```

**Why non-streaming here:**
- Need complete JSON response before parsing
- Quick response anyway (just parsing, not long analysis)
- Structured output requires full response

### 3. Debug Endpoint (`/api/debug`) - Non-Streaming

Similar to query endpoint, this returns structured JSON:

```typescript
const result = await neurolink.generate({
  input: {
    text: userPrompt,
    systemPrompt: systemPrompt,
  },
  temperature: 0.4,
  maxTokens: 2000,
  output: { format: 'json' },
  stream: false, // Keep non-streaming for JSON parsing
});
```

**Why non-streaming here:**
- Returns structured `{ analysis, root_cause, recommendations }`
- Needs complete JSON before parsing
- Could stream in future with JSON streaming parser

## Frontend Integration

### How to Consume Streaming API

```typescript
async function analyzeLogsWithStreaming(logs: any[], question: string) {
  const response = await fetch('/api/analyze', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ logs, question }),
  });

  // Create reader for streaming response
  const reader = response.body.getReader();
  const decoder = new TextDecoder();

  let buffer = '';
  
  while (true) {
    const { done, value } = await reader.read();
    
    if (done) break;
    
    // Decode chunk
    buffer += decoder.decode(value, { stream: true });
    
    // Split by newlines (SSE format)
    const lines = buffer.split('\n');
    buffer = lines.pop() || ''; // Keep incomplete line in buffer
    
    for (const line of lines) {
      if (line.startsWith('data: ')) {
        const data = JSON.parse(line.slice(6));
        
        if (data.error) {
          console.error('Error:', data.error);
          break;
        }
        
        // Update UI in real-time!
        if (!data.done) {
          updateAnalysisDisplay(data.content); // Show partial content
        } else {
          finalizeAnalysis(data.content); // Show final result
        }
      }
    }
  }
}
```

### Svelte Component Example

```svelte
<script lang="ts">
  let analysis = '';
  let isStreaming = false;
  
  async function analyzeWithStreaming(logs: any[], question: string) {
    analysis = '';
    isStreaming = true;
    
    const response = await fetch('/api/analyze', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ logs, question }),
    });
    
    const reader = response.body!.getReader();
    const decoder = new TextDecoder();
    
    let buffer = '';
    
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      
      buffer += decoder.decode(value, { stream: true });
      const lines = buffer.split('\n');
      buffer = lines.pop() || '';
      
      for (const line of lines) {
        if (line.startsWith('data: ')) {
          const data = JSON.parse(line.slice(6));
          
          if (!data.done) {
            analysis = data.content; // Real-time update!
          }
        }
      }
    }
    
    isStreaming = false;
  }
</script>

<div class="analysis-container">
  {#if isStreaming}
    <div class="streaming-indicator">
      🔄 AI is analyzing...
    </div>
  {/if}
  
  <div class="analysis-content">
    {analysis}
    {#if isStreaming}
      <span class="cursor">▊</span>
    {/if}
  </div>
</div>

<style>
  .cursor {
    animation: blink 1s infinite;
  }
  
  @keyframes blink {
    0%, 50% { opacity: 1; }
    51%, 100% { opacity: 0; }
  }
</style>
```

## Benefits of Streaming

### 1. Better User Experience
- ✅ Instant feedback - see AI working
- ✅ No blank loading screens
- ✅ Progress indicator built-in
- ✅ Feels more interactive

### 2. Perceived Performance
- Users perceive it as faster even if total time is same
- Can start reading before completion
- More engaging than waiting

### 3. Scalability
- Lower memory usage (process chunks)
- Can handle longer responses
- Better for mobile/slow connections

### 4. Error Handling
- See partial results even if connection drops
- Can retry from last chunk
- Better debugging

## Server-Sent Events (SSE) Format

We use SSE format for streaming:

```
data: {"content": "The", "done": false}

data: {"content": " errors", "done": false}

data: {"content": " are caused", "done": false}

data: {"content": "...", "done": true, "log_count": 100}

```

**Why SSE?**
- ✅ Built into browsers (EventSource API)
- ✅ Simple protocol
- ✅ Auto reconnection
- ✅ Works through proxies/firewalls
- ✅ Better than WebSockets for unidirectional streaming

## Performance Comparison

### Non-Streaming
```
Request sent → [Wait 5 seconds] → Complete response → Render
Time to first content: 5000ms
Total time: 5000ms
User sees: Loading spinner for 5 seconds
```

### Streaming
```
Request sent → [100ms] → First chunk → [50ms] → Second chunk → ...
Time to first content: 100ms
Total time: 5000ms (same!)
User sees: Text appearing from 100ms onwards
```

**Result:** Same total time, but feels 50x faster! 🚀

## Future Enhancements

### 1. Streaming for Debug Endpoint
Could stream analysis while parsing JSON incrementally:
```typescript
// Stream analysis text
// Parse JSON at the end
```

### 2. Progress Indicators
Show percentage complete:
```typescript
data: {"content": "...", "progress": 45, "done": false}
```

### 3. Cancellation
Allow users to cancel long-running streams:
```typescript
const abortController = new AbortController();
fetch('/api/analyze', { signal: abortController.signal });
// Later: abortController.abort();
```

### 4. Retry Logic
Automatic retry on connection drop:
```typescript
let lastChunkId = 0;
// Resume from lastChunkId if connection drops
```

## Neurolink Streaming Features

Neurolink makes streaming easy:

```typescript
// Simple streaming
const stream = await neurolink.generate({
  input: { text: "..." },
  stream: true, // Just set this!
});

for await (const chunk of stream) {
  console.log(chunk.content); // Process each chunk
}
```

**Supported providers:**
- ✅ OpenAI (GPT-4, GPT-4o, GPT-3.5)
- ✅ Anthropic (Claude 3.5, 3.7)
- ✅ Google AI (Gemini)
- ✅ Azure OpenAI
- ✅ Cohere
- ✅ Most other providers

## Azure OpenAI with Streaming

Your gpt-4o-automatic deployment works perfectly with streaming:

```typescript
const neurolink = new NeuroLink({
  provider: 'azure-openai',
  apiKey: process.env.AZURE_OPENAI_API_KEY,
  endpoint: process.env.AZURE_OPENAI_ENDPOINT,
  deployment: 'gpt-4o-automatic',
  apiVersion: '2024-02-15-preview',
});

// Streaming works out of the box!
const stream = await neurolink.generate({
  input: { text: "..." },
  stream: true,
});
```

**No configuration needed - just works!** ✨

## Summary

### What's Streaming
- ✅ `/api/analyze` - Full streaming for better UX
- ❌ `/api/query` - Non-streaming (needs JSON)
- ❌ `/api/debug` - Non-streaming (needs JSON)

### Why It's Cool
- 🔥 Real-time responses
- 💬 ChatGPT-like experience  
- 🚀 Feels much faster
- ✨ Better user engagement
- 🎯 Professional UX

### Easy to Use
- Simple API: `stream: true`
- Works with all providers
- SSE format (standard)
- Easy frontend integration

**Streaming makes the app feel alive and responsive - much cooler than waiting for complete responses!** 🎉
