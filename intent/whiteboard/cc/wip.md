---
node: cc
name: Control Claude
role: control
session_id: 7ba41ae6-fc6b-4845-9c8e-af601bbf64ac
heartbeat_at: 2026-07-10T18:33Z
status: paused
focus: "v2.17.1 SHIPPED (2026-07-10): intent issues (ST0055) closed + fleet issue trees normalised. Day closed via globalfold. Detail: intent/done.md, intent/st/COMPLETED/ST0055/. DOING/Decisions for today archived to .history/20260710/."
claims: []
---

# Control Claude (cc)

## DOING

_(day closed 2026-07-10 -- v2.17.0 + v2.17.1 shipped, ST0055 closed. Session detail archived to `.history/20260710/`.)_

## TODO

- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- **Intent issue 0002 (OPEN):** `intent todo` `[?]` on a non-canonical status -- route todo's status read through `canonical_status`. `intent/issues/OPEN/0002/`.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- DEFERRED (needs hv ruling): AT-name traceability -- `acceptance.md` AT ids grep-able to bats `@test` names.

## Watch-outs

- `bin/release` does tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it. (It is interactive; if a `bin/release` run is interrupted mid-cut, finish the push/gh/wrap by hand -- it is idempotent on the tag.)
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed (gate 23/23); fleet normalised. Older decisions archived to `.history/20260710/`.
