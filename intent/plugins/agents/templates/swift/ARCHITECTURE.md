# ARCHITECTURE-swift.md

Swift-specific architectural patterns and conventions for this project.

## Package Layout

Standard SwiftPM layout:

- `Package.swift` -- package manifest at root
- `Sources/<TargetName>/` -- per-target source
- `Tests/<TargetName>Tests/` -- per-target tests
- `.build/` -- build output (gitignored)

## Module Organisation

- One target per concern. Public surface declared via `public` keyword; everything else is `internal` by default.
- Test helpers live in a separate `<TargetName>TestHelpers` target; never alongside production code.
- Resources (assets, data) live under `Sources/<TargetName>/Resources/` declared in the manifest.

## Error Handling

- Define module-local error enums conforming to `Error`. Use associated values to carry context.
- Throwing functions propagate via `try` / `do/catch`. Reserve `Result<T, E>` for async APIs and explicit branching.
- Application-level errors (top-level entry points) may transform to user-facing presentation at the boundary.

## Concurrency

- Prefer `async/await` over completion handlers in new code.
- Use `actor` for shared mutable state. `@MainActor` for UI-bound work.
- Never block the main thread with synchronous I/O; use `Task { }` to offload.

## Testing

- Unit tests under `Tests/<TargetName>Tests/`; one test file per source file when practical.
- Use `XCTest` (or `swift-testing` for newer projects). Async tests use `await`.
- UI / integration tests live in a separate target.

## Build / CI

- `swift build` and `swift test` are the canonical CLI invocations.
- `swift-format lint --strict` is gating in CI when configured.
- Pre-commit critic gate runs `intent critic swift` on staged files (per Intent canon).
