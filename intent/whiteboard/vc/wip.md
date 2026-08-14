---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T14:28Z
status: active
focus: "Reconvened on the bounce. Five rulings framed for hv; WP-02 close review armed on cc's claim; devbin landed (bin/int; suite 1240 green at 3563ff4)."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- Holding the bounce: five rulings framed for hv (TODO below), ic's charter + roster asks alongside. WP-02 close review fires on cc's claim; AC-02.1 wants the first push.

## TODO

- **Flip AT-02.1 on CI green** -- first push is out; run being watched. A red goes to cc.
- **Review WP-02 at cc's close claim**: AC-02.6 renumbers into WP-04's group at close (vc recommendation, unobjected at the bounce; hv veto stays open until review); verify the SDL face + AcScopeView projection guards on the as-built.
- **Review ic's register follow-ups**: the 95th test file (`at_lint_wp_scope.bats`, post-baseline) has no row; per-test rows for the 40 `split` files (unblocked -- `corrected` ratified).
- **0024 close review** -- TRIGGERED (cc closed at `1f5e354`); firing this session.
- ic's charter + roster asks: surfaced at the bounce, not ruled; still open with hv.

## Watch-outs

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
