---
verblock: "14 Aug 2026:v0.1: matts - Initial version"
st_id: ST0056
title: "Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files that exposes an MCP server with full API access to Intent -- acceptance contract"
---

# ST0056 Add a Rust-based CLI with a local SQLite DB with bidirectional sync to/from .md files that exposes an MCP server with full API access to Intent -- Acceptance

> Canonical acceptance contract for ST0056. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a. `n/a` belongs to non-test rows ONLY -- it is the doc / eyeball / gate status, and a row carrying it must be marked `(non-test)`.
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). A `(non-test)` AT RECORDS a doc / eyeball check; it never satisfies anything, because `n/a` is not green -- the satisfaction lives on the AC's own `(non-test)` line.
>
> An AC has four states, not two. Beyond satisfied and unsatisfied, a requirement can leave this thread's scope while remaining real: **descoped** (it moved to a named thread -- `intent ac descope <ID> <AC> --to <ID>`) or **withdrawn** (it was dropped outright, with its reason on the record -- `intent ac withdraw <ID> <AC> --reason "..."`). Both are non-blocking and both are reported separately rather than folded into the satisfied count, so a thread that descoped half its contract looks like one. Use them instead of the two dishonest alternatives: satisfying an AC whose work was not done, or deleting the line and losing the audit trail. `intent ac rescope` / `intent ac reinstate` undo them.
>
> **The AT row has an enforced grammar (`intent at lint`, and the close-gate).** Two shapes, and nothing else parses:
>
> ```
> - AT-<gg>.<n> `<repo-relative-path>` -- covers <AC-id>[, <AC-id>...] -- status: to-write|red|green[ -- <free note>]
> - AT-<gg>.<n> (non-test) <prose> -- covers <AC-id>[, <AC-id>...] -- status: n/a[ -- <free note>]
> ```
>
> The reference is the test FILE, backticked, repo-relative, with at least one `/` and no `:` -- not a test name, not a bare filename, not a selector. Name the test by putting the AT's own id INSIDE the test (`describe "AT-03.2 / AC-03.2: ..."`), which is checkable from both ends and survives rewording; a cited name is not. Coverage ids are comma-separated with nothing fused to them (no `and`, no trailing `:`, no possessive). Any trailing note is introduced by a spaced `--` separator, exactly as in the two shapes above, and is never parsed. `intent at lint <ID> --fix` migrates the mechanical part of a legacy contract.
>
> Exemption (ST0048): the close-gate is fail-by-default -- a unit with an empty or missing contract is refused. A unit that is deliberately AC-free (eg a pure content / authorial task) declares `acceptance: exempt` in the frontmatter above; the gate then passes and announces the exemption. Omit it (the default) and the contract is enforced. Never inferred from emptiness; always declared.

## Acceptance Criteria

> WP-02..12 AC/AT groups are a WP-01 deliverable (AC-01.2) and land with its completion. The ST-level group below is the 3.0.0 release gate.

### ST-level -- the v3.0.0 gate

