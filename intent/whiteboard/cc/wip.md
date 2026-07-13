---
node: cc
name: Control Claude
role: control
session_id: cf2d4e1b-70cd-433b-aa56-6de4e94b997d
heartbeat_at: 2026-07-13T21:32Z
status: paused
focus: "v2.17.2 SHIPPED (2026-07-13): issues 0002 + 0003 fixed + closed + released. Tag 22c409e, wrap e525f04, both remotes + GitHub release. Globalfold done. Awaiting matts's next."
claims: []
---

# Control Claude (cc)

## DOING

_(day closed 2026-07-13 -- v2.17.2 shipped: issues 0002 + 0003 fixed + closed + released. Session detail archived to `.history/20260713/`.)_

## TODO

- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- DEFERRED (needs hv ruling): AT-name traceability -- `acceptance.md` AT ids grep-able to bats `@test` names.

## Watch-outs

- `bin/release` does tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it. (It is interactive; if a `bin/release` run is interrupted mid-cut, finish the push/gh/wrap by hand -- it is idempotent on the tag.)
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-13) v2.17.2 SHIPPED: issues 0002 + 0003 fixed + closed; 0003 gate design = defer to `intent critic` exit code (prose no-op), not a `--languages` skip. Detail: `done.md` + `.history/20260713/`.
- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed; fleet normalised. Detail: `.history/20260710/`.
