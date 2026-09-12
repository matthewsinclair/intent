---
st_id: ST0069
title: v3 post-cut: project search, store-backed coordination, and contract drift
---

# ST0069: v3 post-cut: project search, store-backed coordination, and contract drift -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-13 -- Project search: full-text, structural, and the agent search surface (status: Cancelled)

- AC-13.1 **(hv-RATIFIED 2026-08-15 as D31 -- was vc-specced under standing authorisation, which is what blocked ic's register row)** `treeindex` and the `in-handoff` skill are retired whole -- command, `intent/.treeindex/` cache, `/in-essentials` rules 3 and 4, and every canon reference -- and nothing in the repo references either

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Shipped in 3.0.1 as T0: intent treeindex and intent fileindex refuse as retired and the in-handoff skill is gone; nothing remains to satisfy here (by vc)
- AC-13.2 The index scope is the gitignore-aware repository, not `intent/**`: a source file is indexed, and a gitignored file never appears in any result

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-18.1 with its refinement; text carried forward (by vc)
- AC-13.3 Lexical (FTS5) search returns hits across prose and source in the one result shape `{path, span, kind, tier, score, snippet}`

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-19.1 and AC-19.4; text carried forward (by vc)
- AC-13.4 Structural (tree-sitter) search returns definition and reference hits for every language in the project's `languages` array, and a language absent from that array loads no grammar

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-20.1, AC-20.2 and AC-20.5; text carried forward (by vc)
- AC-13.5 The two-corpora staleness policy holds: `intent/**` hashes always (D24), the source corpus is stat-then-hash, and each is asserted against its own missed-edit case

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-18.3; text carried forward unchanged (by vc)
- AC-13.6 A stale or partial index is named at query time or the query refuses with the remedy; a confident subset is never returned

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-19.3; text carried forward unchanged (by vc)
- AC-13.7 intentd maintains the index incrementally in the background, and a daemonless query returns identical results to a daemon-served one

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-22.1 and AC-22.2; text carried forward (by vc)
- AC-13.8 The MCP search tool and `intent search` return the same result shape from the same index -- one surface, two skins

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-19.2; text carried forward (by vc)
- AC-13.9 (non-test) T3 (semantic) and T4 (type-aware) are specified in design.md as staged additions, with S1-S5 shown sufficient to admit them without changing the CLI contract or the MCP tool schema

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- WITHDRAWN: Re-keyed at the 2026-09-12 re-elaboration to AC-23.1, AC-23.2 and AC-23.4; text carried forward (by vc)

### WP-14 -- Coordination model: whiteboard and inboxes in the store, with a bounded API (status: Not Started)

- AC-14.1 `wb_node`, `wb_item` and `wb_message` are model entities with schema faces, held in the DB per the reversed D01, and **sync round-trips them losslessly in both directions**: db-to-disk then disk-to-db reproduces the DB content, and the regenerated files are byte-identical to the committed ones. (Reworded 2026-08-15 for D01's reversal. The original asserted a `rm intent.db` rebuild from committed JSON canon; the property under test is now the round-trip rather than the rebuild, and it is a stronger claim because it binds both directions)

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.10 (non-test) `/in-whiteboard` and the `intent claude ws` family are updated in this WP, so the protocol documents no workflow the tool refuses

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no
- AC-14.11 No modelled entity carries a caller-authored timestamp (D33, project-wide and not whiteboard-local): every stamped field is written by the SERVICE layer from the clock at the moment of the event, **once**, and is never re-derived by a later sync in EITHER direction. A DB-side column default is refused as the mechanism: with the DB as truth (D01 as reversed) and sync running both ways, a disk-to-db resync that re-inserts rows lets `DEFAULT CURRENT_TIMESTAMP` re-stamp them, rewriting history silently and indistinguishably from a correct value -- the fabricated-stamp failure shape reintroduced by its own fix. Discriminating case: stamp an entity, record the value, round-trip it through db-to-disk and disk-to-db, assert byte-identical -- a test that only asserts a stamp EXISTS after the round-trip passes on the defect. (Reworded 2026-08-15: the original argued this from the DB being rebuildable, which D01's reversal removed; the requirement survives with its reasoning inverted, which is why it is now stated independently of which side is durable)

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.12 (non-test) **WP-14 DOES NOT CLOSE WHILE THE FILE-BASED `claude ws` FAMILY STILL EXISTS.** The four verbs `cmd_ws_new` / `cmd_ws_list` / `cmd_ws_archive` / `cmd_ws_hygiene` are gone from `intent/plugins/claude/bin/intent_claude_cwi`, their dispatch arms with them, and `AC-14.7`'s store-served `intent wb` is what answers instead. **THIS ROW EXISTS SOLELY TO GIVE `AC-12.1`'s CUT EXCEPTION AN END THAT IS ENFORCED RATHER THAN REMEMBERED** -- hv ruled the exception dated, and a date nobody checks is not a date. Because an unsatisfied AC blocks `wp done`, the exception is retired BY THE GATE, at the moment WP-14 tries to close, whether or not anyone recalls why the row is here. **MECHANICALLY CHECKABLE AND DELIBERATELY LEFT `non-test` FOR NOW:** the check is that `intent_claude_cwi` carries no `cmd_ws_*` definition or dispatch arm. It is `non-test` to match its sibling `AC-14.10` and because WP-14 is Not Started with no harness to hang an AT on; **WHOEVER BUILDS WP-14 SHOULD LIFT THIS SENTENCE INTO AN AT RATHER THAN SATISFYING IT BY READING.** **AND THE EXCEPTION HAS A SECOND DISCHARGE ROUTE THAT IS NOT THIS ROW: WP-07 PORTING `cwi` OFF `bin/intent_helpers` RETIRES `AC-12.1`'s DEPENDENCY WITHOUT DELETING A VERB.** If WP-07 lands first, this row is still owed -- the verbs are still file-based, still outside the store, and `AC-14.7` is still unmet. **The two routes retire DIFFERENT things and neither substitutes for the other.** Ruled by hv 2026-08-25, recorded by vc under the pen, first-hand.

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no
- AC-14.2 `wip.md` and `inbox.<sender>.md` are 100% generated views: regeneration reproduces the committed bytes exactly, through the repository formatter and not merely through the renderer

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.3 An over-bound write is refused by name, stating the bound and the remedy; nothing is truncated and nothing over-bound is accepted, with a body one byte over the limit as the discriminating case

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.4 Every timestamp is read from the clock by the API, and a caller-supplied timestamp is refused rather than honoured -- the fabricated-stamp class closed by construction, not by detection

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.5 The single-writer invariant is enforced by the API rather than by convention: a node writing another node's board, or an inbox it does not own, is refused

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.6 Archival is the API's: handled messages and completed DOING items transition on schedule, and no node action is required for a board to stay within its bounds

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.7 Every `/in-whiteboard` verb is served by `intent wb` from the store, in-process and over GraphQL, and any workstream can read any node's board

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.8 Boards and inboxes are FTS-indexed and reachable from `intent search` with the same result shape as the rest of the corpus

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-14.9 The existing three-node board migrates into the model with nothing dropped silently: what cannot be carried is named per item, and the count of carried items reconciles against the source

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)

### WP-16 -- Contract drift: a shipped field with no model row is refused (status: Not Started)

- AC-16.1 **A check joins the schema property set to `data-model.md`'s table rows per entity and REFUSES IN BOTH DIRECTIONS**: a property with no row (a field shipped with no contract), and a row with no property (a contract describing something nobody implemented). Both, because they fail differently and the second is the more dangerous half -- it is what made `issue.body` look specified while it was homeless. **It is a SIBLING of `drift_check.sh`, never an arm of it**: that check compares generated faces to the Rust types, this compares a prose document to a schema, and folding them puts two questions behind one exit code.

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-16.2 **THE CHECK RUNS OVER EVERY MODELLED ENTITY AND PRINTS ITS DENOMINATOR.** `N of N entities checked, M properties` in the output -- because a check that silently skipped an entity and a check that found nothing produce the same line otherwise. **THE POPULATION IS DERIVED FROM THE SCHEMA AT RUN TIME AND IS NOT ENUMERATED HERE.** This row carried a nine-name list until 2026-08-29 and the list was WRONG -- derivation yields eleven, the two it omitted being `Invoker` and `Subject` -- **so the criterion warning against "a list maintained beside it" was itself that list, in the same sentence.** (vc, on ic's measurement; the row is reworded rather than corrected in place because the enumeration was the defect, not its contents.) **`Invoker` is the live proof: it is growing under ST0066 AC-00.2's invoker evidence right now, and an entity outside the checked population grows uncontracted, which is `issue.body`'s exact shape and the 434KB this WP exists because of.**

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-16.3 **A table the parser cannot read FAILS LOUDLY and is never skipped.** `data-model.md` is hand-authored markdown with prose between its tables, so an unparseable table is the likely case rather than the exotic one -- and a skipped entity reported as checked is the defect this WP exists to remove, one level up. The refusal names the entity and the table it could not read.

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)
- AC-16.4 **The check is demonstrated RED in both directions before any green is trusted.** Delete a row from a table and watch the right entity fail; add a row for a property that does not exist and watch the other direction fail. **A check whose positive case has never been observed is not evidence**, and both arms are needed because a one-sided demonstration leaves the untested direction free to be a constant pass.

**MOVED FROM `ST0056` ON 2026-08-30 (hv's ruling, performed by vc).** The requirement is UNCHANGED and UNBUILT; only the thread it is counted against moved, so that `ST0056`'s gate measures what 3.0.1 ships rather than what v3 eventually will. **This was not a descope for a green** -- `ST0056` remained BLOCKED across the move. hv's own sequencing is the warrant: *fully ship v3 with all functionality, intentd is one of those priorities; once that's done, we can do tree-sitter and full search.* -- satisfied: no (computed)

### WP-17 -- The structured query door: intent search --sql, read-only over the published schema (status: Not Started)

- AC-17.1 `intent search --sql <statement>` runs one read statement on a connection opened read-only and returns rows; a write, a second statement or a state-changing pragma is refused with the remedy naming the read-only contract. -- satisfied: no (computed)
- AC-17.2 The rows carry the store's schema version, and `--json` emits the same envelope the MCP tool returns for the same statement. -- satisfied: no (computed)
- AC-17.3 A bare query is text and `--sql` is the only structured door; nothing is auto-detected from the query's first word. -- satisfied: no (computed)
- AC-17.4 A result capped by `--limit` reports both denominators, matched and returned; a capped result is never a silent subset. -- satisfied: no (computed)

### WP-18 -- The corpus: the gitignore-aware repository, two staleness policies, the widened watcher (status: Not Started)

- AC-18.1 The index scope is the gitignore-aware repository: a tracked source file is indexed, a gitignored file never appears in a result, and `.git`, the store's own directory and the backup directory are outside by rule rather than by path shape. -- satisfied: no (computed)
- AC-18.2 Every in-scope file the index does not hold has a `file_index` row naming why (binary, over the size cap, unreadable, symlink), and `intent index status` lists them; nothing is skipped silently. -- satisfied: no (computed)
- AC-18.3 Two corpora, two staleness policies: canon hashes always; source is stat-then-hash; each is asserted against its own missed-edit case. -- satisfied: no (computed)
- AC-18.4 The scanner and the watcher read one scope object: a change under a source directory reaches the index through the daemon without a second statement of scope. -- satisfied: no (computed)
- AC-18.5 Rendered views under `intent/st/` and the canon extract are not in the disk corpus; an entity's prose is indexed once, from the store. -- satisfied: no (computed)
- AC-18.6 (non-test) The watcher's registration strategy is recorded per platform with the measurement that chose it. -- satisfied: no

### WP-19 -- Lexical search over the whole corpus: the envelope, --json, the MCP tool, intent index (status: Not Started)

- AC-19.1 Lexical search returns hits across prose and source in one envelope, grouped by tier, ranked within a tier, never blended. -- satisfied: no (computed)
- AC-19.2 `--json` on the CLI and the MCP tool return the same envelope from the same facade call; `--format` is terminal-channel only and not exposed on MCP. -- satisfied: no (computed)
- AC-19.3 A stale, partial or empty index is named in the envelope and on the terminal, or the query refuses with the remedy; a confident subset is never returned. -- satisfied: no (computed)
- AC-19.4 Source text is tokenised without stemming and snake_case splits into its words; the tokeniser decision carries its measured recall on an identifier fixture. -- satisfied: no (computed)
- AC-19.5 A hit's line is reported only when the indexed bytes match the disk; otherwise the hit carries no line and says stale. -- satisfied: no (computed)
- AC-19.6 `intent index status` and `intent index rebuild` exist, report measured sizes, and are registered and exposed like every verb. -- satisfied: no (computed)

### WP-20 -- Structural search: tree-sitter symbols per declared language, and the agent canon that uses them (status: Not Started)

- AC-20.1 Definition and name-matched reference hits are returned for every language in the project's `languages` array, and a language absent from the array parses nothing. -- satisfied: no (computed)
- AC-20.2 Symbols come from each grammar's own tags query; adding a language is a grammar and nothing else. -- satisfied: no (computed)
- AC-20.3 `intent search --kind def <name>` answers whether a thing with that name already exists, from the tree. -- satisfied: no (computed)
- AC-20.4 (non-test) The binary-size delta of each grammar is measured and recorded before it ships; hv rules on any grammar above the line hv sets. -- satisfied: no
- AC-20.5 References are named as name-matched on every surface and never as callers. -- satisfied: no (computed)
- AC-20.6 Every skill, template and rule that names `intent modules find` for a lookup names `intent search --kind def` instead, and the agent guide regenerates; `intent modules find` retires on hv's ruling. -- satisfied: no (computed)

### WP-21 -- The explorer's /search pane (status: Not Started)

- AC-21.1 `/search <query>` in the explorer opens a resident results pane whose rows are the envelope's hits, with the freshness line in the INFO section. -- satisfied: no (computed)
- AC-21.2 Enter on an entity hit lands on its view; Enter on a file hit opens the file at its line through the lent terminal. -- satisfied: no (computed)
- AC-21.3 The pane calls the same facade method as the CLI, and the TUI's pure layers stay pure. -- satisfied: no (computed)

### WP-22 -- Daemon-served search with daemonless parity (status: Not Started)

- AC-22.1 intentd maintains the index incrementally; a daemonless query reconciles first and returns results identical to a daemon-served one for the same tree state. -- satisfied: no (computed)
- AC-22.2 `--no-reconcile` answers from the index as it stands and names what moved. -- satisfied: no (computed)
- AC-22.3 `Op::Search` crosses the wire carrying the same envelope. -- satisfied: no (computed)

### WP-23 -- Semantic seams: the embedder interface, the Null and HTTP embedders, the vector schema (status: Not Started)

- AC-23.1 (non-test) T3 and T4 are specified as staged additions, and the T1 and T2 build proves the seams: no CLI contract change and no MCP schema change admits a tier. -- satisfied: no
- AC-23.2 The `Embedder` interface exists with a Null implementation that refuses a semantic query with the remedy naming the configuration, and an HTTP implementation against an OpenAI-compatible endpoint from `config.json`. -- satisfied: no (computed)
- AC-23.3 The vector schema is a recorded migration; semantic hits are a tier group ranked within itself. -- satisfied: no (computed)
- AC-23.4 (non-test) The local-runtime decision is put to hv with the measured costs of each shape. -- satisfied: no

### WP-24 -- The LLM boundary: the harness's own search becomes a door into the index (status: Not Started)

- AC-24.1 A `.mcp.json` naming `intent mcp` is part of the canon `claude upgrade --apply` seeds when absent and never overwrites, so every project's session sees the tools without configuration. -- satisfied: no (computed)
- AC-24.2 The MCP search tools' descriptions state when to use them and when not to, in the terms a model matches on, and are generated from the register rows like every tool description. -- satisfied: no (computed)
- AC-24.3 `intent search --outline <path>` returns a file's symbols with spans, and `intent search --context <name>` returns a definition and its name-matched references as source spans; each is one facade call and one MCP tool call, in the envelope. -- satisfied: no (computed)
- AC-24.4 A PostToolUse hook served by the install appends the index's structural answer for the symbol a grep pattern named; it never blocks, and it appends nothing when the envelope says the index is not complete for the paths involved. -- satisfied: no (computed)
- AC-24.5 Every skill, template and rule that tells the model how to find code names the index verbs, the agent guide regenerates, and grep is named as the fallback for when the envelope says the index is not complete. -- satisfied: no (computed)
- AC-24.6 (non-test) The PreToolUse redirect of symbol-shaped grep patterns is specified with its safety condition, the freshness contract holding, and is not built until hv rules it on. -- satisfied: no
- AC-24.7 (non-test) Stretch: a symbol hit names the thread and criterion that introduced it, through the commit references the store and git already hold; specified in the design with its data source before it is built. -- satisfied: no

## Acceptance Tests

### WP-13 -- Project search: full-text, structural, and the agent search surface (status: Cancelled)

_(no tests in this group)_

### WP-14 -- Coordination model: whiteboard and inboxes in the store, with a bounded API (status: Not Started)

_(no tests in this group)_

### WP-16 -- Contract drift: a shipped field with no model row is refused (status: Not Started)

_(no tests in this group)_

### WP-17 -- The structured query door: intent search --sql, read-only over the published schema (status: Not Started)

_(no tests in this group)_

### WP-18 -- The corpus: the gitignore-aware repository, two staleness policies, the widened watcher (status: Not Started)

_(no tests in this group)_

### WP-19 -- Lexical search over the whole corpus: the envelope, --json, the MCP tool, intent index (status: Not Started)

_(no tests in this group)_

### WP-20 -- Structural search: tree-sitter symbols per declared language, and the agent canon that uses them (status: Not Started)

_(no tests in this group)_

### WP-21 -- The explorer's /search pane (status: Not Started)

_(no tests in this group)_

### WP-22 -- Daemon-served search with daemonless parity (status: Not Started)

_(no tests in this group)_

### WP-23 -- Semantic seams: the embedder interface, the Null and HTTP embedders, the vector schema (status: Not Started)

_(no tests in this group)_

### WP-24 -- The LLM boundary: the harness's own search becomes a door into the index (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3.0.1 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
