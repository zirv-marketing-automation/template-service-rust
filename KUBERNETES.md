# Kubernetes Deployment Guide

This guide explains how to deploy the template-service to Kubernetes using Helm.

## Overview

The template-service can be deployed to Kubernetes in two modes:
1. **Production Mode**: Uses the production Docker image from GHCR
2. **Development Mode**: Uses a development image with hot reload capabilities

## Prerequisites

### Required Tools
- Docker (for building images)
- Kubernetes cluster (minikube, kind, Docker Desktop, or cloud provider)
- Helm 3.0+
- kubectl

### Setting up a Local Kubernetes Cluster

If you don't have a Kubernetes cluster, you can set up a local one:

#### Option 1: Docker Desktop
Enable Kubernetes in Docker Desktop settings.

#### Option 2: Minikube
```bash
# Install minikube (macOS)
brew install minikube

# Start minikube
minikube start

# Enable ingress (optional)
minikube addons enable ingress
```

#### Option 3: kind (Kubernetes in Docker)
```bash
# Install kind (macOS)
brew install kind

# Create cluster
kind create cluster --name template-service

# Load images to kind
kind load docker-image template-service-dev:latest --name template-service
```

## Production Deployment

### Quick Start

```bash
# Deploy using default values
helm install template-service ./helm/template-service

# Access the service
kubectl port-forward svc/template-service 8080:80
```

### Custom Configuration

```bash
# Deploy with custom values
helm install template-service ./helm/template-service \
  --set image.tag=v1.0.0 \
  --set database.url="mysql://user:pass@mysql-host:3306/dbname" \
  --set ingress.enabled=true

# Or use a custom values file
helm install template-service ./helm/template-service \
  -f my-values.yaml
```

### Upgrading

```bash
# Upgrade the deployment
helm upgrade template-service ./helm/template-service

# Or with new values
helm upgrade template-service ./helm/template-service \
  -f my-values.yaml
```

## Development Deployment with Hot Reload

The development mode enables you to develop and test your code directly in Kubernetes with automatic reload on file changes. The development environment includes the complete infrastructure stack:

- **Template Service**: The main application with hot reload capabilities
- **Kafka**: Message broker with Zookeeper for event streaming
- **Elasticsearch**: Log storage and search engine
- **Kibana**: Web UI for log visualization and analysis

### How It Works

1. **Infrastructure Setup**: Deploys Kafka, Elasticsearch, and Kibana using Bitnami Helm charts
2. **Development Image**: Uses `Dockerfile.dev` which includes `cargo-watch`
3. **Volume Mount**: Mounts your local source code into the container
4. **Auto Rebuild**: `cargo-watch` detects changes and rebuilds/restarts the application
5. **Live Development**: Changes to your code are immediately reflected in the running container

### Quick Start with zirv

```bash
# Using the zirv start script (recommended)
zirv start
```

This automatically:
- Adds the Bitnami Helm repository
- Deploys Kafka with Zookeeper (single replica, no persistence for dev)
- Deploys Elasticsearch (single node, no persistence for dev)
- Deploys Kibana (connected to Elasticsearch)
- Builds the development Docker image
- Deploys the template service to your Kubernetes cluster
- Sets up volume mounts for hot reload
- Displays instructions for accessing all services

### Manual Development Deployment

```bash
# 1. Build the development image
docker build -f Dockerfile.dev -t template-service-dev:latest .

# 2. Load image into your cluster (if using minikube/kind)
# For minikube:
minikube image load template-service-dev:latest
# For kind:
kind load docker-image template-service-dev:latest

# 3. Deploy with development configuration
helm upgrade --install template-service ./helm/template-service \
  -f ./helm/template-service/values-dev.yaml \
  --set dev.sourceMount.hostPath="$(pwd)"

# 4. Access the service
kubectl port-forward svc/template-service 8080:80

# 5. View logs to see auto-reload in action
kubectl logs -f deployment/template-service
```

### Development Workflow

1. Make changes to your code locally
2. Save the file
3. Watch the logs to see cargo-watch rebuild and restart
4. Test your changes at http://localhost:8080
5. Repeat!

Example log output when a file changes:
```
[Running 'cargo run -p backend']
Compiling backend v0.1.0 (/usr/src/app/backend)
Finished dev [unoptimized + debuginfo] target(s) in 3.45s
Running `target/debug/backend`
Server running at http://0.0.0.0:3000
```

## Accessing the Services

### Port Forwarding (Development)

In development mode, you'll have multiple services to access:

```bash
# Template Service
kubectl port-forward svc/template-service 8080:80

# Kafka
kubectl port-forward svc/kafka 9092:9092

# Kibana (for log visualization)
kubectl port-forward svc/kibana 5601:5601

# Elasticsearch (direct access if needed)
kubectl port-forward svc/elasticsearch 9200:9200
```

Service URLs:
- **Template Service**: http://localhost:8080
  - Health check: `curl http://localhost:8080/health`
- **Kibana**: http://localhost:5601
  - View and search application logs
- **Elasticsearch**: http://localhost:9200
  - Direct access to log storage
- **Kafka**: localhost:9092
  - Message broker for event streaming

### Ingress (Production)
```bash
# Enable ingress in values
helm upgrade template-service ./helm/template-service \
  --set ingress.enabled=true \
  --set ingress.hosts[0].host=template-service.example.com

# Access at http://template-service.example.com
```

