# Rust Password

A small Windows app for viewing local Rust and Steam registry data.

It displays:

- Rust's last entered code, hidden until revealed
- Rust console history, newest command first
- Active Steam account and last game name
- Rust installation and running state

## Run

```powershell
cargo run --release
```

## CLI

```powershell
cargo run --example scan
```

## Check

```powershell
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

Registry access is read-only and limited to the current Windows user.
