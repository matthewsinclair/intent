---
id: IN-EX-LV-001
language: elixir
category: lv
severity: critical
title: Two-phase LiveView mount — guard async work with `connected?/1`
summary: >
  `mount/3` is called twice: once for the static HTML render (disconnected),
  once for the WebSocket (connected). Never subscribe to PubSub, start timers,
  or spawn async work during the static render. Guard every side-effecting
  mount-time operation with `if connected?(socket)`.
principles:
  - liveview-lifecycle
applies_when:
  - "Any `mount/3` that subscribes to `Phoenix.PubSub`"
  - "Any `mount/3` that starts a timer (`Process.send_after/3`, `:timer.send_interval/2`)"
  - "Any `mount/3` that spawns a Task or sends a message to another process"
  - "Any `mount/3` that does a slow external call suitable for `assign_async`"
applies_to:
  - "lib/**/live/**/*.ex"
does_not_apply_when:
  - "Idempotent, cheap mount-time work (e.g. assigning derived values from params)"
  - "Operations that deliberately need to run on both disconnected and connected mounts (rare)"
references: []
related_rules:
  - IN-EX-LV-003
aliases: []
tags:
  - elixir
  - liveview
  - lifecycle
  - mount
status: active
version: 1
---

# Two-phase LiveView mount

Every LiveView mounts twice. First, Phoenix renders the static HTML server-side to bootstrap the page — that is disconnected mount, no socket, no live process. Then the client opens a WebSocket and LiveView mounts again — that is connected mount, the one where subscriptions and timers make sense. Doing WebSocket-only work in the disconnected mount double-subscribes, double-spawns, and leaks processes.

## Problem

Three failure modes when disconnected mount does WebSocket-only work:

1. **Double subscription.** `Phoenix.PubSub.subscribe(...)` runs in the disconnected mount, but that process exits when the static render completes. The connected mount subscribes again. Now there is an unowned subscription from the disconnected phase that the system does not know to clean up — though in practice the process termination clears it, the double work is wasteful.
2. **Leaked timers and tasks.** `Process.send_after(self(), :tick, 1000)` in the disconnected mount sends a message to a process that is about to exit. The message is lost, and if the author assumed a tick would arrive for the first render, the UI is wrong for a frame.
3. **Duplicated external calls.** An expensive API call (`Stripe.list_charges/0`) runs in the disconnected mount, then runs again in the connected mount. The user is billed twice the API budget for one page load.

The fix is always the same: guard with `connected?(socket)`. Subscriptions, timers, and async spawns happen once, on the connected phase.

## Detection

Signals:

- `Phoenix.PubSub.subscribe/2` at the top of `mount/3` with no surrounding `if connected?(socket)`.
- `Process.send_after/3` or `:timer.send_interval/2` directly in `mount/3`.
- `Task.async/1`, `Task.start/1`, or `spawn/1` in `mount/3` without the guard.
- A slow external HTTP call in `mount/3` that should be wrapped in `assign_async/3` instead.

**No greppable proxy is authoritative for this rule.** The signal — "a side-effecting call inside `mount/3` not preceded by `connected?(socket)`" — requires `-B5` context plus a negative filter that the headless mechanical runner cannot honour. The reliable structural signal is "if this work runs on the disconnected render, is the effect wasted or duplicated?" Apply this rule via the LLM-driven `critic-elixir` subagent during `/in-review`, not in the headless pre-commit gate.

## Bad

Inline:

```elixir
@impl true
def mount(_params, _session, socket) do
  # Runs on BOTH disconnected and connected mounts.
  Phoenix.PubSub.subscribe(MyApp.PubSub, "updates:#{socket.assigns.current_user.id}")
  Process.send_after(self(), :tick, 1000)

  items = Stripe.list_charges()  # expensive, runs twice

  {:ok, assign(socket, items: items)}
end
```

## Good

Inline:

```elixir
@impl true
def mount(_params, _session, socket) do
  if connected?(socket) do
    Phoenix.PubSub.subscribe(MyApp.PubSub, "updates:#{socket.assigns.current_user.id}")
    Process.send_after(self(), :tick, 1000)
  end

  {:ok,
   socket
   |> assign(:items, [])
   |> assign_async(:items, fn -> {:ok, %{items: Stripe.list_charges()}} end)}
end
```

Subscriptions and timers are behind the guard. The slow external call runs via `assign_async/3`, which itself only fires on connected mount.

## When This Applies

- Every LiveView `mount/3` that subscribes, schedules, or spawns.
- Any LiveView touching external services during initial load.

## When This Does Not Apply

- **Pure assignment.** `mount/3` that only assigns params or session values does not need the guard.
- **Genuinely bi-phase operations.** A LiveView that intentionally mounts differently on SSR vs WebSocket (rare; almost always a smell).

A good test: "if this work ran on the disconnected render, would it be wasted, duplicated, or leaky?" If yes, guard with `connected?/1`.

## Further Reading

- [Intent `IN-EX-LV-003` thin-liveviews](../thin-liveviews/RULE.md) — keeping `mount/3` short makes this rule easier to enforce.
- [Phoenix.LiveView — `connected?/1`](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html#connected?/1) — the language reference.
- [Phoenix.LiveView — `assign_async/3`](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html#assign_async/3) — the non-blocking data-loading mechanism.
