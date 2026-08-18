# Implementation - ST0048: Acceptance close-gate fails empty or missing contract

## Implementation

As-built matches `design.md`. The change is localised to `cmd_ac_gate` in `bin/intent_acceptance` plus a new one-line `acceptance_field` reader (house pattern: `grep -m1 '^acceptance:' | sed`, cf the `status:` read in `bin/intent_st`). Control flow: missing `acceptance.md` -> BLOCK; `acceptance: exempt` frontmatter -> EXEMPT (exit 0, announced); malformed lines -> BLOCK (F1, unchanged); thread-total zero ACs -> BLOCK; in-scope total zero but thread contracted -> exit 0 (WP-lenient roll-up); sat==total -> exit 0; else BLOCK. hv ratified WP-lenient (design.md D3).

Canon flipped off the retired "opt-in / legacy-safe / closes exactly as before" framing in lockstep (WP-02): `working-with-llms.md` D11, the `bin/intent_st` / `bin/intent_wp` consumer comments, the gate header, and the stamped `acceptance.md` template (which now documents the marker). A grep guard (`AT-02.1`) pins it.

## Challenges & Solutions

- **The migration landed on our own suite.** The hardened gate broke three pre-existing `wp_commands.bats` tests that close a WP whose ST has no `acceptance.md` (old opt-in passed; the gate now blocks). They exercise wp-done mechanics, not the contract, so they declare `acceptance: exempt` via a new `write_exempt_acceptance` test helper. Lesson for the fleet: any test or workflow that closes a contractless ST/WP must adapt the same way.
- **Release-coupled AC.** AC-03.2 ("version stamped 2.13.1") is satisfiable only AFTER `scripts/release` + the post-tag bump, not during the build -- so closing ST0048 waited on the release. `st done ST0048` then passed the thread through its own 11/11 gate: the final dogfood. ST0049 (the 2.13.0 note) is `acceptance: exempt`, the marker's first real consumer.

## Technical Details

Shipped in v2.13.1 (release tag `d01a1b2`); fix commit `baeae83`; post-ship wrap `c0eeefe`. Tests: `acceptance_close_gate.bats` 10/10 (the report's three self-tests + the EXEMPT and WP-lenient cases), `wp_commands.bats` 29/29.
