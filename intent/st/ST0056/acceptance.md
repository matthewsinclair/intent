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

- AC-01.1 (non-test) design.md carries the ratified architecture, truth model, decision log D01-D17, alternatives, and stack shortlist -- evidence: hv review at the pre-kickoff check-in -- satisfied: no
- AC-01.2 (non-test) The full-ladder acceptance contract (WP-02..12 AC/AT groups) is authored and ratified -- evidence: this file at WP-01 close -- satisfied: no
- AC-01.3 (non-test) The parity contract exists: v2 command-surface inventory + the keep/retire/deviate register format -- evidence: named WP-01 doc -- satisfied: no
- AC-01.4 (non-test) The design open questions are closed (one binary vs two, launchd label, 3.0.0 subscription extent, `.cache` layout) -- evidence: decision-log additions in design.md -- satisfied: no

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
