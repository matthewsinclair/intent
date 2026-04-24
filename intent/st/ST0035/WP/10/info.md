---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-10
title: "Delete deprecated intent/llm/AGENTS.md and _llm_preamble.md"
scope: ExtraSmall
status: Not Started
---

# WP-10: Delete deprecated intent/llm/AGENTS.md and \_llm_preamble.md

## Objective

Delete the retired Intent artefacts: `intent/llm/AGENTS.md` (Intent's own copy) and `lib/templates/llm/_llm_preamble.md` (unused legacy template). Update MODULES.md. Fail-forward: no shadow retention, no redirect.

## Context

Design D4 and D9 are unambiguous: `intent/llm/AGENTS.md` retires. The real AGENTS.md lives at root (WP08 generator writes there). `_llm_preamble.md` was a pre-v2.9.0 generator template with no live consumers; vestigial.

WP10 is the small cleanup WP. It's gated on WP08 because the generator retarget must land first — otherwise `intent agents sync` would still write to `intent/llm/AGENTS.md` and re-create the file.

Fleet-wide: each project's `intent/llm/AGENTS.md` gets deleted during the WP16 rollout via `intent claude upgrade --apply` (WP11 ships the delete logic).

## Deliverables

1. **Delete** `intent/llm/AGENTS.md` (Intent's own copy) via `git rm`.
2. **Delete** `lib/templates/llm/_llm_preamble.md` via `git rm`.
3. **Update** `intent/llm/MODULES.md` to remove the now-retired entries for both files.
4. **Search and delete** any internal references: grep `intent/llm/AGENTS.md` and `_llm_preamble.md` across the codebase; update any finding to reflect the new locations.
5. **CHANGELOG note**: v2.9.1 entry gets a "Removed" line documenting these deletions.

## Approach

1. Confirm WP08 has landed — root AGENTS.md generator is in place.
2. `git rm intent/llm/AGENTS.md`.
3. `git rm lib/templates/llm/_llm_preamble.md`.
4. Grep for `intent/llm/AGENTS.md` in bin/ and intent/ — audit each hit, update as appropriate.
5. Grep for `_llm_preamble.md` — audit each hit, remove references.
6. Update MODULES.md: remove both file entries.
7. Update CHANGELOG.md v2.9.1 entry (add Removed line).
8. Verify Intent's own `tests/run_tests.sh` passes with the deletions.
9. Commit: `chore: retire intent/llm/AGENTS.md and _llm_preamble.md (v2.9.1 fail-forward)`.

## Acceptance Criteria

- [ ] `test ! -f intent/llm/AGENTS.md` at repo root (file gone).
- [ ] `test ! -f lib/templates/llm/_llm_preamble.md` (file gone).
- [ ] `git log --follow intent/llm/AGENTS.md` shows a deletion commit.
- [ ] `grep -r "intent/llm/AGENTS.md" bin/ intent/ lib/` — no active references remain (comments referencing retirement OK).
- [ ] `grep -r "_llm_preamble" bin/ intent/ lib/` — no active references.
- [ ] `intent/llm/MODULES.md` does not list either file.
- [ ] CHANGELOG.md v2.9.1 entry has a "Removed" line.
- [ ] `tests/run_tests.sh` passes.
- [ ] `intent doctor` on Intent's own repo exits 0 (no errors about missing files).
- [ ] Commit follows Intent conventions.

### Tests to add

None — this is a delete WP. Any tests that referenced the deleted files should have been updated in WP08.

### Tests to update

- Any BATS test asserting `intent/llm/AGENTS.md` presence — flip to root or delete.

## Dependencies

- **Blocked by**: WP08 (generator must write to root before we delete the old location).
- **Blocks**: None — this is the final cleanup step for the intent/llm/ path change.

## Implementation Notes

- **Grep discipline**: audit every hit before deleting. Some references may be intentional (docs explaining the retirement, ST0034 implementation notes, historical CHANGELOG entries). Don't blindly delete; discriminate.
- **Intent's own project**: deleting `intent/llm/AGENTS.md` means re-running `intent agents sync` on Intent itself to produce a root AGENTS.md. WP14 handles the self-apply; WP10 deletes but doesn't regenerate.
- **Fleet deletion timing**: each project's `intent/llm/AGENTS.md` gets deleted by the per-project `intent claude upgrade --apply` during WP14 (Intent self-apply), WP15 (canary), and WP16 (fleet). WP10 only deletes Intent's own + the template.
- **CHANGELOG note matters**: "Removed" entries in CHANGELOG protect against users who grep release notes for breakage. Be explicit.

## Risks and Edge Cases

- **Risk**: Hidden references to `intent/llm/AGENTS.md` break at runtime. **Mitigation**: thorough grep. Run `tests/run_tests.sh`.
- **Risk**: Users with local tooling reading the old path. **Mitigation**: CHANGELOG notice. This is fail-forward by instruction.
- **Risk**: Deleting `_llm_preamble.md` breaks a generator that silently referenced it. **Mitigation**: grep all generator scripts; verify no import.
- **Edge**: Git history for the deleted files remains accessible via `git log --follow`. Expected behaviour.

## Verification Steps

1. `ls intent/llm/` — `MODULES.md` and `DECISION_TREE.md` remain; no AGENTS.md.
2. `ls lib/templates/llm/` — no `_llm_preamble.md`.
3. `grep -r "intent/llm/AGENTS.md" .` (excluding `.git` and `intent/st/COMPLETED` historical refs) — no active hits.
4. `intent doctor` exits 0.
5. `tests/run_tests.sh` green.
6. CHANGELOG has the Removed line.

## Size and Estimate

- **Size**: XS (Extra Small). Single session.

## Exit Checklist

- [ ] Both files deleted.
- [ ] MODULES.md updated.
- [ ] No active references.
- [ ] CHANGELOG updated.
- [ ] Tests green.
- [ ] Committed.
