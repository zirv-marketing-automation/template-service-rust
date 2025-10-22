# Logging Configuration

This document describes the logging setup for the template-service and how logs are sent to Kibana for monitoring and analysis.

## Overview

The template-service uses structured logging with JSON output format, which is designed to integrate seamlessly with the ELK (Elasticsearch, Logstash, Kibana) stack running in the Kubernetes cluster. All logs include contextual information such as service name, environment, timestamps, and request details.

## Logging Architecture

### Technology Stack

- **[tracing](https://crates.io/crates/tracing)**: Modern instrumentation framework for async Rust applications
- **[tracing-subscriber](https://crates.io/crates/tracing-subscriber)**: Subscriber implementations for collecting and processing trace data
- **[tracing-bunyan-formatter](https://crates.io/crates/tracing-bunyan-formatter)**: Formats logs in Bunyan JSON format, compatible with Kibana
- **[tracing-actix-web](https://crates.io/crates/tracing-actix-web)**: Adds HTTP request tracing to actix-web applications

### Log Format

Logs are output in JSON format by default in production, with each log entry containing:

```json
{
  "v": 0,
  "name": "template-service",
  "msg": "Logging initialized",
  "level": 30,
  "hostname": "pod-name",
  "pid": 1234,
  "time": "2025-10-21T18:51:43.369115083Z",
  "target": "backend::utils::logging",
  "line": 50,
  "file": "backend/src/utils/logging.rs",
  "service_name": "template-service",
  "environment": "production",
  "log_level": "info"
}
```

### Log Levels

The following log levels are supported (in order of severity):
- `trace` - Very detailed debugging information
- `debug` - Debugging information
- `info` - General informational messages (default)
- `warn` - Warning messages
- `error` - Error messages

## Configuration

### Environment Variables

The logging system can be configured using the following environment variables:

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `LOG_LEVEL` | Logging level | `info` | `debug`, `info`, `warn`, `error` |
| `LOG_FORMAT` | Output format | `json` | `json` (for Kibana), `pretty` (for development) |
| `SERVICE_NAME` | Service identifier | `template-service` | `template-service` |
| `ENVIRONMENT` | Environment name | `production` | `production`, `staging`, `development` |

### Kubernetes Configuration

#### Production (values.yaml)

```yaml
env:
  - name: LOG_LEVEL
    value: "info"
  - name: LOG_FORMAT
    value: "json"  # JSON format for Kibana
  - name: SERVICE_NAME
    value: "template-service"
  - name: ENVIRONMENT
    value: "production"
```

#### Development (values-dev.yaml)

```yaml
env:
  - name: LOG_LEVEL
    value: "debug"
  - name: LOG_FORMAT
    value: "pretty"  # Human-readable format for development
  - name: SERVICE_NAME
    value: "template-service"
  - name: ENVIRONMENT
    value: "development"
```

## HTTP Request Logging

All HTTP requests are automatically logged with the following information:
- Request method (GET, POST, etc.)
- Request path
- Status code
- Response time
- Request ID (for tracing)

Example HTTP request log:

```json
{
  "v": 0,
  "name": "template-service",
  "msg": "finished processing request",
  "level": 30,
  "time": "2025-10-21T18:51:43.369115083Z",
  "http.method": "GET",
  "http.route": "/health",
  "http.status_code": 200,
  "http.response_time_ms": 2,
  "trace_id": "12345"
}
```

## Integration with Kibana

### Log Collection

When deployed to Kubernetes, logs are:
1. Written to stdout in JSON format
2. Collected by the Kubernetes logging infrastructure
3. Forwarded to Elasticsearch
4. Made available in Kibana for searching and visualization

### Querying Logs in Kibana

#### Common Queries

**Filter by service:**
```
service_name: "template-service"
```

**Filter by environment:**
```
environment: "production"
```

**Filter by log level (errors only):**
```
level: 50
```

Note: Bunyan log levels are numeric:
- 10 = trace
- 20 = debug
- 30 = info
- 40 = warn
- 50 = error

**Filter by HTTP status code:**
```
http.status_code: 500
```

**Search for specific messages:**
```
msg: "Failed to seed database"
```

#### Creating Dashboards

1. Go to Kibana → Discover
2. Select the appropriate index pattern (usually `logstash-*` or `filebeat-*`)
3. Add filters based on `service_name` to show only template-service logs
4. Save as a dashboard for quick access

## Development

### Local Development

For local development, use the `pretty` format for easier reading:

```bash
export LOG_LEVEL=debug
export LOG_FORMAT=pretty
export SERVICE_NAME=template-service
export ENVIRONMENT=development
cargo run
```

Example pretty output:
```
2025-10-21T18:51:58.240308Z  INFO backend::utils::logging: Logging initialized service_name=template-service environment=development log_level=debug
2025-10-21T18:52:01.123456Z  INFO backend: Starting HTTP server address=0.0.0.0:3000
```

### Adding Custom Logs

To add logging to your code:

```rust
use tracing::{info, warn, error, debug};

// Simple log
info!("Server started");

// Log with structured fields
info!(
    user_id = %user_id,
    action = "login",
    "User logged in successfully"
);

// Error logging with context
error!(
    error = ?err,
    user_id = %user_id,
    "Failed to process request"
);
```

### Testing Logging

To verify logging output:

```bash
# Test JSON format
LOG_FORMAT=json cargo test

# Test pretty format
LOG_FORMAT=pretty cargo test
```

## Troubleshooting

### Logs Not Appearing in Kibana

1. **Check pod logs directly:**
   ```bash
   kubectl logs deployment/template-service
   ```

2. **Verify JSON format:**
   ```bash
   kubectl logs deployment/template-service | jq
   ```
   If the output is valid JSON, the format is correct.

3. **Check Elasticsearch/Logstash configuration:**
   Ensure the Kubernetes logging infrastructure is properly configured to forward logs.

4. **Verify index patterns in Kibana:**
   Make sure the index pattern matches the indices where logs are stored.

### No Logs Generated

1. Check the `LOG_LEVEL` environment variable isn't set to a level higher than the logs you're expecting
2. Verify the logging initialization in `main.rs` is being called
3. Check for errors during logging initialization in the startup logs

### Performance Issues

If logging is causing performance problems:
1. Increase the `LOG_LEVEL` to `warn` or `error` in production
2. Review and optimize frequent log statements
3. Consider sampling high-frequency logs

## Best Practices

### Do's
- ✅ Use structured logging with key-value pairs
- ✅ Include relevant context in log messages
- ✅ Use appropriate log levels
- ✅ Keep JSON format enabled in production
- ✅ Include service_name and environment in all logs
- ✅ Use `info` level for normal operations
- ✅ Use `error` level for actual errors that need attention

### Don'ts
- ❌ Log sensitive data (passwords, tokens, PII)
- ❌ Use `println!` or `eprintln!` - use tracing instead
- ❌ Log at `debug` level in production (high volume)
- ❌ Include large payloads in logs
- ❌ Use pretty format in production (not parseable by Kibana)

## References

- [Tracing Documentation](https://docs.rs/tracing/)
- [Tracing Subscriber](https://docs.rs/tracing-subscriber/)
- [Bunyan Log Format](https://github.com/trentm/node-bunyan)
- [Kibana Documentation](https://www.elastic.co/guide/en/kibana/current/index.html)