### Minikube Service
```bash
# Get service URL (minikube only)
minikube service template-service --url
```

## Monitoring and Debugging

### Structured Logging with Kibana

The service outputs structured JSON logs that are automatically collected and sent to Kibana. Each log includes service name, environment, and contextual information.

For detailed logging documentation, see [LOGGING.md](LOGGING.md).

**Viewing logs in Kibana:**
1. Access your Kibana dashboard
2. Use the filter: `service_name: "template-service"`
3. Filter by environment: `environment: "production"`
4. Filter by log level (e.g., errors only): `level: 50`

### View Logs
```bash
# Follow logs
kubectl logs -f deployment/template-service

# View logs from all pods
kubectl logs -l app.kubernetes.io/name=template-service

# Previous logs (if pod crashed)
kubectl logs deployment/template-service --previous
```

### Check Pod Status
```bash
# Get pod status
kubectl get pods -l app.kubernetes.io/name=template-service

# Describe pod for events
kubectl describe pod <pod-name>

# Get detailed deployment status
kubectl get deployment template-service -o yaml
```

### Execute Commands in Pod
```bash
# Get shell access
kubectl exec -it deployment/template-service -- /bin/bash

# Run a command
kubectl exec deployment/template-service -- env
```

### Health Checks
```bash
# Check if health endpoint is responding
kubectl exec deployment/template-service -- curl localhost:3000/health
```

## Configuration Options

### Environment Variables

The application can be configured using environment variables in `values.yaml`:

```yaml
env:
  - name: RUST_LOG
    value: "info"  # debug, info, warn, error
  - name: APP_HOST
    value: "0.0.0.0"
  - name: APP_PORT
    value: "3000"
  - name: ENV
    value: "production"  # development, staging, production
  # Logging configuration
  - name: LOG_LEVEL
    value: "info"  # Logging level
  - name: LOG_FORMAT
    value: "json"  # json for Kibana, pretty for development
  - name: SERVICE_NAME
    value: "template-service"  # Service identifier
  - name: ENVIRONMENT
    value: "production"  # Environment name for log filtering
```

See [LOGGING.md](LOGGING.md) for detailed logging configuration.

### Database Configuration

```yaml
# Option 1: Direct URL in values.yaml (not recommended for production)
database:
  url: "mysql://user:pass@host:3306/dbname"

# Option 2: Use existing Kubernetes secret (recommended)
database:
  existingSecret: "my-db-secret"
  existingSecretKey: "database-url"
```

Create the secret:
```bash
kubectl create secret generic my-db-secret \
  --from-literal=database-url='mysql://user:pass@host:3306/dbname'
```

### Resource Limits

```yaml
resources:
  limits:
    cpu: 500m
    memory: 512Mi
  requests:
    cpu: 250m
    memory: 256Mi
```

### Autoscaling

```yaml
autoscaling:
  enabled: true
  minReplicas: 2
  maxReplicas: 10
  targetCPUUtilizationPercentage: 80
```

## Uninstalling

### Remove Template Service Only
```bash
# Remove the Helm release
helm uninstall template-service

# Verify removal
kubectl get all -l app.kubernetes.io/name=template-service
```

### Remove All Development Services
If you're using the full development environment with Kafka and Kibana:

```bash
# Remove all services
helm uninstall template-service kafka elasticsearch kibana

# Verify all resources are removed
kubectl get all
```

## Troubleshooting

### Image Pull Errors
```bash
# Check image pull policy
kubectl describe pod <pod-name> | grep -A 5 "Image"

# For local development, ensure image is loaded into cluster
# minikube:
minikube image ls | grep template-service
# kind:
docker exec -it kind-control-plane crictl images | grep template-service
```

### Volume Mount Issues (Development)
```bash
# Verify volume is mounted correctly
kubectl exec deployment/template-service -- ls -la /usr/src/app

# Check volume configuration
kubectl describe pod <pod-name> | grep -A 10 "Volumes:"
```

### Database Connection Issues
```bash
# Check database URL in secret
kubectl get secret template-service-db -o jsonpath='{.data.database-url}' | base64 -d

# Check if database is accessible from pod
kubectl exec deployment/template-service -- nc -zv mysql 3306
```

### Hot Reload Not Working
1. Ensure volume mount path is correct
2. Check cargo-watch is running: `kubectl logs deployment/template-service`
3. Verify file changes are synced to the pod
4. Check if cargo-watch is watching the correct paths

## Best Practices

### Production
- Use specific image tags (not `latest` or `main`)
- Set resource limits and requests
- Use secrets for sensitive data
- Enable readiness and liveness probes
- Use ingress with TLS for external access
- Enable horizontal pod autoscaling for high traffic

### Development
- Use local Kubernetes cluster (minikube/kind)
- Mount source code for hot reload
- Use debug logging level
- Disable or relax health check probes
- Use higher resource limits for faster builds

## CI/CD Integration

The production Docker image is automatically built and pushed to GHCR by GitHub Actions. See `.github/workflows/cd.yml` for details.

To deploy the latest version:
```bash
helm upgrade template-service ./helm/template-service \
  --set image.tag=main
```

## Further Reading

- [Helm Documentation](https://helm.sh/docs/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Chart README](./helm/template-service/README.md)
