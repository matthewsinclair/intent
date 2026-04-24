---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-01
title: "Self-upgrade Intent to v2.9.1 and cancel ST0010 and ST0015"
scope: ExtraSmall
status: Done
---

# WP-01: Self-upgrade Intent to v2.9.1 and cancel ST0010 and ST0015

## Objective

Bump Intent's own `.intent/config.json` from `2.8.2` to `2.9.1`, add the v2.9.0 → v2.9.1 migration stub in `bin/intent_helpers`, and cancel the two stale NOT-STARTED steel threads (ST0010, ST0015) via `intent st` — moving them to `intent/st/CANCELLED/` with a deprecation one-liner at the top of each `info.md`.

## Context

Intent v2.9.0 shipped 2026-04-23 but Intent's own version stamp in `.intent/config.json` never bumped past `2.8.2` (audit confirmed). The generated `intent/llm/AGENTS.md` footer correctly reads `Intent v2.9.0`, so the content is current — it's just the stamp that lags. WP01 fixes that and adds the migration entry point for v2.9.1 before any later WP touches project state.

Two NOT-STARTED steel threads are overtaken:

- **ST0010 "Anthropic MCP Integration"** (created 2025-06-03, intent_version 2.0.0). References "STP" (Intent's pre-rename name). "Low-priority exploration." Superseded by v2.9.0 skill/subagent/extension system.
- **ST0015 "Enhanced Steel Thread Templates"** (created 2025-07-09, intent_version 2.0.0). Aspirational enhancements to ST directory structure. Overtaken by v2.8.x directory work and v2.9.0 extension system.

Both cancel; neither deletes. User decision: use existing `Cancelled` status + top-of-file annotation; no new directory convention. `intent/st/CANCELLED/` already exists.

WP01 is deliberately the smallest WP in the ST — it exercises the `intent st` CLI and the migration-stub pattern without touching any templates, generators, or rollout code. If this WP is clean, all later WPs have a known-good starting point.

## Deliverables

1. **Version bump**: `.intent/config.json` `intent_version` → `2.9.1`.
2. **Migration stub**: `bin/intent_helpers` gets `migrate_v2_9_0_to_v2_9_1()` function, called from the existing migration chain. At this point the stub only bumps the version stamp and logs the migration; canon-apply logic lands in WP11.
3. **Version constant**: `lib/VERSION` (or equivalent) updated to `2.9.1`.
4. **CHANGELOG entry**: stub entry for `v2.9.1` added — title only, body fills out across the ST.
5. **ST0010 cancellation**:
   - Run `intent st done ST0010 --status cancelled` (or the equivalent — confirm exact flag syntax in WP01 kickoff; `intent st` supports a cancelled state per the help output).
   - Confirm move to `intent/st/CANCELLED/ST0010/`.
   - Prepend to `info.md`: `> **Deprecated:** superseded by Intent v2.9.0 (skill / subagent / extension system). Cancelled 2026-MM-DD.`
6. **ST0015 cancellation**: same process.
7. **Verification that `intent st list --status cancelled` shows both**.

## Approach

1. Confirm the `intent st` subcommand for cancelling (the help text lists `start`, `done`, and "Valid status values" including `cancelled` — verify exact invocation; may be `intent st done <id>` with a status flag, or may require a direct status update via `intent st edit`).
2. Bump `.intent/config.json` by running the upgrade command first (dry-run, then apply) — existing `intent upgrade` pattern handles the stamp.
3. Author `migrate_v2_9_0_to_v2_9_1()` in `bin/intent_helpers` as a mirror of the existing `migrate_v2_8_2_to_v2_9_0()` pattern (read the existing function for shape). Keep the stub minimal at this WP: bump stamp, log migration, return success. Canon-apply logic is WP11.
4. Cancel ST0010 via CLI. Immediately edit `info.md` to prepend deprecation annotation (use `intent st edit ST0010` to open in editor OR use direct Edit on the moved file).
5. Cancel ST0015 via CLI. Annotate similarly.
6. Run `intent doctor` to confirm clean state.
7. Update `CHANGELOG.md` v2.9.1 stub.
8. Commit: `chore: bump to v2.9.1 and cancel ST0010/ST0015`.

## Acceptance Criteria

- [ ] `cat .intent/config.json | jq .intent_version` returns `"2.9.1"`.
- [ ] `grep -E 'migrate_v2_9_0_to_v2_9_1' bin/intent_helpers` returns the function definition.
- [ ] `bin/intent version` (or `bin/intent info`) reports `2.9.1`.
- [ ] `ls intent/st/CANCELLED/` includes both `ST0010/` and `ST0015/`.
- [ ] First line of `intent/st/CANCELLED/ST0010/info.md` (under frontmatter) reads: `> **Deprecated:** superseded by Intent v2.9.0 ...`.
- [ ] First line of `intent/st/CANCELLED/ST0015/info.md` (under frontmatter) reads: `> **Deprecated:** superseded by Intent v2.9.0 ...`.
- [ ] Status in both `info.md` frontmatter reads `Cancelled`.
- [ ] `bin/intent st list --status cancelled` shows both ST0010 and ST0015.
- [ ] `bin/intent doctor` exits 0, no errors.
- [ ] `tests/run_tests.sh` exits 0 (pristine invariant — WP01 adds no runtime changes, so baseline holds).
- [ ] `CHANGELOG.md` has a `v2.9.1` heading (body can be empty at this point).
- [ ] Commit message follows Intent's commit conventions (no Claude attribution).

### Tests to add

None. WP01 is a metadata and ST-state change only; no new testable behaviour.

### Tests to update

None. Baseline stays green.

## Dependencies

None. Foundational WP; everything else depends on this.

## Implementation Notes

- Existing `migrate_v2_8_2_to_v2_9_0` is at `bin/intent_helpers`. Read it as a structural template. Key concerns it handles: stamp, bootstrap of `~/.intent/ext/`, seed worker-bee, prune deprecated agents. The v2.9.0 → v2.9.1 stub just needs the stamp bump at this WP — other concerns are later WPs (canon artefacts in WP11).
- **Cancelled state mechanics**: double-check `bin/intent_st` for the exact cancel semantics. If there's no direct `intent st cancel`, the path may be `intent st edit` + manual status change + `intent st organize --write` (which moves files to the right status directory per the audit). Confirm before executing.
- **Deprecation annotation format**: prepend immediately after the YAML frontmatter close (`---`) and before the first heading. Use a blockquote `>` for visual distinction.
- **Date in annotation**: use the actual commit date, not a projection.
- **verblock update**: when editing ST0010/ST0015 `info.md`, bump the `verblock` line to record this edit.
- **Do not reword the existing objective/context** of ST0010 or ST0015 — the deprecation is a cap, not a rewrite. Keep history intact.

## Risks and Edge Cases

- **Risk**: `intent st` CLI has no single-command cancel; requires edit + organize sequence. **Mitigation**: Check help output at WP start; if needed, do it via `intent st edit` + file move + `intent st organize --write`. Worst case: manual `git mv` + info.md edit (document this in as-built if needed).
- **Risk**: Migration stub triggers on other projects running `intent upgrade` from v2.9.0 during rollout, and the stamp-only bump is misleading because canon-apply logic isn't ready. **Mitigation**: Stub is safe — bumping the stamp from 2.9.0 to 2.9.1 on a project is always OK; canon-apply in WP11 is idempotent and can re-run without issue.
- **Edge**: ST0010 and ST0015 have `verblock` entries from 2025. Preserve the history chain when editing.
- **Edge**: `CHANGELOG.md` v2.9.1 entry with an empty body may trip over any markdown linter that requires content. Add a placeholder line like `- In progress (ST0035).` to keep linters happy.

## Verification Steps

1. `git log --oneline -1` shows WP01 commit with correct message format.
2. `bin/intent doctor` (from Intent root) returns 0 with no warnings.
3. `bin/intent info` reports `intent_version: 2.9.1`.
4. `bin/intent st list --status cancelled` shows exactly 2 entries (ST0010, ST0015) — plus any pre-existing cancelled STs.
5. `diff <(git show HEAD~1:.intent/config.json) .intent/config.json` shows only the version bump.
6. Manually view `intent/st/CANCELLED/ST0010/info.md` and confirm the deprecation annotation is visible at the top.
7. Run a no-op test of migration idempotence: `bin/intent upgrade --dry-run` should report "already at v2.9.1" (or equivalent).

## Size and Estimate

- **Size**: XS (Extra Small). Single session.
- Session: do all steps in sequence, verify, commit.

## Exit Checklist

Before closing WP01:

- [ ] All acceptance criteria met.
- [ ] Migration stub tested: running `intent upgrade` on a v2.9.0 project bumps to v2.9.1 without error.
- [ ] Both cancelled STs visible in `bin/intent st list --status cancelled`.
- [ ] `tests/run_tests.sh` passes at baseline.
- [ ] CHANGELOG.md v2.9.1 stub present.
- [ ] Committed and branch clean.
