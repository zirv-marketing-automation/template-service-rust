# template-service-rust

[![CI](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/ci.yml)
[![CD](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/cd.yml/badge.svg)](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/cd.yml)
[![Security Audit](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/ci.yml/badge.svg?event=schedule)](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/ci.yml)

Template used to create services

## CI/CD Pipelines

This repository includes automated CI/CD pipelines using GitHub Actions. For detailed documentation, see [.github/WORKFLOWS.md](.github/WORKFLOWS.md).

### Continuous Integration (CI)
The CI pipeline runs on every push and pull request to `main`, `master`, or `develop` branches:
- **Lint**: Checks code formatting (`cargo fmt`) and linting (`cargo clippy`)
- **Build**: Compiles the project in release mode
- **Test**: Runs all unit and integration tests
- **Security Audit**: Scans dependencies for known security vulnerabilities

### Continuous Deployment (CD)
The CD pipeline automatically builds and publishes Docker images:
- Triggered on pushes to `main`/`master` branches and on version tags
- Publishes images to GitHub Container Registry (ghcr.io)
- Supports semantic versioning tags (e.g., `v1.0.0`)
- Can be manually triggered via workflow dispatch

### Release Management
Automated release creation:
- Triggered when pushing version tags (e.g., `v1.0.0`)
- Creates GitHub releases with changelog
- Attaches compiled release binaries

### Dependency Review
- Automatically reviews dependency changes in pull requests
- Flags security vulnerabilities and license issues
- Posts review comments on PRs

## Development

### Prerequisites
- Rust 1.90.0 or later
- Docker (for containerization)
- Kubernetes cluster (for Helm deployment)
- Helm 3.0+ (for Kubernetes deployment)
- kubectl (for Kubernetes deployment)

### Local Development (without Kubernetes)

#### Building
```bash
cargo build --release
```

#### Running Tests
```bash
cargo test
```

#### Linting
```bash
cargo fmt --check
cargo clippy -- -D warnings
```

### Development with Hot Reload in Kubernetes

This project supports hot-reloadable development in a Kubernetes cluster using Helm:

```bash
# Using zirv (if available)
zirv start

# Or manually with Helm
docker build -f Dockerfile.dev -t template-service-dev:latest .
helm upgrade --install template-service ./helm/template-service \
  -f ./helm/template-service/values-dev.yaml \
  --set dev.sourceMount.hostPath="$(pwd)"
```

The hot reload setup:
- Builds a development Docker image with `cargo-watch`
- Deploys to your local Kubernetes cluster
- Mounts your source code as a volume
- Automatically rebuilds and restarts when code changes

Access the service:
```bash
kubectl port-forward svc/template-service 8080:80
```

Then visit http://localhost:8080

To view logs:
```bash
kubectl logs -f deployment/template-service
```

To stop:
```bash
helm uninstall template-service
```

## Deployment

### Kubernetes Deployment with Helm

Production deployment to Kubernetes:

```bash
# Install
helm install template-service ./helm/template-service

# Upgrade
helm upgrade template-service ./helm/template-service

# Uninstall
helm uninstall template-service
```

For comprehensive Kubernetes deployment documentation, including development with hot reload, troubleshooting, and best practices, see [KUBERNETES.md](KUBERNETES.md).

For detailed Helm chart documentation, see [helm/template-service/README.md](helm/template-service/README.md).

### Docker

The project includes two Dockerfiles:
- `Dockerfile`: Production build (multi-stage, optimized)
- `Dockerfile.dev`: Development build with cargo-watch for hot reload

Production images are automatically built and published to GitHub Container Registry (ghcr.io) via the CD pipeline.

## Additional Workflows

This repository includes several automated workflows to improve code quality and collaboration:

- **Auto-labeling**: PRs are automatically labeled based on changed files
- **Dependency Review**: Security scanning of dependency changes in PRs
- **Stale Management**: Automatic cleanup of inactive issues and PRs
- **Release Automation**: Automatic release creation with version tags

For more details, see [.github/WORKFLOWS.md](.github/WORKFLOWS.md).

## Contributing

When contributing to this repository:
1. Create a feature branch from `develop`
2. Make your changes following the coding standards
3. Ensure all tests pass locally
4. Submit a pull request using the PR template
5. Wait for CI checks to pass and address any issues
6. Request review from maintainers
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
