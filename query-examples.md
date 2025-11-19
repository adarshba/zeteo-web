# AI-Powered Log Query Examples

This guide demonstrates how to use natural language to query your logs using otel-mcp-server.

## Basic Log Queries

### Viewing Recent Logs

```
Show me the latest 10 logs
```

```
Get recent logs from the last 5 minutes
```

```
What are the most recent application logs?
```

### Filtering by Log Level

```
Show me all ERROR logs from the last hour
```

```
Find all WARN and ERROR level logs from today
```

```
Get INFO level logs from the payment service
```

```
Show me critical logs from the last 24 hours
```

## Service-Specific Queries

### Filtering by Service/Application

```
Show me logs from the user-authentication-service
```

```
Get all logs related to the payment-gateway
```

```
What errors occurred in the database-service today?
```

```
Show me logs from api-gateway between 2pm and 4pm today
```

### Container/Pod Queries

```
Get logs from the payment-service-pod-abc123
```

```
Show me logs from all pods in the production namespace
```

```
What containers are generating the most errors?
```

## Time-Based Queries

### Recent Time Windows

```
Show me logs from the last 15 minutes
```

```
Get errors from the past 2 hours
```

```
What happened in the last 30 seconds?
```

### Specific Time Ranges

```
Show me logs between 9am and 11am today
```

```
Get yesterday's error logs
```

```
What logs do we have from last Monday?
```

```
Show me logs from December 1st to December 5th
```

### Patterns Over Time

```
Show me the error rate over the last 24 hours
```

```
How many requests per hour did we receive today?
```

```
What's the trend of 500 errors over the last week?
```

## Pattern and Keyword Searches

### Finding Specific Messages

```
Find logs containing "connection timeout"
```

```
Search for logs with "OutOfMemoryError"
```

```
Show me logs mentioning user ID 12345
```

```
Find all logs with "database" in the message
```

### Regular Expression Patterns

```
Find logs matching the pattern "failed to connect to .* after .* attempts"
```

```
Search for IP addresses in error logs
```

```
Show me logs with email addresses
```

## Correlation and Tracing

### Request/Transaction Tracing

```
Show me all logs for request ID abc-123-def-456
```

```
Trace transaction xyz789 across all services
```

```
Find all logs related to trace ID 1234567890abcdef
```

### User Session Analysis

```
Get all logs for user session sess_abc123
```

```
What actions did user user@example.com perform today?
```

```
Show me the complete journey for session ID xyz
```

## Error Analysis

### Error Detection

```
What are the most common errors in the last hour?
```

```
Show me unique error messages from today
```

```
Which services have the highest error rate?
```

```
Find all 500 errors from the API gateway
```

### Error Patterns

```
Are there any recurring error patterns?
```

```
Show me errors that happened more than 10 times
```

```
What's causing the most database connection failures?
```

### Stack Trace Analysis

```
Show me the full stack trace for recent NullPointerExceptions
```

```
Find all logs with Java exceptions
```

```
Get stack traces for errors in the payment service
```

## Performance Monitoring

### Response Time Analysis

```
Show me slow requests (>2 seconds) from the last hour
```

```
What endpoints have the highest response times?
```

```
Find all requests that took longer than 5 seconds
```

### Resource Usage

```
Show me memory-related errors
```

```
Find logs about CPU throttling
```

```
What services are reporting high memory usage?
```

## Security and Audit Queries

### Authentication/Authorization

```
Show me failed login attempts
```

```
Find all authentication errors from the last hour
```

```
Who accessed admin endpoints today?
```

```
Show me unauthorized access attempts
```

### Suspicious Activity

```
Find logs with multiple failed login attempts from the same IP
```

```
Show me unusual API access patterns
```

```
What are the top IPs generating 403 errors?
```

## Aggregation and Statistics

### Counting and Grouping

```
How many error logs do we have per service?
```

```
Count logs by severity level for the last hour
```

```
What are the top 10 most frequent error messages?
```

```
Group logs by hostname and show counts
```

