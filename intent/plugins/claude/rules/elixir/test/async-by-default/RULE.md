---
id: IN-EX-TEST-003
upstream_id: async-by-default
language: elixir
category: test
severity: warning
title: "`async: true` by default"
summary: >
  Every ExUnit test module should declare `async: true` unless it genuinely
  needs exclusive access to shared global state. Sequential-by-default hides
  isolation bugs that surface only when the suite grows large enough to
  parallelise. Declare async from day one; opt out only where required.
principles:
  - async-default
applies_when:
  - "Any ExUnit test module that does not mutate shared global state"
  - "Tests that use Ecto SQL Sandbox (supports async)"
  - "Tests that use Mox (async-safe with allowances)"
  - "Tests for pure functions, data transformations, or stateless logic"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Tests that call `Application.put_env/3` or `Application.delete_env/2`"
  - "Tests that write to named ETS tables shared across processes"
  - "Tests that depend on a singleton named process with no isolation"
  - "Tests that mutate global Logger or Telemetry configuration"
references: []
related_rules:
  - IN-EX-TEST-002
  - IN-EX-TEST-004
aliases: []
tags:
  - elixir
  - testing
  - async
  - isolation
status: active
version: 1
---

# `async: true` by default

`use ExUnit.Case, async: true` is the default state of a healthy test module. It parallelises across CPU cores and — more importantly — forces the test author to confront hidden coupling early, when the fix is cheap. A suite that drifts into sequential-by-default accumulates isolation debt that is miserable to repay.

## Problem

Three failure modes with sequential-by-default:

1. **Hidden coupling compounds.** Tests that share a named process, an ETS table, or a global env var pass individually and pass together in the current order. Change the order, the suite breaks. Add a test, the suite breaks. The fix is no longer "add `async: true`" — it is a refactor across many files.
2. **Slow suites train impatience.** A 40-second sequential suite that could be 6 seconds trains developers to not run it locally. Regressions slip into PRs; CI becomes the primary test runner; feedback lag grows.
3. **Opt-in is forgettable.** If the default is sequential, every new module is sequential unless the author remembers the flag. If the default is `async: true`, every new module is async unless the author remembers to opt out — and that "opt out" is now a decision, not an omission.

Declaring `async: true` on day one is the single cheapest isolation-discipline decision available.

## Detection

Signals:

- `use ExUnit.Case` without `, async: true` in any test module that does not touch global state.
- A test module whose only imports/aliases are pure functions (no `Application.put_env`, no named GenServers, no shared ETS).
- A test module with no `Application.put_env`, no `:telemetry.attach`, no singleton process, no Process dictionary writes.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnL 'use ExUnit\.Case, async: true' test/ | xargs grep -l 'use ExUnit\.Case' 2>/dev/null
```

The reliable structural signal is "does this module deliberately mutate global state?" If no, it should be `async: true`.

## Bad

See `bad_test.exs` for the runnable form. Inline:

```elixir
defmodule MyApp.MathTest do
  use ExUnit.Case   # missing `async: true` — runs sequentially for no reason

  test "adds two numbers" do
    assert MyApp.Math.add(1, 2) == 3
  end
end
```

## Good

See `good_test.exs` for the runnable form. Inline:

```elixir
defmodule MyApp.MathTest do
  use ExUnit.Case, async: true

  test "adds two numbers" do
    assert MyApp.Math.add(1, 2) == 3
  end
end
```

For modules that genuinely need exclusive access, opt out deliberately with a comment:

```elixir
defmodule MyApp.GlobalConfigTest do
  # Mutates Application env — must run sequentially to avoid leaking between tests.
  use ExUnit.Case, async: false
end
```

## When This Applies

- Test modules for pure functions, data transformations, or stateless logic.
- Tests that use Ecto SQL Sandbox (async-safe by design).
- Tests that use Mox (async-safe with `Mox.set_mox_from_context/1` + allowances).
- Tests that use `start_supervised!/2` (the supervisor is per-test and safe for async).

## When This Does Not Apply

- **Global env mutation.** `Application.put_env/3` or `Application.delete_env/2` at the test level.
- **Shared ETS tables.** Writing to a named ETS table without per-test isolation.
- **Singleton named processes.** Tests that depend on a single, globally-named GenServer with no instance-per-test mechanism.
- **Global Logger / Telemetry state.** Tests that reconfigure the Logger backend or attach/detach telemetry handlers for the suite duration.

A good test: "would two copies of this test running simultaneously interfere?" If yes, opt out of async and add a comment explaining which shared resource forces the decision. If no, `async: true`.

## Further Reading

- [elixir-test-critic upstream rule](https://github.com/iautom8things/elixir-test-critic/blob/main/rules/core/async-by-default/RULE.md) — `ETC-CORE-001`, the upstream source for this rule.
- [ExUnit.Case — `async` option](https://hexdocs.pm/ex_unit/ExUnit.Case.html) — language reference.
- [Mox docs — `set_mox_from_context/1`](https://hexdocs.pm/mox/Mox.html#set_mox_from_context/1) — how to keep Mox tests async.
