---
id: IN-EX-TEST-004
upstream_id: start-supervised
language: elixir
category: test
severity: critical
title: Use start_supervised for process cleanup
summary: >
  Start processes in tests with `start_supervised!/2`. ExUnit registers them
  with the test supervisor and shuts them down when the test ends, regardless
  of outcome. Bare `GenServer.start_link` leaks processes between tests and
  causes mysterious cross-test failures.
principles:
  - async-default
  - thin-processes
applies_when:
  - "A test starts a GenServer, Agent, Task, or other long-lived process"
  - "A test needs a supervised process that should not outlive the test"
  - "Tests that register named processes or attach telemetry handlers"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Testing `start_link/1` itself — you need the raw result to assert on the pid/error"
  - "Testing name-registration conflicts where you deliberately start two processes with the same name"
  - "One-off Tasks whose entire lifetime is a synchronous `Task.await/1` in the test body"
references: []
related_rules:
  - IN-EX-TEST-003
aliases: []
tags:
  - elixir
  - testing
  - supervision
  - cleanup
status: active
version: 1
---

# Use start_supervised for process cleanup

`start_supervised!/2` registers the started process with ExUnit's per-test supervisor. When the test ends — pass, fail, or raise — ExUnit stops the process. Processes started with bare `GenServer.start_link` or `Agent.start_link` outlive the test and contaminate every test that follows.

## Problem

Three failure modes when tests leak processes:

1. **Name collisions.** Two tests start a GenServer with the same registered name. The second test crashes with `{:error, {:already_started, pid}}` — or worse, reuses the leftover process and the test's assumptions about initial state silently fail.
2. **Ghost state.** A leftover ETS-backed Agent accumulates data across tests. The failing test is not the one that started the process; diagnosis takes hours.
3. **Async interference.** In `async: true` runs, leftover processes from one test receive messages from another test's operations. Failures are non-deterministic and hard to reproduce.

These bugs are all cases of _locality violation_: the test that starts a process is not the test that suffers. `start_supervised!/2` restores locality — the supervisor guarantees the process is gone when the test ends.

## Detection

Signals:

- `GenServer.start_link`, `Agent.start_link`, or `Task.start_link` in a `setup`, `setup_all`, or test body, without `start_supervised`.
- `{:ok, pid} = MyServer.start_link(...)` in a test — should be `pid = start_supervised!(MyServer, ...)`.
- Missing `on_exit` cleanup for processes started with `start_link` (second-best fallback for code that cannot use `start_supervised!/2`).

**No greppable proxy is authoritative for this rule.** Without the `grep -v start_supervised` filter the headless mechanical runner refuses, every legitimate `start_supervised!` test (the _compliant_ form) would be flagged. The reliable structural signal is "is this process going to outlive the test that started it? If yes, supervise it." Apply this rule via the LLM-driven `critic-elixir` subagent during `/in-review`, not in the headless pre-commit gate.

## Bad

See `bad_test.exs` for the runnable form. Inline:

```elixir
setup do
  {:ok, pid} = MyCache.start_link([])
  %{cache: pid}
  # Process leaks when the test ends — next test may find it still running.
end
```

## Good

See `good_test.exs` for the runnable form. Inline:

```elixir
setup do
  pid = start_supervised!(MyCache)
  %{cache: pid}
  # ExUnit stops MyCache after this test regardless of outcome.
end
```

For processes that need args:

```elixir
setup do
  pid = start_supervised!({MyCache, name: :test_cache, ttl: 100})
  %{cache: pid}
end
```

## When This Applies

- Any process started in `setup`, `setup_all`, or the test body that should not outlive the test.
- Named processes especially — name conflicts are the most common symptom of leaked supervision.
- Ecto repos, Mox servers, Phoenix endpoints that should be per-test.

## When This Does Not Apply

- **Testing `start_link` directly.** If the test's subject _is_ the start sequence (`assert {:error, :already_started} = MyServer.start_link(name: :dup)`), you need the raw call result.
- **Testing name-registration conflicts.** When you deliberately start two processes under the same name to test the conflict, `start_supervised` can interfere.
- **Short-lived Tasks.** A `Task.async/1` + `Task.await/1` whose lifetime is scoped to a single test line does not need supervision; it terminates before the next line.

A good test: "if this process is still alive after this test returns, is that a bug?" If yes, use `start_supervised!/2`.

## Further Reading

- [elixir-test-critic upstream rule](https://github.com/iautom8things/elixir-test-critic/blob/main/rules/core/start-supervised/RULE.md) — `ETC-CORE-006`, the upstream source for this rule.
- [ExUnit.Callbacks — `start_supervised/2`](https://hexdocs.pm/ex_unit/ExUnit.Callbacks.html#start_supervised/2) — the runtime-option variant.
- [ExUnit.Callbacks — `start_supervised!/2`](https://hexdocs.pm/ex_unit/ExUnit.Callbacks.html#start_supervised!/2) — the bang variant that raises on failure (almost always what you want).
