# foodeq_be




## Setup

### Rust version
```bash
rustup check     
#stable-x86_64-unknown-linux-gnu - Update available : 1.82.0 (f6e511eec 2024-10-15) -> 1.88.0 (6b00bc388 2025-06-23)
#rustup - Update available : 1.27.1 -> 1.28.2
```

#### Folder structure

```bash
cargo new server
# copy generated files into root dir

cargo build
target/debug/server
# Hello, world!

cargo run
# Hello, world!
```

### Dependencies
```bash
cargo add tokio --features=full
cargo add axum --features=macros,http2,ws
cargo add serde --features=derive # de/serializing data
cargo add uuid --features=v4
```

## Dev

```bash
cargo run
```

### Debugger

Enable macros


## Tests
[docs/tests.md](docs/tests.md)

## Notes
- axum's macros features is used for debug handler
- serde's derive feature allows macros to add de/serialize traits to our custom structs
- axum's extractors does json de/serialization, so we don't need serde's json feature
- Functions and structs are private by default, add `pub` to make public
- `Option<>` allows None values, but can have values wrapped with `Some()`.
- Extractors are used for getting path parameters.