- AC-00.1 The v3 binary passes the narrowed BATS conformance contract (stdout, exit codes, behaviour), with every file-layout divergence recorded in the ratified keep/retire/deviate register, **and no command family has zero burning coverage** -- a contract with a hole passes trivially at that hole (strengthened by vc 2026-08-14 on ic's `config` finding; ADOPTED under hv standing authorisation)
- AC-00.2 The fleet corpus (Intent's own tree as canary, then Lamplight/Utilz/Baize at named post-sweep revisions) ingests losslessly, or every unconverted artefact appears in the residue report by name
- AC-00.3 intentd ships in the release: project registry, unix-socket GraphQL, mgmt plane, debounced watching, CLI-owned launchd lifecycle
- AC-00.4 MCP ships: stdio server with the tiered typed tools + `intent_graphql` escape hatch, bridging to intentd when it is up
- AC-00.5 (non-test) `brew install` on a clean macOS machine yields a working `intent` + daemon lifecycle -- evidence: install transcript in the release record -- satisfied: no
- AC-00.6 (non-test) The shell implementation (`bin/`) is pruned at the cut, fail-forward -- evidence: the v3.0.0 release diff -- satisfied: no
- AC-00.7 `rusqlite` appears in exactly one Cargo.toml (intentsvcs) and the dual-path conformance suite (in-process vs intentd, identical results) is green across the verb surface
- AC-00.8 A v2.19.0 project migrates in one visible commit with refuse-lossy residue named, `project_id` stamped, hooks and settings.json unchanged, and documented git-revert rollback

### WP-01 -- Design canon (status: WIP)

- AC-01.1 (non-test) design.md carries the ratified architecture, truth model, decision log D01-D17, alternatives, and stack shortlist -- evidence: hv check-in 2026-08-14: 'ST0056/WP01: Ratified' -- satisfied: yes
- AC-01.2 (non-test) The full-ladder acceptance contract (WP-02..12 AC/AT groups) is authored and ratified -- evidence: hv ratification 2026-08-14: full-ladder contract approved at check-in -- satisfied: yes
- AC-01.3 (non-test) The parity contract exists: v2 command-surface inventory + the keep/retire/deviate register format -- evidence: intent/st/ST0056/parity.md -- satisfied: yes
- AC-01.4 (non-test) The design open questions are closed (one binary vs two, launchd label, 3.0.0 subscription extent, `.cache` layout) -- evidence: design.md D18-D21 -- satisfied: yes

### WP-02 -- Workspace and reified model (status: Not Started)

- AC-02.1 (non-test) The cargo workspace builds with fmt + clippy gates on macOS and Linux CI -- evidence: rust CI run 31812129560 green on 736033d -- macOS+Linux matrix, fmt --check + clippy -D warnings + tests (first run) -- satisfied: yes
- AC-02.2 The three schema faces (JSON Schema, DDL, SDL) are generated, committed, and CI fails on drift between types and committed faces
- AC-02.3 Deleting the DB and rebuilding from committed canon yields identical queryable content (the D01 disposability invariant)
- AC-02.4 Serialise/deserialise laws hold under proptest, and an unknown field in canonical JSON is refused by name, never dropped
- AC-02.5 `rusqlite` appears in exactly one Cargo.toml and CI asserts the dependency graph

### WP-03 -- Ingest, views and sync (status: Not Started)

- AC-03.1 Strict ingest refuses schema-invalid canon with file + finding named, exit non-zero, nothing partially loaded
- AC-03.2 View rendering is deterministic and idempotent: same model renders the same bytes, twice, on both platforms
- AC-03.3 The stat-scan detects external edits by content hash, including a same-size same-mtime rewrite
- AC-03.4 The skew check catches a hand-edited generated view and names the file (never silently outvotes it)
- AC-03.5 A conflict-markered artefact enters the named unparsed state; commands needing it refuse with the finding; v2's silent grep-through is unconstructible
- AC-03.6 Prose bodies ingest verbatim into FTS-indexed doc_sections; a body round-trips byte-identical out of the store and is retrievable by full-text query

### WP-04 -- intentsvcs facade, core families (status: Not Started)

- AC-04.1 st/wp lifecycle verbs (new, start, done, cancel, list, show) run through the facade with canon + views + DB written transactionally -- a failure mid-write leaves no torn state
- AC-04.2 ac/at operations implement the four AC states with computed satisfaction for test-backed ACs (never stored) and inline evidence for non-test
- AC-04.3 The close-gate reads the model and reproduces v2 gate verdicts -- including every one of v2's own gate rules that remains constructible in v3, each with its own distinguishable diagnosis, proven against contracts built to trip them rather than against a corpus that happens to be clean
- AC-04.4 Every facade error is typed and renders a remedy with its full cause chain (no same-text-for-different-causes collapses)
- AC-04.5 Every mutation path writes an event-log envelope carrying principal + project_id (the D15 seams exist end to end; renumbered from AC-02.6 at WP-02 close per the 2026-08-14 bounce ruling)

### WP-05 -- CLI in-process + conformance harness (status: Not Started)

- AC-05.1 The dispatch table is the SSOT: the clap surface and help text are generated from it, asserted by test
- AC-05.2 Core families are green under the v2 BATS estate via `INTENT_BIN` with voice and exit codes byte-compatible
- AC-05.3 (non-test) Every v2 test file is classified in the keep/retire/deviate register; no unclassified rows -- evidence: parity register at WP close -- satisfied: no
- AC-05.4 (non-test) The clap layer holds no business logic (parse -> facade -> render only) -- evidence: review + the AC-02.5 dep guard -- satisfied: no

### WP-06 -- CLI parity long tail (status: Not Started)

- AC-06.1 The full command surface is ported and the conformance suite is green minus register-recorded deviations only, with `coverage_map.sh` reporting no command family at zero burning coverage -- `intent config` lands a conformance test BEFORE its behaviour is designed, or the `undefined` ruling on it is unverifiable by construction
- AC-06.2 doctor is rebuilt as model/DB integrity queries + file checks including the skew check and unparsed-state reporting
- AC-06.3 (non-test) The register is complete: every deviation was recorded at land time, none discovered after -- evidence: register diff history -- satisfied: no
- AC-06.4 `intent search` returns hits across ST prose, issue bodies and WP text from the FTS index, in the shipped voice and exit codes (new surface with no v2 antecedent -- an ADDITION in the register, never a deviation)
- AC-06.5 `intent schema` prints the three committed faces (JSON Schema, DDL, SDL), and what it prints is byte-identical to the files under `schema/` (re-homed from WP-02's deliverable list -- see the WP-02 note below)

### WP-07 -- Canon and claude subsystem (status: Not Started)

- AC-07.1 A fresh `intent init` works offline from the binary alone (embedded canon); the 0022 broken-install class is unconstructible
- AC-07.2 `intent claude hook <name>` output is byte-compatible with v2 for every shipped hook; consumer settings.json untouched
- AC-07.3 Skills install/sync/uninstall reproduce v2 SHA256-manifest behaviour
- AC-07.4 The critic headless runner reads embedded rules and preserves the ST0039 strict-proxy contract
- AC-07.5 `intent agents sync` output matches v2 for a corpus project, modulo ratified deviations
- AC-07.6 Generated views have exactly one writer: `intent init` and migration converge a formatter-exclusion for tool-owned paths, and a format pass over a freshly-rendered view leaves it byte-unchanged in a consumer repo as well as this one

### WP-08 -- intentd (status: Not Started)

- AC-08.1 The registry serves N projects with per-connection binding; a moved or deleted root surfaces in doctor, never as a crash
- AC-08.2 The dual-path conformance suite is green across the verb surface (in-process vs daemon, identical results)
- AC-08.3 The CLI routing rule holds: socket present routes to the daemon, absent runs in-process, and two sync engines never run concurrently
- AC-08.4 Lifecycle works end to end: launchd install/uninstall, PID discipline with observable cleanup, logs in the named location
- AC-08.5 Debounced gitignore-aware watching drives ingest on external edits
- AC-08.6 `projectChanged` and `fileChanged` subscriptions deliver over the socket (D20 -- and nothing more in 3.0.0)
- AC-08.7 Policy-stamp self-healing works: a generated local artefact whose version marker is missing or stale is regenerated on boot, and an old install heals without a migration

### WP-09 -- MCP (status: Not Started)

- AC-09.1 The typed tool tier is generated from the dispatch table with schemars-derived parameter schemas
- AC-09.2 The `intent_graphql` escape hatch reaches the full API surface
- AC-09.3 Bridge mode survives a daemon restart mid-session via per-request target resolution
- AC-09.4 `intent llm` renders the agent guide from the dispatch table; no hand-maintained command list exists
- AC-09.5 MCP resources serve the read surfaces (wip, whiteboard boards, ST docs) and their contents match what the equivalent CLI read returns

### WP-10 -- Migration and fleet harness (status: Not Started)

- AC-10.1 A pre-2.19.0 project is refused with the two-hop instruction named
- AC-10.2 Phase A residue yields BLOCKED with the per-line classed report, atomic: nothing written, exit non-zero
- AC-10.3 A clean estate converts in one commit: structured canon emitted, views regenerated, config stamped 3.0.0 + project_id, DB built, gitignore converged
- AC-10.4 Hooks continuity: `.claude/settings.json` + `.claude/scripts/**` byte-identical pre/post, asserted not assumed
- AC-10.5 The fleet corpus (Intent canary, then Lamplight/Utilz/Baize at named post-sweep revisions) satisfies artefact conservation, semantic completeness, and prose conservation, or names its residue
- AC-10.6 (non-test) Rollback exercised for real on the canary: migrate, `git revert`, tree-identical -- evidence: canary record in impl.md -- satisfied: no

### WP-11 -- Distribution (status: Not Started)

- AC-11.1 (non-test) cargo-dist produces installable artefacts + the tap formula for both binaries -- evidence: release CI run -- satisfied: no
- AC-11.2 (non-test) The signing/notarisation decision is recorded and implemented -- evidence: decision-log entry + a notarised artefact -- satisfied: no
- AC-11.3 The binary is fully functional with no INTENT_HOME in the environment (demoted to dev override)

### WP-12 -- Cutover and release (status: Not Started)

- AC-12.1 The shell implementation is pruned at the cut and nothing in the repo references `bin/` intent scripts
- AC-12.2 (non-test) Docs converged: working-with-llms.md, canon narrative, README, CHANGELOG -- evidence: docs sweep commit -- satisfied: no
- AC-12.3 (non-test) Release docs written BEFORE the cut so the tag carries them -- evidence: tag contents -- satisfied: no
- AC-12.4 (non-test) v3.0.0 tagged on both remotes, GitHub release published, formula live -- evidence: release record -- satisfied: no

### WP-13 -- Project search and the agent search surface (status: Not Started)

> Tiered by hv's "reach for the stars, but not all at once". T0-T2 need no external model and are in the 3.0.0 gate; T3 (semantic) and T4 (type-aware) are specified now and land later, which is only safe because AC-13.9 proves the seams admit them. D01's rebuildable DB is what makes deferring a tier free -- adding vector tables later is a `rm intent.db`, never a migration.

- AC-13.1 `treeindex` and the `in-handoff` skill are retired whole -- command, `intent/.treeindex/` cache, `/in-essentials` rules 3 and 4, and every canon reference -- and nothing in the repo references either
- AC-13.2 The index scope is the gitignore-aware repository, not `intent/**`: a source file is indexed, and a gitignored file never appears in any result
- AC-13.3 Lexical (FTS5) search returns hits across prose and source in the one result shape `{path, span, kind, tier, score, snippet}`
- AC-13.4 Structural (tree-sitter) search returns definition and reference hits for every language in the project's `languages` array, and a language absent from that array loads no grammar
- AC-13.5 The two-corpora staleness policy holds: `intent/**` hashes always (D24), the source corpus is stat-then-hash, and each is asserted against its own missed-edit case
- AC-13.6 A stale or partial index is named at query time or the query refuses with the remedy; a confident subset is never returned
- AC-13.7 intentd maintains the index incrementally in the background, and a daemonless query returns identical results to a daemon-served one
- AC-13.8 The MCP search tool and `intent search` return the same result shape from the same index -- one surface, two skins
- AC-13.9 (non-test) T3 (semantic) and T4 (type-aware) are specified in design.md as staged additions, with S1-S5 shown sufficient to admit them without changing the CLI contract or the MCP tool schema -- evidence: design.md "Project search" section -- satisfied: no

## Acceptance Tests

### ST-level

- AT-00.1 `tests/conformance/run_v2_suite.bash` -- covers AC-00.1 -- status: to-write -- the v2 BATS estate run against the v3 binary via an INTENT_BIN override
- AT-00.2 `crates/intentsvcs/tests/fleet_corpus_ingest.rs` -- covers AC-00.2 -- status: to-write -- red-first against the canary tree
- AT-00.3 `crates/intentd/tests/daemon_lifecycle.rs` -- covers AC-00.3 -- status: to-write
- AT-00.4 `crates/intent-cli/tests/mcp_surface.rs` -- covers AC-00.4 -- status: to-write
- AT-00.5 `crates/intentsvcs/tests/dep_graph_guard.rs` -- covers AC-00.7 -- status: to-write -- asserts the rusqlite Highlander + drives the dual-path suite
- AT-00.6 `crates/intentsvcs/tests/migrate_v2_project.rs` -- covers AC-00.8 -- status: to-write
- Coverage: AC-00.5 and AC-00.6 are non-test (evidence on the AC lines); intended paths above are refined as the workspace lands in WP-02

### WP-01

- AT-01.1 (non-test) hv reads design.md at the pre-kickoff check-in and ratifies or amends -- covers AC-01.1 -- status: n/a
- AT-01.2 (non-test) this contract carries ratified WP-02..12 groups at WP-01 close -- covers AC-01.2 -- status: n/a
- AT-01.3 (non-test) parity inventory + register reviewed against `bin/` -- covers AC-01.3 -- status: n/a
- AT-01.4 (non-test) design.md decision log gains the four closures -- covers AC-01.4 -- status: n/a
- Coverage: every WP-01 AC has a recording AT; satisfaction lives on the AC lines

### WP-02

- AT-02.1 (non-test) first green CI run on macOS + Linux with fmt/clippy gates -- covers AC-02.1 -- status: n/a
- AT-02.2 `crates/intentsvcs/tests/schema_faces_drift.rs` -- covers AC-02.2 -- status: green
- AT-02.3 `crates/intentsvcs/tests/store_rebuild.rs` -- covers AC-02.3 -- status: green
- AT-02.4 `crates/intentsvcs/tests/model_laws.rs` -- covers AC-02.4 -- status: green
- AT-02.5 `crates/intentsvcs/tests/dep_graph_guard.rs` -- covers AC-02.5 -- status: green
- Coverage: complete; AC-02.1 is non-test with evidence on its line; the envelope requirement lives at AC-04.5 (renumbered at close)

**Finding (vc, 2026-08-14): WP-02 closed 5/5 with one of its named deliverables unbuilt and uncovered, and the gate could not have seen it.** WP-02's deliverable line reads "The three schema faces generated + committed + CI drift-checked; **`intent schema` prints them**". The first clause is built and is AC-02.2. The second is not: `crates/intent-cli/src/main.rs` is a 13-line placeholder that prints a scaffold string. No AC in any group covered the command, so the close gate read a complete AC set over an incomplete deliverable and returned PASS -- correctly, by its own contract.

The instance is trivial (`intent schema` is a small command, re-homed above as AC-06.5, and the placeholder is honestly self-described). **The class is not: nothing in the process reads the deliverable lists.** ACs are gated; `WP/<NN>/info.md` deliverables are prose nobody diffs against the contract. That holds for all twelve WPs, and it is how a WP ships green while a named deliverable quietly does not ship at all -- the same "green gate over unbuilt work" shape the close-gate was built to prevent, one level up.

Found by cross-checking all twelve deliverable lists against all sixty-two ACs, which is how AC-03.6, AC-06.5, AC-08.7 and AC-09.5 were found in one pass. Recommendation to hv: either the deliverable-to-AC cross-check becomes a standing step at WP close, or deliverable lists stop being written as if they were commitments. Doing neither leaves the gap open and now documented, which is the worst of the three.

### WP-03

- AT-03.1 `crates/intentsvcs/tests/ingest_refusal.rs` -- covers AC-03.1 -- status: green
- AT-03.2 `crates/intentsvcs/tests/view_determinism.rs` -- covers AC-03.2 -- status: green
- AT-03.3 `crates/intentsvcs/tests/sync_scan.rs` -- covers AC-03.3 -- status: green
- AT-03.4 `crates/intentsvcs/tests/view_skew_check.rs` -- covers AC-03.4 -- status: green
- AT-03.5 `crates/intentsvcs/tests/unparsed_state.rs` -- covers AC-03.5 -- status: green
- AT-03.6 `crates/intentsvcs/tests/prose_ingest_fts.rs` -- covers AC-03.6 -- status: green
- Coverage: complete

WP-03 dispositions (vc, 2026-08-14, ADOPTED under hv standing authorisation):

- **AC-03.6 / AT-03.6 are an ADDITION to the WP-01-ratified boundary**, not a reinterpretation of it. Prose ingest, FTS5 and `intent search` are design.md deliverables (`:68`, and the WP-03 deliverable list) that a grep of all 62 ACs for `fts|search` matched zero times. Widening the boundary is the safe direction under this file's own change-control note; shrinking it would need the owner.
- **`intent ingest --from-md` deliberately has no WP-03 AT.** It is a WP-03 deliverable whose acceptance lives at AC-10.2 / AC-10.3, where the migrator exercises it for real. Recorded here as a decision so it stops reading like an oversight; the scaffolding still ships in WP-03.
- **AT-03.2 asserts a law, not a file.** No generated view may contain a render-time value and the renderer has no clock (data-model.md, "Generated views: the renderer has no clock"). AC-03.4 forces this: a view that stamps its own render time makes regenerate-and-diff non-empty on every run, so the skew check becomes trained-to-be-ignored. Three v2 instances existed at `f7434f1`, one of them inside the generated-banner pattern the data model ratifies -- so a test scoped to one view would have certified the status quo.
- **AC-03.2 idempotence means idempotent THROUGH the formatter, not merely through the renderer.** Every view v3 generates lands in a repository running a markdown formatter, Intent's own included. A renderer idempotent only against itself still emits a view the formatter rewrites on commit and the renderer un-rewrites on regeneration, so the skew check flags a file nobody touched -- permanently. AT-03.2 therefore renders, runs the project formatter over the output, and asserts unchanged. Found live by ic on `dispatch-table.md` (fixed at `f037649` by aligning the generator to the formatter's column widths; vc verified regeneration is byte-identical), and it pairs with the no-clock law rather than duplicating it: that law removes nondeterminism from INSIDE the renderer, this one removes a second writer DOWNSTREAM of it, and neither guard catches the other's case.

### WP-04

- AT-04.1 `crates/intentsvcs/tests/facade_st_wp.rs` -- covers AC-04.1 -- status: to-write -- includes a mid-write failure injection
- AT-04.2 `crates/intentsvcs/tests/facade_acceptance.rs` -- covers AC-04.2 -- status: to-write
- AT-04.3 `crates/intentsvcs/tests/close_gate_parity.rs` -- covers AC-04.3 -- status: to-write
- AT-04.4 `crates/intentsvcs/tests/error_remedies.rs` -- covers AC-04.4 -- status: to-write
- AT-04.5 `crates/intentsvcs/tests/event_log_envelopes.rs` -- covers AC-04.5 -- status: to-write -- renumbered from AT-02.6 at WP-02 close (bounce ruling 2026-08-14)
- Coverage: complete

### WP-05

- AT-05.1 `crates/intent-cli/tests/dispatch_ssot.rs` -- covers AC-05.1 -- status: to-write
- AT-05.2 `tests/conformance/run_v2_suite.bash` -- covers AC-05.2 -- status: to-write -- core-family subset; shared with AT-00.1
- AT-05.3 (non-test) the register reviewed complete, no unclassified rows -- covers AC-05.3 -- status: n/a
- AT-05.4 (non-test) clap-layer review against the thin-coordinator rule -- covers AC-05.4 -- status: n/a
- Coverage: complete

### WP-06

- AT-06.1 `tests/conformance/run_v2_suite.bash` -- covers AC-06.1 -- status: to-write -- full estate
- AT-06.2 `crates/intentsvcs/tests/doctor_checks.rs` -- covers AC-06.2 -- status: to-write
- AT-06.3 (non-test) register diff history shows land-time recording -- covers AC-06.3 -- status: n/a
- AT-06.4 `crates/intent-cli/tests/search_surface.rs` -- covers AC-06.4 -- status: to-write
- AT-06.5 `crates/intent-cli/tests/schema_command.rs` -- covers AC-06.5 -- status: to-write
- Coverage: complete

WP-06 disposition (vc, 2026-08-14, ADOPTED under hv standing authorisation): AC-06.4 puts one NON-parity command inside a work package titled "CLI parity long tail". That is deliberate -- it is where the remaining CLI surface lands -- but the title now understates the WP by one command, and `intent search` must be recorded in the register as an **addition**, never as a deviation. A deviation is a v2 behaviour we chose not to reproduce; this has no v2 behaviour to deviate from. Flagged to hv in case the WP wants renaming rather than a footnote.

### WP-07

- AT-07.1 `crates/intent-cli/tests/embedded_init.rs` -- covers AC-07.1 -- status: to-write -- offline fresh init
- AT-07.2 `crates/intent-cli/tests/hook_compat.rs` -- covers AC-07.2 -- status: to-write -- byte-compares every shipped hook
- AT-07.3 `crates/intent-cli/tests/skills_sync.rs` -- covers AC-07.3 -- status: to-write
- AT-07.4 `crates/intent-cli/tests/critic_runner.rs` -- covers AC-07.4 -- status: to-write
- AT-07.5 `crates/intent-cli/tests/agents_sync_parity.rs` -- covers AC-07.5 -- status: to-write
- AT-07.6 `crates/intent-cli/tests/view_single_writer.rs` -- covers AC-07.6 -- status: to-write -- renders a view carrying authored `*emphasis*`, runs the formatter, asserts unchanged
- Coverage: complete

WP-07 disposition (vc, 2026-08-14, ADOPTED under hv standing authorisation) -- **the third formatter class, and why the fix is repo-level rather than renderer-level:**

Three distinct classes have now been found, each by a different route:

| Class | What moves the bytes                                                     | Fix                                                     |
| ----- | ------------------------------------------------------------------------ | ------------------------------------------------------- |
| 1     | layout the renderer controls (column widths, blank runs, trailing space) | renderer -- done, `finish()` / `kv()`                   |
| 2     | markup the renderer ADDS around data carrying its own delimiters         | renderer -- never wrap a possibly-markdown value inline |
| 3     | markup the AUTHOR wrote (`*major*` -> `_major_`)                         | **not fixable in the renderer at all**                  |

Class 3 is unfixable by construction: the only renderer-side answers are to rewrite authored prose, which migration.md forbids outright, or to embed a formatter in the binary, which rebuilds the second-writer problem inside our own process. Reproduced independently at prettier 3.9.6, which is the same prettier the pre-commit hook invokes from PATH -- so the measurement is of the real second writer, not a lookalike.

**The defect is two writers over one file, and we had been making writer A imitate writer B rather than removing writer B.** A generated view is tool-owned and has exactly one legitimate writer -- D02 applied to writers instead of to content, which is the whiteboard's single-writer rule for a third time.

Ruling: exclude tool-owned generated views from the formatter. **cc's proposal was right and incomplete, and the incomplete half is the one that matters**: a `.prettierignore` in this repo fixes the dogfood tree and leaves the shipped product broken everywhere else. Intent installs pre-commit hooks into consumer repos, so the skew check would cry wolf in Lamplight, Utilz and Baize permanently while being clean here. Hence AC-07.6 is written as convergence at `init` and migration -- the mechanism that already gives `.gitignore` its `intent/.cache/` entry -- and asserts the property **in a consumer repo as well as this one**.

Rejected alternative: define skew as `format(render(model)) != file`, which closes all three classes everywhere with no convergence needed. It makes Intent's committed artefacts hostage to a JS toolchain version -- a prettier 4.x upgrade would churn every view in every repo -- and that contradicts the self-containment principle used to cut T3 out of WP-13's 3.0.0 gate. Applying a principle in one ruling and abandoning it in the next is worse than either answer alone.

cc's `authored_prose_emphasis_is_the_one_case_the_renderer_cannot_stabilise` stays until AC-07.6 lands, then becomes the stability assertion its own failure message already prescribes. A named gap beats a green that means less than it looks like.

**What AC-07.6 does NOT retire (asked by ic, ruled):** table alignment in generated views. The aligners have two justifications and only one expires. Matching the formatter's exact column widths so regeneration reproduces committed bytes expires the moment the formatter stops writing the file. The house rule -- `in-standards`, all markdown tables column-aligned -- does not, because it is about the quality of what the renderer emits and is indifferent to who else writes the file. The formatter was correcting a real defect, not imposing a preference.

The consequence, which is the part that must be written down before the referee leaves: **once the formatter no longer touches these files, "aligned" needs a definition the renderer owns.** Today it means, in practice, "whatever prettier does" -- and that meaning evaporates with the exclusion. The structural assertions that already exist on both sides (cc's `every_rendered_table_is_in_canonical_padded_form`, ic's equivalent) become the definition rather than a check against an external authority. A renderer whose output nothing else formats must carry its own canonical form, or "aligned" quietly becomes undefined and the next renderer picks its own.

**EXP-01 and the `known_exposures` idea are adopted into the register's working rules** (ic, 2026-08-14): a view can be clean because nothing in its canon happens to trip a class, which is luck rather than protection -- ic's dispatch table carries zero `*emphasis*` spans today and one canon note written tomorrow breaks it. **A file clean by luck and a file clean by construction look identical in a diff, and only one of them stays clean.** A green that cannot say which kind it is has told you less than it appears to, so the exposure is rendered and named with its resolving AC rather than left to be rediscovered as drift.

### WP-08

- AT-08.1 `crates/intentd/tests/daemon_registry.rs` -- covers AC-08.1 -- status: to-write
- AT-08.2 `crates/intentd/tests/dual_path_conformance.rs` -- covers AC-08.2 -- status: to-write
- AT-08.3 `crates/intent-cli/tests/cli_routing.rs` -- covers AC-08.3 -- status: to-write
- AT-08.4 `crates/intentd/tests/daemon_lifecycle.rs` -- covers AC-08.4 -- status: to-write -- shared with AT-00.3
- AT-08.5 `crates/intentd/tests/daemon_watch.rs` -- covers AC-08.5 -- status: to-write
- AT-08.6 `crates/intentd/tests/daemon_subscriptions.rs` -- covers AC-08.6 -- status: to-write
- AT-08.7 `crates/intentd/tests/policy_stamp_healing.rs` -- covers AC-08.7 -- status: to-write
- Coverage: complete

### WP-09

- AT-09.1 `crates/intent-cli/tests/mcp_surface.rs` -- covers AC-09.1 -- status: to-write -- shared with AT-00.4
- AT-09.2 `crates/intent-cli/tests/mcp_graphql_tool.rs` -- covers AC-09.2 -- status: to-write
- AT-09.3 `crates/intent-cli/tests/mcp_bridge_restart.rs` -- covers AC-09.3 -- status: to-write
- AT-09.4 `crates/intent-cli/tests/llm_guide_gen.rs` -- covers AC-09.4 -- status: to-write
- AT-09.5 `crates/intent-cli/tests/mcp_resources.rs` -- covers AC-09.5 -- status: to-write
- Coverage: complete

### WP-10

- AT-10.1 `crates/intentsvcs/tests/migrate_floor.rs` -- covers AC-10.1 -- status: to-write
- AT-10.2 `crates/intentsvcs/tests/migrate_refusal.rs` -- covers AC-10.2 -- status: to-write -- asserts atomicity: nothing written on BLOCKED
- AT-10.3 `crates/intentsvcs/tests/migrate_v2_project.rs` -- covers AC-10.3 -- status: to-write -- shared with AT-00.6
- AT-10.4 `crates/intentsvcs/tests/migrate_hooks_continuity.rs` -- covers AC-10.4 -- status: to-write
- AT-10.5 `crates/intentsvcs/tests/fleet_corpus_ingest.rs` -- covers AC-10.5 -- status: to-write -- shared with AT-00.2
- AT-10.6 (non-test) canary rollback exercised and recorded in impl.md -- covers AC-10.6 -- status: n/a
- Coverage: complete

### WP-11

- AT-11.1 (non-test) release CI run yields artefacts + formula -- covers AC-11.1 -- status: n/a
- AT-11.2 (non-test) notarised artefact verified against the recorded decision -- covers AC-11.2 -- status: n/a
- AT-11.3 `crates/intent-cli/tests/no_intent_home.rs` -- covers AC-11.3 -- status: to-write
- Coverage: complete; the clean-machine brew install lives at ST level (AC-00.5)

### WP-12

- AT-12.1 `tests/cutover_guard.bash` -- covers AC-12.1 -- status: to-write -- greps the repo for shell-intent references post-prune
- AT-12.2 (non-test) docs sweep reviewed -- covers AC-12.2 -- status: n/a
- AT-12.3 (non-test) tag contents carry history + release notes -- covers AC-12.3 -- status: n/a
- AT-12.4 (non-test) release record: tag on both remotes, GitHub release, formula -- covers AC-12.4 -- status: n/a
- Coverage: complete

### WP-13

- AT-13.1 `tests/retirement_guard.bash` -- covers AC-13.1 -- status: to-write -- greps the whole repo for treeindex and in-handoff references after the prune
- AT-13.2 `crates/intentsvcs/tests/index_scope.rs` -- covers AC-13.2 -- status: to-write -- includes a gitignored file that must never surface
- AT-13.3 `crates/intentsvcs/tests/search_lexical.rs` -- covers AC-13.3 -- status: to-write
- AT-13.4 `crates/intentsvcs/tests/search_structural.rs` -- covers AC-13.4 -- status: to-write -- asserts no grammar loads for an undeclared language
- AT-13.5 `crates/intentsvcs/tests/index_staleness.rs` -- covers AC-13.5 -- status: to-write -- a same-size same-mtime edit in each corpus, with opposite expectations
- AT-13.6 `crates/intentsvcs/tests/search_degradation.rs` -- covers AC-13.6 -- status: to-write
- AT-13.7 `crates/intentd/tests/background_index.rs` -- covers AC-13.7 -- status: to-write -- daemon and daemonless results compared, the dual-path pattern
- AT-13.8 `crates/intent-cli/tests/mcp_search_tool.rs` -- covers AC-13.8 -- status: to-write
- AT-13.9 (non-test) design.md carries the T3/T4 staging with S1-S5 shown sufficient -- covers AC-13.9 -- status: n/a
- Coverage: complete

WP-13 dispositions (vc, 2026-08-14, ADOPTED under hv standing authorisation):

- **AT-13.5 is the one to write first and the one most likely to be written vacuously.** It asserts a deliberate ASYMMETRY -- the same edit must be caught in `intent/**` and may be missed in the source corpus -- so a test that merely proves "edits are detected" passes while proving nothing. Both arms come from the same fixture, and the source-corpus arm must fail if someone quietly makes it hash-always for tidiness.
- **`fileindex` is deliberately NOT in this WP.** It shares a naming convention with treeindex and nothing else -- it manages file lists with checkbox states, not an index. It stays in WP-06's port list until hv rules on it directly. Bundling it because the names rhyme is the error class this thread exists to remove.
- **AC-13.1 reduces WP-06.** Retiring treeindex removes 762 lines of bash from WP-06's port list and collapses two `/in-essentials` rules into one. Sequence T0 before WP-06 ports commands that are about to be deleted.
