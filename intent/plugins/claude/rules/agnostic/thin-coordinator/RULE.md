---
id: IN-AG-THIN-COORD-001
language: agnostic
category: architecture
severity: critical
title: Thin Coordinator
summary: >
  Coordinators — controllers, LiveViews, CLI dispatchers, event handlers,
  job runners — parse inputs, call a domain function, and format the result.
  They do not compute, validate deeply, aggregate, or orchestrate
  multi-step logic inline. Business decisions live in services or domains.
principles:
  - thin-coordinator
  - pfic
applies_when:
  - "HTTP controllers, LiveView callbacks, CLI command handlers, Oban workers, GenServer callbacks"
  - "Any adapter layer that sits between a transport (HTTP, WebSocket, argv, queue) and the domain"
  - "Refactor review: a coordinator function body exceeds ~20 lines or contains more than one non-trivial branch"
does_not_apply_when:
  - "Trivial scripts whose entire purpose is transport-level plumbing"
  - "Prototype sketches that have not yet earned a domain boundary"
  - "Adapters translating between wire formats where the translation is the only interesting thing"
references:
  - IN-AG-PFIC-001
related_rules:
  - IN-AG-HIGHLANDER-001
  - IN-AG-NO-SILENT-001
concretised_by:
  - IN-EX-PHX-001
  - IN-EX-LV-003
aliases: []
status: active
version: 1
---

# Thin Coordinator

Coordinators parse, call, and format. They do not decide. Business logic lives behind a domain boundary where it is testable without invoking HTTP, WebSockets, or argv parsing.

## Problem

A fat coordinator couples transport concerns to domain concerns. Once they are entangled, three failures follow:

1. **The domain is not testable standalone.** To test whether "publishing a draft post with fewer than 100 words is rejected," the tester must either stand up a full Phoenix endpoint and drive it with a simulated HTTP request, or duplicate the logic they wanted to test into a test fixture. Both are wrong: the first is slow and brittle, the second is a Highlander violation waiting to happen.
2. **Transport changes cascade into domain changes.** Porting from REST to GraphQL, or adding a CLI for the same feature, becomes a rewrite rather than a new adapter. If the controller knows how orders settle, only the controller knows; the business logic has no independent existence.
3. **Reviewers lose the forest.** A 120-line controller action mixing param parsing, authorization, validation, database access, side effects, and response formatting is unreadable. Reviewers approve or reject based on style, not correctness.

The cure is a hard discipline: the coordinator does three things and stops. It (1) extracts inputs from the transport layer, (2) calls one domain function with those inputs, (3) translates the domain result back to the transport format. If it is doing a fourth thing, that fourth thing belongs in the domain.

## Detection

Signals in the coordinator:

- Function body longer than ~20 lines (heuristic, not a hard limit).
- More than one non-trivial `if/case/cond` branch.
- A database access, external API call, or file I/O in the coordinator body.
- Multi-step orchestration: fetch, validate, transform, save, notify — all inline.
- Business rule constants (minimum word counts, tax rates, validity thresholds) appearing as literals in the coordinator.
- A test for the coordinator that asserts on domain outcomes rather than transport outcomes (the test is secretly testing the domain; the coordinator is secretly holding the domain).

Signals at the boundary:

- The domain has no function that matches the operation the coordinator is performing. The operation exists only inside the coordinator.
- The same orchestration appears in a controller, a LiveView, and a CLI — three copies, diverging over time (Highlander violation as a symptom of Thin Coordinator violation).

## Bad

```
# Phoenix controller action — fat coordinator.
def publish(conn, %{"id" => id}) do
  post = Content.get_post!(id)
  if post.status == :draft and post.word_count >= 100 do
    {:ok, updated} = Content.update_post(post, %{
      status: :published,
      published_at: DateTime.utc_now()
    })
    Notifications.send_post_published(updated, conn.assigns.current_user)
    Analytics.record(:post_published, %{post_id: updated.id})
    conn
    |> put_flash(:info, "Post published")
    |> redirect(to: ~p"/posts/#{updated.id}")
  else
    conn
    |> put_flash(:error, "Cannot publish: post must be a draft with 100+ words.")
    |> redirect(to: ~p"/posts/#{post.id}/edit")
  end
end
```

