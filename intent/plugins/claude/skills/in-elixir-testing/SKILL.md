---
description: "Elixir testing rules: strong assertions, no control flow in tests, async by default, real code over mocks, Highlander for tests"
rules:
  - IN-EX-TEST-001
  - IN-EX-TEST-002
  - IN-EX-TEST-003
  - IN-EX-TEST-004
  - IN-EX-TEST-005
  - IN-EX-TEST-006
  - IN-EX-TEST-007
---

# Elixir Testing Essentials

Load the Intent Elixir test-rule pack into context. Invoke before writing ExUnit tests or during code review of test code.

**NEVER worry about test counts.** Do not count tests, do not report coverage percentages, do not set numeric targets. Quality over quantity, always. One strong test that proves a domain invariant is worth more than twenty shape tests that pass for any implementation.

## Procedure

### 1. Load the test rules

Read each `RULE.md` on demand when the situation matches. The full text lives in `intent/plugins/claude/rules/elixir/test/<slug>/RULE.md`.

| Rule ID          | Slug                           | What it enforces                                                      |
| ---------------- | ------------------------------ | --------------------------------------------------------------------- |
| `IN-EX-TEST-001` | `strong-assertions`            | Assert concrete values, never shape alone (`is_struct`, `is_map`).    |
| `IN-EX-TEST-002` | `no-process-sleep`             | Synchronise by message or call; never by wall-clock sleep.            |
| `IN-EX-TEST-003` | `async-by-default`             | Every test module declares `async: true` unless state forces opt-out. |
| `IN-EX-TEST-004` | `start-supervised`             | Use `start_supervised!/2`; bare `start_link` leaks processes.         |
| `IN-EX-TEST-005` | `no-control-flow-in-tests`     | Test bodies are straight-line; no `if`/`case`/`cond` inside.          |
| `IN-EX-TEST-006` | `real-code-over-mocks`         | Mock only at external boundaries; never your own modules.             |
| `IN-EX-TEST-007` | `test-highlander-shared-setup` | No duplicated setup; fixtures with async-safe identity.               |

Three rules (002, 003, 004) are upstream-derived from `elixir-test-critic`. See `intent/plugins/claude/rules/_attribution/elixir-test-critic.md` for full MIT attribution.

### 2. Additional operational conventions

Not yet first-class rules in the library:

- **One assertion focus per test.** Each test verifies one outcome. Name with `success:`, `failure:`, or `invariant:` prefix. If the test would need two `describe` blocks, it is two tests. (Implicit in IN-EX-TEST-001.)
- **Test the domain contract, not the implementation.** Test through public API (Ash code interfaces). Never test private functions. (Supports IN-EX-TEST-006.)
- **Spec-driven tests when a spec exists.** When `*_test.spec.md` exists, test names match spec assertions exactly — no tests without a spec counterpart, no spec assertions without a test. (Diogenes-adjacent discipline.)

### 3. Run the tests often

Tests only protect against regressions if they are green. Run `mix test` after each meaningful change; stop and fix before moving on.

## Red Flags

| Rationalisation                                     | Reality                                                           |
| --------------------------------------------------- | ----------------------------------------------------------------- |
| "Shape assertion is good enough to prove it works." | See IN-EX-TEST-001. Shape passes for any value of the right type. |
| "Process.sleep is the only way to sync this."       | See IN-EX-TEST-002. There is always a call, message, or monitor.  |
| "Sequential tests are fine for now."                | See IN-EX-TEST-003. "For now" becomes "forever" once coupled.     |
| "I'll mock Accounts to simplify this test."         | See IN-EX-TEST-006. Use the real Accounts; mock only the mailer.  |
| "Same 4 lines in every test is fine."               | See IN-EX-TEST-007. Fixture drift is how tests start lying.       |
