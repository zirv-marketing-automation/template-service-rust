# Quick Deployment Summary

## What Was Added

### Helm Chart (`helm/template-service/`)
A complete Helm chart for deploying the template-service to Kubernetes with:
- Production and development configurations
- ConfigMap for environment variables
- Secret for database credentials
- Deployment with health checks
- Service for pod networking
- Optional Ingress for external access
- Optional HorizontalPodAutoscaler for scaling

### Development Setup
- `Dockerfile.dev`: Development image with cargo-watch for hot reload
- `values-dev.yaml`: Development-specific Helm values
- Updated `.zirv/start.yaml`: Script to deploy development environment to Kubernetes

### Documentation
- `KUBERNETES.md`: Comprehensive Kubernetes deployment guide
- `helm/template-service/README.md`: Helm chart documentation
- `test-helm.sh`: Script to validate Helm chart
- Updated main `README.md` with deployment instructions

### Fixes
- Fixed `Dockerfile` to remove references to non-existent `shared` directory

## Quick Start

### Development with Hot Reload
```bash
# Using zirv (recommended)
zirv start

# Manual
docker build -f Dockerfile.dev -t template-service-dev:latest .
helm upgrade --install template-service ./helm/template-service \
  -f ./helm/template-service/values-dev.yaml \
  --set dev.sourceMount.hostPath="$(pwd)"
kubectl port-forward svc/template-service 8080:80
```

### Production Deployment
```bash
helm install template-service ./helm/template-service
kubectl port-forward svc/template-service 8080:80
```

### Validate Chart
```bash
./test-helm.sh
```

## Key Features

### Hot Reload Development
- Mount local source code into container
- Automatic rebuild on file changes via cargo-watch
- Full Kubernetes environment for realistic testing
- No need to rebuild image for code changes

### Production Ready
- Multi-stage Docker build for optimized images
- Proper security context (non-root user)
- Health checks (liveness and readiness probes)
- Resource limits and requests
- Autoscaling support
- Ingress support

### Flexible Configuration
- Separate values files for different environments
- Secret management for sensitive data
- ConfigMap for environment variables
- Support for existing secrets

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Developer Machine                                       │
│  ┌────────────────────────────────────────────────┐    │
│  │  Source Code (mounted as volume)               │    │
│  └─────────────────┬──────────────────────────────┘    │
│                    │                                     │
└────────────────────┼─────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Kubernetes Cluster                                      │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Pod: template-service                           │  │
│  │  ┌────────────────────────────────────────────┐ │  │
│  │  │  Container: template-service               │ │  │
│  │  │  - cargo-watch (monitors files)            │ │  │
│  │  │  - Auto rebuild on changes                 │ │  │
│  │  │  - Source: /usr/src/app (mounted volume)   │ │  │
│  │  │  - Port: 3000                              │ │  │
│  │  └────────────────────────────────────────────┘ │  │
│  └──────────────────────────────────────────────────┘  │
│                     │                                    │
│                     ▼                                    │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Service: template-service                       │  │
│  │  - Type: ClusterIP                              │  │
│  │  - Port: 80 → 3000                              │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                     │
                     ▼ (port-forward)
              http://localhost:8080
```

## Files Changed/Added

### New Files
- `Dockerfile.dev` - Development Docker image
- `helm/template-service/Chart.yaml` - Helm chart metadata
- `helm/template-service/values.yaml` - Default values
- `helm/template-service/values-dev.yaml` - Development values
- `helm/template-service/templates/_helpers.tpl` - Template helpers
- `helm/template-service/templates/deployment.yaml` - Deployment manifest
- `helm/template-service/templates/service.yaml` - Service manifest
- `helm/template-service/templates/configmap.yaml` - ConfigMap manifest
- `helm/template-service/templates/secret.yaml` - Secret manifest
- `helm/template-service/templates/serviceaccount.yaml` - ServiceAccount manifest
- `helm/template-service/templates/ingress.yaml` - Ingress manifest
- `helm/template-service/templates/hpa.yaml` - HPA manifest
- `helm/template-service/README.md` - Helm chart documentation
- `helm/template-service/.helmignore` - Helm ignore file
- `KUBERNETES.md` - Kubernetes deployment guide
- `test-helm.sh` - Helm validation script

### Modified Files
- `.zirv/start.yaml` - Updated to deploy to Kubernetes
- `README.md` - Added deployment documentation
- `Dockerfile` - Fixed shared directory reference

## Testing

### Validate Helm Chart
```bash
./test-helm.sh
```

### Test in Kubernetes
```bash
# Start minikube (if not running)
minikube start

# Deploy
docker build -f Dockerfile.dev -t template-service-dev:latest .
minikube image load template-service-dev:latest
helm install template-service ./helm/template-service \
  -f ./helm/template-service/values-dev.yaml \
  --set dev.sourceMount.hostPath="$(pwd)"

# Check status
kubectl get pods
kubectl logs -f deployment/template-service

# Test the service
kubectl port-forward svc/template-service 8080:80
curl http://localhost:8080/health

# Clean up
helm uninstall template-service
```

## Troubleshooting

### Issue: Image not found in cluster
**Solution**: Load image into cluster
```bash
# minikube
minikube image load template-service-dev:latest
# kind
kind load docker-image template-service-dev:latest
```

### Issue: Volume mount not working
**Solution**: Ensure path is absolute and cluster can access it
```bash
helm upgrade --install template-service ./helm/template-service \
  -f ./helm/template-service/values-dev.yaml \
  --set dev.sourceMount.hostPath="$(pwd)"
```

### Issue: Hot reload not working
**Solution**: Check cargo-watch is running
```bash
kubectl logs deployment/template-service | grep cargo-watch
```

## Next Steps

1. Customize `values.yaml` for your environment
2. Add MySQL/database deployment to cluster (if needed)
3. Configure ingress for external access
4. Set up CI/CD to deploy to Kubernetes
5. Configure monitoring and logging
6. Add PersistentVolumeClaims if needed
7. Configure RBAC policies
8. Set up secrets management (e.g., sealed-secrets, external-secrets)

## References

- [KUBERNETES.md](KUBERNETES.md) - Full deployment guide
- [helm/template-service/README.md](helm/template-service/README.md) - Helm chart docs
- [Helm Documentation](https://helm.sh/docs/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
