# Template Service Helm Chart

This Helm chart deploys the template-service Rust application to a Kubernetes cluster.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+
- kubectl configured to communicate with your cluster

## Installation

### Production Deployment

```bash
helm install template-service ./helm/template-service
```

### Development Deployment with Hot Reload

For development with hot reload capabilities:

```bash
# Set the path to your source code
helm install template-service ./helm/template-service \
  -f ./helm/template-service/values-dev.yaml \
  --set dev.sourceMount.hostPath="$(pwd)"
```

Or use the zirv start script:

```bash
# This automatically builds the dev image and deploys with hot reload
zirv start
```

## Configuration

The following table lists the configurable parameters and their default values.

### Basic Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of replicas | `1` |
| `image.repository` | Image repository | `ghcr.io/zirv-marketing-automation/template-service-rust` |
| `image.tag` | Image tag | `main` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `service.type` | Kubernetes service type | `ClusterIP` |
| `service.port` | Service port | `80` |
| `service.targetPort` | Container port | `3000` |

### Database Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `database.url` | Database connection URL | `mysql://root:password@mysql:3306/template_service` |
| `database.existingSecret` | Name of existing secret with database credentials | `nil` |
| `database.existingSecretKey` | Key in the secret containing the database URL | `nil` |

### Development Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `dev.enabled` | Enable development mode | `false` |
| `dev.sourceMount.enabled` | Mount source code for hot reload | `false` |
| `dev.sourceMount.hostPath` | Path to source code on host | `""` |
| `dev.image.repository` | Development image repository | `template-service-dev` |
| `dev.image.tag` | Development image tag | `latest` |

### Resource Limits

| Parameter | Description | Default |
|-----------|-------------|---------|
| `resources.limits.cpu` | CPU limit | `500m` |
| `resources.limits.memory` | Memory limit | `512Mi` |
| `resources.requests.cpu` | CPU request | `250m` |
| `resources.requests.memory` | Memory request | `256Mi` |

### Ingress Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `ingress.enabled` | Enable ingress | `false` |
| `ingress.className` | Ingress class name | `""` |
| `ingress.hosts` | Ingress hosts configuration | See values.yaml |

## Accessing the Service

After deployment, you can access the service using port-forward:

```bash
kubectl port-forward svc/template-service 8080:80
```

Then access the service at http://localhost:8080

## Health Check

The service provides a health check endpoint at `/health`

## Hot Reload Development

The development configuration enables hot reload by:
1. Using a development Docker image with `cargo-watch`
2. Mounting the source code as a volume
3. Watching for file changes and automatically rebuilding

When files change on your host machine, cargo-watch inside the container will detect the changes and rebuild/restart the application.

## Uninstalling

```bash
helm uninstall template-service
```

## Upgrading

```bash
helm upgrade template-service ./helm/template-service
```

## Examples

### Deploy with custom database URL

```bash
helm install template-service ./helm/template-service \
  --set database.url="mysql://user:pass@hostname:3306/dbname"
```

### Deploy with existing secret

```bash
helm install template-service ./helm/template-service \
  --set database.existingSecret=my-db-secret \
  --set database.existingSecretKey=url
```

### Enable ingress

```bash
helm install template-service ./helm/template-service \
  --set ingress.enabled=true \
  --set ingress.hosts[0].host=template-service.example.com
```

### Deploy with autoscaling

```bash
helm install template-service ./helm/template-service \
  --set autoscaling.enabled=true \
  --set autoscaling.minReplicas=2 \
  --set autoscaling.maxReplicas=10
```
