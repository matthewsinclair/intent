---
verblock: "25 Jun 2026:v0.2: matts - Scope the promotion WP"
wp_id: WP-04
title: "Promote to Intent (intent claude) + back-fill siblings"
scope: Small
status: Done
---

# WP-04: Promote to Intent (intent claude) + back-fill siblings

## Objective

Lift the proven Baize prototype into first-class Intent: the `intent claude` command family gains `start` + `ws`, the scaffold logic shares one SSOT with the `/in-whiteboard` skill, and the capability back-fills the sibling projects.

## Deliverables

- Confirm the `intent claude` dispatch location (not a `bin/intent_claude` file) and relocate the capability into it -> `intent claude start|ws ...`.
- Port the prototype, do not fork: `bin/claude_with_intent` logic + `test/cwi/cwi_test.sh` travel to Intent's home (where `critic-shell` governs).
- Wire the one format SSOT: the `/in-whiteboard` skill's hand-scaffold prose points at `ws new` (no divergent scaffolder). (Highlander)
- Back-fill Laksa + Lamplight + Intent (each runs `intent claude start <ws>` against its own whiteboard) -- per-project smoke.
- Fold any Baize-specific assumptions (paths, restart-file name) into Intent config so the command is project-agnostic.
- **Retire the Baize prototype in one cutover** once `intent claude` is proven + back-filled: a single Baize commit -- `git rm bin/claude_with_intent test/cwi/` + switch muscle-memory to `intent claude start <ws>`. The whiteboard data (`intent/whiteboard/`) STAYS; only the launcher leaves. No coexistence window (Highlander + no-backwards-compat-shims).

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-04` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-01..03 green + the Baize dogfood pass (don't promote an unproven prototype).
- External: the `intent` CLI dispatch internals; the `/in-whiteboard` skill (the SSOT it must converge with).
