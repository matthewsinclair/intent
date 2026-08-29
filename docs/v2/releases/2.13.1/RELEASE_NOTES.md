# Intent v2.13.1 Release Notes

**Release Date**: 2026-06-29

## Overview

Intent v2.13.1 is a patch release that hardens the acceptance close-gate (ST0048).

The gate behind `intent st done` / `intent wp done` decides "done" by asking whether every in-scope Acceptance Criterion is satisfied. When a work package or steel thread had **zero acceptance criteria**, that question was vacuously true -- so the unit reported green and closed with no verifiable definition of done. A missing or empty acceptance contract was silently treated as a satisfied one, the exact inverse of the contract's purpose. It had slipped through repeatedly.

v2.13.1 makes that a hard failure. An empty or missing contract is refused; the sole escape is an explicit, visible `acceptance: exempt` marker. This is the No-Silent-Errors principle applied to the acceptance layer: an absent contract is a failure that must surface, not a quiet pass.

## Migration -- read this first

This is a behaviour change for **every** project, the moment the tool updates (the gate is served centrally from `$INTENT_HOME`; there is no per-project rollout):

- Any in-flight ST or WP that **never authored acceptance criteria** will now **fail to close** -- `intent st done` / `intent wp done` refuse it.
- Any thread with **no `acceptance.md` at all** (eg one created before the acceptance process existed) will likewise fail to close.

Two ways to adapt each affected unit:

1. **Author its contract.** Add real Acceptance Criteria to the unit's `acceptance.md` (`intent ac list <id>` shows the current state) and satisfy them the usual way.
2. **Declare it exempt.** If the unit is deliberately AC-free (eg a pure content / authorial task), add one line to its `acceptance.md` frontmatter:

   ```yaml
   ---
   st_id: ST0042
   acceptance: exempt
   ---
   ```

   The gate then passes and prints an `EXEMPT` line naming the exemption. Nothing is auto-applied: Intent never writes an exemption on your behalf -- that would re-hide the hole.

To find affected units, run `intent ac status <id>` on each open thread: `0/0 satisfied` (or a "no acceptance.md" error) means it will now block.

## What changed

`intent ac gate` (the single authority `st done` / `wp done` consult) now refuses to open when the in-scope contract is not verifiably done:

| Case                      | Before           | After                                           |
| ------------------------- | ---------------- | ----------------------------------------------- |
| No `acceptance.md`        | passed (vacuous) | **BLOCKED** -- "no acceptance.md (no contract)" |
| Present, zero ACs         | passed (vacuous) | **BLOCKED** -- "zero acceptance criteria"       |
| `acceptance: exempt`      | (not read)       | **EXEMPT** -- passes, announced                 |
| Real ACs, all satisfied   | passed           | passed (unchanged)                              |
| Real ACs, any unsatisfied | BLOCKED          | BLOCKED (unchanged)                             |
| Malformed AC/AT lines     | BLOCKED          | BLOCKED (unchanged)                             |

**WP scope is WP-lenient.** A work package with no acceptance criteria of its own still closes, as long as the steel thread carries a contract (its proof rolls up to the ST boundary). Only a fully-empty, non-exempt thread blocks `wp done`. A missing thread directory is not a unit and is unaffected.

## Upgrade

```bash
intent upgrade --apply
```

The upgrade installs the hardened gate; it does **not** mutate any project's `acceptance.md`. After upgrading, a unit that used to close on an empty contract will report BLOCKED until you author its ACs or mark it exempt (see Migration above). Restarting your Claude Code session is not required -- the gate is a plain CLI path.

## Why

A self-reported "done" drifts; ST0044 bound "done" to a ratified AC set, proven by red-first tests and enforced by a computed close-gate. But an _absent_ contract slipped under that bar -- zero criteria satisfied zero criteria. v2.13.1 closes that: the default is now fail, and any exemption is explicit and visible both in the file and in the tool output. It deliberately reverses the earlier opt-in-by-presence behaviour, because a safety gate that is silent when un-adopted is not a safety gate.
