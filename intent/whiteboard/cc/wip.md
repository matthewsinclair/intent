---
node: cc
name: Control Claude
role: control
session_id: b9414b4d-2a1e-42bc-98e9-9fc6b795f865
heartbeat_at: 2026-07-02T22:06Z
status: paused
focus: "v2.14.0 SHIPPED (ST0050 + ST0051); self-upgraded 2.14.0, doctor green, vc PASS; session wrapped via /in-finish -- node paused"
claims: [ST0050, ST0051]
---

# Control Claude (cc)

## DOING

_(idle -- session wrapped via /in-finish)_ **v2.14.0 SHIPPED** (ST0050 `intent todo` + ST0051 width): tag `v2.14.0` + GitHub release on both remotes; self-upgraded to 2.14.0 (`intent doctor` green); post-tag wrap `a6f6662`. vc audit PASS; matts accepted the WP-06 sticky-watermark. Homerun detail archived in `.history/20260702/`.

## TODO

Nothing active. The v2.14.1 follow-ups (from vc's audit) live in `intent/wip.md`: AC-01.8 enumeration Highlander, AT-name traceability, `intent upgrade` false-no-op + `scripts/release` `confirm()` hardening.

## Watch-outs

- Everything pushed -- main (`3f4452a`+) and the `v2.14.0` tag on both remotes; nothing outstanding.

## Decisions (ratified)

- (2026-07-02) matts ACCEPTED the WP-06 sticky-watermark model (DONE = "completed since the last flush") -- the acceptance-verify flag is closed.
- (2026-07-02) hv RATIFIED D1-D4; cc RULED WP-06 as-built (verb placement, sticky `>=` watermark, UTC, prune stdout/note-stderr). Full detail archived in `.history/20260702/wip.md`; canon in `intent/st/COMPLETED/ST0050/design.md`.
- (earlier) v2.13.1 SHIPPED. Detail in `intent/wip.md` + git history.
