---
verblock: "26 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-03
title: "Literal sweep: replace .intent/ with intent/.config/"
scope: Medium
status: Not Started
---

# WP-03: Literal sweep -- replace `.intent/` with `intent/.config/`

## Objective

Sweep every prose, doc, plugin, and template literal mention of `.intent/` (per-project config dir) across the non-`bin/` tree and flip to `intent/.config/`. Guard against false positives: `~/.intent/ext/` (the user-level extension root, unaffected by this ST), `.intent_critic.yml` (file at project root, not in `.intent/`), and any references describing pre-v2.10.0 history that should remain truthful.

## Context

Recon (`grep -rn '\.intent/' intent/plugins/ intent/docs/ intent/usr/ lib/templates/` minus the `intent/.treeindex/` and `.intent_critic` filters) returned 221 hits across:

- `intent/plugins/` -- 43 hits. Plugin code, docs, prompts. Most are within `intent/plugins/claude/` (subagents, skills) describing where project state lives.
- `intent/docs/` -- 29 hits. Tech notes (working-with-llms.md, rules.md, critics.md, writing-extensions.md, pre-commit-hook.md, …). Both prose and code blocks.
- `intent/usr/` -- 15 hits. User-facing guides (user_guide, reference_guide, deployment_guide). Pre-v2.9.0 docs; some may need broader updates than just path flips (out of scope -- WP18 in ST0035 handles the `intent/usr/` audit).
- `lib/templates/` -- 10 hits. Template strings emitted by generators. Owned by WP04.

