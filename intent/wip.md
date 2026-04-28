---
verblock: "28 Apr 2026:v0.65: matts - v2.10.1 shipped; scripts/release lives; gate-firing fix in"
intent_version: 2.10.1
---

# Work In Progress

## Current State

**Intent v2.10.1 shipped 2026-04-28.** Tag pushed to `local` + `upstream`; GitHub release live at https://github.com/matthewsinclair/intent/releases/tag/v2.10.1. No active steel thread.

The cut was the first end-to-end exercise of the new `scripts/release` orchestrator, which landed in this same line. Single invocation did pre-flight, sidecar sync, commit, tag, multi-remote push, and `gh release create` -- one watch-able trace, one confirmation point at the push step. Replaces the manual six-step ceremony from the v2.10.0 cut.

Tests **836/836 green**. `intent doctor` clean.

## v2.10.1 ship list (Completed)

| Theme        | Item                                                                               | Commit            |
| ------------ | ---------------------------------------------------------------------------------- | ----------------- |
| Fix (urgent) | `/in-session` gate-firing loop -- extract awk from SKILL.md to release-gate.sh     | f44717e           |
| Add          | `intent doctor` check 4d -- warn on leftover top-level `.intent/` post-migration   | f44717e           |
| Add          | `scripts/release` -- one-shot release orchestrator (Conflab-pattern, Intent-shape) | 0f7bcef           |
| Polish       | `intent claude upgrade --dry-run` UX (canonical / legacy / absent triage)          | cf3dfa8           |
| Polish       | Diogenes test-spec handoff alignment across all four critic agents                 | cf3dfa8           |
| Polish       | IN-RS-CODE-005 carve-out for teaching fixtures                                     | cf3dfa8           |
| Release      | CHANGELOG promotion + sidecar sync + tag + push + GitHub release                   | 09700df + c453cbb |

## Recent

- **2026-04-28**: v2.10.1 cut and shipped via the new `scripts/release` script. Gate-firing fix surfaced from this morning's user report (SKILL.md `awk '{print $1}'` had the `$1` silently stripped by Claude Code's skill renderer; producing a malformed `project_key` and no-op gate release). Fixed by extracting the waterfall to `intent/plugins/claude/skills/in-session/scripts/release-gate.sh` so the renderer never sees the pipeline. Bundled the four pre-existing v2.10.x dogfood follow-ups (doctor warning, dry-run UX polish, Diogenes alignment, IN-RS-CODE-005 carve-out) into the same release line. +26 BATS scenarios across release_gate_script.bats, doctor_leftover_intent.bats, and release_script.bats.

- **2026-04-27**: v2.10.0 shipped. ST0035 + ST0036 closed (see `intent/restart.md` for that arc).

## Next Up

No active steel thread. Open follow-ups for v2.11 or later:

- **`docs/blog/_drafts/####-shell-critic-inception.md`** -- blog draft. Now has a real second dogfood datapoint (the v2.10.1 cut itself, a single-command release).
- **Homebrew tap for Intent** -- when ready to broaden distribution. Conflab's release script has the formula-update pattern as a reference.
- **`scripts/release` v2** -- post-real-use refinements. Candidates: `--rollback`, auto-bump (rejected for v2.10.1; reconsider after a few cuts), prettier progress output, log-to-file mirror.

## Parked

_(None.)_

## Open Follow-ups (post v2.10.1)

- Blog draft retains its slot (was deferred from v2.10.0). The "shell-critic inception" angle now has the v2.10.1 cut as second datapoint -- the script ate its own dogfood.
- The skill-renderer bug is upstream-of-Intent (Claude Code platform behaviour). Worth a forensic note in MEMORY for future skill authors: don't inline `awk '{print $N}'` in SKILL.md; positional-field expansions get stripped during prompt injection. Move logic to a `scripts/<name>.sh` file and invoke by path.
