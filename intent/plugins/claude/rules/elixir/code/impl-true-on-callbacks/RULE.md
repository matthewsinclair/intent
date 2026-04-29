---
id: IN-EX-CODE-003
language: elixir
category: code
severity: warning
title: "@impl true on behaviour callbacks"
summary: >
  Every function that implements a behaviour callback must be annotated with
  `@impl true`. The annotation catches typos in the callback name at compile
  time and makes callback-vs-custom distinction self-evident.
principles:
  - compile-time-enforcement
applies_when:
  - "A module `use`s a behaviour (GenServer, LiveView, Phoenix.Controller, Plug, Ash.Resource.Change, etc.)"
  - "A module implements a behaviour with `@behaviour MyBehaviour`"
  - "A function name matches a known callback name for a behaviour the module uses"
applies_to:
  - "lib/**/*.ex"
does_not_apply_when:
  - "Plain custom functions in a module that does not `use` or `@behaviour` anything"
  - "Private helper functions (`defp`)"
  - "Functions in modules using a behaviour whose callbacks are not being implemented by that function"
references: []
related_rules:
  - IN-EX-CODE-001
aliases: []
tags:
  - elixir
  - callbacks
  - compile-time
status: active
version: 1
---

# `@impl true` on behaviour callbacks

An unannotated callback is a bug waiting to happen. `handel_event/3` (one-letter typo) compiles cleanly, silently fails to dispatch, and you spend ten minutes staring at a LiveView that does nothing when the user clicks. `@impl true` turns the typo into a compile-time error and makes the callback obvious to the next reader.

## Problem

Three failure modes when callbacks are unannotated:

1. **Typo compiles.** Elixir does not know `handel_event/3` was meant to be `handle_event/3`. The compiler accepts it as a custom function; the behaviour's dispatch never finds it; the user sees a silent no-op. With `@impl true`, the compiler emits `warning: got "@impl true" for function handel_event/3 but no behaviour specifies this callback` and you know immediately.
2. **Reader cannot tell callbacks from helpers.** In a 400-line LiveView with `handle_event`, `handle_info`, `apply_action`, `load_posts`, `normalise_filters`, a reader cannot tell which are callbacks and which are custom without chasing behaviour documentation. `@impl true` is a visual landmark.
3. **Behaviour changes go unnoticed.** When a library renames a callback or deprecates one, `@impl true` is the signal. Without it, you might be implementing a dead callback for years.

`@impl true` costs one line and buys compile-time safety on every callback.

## Detection

Signals:

- A module with `use GenServer`, `use Phoenix.LiveView`, `use Phoenix.Controller`, `@behaviour MyBehaviour`, or similar.
- Functions matching the behaviour's known callback names (`init/1`, `handle_call/3`, `mount/3`, `render/1`, `handle_event/3`, `action/2`, `change/3`, `atomic/3`).
- No `@impl true` directly above the `def`.

**No greppable proxy is authoritative for this rule.** The signal — "a `def` of a known callback name without an `@impl true` immediately above" — requires `-B1` context plus a negative filter (`grep -v '@impl'`) that the headless mechanical runner cannot honour. The reliable structural signal is "does this `def` implement a callback of any behaviour this module uses?" Apply this rule via the LLM-driven `critic-elixir` subagent during `/in-review`, not in the headless pre-commit gate.

## Bad

See `bad.exs` for the runnable form. Inline:

```elixir
defmodule MyServer do
  use GenServer

  def init(state), do: {:ok, state}
  def handle_call(:ping, _from, state), do: {:reply, :pong, state}
end
```

No `@impl true`. A typo (`def handle_cal(...)`) would compile silently and never be invoked.

## Good

See `good.exs` for the runnable form. Inline:

```elixir
defmodule MyServer do
  use GenServer

  @impl true
  def init(state), do: {:ok, state}

  @impl true
  def handle_call(:ping, _from, state), do: {:reply, :pong, state}
end
```

A typo now fails at compile time: `got "@impl true" for function handle_cal/3 but no behaviour specifies this callback`.

For multiple behaviours in one module, use `@impl BehaviourName` to disambiguate:

```elixir
defmodule MyWorker do
  use Oban.Worker, queue: :default
  @behaviour MyApp.Retrier

  @impl Oban.Worker
  def perform(%Oban.Job{args: args}), do: handle(args)

  @impl MyApp.Retrier
  def retry_delay(attempt), do: :math.pow(2, attempt) * 1000 |> trunc()
end
```

## When This Applies

- Any module that `use`s a Phoenix/Ecto/Ash/Oban/library behaviour that has callbacks.
- Any module declaring `@behaviour MyApp.SomeContract`.
- Custom change/validation modules in Ash (`Ash.Resource.Change` with `change/3` or `atomic/3`).
- Plugs (`init/1`, `call/2`).

## When This Does Not Apply

- **Plain Elixir modules.** A utility module with no behaviour has no callbacks; there is nothing to annotate.
- **Private helpers.** `defp` cannot be a callback; `@impl true` is invalid on `defp`.
- **Non-callback functions on a behaviour module.** A LiveView may have `apply_action/3` that is not a callback — leave it unannotated. Annotate only functions that actually match callback names.

A good test: "if I rename this function by one letter, should the compiler complain?" If yes, it is a callback and needs `@impl true`.

## Further Reading

- [Elixir docs — behaviours](https://hexdocs.pm/elixir/typespecs.html#behaviours) — the language reference, including `@impl` semantics.
- [Phoenix LiveView — lifecycle](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html) — the authoritative list of LiveView callbacks that need annotating.
- [Ash Framework — changes](https://hexdocs.pm/ash/changes.html) — `change/3` and `atomic/3` are the most-missed Ash callbacks.
