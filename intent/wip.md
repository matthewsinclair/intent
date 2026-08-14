---
verblock: "14 Aug 2026:v1.08: vc - ST0056 (Intent v3.0.0) begun: architecture ratified, 12-WP ladder cut, WP-01 WIP"
intent_version: 2.19.0
---

# Work In Progress

## Current State

**ST0056 -- Intent v3.0.0 -- is underway (2026-08-14, same day v2.19.0 shipped).** The architecture was ratified with hv in a rubber-duck session and is recorded in `intent/st/ST0056/design.md` (decision log D01-D17): a reified schema-as-truth data model (committed JSON canon + rebuildable per-project SQLite + markdown as generated views and authored prose), the `intentsvcs` layering contract (sole owner of DB and file canon; CLI dual-mode in-process/GraphQL; one intentd per machine), strict ingest with refuse-lossy discipline, MCP as the primary agent write surface, migration floored at v2.19.0 (two-hop), Homebrew as a core deliverable, and intentd IN the 3.0.0 gate. Prior art trawled: Lamplight `native/cli` and Conflab `native/daemon` (the conflabd stack maps nearly 1:1 onto intentd). The 12-WP ladder is cut (`intent wp list ST0056`); WP-01 (design canon) is WIP; the acceptance contract carries the ST-level v3.0.0 gate (AC-00.1..8) with WP-02..12 groups landing at WP-01 close. vc drives ST0056 on direct hv assignment.

**v2.19.0 SHIPPED earlier today -- tag `071c612`, both remotes + GitHub release.** Fifteen issues (0009-0023); narrative `intent/history/v2.19.0.md`; per-issue record `intent/issues/CLOSED/0009..0023`.

## Next Up

1. **hv pre-kickoff check-in on ST0056** -- review design.md + the ladder; then WP-01 completes (data-model spec, migration spec, parity contract, full-ladder acceptance contract, four open questions closed as D18+).
2. **Consumer sweeps -- one `intent upgrade` per project (cc's lane).** Lamplight first (baseline: 1639 AT rows at `15dbccc92`, ~70% expected residue -- that is the fix working), then Utilz, Baize (baseline-first is part of each sweep). cc runs the post-sweep counts as its stop condition; vc re-runs them independently as the record. **Now doubly load-bearing: the sweeps are v3 migration prep** -- WP-10's fleet corpus is the post-sweep trees at named revisions.
3. **Parity raw material (IC-friendly, design-neutral, start any time):** the v2 command-surface inventory (every command/flag/output/exit code from `bin/` + help + tests) and the BATS harness retarget (`INTENT_BIN` override + per-test keep/retire/deviate classification). Both feed WP-01's parity contract and WP-05.
4. **`credo_checks/` cleanup in the Elixir fleet** -- issues filed: Baize 0001, Lamplight 0003, Conflab 0008; Laksa + Prolix clean. Each project fixes its own (hv running these).
5. **Push the fleet issue-normalisation commits**: Utilz `0171297`, Lamplight `7058fd3a8`.
6. **hv-ruling queue (v2, fix-under-issue):** plugin bins writing errors to stdout (0023's named-and-left half) + `intent_claude_prime:212`; the dead `CREATED` block in `intent_st`'s in-progress arm (anchor: the `# Extract created date for index update` comment); 0004 item 4 close ruling; a `javascript` pack for 0009's Node exception; pruning consumers' inert `.claude/scripts/` copies. The whiteboard pickup-time-inbox limitation is now designed-for in ST0056's 3.2 agent-bus ST rather than a standalone ruling.

## Recent

- **2026-08-14 (pm)**: ST0056 begun -- v3.0.0 architecture ratified with hv, Conflab + Lamplight trawled, 12 WPs cut, WP-01 WIP.
- **2026-08-14 (am)**: v2.19.0 SHIPPED (tag `071c612`). Fifteen issues, 0009-0023; release docs written pre-cut for the first time.
- **2026-07-30**: v2.18.0 + v2.17.4 shipped. Earlier: `intent/history/202607-done.md`.

## Parked

- 3.x steel threads (post-v3.0.0, each on its own): TUI dashboard; the agent bus (whiteboard restructure + hv oversight gates); Laksa web page; macOS menubar app; `intent_ex` hex client; sqlite-vec semantic search.
