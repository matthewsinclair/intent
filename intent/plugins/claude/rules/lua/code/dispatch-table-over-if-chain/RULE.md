---
id: IN-LU-CODE-006
language: lua
category: code
severity: warning
title: Dispatch table over if-chain for value dispatch
summary: >
  When a Lua function dispatches on a discriminating value (eg
  `perturbation.tag`, a token `kind`, a `verb`) to call different
  downstream functions, use a `HANDLERS` table-of-functions keyed
  by the value and a single lookup + invoke. Lua has no pattern
  matching and no multi-head function definitions; the dispatch-
  table idiom is the substitute. Inside-handler guard clauses on
  derived booleans (alive checks, nil checks, invariant violations)
  stay as `if`.
principles:
  - pfic
applies_when:
  - "A function dispatches on a single discriminating value via if/elseif"
  - "Each branch returns or invokes a different downstream function"
  - "There are two or more branches at the same nesting level"
applies_to:
  - "**/*.lua"
does_not_apply_when:
  - "Guard clauses (early returns) on derived booleans or invariant checks"
  - "Branches that compute different values inline rather than dispatching to functions"
  - "Single-branch conditionals (`if`/`then`/no `else`)"
  - "Branches with substantial per-branch preamble that differs across cases"
references:
  - IN-AG-PFIC-001
  - IN-EX-CODE-001
related_rules:
  - IN-EX-CODE-001
aliases: []
tags:
  - lua
  - dispatch
  - control-flow
status: active
version: 1
---

# Dispatch table over if-chain for value dispatch

Lua has no pattern matching and no multi-head function definitions. The idiomatic substitute is a table-of-functions keyed by the discriminating value, with one lookup at the call site.

## Problem

Lua's only branching primitive for value-dispatch is `if/elseif/else`. Unlike Erlang/Elixir's multi-head functions, OCaml/Rust/Haskell's pattern matching, or Python's `match`, Lua offers no language-level dispatch on tag/value. The naive habit transfers a chain of equality checks into the hot path:

```lua
if x.tag == "a" then return on_a(x) end
if x.tag == "b" then return on_b(x) end
if x.tag == "c" then return on_c(x) end
```

Three problems:

1. The dispatch _is_ a sequence — readers walk the chain top to bottom to find the relevant branch.
2. Adding a new value means adding a new clause, often duplicating the return shape.
3. The branches are visually heavy compared to their content (a single function call).

Lua's idiomatic answer is the dispatch table: a `local` table of functions keyed by the discriminating value, plus a one-line lookup + invoke:

```lua
local HANDLERS = {
  a = on_a,
  b = on_b,
  c = on_c,
}

return function(x)
  local h = HANDLERS[x.tag]
  if h then return h(x) end
  return default
end
```

Three benefits, mirroring the problems:

1. The dispatch is declarative — readers scan the table to see the routing.
2. Adding a new value is one new row in the table.
3. The branches collapse to data; the active code is the lookup.

## Detection

Static signals:

- A function (or returned function) with two or more sibling `if` statements at the same nesting level, each testing equality of the **same field** on the same value (`if x.tag == "..." then`).
- Each branch returns or calls a different function.
- A linear search over a closed set of string / atom values.

Negative signals (rule does not apply):

- Branches compute different values inline (not dispatching to functions).
- Conditions test different fields or derived booleans (guard clauses).
- Only one `if` branch — no dispatch.

## Bad

```lua
return function(state, perturbation)
  if perturbation.tag == "phase_entered" then
    return on_phase_entered(state, perturbation)
  end
  if perturbation.tag == "option_selected" then
    return on_option_selected(state, perturbation)
  end
  if perturbation.tag == "character_died" then
    return on_character_died(state, perturbation)
  end
  return state
end
```

A chain of equality checks dispatching on `tag`. The structure is repetitive; the dispatch information is mixed with the dispatch mechanism. Adding a fourth case is a copy-paste-edit operation.

## Good

```lua
local HANDLERS = {
  phase_entered   = on_phase_entered,
  option_selected = on_option_selected,
  character_died  = on_character_died,
}

return function(state, perturbation)
  local handler = HANDLERS[perturbation.tag]
  if handler then return handler(state, perturbation) end
  return state
end
```

The routing is a table you can read at a glance; the dispatcher is a single lookup. Adding a fourth handler is one row.

### Multiple values sharing one handler

When two keys route to identical behaviour, the table makes the sharing visible:

```lua
local function first_hand(state, p, pctx)
  return record_fact(state, p.fact, "self", pctx)
end

local function hearsay(state, p, pctx)
  return record_fact(state, p.fact, p.character_id, pctx)
end

local HANDLERS = {
  evidence_discovered  = first_hand,
  observation_recorded = first_hand,
  dialogue_revealed    = hearsay,
}
```

Two keys pointing to one function is cleaner than two near-duplicate `if` branches.

## When This Applies

- Mechanic hooks dispatching on `perturbation.tag` (the canonical case in worldwright-authored Lua).
- Parsers dispatching on a token `kind`.
- State machines with named transitions.
- Any function whose primary job is "different incoming kind → different downstream function".

## When This Does Not Apply

- **Guard clauses** — early returns on alive checks, nil checks, invariant violations. These are guards, not dispatch. They stay as `if`.
- **Two-arm conditionals** computing different values inline rather than dispatching to functions.
- **Single-branch** `if` — no dispatch to speak of.
- **Branches with non-trivial per-branch preamble** that differs across cases (then a function-call dispatch may obscure rather than clarify; refactor case by case).

## Further Reading

- Programming in Lua (Ierusalimschy), ch. 5.2 "Functions as First-Class Values" — the dispatch-table idiom (<https://www.lua.org/pil/5.2.html>)
- Lua-users wiki, "Function Tables" (<http://lua-users.org/wiki/FunctionTables>)
- IN-EX-CODE-001 — Elixir sister rule (multi-head function dispatch via pattern matching); same spirit, different language affordances.
- IN-AG-PFIC-001 — agnostic principle (Pure-Functional-Idiomatic-Coordination); declarative composition over imperative branching.
