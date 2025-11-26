# AGENTS.md - Development Guidelines for nodana-cli-rust

## Commands
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test all**: `cargo test`
- **Test single**: `cargo test test_function_name`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`
- **Check**: `cargo check`

## Code Style
- **Formatting**: Use `cargo fmt` (rustfmt) for consistent formatting
- **Linting**: Use `cargo clippy` for code quality checks
- **Naming**: snake_case for functions/variables, PascalCase for types/structs
- **Error Handling**: Use `Result<T, E>` and `Option<T>`, avoid unwrap() in production code
- **Imports**: Group std library imports first, then external crates, then local modules
- **Types**: Prefer explicit types over inference for public APIs
- **Documentation**: Use `///` for public items, `//!` for modules