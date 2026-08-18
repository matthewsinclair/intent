# Tasks - ST0040: Whiteboard protocol for multi-Claude sessions in the one repo

## Tasks

What was landed out-of-cycle on 2026-05-18 (already done):

- [x] Write `intent/plugins/claude/skills/in-whiteboard/SKILL.md` (canonical source).
- [x] Edit `intent/plugins/claude/skills/in-session/SKILL.md` to chain to `/in-whiteboard pickup`.
- [x] Edit `intent/plugins/claude/skills/in-finish/SKILL.md` to chain to `/in-whiteboard release`.
- [x] Live-test in Lamplight (`/Users/matts/Devel/prj/Lamplight/intent/whiteboard/`).

Shipped in v2.11.7 (2026-05-18):

- [x] **Manifest / registry**: discovery is directory-scan; no explicit manifest entry needed. `intent claude skills list` picks up `in-whiteboard` from canon at `intent/plugins/claude/skills/`. `tests/unit/skills_commands.bats` enumerates `in-whiteboard` in the canonical roster as a regression guard.
- [x] **Documentation**: new "Multi-session coordination" section in `intent/docs/working-with-llms.md` after "Skills and /in-session auto-load". Covers tense/reader/cadence distinction, file layout, stream identity discovery, ST-only claims, shared platform layer pattern, chain integration, heartbeat semantics, Lamplight live reference.
- [x] **Changelog**: `[2.11.7] - 2026-05-18` entry under `[Unreleased]`. Shipped as **patch** at user direction, overriding the "new skill = minor" precedent (the protocol is opt-in by directory presence, behaviour change is zero for projects that don't create `intent/whiteboard/`).
- [x] **Version bump**: VERSION → 2.11.7 via `scripts/release --patch`.
- [x] **Auto-install in upgrade path**: `bin/intent_upgrade` now calls `intent claude skills install in-whiteboard` and `intent claude skills sync` after the migration dispatcher completes (idempotent + failure-tolerant; no `--force` so user customisations are never silently lost). Regression test in `tests/unit/intent_upgrade_dispatcher.bats` asserts the install lands on a v2.10.x → current-target upgrade.
- [x] **Hook integration consideration**: decided **no** — the `/in-whiteboard pickup` chain from `/in-session` is sufficient; surfacing other-stream state earlier in the SessionStart hook would duplicate behaviour for marginal value. Revisit only if multi-session collisions surface before `/in-session` runs.
- [x] **Reference the Lamplight live example**: linked from the new docs section.
- [x] **LLMsend cross-pollination**: added `Re:` and `FYI only` header conventions to the `ask` subcommand prose. Borrowed from the cross-project LLMsend protocol — in-whiteboard is the intra-project sibling. The tmux/kitty live-ping mechanism was considered and deliberately not adopted (intra-project pickup at session-start is sufficient; the dependency footprint of tmux + kitty CSI u isn't justified).

Out-of-scope (do NOT "fix" these as part of this ST):

- [ ] **`intent st new` ST-ID allocation race**. The scan-disk-max+1 pattern can produce ST-ID collisions when two sessions create STs concurrently. Bit Lamplight on 2026-05 (ST0178). Fix needs flock + atomic stub-info.md write, in `intent st new` itself. File as its own bug/ST when picked up.

## Task Notes

### What was deliberately deferred (don't "fix" these)

These design decisions are intentional. Each was considered and rejected for the reasons in `design.md`:

- **No hook-based enforcement** of claim scope. v0 is advisory only. A `PreToolUse` hook that blocks edits outside claimed scope is a possible v1; advisory is sufficient for the two-stream case.
- **No `decisions.md` cross-stream event log**. Duplicates `done.md` past-tense, badly. Cross-stream decision-signal lives in the deciding stream's own file + `asks.md`.
- **No multi-session-per-stream modelling**. Misuse mode that has to be visible at the human layer; surface on `pickup` when `current_session_id` differs, but don't try to model concurrent sessions per stream.
- **No `intent/.config/whiteboard.json`** persistent per-project config. Stream-identity inference on `pickup` is sufficient; revisit if inference proves brittle.

### Naming note

The shared-platform-layer file is named `lamplight.md` in the live Lamplight example because Lamplight's shared platform layer is at `apps/lamplight/**`. In a non-Lamplight project, this file should be renamed to match -- eg `core.md`, `shared.md`, or whatever ID matches the project's shared-platform-layer naming. The documentation should call this out explicitly.

## Dependencies

- No upstream dependencies. Skill addition is self-contained.
- The `intent st new` race-fix is downstream (it would close the ST-ID-collision class entirely) but is NOT a blocker for this ST. Track separately.
- Lamplight is the live test environment; keep `/Users/matts/Devel/prj/Lamplight/intent/whiteboard/` in tree as the running-example reference.
