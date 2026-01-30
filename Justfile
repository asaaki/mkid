# Justfile for mkid development
# Install just: https://github.com/casey/just

# Default recipe - show available commands
default:
    @just --list

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Format code and run linter
lint:
    cargo fmt
    cargo clippy -- -D warnings

# Auto-fix issues
fix:
    cargo clippy --fix --allow-dirty --allow-staged
    cargo fmt

# Run all checks
check: lint test

# Clean build artifacts
clean:
    cargo clean

# Install mkid locally
install:
    cargo install --path .

# Generate documentation
doc:
    cargo doc --no-deps --open

# Run cargo publish dry-run to verify package
publish-check:
    cargo publish --dry-run

# Bump version to patch (0.1.x)
version-patch:
    #!/usr/bin/env bash
    set -euo pipefail
    current=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
    echo "Current version: $current"
    major=$(echo $current | cut -d. -f1)
    minor=$(echo $current | cut -d. -f2)
    patch=$(echo $current | cut -d. -f3)
    new_patch=$((patch + 1))
    new_version="$major.$minor.$new_patch"
    echo "New version: $new_version"
    sed -i.bak "s/^version = \"$current\"/version = \"$new_version\"/" Cargo.toml
    rm Cargo.toml.bak
    echo "Version bumped to $new_version"

# Bump version to minor (0.x.0)
version-minor:
    #!/usr/bin/env bash
    set -euo pipefail
    current=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
    echo "Current version: $current"
    major=$(echo $current | cut -d. -f1)
    minor=$(echo $current | cut -d. -f2)
    new_minor=$((minor + 1))
    new_version="$major.$new_minor.0"
    echo "New version: $new_version"
    sed -i.bak "s/^version = \"$current\"/version = \"$new_version\"/" Cargo.toml
    rm Cargo.toml.bak
    echo "Version bumped to $new_version"

# Bump version to major (x.0.0)
version-major:
    #!/usr/bin/env bash
    set -euo pipefail
    current=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
    echo "Current version: $current"
    major=$(echo $current | cut -d. -f1)
    new_major=$((major + 1))
    new_version="$new_major.0.0"
    echo "New version: $new_version"
    sed -i.bak "s/^version = \"$current\"/version = \"$new_version\"/" Cargo.toml
    rm Cargo.toml.bak
    echo "Version bumped to $new_version"

# Tag and push a release (use after version bump)
release message="Release":
    #!/usr/bin/env bash
    set -euo pipefail
    version=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
    echo "Creating release for version $version"
    git add Cargo.toml Cargo.lock
    git commit -m "{{ message }} v$version"
    git tag "v$version"
    git push origin main
    git push origin "v$version"
    echo "Released v$version"

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run clippy
watch-lint:
    cargo watch -x "clippy -- -D warnings"

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Security audit (requires cargo-audit)
audit:
    cargo audit

# Show current version
version:
    @grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2

# Full CI check (what CI would run)
ci: lint test
    cargo build --release
    cargo publish --dry-run

# Development workflow
dev: lint test
    @echo "✓ All checks passed"
