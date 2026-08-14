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

- AC-00.1 The v3 binary passes the narrowed BATS conformance contract (stdout, exit codes, behaviour), with every file-layout divergence recorded in the ratified keep/retire/deviate register
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

- AC-02.1 (non-test) The cargo workspace builds with fmt + clippy gates on macOS and Linux CI -- evidence: first green CI run -- satisfied: no
- AC-02.2 The three schema faces (JSON Schema, DDL, SDL) are generated, committed, and CI fails on drift between types and committed faces
- AC-02.3 Deleting the DB and rebuilding from committed canon yields identical queryable content (the D01 disposability invariant)
- AC-02.4 Serialise/deserialise laws hold under proptest, and an unknown field in canonical JSON is refused by name, never dropped
- AC-02.5 `rusqlite` appears in exactly one Cargo.toml and CI asserts the dependency graph
- AC-02.6 Every mutation path writes an event-log envelope carrying principal + project_id (the D15 seams exist end to end)

### WP-03 -- Ingest, views and sync (status: Not Started)

- AC-03.1 Strict ingest refuses schema-invalid canon with file + finding named, exit non-zero, nothing partially loaded
- AC-03.2 View rendering is deterministic and idempotent: same model renders the same bytes, twice, on both platforms
- AC-03.3 The stat-scan detects external edits by content hash, including a same-size same-mtime rewrite
- AC-03.4 The skew check catches a hand-edited generated view and names the file (never silently outvotes it)
- AC-03.5 A conflict-markered artefact enters the named unparsed state; commands needing it refuse with the finding; v2's silent grep-through is unconstructible

### WP-04 -- intentsvcs facade, core families (status: Not Started)

- AC-04.1 st/wp lifecycle verbs (new, start, done, cancel, list, show) run through the facade with canon + views + DB written transactionally -- a failure mid-write leaves no torn state
- AC-04.2 ac/at operations implement the four AC states with computed satisfaction for test-backed ACs (never stored) and inline evidence for non-test
- AC-04.3 The close-gate reads the model and reproduces v2 gate verdicts across the corpus contracts
- AC-04.4 Every facade error is typed and renders a remedy with its full cause chain (no same-text-for-different-causes collapses)

### WP-05 -- CLI in-process + conformance harness (status: Not Started)

- AC-05.1 The dispatch table is the SSOT: the clap surface and help text are generated from it, asserted by test
- AC-05.2 Core families are green under the v2 BATS estate via `INTENT_BIN` with voice and exit codes byte-compatible
- AC-05.3 (non-test) Every v2 test file is classified in the keep/retire/deviate register; no unclassified rows -- evidence: parity register at WP close -- satisfied: no
- AC-05.4 (non-test) The clap layer holds no business logic (parse -> facade -> render only) -- evidence: review + the AC-02.5 dep guard -- satisfied: no

### WP-06 -- CLI parity long tail (status: Not Started)

- AC-06.1 The full command surface is ported and the conformance suite is green minus register-recorded deviations only
- AC-06.2 doctor is rebuilt as model/DB integrity queries + file checks including the skew check and unparsed-state reporting
- AC-06.3 (non-test) The register is complete: every deviation was recorded at land time, none discovered after -- evidence: register diff history -- satisfied: no

### WP-07 -- Canon and claude subsystem (status: Not Started)

- AC-07.1 A fresh `intent init` works offline from the binary alone (embedded canon); the 0022 broken-install class is unconstructible
- AC-07.2 `intent claude hook <name>` output is byte-compatible with v2 for every shipped hook; consumer settings.json untouched
- AC-07.3 Skills install/sync/uninstall reproduce v2 SHA256-manifest behaviour
- AC-07.4 The critic headless runner reads embedded rules and preserves the ST0039 strict-proxy contract
- AC-07.5 `intent agents sync` output matches v2 for a corpus project, modulo ratified deviations

### WP-08 -- intentd (status: Not Started)

- AC-08.1 The registry serves N projects with per-connection binding; a moved or deleted root surfaces in doctor, never as a crash
- AC-08.2 The dual-path conformance suite is green across the verb surface (in-process vs daemon, identical results)
- AC-08.3 The CLI routing rule holds: socket present routes to the daemon, absent runs in-process, and two sync engines never run concurrently
- AC-08.4 Lifecycle works end to end: launchd install/uninstall, PID discipline with observable cleanup, logs in the named location
- AC-08.5 Debounced gitignore-aware watching drives ingest on external edits
- AC-08.6 `projectChanged` and `fileChanged` subscriptions deliver over the socket (D20 -- and nothing more in 3.0.0)

