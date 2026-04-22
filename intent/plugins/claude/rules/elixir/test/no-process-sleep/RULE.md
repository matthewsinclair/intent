---
id: IN-EX-TEST-002
upstream_id: no-process-sleep
language: elixir
category: test
severity: critical
title: Never use Process.sleep for synchronisation
summary: >
  Do not use `Process.sleep/1` to wait for asynchronous work to complete.
  Synchronise by message (`assert_receive`), by call (`GenServer.call`), or
  by monitor. Sleeping for synchronisation produces tests that are flaky under
  load and arbitrarily slow on fast machines.
principles:
  - assert-not-sleep
applies_when:
  - "A test starts an async operation and then asserts on its side effects"
  - "A test waits for a message, state change, or side effect in another process"
  - "A test uses `Process.sleep` immediately before an assertion"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Rate limiter tests where `sleep` is testing actual wall-clock behaviour"
  - "Debounce tests where the system under test is time-based"
  - "Backoff/TTL/scheduled-job tests where elapsed time is the invariant"
references: []
related_rules:
  - IN-EX-TEST-003
aliases: []
tags:
  - elixir
  - testing
  - synchronisation
  - flakiness
status: active
version: 1
---

# Never use Process.sleep for synchronisation

`Process.sleep/1` pauses the caller for a wall-clock duration. Used as a way to "wait for the other process to finish", it is a race waiting to happen: shorter than the work under load and tests fail intermittently; longer than necessary and the suite crawls.

## Problem

Two failure modes, both inevitable at scale:

1. **Too short.** The sleep was tuned on the author's machine. CI runs hotter, or slower, or under load. The async operation has not completed when the assertion fires. The failure is intermittent and non-reproducible locally. Nobody believes it; a `retry` badge appears in CI config; the next flake is ignored by default.
2. **Too long.** The sleep is padded to make flakiness go away. Every test run now wastes the padded duration on every machine, regardless of actual work time. Fifty such tests sleeping 100ms each add five seconds to the suite — pure waste.

Both collapse into the same root cause: the test is guessing at duration instead of receiving confirmation. Elixir gives you confirmation for free via `assert_receive`, `GenServer.call`, or process monitors. Use those.

## Detection

Signals:

- `Process.sleep(N)` in a test body or test helper (outside the `does_not_apply_when` cases)
- `Process.sleep` followed by an assertion that reads state from another process
- `Process.sleep` followed by `assert_received`
- A comment above a sleep saying "wait for the GenServer" or "let it process"

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE 'Process\.sleep\(' test/
```

The reliable structural signal is "is this sleep trying to synchronise with work happening elsewhere?" If yes, replace it with an explicit synchronisation mechanism.

## Bad

See `bad_test.exs` for the runnable form. Inline:

```elixir
test "counter increments after cast" do
  {:ok, pid} = Counter.start_link(0)
  Counter.increment(pid)          # async cast
  Process.sleep(100)              # hope 100ms is enough
  assert Counter.value(pid) == 1  # sometimes fails on slow CI
end
```

## Good

See `good_test.exs` for the runnable form. Inline:

```elixir
test "counter increments after cast" do
  {:ok, pid} = Counter.start_link(0)
  Counter.increment(pid)          # async cast
  # The sync call cannot return until the preceding cast is processed.
  assert Counter.value(pid) == 1
end
```

For message-based work, wait for the message:

```elixir
test "worker notifies completion" do
  test_pid = self()
  Worker.process_async(test_pid)
  assert_receive {:done, _result}, 1000
end
```

## When This Applies

- Tests that start a GenServer, Task, or Agent and assert on its state or its side effects.
- Tests that use `Phoenix.PubSub`, `Registry`, or any pub/sub mechanism.
- Tests that cast to a process and then ask the process for state.

## When This Does Not Apply

- **Rate limiters.** Verifying that the limiter allows N requests per second requires actual elapsed time; `Process.sleep` is testing that behaviour, not masking a race.
- **Debounce / throttle.** The "quiet period" is the contract; sleeping is how you verify it.
- **Backoff and TTL.** Retry backoff schedules, TTL expiry, and scheduled jobs all have elapsed time in their contract.

A good test: "is the sleep there because I want to observe what happens after some real time, or because I want to give another process a chance to finish?" The second case is the antipattern.

## Further Reading

- [elixir-test-critic upstream rule](https://github.com/iautom8things/elixir-test-critic/blob/main/rules/core/no-process-sleep/RULE.md) — `ETC-CORE-005`, the upstream source for this rule.
- [ExUnit.Assertions — `assert_receive/3`](https://hexdocs.pm/ex_unit/ExUnit.Assertions.html#assert_receive/3) — the explicit-synchronisation replacement.
- [Saša Jurić — "The Soul of Erlang and Elixir"](https://www.youtube.com/watch?v=JvBT4XBdoUE) — the canonical synchronisation-patterns talk.
