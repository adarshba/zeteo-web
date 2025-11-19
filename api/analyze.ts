import type { VercelRequest, VercelResponse } from '@vercel/node';
import { NeuroLink } from '@juspay/neurolink';

interface AnalyzeRequest {
  logs: any[];
  question: string;
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

export default async function handler(
  req: VercelRequest,
  res: VercelResponse
) {
  // Only allow POST requests
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const requestBody = req.body as AnalyzeRequest;
    
    if (!requestBody.logs || !Array.isArray(requestBody.logs)) {
      return res.status(400).json({ error: 'Logs array is required' });
    }

    if (!requestBody.question) {
      return res.status(400).json({ error: 'Question is required' });
    }

    console.log('Analyzing', requestBody.logs.length, 'logs');

    const neurolink = getNeurolink();

    const systemPrompt = 'You are a log analysis expert. Analyze the provided logs and answer the user\'s question with detailed insights, patterns, and actionable recommendations.';

    const userPrompt = `Question: ${requestBody.question}

Logs:
${JSON.stringify(requestBody.logs, null, 2)}

Provide a detailed analysis.`;

    // Set headers for streaming
    res.setHeader('Content-Type', 'text/event-stream');
    res.setHeader('Cache-Control', 'no-cache');
    res.setHeader('Connection', 'keep-alive');

    const stream = await neurolink.generate({
      input: {
        text: userPrompt,
        systemPrompt: systemPrompt,
      },
      temperature: 0.5,
      maxTokens: 1500,
      stream: true, // Enable streaming!
    });

    let fullContent = '';

    // Stream the response
    for await (const chunk of stream) {
      const content = typeof chunk.content === 'string' ? chunk.content : '';
      fullContent += content;
      
      // Send as Server-Sent Events
      res.write(`data: ${JSON.stringify({ content, done: false })}\n\n`);
    }

    // Send final message
    res.write(`data: ${JSON.stringify({ 
      content: fullContent, 
      done: true,
      log_count: requestBody.logs.length 
    })}\n\n`);
    
    res.end();
  } catch (error: any) {
    console.error('Analysis error:', error);
    res.write(`data: ${JSON.stringify({ 
      error: 'Internal server error',
      message: error.message,
      done: true
    })}\n\n`);
    res.end();
  }
}
