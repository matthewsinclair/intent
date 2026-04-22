---
id: IN-EX-CODE-004
language: elixir
category: code
severity: warning
title: "`with` for railway-oriented composition"
summary: >
  Chain two or more fallible operations with `with`. Normalise errors in
  private wrappers rather than untangling them in `else` blocks. The
  happy path reads top-to-bottom; failures short-circuit to the first
  `{:error, _}`.
principles:
  - pattern-matching
  - no-silent-errors
applies_when:
  - "Composing two or more `{:ok, _}` / `{:error, _}` functions in sequence"
  - "Replacing nested `case` blocks where each inner branch handles `{:error, _}`"
  - "Coordinator functions (controllers, LiveView handlers, Oban workers) that dispatch to multiple services"
applies_to:
  - "lib/**/*.ex"
does_not_apply_when:
  - "Single-step operations — a single `case` is clearer"
  - "All steps are infallible — a pipe chain is the right tool"
  - "The `else` needs more than two clauses for error normalisation — extract each step into a private helper with tagged-tuple wrapping instead"
references:
  - IN-AG-NO-SILENT-001
  - IN-AG-PFIC-001
related_rules:
  - IN-EX-CODE-002
  - IN-EX-CODE-005
aliases: []
tags:
  - elixir
  - railway
  - control-flow
  - with
status: active
version: 1
---

# `with` for railway-oriented composition

Nested `case` blocks over `{:ok, _}` / `{:error, _}` are the Elixir version of arrow-code. Each new fallible step adds another indent. `with` flattens the chain into a sequence of `<-` bindings, short-circuits on the first failure, and puts the happy path where it belongs — at the top, at the margin.

## Problem

Three failure modes without `with`:

1. **Pyramid of doom.** Three nested `case` blocks means three levels of indentation, three `error -> error` forwarders, and a happy path buried at the bottom. A reader has to traverse the whole pyramid to see what the function actually does.
2. **`else` clauses become an error-handling spec.** `with ... else` is tempting for aggregating every possible error, but the result is a long `else` block where each step's errors are re-mapped. That `else` becomes a second decision tree the reader must hold alongside the main one.
3. **Errors lose provenance.** A nested `case` forwards `error` as-is, but four steps later a reader cannot tell which step produced the error. `with` keeps the error shape intact; the tag tells the caller which step failed.

`with` replaces the pyramid with a railway: each `<-` is a track, the happy path stays on the rail, `{:error, _}` derails and the expression returns that error directly. Normalise errors in each step's return shape, not in an `else`.

## Detection

Signals:

- Three or more nested `case` blocks in a function body, each branching on `{:ok, _}` / `{:error, _}`.
- An `else -> error` forwarder at the inner end of a `case`.
- A function body with "step 1 / step 2 / step 3" comments indicating sequential operations.
- `with ... else` blocks with more than two clauses, often remapping the same `{:error, _}` into a normalised form.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE 'case.*do$' lib/ | wc -l      # count case blocks per file
grep -rnE '^[[:space:]]+error -> error$' lib/  # the forwarder antipattern
```

The reliable structural signal is "does this function compose two or more fallible operations whose success feeds the next?"

## Bad

See `bad.exs` for the runnable form. Inline:

```elixir
def create_order(params) do
  case validate(params) do
    {:ok, validated} ->
      case charge_payment(validated) do
        {:ok, payment} ->
          case save_order(validated, payment) do
            {:ok, order} -> {:ok, order}
            error -> error
          end
        error -> error
      end
    error -> error
  end
end
```

Three levels of nesting; the happy path (`{:ok, order}`) is buried 4 levels deep; two `error -> error` forwarders add noise.

## Good

See `good.exs` for the runnable form. Inline:

```elixir
def create_order(params) do
  with {:ok, validated} <- validate(params),
       {:ok, payment} <- charge_payment(validated),
       {:ok, order} <- save_order(validated, payment) do
    {:ok, order}
  end
end
```

Three bindings, one happy-path result. Any `{:error, _}` short-circuits. No `else` needed — the error shape flows through unchanged.

When error normalisation is needed, push it into each step rather than into `else`:

```elixir
def create_order(params) do
  with {:ok, validated} <- normalise_validate(params),
       {:ok, payment} <- normalise_charge(validated),
       {:ok, order} <- normalise_save(validated, payment) do
    {:ok, order}
  end
end

defp normalise_validate(params) do
  case validate(params) do
    {:ok, _} = ok -> ok
    {:error, reason} -> {:error, {:validation, reason}}
  end
end
```

The `with` stays clean; the step wrappers encode the error context.

## When This Applies

- Coordinator functions that dispatch to two or more services.
- Phoenix controller actions that `with`-chain `fetch resource → authorise → mutate → render`.
- Oban workers whose `perform/1` pipelines call several services.
- Any function whose body was previously a nested `case` over tagged tuples.

## When This Does Not Apply

- **One step.** `case Repo.get(...) do` is fine as a single `case`. `with` on one binding adds syntax, not clarity.
- **All-infallible pipe.** If every step returns a bare value (no `{:ok, _}`), use `|>` instead.
- **Early returns with divergent shapes.** `with` expects each `<-` to produce the same tag family. If one step returns `nil`, another returns `{:error, _}`, and a third raises, normalise them first — either by wrapping each in its own helper (see above) or by using `case` explicitly.

A good test: "does this function compose two or more `{:ok, _}` / `{:error, _}` calls?" If yes, `with`. If no, `case` or `|>`.

## Further Reading

- [Intent `IN-AG-NO-SILENT-001`](../../../agnostic/no-silent-errors/RULE.md) — `with` is the Elixir mechanism that makes no-silent-errors ergonomic.
- [Intent `IN-EX-CODE-002` tagged-tuple-returns](../tagged-tuple-returns/RULE.md) — `with` only works if the steps return tagged tuples.
- [Elixir docs — `with`](https://hexdocs.pm/elixir/Kernel.SpecialForms.html#with/1) — language reference.
- [José Valim — "Complex `else` clauses in `with`"](https://hexdocs.pm/elixir/anti-patterns.html#complex-else-clauses-in-with) — the canonical anti-pattern this rule points away from.
