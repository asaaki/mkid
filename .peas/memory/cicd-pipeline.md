+++
key = "cicd-pipeline"
tags = [
    "cicd",
    "github-actions",
]
created = "2026-01-30T19:09:08.421826Z"
updated = "2026-01-30T19:09:08.421826Z"
+++

Complete CI/CD pipeline setup using GitHub Actions. CI workflow runs on push/PR/tags, tests on Ubuntu/macOS/Windows with formatting, clippy, build, and tests. Release workflow waits for CI, builds multi-platform binaries (x86_64-unknown-linux-gnu, aarch64-apple-darwin, x86_64-pc-windows-msvc), creates GitHub releases with artifacts, and publishes to crates.io. Justfile provides automation for version bumps and releases. Cargo.toml includes binstall metadata for binary installation.
