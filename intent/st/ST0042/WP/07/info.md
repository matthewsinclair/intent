---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-07
title: "Reconcile MODULES.md registry"
scope: XS
status: WIP
---

# WP-07: Reconcile MODULES.md registry

## Objective

Restore the Highlander-registry integrity of `intent/llm/MODULES.md` (theme T7) so the check-before-you-create gate works.

## Evidence

- F-ARCH-8: three live subagents on disk (`intent`, `diogenes`, `socrates`) unregistered in MODULES.md.
- F-DOCS-13: `needs_v2_9_0_upgrade` registry row describes config-file-reading behaviour the function (which takes a version arg) does not have, with a stale `.intent/` path.

## Deliverables

- Rows for the three subagents added with accurate descriptions.
- `needs_v2_9_0_upgrade` row corrected to the function's real signature/behaviour and current path.
- A quick sweep for any other rows describing behaviour that drifted (bounded: registry vs reality spot-check, not a re-audit).

## Acceptance Criteria

- [ ] Every live module/subagent on disk has a truthful MODULES.md row.
- [ ] No row references the pre-v2.10 `.intent/` path.

## Dependencies

- Coordinates with WP-05 (relocated helpers get rows) and WP-06 (deleted modules lose rows). Cheap to do mid-stream and re-touch.
