---
id: IN-EX-ASH-002
language: elixir
category: ash
severity: warning
title: Set actor on query/changeset, not on action call
summary: >
  Pass `actor: current_user` when building the query or changeset
  (`Ash.Query.for_read(...)`, `Ash.Changeset.for_create(...)`), not on the
  terminal `Ash.read!/2` or `Ash.create!/2` call. Code interfaces handle this
  correctly; manual pipelines often do not.
principles:
  - ash-usage-rules
applies_when:
  - "Manually building an Ash.Query or Ash.Changeset before calling Ash.read/2 or Ash.create/2"
  - "Policies or calculations depend on the actor"
  - "Passing actor through a service-layer function that composes queries"
applies_to:
  - "lib/**/*.ex"
does_not_apply_when:
  - "Using a domain code interface (`MyApp.Content.list_posts!(actor: user)`) — the interface routes the actor correctly by construction"
  - "Actions that are genuinely public (no actor, no policy)"
references: []
related_rules:
  - IN-EX-ASH-001
aliases: []
tags:
  - elixir
  - ash
  - actor
  - policy
status: active
version: 1
---

# Set actor on query/changeset, not on action call

Ash routes the actor into policies and calculations at query-build time, not at execution time. Passing `actor:` to `Ash.read!/2` looks right and often works for simple cases, but it bypasses the `for_read/3` step where policies evaluate and calculations resolve. Set the actor where the query is born, not where it is run.

## Problem

Three failure modes when the actor is passed on the terminal call:

1. **Calculations see `nil` actor.** Calculations registered with `calculations do` can reference `actor/0`. If the actor is not on the query at build time, the calculation sees `nil` — even though `Ash.read!(query, actor: user)` is called seconds later.
2. **Policies silently fall through.** A policy that checks `actor_attribute_equals(:role, :admin)` evaluates at query-build time. Without the actor set at build time, it has no actor to check; the policy's default may allow the read that should have been denied.
3. **Inconsistent with code interfaces.** Domain code interfaces route the actor through `for_read/3` correctly. Manual pipelines that pass `actor:` on the terminal call drift from that convention — two patterns in the codebase, one of which is wrong for certain policy types.

The rule is simple: actor on the query, not on the action.

## Detection

Signals:

- `Ash.read!(query, actor: user)` or `Ash.create!(changeset, actor: user)` at the terminal call site, where `query` or `changeset` was built without `actor:`.
- `Ash.Query.for_read(Resource, :name, %{}) |> Ash.read!(actor: user)` — actor should be inside `for_read`.
- Manual pipelines in web modules building queries and applying actor at the end (this usually co-occurs with IN-EX-ASH-001 too).

**No greppable proxy is authoritative for this rule.** The naive proxy `Ash\.(read|...)\([^)]*actor:` fires on the terminal call site — but the violation depends on whether the **query** (built earlier) had `actor:` already. Detecting that requires reading both the query construction and the call site. The reliable structural signal is "was the actor present when the query/changeset was built?" Apply this rule via the LLM-driven `critic-elixir` subagent during `/in-review`, not in the headless pre-commit gate.

## Bad

Inline:

```elixir
# Actor arrives at the terminal call — policies and calculations may have
# already evaluated against a nil actor.
MyApp.Content.Post
|> Ash.Query.for_read(:read, %{})
|> Ash.read!(actor: current_user)
```

## Good

Inline:

```elixir
# Actor is on the query at build time.
MyApp.Content.Post
|> Ash.Query.for_read(:read, %{}, actor: current_user)
|> Ash.read!()

# Even better — code interface handles this by construction.
MyApp.Content.list_posts!(actor: current_user)
```

For changesets:

```elixir
# Actor on the changeset at build time.
MyApp.Content.Post
|> Ash.Changeset.for_create(:create, params, actor: current_user)
|> Ash.create!()

# Code interface variant.
MyApp.Content.create_post!(params, actor: current_user)
```

## When This Applies

- Any manual `Ash.Query.for_read` or `Ash.Changeset.for_create`/`for_update`/`for_destroy` pipeline.
- Any policy that uses `actor_attribute_equals`, `relates_to_actor_via`, or a custom check that reads the actor.
- Any calculation that references `actor/0`.
- Any service-layer function that composes a query and passes it on to another function.

## When This Does Not Apply

- **Domain code interfaces.** `MyApp.Content.list_posts!(actor: current_user)` wires the actor through `for_read/3` automatically. Use interfaces and this rule is auto-satisfied.
- **Genuinely public actions.** An action with no policies and no actor-dependent calculations can omit the actor. Flag it as public in the resource.
- **System-level operations.** Background jobs acting on behalf of the system (not a user) pass `authorize?: false` or a system actor — which is the actor, passed at build time.

A good test: "does the policy or calculation read the actor?" If yes, the actor must be on the query at build time.

## Further Reading

- [Intent `IN-EX-ASH-001` code-interfaces-only](../code-interfaces-only/RULE.md) — the related rule; together they keep Ash boundaries clean.
- [Ash Framework — policies and actors](https://hexdocs.pm/ash/policies.html) — how Ash uses the actor during query evaluation.
- [Ash Framework — calculations](https://hexdocs.pm/ash/calculations.html) — where `actor/0` can appear and why it needs the actor at build time.
