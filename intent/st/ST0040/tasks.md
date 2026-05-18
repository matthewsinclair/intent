# Tasks - ST0040: Whiteboard protocol for multi-Claude sessions in the one repo

## Tasks

What was landed out-of-cycle on 2026-05-18 (already done):

- [x] Write `intent/plugins/claude/skills/in-whiteboard/SKILL.md` (canonical source).
- [x] Edit `intent/plugins/claude/skills/in-session/SKILL.md` to chain to `/in-whiteboard pickup`.
- [x] Edit `intent/plugins/claude/skills/in-finish/SKILL.md` to chain to `/in-whiteboard release`.
- [x] Live-test in Lamplight (`/Users/matts/Devel/prj/Lamplight/intent/whiteboard/`).

What still needs to happen for a formal Intent release:

- [ ] **Manifest / registry**: verify `intent claude skills list` picks up `in-whiteboard`. If Intent uses an explicit manifest (eg `intent/plugins/claude/skills/_manifest.json`), add an entry; if discovery is directory-scan, no action needed -- `intent claude skills list -v` from any project should already show `in-whiteboard` as available.
- [ ] **Documentation**: add a section to `docs/working-with-llms.md` (or equivalent) on multi-session coordination. Cover the tense / reader / cadence distinction vs `wip.md`, stream-identity discovery on `pickup`, the ST-only claim primitive, and the shared-platform `lamplight.md` pattern (renamed per project).
- [ ] **Changelog**: add an entry to `CHANGELOG.md` for the next minor version describing the skill + the in-session/in-finish chain additions.
- [ ] **Version bump**: bump `VERSION` per Intent's semver rules (likely a minor bump -- new skill is feature-level).
- [ ] **Fresh-project smoke test**: bootstrap a fresh Intent project from canonical templates and verify:
  - [ ] `intent claude skills install in-whiteboard` succeeds.
  - [ ] `intent claude skills sync` picks up the in-session + in-finish edits.
  - [ ] `/in-session` in a project with no `intent/whiteboard/` directory skips the pickup chain silently (no behaviour change).
  - [ ] `/in-session` in a project with `mkdir intent/whiteboard` + a stream file triggers `pickup` cleanly and surfaces "no other streams active".
- [ ] **Hook integration consideration**: decide whether the SessionStart hook (`.claude/scripts/session-context.sh`) should also surface "other-stream active" warnings earlier in the lifecycle than `/in-whiteboard pickup`. Likely not worth the duplication; document the decision either way.
- [ ] **Reference the Lamplight live example**: link `/Users/matts/Devel/prj/Lamplight/intent/whiteboard/` from the new documentation section as the running-example reference.

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
