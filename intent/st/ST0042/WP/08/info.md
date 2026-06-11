---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-08
title: "Canon docs reconciliation + st cancel"
scope: M
status: Not Started
---

# WP-08: Canon docs reconciliation + st cancel

## Objective

Make the canon docs true (theme T9). Includes adding `intent st cancel <ID>` (gate decision 2026-06-11: the docs already promise it; add the command rather than retract the docs).

## Evidence

- F-DOCS-2 (HIGH, confirmed): `usage-rules.md:55,338` mandates `intent st cancel` and forbids manual `status:` edits, but `bin/intent_st` has no `cancel` dispatch case.
- F-DOCS-1 (HIGH): `usage-rules.md:22-23,339` documents the pre-v2.10 `.intent/config.json` path.
- F-DOCS-3 (HIGH): `working-with-llms.md:199-237,254,466,483` documents hook scripts and a `matchers` shape that never existed; real template uses `matcher` + `.claude/scripts/{session-context,require-in-session}.sh`; the documented strict->soft escape hatch is unexecutable.
- F-DOCS-6 (HIGH): `critics.md:152-162` describes filesystem-probe language dispatch removed by ST0037; real `in-review` reads the `languages` array.
- F-DOCS-5 (MEDIUM): `working-with-llms.md:529` references nonexistent `intent claude skills status`.
- Lower severity: README claims v2.6.0 (F-DOCS-9); `in-whiteboard` missing from `usage-rules.md` skills table (F-DOCS-14); CLAUDE.md says v2.11.0 (F-DOCS-10); `working-with-llms.md` stamp v2.9.1 (F-DOCS-11); `rules.md` says nine required sections vs validator's seven (F-DOCS-16); stale ST paths now under `COMPLETED/` (F-DOCS-15); `writing-extensions.md` promises unshipped v2.10 features (F-DOCS-12); README lists nonexistent `intent/usr/` and misplaces AGENTS.md (F-DOCS-8).

## Deliverables

- New `intent st cancel <ID>` dispatch case in `bin/intent_st`: sets status Cancelled and relocates per the existing CANCELLED discipline; thin (parse -> call -> render); bats coverage.
- Each F-DOCS item above corrected to match the as-built system (verify against code at fix time -- line numbers may have moved).
- `usage-rules.md` regenerated/edited consistently with its template if applicable (check `lib/templates/llm/_usage-rules.md` ownership before editing the root file -- coordinate with WP-02).

## Acceptance Criteria

- [ ] `intent st cancel <ID>` works end-to-end and is the documented, compliant cancellation path.
- [ ] No canon doc describes a command, path, flag, or hook shape that does not exist.
- [ ] Version stamps in README/CLAUDE.md/working-with-llms.md reflect the current release at fix time.
- [ ] Full bats suite green.

## Dependencies

- After WP-02 (rules-path swap lands first so docs describe the post-fix state once); before WP-06 (prune) lands its doc edits, or coordinated with it.
