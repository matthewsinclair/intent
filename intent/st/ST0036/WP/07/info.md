---
verblock: "26 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-07
title: "Migration guide: intent/docs/migration-v2.10.0.md"
scope: Extra Small
status: Done
---

# WP-07: Migration guide -- `intent/docs/migration-v2.10.0.md`

## Objective

Write the user-facing migration guide for v2.9.0 -> v2.10.0. Covers the directory move (`.intent/` -> `intent/.config/`), what user-side scripts/aliases/CI need updating, recovery from interrupted migrations, and an FAQ. Becomes the canonical reference cited from WP01's recovery diagnostic and from CHANGELOG.

## Context

Intent has not previously shipped per-version migration guides. Per recon, `intent/docs/` contains tech notes (working-with-llms.md, rules.md, critics.md, …) but no `migration-*.md`. WP07 establishes the convention.

The WP01 migration function emits a recovery diagnostic that points at `intent/docs/migration-v2.10.0.md#recovery-from-interrupted-migration` when the sentinel is present. WP07 must own this anchor exactly.

The audience is Intent users running `intent upgrade` from v2.9.0:

- Most read the CHANGELOG, see "breaking change", click through to this guide.
- A small subset hits the recovery path (sentinel detected) and lands directly on the recovery section.
- Power users who script against `.intent/...` paths read the user-side action list to know what to update.

## Deliverables

1. **`intent/docs/migration-v2.10.0.md`** (~150-250 lines, sectioned as below).
2. **Cross-references** added to:
   - `CHANGELOG.md` v2.10.0 entry: link to this guide from the breaking-changes block (if WP03 did not already add).
   - `AGENTS.md` "Additional Resources" section: link.
   - `intent/docs/working-with-llms.md`: brief mention in the "v2.10.0 changes" subsection (if such a subsection exists; create if not).
   - `README.md`: short mention in upgrade-path prose.
3. **Anchor `recovery-from-interrupted-migration`** present and stable -- WP01's diagnostic text references it exactly.

## Document outline

```
# Intent v2.9.0 -> v2.10.0 Migration Guide

## Summary
One paragraph: what changed, why bundled with ST0035 LLM canon, how to upgrade.

## What moved
Path-mapping table:

| Before (v2.9.0)        | After (v2.10.0)              | Notes              |
| ---------------------- | ---------------------------- | ------------------ |
| .intent/config.json    | intent/.config/config.json   | Project metadata   |
| .intent/cache/         | intent/.config/cache/        | Always-gitignored  |
| .intent/backup/        | intent/.config/backup/       | Always-gitignored  |
| .intent/<custom>       | intent/.config/<custom>      | Whole-tree move    |

What did NOT move:
- ~/.intent/ext/ (user-level extension root) -- unchanged.
- .intent_critic.yml (file at project root) -- unchanged.

## How to upgrade
Three commands:
1. `cd <project>`
2. `intent upgrade` (runs migrate_v2_9_0_to_v2_10_0; performs the move atomically).
3. Verify: `intent doctor` should report clean.

## What to update on your side
- Shell aliases referencing .intent/... -> flip to intent/.config/...
- CI pipelines that read .intent/config.json -> flip path.
- Editor plugins that probe .intent/ -> flip probe.
- Custom scripts -> flip.

## Recovery from interrupted migration {#recovery-from-interrupted-migration}
If `intent upgrade` failed mid-relocation, you may see a sentinel file at
`intent/.config/.migration-in-progress`. Steps to recover:

1. Inspect: `ls intent/.config/`. If both `config.json` and the sentinel
   are present, the move succeeded but the stamp didn't.
2. If `intent/.config/config.json` exists with no version stamp (or a
   stale stamp): manually update via `jq` to `2.10.0`, remove sentinel,
   re-run `intent upgrade` (idempotent at this point).
3. If `intent/.config/` is incomplete (missing files that should be
   there): restore from the previous backup (your last commit, or
   `intent/.config/backup/` if that survived).
4. If `.intent/` and `intent/.config/` both exist: the migration aborted
   safely before `mv`. Inspect each, decide which is canonical, remove
   the other, re-run.

## FAQ
- Why move?
- Does this affect `~/.intent/ext/`?
- What if I have custom files under .intent/?
- Will my `.git/hooks/pre-commit` keep working?
- Is rollback possible?

## See also
- CHANGELOG.md (v2.10.0 entry).
- ST0036 design doc (`intent/st/ST0036/design.md` if open-source) -- decision rationale.
- ST0035 (canonical LLM config -- bundled with this release).
```

