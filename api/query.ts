import type { VercelRequest, VercelResponse } from '@vercel/node';
import { NeuroLink } from '@juspay/neurolink';
import { Client } from '@elastic/elasticsearch';

interface QueryRequest {
  query: string;
  source: 'elasticsearch' | 'openobserve';
  config: {
    url: string;
    username?: string;
    password?: string;
    organization?: string;
  };
}

interface SearchParams {
  query: string;
  time_range?: string;
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

async function parseQueryWithAI(query: string): Promise<SearchParams> {
  const neurolink = getNeurolink();

  const systemPrompt = `You are a log query parser. Convert natural language queries into structured search parameters.
Extract:
1. The main search query (keywords to search in logs)
2. Time range if mentioned (format: 1h, 24h, 7d, etc.)
3. Filters like service name, log level, etc.

Respond in JSON format:
{
  "query": "extracted keywords",
  "time_range": "1h" or null,
  "filters": {"field": "value"}
}

Examples:
- "Show errors in payment service last hour" -> {"query": "error", "time_range": "1h", "filters": {"service": "payment", "level": "ERROR"}}
- "Database timeouts" -> {"query": "database timeout", "time_range": null, "filters": {}}
- "Recent logs" -> {"query": "", "time_range": "1h", "filters": {}}`;

  try {
    const result = await neurolink.generate({
      input: {
        text: query,
        systemPrompt: systemPrompt,
      },
      temperature: 0.3,
      output: { format: 'json' },
      stream: false, // Keep this non-streaming for JSON parsing
    });

    let parsed: any;
    if (typeof result.content === 'string') {
      parsed = JSON.parse(result.content);
    } else {
      parsed = result.content;
    }

    return {
      query: parsed.query || query,
      time_range: parsed.time_range || null,
      filters: parsed.filters || {},
      size: 100,
    };
  } catch (error) {
    console.error('AI parsing error:', error);
    // Fallback to simple query
    return {
      query: query,
      time_range: '1h',
      filters: {},
      size: 100,
    };
  }
}

async function searchElasticsearch(
  config: QueryRequest['config'],
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

  // Add query string if present
  if (searchParams.query) {
    mustClauses.push({
      query_string: {
        query: searchParams.query,
        default_field: 'message',
      },
    });
  }

  // Add time range filter
  if (searchParams.time_range) {
    mustClauses.push({
      range: {
        '@timestamp': {
          gte: `now-${searchParams.time_range}`,
        },
      },
    });
  }

  // Add field filters
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

export default async function handler(
  req: VercelRequest,
  res: VercelResponse
) {
  // Only allow POST requests
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const requestBody = req.body as QueryRequest;
    
    if (!requestBody.query) {
      return res.status(400).json({ error: 'Query is required' });
    }

    if (!requestBody.source) {
      return res.status(400).json({ error: 'Source is required' });
    }

    if (!requestBody.config || !requestBody.config.url) {
      return res.status(400).json({ error: 'Config with URL is required' });
    }

    console.log('Processing query:', requestBody.query);

    // Parse query with AI
    const searchParams = await parseQueryWithAI(requestBody.query);
    console.log('Search params:', searchParams);

    // Search logs
    let logs: any[] = [];
    if (requestBody.source === 'elasticsearch') {
      logs = await searchElasticsearch(requestBody.config, searchParams);
    } else if (requestBody.source === 'openobserve') {
      // OpenObserve uses Elasticsearch-compatible API
      logs = await searchElasticsearch(requestBody.config, searchParams);
    } else {
      return res.status(400).json({ error: 'Unknown source' });
    }

    console.log('Found logs:', logs.length);

    return res.status(200).json({
      query: requestBody.query,
      results: logs,
      total: logs.length,
      summary: logs.length > 0 
        ? `Found ${logs.length} logs` 
        : 'No logs found matching your query',
    });
  } catch (error: any) {
    console.error('Query error:', error);
    return res.status(500).json({ 
      error: 'Internal server error',
      message: error.message 
    });
  }
}
