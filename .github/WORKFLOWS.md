# CI/CD Workflows Documentation

This document provides detailed information about all the automated workflows configured in this repository.

## Overview

This repository uses GitHub Actions for continuous integration and deployment. The workflows are designed to:
- Ensure code quality through automated testing and linting
- Automate the build and deployment process
- Manage dependencies and security
- Improve collaboration through automated PR management

## Workflows

### 1. CI (Continuous Integration) - `ci.yml`

**Triggers:**
- Push to `main`, `master`, or `develop` branches
- Pull requests to `main`, `master`, or `develop` branches

**Jobs:**
- **Lint**: Runs `cargo fmt` and `cargo clippy` to ensure code quality
- **Build**: Compiles the project in release mode and uploads artifacts
- **Test**: Runs all unit and integration tests
- **Security Audit**: Scans dependencies for known vulnerabilities using `cargo-audit`

**Caching:**
- Cargo registry, git index, and build artifacts are cached to speed up subsequent runs

### 2. CD (Continuous Deployment) - `cd.yml`

**Triggers:**
- Push to `main` or `master` branches
- Push of version tags (e.g., `v1.0.0`)
- Manual workflow dispatch

**Actions:**
- Builds Docker image using multi-stage build from the Dockerfile
- Pushes image to GitHub Container Registry (ghcr.io)
- Tags images with branch name, commit SHA, and semantic versions
- Generates build provenance attestation for supply chain security

**Permissions Required:**
- `contents: read` - Read repository contents
- `packages: write` - Push to GitHub Container Registry

### 3. Release - `release.yml`

**Triggers:**
- Push of version tags matching `v*.*.*` pattern

**Actions:**
- Builds release binary
- Generates changelog from git commits
- Creates GitHub Release with the binary artifact
- Attaches release notes

**Usage:**
```bash
# To create a new release, tag the commit and push:
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

### 4. Dependency Review - `dependency-review.yml`

**Triggers:**
- Pull requests to `main` or `master` branches

**Actions:**
- Reviews dependency changes in the PR
- Flags vulnerabilities and license issues
- Posts review summary as PR comment
- Fails on dependencies with moderate or higher severity issues

### 5. Labeler - `labeler.yml`

**Triggers:**
- Pull requests are opened, synchronized, or reopened

**Actions:**
- Automatically labels PRs based on changed files
- Uses configuration from `.github/labeler.yml`

**Label Categories:**
- `documentation` - Changes to markdown files
- `rust` - Changes to Rust source files or Cargo files
- `docker` - Changes to Docker-related files
- `ci-cd` - Changes to GitHub Actions workflows
- `dependencies` - Changes to Cargo.toml or Cargo.lock
- `backend` - Changes to backend code

### 6. Stale Issues and PRs - `stale.yml`

**Triggers:**
- Daily at midnight UTC
- Manual workflow dispatch

**Actions:**
- Marks issues/PRs as stale after 60 days of inactivity
- Closes stale items after 7 additional days
- Exempts items labeled with `pinned`, `security`, or `bug`

**Configuration:**
- `days-before-stale`: 60
- `days-before-close`: 7

## Repository Secrets

Some workflows require repository secrets to be configured:

### Required Secrets:
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions (no configuration needed)

### Optional Secrets:
If deploying to external services, you may need to add:
- Container registry credentials (if not using GitHub Container Registry)
- Cloud provider credentials for deployment
- Notification service tokens

## Workflow Status Badges

Add these badges to your README.md to display workflow status:

```markdown
[![CI](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/ci.yml)
[![CD](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/cd.yml/badge.svg)](https://github.com/zirv-marketing-automation/template-service-rust/actions/workflows/cd.yml)
```

## Best Practices

1. **Always create pull requests** for changes to trigger CI checks
2. **Wait for CI to pass** before merging PRs
3. **Use semantic versioning** for releases (v1.0.0, v1.1.0, etc.)
4. **Keep dependencies up to date** - Dependabot can help with this
5. **Review security audit results** and address vulnerabilities promptly

## Customization

To customize these workflows:

1. Edit workflow files in `.github/workflows/`
2. Modify labeler configuration in `.github/labeler.yml`
3. Adjust PR template in `.github/PULL_REQUEST_TEMPLATE/`

## Troubleshooting

### CI Failures

**Lint failures:**
- Run `cargo fmt` locally to fix formatting issues
- Run `cargo clippy --fix` to automatically fix common issues

**Build failures:**
- Ensure all dependencies are properly declared in `Cargo.toml`
- Check for compilation errors in your code

**Test failures:**
- Run `cargo test` locally to reproduce
- Check test logs in the Actions tab

### CD Failures

**Docker build failures:**
- Verify Dockerfile syntax
- Ensure all required files are present
- Check Docker build logs in Actions

**Push failures:**
- Verify repository permissions
- Check if GitHub Container Registry is enabled

## Support

For issues with workflows:
1. Check the Actions tab for detailed logs
2. Review this documentation
3. Create an issue with the `ci-cd` label
