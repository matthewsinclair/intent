---
node: cc
name: Control Claude
role: control
session_id: 4ec8fcd8-3834-43f3-af24-8cabe1eb0958
heartbeat_at: 2026-07-24T08:57Z
status: active
focus: "Issue 0004 (high) FIXED + CLOSED + COMMITTED (e135349). v2.17.3 staged in CHANGELOG; matts runs the release manually. Awaiting next."
claims: []
---

# Control Claude (cc)

## DOING

- **Issue 0004 (high, opened 2026-07-24 by hv) -- `intent ac gate` vacuous pass. FIXED + CLOSED + COMMITTED (`e135349`); v2.17.3 staged, matts releases manually.** Target resolution is now a distinct failable step ahead of evaluation, in ONE resolver (`resolve_target`) the whole `ac`/`at` family shares -- it validates the `/NN` segment nothing validated before, and reports a bad target as BLOCKED+exit 1 from the gate, `Error:`+exit 1 from the readers. New `resolve_wp_dir` in `intent_helpers` (WP analogue of `resolve_st_dir`); the 3 resolving `intent_wp` sites share it; `acc_path` retired. Every gate verdict now announced (PASS joins EXEMPT/BLOCKED) -- silence-on-success is what hid this through 3 releases. Also fixed: `parse_wp_specifier` fed `/NN` to a bare `10#` expansion, so `wp show|start|done <st>/abc` aborted with raw bash noise. hv's fix items 1-3 done; **item 4 NOT actioned -- premise does not reproduce** (`ac status` exits 0 on BOTH BLOCKED shapes here, and `intent_acceptance_cli.bats:111` asserts it; status is the reporter, gate is the gate). Flagged in the issue for an hv ruling. 16/16 gate tests green, 6 new + all verified RED pre-fix; 8 adjacent suites clean; critic-shell clean.

_(day closed 2026-07-13 -- v2.17.2 shipped: issues 0002 + 0003 fixed + closed + released. Session detail archived to `.history/20260713/`.)_

## TODO

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
