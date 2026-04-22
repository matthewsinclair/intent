---
description: "Phoenix LiveView rules: two-phase mount, streams for lists, thin LiveViews, @impl true on callbacks"
---

# Phoenix LiveView Essentials

Load the Intent LiveView and Phoenix rule pack into context. LiveView-specific rules live in `rules/elixir/lv/`; shared controller-layer rules live in `rules/elixir/phoenix/`.

## Procedure

### 1. Load the LiveView rules

| Rule ID        | Slug                | What it enforces                                                    |
| -------------- | ------------------- | ------------------------------------------------------------------- |
| `IN-EX-LV-001` | `two-phase-mount`   | Guard `subscribe`, timers, async work with `if connected?(socket)`. |
| `IN-EX-LV-002` | `streams-for-lists` | Use `stream/3` for lists that grow; only deltas go over the wire.   |
| `IN-EX-LV-003` | `thin-liveviews`    | LiveView event handlers delegate to the domain; no business logic.  |

### 2. Load the Phoenix controller rules

| Rule ID         | Slug               | What it enforces                                              |
| --------------- | ------------------ | ------------------------------------------------------------- |
| `IN-EX-PHX-001` | `thin-controllers` | Controllers parse, call, shape. No business logic in actions. |

### 3. Load the shared Elixir rules that apply to LiveView code

| Rule ID          | Slug                              | Why it matters for LiveView                                                                  |
| ---------------- | --------------------------------- | -------------------------------------------------------------------------------------------- |
| `IN-EX-CODE-003` | `impl-true-on-callbacks`          | `@impl true` on `mount/3`, `render/1`, `handle_event/3`, `handle_info/2`, `handle_params/2`. |
| `IN-EX-CODE-004` | `with-for-railway`                | `handle_event/3` bodies often chain fallible operations — use `with`.                        |
| `IN-EX-CODE-001` | `pattern-match-over-conditionals` | Branch `handle_event/3` clauses on event name or params shape, not nested `if`.              |

### 4. Additional operational conventions

Not yet first-class rules:

- **`push_navigate` vs `push_patch`.** `push_patch` stays in the same LiveView and triggers `handle_params/2`. `push_navigate` goes to a _different_ LiveView and triggers a full `mount/3`. Do not `push_patch` to a route served by a different LiveView.
- **`assign_async/3` for slow data loads.** Never block `mount/3` with an expensive query. Wrap in `assign_async/3` and render a loading state via `<.async_result>`.
- **Extract repeated HEEX into function components.** When the same HTML block appears in two places, pull it into a component with typed `attr/3` declarations for compile-time validation.

### 5. Check Phoenix / LiveView Usage Rules

Read `deps/phoenix_live_view/usage-rules.md` for the upstream authoritative contract. Anything the rules above do not cover defaults to upstream Usage Rules.

## Red Flags

| Rationalisation                                          | Reality                                                                |
| -------------------------------------------------------- | ---------------------------------------------------------------------- |
| "Subscribing in mount is fine; it works."                | See IN-EX-LV-001. It double-subscribes and wastes the SSR render.      |
| "Assigning the full list is simpler than streams."       | See IN-EX-LV-002. Simpler until the list grows; then it is a refactor. |
| "The business logic is too small to move to the domain." | See IN-EX-LV-003. Even small logic duplicates when Oban needs it.      |
