# Code Quality & Coverage Setup

This document explains how to set up Codecov and SonarQube integration for the SkillCapped Generator project.

## Prerequisites

- GitHub repository with Actions enabled
- Codecov account (free for public repositories)
- SonarCloud account (free for public repositories)

## Codecov Setup

### 1. Create Codecov Account
1. Go to [codecov.io](https://codecov.io/)
2. Sign up with your GitHub account
3. Select your repository

### 2. Get Upload Token
1. Navigate to your repository on Codecov
2. Go to Settings → General
3. Copy the Repository Upload Token

### 3. Add GitHub Secret
1. Go to your GitHub repository
2. Navigate to Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Name: `CODECOV_TOKEN`
5. Value: Paste your Codecov upload token

## SonarCloud Setup

### 1. Create SonarCloud Account
1. Go to [sonarcloud.io](https://sonarcloud.io/)
2. Sign up with your GitHub account
3. Import your GitHub repository

### 2. Configure Project
1. Select your organization
2. Choose "Import repository from GitHub"
3. Select your repository
4. Set the project key (e.g., `Xerrion_skillcapped-generator`)

### 3. Get Analysis Token
1. Go to your SonarCloud project
2. Navigate to Administration → Analysis Method
3. Choose "GitHub Actions"
4. Copy the SONAR_TOKEN

### 4. Add GitHub Secret
1. Go to your GitHub repository
2. Navigate to Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Name: `SONAR_TOKEN`
5. Value: Paste your SonarCloud token

## Configuration Files

### Codecov Configuration (`codecov.yml`)
- Sets coverage targets (80% project, 80% patch)
- Configures comment layout and notifications
- Ignores test files and build artifacts
- Enables GitHub check annotations

### SonarQube Configuration (`sonar-project.properties`)
- Defines project metadata
- Sets source and test directories
- Configures coverage report locations
- Excludes build artifacts and dependencies

## Workflow Integration

The GitHub Actions workflow includes:

1. **Test Suite**: Fast parallel testing with nextest on multiple OS/Rust versions
2. **Coverage Generation**: Uses `cargo-llvm-cov` with nextest for accurate coverage including doctests
3. **Codecov Upload**: Automatic coverage reporting
4. **SonarQube Analysis**: Code quality and security analysis
5. **Security Audit**: Dependency vulnerability scanning
6. **MSRV Check**: Minimum Supported Rust Version validation

### Nextest Integration

The project uses [`cargo-nextest`](https://nexte.st/) for faster and more reliable test execution:

- **Parallel Execution**: Tests run in parallel with optimal thread utilization
- **Flaky Test Handling**: Automatic retries for unstable tests
- **Rich Output**: Better test result reporting and failure diagnostics
- **CI Optimization**: Separate profiles for local development and CI
- **JUnit Reports**: XML output for integration with CI/CD systems
- **Coverage Integration**: Seamless integration with llvm-cov for comprehensive coverage

Nextest profiles (configured in `.config/nextest.toml`):
- **default**: Local development with num-cpus threads and 2 retries for flaky tests
- **ci**: CI-optimized with 4 threads, fail-fast disabled, and comprehensive output
- **coverage**: Single-threaded execution with no retries for consistent coverage collection

### Smart Caching with rust-cache

The workflows use [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache) for intelligent caching:

- **Automatic Cache Keys**: Based on Rust version, OS, and dependency changes
- **Selective Caching**: Only caches dependencies, not workspace crates
- **Cache Cleaning**: Removes unused dependencies and old artifacts
- **Cross-Job Sharing**: Shared cache keys for similar build configurations
- **Master-Only Saves**: Main caches only saved from master branch to prevent pollution

Cache configuration per job:
- **Test Suite**: OS and Rust version specific keys
- **Coverage**: Shared cache key for consistent coverage builds  
- **Security**: Tool-only caching (no build artifacts)
- **MSRV**: Nightly-specific cache (restore-only to avoid daily invalidation)
- **Release**: Target-specific caching for cross-compilation

## Branch Protection

Recommended branch protection rules:
- Require status checks to pass
- Require branches to be up to date
- Include required checks:
  - `Test Suite`
  - `Code Coverage`
  - `SonarQube Analysis`
  - `Security Audit`

## Monitoring

Both services provide:
- **Codecov**: Coverage trends, PR coverage diffs, file-level coverage
- **SonarCloud**: Code quality gates, security hotspots, code smells, duplications

Access your dashboards:
- Codecov: `https://codecov.io/gh/{username}/{repository}`
- SonarCloud: `https://sonarcloud.io/project/overview?id={project-key}`

## Troubleshooting

### Common Issues
1. **401 Unauthorized**: Check token validity and permissions
2. **Coverage not uploading**: Verify lcov.info file generation
3. **SonarQube analysis failed**: Check project key and token
4. **Missing coverage**: Ensure all test files are included

### Debug Steps
1. Check GitHub Actions logs for detailed error messages
2. Verify token configuration in repository secrets
3. Test locally using `act` for GitHub Actions
4. Validate configuration files syntax
