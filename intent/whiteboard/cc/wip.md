---
node: cc
name: Control Claude
role: control
session_id: 4ec8fcd8-3834-43f3-af24-8cabe1eb0958
heartbeat_at: 2026-07-24T10:42Z
status: paused
focus: "v2.17.3 SHIPPED (2026-07-24): issue 0004 fixed + closed + released. Tag 2828f89, wrap 5793d7d, both remotes + GitHub release. Full suite green. Globalfold done, day closed."
claims: []
---

# Control Claude (cc)

## DOING

_(day closed 2026-07-24 -- v2.17.3 shipped: issue 0004 fixed + closed + released; globalfold done. Session detail in `intent/done.md` + `intent/issues/CLOSED/0004/`.)_

## TODO

- **hv ruling owed -- issue 0004 item 4 (`ac status` exit code).** hv asked for uniform non-zero on a BLOCKED `ac status`; premise does not reproduce (exits 0 on both BLOCKED shapes; `intent_acceptance_cli.bats:111` asserts it). `status` = reporter (stdout), `gate` = gate (`$?`). Own issue if hv wants it changed.
- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- DEFERRED (needs hv ruling): AT-name traceability -- `acceptance.md` AT ids grep-able to bats `@test` names.

## Watch-outs

- `bin/release` does tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it. (It is interactive; if a `bin/release` run is interrupted mid-cut, finish the push/gh/wrap by hand -- it is idempotent on the tag.)
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-24) Issue 0004 fixed under the issue (no ST), per the standing hv ruling from 0002/0003. Resolution design: target resolution is a distinct FAILABLE step ahead of evaluation, shared by the whole ac/at family; the gate announces every verdict including PASS, because silence-on-success is what made the vacuous passes invisible. hv's fix item 4 (non-zero exit on a BLOCKED `ac status`) deliberately NOT actioned -- premise does not reproduce; `ac status` is the reporter (verdict on stdout, exit 0), `ac gate` is the gate (verdict in `$?`). Left for an hv ruling as its own issue.
- (2026-07-13) v2.17.2 SHIPPED: issues 0002 + 0003 fixed + closed; 0003 gate design = defer to `intent critic` exit code (prose no-op), not a `--languages` skip. Detail: `done.md` + `.history/20260713/`.
- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed; fleet normalised. Detail: `.history/20260710/`.
