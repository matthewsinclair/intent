---
archived: 2026-07-13
node: cc
---

# Control Claude (cc) -- archived 2026-07-13

## DOING (shipped)

- **v2.17.2 SHIPPED -- issues 0002 + 0003 fixed + closed + released.** No ST per hv. Fix commit `9d14ad3`, release `22c409e` (tag `v2.17.2`), wrap `e525f04`; both remotes + GitHub release; globalfold done (done.md / wip.md / restart.md / .claude/restart.md). Affected bats suites green; shell critic clean on the changed files.
  - 0002 (todo `[?]`): `canonical_status` relocated `intent_st` -> `intent_helpers` (the shared lib both source); `intent_todo` `status_box` now routes through it. Guard: intent_todo.bats.
  - 0003 (critic rejects author/content): one language registry in `critic_runner.sh`; `intent critic` no-ops prose at exit 0 + `--languages`; the gate is UNCHANGED (defers to exit code). Prose-only-on-content verified via `applies_to` (matts follow-up). Guards: intent_critic / pre_commit_hook / critic_runner_applies_to.bats.

## Decisions (archived)

- (2026-07-13) Issue 0003 gate design: the pre-commit gate defers to `intent critic`'s exit code (prose -> exit-0 no-op), rather than querying `intent critic --languages` to skip prose itself. The query approach was built and reverted -- it made the gate depend on the query returning a clean list (a broken/old CLI could then silently skip a REAL code critic) and broke the stub-based `critic_dispatch.bats` model. One registry lives in `critic_runner.sh`; the gate stays language-agnostic. [Also recorded in done.md 2026-07-13 + the CLOSED issue 0003 Resolution.]
