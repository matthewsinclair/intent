---
id: IN-EX-ASH-001
language: elixir
category: ash
severity: critical
title: All database access through Ash domain code interfaces
summary: >
  Never call `Ash.get!/2`, `Ash.read!/2`, `Ash.create!/2`, or `Ash.load!/2`
  directly from LiveViews, controllers, or other web modules. Every database
  access goes through a domain-level code interface. Web modules are
  coordinators; the domain is where Ash lives.
principles:
  - thin-coordinator
  - ash-usage-rules
applies_when:
  - "Any LiveView, controller, channel, or channel join that needs to read or mutate an Ash resource"
  - "Any Oban worker or mix task that touches the database"
  - "Any module in the web/ directory or equivalent outside the domain"
applies_to:
  - "lib/**/*.ex"
  - "lib/**/live/**/*.ex"
  - "lib/**/controllers/**/*.ex"
does_not_apply_when:
  - "Domain modules themselves (the code interface is *defined* in the domain, not called through it)"
  - "Ash-internal migrations or seeds where there is no domain yet"
  - "Test fixture helpers that need to bypass policy for setup (document the bypass explicitly)"
references:
  - IN-AG-THIN-COORD-001
related_rules:
  - IN-EX-ASH-002
aliases: []
tags:
  - elixir
  - ash
  - domain-boundary
status: active
version: 1
---

# All database access through Ash domain code interfaces

Ash gives you a clean, generated API per domain action. Calling `Ash.get!/2` or `Ash.read!/2` directly from a LiveView is ignoring the gift. The code interface is where actor-passing, authorisation, and query composition all happen correctly by default. Bypassing it in web modules moves those concerns into the web layer, and they rot there.

## Problem

Three failure modes when web modules call `Ash.*` directly:

1. **Policy bypasses creep in.** A LiveView written in a hurry does `Ash.get!(Post, id)` without an actor. Policy never runs. The feature ships; six months later someone finds that unauthenticated users can view draft posts.
2. **Query composition duplicates.** Web modules build their own `Ash.Query.filter(...)` pipelines. The same "active, published, not-deleted" filter appears in the LiveView, the controller, and the admin dashboard — each slightly different. The domain had a code interface that did exactly this; nobody found it.
3. **Refactor scope explodes.** Renaming an action or adding a required argument now requires editing every `Ash.read!` call site. A code interface makes this one change in the domain.

The code interface is the Ash boundary. Cross it from the outside; never reach past it.

## Detection

Signals:

- `Ash.get!/2`, `Ash.read!/2`, `Ash.create!/2`, `Ash.update!/2`, `Ash.destroy!/2`, `Ash.load!/2` in a module outside the domain.
- `Ash.Query.filter(...)` or `Ash.Query.for_read(...)` in a LiveView, controller, or channel.
- A `require Ash.Query` at the top of a web module (a direct signal the module is building queries itself).
- `MyApp.Resource |> Ash.Query.for_read(...)` pipelines inside `mount/3` or `handle_event/3`.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE 'Ash\.(get|read|create|update|destroy|load)!?' lib/*_web/ lib/*/live/ lib/*/controllers/
```

The reliable structural signal is "is this module outside the domain, and is it calling Ash.\*?"

## Bad

Inline:

```elixir
def mount(%{"id" => id}, _session, socket) do
  post = MyApp.Content.Post |> Ash.get!(id) |> Ash.load!([:author])
  {:ok, assign(socket, :post, post)}
end

def handle_event("publish", %{"id" => id}, socket) do
  MyApp.Content.Post
  |> Ash.Query.filter(id == ^id)
  |> Ash.read_one!()
  |> Ash.Changeset.for_update(:publish)
  |> Ash.update!()

  {:noreply, socket}
end
```

Web module is doing Ash plumbing. No actor is being passed. Policy is invisible. Query composition is ad hoc.

## Good

Inline:

```elixir
def mount(%{"id" => id}, _session, socket) do
  post =
    MyApp.Content.get_post!(id,
      load: [:author],
      actor: socket.assigns.current_user
    )

  {:ok, assign(socket, :post, post)}
end

def handle_event("publish", %{"id" => id}, socket) do
  case MyApp.Content.publish_post(id, actor: socket.assigns.current_user) do
    {:ok, _post} -> {:noreply, put_flash(socket, :info, "Published")}
    {:error, reason} -> {:noreply, put_flash(socket, :error, inspect(reason))}
  end
end
```

Domain code interfaces (`get_post!/2`, `publish_post/2`) carry the contract. Actor is passed. Load is a declarative option. The LiveView is a coordinator.

## When This Applies

- All web modules (LiveViews, controllers, channels, router plugs).
- Oban workers, GenServers, and mix tasks that need Ash data.
- Any non-domain module reaching for an Ash resource.

## When This Does Not Apply

- **Domain modules.** The domain is where `code_interface do ... end` is declared and where `Ash.*` calls the underlying machinery.
- **Intent-level seed scripts.** When seeding a database before any domain exists (a rare edge case), direct `Ash.create!/2` is fine — but only during setup.
- **Test fixtures with documented bypass.** `authorize?: false` in a fixture helper is acceptable if the fixture's only job is to put rows in the DB. Document the bypass.

A good test: "if policy changed tomorrow, does this call pick up the new policy automatically?" If no, it is bypassing the domain.

## Further Reading

- [Intent `IN-AG-THIN-COORD-001`](../../../agnostic/thin-coordinator/RULE.md) — the agnostic principle this rule concretises.
- [Ash Framework — code interfaces](https://hexdocs.pm/ash/code-interfaces.html) — the authoritative Ash reference for the mechanism this rule depends on.
- [Ash usage rules (`deps/ash/usage-rules.md`)](https://github.com/ash-project/ash/blob/main/usage-rules.md) — the upstream Usage Rules that include this discipline.
