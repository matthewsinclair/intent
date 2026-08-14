---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T11:55Z
status: paused
focus: "ST0056 (Intent v3.0.0): design canon landed, 12-WP ladder cut, WP-01 WIP. Released for compact; resumes at hv's pre-kickoff check-in."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **ST0056 / WP-01 -- design canon** (hv-assigned). design.md (architecture + decision log D01-D17), acceptance contract (ST gate + WP-01 group, lint-clean), tasks.md ladder, 12 WP info files -- all landed 2026-08-14 pm. Holding for hv's pre-kickoff check-in, then WP-01 completes: data-model spec + JSON Schema draft, migration spec, parity contract (command-surface inventory + keep/retire/deviate register), WP-02..12 AC/AT groups, four open questions closed (bin count -- lean two; launchd label; subscription extent; `.cache` layout).

## TODO

- **Measure cc's consumer sweep** (protocol agreed with cc): cc counts immediately post-sweep as its stop condition; vc re-runs the AT-row-scoped counts independently as the record, against `intent/analysis/20260814-lamplight-at-sweep-baseline.md` (1639 rows at `15dbccc92`). Same method for Utilz/Baize once cc takes their baselines. **Post-sweep revisions per project must be recorded -- they are WP-10's fleet-corpus fixture.**
- Carry into WP-01: the IC-delegable parity raw material (v2 command-surface inventory; BATS `INTENT_BIN` retarget + classification) -- offered to hv for an IC fire-up.

## Watch-outs

- **Never mutate `bin/**` in place** -- `~/.local/bin/intent` symlinks into this repo; sacrificial worktrees only. Same while any suite runs.
- **This shell is zsh; MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal.** Measure stream separation by redirecting to a file and counting bytes.
- **Commit by explicit pathspec, never `-A`** -- cc runs concurrently and its board is frequently dirty in the tree.
- Release-window mechanics live in `intent/restart.md`'s checklist (not re-carried here).

## Decisions

- (2026-08-14) **ST0056 is claimed by vc on direct hv assignment**, overriding the standing vc-claims-no-STs default for this thread only. The validation role continues for cc's lane (sweeps) unchanged.
- (2026-08-14) **Stamp heartbeats from `date -u`, never a local clock reading suffixed `Z`.**
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs (ST0056 excepted above).
