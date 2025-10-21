#!/bin/bash
# Helper script to test Helm chart templates without full deployment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Helm Chart Validation ==="
echo ""

echo "1. Linting Helm chart..."
helm lint ./helm/template-service
echo "✓ Lint passed"
echo ""

echo "2. Testing template rendering (production)..."
helm template template-service ./helm/template-service --dry-run > /dev/null
echo "✓ Production templates render successfully"
echo ""

echo "3. Testing template rendering (development)..."
helm template template-service ./helm/template-service \
  -f ./helm/template-service/values-dev.yaml \
  --set dev.sourceMount.hostPath="$(pwd)" \
  --dry-run > /dev/null
echo "✓ Development templates render successfully"
echo ""

echo "4. Checking for required Kubernetes resources..."
RESOURCES=$(helm template template-service ./helm/template-service --dry-run | grep "^kind:" | sort -u)
echo "$RESOURCES"
echo ""

echo "=== All validations passed! ==="
echo ""
echo "To deploy to a Kubernetes cluster:"
echo "  Production: helm install template-service ./helm/template-service"
echo "  Development: helm install template-service ./helm/template-service -f ./helm/template-service/values-dev.yaml --set dev.sourceMount.hostPath=\"\$(pwd)\""