## Approach

1. Draft the doc following the outline. Keep prose tight; tables for path mappings.
2. The `## Recovery` section anchor MUST be `{#recovery-from-interrupted-migration}` -- WP01 hard-codes that anchor in its diagnostic.
3. Write the FAQ from anticipated questions (most asked: the four listed in the outline).
4. Add cross-refs to CHANGELOG, AGENTS.md, working-with-llms.md, README.md.
5. Read end-to-end; confirm clarity for the three audiences (CHANGELOG-clickers, recovery-needers, script-updaters).

## Acceptance Criteria

- [ ] `intent/docs/migration-v2.10.0.md` exists.
- [ ] Sections present: Summary, What moved (with table), How to upgrade, What to update on your side, Recovery (with anchor), FAQ, See also.
- [ ] Anchor `{#recovery-from-interrupted-migration}` present and exactly matches WP01's diagnostic reference.
- [ ] Cross-references in CHANGELOG, AGENTS.md, working-with-llms.md, README.md.
- [ ] FAQ answers four anticipated questions with concrete code examples where relevant.

### Tests to add / update

- None. Doc only.

## Dependencies

- **Blocks**: WP08 (Intent self-apply -- README/CHANGELOG visibility expects the guide to exist).
- **Blocked by**: WP01 (the recovery section describes WP01's behaviour and must reference the function's actual error semantics).

## Implementation Notes

- Tone: practical, terse, code-first. Mirror the existing style in `intent/docs/working-with-llms.md` (which was written during ST0035/WP03).
- The path-mapping table should be exhaustive -- all known directory contents, not just the obvious ones.
- The recovery section is the most critical: a user landing here is in a degraded state and needs unambiguous instructions. Numbered steps; explicit commands.
- The FAQ pre-empts the most common downstream questions. Better to over-cover than to under-cover.
- The `~/.intent/ext/` mention in "What did NOT move" is important -- preempts a common confusion.
- Cross-link to ST0036's `design.md` only if the steel-thread docs are public-facing in the user's repo. Most projects keep `intent/st/` private to authors. Spell out the link's purpose when including.

## Risks and Edge Cases

- **Anchor drift**: if WP01 changes its diagnostic text after WP07 lands, the anchor pointer breaks. Mitigation: WP01 hard-codes the anchor in code; if either changes, both must.
- **README.md style**: Intent's README.md is marketing-flavoured; the migration mention should be a one-liner, not a section. Don't bloat the README.
- **CHANGELOG format**: Intent's CHANGELOG follows Keep-a-Changelog; the breaking-changes entry was drafted during ST0035 retarget. Verify it's not double-touched (WP03 may have added it; WP07 just adds the link).
- **FAQ rot**: as v2.10.0 ages, the FAQ becomes stale. Acceptable -- migration guides are version-specific docs; this one is fossilised on purpose.

## Verification Steps

1. `cat intent/docs/migration-v2.10.0.md` -- read end-to-end.
2. `grep -n recovery-from-interrupted-migration` -- anchor present.
3. Cross-refs resolve: open CHANGELOG, search for the link; same for AGENTS.md, working-with-llms.md, README.md.
4. WP01's diagnostic text references the same anchor (cross-check after WP01 lands).

## Size and Estimate

- **Size**: XS (Extra Small). One sitting.

## Exit Checklist

- [ ] migration-v2.10.0.md exists with all sections.
- [ ] Recovery anchor in place.
- [ ] Cross-refs added in all four places.
- [ ] End-to-end read passed.
- [ ] Committed: `docs: ST0036/WP-07 v2.10.0 migration guide`.
