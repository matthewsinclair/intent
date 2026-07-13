---
node: cc
name: Control Claude
role: control
session_id: cf2d4e1b-70cd-433b-aa56-6de4e94b997d
heartbeat_at: 2026-07-13T20:30Z
status: active
focus: "Fixing issues 0002 (intent todo [?] on non-canonical status -> route through canonical_status) + 0003 (intent critic rejects declared author/content -> gate skips non-code langs + one shared registry). No ST per hv; tracked under each issue."
claims: []
---

# Control Claude (cc)

## DOING

- **Issues 0002 + 0003 -- fixed in the working tree, pending matts verify + commit (issues left OPEN, uncommitted).** No ST per hv. 246 pass / 0 fail across 14 affected bats suites; mechanical shell critic clean on all 6 changed shell files. Resolutions written into each issue file.
  - 0002 (todo `[?]`): `canonical_status` relocated `intent_st` -> `intent_helpers` (the shared lib both source); `intent_todo` `status_box` now routes through it. Guard: intent_todo.bats.
  - 0003 (critic rejects author/content): one language registry in `critic_runner.sh` (`critic_code_languages`/`critic_prose_languages`); `intent critic` no-ops prose at exit 0 + `intent critic --languages`; the gate is UNCHANGED (defers to exit code). Prose-only-on-content verified via `applies_to` (matts follow-up). Guards: intent_critic / pre_commit_hook / critic_runner_applies_to.bats.

## TODO

- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- **Intent issue 0002 (OPEN):** `intent todo` `[?]` on a non-canonical status -- route todo's status read through `canonical_status`. `intent/issues/OPEN/0002/`.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- DEFERRED (needs hv ruling): AT-name traceability -- `acceptance.md` AT ids grep-able to bats `@test` names.

## Watch-outs

- `bin/release` does tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it. (It is interactive; if a `bin/release` run is interrupted mid-cut, finish the push/gh/wrap by hand -- it is idempotent on the tag.)
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-13) Issue 0003 gate design: the pre-commit gate defers to `intent critic`'s exit code (prose -> exit-0 no-op), rather than querying `intent critic --languages` to skip prose itself. The query approach was built and reverted -- it made the gate depend on the query returning a clean list (a broken/old CLI could then silently skip a REAL code critic) and broke the stub-based `critic_dispatch.bats` model. One registry lives in `critic_runner.sh`; the gate stays language-agnostic.
- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed (gate 23/23); fleet normalised. Older decisions archived to `.history/20260710/`.