### WP-09 -- MCP (status: Not Started)

- AC-09.1 The typed tool tier is generated from the dispatch table with schemars-derived parameter schemas
- AC-09.2 The `intent_graphql` escape hatch reaches the full API surface
- AC-09.3 Bridge mode survives a daemon restart mid-session via per-request target resolution
- AC-09.4 `intent llm` renders the agent guide from the dispatch table; no hand-maintained command list exists

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
- AT-02.6 `crates/intentsvcs/tests/event_log_envelopes.rs` -- covers AC-02.6 -- status: to-write
- Coverage: complete; AC-02.1 is non-test with evidence on its line

### WP-03

- AT-03.1 `crates/intentsvcs/tests/ingest_refusal.rs` -- covers AC-03.1 -- status: to-write
- AT-03.2 `crates/intentsvcs/tests/view_determinism.rs` -- covers AC-03.2 -- status: to-write
- AT-03.3 `crates/intentsvcs/tests/sync_scan.rs` -- covers AC-03.3 -- status: to-write -- includes the same-size same-mtime rewrite case
- AT-03.4 `crates/intentsvcs/tests/view_skew_check.rs` -- covers AC-03.4 -- status: to-write
- AT-03.5 `crates/intentsvcs/tests/unparsed_state.rs` -- covers AC-03.5 -- status: to-write
- Coverage: complete

### WP-04

- AT-04.1 `crates/intentsvcs/tests/facade_st_wp.rs` -- covers AC-04.1 -- status: to-write -- includes a mid-write failure injection
- AT-04.2 `crates/intentsvcs/tests/facade_acceptance.rs` -- covers AC-04.2 -- status: to-write
- AT-04.3 `crates/intentsvcs/tests/close_gate_parity.rs` -- covers AC-04.3 -- status: to-write
- AT-04.4 `crates/intentsvcs/tests/error_remedies.rs` -- covers AC-04.4 -- status: to-write
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
- Coverage: complete

### WP-07

- AT-07.1 `crates/intent-cli/tests/embedded_init.rs` -- covers AC-07.1 -- status: to-write -- offline fresh init
- AT-07.2 `crates/intent-cli/tests/hook_compat.rs` -- covers AC-07.2 -- status: to-write -- byte-compares every shipped hook
- AT-07.3 `crates/intent-cli/tests/skills_sync.rs` -- covers AC-07.3 -- status: to-write
- AT-07.4 `crates/intent-cli/tests/critic_runner.rs` -- covers AC-07.4 -- status: to-write
- AT-07.5 `crates/intent-cli/tests/agents_sync_parity.rs` -- covers AC-07.5 -- status: to-write
- Coverage: complete

### WP-08

- AT-08.1 `crates/intentd/tests/daemon_registry.rs` -- covers AC-08.1 -- status: to-write
- AT-08.2 `crates/intentd/tests/dual_path_conformance.rs` -- covers AC-08.2 -- status: to-write
- AT-08.3 `crates/intent-cli/tests/cli_routing.rs` -- covers AC-08.3 -- status: to-write
- AT-08.4 `crates/intentd/tests/daemon_lifecycle.rs` -- covers AC-08.4 -- status: to-write -- shared with AT-00.3
- AT-08.5 `crates/intentd/tests/daemon_watch.rs` -- covers AC-08.5 -- status: to-write
- AT-08.6 `crates/intentd/tests/daemon_subscriptions.rs` -- covers AC-08.6 -- status: to-write
- Coverage: complete

### WP-09

- AT-09.1 `crates/intent-cli/tests/mcp_surface.rs` -- covers AC-09.1 -- status: to-write -- shared with AT-00.4
- AT-09.2 `crates/intent-cli/tests/mcp_graphql_tool.rs` -- covers AC-09.2 -- status: to-write
- AT-09.3 `crates/intent-cli/tests/mcp_bridge_restart.rs` -- covers AC-09.3 -- status: to-write
- AT-09.4 `crates/intent-cli/tests/llm_guide_gen.rs` -- covers AC-09.4 -- status: to-write
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
