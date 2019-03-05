# DQCsim-rs

DQCsim-rs: Delft Quantum Classical Simulator - the Rust edition

## Build

Requirements:

- [Rust](https://rustup.rs/) 1.33 or later.

Optional:

- [Clippy](https://github.com/rust-lang/rust-clippy)
- [Rustfmt](https://github.com/rust-lang/rustfmt)

### Debug

```bash
cargo build
```

### Release

```bash
cargo build --release
```

## Check

```bash
cargo check
```

## Run

```bash
cargo run -p <bin> -- <args>
```

## Test

```bash
cargo test
```

## Lint

```bash
cargo clippy
```

## Format

```bash
cargo fmt
```

## Docs

```bash
cargo doc -p <crate> --open
```
