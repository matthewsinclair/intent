---
node: vc
name: Validation Claude
role: validation
session_id: 1f98ea43-d71a-47a3-aba8-9597183b2e67
heartbeat_at: 2026-08-14T02:40Z
status: paused
focus: "HOLDING: suite green, globalfold done (2c928c8), v2.19.0 awaits the cut first thing next session; vc fires on the cut + the estate sweeps"
claims: []
---

# Validation Claude (vc)

## DOING

- (holding -- suite green at HEAD per hv, globalfold committed (`2c928c8`), F6 restore landed (`ee44f63`). The release cut is the next project action, hv's hand, first thing next session. Day detail: `.history/20260814/` + `intent/done.md`.)

## TODO

- Fire on the suite verdict: commits `8aba5ab` (warning voice) + `ba52339` (0019 index fix) postdate any suite run started before them -- they touch `intent_st`, `intent_helpers`, `intent_migrations` + two test files. If hv's run predates them, the delta needs a re-run or targeted files.
- Fire on the cut: `bin/release` stamps all five sidecars (still 2.18.0 at HEAD by design); CHANGELOG heading `## [2.19.0] - in progress` for the script to date. hv cuts; NEVER `--no-confirm`.
- Post-cut estate sweeps: Lamplight first (`intent upgrade` now converges AT grammar via `at lint --fix` -- 314 rows, expect BLOCKED-until-swept with residue named never guessed -- plus AGENTS.md, settings hooks, gitignore entries, and a printed-never-run `git rm` for any tracked treeindex). Utilz / Baize follow.

## Watch-outs

- Trusted on cc's record, not independently re-run: the poisoned-consumer upgrade fixtures and the adapted gitignore-idempotence fixture. hv's external suite is the final word.
- `intent_claude_prime:212` still prints its truncation notice to STDOUT with a capital prefix -- deliberately left (changing its voice means changing its stream); surface if prime output pollution ever bites.

## Decisions

- (2026-08-13) vc triage rulings for the 0009-0018 corpus: all executed and recorded in the issues' Resolutions; archived detail in `.history/20260814/wip.md`.
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs.
