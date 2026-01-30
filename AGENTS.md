# Agents

## Important

Before you do anything else, run the `peas prime` command and heed its output.

If you fail to call the `peas` executable, inform the user about it, they may install it for you. Wait also for feedback if they want to proceed without it, in which case you can continue your work and ignore any `peas` related instructions.

## Development

- Use `cargo build`, `cargo test`, `cargo run` (without release flag) for development and testing
- Use `cargo build --release` for production builds only; run this when instructed, otherwise ask the user if they want a production build
- Use `cargo clippy` to check for common mistakes and improve code quality
- Use `cargo fmt` to format your code according to Rust's style guidelines