### Trends and Analytics

```
Show me the hourly distribution of logs today
```

```
What's the average response time for the API?
```

```
How many requests per minute are we handling?
```

## Complex Multi-Condition Queries

### Combined Filters

```
Show me ERROR logs from the payment-service in the production environment from the last 2 hours
```

```
Find WARN or ERROR logs from any microservice that mention "database" or "connection"
```

```
Get logs from kubernetes namespace "prod" with log level ERROR and containing "timeout"
```

### Exclusion Queries

```
Show me all errors except 404 Not Found
```

```
Get logs from all services except healthcheck endpoints
```

```
Find errors but exclude logs from the monitoring service
```

## Anomaly Detection

### Unusual Patterns

```
Are there any unusual error spikes in the last 24 hours?
```

```
Detect anomalies in request volume today
```

```
What services are behaving differently than usual?
```

```
Show me any sudden changes in error rates
```

### Baseline Comparison

```
Compare today's error rate with last week
```

```
Is the current traffic normal compared to yesterday?
```

```
Show me metrics that deviate from the baseline
```

## Debugging Workflows

### Investigation Flow

1. **Initial Check:**
   ```
   Are there any errors in the last 5 minutes?
   ```

2. **Identify Problem Service:**
   ```
   Which service is generating the most errors?
   ```

3. **Drill Down:**
   ```
   Show me detailed error logs from the [service-name]
   ```

4. **Find Root Cause:**
   ```
   What were the logs immediately before the first error?
   ```

5. **Check Impact:**
   ```
   How many users were affected by this error?
   ```

### Deployment Verification

```
Compare error rates before and after the deployment at 3pm
```

```
Are there any new error types since the last deployment?
```

```
Show me logs from the new version of the service
```

## Best Practices for Queries

### Be Specific with Time Ranges
❌ Bad: `Show me all errors`
✅ Good: `Show me all errors from the last hour`

### Include Context
❌ Bad: `Find timeouts`
✅ Good: `Find database connection timeouts in the payment service from the last 30 minutes`

### Start Broad, Then Narrow
1. `Show me errors from the last hour`
2. `Which service has the most errors?`
3. `Show me detailed logs from [that-service]`
4. `What's the pattern in these error messages?`

### Use Specific Identifiers
✅ `Show me logs for request ID abc-123`
✅ `Get logs from pod payment-service-7d9f8b-xyz`
✅ `Find logs with trace ID 1234567890abcdef`

## Integration with Development Workflow

### During Development
```
Show me my service's logs from the dev environment
```

```
Are there any errors from my recent changes?
```

### Code Review
```
What errors are related to the user authentication module?
```

```
Show me how often the new API endpoint is called
```

### Debugging Production Issues
```
Show me errors around 2:30 PM when the incident started
```

```
What was the sequence of events leading to the outage?
```

```
Find all related errors across microservices for transaction ID xyz
```

## Tips for Effective Querying

1. **Use Natural Language:** The AI understands context and intent
2. **Be Specific:** Include timeframes, service names, and log levels
3. **Iterate:** Start with broad queries, then refine based on results
4. **Ask Follow-up Questions:** Build on previous queries
5. **Use Domain Terms:** Service names, error types, metric names
6. **Combine Conditions:** Multiple filters in one query for precision

## Quick Reference

| Goal | Example Query |
|------|---------------|
| Recent errors | `Show me errors from the last hour` |
| Specific service | `Get logs from the payment-service` |
| Time range | `Show me logs between 2pm and 4pm` |
| Pattern search | `Find logs containing "timeout"` |
| Trace request | `Show all logs for request ID abc-123` |
| Count errors | `How many errors occurred in the last hour?` |
| Find anomalies | `Are there any unusual patterns today?` |
| Compare periods | `Compare errors today vs yesterday` |

## Getting Started

1. Start with simple queries to verify connectivity
2. Explore your log structure with broad queries
3. Learn what fields are available in your logs
4. Build complex queries incrementally
5. Save useful query patterns for your team
