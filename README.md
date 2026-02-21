# memcache-proto

Memcache ASCII and binary protocol parser and encoder.

A standalone, zero-copy protocol implementation with no runtime dependencies.

## Features

- **ASCII protocol** — text-based memcache protocol (enabled by default)
- **Binary protocol** — binary memcache protocol via the `binary` feature flag
- Streaming parser for incremental protocol decoding

## Usage

```toml
[dependencies]
memcache-proto = "0.0.1"

# With binary protocol support:
memcache-proto = { version = "0.0.1", features = ["full"] }
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
