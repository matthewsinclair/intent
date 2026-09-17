---
st_id: ST0076
title: A typed symbol index for Rust and Elixir: kinds, containers, qualified references and resolved references
---

# ST0076: A typed symbol index for Rust and Elixir: kinds, containers, qualified references and resolved references -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) After hv's rebuild, on the Intent estate: the defined-but-never-referenced query over `symbols` no longer lists `is_local`, `AddressError` or `from_project`; `intent search --kind method --in AddressError` lists each of its methods once; and a fresh Claude session asked where a Rust symbol is defined reaches the index's MCP search tool before grep. -- satisfied: no

### WP-01 -- Typed definitions: kind, container, arity and span, one row per syntax node, with an extractor version (status: WIP)

- AC-01.1 For Rust and Elixir, every definition row carries its `subkind`, `container`, `container_kind`, `trait_name`, `arity` and `arity_min` as ST0076's design vocabulary table states, read from the file's own syntax only. -- satisfied: no (computed)
- AC-01.2 One symbol row per (name node, kind) in every language, fixed at extraction and never by a `DISTINCT` at read; a definition's own name is never also a reference to it. -- satisfied: no (computed)
- AC-01.3 A code file whose symbols an older extractor wrote is re-extracted by the next reconcile, and a store migrated to the typed schema keeps its rows until then; two row shapes never answer one query. -- satisfied: no (computed)

### WP-02 -- Rust qualified references: scoped calls, type uses and macro token trees, with the qualifier (status: Not Started)

- AC-02.1 Rust references by syntax include scoped calls (`Type::f(..)`), type uses in signatures and bodies, and names inside macro invocations, each with its qualifier as written and `level` 2; `intent search --context nearest_project` on this repository lists `views.rs:437` and `views.rs:439` (issue 0429). -- satisfied: no (computed)
- AC-02.2 Every reference row spans the name as written, not the enclosing call or item, and the change raises the extractor version so stored rows re-extract. -- satisfied: no (computed)

### WP-03 -- Elixir qualified references: remote calls with their module, captures, pipes, alias expansion, use, import and require (status: Not Started)

- AC-03.1 Elixir references by syntax include remote calls and captures (`&Mod.fun/2`) with their module, pipe targets, and `alias`, `import`, `require` and `use` as references, with arity where written and `level` 2; the qualifier is the module with the file's own `alias` forms and `__MODULE__` expanded within the enclosing do-block, and nothing from `import` or `use` applied. -- satisfied: no (computed)

### WP-04 -- Surfaces: search by kind and container, qualified context naming its level, SQL columns, MCP parameters and instructions (status: Not Started)

- AC-04.1 `intent search` filters definitions by subkind and container (`--kind struct`, `--in <container>`), `--context` prints each reference with its qualifier and names the level that answered, the SQL door and `intent schema` expose the new columns, and the MCP search tool takes the same filters. -- satisfied: no (computed)
- AC-04.2 The search row's `when_to_use`, the MCP `instructions` line and the explorer's hits say what the index now answers and what it still does not, per level and per language, with no claim wider than the build. -- satisfied: no (computed)

### WP-05 -- Rust resolved references through rust-analyzer's SCIP export (status: Not Started)

- AC-05.1 Rust level 3: rust-analyzer's SCIP export is read into the store, and a resolved row always joins a written level-2 reference (same file, line and name); `intent index status` counts matched, unmatched and dropped; a missing rust-analyzer is named in the envelope with its reason and never answers as an empty tier. -- satisfied: no (computed)

### WP-06 -- Elixir resolved references through the compiler's tracer, on an explicit verb (status: Not Started)

- AC-06.1 Elixir level 3, on an explicit verb only and never from intentd unasked: the compiler tracer's events join written level-2 references (same file, line and name) and expansion events are not stored; the tracer never raises; a failed compile stores nothing new, names the failure, and marks earlier rows stale by content hash; rows are replaced per file, `mix compile` runs without force and `--full` forces. -- satisfied: no (computed)
- AC-06.2 (non-test) On a scratch copy of Laksa, the level-3 callers of `Map.get/2` are listed without any call to `Access.get`, `Process.get` or `Keyword.get`. -- satisfied: no

## Acceptance Tests

### ST-level

_(no tests in this group)_

### WP-01 -- Typed definitions: kind, container, arity and span, one row per syntax node, with an extractor version (status: WIP)

_(no tests in this group)_

### WP-02 -- Rust qualified references: scoped calls, type uses and macro token trees, with the qualifier (status: Not Started)

_(no tests in this group)_

### WP-03 -- Elixir qualified references: remote calls with their module, captures, pipes, alias expansion, use, import and require (status: Not Started)

_(no tests in this group)_

### WP-04 -- Surfaces: search by kind and container, qualified context naming its level, SQL columns, MCP parameters and instructions (status: Not Started)

_(no tests in this group)_

### WP-05 -- Rust resolved references through rust-analyzer's SCIP export (status: Not Started)

_(no tests in this group)_

### WP-06 -- Elixir resolved references through the compiler's tracer, on an explicit verb (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
