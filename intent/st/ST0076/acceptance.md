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

- AC-00.1 (non-test) After hv's rebuild, on the Intent estate: the defined-but-never-referenced query over `symbols` no longer lists `is_local`, `AddressError` or `from_project`; `intent search --subkind method --in AddressError` lists each of its methods once; and a fresh Claude session asked where a Rust symbol is defined reaches the index's MCP search tool before grep. -- evidence: intent/st/ST0076/ac-00.1-drives.md -- satisfied: yes

### WP-01 -- Typed definitions: kind, container, arity and span, one row per syntax node, with an extractor version (status: Done)

- AC-01.1 For Rust and Elixir, every definition row carries its `subkind`, `container`, `container_kind`, `trait_name`, `arity` and `arity_min` as ST0076's design vocabulary table states, read from the file's own syntax only. -- satisfied: yes (computed)
- AC-01.2 One symbol row per (name node, kind) in every language, fixed at extraction and never by a `DISTINCT` at read; a definition's own name is never also a reference to it. -- satisfied: yes (computed)
- AC-01.3 A code file whose symbols an older extractor wrote is re-extracted by the next reconcile, and a store migrated to the typed schema keeps its rows until then; two row shapes never answer one query. -- satisfied: yes (computed)

### WP-02 -- Rust qualified references: scoped calls, type uses and macro token trees, with the qualifier (status: Done)

- AC-02.1 Rust references by syntax include scoped calls (`Type::f(..)`), type uses in signatures and bodies, and names inside macro invocations, each with its qualifier as written and `level` 2; `intent search --context nearest_project` on this repository lists `views.rs:437` and `views.rs:439` (issue 0429). -- satisfied: yes (computed)
- AC-02.2 Every reference row spans the name as written, not the enclosing call or item, and the change raises the extractor version so stored rows re-extract. -- satisfied: yes (computed)

### WP-03 -- Elixir qualified references: remote calls with their module, captures, pipes, alias expansion, use, import and require (status: Done)

- AC-03.1 Elixir references by syntax include remote calls, remote captures (`&Mod.fun/2`) and remote pipe targets, each carrying its module as the qualifier with `level` 2 and arity where written; local calls, local captures (`&fun/1`, with their arity) and local pipe targets are unqualified at `level` 1; and `alias`, `import`, `require` and `use` are references at `level` 1 whose name is the whole module. A module in a qualifier or a directive name has the file's own `alias` forms and `__MODULE__` expanded within the enclosing do-block, and nothing from `import` or `use` applied. -- satisfied: yes (computed)

### WP-04 -- Surfaces: search by kind and container, qualified context naming its level, SQL columns, MCP parameters and instructions (status: Done)

- AC-04.1 `intent search` filters by hit kind (`--kind def`) and, as a separate flag, by symbol subkind (`--subkind struct`, from a roster derived from the query sources and refused by name when unknown) and by container (`--in <container>`); a search with `--subkind` or `--in` and no query lists the symbols that pass those filters, and a search with nothing to search for is refused with a remedy naming that form; `--context` and `--outline` honour every filter; `--context` prints each reference with its qualifier and names the level that answered; the SQL door and `intent schema` expose the new columns under the same names as the JSON hit; and the MCP search tool takes the same filters, the query-less form included. -- satisfied: yes (computed)
- AC-04.2 The search row's `when_to_use`, the MCP `instructions` line and the explorer's hits say what the index now answers and what it still does not, per level and per language, with no claim wider than the build. -- satisfied: yes (computed)

### WP-05 -- Rust resolved references through rust-analyzer's SCIP export (status: WIP)

- AC-05.1 Rust level 3, on an explicit verb only and never from intentd unasked: rust-analyzer's SCIP export is read into the store, and a resolved row always joins a written reference (same file, line and name); `intent index status` counts matched, unmatched, dropped and ambiguous, and the paths whose resolved rows are stale; the export builds into a directory of Intent's own under `intent/.cache`; a missing rust-analyzer is named in the envelope with its reason and never answers as an empty tier. -- satisfied: no (computed)

### WP-06 -- Elixir resolved references through the compiler's tracer, on an explicit verb (status: Not Started)

- AC-06.1 Elixir level 3, on an explicit verb only and never from intentd unasked: the compiler tracer's events join written references (same file, line and name) and expansion events are not stored; the compile builds into a directory of Intent's own under `intent/.cache`, seeded from the project's build when one exists, and never writes the project's `_build`; the tracer never raises; a failed compile stores nothing new, names the failure, and marks earlier rows stale by content hash; rows are replaced per file; the first run and `--full` force the compile, and every other run is incremental. -- satisfied: no (computed)
- AC-06.2 (non-test) On a scratch copy of Laksa, the level-3 callers of `Map.get/2` are listed without any call to `Access.get`, `Process.get` or `Keyword.get`. -- satisfied: no

