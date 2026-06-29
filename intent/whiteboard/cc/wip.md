---
node: cc
name: Control Claude
role: control
session_id: fccf3901-d524-4361-818c-d7007b5b7185
heartbeat_at: 2026-06-29T13:05Z
status: active
focus: "ST0048 fix + ST0049 note committed to main; awaiting external full-suite pass, then hv 2.13.1 release"
claims: [ST0048, ST0049]
---

# Control Claude (cc)

## DOING

- ST0048 (fix): WP-01/02 GREEN + closed through the new gate; WP-03 content (docs/releases/2.13.1 + CHANGELOG). hv signed off AC-00.1 -> 10/11; only AC-03.2 (version stamp) remains, which the release performs. `acceptance_close_gate.bats` 10/10.
- ST0049 (2.13.0 MAAC note): `docs/releases/2.13.0/RELEASE_NOTES.md` authored; thread `acceptance: exempt`, dogfood GREEN.
- Committed to main (this commit). hv running the full suite externally; release gated on 100% green.

## TODO

- Blocked on external full-suite result (hv running). On green: hv `scripts/release --patch` cuts 2.13.1, then the post-tag config.json/VERSION bump to 2.13.1.
- After the release: cc satisfies AC-03.2 (version now stamped) -> closes ST0048 (`st done` = final dogfood) + ST0049 (exempt). NOT pushed yet (commit only).
- Release as PATCH 2.13.1 with a migration-led release note (behaviour change for all consumers).
- AFTER the fix (own ST, likely ST0049): author `docs/releases/2.13.0/RELEASE_NOTES.md` (comprehensive: how MAAC works) + `docs/releases/2.13.1/RELEASE_NOTES.md` (the fix). Do NOT backfill 2.10.0/2.11.x/2.12.0 (hv ruling). Source: `intent/history/v2.13.0.md` + `v2.12.0.md` + `/in-whiteboard` skill + whiteboard README.

## Watch-outs

- `intent st new` stamps `acceptance.md` by default (`bin/intent_st:1207`), so file-presence is NOT the opt-in line today; the real opt-in line is the `total -eq 0` open at `bin/intent_acceptance:226`. Reversing it makes the contract mandatory fleet-wide -- a behaviour change for EVERY consumer (each upgraded project's open un-authored units stop closing). Ships as patch 2.13.1 on the shipped-as-broken framing (hv), so the release note + upgrade path MUST lead with the migration + the exemption escape hatch.

## Decisions

- (2026-06-29) hv ruling: fail BOTH the missing-acceptance.md path and the zero-AC path; an explicit exemption marker is the sole escape hatch. Default = enforced.
- (2026-06-29) hv ruling: ship the fix + both release notes in PATCH 2.13.1 (shipped-as-broken framing). docs/releases series lapsed after 2.9.0; resume at 2.13.x, no backfill of 2.10-2.12. 2.13.0 note is comprehensive MAAC how-it-works.
