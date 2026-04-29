---
id: IN-EX-LV-003
language: elixir
category: lv
severity: warning
title: Thin LiveViews — delegate domain logic to the domain
summary: >
  LiveViews are coordinators. They assign state, dispatch events to domain
  functions, and update assigns from the result. No business logic, no
  aggregation queries, no data transformation that does not belong to the
  view.
principles:
  - thin-coordinator
applies_when:
  - "A LiveView event handler does more than parse → call → update assigns"
  - "A LiveView does domain queries beyond what is needed to render"
  - "A LiveView branches on domain state to decide business outcomes"
applies_to:
  - "lib/**/live/**/*.ex"
does_not_apply_when:
  - "Purely presentational computation (formatting a date, building a CSS class, ordering assigns for display)"
  - "Framework-level LiveView concerns (handling navigation, flashes, redirects)"
  - "LiveViews that are effectively components with tiny amounts of state"
references:
  - IN-AG-THIN-COORD-001
related_rules:
  - IN-EX-PHX-001
  - IN-EX-LV-001
aliases: []
tags:
  - elixir
  - liveview
  - thin-coordinator
status: active
version: 1
---

# Thin LiveViews

A LiveView is an HTTP controller with a socket. Everything that was true of `IN-EX-PHX-001` for controllers — parse, call, shape — is true here, with one addition: LiveViews also hold per-session state and react to events. That does not make them a home for business logic; it makes them the _coordinator_ that has more state to juggle. The domain is still where the logic lives.

## Problem

Three failure modes when LiveViews fatten:

1. **Domain logic invisible to non-LiveView callers.** A `handle_event("publish", ...)` that computes "is this publishable?" inline duplicates logic that Oban workers, controllers, and mix tasks all need. The workaround is usually copy-paste. The rule ends up in three places.
2. **Untestable via LiveViewTest.** Business logic inside `handle_event/3` can only be tested by simulating the event. The same logic as a domain function is testable by calling it directly, with real inputs and real expected outputs.
3. **Assign explosion.** LiveViews that do aggregation queries (`total_spend`, `active_users_by_role`, etc.) end up with 20 assigns because every stat lives in the view. The socket is carrying the data model instead of rendering it.

The discipline is identical to controllers: parse the event, call the domain, update assigns from the result, render. If the handler is more than five lines, it is doing too much.

## Detection

Signals:

- `handle_event/3` longer than ~10 lines.
- `if`, `case`, or `cond` on domain state inside a `handle_event` to decide what to update.
- Aggregation queries (`Repo.aggregate/3`, `Ash.Query.aggregate(...)`) inside `mount/3` or `handle_info/2`.
- Data transformation (group-by, pivot, percentile) in the LiveView's helpers.
- `Repo.*` or `Ash.*` calls outside a domain wrapper (see `IN-EX-ASH-001`).

**No greppable proxy is authoritative for this rule.** The signal — "a `handle_event/3` longer than ~15 lines, or `Repo.*` / `Ash.*` calls outside a domain wrapper" — requires line-counting state machines (awk) the headless mechanical runner deliberately rejects. The reliable structural signal is "would an Oban worker or a controller need this same logic?" Apply this rule via the LLM-driven `critic-elixir` subagent during `/in-review`, not in the headless pre-commit gate.

## Bad

Inline:

```elixir
@impl true
def handle_event("publish", %{"id" => id}, socket) do
  post = MyApp.Content.get_post!(id, actor: socket.assigns.current_user)

  if post.status == :draft and post.word_count > 100 do
    {:ok, updated} =
      MyApp.Content.update_post(post, %{status: :published, published_at: DateTime.utc_now()})

    Notifications.notify_subscribers(updated)
    Analytics.track("post_published", id: updated.id)

    {:noreply, stream_insert(socket, :posts, updated)}
  else
    {:noreply, put_flash(socket, :error, "Cannot publish")}
  end
end
```

Five concerns inline: read, validate, update, notify, track. A mix task needing to publish posts cannot reuse any of this.

## Good

Inline:

```elixir
@impl true
def handle_event("publish", %{"id" => id}, socket) do
  case MyApp.Content.publish_post(id, actor: socket.assigns.current_user) do
    {:ok, post} ->
      {:noreply, stream_insert(socket, :posts, post)}

    {:error, reason} ->
      {:noreply, put_flash(socket, :error, humanise(reason))}
  end
end
```

The handler is four lines of coordination. `MyApp.Content.publish_post/2` does validation, update, notification, and analytics — and any Oban worker or mix task can call it.

## When This Applies

- Every LiveView event handler, `handle_info/2`, and `mount/3`.
- Any LiveView that finds itself reaching for `Repo.*` or `Ash.*` primitives.

## When This Does Not Apply

- **Presentational computation.** `format_currency/1`, `status_badge_class/1`, `active_tab_class/2` — helpers that format rendered output. These live in the LiveView or a component module.
- **Navigation and flash orchestration.** `push_patch`, `push_navigate`, `put_flash` are LiveView-specific response mechanisms. Orchestrating them in handlers is coordination, not business logic.
- **Effectively-stateless components.** A tiny LiveView that is really a live component with a bit of session state may not have enough logic to move; leave it alone.

A good test: "could a mix task trigger this behaviour without going through a WebSocket?" If the behaviour is reusable outside LiveView, it belongs in the domain.

## Further Reading

- [Intent `IN-AG-THIN-COORD-001`](../../../agnostic/thin-coordinator/RULE.md) — the agnostic principle.
- [Intent `IN-EX-PHX-001` thin-controllers](../../phoenix/thin-controllers/RULE.md) — the controller-layer equivalent.
- [Intent `IN-EX-LV-001` two-phase-mount](../two-phase-mount/RULE.md) — the related LiveView-lifecycle rule.
- [Chris McCord — "Rethinking server-rendered apps"](https://www.phoenixframework.org/blog/the-road-to-live-view-1.0) — the LiveView-as-coordinator design intent.
