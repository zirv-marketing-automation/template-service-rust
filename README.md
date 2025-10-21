# template-service-rust
Template used to create services

## CI/CD Pipelines

This repository includes automated CI/CD pipelines using GitHub Actions:

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

### Building
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Linting
```bash
cargo fmt --check
cargo clippy -- -D warnings
```
