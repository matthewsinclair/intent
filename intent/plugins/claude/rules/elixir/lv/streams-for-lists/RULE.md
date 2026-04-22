---
id: IN-EX-LV-002
language: elixir
category: lv
severity: warning
title: Use LiveView streams for large or dynamic lists
summary: >
  Do not assign full collections that grow, shrink, or update item-by-item.
  Use `stream/3`, `stream_insert/4`, and `stream_delete/3`. Only the diff is
  sent to the client. The full list never lives in socket assigns.
principles:
  - liveview-efficiency
applies_when:
  - "Rendering a list that can grow past ~20 items"
  - "A list that receives individual inserts (new messages, new rows from PubSub)"
  - "A list that needs item-level delete/update"
applies_to:
  - "lib/**/live/**/*.ex"
does_not_apply_when:
  - "Small, static lists rendered once and never updated (nav menus, short tabs)"
  - "Lists where every update replaces the whole list (search results that rebuild)"
  - 'Lists that must support client-side reordering (`phx-update="stream"` has specific idioms)'
references: []
related_rules:
  - IN-EX-LV-001
aliases: []
tags:
  - elixir
  - liveview
  - streams
  - performance
status: active
version: 1
---

# Use LiveView streams for large or dynamic lists

A LiveView that assigns a list of 500 messages re-sends the full list to the client on every update. A single new message means 500 items on the wire. Streams fix this: the full list never lives in socket assigns, only a server-side identifier, and only the delta (the new item, the removed item) goes to the client.

## Problem

Three failure modes with full-list assigns:

1. **O(n) diffs on every append.** LiveView's default diffing sees the assign as changed and re-sends the full list. 500 messages × a few KB each = multi-megabyte delta for one new message.
2. **Memory grows on the server.** Every socket holds every message it has ever seen. A long-lived LiveView tab with 10,000 messages holds 10,000 items in GenServer state per connected user.
3. **Stale re-renders.** Replacing the whole list to insert one item means every item re-renders (even if only one changed). For `<%= for %>` loops with expensive item components, this is noticeably slow.

Streams flip the model: items live in the template's DOM, identified by `dom_id`; the server tracks which items exist but not their full data. Updates go through `stream_insert`, `stream_delete`, `stream_config`.

## Detection

Signals:

- `assign(socket, :messages, [...])` where messages grow over time.
- `update(socket, :messages, &[new | &1])` or `List.insert_at` on an assign that is already a big list.
- Template loops on `@messages` where `@messages` comes from a growing assign.
- PubSub handlers that prepend/append to a list assign.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE 'update\([^,]+,[[:space:]]*:(messages|posts|items|events)' lib/**/live/
```

The reliable structural signal is "does this list grow or update item-by-item over the LiveView's lifetime?" If yes, use a stream.

## Bad

Inline:

```elixir
@impl true
def mount(_params, _session, socket) do
  {:ok, assign(socket, :messages, Chat.list_messages())}
end

@impl true
def handle_info({:new_message, msg}, socket) do
  {:noreply, update(socket, :messages, &[msg | &1])}
end
```

Template:

```heex
<ul>
  <li :for={msg <- @messages}>{msg.body}</li>
</ul>
```

Every new message re-sends the full list to the client. Memory on the server grows as messages accumulate in the assign.

## Good

Inline:

```elixir
@impl true
def mount(_params, _session, socket) do
  {:ok, stream(socket, :messages, Chat.list_messages())}
end

@impl true
def handle_info({:new_message, msg}, socket) do
  {:noreply, stream_insert(socket, :messages, msg, at: 0)}
end
```

Template:

```heex
<ul id="messages" phx-update="stream">
  <li :for={{dom_id, msg} <- @streams.messages} id={dom_id}>{msg.body}</li>
</ul>
```

Only the new item crosses the wire. The parent `<ul>` needs `id` and `phx-update="stream"`; every `<li>` needs `id={dom_id}`. LiveView handles the rest.

## When This Applies

- Chat logs, activity feeds, event streams.
- Paginated search results that stream additional pages.
- Admin tables with live updates from PubSub.
- Any list that could realistically exceed 50 items and receives updates.

## When This Does Not Apply

- **Static lists.** Navigation menus, category lists, filter options — rendered once, never updated.
- **Full-list replacement flows.** If every update rebuilds the entire list (search result sets where each query replaces everything), a regular assign is fine.
- **Tiny lists.** A list of 3 user roles does not need streaming; the overhead of stream wiring exceeds the benefit.

A good test: "if this list had 1000 items and gained one, what would go over the wire?" If the answer is "all 1000 items again", use a stream.

## Further Reading

- [Phoenix.LiveView — streams](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html#stream/4) — the language reference, including `stream_config` for limits and sort ordering.
- [Phoenix.LiveView — `phx-update="stream"`](https://hexdocs.pm/phoenix_live_view/bindings.html#phx-update) — the template-side contract.
- [Chris McCord — "Streams in LiveView"](https://www.phoenixframework.org/blog/phoenix-liveview-streams) — the design intent behind streams.
