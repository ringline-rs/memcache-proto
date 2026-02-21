# CLAUDE.md

## Build & Test Commands

```bash
# Build
cargo build
cargo build --all-features

# Lint
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all
cargo test --all --all-features
cargo test --all --release

# Docs
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

## Architecture

Pure protocol parser/encoder for Memcache ASCII and binary protocols. No runtime dependencies.

### Features

- `ascii` (default) — ASCII text protocol
- `binary` — binary protocol
- `full` — enables both `ascii` and `binary`

### Key Types

- `Request` — parsed ASCII request
- `Response` — parsed ASCII response
- `Command` — command enum
- `StreamingParser` — incremental protocol decoder
- `binary::Request` / `binary::Response` — binary protocol types
- `binary::Header` — binary protocol header
