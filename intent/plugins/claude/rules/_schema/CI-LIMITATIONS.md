# CI Limitations for Runnable Examples

Intent's rule library supports runnable good / bad examples for some languages and textual-only examples for others. This document records which is which in v2.9.0, why, and what it means for rule authoring and validation.

## Runnable-examples matrix (v2.9.0)

| Language | Runnable examples | File convention                                                                  | Validator                                     |
| -------- | :---------------: | -------------------------------------------------------------------------------- | --------------------------------------------- |
| Elixir   |        Yes        | `good_test.exs`, `bad_test.exs` (test rules); `good.exs`, `bad.exs` (code rules) | `mix test` via `intent claude rules validate` |
| Shell    |       Mixed       | Optional `good.sh` / `bad.sh` where feasible; fenced blocks otherwise            | `bash -n` / `zsh -n` syntax check             |
| Agnostic |        N/A        | No examples; `concretised_by:` language-specific rules                           | —                                             |
| Rust     |   Textual only    | Fenced code blocks in `## Bad` / `## Good` sections of RULE.md                   | Syntax review only                            |
| Swift    |   Textual only    | Fenced code blocks in `## Bad` / `## Good` sections of RULE.md                   | Syntax review only                            |
| Lua      |   Textual only    | Fenced code blocks in `## Bad` / `## Good` sections of RULE.md                   | Syntax review only                            |

## Why Elixir is runnable and the others are not

Intent runs on macOS. Elixir (via Homebrew) is a first-class development dependency — Intent itself uses Elixir subagents, Credo checks, and Elixir rule authoring. The validator can assume `mix` is on PATH.

Rust, Swift, and Lua are in-scope languages for Critic subagents, but the Intent repo does not mandate Rust / Swift / Lua toolchains as dev dependencies. Requiring them:

- Adds to the local-setup burden for Intent contributors.
- Complicates CI (which would need three additional language runtimes).
- Locks Intent to specific Rust editions / Swift versions / Lua dialects, creating version drift maintenance.

For v2.9.0, the cost-benefit falls on "textual examples are enough". Critic subagents perform Detection against real project files at invocation time; the rule's good / bad serve as teaching examples for Claude and human readers, not as validation fixtures.

## What textual-only means in practice

### Authoring

For a Rust / Swift / Lua rule:

- `## Bad` section contains a fenced code block in the target language.
- `## Good` section contains a fenced code block in the target language.
- No separate `.rs` / `.swift` / `.lua` files alongside `RULE.md`.
- The code inside the fence must be syntactically plausible. Reviewer eyeballs it; Intent does not compile it.

Example — Rust rule:

````markdown
## Bad

```rust
fn load(id: u32) -> User {
  let user = db.find(id).unwrap();  // panics on missing
  user
}
```

## Good

```rust
fn load(id: u32) -> Result<User, Error> {
  db.find(id).ok_or(Error::NotFound)
}
```
````

### Validation

`intent claude rules validate` for Rust / Swift / Lua rules:

- Checks that `## Bad` and `## Good` sections exist.
- Checks that each section contains at least one fenced code block.
- Checks the fence's language tag (` ```rust `, ` ```swift `, ` ```lua `) matches the rule's `language:` frontmatter.
- Does NOT compile, parse, or lint the example code.

Syntactic errors in examples are caught by human review or by Critic subagents noticing drift at invocation time.

### What Critics do with these rules

The Critic subagent reads RULE.md, extracts the Detection heuristic, and applies it to real source files in a real project. Detection does not depend on the examples being runnable; the examples are documentation, not fixtures.

## Future work (explicitly deferred)

Runnable examples for Rust / Swift / Lua would require:

1. **Tooling dependencies**: Rust (`cargo test`), Swift (`swift test`), Lua (`busted` or `luaunit`) available in Intent's dev environment and CI.
2. **Per-language validator glue**: extending `intent claude rules validate` to dispatch to the right runner for non-Elixir languages.
3. **File conventions**: agreeing on `good.rs` / `bad.rs` structures that run in isolation (probably as `#[test]` modules or cargo-managed small crates).
4. **CI environment**: GitHub Actions workflows per language, with cache / toolchain setup.

This is a future-ST concern. For v2.9.0 the textual-only convention is stable; upgrading to runnable does not require re-authoring existing rule content — only adding sibling files and validator plumbing.

## Consequences for rule authors

If you are authoring a Rust / Swift / Lua rule:

- Your Bad / Good examples are the authoritative teaching material. Make them convincing.
- Cite canonical sources in "Further Reading": Rust API Guidelines, Clippy lint names, Swift Evolution proposal numbers, Lua style guides. External references compensate for the lack of runtime validation.
- When a Critic later flags a false positive because your example was ambiguous, update the Detection section, not the example.

If you are authoring an Elixir rule:

- Your good / bad `.exs` files must exit 0 when run (upstream convention — see `rule-schema.md` "Exit code contract").
- Run them locally before committing: `mix test good_test.exs && mix test bad_test.exs`.
- The validator (WP02) runs them in CI on each change.

## Consequences for Critics

Each language Critic (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`) reads RULE.md and extracts:

- The Detection heuristic (grep pattern, structural signal) from the `## Detection` section.
- Bad / Good examples for reference when composing findings.

Critics don't run good / bad examples. They apply Detection to the target file provided at invocation time. The runnable-vs-textual distinction does not affect Critic behaviour.
