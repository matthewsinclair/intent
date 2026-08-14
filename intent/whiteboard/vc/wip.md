---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T15:28Z
status: paused
focus: "EOD: WP-02 closed through the gate (5/5) on cc's claim; five bounce rulings executed; CI green twice on 736033d; boards folded. WP-03 review armed on cc's claim."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- (EOD fold: session wrapped; the day's detail is in `.history/20260814/wip.md`, PM-3)

## TODO

- **WP-03 review fires on cc's claim** (ingest, views, sync -- the carry policy and its marked-legacy model consequence are new inputs).
- **Spec the marked-legacy AT form in data-model.md before WP-08** (the named consequence in migration.md; raw v2 reference carried verbatim beside the parsed fields).
- **Review ic's regenerated register** (`393a8e1`) and the split-files per-test pass when it lands (`corrected` now ratified).
- ic's charter + roster asks: surfaced at the bounce, not ruled; still open with hv.

## Watch-outs

- **Whiteboard stamps carry a trailing `Z`, always.** The pre-commit clock guard (`ddac6ba`; `Re:`-anchor fix `98ce764`) refuses a commit ADDING an unmarked stamp (check B), a stamp postdating the commit (check A), or an inbox going backwards (check C). Stamp from `date -u`, never rounded up; pre-existing unmarked entries are not fired on and are never rewritten.
- **Never mutate `bin/**` or `tests/**` in place** -- `~/.local/bin/intent` symlinks into this repo; sacrificial worktrees only. `crates/**` has no symlink hazard, but suites run concurrently.
- **The machine-global gitignore ignores `*.sql`** -- committed faces need their `!` exception; `git check-ignore -v` any new non-json artefact.
- **This shell is zsh; MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal.**
- **`git add <paths>` + bare `git commit` commits the WHOLE INDEX -- a peer's staged work rides along.** It happened: cc's staged `bin/release` -> devbin rename rode vc's `072d277`. The protocol's own spelling is `git commit --only <paths>`; use it verbatim, every time, and never `-A`.
- Release-window mechanics live in `intent/restart.md`'s checklist.

## Decisions

- (2026-08-14) **hv bounce rulings**: (1) `corrected` parity class RATIFIED; (2) migration carry policy RATIFIED -- lossless-by-carrying for CLOSED threads, BLOCKED-until-clean for live, neither ever lossy; (3) organize (both faces) planned VESTIGIAL by construction, retire confirmed at surface cut; (4) push authorized "as soon as it makes sense"; (5) v2 maintenance DEFAULT-DEFER, show-stoppers only.
- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc keeps the ST0056 claim as steward (contract, verification at WP closes, hv interface) and does not build. The WP-02 foundation vc built pre-ruling was mutation-proven before handover and is cc's to extend.
- (2026-08-14) **Contract changes route through vc even when proposed by builders** -- ic's `corrected` class and cc's scope-honouring property were both drafted into parity.md by vc, marked for hv ratification; cc's law finding was resolved by rewording the law (authored-once is about where names are authored, not every transport spelling them identically).
- (2026-08-14) **Stamp heartbeats from `date -u`, never a local clock reading suffixed `Z`.**
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs (ST0056 stewardship excepted).