### WP-07 -- Level-3 surfaces: resolution in the envelope, resolved facts on a hit, and search by target (status: WIP)

- AC-07.1 Every search answer carries `index.resolution`, index-wide, naming each language whose level 3 is not current as missing, failed, stale (with its paths) or unresolved, in words one function owns and the register quotes verbatim; a reference whose key joins exactly one current resolved row answers at `level` 3 with `target`, `target_path` and `target_line`, one joining several keeps its syntax level and carries them as `candidates`, and one in a stale file keeps its syntax level; the terminal, the explorer and the MCP tool say the same. -- satisfied: no (computed)
- AC-07.2 `intent search --target <target>`, the MCP tool's `target` and the SQL door's `resolved.target` ask one question: the written references a current resolved row joins to exactly that target, a key that names other targets too marked as one of N; it narrows and is narrowed by every other filter, lists the references alone when nothing else is asked, keeps no definition, and takes no hit from a stale file, which the note names; a target no resolved row names is refused with the nearest targets when any end the same way, and otherwise its empty answer says that a target nothing references and a misspelt one read the same; and a build with no resolver, or an index with no stored resolution, is refused with its remedy and never answered as an empty list. -- satisfied: no (computed)

## Acceptance Tests

### ST-level

_(no tests in this group)_

### WP-01 -- Typed definitions: kind, container, arity and span, one row per syntax node, with an extractor version (status: Done)

- AT-01.1 `native/rust/crates/intentsvcs/tests/symbols_are_typed_definitions.rs` -- covers AC-01.1, AC-01.2 -- status: green -- Red on 655bec8a6 before WP-01's first bank: the arms name columns and rows HEAD did not have. Green at f030c8804 (WP-01 landing, patch-id 829dca8a, vc's GO): Rust and Elixir typed definitions with container, trait and arity; an Elixir definition is never a reference to itself (red on the first bank with five self-references). Gate: whole intentsvcs and intent-cli suites.
- AT-01.2 `native/rust/crates/intentsvcs/tests/one_definition_is_one_symbol_row.rs` -- covers AC-01.2 -- status: green -- Red on 655bec8a6 before WP-01's first bank: the arms name columns and rows HEAD did not have. Green at f030c8804 (WP-01 landing, patch-id 829dca8a, vc's GO): a Rust and a Swift method defined once are one definition row, red on HEAD before WP-01. Gate: whole intentsvcs and intent-cli suites.
- AT-01.3 `native/rust/crates/intentsvcs/tests/symbols_are_typed_definitions.rs` -- covers AC-01.3 -- status: green -- Red on 655bec8a6 before WP-01's first bank: the arms name columns and rows HEAD did not have. Green at f030c8804 (WP-01 landing, patch-id 829dca8a, vc's GO): a file whose version is cleared and rows untyped is re-extracted by the next refresh and no untyped row survives; store_schema_version's earlier-draft arm walks a rung-10 store through the rung 28 rebuilds carrying rows. Gate: whole intentsvcs and intent-cli suites.

### WP-02 -- Rust qualified references: scoped calls, type uses and macro token trees, with the qualifier (status: Done)

