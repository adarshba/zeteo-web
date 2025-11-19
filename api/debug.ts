import type { VercelRequest, VercelResponse } from '@vercel/node';
import { NeuroLink } from '@juspay/neurolink';
import { Client } from '@elastic/elasticsearch';

interface DebugRequest {
  issue_description: string;
  context: {
    service?: string;
    time_range?: string;
    environment?: string;
    user_id?: string;
    request_id?: string;
  };
  config: {
    source: 'elasticsearch' | 'openobserve';
    url: string;
    username?: string;
    password?: string;
    organization?: string;
  };
}

interface SearchParams {
  query: string;
  time_range: string;
  filters: Record<string, string>;
  size: number;
}

// Initialize Neurolink with environment variables
const getNeurolink = () => {
  const config: any = {
    provider: (process.env.AI_PROVIDER as any) || 'openai',
    model: process.env.AI_MODEL || 'gpt-4o-mini',
  };

  // Azure OpenAI configuration
  if (process.env.AI_PROVIDER === 'azure-openai') {
    config.apiKey = process.env.AZURE_OPENAI_API_KEY;
    config.endpoint = process.env.AZURE_OPENAI_ENDPOINT;
    config.deployment = process.env.AZURE_OPENAI_DEPLOYMENT;
    config.apiVersion = process.env.AZURE_OPENAI_API_VERSION;
  } else {
    config.apiKey = process.env.OPENAI_API_KEY || process.env.AI_API_KEY;
  }

  return new NeuroLink(config);
};

async function createDebugQuery(
  issue: string,
  context: DebugRequest['context']
): Promise<SearchParams> {
  const neurolink = getNeurolink();

  const contextStr = `Service: ${context.service || 'N/A'}, Time: ${context.time_range || 'N/A'}, Env: ${context.environment || 'N/A'}, User: ${context.user_id || 'N/A'}, Request: ${context.request_id || 'N/A'}`;

  const systemPrompt = `You are a debugging expert. Based on the issue description and context, create an effective search query to find relevant logs.

Respond in JSON format:
{
  "query": "search keywords",
  "time_range": "suggested time range",
  "filters": {"field": "value"}
}`;

  const userPrompt = `Issue: ${issue}
Context: ${contextStr}

Create an optimal search query.`;

  try {
    const result = await neurolink.generate({
      input: {
        text: userPrompt,
        systemPrompt: systemPrompt,
      },
      temperature: 0.3,
      output: { format: 'json' },
      stream: false, // Keep non-streaming for JSON parsing
    });

    let parsed: any;
    if (typeof result.content === 'string') {
      parsed = JSON.parse(result.content);
    } else {
      parsed = result.content;
    }

    const filters = parsed.filters || {};
    if (context.service) {
      filters.service = context.service;
    }

    return {
      query: parsed.query || issue,
      time_range: parsed.time_range || context.time_range || '1h',
      filters,
      size: 200,
    };
  } catch (error) {
    console.error('Debug query creation error:', error);
    const filters: Record<string, string> = {};
    if (context.service) {
      filters.service = context.service;
    }
    
    return {
      query: issue,
      time_range: context.time_range || '1h',
      filters,
      size: 200,
    };
  }
}

async function searchElasticsearch(
  config: DebugRequest['config'],
  searchParams: SearchParams
) {
  const client = new Client({
    node: config.url,
    auth: config.username && config.password
      ? {
          username: config.username,
          password: config.password,
        }
      : undefined,
  });

  const mustClauses: any[] = [];

  if (searchParams.query) {
    mustClauses.push({
      query_string: {
        query: searchParams.query,
        default_field: 'message',
      },
    });
  }

  mustClauses.push({
    range: {
      '@timestamp': {
        gte: `now-${searchParams.time_range}`,
      },
    },
  });

  for (const [field, value] of Object.entries(searchParams.filters)) {
    mustClauses.push({
      term: { [field]: value },
    });
  }

  const response = await client.search({
    index: 'logs-*',
    body: {
      query: {
        bool: {
          must: mustClauses.length > 0 ? mustClauses : [{ match_all: {} }],
        },
      },
      size: searchParams.size,
      sort: [{ '@timestamp': 'desc' }],
    },
  });

  return response.hits.hits.map((hit: any) => ({
    timestamp: hit._source['@timestamp'],
    message: hit._source.message,
    level: hit._source.level,
    service: hit._source.service,
    ...hit._source,
  }));
}

async function debugWithLogs(
  issue: string,
  logs: any[],
  context: DebugRequest['context']
): Promise<any> {
  const neurolink = getNeurolink();

  const logsSummary = logs.length > 50
    ? `Found ${logs.length} logs. Here are the most recent:\n${JSON.stringify(logs.slice(0, 50), null, 2)}`
    : JSON.stringify(logs, null, 2);

  const contextStr = JSON.stringify(context, null, 2);

  const systemPrompt = `You are an expert SRE and debugging assistant. Analyze the issue, logs, and context to provide:
1. A detailed analysis of what's happening
2. The root cause (if identifiable)
3. Step-by-step recommendations to fix the issue

Respond in JSON format:
{
  "analysis": "detailed analysis",
  "root_cause": "identified root cause or null",
  "recommendations": ["step 1", "step 2", ...]
}`;

  const userPrompt = `Issue: ${issue}

Context:
${contextStr}

Logs:
${logsSummary}

Provide debugging analysis.`;

  try {
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

    let parsed: any;
    if (typeof result.content === 'string') {
      parsed = JSON.parse(result.content);
    } else {
      parsed = result.content;
    }

    return {
      analysis: parsed.analysis || result.content,
      root_cause: parsed.root_cause || null,
      recommendations: parsed.recommendations || [],
    };
  } catch (error) {
    console.error('Debug analysis error:', error);
    return {
      analysis: 'Error analyzing logs',
      root_cause: null,
      recommendations: [],
    };
  }
}

export default async function handler(
  req: VercelRequest,
  res: VercelResponse
) {
  // Only allow POST requests
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const requestBody = req.body as DebugRequest;
    
    if (!requestBody.issue_description) {
      return res.status(400).json({ error: 'Issue description is required' });
    }

    if (!requestBody.config || !requestBody.config.url) {
      return res.status(400).json({ error: 'Config with URL is required' });
    }

    console.log('Debug request:', requestBody.issue_description);

    // Create debug query
    const searchParams = await createDebugQuery(
      requestBody.issue_description,
      requestBody.context
    );
    console.log('Search params:', searchParams);

    // Search logs
    let logs: any[] = [];
    if (requestBody.config.source === 'elasticsearch' || requestBody.config.source === 'openobserve') {
      logs = await searchElasticsearch(requestBody.config, searchParams);
    } else {
      return res.status(400).json({ error: 'Unknown source' });
    }

    console.log('Found logs:', logs.length);

    // Analyze with AI
    const debugAnalysis = await debugWithLogs(
      requestBody.issue_description,
      logs,
      requestBody.context
    );

    return res.status(200).json({
      issue: requestBody.issue_description,
      analysis: debugAnalysis.analysis,
      root_cause: debugAnalysis.root_cause,
      recommendations: debugAnalysis.recommendations,
      relevant_logs: logs,
    });
  } catch (error: any) {
    console.error('Debug error:', error);
    return res.status(500).json({ 
      error: 'Internal server error',
      message: error.message 
    });
  }
}
