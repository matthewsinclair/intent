# Design - ST0076: a typed symbol index for Rust and Elixir

## What hv asked for

On 2026-09-16, after driving the index by hand, hv said the index has to give typed, semantic-aware search over source, not something little better than grep. hv accepted vc's recommendation: keep what tree-sitter already gives, add Intent-written reference queries, and then add resolved references. Rust and Elixir are essential; Swift can wait.

## What the index is today, measured 2026-09-16

ST0069's T2 tier stores one row per tags-query capture: `path, lang, name, kind (def | ref), start_line, end_line`. Measured on the store at 21:20Z and read from the grammars by cc and vc:

- **The capture's category is discarded.** Every grammar's tags query already says function, method, class (struct, enum, union, type alias), interface (trait), module or macro. The store keeps only `def` or `ref`, so nobody can ask for structs named X or the methods of a type.
- **No container.** A method does not record the impl, trait or module it belongs to; an Elixir function does not record its `defmodule`.
- **Duplicate definitions.** Rust's tags query matches a method twice, once under `declaration_list` as a method and once as a bare `function_item`, and a raw query cursor emits both. 1857 (path, name, line) groups hold two def rows. Swift's query has the same shape.
- **Rust references are thin.** `@reference.call` matches `function: (identifier)` and method calls only. Scoped calls (`Type::f(..)`, `AddressError::X(..)`), type uses in signatures and bodies, and anything inside a macro invocation (`assert!`, `format!`, `vec!`) are not references. `is_local` reads as never referenced; all six uses are inside `assert!`.
- **Elixir references drop the module.** A remote call `Repo.get(..)` is stored as a reference to `get`, the same as `Map.get(..)`. Arity is not stored, `def`, `defp` and `defmacro` are one kind, and aliases are not expanded.
- **Swift has no references at all.** tree-sitter-swift 0.7.3 ships definition patterns only.
- References are name matches, and the design said so; there is no resolution tier. T4, type-aware, was parked in ST0069.

ST0069 ruled on 2026-09-12 (AC-20.2) that no language gets an Intent-written tags query. hv's ruling of 2026-09-16 reverses that for Rust and Elixir.

## What typed search means here

Three levels, each a strict improvement, each usable alone:

1. **Typed definitions.** Every definition row carries its kind (function, method, struct, enum, trait, type, module, macro, and for Elixir public function, private function, macro, guard, delegate), its container (the impl, trait or module; for Elixir the module), its arity where the language has one, and its full span. One row per syntax node.
2. **Qualified references, by syntax.** Intent-written queries for Rust and Elixir record what a reference names as written, with its qualifier: `AddressError::new` as name `new` qualified by `AddressError`, a type use as a type reference, calls inside macro token trees, Elixir `Repo.get/2` as `get` qualified by `Repo`, with `alias` expanded within the file. Still not resolved: a reference says what the source wrote, and every surface says so.
3. **Resolved references.** A reference joined to the definition it actually names, from the language's own toolchain rather than from Intent's parse:
   - **Rust:** rust-analyzer's SCIP export (`rust-analyzer scip <path>`), read into the store. Not installed on this host today, so availability is a named absence and never a silent empty tier.
   - **Elixir:** the compiler's own tracer events (`remote_function`, `local_function`, `imported_function`, `alias_reference` and the macro variants, each with module, name, arity and line), collected by compiling the project with a tracer. Compiling runs the project's macros and its dependencies' code, so it runs only on an explicit verb in the project's own environment, never from intentd unasked.

Each level answers questions the one before cannot: level 1 answers the methods of `AddressError` and every struct named `Config`; level 2 answers the places that write `AddressError::new`; level 3 answers the callers of this `new`, and nothing else.

## Decisions ruled

- **hv, 2026-09-16:** levels 1 and 2 now, then level 3; Rust and Elixir required; Swift not required. AC-20.2 is reversed for Rust and Elixir: Intent keeps its own reference queries for those two grammars beside the grammars' tags queries.
- **vc, under the pen:** one row per syntax node, fixed at extraction and never by a `DISTINCT` at read. The extractor carries a version, and a store indexed by an older extractor re-extracts rather than mixing rows. A resolved tier that did not run (tool absent, compile failed) is named in the envelope with its reason, beside the syntactic answer, never an empty result.

## Work packages

- **WP-01 Typed definitions (all grammars, M).** Kind, container, arity, full span; one row per node; extractor version and re-extraction. Swift and Lua get kind and dedupe for free and nothing more.
- **WP-02 Rust reference queries (M).** Scoped calls, type uses, macro token trees, with the qualifier stored.
- **WP-03 Elixir reference queries (M).** Remote calls with their module, captures (`&Mod.fun/2`), pipes, `alias` expansion in the file, `use`, `import` and `require` as references, arity where written.
- **WP-04 Surfaces (M).** `intent search` filters by kind and container (`--kind struct`, `--in AddressError`), `--context` prints qualified references and says which level answered, the SQL door's `symbols` columns, the MCP tool's parameters and the `mcp_instructions` line, the explorer's hits.
- **WP-05 Rust resolved references (L).** Measure first: rust-analyzer's install path, SCIP export time and size on this repository, what a missing tool reports. Then the ingest and the `resolved` level in the envelope.
- **WP-06 Elixir resolved references (L).** Measure first on a real Elixir estate (Laksa or Lamplight): tracer output, compile time, what a failed compile reports. Then the ingest and the `resolved` level.

Order: WP-01, then WP-02, WP-03 and WP-04 in parallel, then WP-05 and WP-06. Issue 0429 is descoped into this thread.

## Proof

After hv's rebuild, the defined-but-never-referenced query that failed on 2026-09-16 no longer lists `is_local`, `AddressError` or `from_project`; `intent search --kind method --in AddressError` lists its methods once each; an Elixir estate answers the callers of one `Repo` function without the same-named `Map` function.
