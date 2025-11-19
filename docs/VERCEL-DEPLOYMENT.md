# Vercel Deployment Guide

This guide explains how to deploy the Logs Explorer application to Vercel.

## Architecture

The application has been refactored for Vercel deployment:

- **Frontend**: SvelteKit application (static build)
- **API**: Vercel Serverless Functions (TypeScript/Node.js)
- **AI**: Neurolink SDK for multi-provider AI operations

## Prerequisites

1. **Vercel Account**: Sign up at [vercel.com](https://vercel.com)
2. **GitHub Repository**: Fork or clone this repository
3. **AI API Key**: Get an API key from:
   - OpenAI: [platform.openai.com](https://platform.openai.com)
   - Anthropic: [console.anthropic.com](https://console.anthropic.com)
   - Google AI: [ai.google.dev](https://ai.google.dev)
   - Or any other provider supported by Neurolink

## Quick Deploy to Vercel

### Option 1: One-Click Deploy

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https%3A%2F%2Fgithub.com%2Fadarshba%2Flogs-explorer)

1. Click the "Deploy with Vercel" button above
2. Connect your GitHub account
3. Configure environment variables (see below)
4. Click "Deploy"

### Option 2: Vercel CLI

```bash
# Install Vercel CLI
npm i -g vercel

# Navigate to project
cd logs-explorer

# Login to Vercel
vercel login

# Deploy
vercel
```

### Option 3: GitHub Integration

1. Go to [vercel.com/dashboard](https://vercel.com/dashboard)
2. Click "New Project"
3. Import your GitHub repository
4. Configure settings (see below)
5. Click "Deploy"

## Environment Variables

Configure these in Vercel Dashboard → Settings → Environment Variables:

### Required Variables

```bash
# AI Provider (openai, anthropic, google-ai, etc.)
AI_PROVIDER=openai

# OpenAI API Key (if using OpenAI)
OPENAI_API_KEY=sk-...

# Or use generic AI_API_KEY for other providers
AI_API_KEY=your_api_key

# AI Model (e.g., gpt-4o-mini, claude-3-sonnet, gemini-pro)
AI_MODEL=gpt-4o-mini
```

### Optional Variables

These can be configured in the UI, but you can set defaults:

```bash
# Elasticsearch Configuration (optional)
ELASTICSEARCH_URL=http://localhost:9200
ELASTICSEARCH_USERNAME=elastic
ELASTICSEARCH_PASSWORD=changeme

# OpenObserve Configuration (optional)
OPENOBSERVE_URL=http://localhost:5080
OPENOBSERVE_USERNAME=admin
OPENOBSERVE_PASSWORD=admin
OPENOBSERVE_ORGANIZATION=default
```

## Project Settings in Vercel

### Build & Development Settings

- **Framework Preset**: SvelteKit
- **Build Command**: `npm run vercel-build`
- **Output Directory**: `frontend/build`
- **Install Command**: `npm install`
- **Development Command**: `npm run dev`

### Root Directory

Leave as `.` (root directory)

### Node.js Version

20.x or higher (automatic in Vercel)

## API Endpoints

Once deployed, your API will be available at:

```
https://your-app.vercel.app/api/health
https://your-app.vercel.app/api/query
https://your-app.vercel.app/api/analyze
https://your-app.vercel.app/api/debug
```

### Testing API Endpoints

```bash
# Health check
curl https://your-app.vercel.app/api/health

# Query logs
curl -X POST https://your-app.vercel.app/api/query \
  -H "Content-Type: application/json" \
  -d '{
    "query": "show errors",
    "source": "elasticsearch",
    "config": {
      "url": "https://your-elasticsearch-url",
      "username": "elastic",
      "password": "password"
    }
  }'
```

## Neurolink AI Integration

This application uses the Neurolink SDK for AI operations, providing:

- **Multi-Provider Support**: OpenAI, Anthropic, Google AI, and 12+ more
- **Cost Optimization**: Automatic model selection for cost efficiency
- **100+ Models**: Access to a wide range of AI models
- **Provider Fallback**: Automatic failover if a provider is down

### Switching AI Providers

Update environment variables in Vercel:

```bash
# Use Anthropic Claude
AI_PROVIDER=anthropic
AI_API_KEY=sk-ant-...
AI_MODEL=claude-3-sonnet-20240229

# Use Google AI (free tier available!)
AI_PROVIDER=google-ai
AI_API_KEY=...
AI_MODEL=gemini-pro

# Use OpenAI
AI_PROVIDER=openai
OPENAI_API_KEY=sk-...
AI_MODEL=gpt-4o-mini
```

## Local Development

### Setup

```bash
# Install dependencies
npm install

# Create .env file
cp .env.example .env

# Edit .env with your configuration
nano .env

# Install API dependencies
cd api
npm install
cd ..

# Install frontend dependencies
cd frontend
npm install
cd ..
```

### Run Development Server

```bash
# Run both API and frontend
npm run dev

# Or run separately
npm run dev:api      # API on http://localhost:3000
npm run dev:frontend # Frontend on http://localhost:5173
```

### Build for Production

```bash
# Build everything
npm run build

# Or build separately
npm run build:api
npm run build:frontend
```

## Troubleshooting

### Deployment Fails

1. Check Vercel build logs for errors
2. Verify all environment variables are set correctly
3. Ensure Node.js version is 18 or higher
4. Check that `package.json` scripts are correct

### API Returns 500 Error

1. Check Vercel Function Logs
2. Verify AI API key is valid
3. Check AI provider status
4. Ensure Elasticsearch/OpenObserve is accessible

### Frontend Not Loading

1. Check browser console for errors
2. Verify build completed successfully
3. Check that `frontend/build` directory exists
4. Verify static adapter is configured correctly

### Cannot Connect to Elasticsearch

1. Verify Elasticsearch URL is accessible from Vercel
2. Check credentials are correct
3. Ensure Elasticsearch allows connections from Vercel IPs
4. Consider using Elastic Cloud for public access

## Security Considerations

### API Keys

- Never commit API keys to Git
- Use Vercel environment variables (encrypted at rest)
- Rotate keys regularly
- Use different keys for development and production

### Elasticsearch Access

- Use authentication (username/password or API keys)
- Restrict IP access if possible
- Use HTTPS for connections
- Consider using Elastic Cloud with security features

### CORS

The API endpoints are configured for same-origin requests. If you need cross-origin access:

1. Add custom CORS headers in `vercel.json`
2. Restrict to specific domains
3. Don't allow `*` in production

## Performance Optimization

### Caching

Vercel automatically caches static assets. For API responses:

```typescript
// Add cache headers in your API functions
res.setHeader('Cache-Control', 's-maxage=60, stale-while-revalidate');
```

### Function Size

Keep serverless functions small:
- Import only what you need
- Use tree-shaking
- Consider splitting large functions

### Cold Starts

To minimize cold starts:
- Use lightweight AI models (e.g., `gpt-4o-mini`)
- Keep dependencies minimal
- Use Vercel's Edge Functions for ultra-fast responses

## Monitoring

### Vercel Analytics

Enable in Vercel Dashboard → Analytics:
- Real User Monitoring (RUM)
- Web Vitals
- Function metrics

### Logs

View logs in Vercel Dashboard → Deployments → Logs:
- Build logs
- Function logs
- Error tracking

### Cost Monitoring

Check usage in Vercel Dashboard:
- Function execution time
- Bandwidth usage
- API calls

Also monitor AI provider costs:
- OpenAI usage dashboard
- Anthropic console
- Google Cloud billing

## Production Checklist

Before going to production:

- [ ] Set all environment variables
- [ ] Test all API endpoints
- [ ] Verify Elasticsearch connectivity
- [ ] Enable HTTPS (automatic in Vercel)
- [ ] Set up custom domain (optional)
- [ ] Configure monitoring/alerts
- [ ] Set up error tracking (Sentry, etc.)
- [ ] Review security settings
- [ ] Test with production data
- [ ] Load test API endpoints
- [ ] Set up CI/CD (automatic with GitHub)
- [ ] Document for your team

## Scaling

Vercel automatically scales:
- Serverless functions scale to zero
- CDN distribution worldwide
- Automatic HTTPS and SSL

For high traffic:
- Consider Vercel Pro plan
- Use Edge Functions for lower latency
- Implement caching strategies
- Monitor AI costs

## Custom Domain

1. Go to Vercel Dashboard → Settings → Domains
2. Add your custom domain
3. Configure DNS records as shown
4. Wait for DNS propagation (up to 48 hours)
5. HTTPS is automatic with Let's Encrypt

## CI/CD

Vercel automatically deploys:
- **Production**: Deploys from `main` branch
- **Preview**: Deploys from pull requests and other branches

Configure in `vercel.json` or Vercel Dashboard.

## Support

- **Vercel Docs**: [vercel.com/docs](https://vercel.com/docs)
- **Neurolink Docs**: [github.com/juspay/neurolink](https://github.com/juspay/neurolink)
- **SvelteKit Docs**: [kit.svelte.dev](https://kit.svelte.dev)
- **Issues**: [GitHub Issues](https://github.com/adarshba/logs-explorer/issues)

## Cost Estimation

### Vercel (Free Tier)
- 100 GB-hours of function execution
- 100 GB bandwidth
- Automatic HTTPS

### AI Costs (varies by provider)
- OpenAI gpt-4o-mini: ~$0.15 per 1M input tokens
- Google AI Gemini: Free tier available
- Anthropic Claude: ~$3 per 1M input tokens

Typical query: ~500 tokens = $0.000075 - $0.0015 per query

Monitor usage to estimate actual costs.

## Next Steps

1. Deploy to Vercel
2. Configure environment variables
3. Test with sample data
4. Customize for your needs
5. Share with your team!

Happy log exploring! 🔍🚀
