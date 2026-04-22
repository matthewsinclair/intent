---
id: IN-RS-CODE-004
language: rust
category: code
severity: warning
title: thiserror for libraries, anyhow for applications
summary: >
  Libraries define structured error types so callers can `match` on variants.
  Binaries use `anyhow` to aggregate errors from anywhere with context. Never
  use `Box<dyn Error>` in a public library API, never use `String` as the
  error type.
principles:
  - no-silent-errors
applies_when:
  - "Designing the error type for a new module or crate"
  - "Converting errors from upstream libraries (`io::Error`, `serde_json::Error`, ...)"
  - "Returning `Result<T, String>` or `Result<T, Box<dyn Error>>`"
applies_to:
  - "src/**/*.rs"
does_not_apply_when:
  - "Tiny single-file scripts where an ad-hoc error type is more ceremony than value"
  - "Prototypes not yet stabilised — `anyhow` everywhere is fine during exploration"
  - "Errors explicitly modelled as data (state machines where errors are expected return values)"
references:
  - IN-AG-NO-SILENT-001
  - IN-RS-CODE-001
related_rules:
  - IN-RS-CODE-001
aliases: []
tags:
  - rust
  - error-handling
  - thiserror
  - anyhow
status: active
version: 1
---

# thiserror for libraries, anyhow for applications

Pick the error strategy that matches the audience. Library callers need to match variants; binary operators just need a readable backtrace with context.

## Problem

Rust's error ergonomics split along the library / application axis:

- A library whose errors are `Result<T, String>` forces every caller to parse strings to decide whether to retry, give up, or show a user-facing message. The information is there, but it is not _queryable_.
- A library whose errors are `Box<dyn Error>` gives the caller no way to distinguish recoverable errors from terminal ones without downcasting by concrete type, which defeats the abstraction.
- An application using `thiserror` for every error type drowns in handwritten `From` impls and variant boilerplate just to pass errors up to `main()`, which is going to print them anyway.

The split is: libraries need _structure_ (variants to match on, kind-tests, error-code enums). Applications need _context_ (what was I doing when this failed? `.context("loading config")`). `thiserror` gives libraries the structure cheaply. `anyhow` gives applications the context cheaply. Mixing them up leads to either ceremony-heavy application code or opaque library APIs.

## Detection

Static signals:

- Library code (`src/lib.rs` or a library crate) returning `Result<T, Box<dyn Error>>` or `Result<T, String>` from public functions.
- Binary code (`src/main.rs` or `src/bin/*.rs`) defining elaborate error enums with `thiserror` when the errors only ever flow to `fn main() -> anyhow::Result<()>`.
- Missing `#[from]` conversions where manual `map_err` forwarding could be a one-line derive.
- Error messages that lose the underlying cause: `return Err(MyError::Io)` discarding the `io::Error`'s detail.

## Bad

```rust
// A library returning String errors.
pub fn load_config(path: &str) -> Result<Config, String> {
  let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
  let config = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
  Ok(config)
}

// A binary defining thiserror variants for every error.
#[derive(thiserror::Error, Debug)]
enum AppError {
  #[error(transparent)]
  Config(#[from] ConfigError),
  #[error(transparent)]
  Http(#[from] reqwest::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

fn main() -> Result<(), AppError> { run() }
```

The library's caller cannot tell "file missing" from "malformed JSON". The binary adds types that only ever exist to satisfy `?` inside `main`.

## Good

```rust
// Library: structured errors with thiserror.
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
  #[error("config file not readable: {0}")]
  Io(#[from] std::io::Error),
  #[error("config is not valid JSON: {0}")]
  Parse(#[from] serde_json::Error),
}

pub fn load_config(path: &str) -> Result<Config, ConfigError> {
  let bytes = std::fs::read(path)?;
  let config = serde_json::from_slice(&bytes)?;
  Ok(config)
}

// Binary: anyhow with context.
use anyhow::{Context, Result};

fn main() -> Result<()> {
  let config = load_config("config.json").context("starting app")?;
  run(&config).context("running main loop")?;
  Ok(())
}
```

Library callers can `match err { ConfigError::Io(_) => ..., ConfigError::Parse(_) => ... }`. The binary's users see "Error: starting app: config file not readable: ..." with full chain on `{:?}`.

## When This Applies

- Crate with `lib.rs`: use `thiserror` for public error enums. One per module is fine; one per function is overkill.
- Crate with `main.rs` or `bin/`: use `anyhow::Result` for `main()` and internal application-layer functions.
- Workspace with both: the library crate uses `thiserror`, the binary crate adds `anyhow` and lets `?` convert between them.

## When This Does Not Apply

- Single-file `bin` scripts where the error type does not matter; `unwrap` or `?` with `Box<dyn Error>` is acceptable.
- Error-as-data patterns: an `enum ValidationError` that is part of the successful return value of a validator is not an error type in the `Result` sense.
- FFI boundaries where the error must be a C-compatible integer code.

## Further Reading

- `thiserror` crate docs (<https://docs.rs/thiserror>)
- `anyhow` crate docs (<https://docs.rs/anyhow>)
- "Error Handling in Rust" — BurntSushi (<https://blog.burntsushi.net/rust-error-handling/>)
- Rust API Guidelines, "Error types" (<https://rust-lang.github.io/api-guidelines/dependability.html#error-types-are-meaningful-and-well-behaved-c-good-err>)
- IN-RS-CODE-001 — `Result` over panic is the foundation this rule builds on
