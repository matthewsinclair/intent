---
id: IN-EX-CODE-001
language: elixir
category: code
severity: warning
title: Pattern matching over conditionals
summary: >
  Prefer multi-clause function heads with destructuring over nested
  `if`/`case`/`cond` that branch on struct or map fields. Each clause
  is a single expression keyed on shape; guards handle type or range
  decisions.
principles:
  - pattern-matching
  - public-interface
applies_when:
  - "Dispatching on the shape of a struct, map, or tuple"
  - "Switching behaviour based on a field value or a type class"
  - "Any chain of nested `if ... else if ... else` on the same data"
applies_to:
  - "lib/**/*.ex"
does_not_apply_when:
  - "Single-branch conditionals used once for clarity"
  - "Boolean gates where all three decisions (`if/else`) are a single pure expression"
  - "Branches on computed values where pattern-match would require match specs outside the core language"
references: []
related_rules:
  - IN-EX-CODE-003
  - IN-EX-CODE-004
aliases: []
tags:
  - elixir
  - pattern-matching
status: active
version: 1
---

# Pattern matching over conditionals

Nested `if`/`case`/`cond` on struct or map fields reads like an unfolded pattern match with the match disabled. Multi-clause function heads with destructuring express the same decision tree in half the lines and crash loudly on unhandled shapes.

## Problem

Conditionals hide the decision space. A function that dispatches on `user.status` with an `if/else if/else` chain reads top-to-bottom; you have to read every branch to learn what shapes the function handles. The unhandled case is the `else` — implicit, and likely wrong.

Three failure modes:

1. **Silent fall-through.** `if/else` collapses every shape that does not match the first predicate into the `else` branch. A new `status: :suspended` slips through to the `:inactive` path because nobody remembered to add the new predicate.
2. **Compound predicates drift.** `if user.status == :active and user.role == :admin` is one branch today; three branches tomorrow as `:role` grows `:editor` and `:viewer`. The nested conditionals become unreadable, and the tests lag.
3. **Unmatched shapes are mysteries.** A `cond` whose last clause is `true -> _` returns a default for any unexpected input. The function appears to work on every input, including ones the author never considered. Bugs are silent downstream.

Multi-clause functions turn the decision space into a structural contract: every handled shape is a function head; an unhandled shape raises `FunctionClauseError` at the binding site, pointing exactly at the missing case.

## Detection

Signals:

- Three or more arms of `if`/`else if`/`else` branching on fields of a single argument (`user.status`, `order.state`).
- A `case` block whose arms pattern-match only to extract a field, then `if` on the field.
- `cond` blocks where every arm is a predicate on one input struct.
- A function body whose first line is `user = ...; if user.role == :admin do ...`.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE '^[[:space:]]+if[[:space:]]+[a-z_]+\.(status|role|state|kind)' lib/
```

The reliable structural signal is "could this be expressed as two or more function heads of the same arity, each destructuring?"

## Bad

See `bad.exs` for the runnable form. Inline:

```elixir
def process(user) do
  if user.status == :active do
    if user.role == :admin do
      :allowed
    else
      :denied
    end
  else
    :inactive
  end
end
```

Two nested `if`s obscure a three-way decision. Add `:editor` as a fourth role and the branching gets worse. Any new `status` value silently falls through to `:inactive`.

## Good

See `good.exs` for the runnable form. Inline:

```elixir
def process(%{status: :active, role: :admin}), do: :allowed
def process(%{status: :active}), do: :denied
def process(_), do: :inactive
```

Three clauses, each one line. The shape is the contract. Adding `:editor` is one new clause, not a nested refactor.

For type- or range-based decisions, use guards on a single head:

```elixir
def format(value) when is_binary(value), do: String.trim(value)
def format(value) when is_integer(value), do: Integer.to_string(value)
def format(value), do: inspect(value)
```

## When This Applies

- Dispatching on the discrete shape of a struct, map, or tuple (the common case).
- Switching behaviour based on a field value drawn from a small enumerated set (`:pending`, `:approved`, `:rejected`).
- Type-class dispatch (`is_binary`, `is_list`, `is_integer`) where guards are the right tool.
- `{:ok, value}` / `{:error, reason}` destructuring in coordinators.

## When This Does Not Apply

- **Single-branch conditionals used once for clarity.** A one-off `if condition?, do: expensive_computation()` is fine; turning it into a single-clause function just adds noise.
- **Boolean gates where all three arms are pure expressions.** `if approved?, do: :accept, else: :reject` is already at the minimal form; a pattern match would not help.
- **Computed predicates that the language cannot match directly.** `case user_age > 65 do ... end` compares a dynamic value; fall back to `cond` or guards. Pattern matching is for _shapes_, not for arbitrary Boolean expressions.
- **Top-level CLI arg parsing.** `System.argv` is an unstructured list; pattern-match it once at the entry point, not throughout the program.

A good test: "if I add a new struct to handle, is it one new clause or a refactor?" Pattern matching keeps it to one new clause.

## Further Reading

- [Intent `IN-AG-PFIC-001`](../../../agnostic/pfic/RULE.md) — pattern matching is what turns a coordinator-style conditional chain into a pure-core dispatch.
- [Elixir docs — Pattern matching](https://hexdocs.pm/elixir/pattern-matching.html) — the language reference for what is matchable.
- [José Valim — "Elixir anti-patterns"](https://hexdocs.pm/elixir/anti-patterns.html) — "complex else clauses in with" touches the same nerve as this rule from the `with` side.
