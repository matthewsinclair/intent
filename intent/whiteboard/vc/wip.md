---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T13:05Z
status: active
focus: "ST0056 steward + verifier (hv ruling: cc builds, ic runs parity, vc ensures). WP-01 Done; WP-02 WIP, foundation handed to cc at 5e4b766."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **ST0056 stewardship.** hv ruling (2026-08-14): the coding reins are cc's (build lane, WP-02 onward) and ic's (parity deep pass); vc keeps the thread claim as steward -- contract keeper, verifier at WP closes, hv interface. Handover briefs sent: `cc/inbox.vc.md` (build lane + WP-02 state + conventions), `ic/inbox.vc.md` (parity work order). WP-01 Done (gate 4/4); WP-02 WIP with its foundation landed and mutation-proven at `5e4b766`.

## TODO

- **Review WP-02 at cc's close claim** (fire-on-claim per the standing role): the SDL face, AC-02.1's first green CI run, and the AC-02.6 red-until-WP-04 vs descope-to-WP-04 call -- contract change is mine to record either way.
- **Review ic's parity deliverables** into WP-05: the flag-level inventory + the first-pass register; anything fitting no deviation class is a contract gap for me.
- **Measure cc's consumer sweep** (v2 lane, protocol agreed): cc counts post-sweep as stop condition, vc re-counts as the record against the Lamplight baseline (1639 rows at `15dbccc92`); post-sweep revisions become WP-10's corpus manifest.

## Watch-outs

- **Never mutate `bin/**` or `tests/**` in place** -- `~/.local/bin/intent` symlinks into this repo; sacrificial worktrees only. `crates/**` has no symlink hazard, but suites run concurrently -- coordinate.
- **The machine-global gitignore ignores `*.sql`** -- it silently dropped `schema/ddl.sql` from the scaffold commit (caught because the commit's file list was read, not assumed). Committed faces need their `!` exception; check any new non-json artefact against `git check-ignore -v`.
- **This shell is zsh; MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal.**
- **Commit by explicit pathspec, never `-A`** -- cc and ic run concurrently.
- Release-window mechanics live in `intent/restart.md`'s checklist.

## Decisions

- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc keeps the ST0056 claim as steward (contract, verification, hv interface) and does not build. The one exception already landed -- the WP-02 foundation vc built before the ruling -- was verified by mutation before handover and is cc's to extend.
- (2026-08-14) **ST0056 is claimed by vc on direct hv assignment** (stewardship form, per the ruling above).
- (2026-08-14) **Stamp heartbeats from `date -u`, never a local clock reading suffixed `Z`.**
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs (ST0056 stewardship excepted).
