#!/bin/bash

# Demo script to showcase structured logging output

echo "=== JSON Logging Format (for Kibana) ==="
echo ""
LOG_LEVEL=info LOG_FORMAT=json SERVICE_NAME=template-service ENVIRONMENT=production cargo run -q --example logging_demo 2>&1 | head -10 || echo "Example not available"

echo ""
echo "=== Pretty Logging Format (for Development) ==="
echo ""
LOG_LEVEL=debug LOG_FORMAT=pretty SERVICE_NAME=template-service ENVIRONMENT=development cargo run -q --example logging_demo 2>&1 | head -10 || echo "Example not available"