Validation rules, state transitions, notifications, analytics, and flash messages are all in the controller. Porting to a GraphQL mutation means rewriting the rules; testing the "drafts under 100 words stay drafts" invariant requires a `conn` and a session.

## Good

```
# Domain function — does the work, returns a tagged result.
def publish_post(id, actor: actor) do
  with {:ok, post} <- get_post(id),
       :ok <- ensure_draft(post),
       :ok <- ensure_min_word_count(post, 100),
       {:ok, published} <- update_post(post, %{status: :published, published_at: DateTime.utc_now()}),
       :ok <- Notifications.send_post_published(published, actor),
       :ok <- Analytics.record(:post_published, %{post_id: published.id}) do
    {:ok, published}
  end
end

# Controller — thin coordinator.
def publish(conn, %{"id" => id}) do
  case Content.publish_post(id, actor: conn.assigns.current_user) do
    {:ok, post} ->
      conn |> put_flash(:info, "Post published") |> redirect(to: ~p"/posts/#{post.id}")

    {:error, :not_draft} ->
      conn |> put_flash(:error, "Only drafts can be published.") |> redirect(to: ~p"/posts/#{id}/edit")

    {:error, {:below_min_word_count, n}} ->
      conn |> put_flash(:error, "Posts must be at least #{n} words.") |> redirect(to: ~p"/posts/#{id}/edit")
  end
end
```

The controller does three things. The same `Content.publish_post/2` works from a LiveView event handler, a CLI command, or a GraphQL mutation with a different response shape — no duplication.

## When This Applies

- **HTTP controllers and GraphQL resolvers.** The transport is the request/response cycle; the domain is the operation.
- **LiveView `handle_event/3`, `handle_info/2`, `handle_params/2`.** The transport is the WebSocket; the domain is the state change.
- **CLI dispatchers and Mix tasks.** The transport is argv; the domain is the operation.
- **Job runners and queue workers.** The transport is the queue; the domain is the processing.
- **GenServer and actor callbacks in long-lived processes.** The transport is the message protocol; the domain is the state machine.

Any layer that sits between "a thing happened in the outside world" and "the domain mutates in response" is a coordinator.

## When This Does Not Apply

- **Pure transport plumbing.** A module whose only job is to translate between two wire formats — JSON to Protobuf, stdin to stdout — is legitimately doing translation work inline. There is no domain behind it to extract.
- **Prototype sketches.** In the first day of a feature, writing the logic inline in a controller to see if the shape is right is legitimate. The rule applies once the feature has consumers, a second call site, or a production deploy.
- **Framework extension points.** If a library explicitly wants the caller to supply behaviour as a callback (Phoenix plugs, Rust middleware, Swift property wrappers), the callback body is not a coordinator — it is a deliberately configured extension point.

Heuristic: if removing the transport layer and calling the function from a test reveals that there is no function to call — only a `conn` to set up — the coordinator is too fat.

## Further Reading

- Martin Fowler, "Presentation Domain Data Layering" — the classical layered architecture that this rule concretises.
- Chris Keathley, "Idiomatic Phoenix" (talk, 2023) — argues for controllers as "pass-through" to contexts as a hard rule.
- Intent `IN-AG-PFIC-001` — Thin Coordinator follows from PFIC: if the domain is pure, coordination is what's left.
- Intent `IN-AG-HIGHLANDER-001` — duplicated business logic across three coordinators is the symptom; Thin Coordinator is the cure.
- Concretising rules: `IN-EX-PHX-001` (Phoenix thin controllers), `IN-EX-LV-003` (LiveView thin handlers).
