---
id: IN-EX-CODE-002
language: elixir
category: code
severity: warning
title: Tagged tuples for fallible functions
summary: >
  Functions that can fail return `{:ok, value}` or `{:error, reason}`. Never
  bare values that might be `nil`. Tagged tuples make success and failure
  first-class shapes that compose with `with`, `case`, and multi-clause heads.
principles:
  - pattern-matching
  - no-silent-errors
applies_when:
  - "Any function whose success path has a value and whose failure path has a reason"
  - "Repo/API/parsing wrappers that could previously return `nil` or raise"
  - "Boundary between a coordinator and a service that may fail"
applies_to:
  - "lib/**/*.ex"
does_not_apply_when:
  - "Functions that genuinely cannot fail (pure transformations on their own output)"
  - "Bang variants (`fetch!/1`) whose contract is to raise on failure"
  - "Boolean predicates (`valid?/1`, `active?/1`) — tag is already in the name"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-EX-CODE-004
  - IN-EX-CODE-005
aliases: []
tags:
  - elixir
  - error-handling
  - tagged-tuples
status: active
version: 1
---

# Tagged tuples for fallible functions

Bare return values hide failure. A function that returns `nil` on "not found" forces every caller to distinguish "the value is nil because it's genuinely nil" from "the value is nil because something went wrong". Tagged tuples — `{:ok, value}` and `{:error, reason}` — make the two paths different shapes, so the compiler, `with`, and pattern matching can tell them apart.

## Problem

Three failure modes when fallible functions return bare values:

1. **`nil` means two things.** `Repo.get(User, id)` returns `nil` for "no user with that id" and also for "the database was unreachable, the adapter swallowed the error, and we got nothing back". Callers cannot tell the difference. Bugs are silent.
2. **Callers forget to check.** A function that returns `nil | value` invites callers to use the value directly — `user = find_user(id); user.email` — and crash three lines later with `** (KeyError) key :email not found in: nil`. The crash is far from the cause.
3. **Errors lose context.** Returning `:error` as an atom, or `false`, strips every useful detail about _why_ it failed. Tagged tuples carry the reason with them: `{:error, :not_found}`, `{:error, {:validation, %{email: ["is required"]}}}`, `{:error, :timeout}`.

Tagged tuples force the caller to handle both shapes. `with` chains short-circuit on the first `{:error, _}` and bind success values cleanly. Multi-clause function heads destructure the tag without boilerplate.

## Detection

Signals:

- A function name like `find`, `get`, `fetch`, `parse`, `load` whose body ends in `Repo.get/2`, `Map.get/2`, or an expression that can yield `nil`.
- Callers pattern-matching on the return with `if result do`, `unless is_nil(result) do`, or `result && result.field`.
- `:error` (bare atom) or `false` returned as a failure signal.
- A mix of `raise` and `nil` in the same function's failure modes.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE 'def [a-z_]+\([^)]*\) do$' lib/ | xargs grep -l 'nil$\|false$' 2>/dev/null
```

The reliable structural signal is "does this function have a failure mode, and is that mode distinguishable from its success mode in the return shape?"

## Bad

See `bad.exs` for the runnable form. Inline:

```elixir
def find_user(id) do
  Repo.get(User, id)
end

def create_order(params) do
  case validate(params) do
    :ok -> Repo.insert(%Order{...})
    :error -> nil
  end
end
```

`find_user/1` returns `nil` for not-found. `create_order/1` returns `nil` for both "validation failed" and "insert failed", and the caller cannot tell which.

## Good

See `good.exs` for the runnable form. Inline:

```elixir
def find_user(id) do
  case Repo.get(User, id) do
    nil -> {:error, :not_found}
    user -> {:ok, user}
  end
end

def create_order(params) do
  with {:ok, validated} <- validate(params),
       {:ok, order} <- Repo.insert(Order.changeset(%Order{}, validated)) do
    {:ok, order}
  end
end
```

Both success and failure are tagged. Callers compose with `with` or pattern-match with `{:ok, _}` / `{:error, _}` clauses.

## When This Applies

- Repo wrappers, API clients, parsers, file loaders — any function where the caller needs to know "did this work?"
- Service-layer functions that a coordinator (LiveView, controller, Oban worker) will `case` or `with` on.
- Functions returning "optional" values where `nil` is being used as a sentinel for failure.

## When This Does Not Apply

- **Pure transformations.** `String.upcase/1` cannot fail; tagging its return with `{:ok, "HELLO"}` is noise.
- **Bang variants.** `fetch!/1` by convention raises on failure and returns the bare value on success. The `!` is the tag.
- **Predicates.** `valid?/1` returning `true`/`false` is idiomatic; the name carries the shape.
- **Genuinely-optional values.** `Map.get(map, :nickname)` returns `nil` for "no nickname", which is a domain value, not a failure. Wrapping in `{:ok, nil}` / `{:error, :not_set}` obscures the fact that "no nickname" is fine.

A good test: "if this returns `nil`, is something wrong, or is `nil` a valid domain answer?" If something is wrong, tag the tuple. If `nil` is the answer, leave it alone.

## Further Reading

- [Intent `IN-AG-NO-SILENT-001`](../../../agnostic/no-silent-errors/RULE.md) — tagged tuples are the Elixir-specific concretisation of this agnostic rule.
- [Intent `IN-EX-CODE-004` with-for-railway](../with-for-railway/RULE.md) — `with` is what makes tagged tuples compose without nested `case`.
- [Elixir docs — `with` / Kernel.SpecialForms](https://hexdocs.pm/elixir/Kernel.SpecialForms.html#with/1) — the language reference for `with`.
