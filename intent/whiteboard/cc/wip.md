---
node: cc
name: Control Claude
role: control
session_id: cf2d4e1b-70cd-433b-aa56-6de4e94b997d
heartbeat_at: 2026-07-13T20:30Z
status: active
focus: "v2.17.2 SHIPPED (2026-07-13): issues 0002 + 0003 fixed + closed + released. Tag 22c409e, wrap e525f04, both remotes + GitHub release. Globalfold done. Awaiting matts's next."
claims: []
---

# Control Claude (cc)

## DOING

- **v2.17.2 SHIPPED -- issues 0002 + 0003 fixed + closed + released.** No ST per hv. Fix commit `9d14ad3`, release `22c409e` (tag `v2.17.2`), wrap `e525f04`; both remotes + GitHub release; globalfold done (done.md / wip.md / restart.md / .claude/restart.md). Affected bats suites green; shell critic clean on the changed files.
  - 0002 (todo `[?]`): `canonical_status` relocated `intent_st` -> `intent_helpers` (the shared lib both source); `intent_todo` `status_box` now routes through it. Guard: intent_todo.bats.
  - 0003 (critic rejects author/content): one language registry in `critic_runner.sh`; `intent critic` no-ops prose at exit 0 + `--languages`; the gate is UNCHANGED (defers to exit code). Prose-only-on-content verified via `applies_to` (matts follow-up). Guards: intent_critic / pre_commit_hook / critic_runner_applies_to.bats.

## TODO

- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- _(v2.17.2 shipped -- release + wrap + globalfold all done. No release in flight.)_
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- DEFERRED (needs hv ruling): AT-name traceability -- `acceptance.md` AT ids grep-able to bats `@test` names.

## Watch-outs

- `bin/release` does tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it. (It is interactive; if a `bin/release` run is interrupted mid-cut, finish the push/gh/wrap by hand -- it is idempotent on the tag.)
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-13) Issue 0003 gate design: the pre-commit gate defers to `intent critic`'s exit code (prose -> exit-0 no-op), rather than querying `intent critic --languages` to skip prose itself. The query approach was built and reverted -- it made the gate depend on the query returning a clean list (a broken/old CLI could then silently skip a REAL code critic) and broke the stub-based `critic_dispatch.bats` model. One registry lives in `critic_runner.sh`; the gate stays language-agnostic.
- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed (gate 23/23); fleet normalised. Older decisions archived to `.history/20260710/`.
