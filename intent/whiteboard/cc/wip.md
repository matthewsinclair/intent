---
node: cc
name: Control Claude
role: control
session_id: fccf3901-d524-4361-818c-d7007b5b7185
heartbeat_at: 2026-06-29T14:19Z
status: active
focus: "v2.13.1 SHIPPED; ST0048 + ST0049 COMPLETED"
claims: []
---

# Control Claude (cc)

## DOING

- v2.13.1 SHIPPED: tag `v2.13.1` (release commit `d01a1b2`) on both remotes + GitHub release; `intent upgrade` bumped this repo 2.13.0 -> 2.13.1 (config.json + CLAUDE.md). ST0048 fix landed in `baeae83`.
- ST0048 + ST0049 COMPLETED. FINAL DOGFOOD: `st done ST0048` closed through its OWN hardened gate (11/11 PASS); `st done ST0049` closed via `gate EXEMPT` -- the exemption proven end-to-end on a real thread.
- Pre-release catch: the gate broke 3 contractless-close tests in wp_commands.bats; adapted via a `write_exempt_acceptance` exempt fixture. Suite green in the release pre-flight.
- Post-ship wrap committed locally (bump + ST closures + board). NOT pushed -- hv pushes.

## TODO

- v2.13.1 stream complete. Optional: push the wrap commit; `/in-finish` for wip.md/restart.md + board release.
- Release as PATCH 2.13.1 with a migration-led release note (behaviour change for all consumers).
- AFTER the fix (own ST, likely ST0049): author `docs/releases/2.13.0/RELEASE_NOTES.md` (comprehensive: how MAAC works) + `docs/releases/2.13.1/RELEASE_NOTES.md` (the fix). Do NOT backfill 2.10.0/2.11.x/2.12.0 (hv ruling). Source: `intent/history/v2.13.0.md` + `v2.12.0.md` + `/in-whiteboard` skill + whiteboard README.

## Watch-outs

- `intent st new` stamps `acceptance.md` by default (`bin/intent_st:1207`), so file-presence is NOT the opt-in line today; the real opt-in line is the `total -eq 0` open at `bin/intent_acceptance:226`. Reversing it makes the contract mandatory fleet-wide -- a behaviour change for EVERY consumer (each upgraded project's open un-authored units stop closing). Ships as patch 2.13.1 on the shipped-as-broken framing (hv), so the release note + upgrade path MUST lead with the migration + the exemption escape hatch.

## Decisions

- (2026-06-29) hv ruling: fail BOTH the missing-acceptance.md path and the zero-AC path; an explicit exemption marker is the sole escape hatch. Default = enforced.
- (2026-06-29) hv ruling: ship the fix + both release notes in PATCH 2.13.1 (shipped-as-broken framing). docs/releases series lapsed after 2.9.0; resume at 2.13.x, no backfill of 2.10-2.12. 2.13.0 note is comprehensive MAAC how-it-works.