- AT-02.1 `native/rust/crates/intentsvcs/tests/symbols_are_qualified_references.rs` -- covers AC-02.1 -- status: green -- Red on 35c238658 before WP-02: main had no qualifier, no token-tree patterns and call spans over the whole call. Green at d3b69c7de (WP-02 landing, patch-id 8d7b5cbc, vc's GO): scoped calls, uses, paths and type uses keep their qualifier at level 2, the last segment of a qualifier is a reference, and macro token trees give call, path, macro and token rows. Proof on a clone of main with the worktree binary: search --context nearest_project answers views.rs:212 def, run.rs:861, views.rs:390, 418, 437 and 439 -- grep's 6 lines, 437 and 439 inside assert_eq!.
- AT-02.2 `native/rust/crates/intentsvcs/tests/symbols_are_qualified_references.rs` -- covers AC-02.2 -- status: green -- Red on 35c238658 before WP-02: main had no qualifier, no token-tree patterns and call spans over the whole call. Green at d3b69c7de (WP-02 landing, patch-id 8d7b5cbc, vc's GO): a call with arguments over many lines spans its name's line, every reference pattern captures its name node, and EXTRACTOR_VERSION is 2 so rows WP-01 wrote re-extract.

### WP-03 -- Elixir qualified references: remote calls with their module, captures, pipes, alias expansion, use, import and require (status: Done)

- AT-03.1 `native/rust/crates/intentsvcs/tests/symbols_are_qualified_elixir_references.rs` -- covers AC-03.1 -- status: green -- Red on d4d61052c before WP-03, with cc's shapes (patch-id 69d6653d) and the query as it stood: all ten arms failed in wt-wp03 on its own target at 13:27Z, against the query whose published gap was that a remote call such as Repo.get(..) is a reference to get without its module and alias is not expanded. Green at 997d760b1 (WP-03 landing, patch-id 4b4ef24b, vc's GO, judged with cc's shapes as one train on d4d61052c): remote calls, captures and pipe targets carry their module at level 2 with the arity written or piped; local calls, local captures and local pipe targets stay unqualified at level 1; use, import, require and alias name the whole module, one row per brace entry; the file's aliases and `__MODULE__` expand within the enclosing do-block.

### WP-04 -- Surfaces: search by kind and container, qualified context naming its level, SQL columns, MCP parameters and instructions (status: Done)

- AT-04.1 `native/rust/crates/intent-cli/tests/the_structural_doors_take_the_filters.rs` -- covers AC-04.1 -- status: green -- Red on 70c65b464 before WP-04: 4 of its 5 arms fail there, because `--subkind` and `--in` are unknown arguments and a context hit carries no container, so the control reads `-` for both definitions of `new`; its SQL-door arm passes there, because WP-01 gave `symbols` its columns. Green at e5b7feb1b (WP-04 landing, patch-id 013ce9d4, vc's GO): `--context` and `--outline` honour `--kind`, `--lang`, `--subkind`, `--in` and `--limit`, and `--tier lexical` on a structural door is refused; a context hit names its subkind, container kind, arity and level; `--subkind method --in AddressError` with no query lists `is_local` alone, `--in AddressError --kind def` lists `new` and `is_local`, and the MCP tool answers the same groups; a search with nothing to search for is refused on both faces, naming `--subkind` and `--in`; an unknown subkind is refused in the same words by both; the SQL door and `schema ddl.sql` carry the symbol columns. Gate: whole intentsvcs, intent-cli and intentd suites.
- AT-04.2 `native/rust/crates/intentsvcs/tests/a_subkind_is_a_word_the_index_writes.rs` -- covers AC-04.1 -- status: green -- Red on 70c65b464 before WP-04: the arm does not compile there, because the index has no subkind roster (`symbols::subkinds`) and search has no filter words (`search::FilterWords`). Green at e5b7feb1b (WP-04 landing, patch-id 013ce9d4, vc's GO): every subkind the extractor writes for a Rust and an Elixir fixture is in its language's roster, Elixir's clause words come from elixir.scm, a language with no query has no roster, and an unknown subkind is refused with the roster of the languages in scope.
- AT-04.3 `native/rust/crates/intent-cli/tests/the_search_pane_is_resident.rs` -- covers AC-04.2 -- status: green -- Red on 70c65b464 before WP-04: the arm does not compile there, because a hit has no `symbol` facts and `run::arrive` takes no note; driven in a pty before the fix, the pane's info row held only the key hints (issue 0435). Green at e5b7feb1b (WP-04 landing, patch-id 013ce9d4, vc's GO): a symbol hit's row reads `ref call in Other`, the note names level 1 and Rust's macro gap, `run::arrive` puts it on the screen's hint, and a view with no note clears it (issue 0435).
- AT-04.4 `native/rust/crates/intent-cli/src/mcp.rs` -- covers AC-04.2 -- status: green -- Red on 70c65b464 before WP-04: the arm does not compile there, because neither `search::level_words` nor `symbols::what_a_reference_misses` exists. Green at e5b7feb1b (WP-04 landing, patch-id 013ce9d4, vc's GO): the search row's `when_to_use` and the MCP instructions carry level 1's and level 2's words and every language's gap from `what_a_reference_misses`, verbatim.

### WP-05 -- Rust resolved references through rust-analyzer's SCIP export (status: WIP)

_(no tests in this group)_

### WP-06 -- Elixir resolved references through the compiler's tracer, on an explicit verb (status: Not Started)

_(no tests in this group)_

### WP-07 -- Level-3 surfaces: resolution in the envelope, resolved facts on a hit, and search by target (status: WIP)

_(no tests in this group)_

---

_Generated by Intent v3.0.3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
