# ARCHITECTURE-rust.md

Rust-specific architectural patterns and conventions for this project.

## Crate Layout

Standard Cargo workspace layout:

- `Cargo.toml` -- workspace manifest at root
- `crates/<name>/` -- per-crate source
- `crates/<name>/src/lib.rs` -- crate entry
- `crates/<name>/src/bin/` -- binary targets
- `crates/<name>/tests/` -- integration tests
- `target/` -- build output (gitignored)

## Module Organisation

- One concern per module. Re-export the public surface from `lib.rs`.
- Internal helpers live in `crate::internal::*` and are not re-exported.
- Test fixtures live under `tests/fixtures/`; never under `src/`.

## Error Handling

- Crate-local error enum derived via `thiserror` (or hand-rolled) -- never `Box<dyn Error>` in public APIs.
- Application-level errors (in `bin/`) may use `anyhow::Result` for ergonomic propagation.
- `Result` propagation uses `?`. Reserve `match` for conditional error transformation.

## Async Runtime

- Pick one runtime per workspace (typically `tokio`). Never mix `tokio` and `async-std` in the same crate graph.
- Async functions return `impl Future` or are marked `async fn`. Avoid `Pin<Box<dyn Future>>` in public APIs unless erasure is required.

## Testing

- Unit tests live in `#[cfg(test)] mod tests` blocks at the bottom of each module.
- Integration tests live in `crates/<name>/tests/` -- one file per integration scenario.
- Property tests use `proptest` or `quickcheck`; document the invariant being checked.

## Build / CI

- `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings` are gating in CI.
- Pre-commit critic gate runs `intent critic rust` on staged files (per Intent canon).