WP03 covers the first three (plugins, docs, usr) and explicitly **defers `lib/templates/`** to WP04 (which has additional generator-output verification). The `intent/usr/` audit conflict with ST0035/WP18: WP03 here flips the literal `.intent/` paths (mechanical correctness) but does NOT rewrite the user-guide narrative for v2.10.0 surface area (that is WP18's territory).

`bin/` is owned by WP02. Tests are owned by WP05. Templates are owned by WP04.

## Deliverables

1. **`intent/plugins/` flip**: every `.intent/` literal that refers to per-project config flipped. False positives preserved: `~/.intent/ext/` matches in extension-system docs, prompt examples that reference `.intent_critic.yml`, prose mentioning the v2.9.0-and-earlier layout in historical context.
2. **`intent/docs/` flip**: every `.intent/` literal in tech notes flipped. Where prose explains structure (e.g., "configuration lives at `.intent/config.json`"), update both the path AND any structural commentary that becomes inaccurate.
3. **`intent/usr/` flip (mechanical only)**: literal path flips. **Do not** rewrite v2.10.0 surface area in user docs -- that is WP18's scope.
4. **`CHANGELOG.md`** entry under v2.10.0 with a `### Breaking changes` heading explicitly calling out the `.intent/` -> `intent/.config/` move and pointing at `intent/docs/migration-v2.10.0.md` (created in WP07).
5. **Cross-reference audit**: any link to `.intent/` paths from doc to doc resolved post-flip.

## Approach

1. Generate the full hit list:
   ```bash
   grep -rnE '(^|[^.~/])\.intent/' intent/plugins/ intent/docs/ intent/usr/
   ```
   The leading anchor excludes `~/.intent/` and `../.intent/` patterns that the broader grep over-includes.
2. Categorise each hit into one of:
   - **Path probe in code**: flip.
   - **Doc prose describing per-project config**: flip and review surrounding sentence for structural drift.
   - **`~/.intent/ext/`**: keep (user-level extension root; orthogonal to this ST).
   - **`.intent_critic.yml`**: keep (file at project root; not under `.intent/` dir).
   - **Historical note**: keep with prose update if needed (e.g., "pre-v2.10.0 projects had `.intent/`; v2.10.0+ uses `intent/.config/`").
3. Apply edits in batches by directory (plugins, docs, usr). For files with many hits, use a single multi-edit pass.
4. After each batch: spot-check 3-4 random hits per directory for context correctness.
5. Write the `CHANGELOG.md` v2.10.0 breaking-changes entry.
6. Cross-reference audit: in updated docs, ensure every link to a moved path resolves to the new path.

## Acceptance Criteria

- [ ] `grep -rnE '(^|[^.~/])\.intent/' intent/plugins/ intent/docs/ intent/usr/` returns only intentional matches (extension system, `.intent_critic.yml`, historical notes -- each documented with a comment or surrounding prose making the intent clear).
- [ ] `grep -rn 'intent/\.config/' intent/plugins/ intent/docs/ intent/usr/` shows the flipped surface; counts roughly equal the pre-flip count of `.intent/` minus the false-positive set.
- [ ] `CHANGELOG.md` v2.10.0 entry includes `### Breaking changes` mentioning the move + link to migration guide.
- [ ] Spot-check pass: 5 random doc files read end-to-end (e.g., working-with-llms.md, rules.md, user_guide.md, a plugins README, an agent.md) -- prose flows naturally.
- [ ] No broken cross-doc links to old paths.

### Tests to add

- None directly. WP05 owns BATS coverage of code-path flips.

### Tests to update

- None at WP03 layer. The `tests/` literal sweep is WP05.

## Dependencies

- **Blocks**: WP04 (templates inherit prose conventions established here), WP05 (BATS sweep can use the same categorisation rules), WP06 (ignore patterns reference the new path).
- **Blocked by**: WP02 (path probes flipped in `bin/` first; some docs reference `bin/` behaviour).

## Implementation Notes

- The grep anchor `(^|[^.~/])\.intent/` excludes the common false-positives at the regex level. Easier than post-filtering.
- For files with many hits, prefer one focused multi-edit per file over multiple small edits -- reduces churn.
- `intent/usr/` files are pre-v2.9.0 in tone. WP03's discipline: flip the path literal, leave the surrounding prose alone unless it literally lies post-flip. WP18 (ST0035) handles the broader audit.
- `CHANGELOG.md` v2.10.0 entry already exists (drafted during ST0035 retarget per design D5); WP03 adds the breaking-changes subsection if not present.
- The `~/.intent/ext/` extension root is **always** at the user level and never affected by this ST. Any tooling that resolves `~/.intent/ext/<name>/` continues to work.

## Risks and Edge Cases

- **Regex misses**: `(^|[^.~/])` is good but not bulletproof; e.g., `\\.intent/` (escaped dot in a code example). Spot-check final grep with `[^.~/]` and broader.
- **Markdown code blocks**: paths inside fenced ``` blocks flip same as prose. Confirm.
- **Plugin command examples**: prompts and SKILL.md files often show example commands referencing `.intent/`. These flip too, but watch for examples that intentionally show pre-v2.10.0 behaviour (rare).
- **Doc paragraphs that explain structure** can become subtly wrong after flipping the path. Read each paragraph before/after.
- **Translated/external content**: not applicable to Intent today (English-only repo, no external mirrors).

## Verification Steps

1. Final grep with strict regex: `grep -rnE '(^|[^.~/])\.intent/' intent/plugins/ intent/docs/ intent/usr/` -- review every remaining hit.
2. Count check: pre-flip count of in-scope hits vs post-flip count of `intent/.config/` should match (modulo false-positive removals and historical-note rewrites).
3. End-to-end re-read of: `intent/docs/working-with-llms.md`, `intent/docs/rules.md`, `intent/docs/critics.md`, `intent/docs/writing-extensions.md`, one `intent/usr/*.md`, two random `intent/plugins/.../{SKILL,agent}.md`.
4. `CHANGELOG.md` -- v2.10.0 breaking-changes entry present, links resolve.

## Size and Estimate

- **Size**: M (Medium). 1 session, possibly 2 if `intent/usr/` flips uncover unexpected structural drift.

## Exit Checklist

- [ ] All in-scope `.intent/` literals flipped.
- [ ] Intentional preservations (extension root, `.intent_critic.yml`, historical notes) documented.
- [ ] CHANGELOG breaking-changes entry written.
- [ ] Cross-doc links resolve post-flip.
- [ ] Spot-check pass on 5 random files.
- [ ] Committed: `refactor: ST0036/WP-03 literal sweep .intent to intent/.config across plugins, docs, usr`.
