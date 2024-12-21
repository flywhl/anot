default:
    @just --list

# Build the project
build:
    cargo build

dev:
    uv run maturin develop --uv

# Run the project
run:
    cargo run

# Build with optimizations
release:
    cargo build --release

# Run tests
test:
    cargo test

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Run rustfmt checker
fmt-check:
    cargo fmt -- --check

# Format code
fmt:
    cargo fmt

# Run all lints
lint:
    just clippy
    just fmt-check

# Check a specific file
lint-file file:
    cargo clippy -- {{file}}
    cargo fmt -- {{file}} --check

# Watch tests
watch:
    cargo watch -x test

