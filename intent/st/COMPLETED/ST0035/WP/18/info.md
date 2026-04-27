---
verblock: "27 Apr 2026:v0.3: matts - retarget v2.9.1 -> v2.10.0; review memo populated; decision: throw all three"
wp_id: WP-18
title: "Review and update (or retire) intent/usr/*.md"
scope: Medium
status: Done
---

# WP-18: Review and update (or retire) intent/usr/\*.md

> **Scope as built (2026-04-27)**: all three files were retired wholesale. The 2026-04-24 spec assumed a per-file keep / update / throw audit was needed; in fact every file was 7 versions stale (frontmatter `intent_version: 2.6.0`) and substantially duplicated by the v2.9.0+ canon (`README.md`, `intent/docs/working-with-llms.md`, `intent help <cmd>`, `AGENTS.md`). Per the project fail-forward principle (no preservation, prune actively), the audit collapsed to a one-line decision: throw all three, redirect the surviving cross-refs to canon docs. See "Review memo" below.

## Objective

Review the three user-facing docs under `intent/usr/` against the v2.10.0 canon and make a per-file keep / update / throw decision, applying the decision before v2.10.0 ships. No user doc should survive into v2.10.0 with a stale surface description (missing skills, missing critics, missing extensions, missing hooks).

## Context

`intent/usr/` holds three hand-authored docs currently linked from `README.md`:

| File                             | Size       | Last touched | Purpose (original)                                    |
| -------------------------------- | ---------- | ------------ | ----------------------------------------------------- |
| `intent/usr/user_guide.md`       | 877 lines  | 2026-04-12   | Step-by-step guide for new users                      |
| `intent/usr/reference_guide.md`  | 1370 lines | 2026-04-12   | Complete command reference and detailed documentation |
| `intent/usr/deployment_guide.md` | 619 lines  | 2026-03-06   | How to install Intent, set up PATH, and get running   |

All three pre-date the v2.9.0 release (2026-04-23). They therefore pre-date:

- The `/in-*` skill family (23 skills shipped in ST0034/v2.9.0).
- The `critic-*` subagent family (5 critics shipped in ST0034).
- The rule library at `intent/plugins/claude/rules/` and the `intent claude rules` CLI (v2.9.0).
- The extension system at `~/.intent/ext/` and the `intent ext` CLI (v2.9.0).
- The deletion of the standalone `elixir` subagent and the relocation of `worker-bee` to the reference extension (v2.9.0).
- The v2.9.1 canon: three-file architecture (AGENTS.md at root as a real file), `.claude/settings.json` hooks, `bin/intent_critic` + `.git/hooks/pre-commit`, `intent/docs/working-with-llms.md`, cancelled-ST convention.

Because `intent/usr/user_guide.md` and `intent/usr/reference_guide.md` are both cross-referenced from `README.md`, they are the first-touch surface for new Intent users — the most visible place for staleness. `deployment_guide.md` is more isolated but may still reference retired installation steps.

`README.md` itself was partially refreshed in WP-03 (added "For LLM Collaboration" section). Intent's top-level doc posture post-v2.9.1 is:

- `README.md` — landing page.
- `intent/usr/*` — how a new user installs, onboards, and uses Intent (this WP's scope).
- `intent/docs/*` — in-depth authoring/rationale docs for Intent developers and advanced users (working-with-llms.md, rules.md, critics.md, writing-extensions.md).

If an `intent/usr/*.md` file's content is substantially duplicated by `intent/docs/*` or `README.md`, retiring it is preferable to maintaining two sources of truth (Highlander).

## Deliverables

1. **Per-file review memo** in this `info.md`'s "Implementation Notes" section after audit completes. For each of the three files, capture:
   - Decision: keep (no change), update (list of gaps), or throw (justification + redirect).
   - Gap list if updating: specific sections to add, rewrite, or delete, citing v2.9.1 canon features they miss.
   - Highlander check: content overlap with `README.md`, `AGENTS.md`, `usage-rules.md`, and `intent/docs/*`.
2. **Applied update** — refreshed content (if keep-with-update), deleted file (if throw), or confirmed-current flag (if keep-as-is).
3. **Cross-reference cleanup**:
   - `README.md`'s links to retired files removed or redirected.
   - Any `intent/docs/*` cross-reference to retired files fixed.
   - `AGENTS.md` footer (auto-generated) does not need manual fixing — `intent agents sync` handles it.
4. **CHANGELOG.md v2.9.1 entry** notes user-doc refresh / retirement as appropriate.
5. **MODULES.md** registration updated if any `intent/usr/` file is retired or renamed.

## Approach

### Step 1: Audit (read-only)

Read each file end-to-end. For each, tag by paragraph whether content is:

- **Canon-current** (accurate against v2.9.1).
- **Canon-stale** (references v2.8.x or pre-v2.9.0 state — mentions deleted `elixir` subagent, missing critics, missing `/in-*` skills, wrong AGENTS.md location, wrong `intent/llm/` structure).
- **Duplicated** (content is already in `README.md` / `usage-rules.md` / `intent/docs/*`).
- **Unique and current** (content that's nowhere else and still accurate).

Capture the tag distribution in the review memo.

### Step 2: Per-file decision

Apply the Highlander rule:

- **user_guide.md**: if >60% canon-stale, update in place (it's the new-user path, can't be retired without a replacement). If heavily duplicated by `README.md`'s quick-start + `intent/docs/working-with-llms.md` + `usage-rules.md`, throw and expand the survivors to cover the gap.
- **reference_guide.md**: if content is reproducible by `intent help <cmd>` + `AGENTS.md` + `README.md` command lists, throw (kill the duplication). Otherwise update.
- **deployment_guide.md**: likely keep (deployment is a domain not covered elsewhere), update for v2.9.1 installation surface including the `.claude/settings.json` + `.git/hooks/pre-commit` setup that `intent claude upgrade --apply` now performs.

These are defaults — the audit in step 1 drives the actual decisions.

### Step 3: Apply decision

- **Update**: rewrite stale sections, cite `intent/docs/working-with-llms.md` for narrative and `usage-rules.md` for rules. Keep the DO / NEVER structure intact if the doc already has one. Respect the 2-space-indentation rule for fenced code blocks. Never manually wrap lines.
- **Throw**: delete the file. Update `README.md` links. Add one sentence to the v2.9.1 CHANGELOG noting the retirement and redirecting readers.
- **Keep as-is**: verify with a grep pass that no stale surface slipped through; note confirmation in the review memo.

### Step 4: Verify

- `grep -rn "elixir subagent\|intent/llm/AGENTS" intent/usr/` returns nothing.
- `grep -rn "worker-bee" intent/usr/` returns only v2.9.0-aware references (relocation to `~/.intent/ext/`).
- `grep -n "intent/usr/" README.md` — all links resolve to files that still exist.
- `wc -l` per file — sanity-check length post-update; 20-30% smaller than the audit start is healthy.

### Step 5: Commit and close

- One commit per file if the diffs are large (`docs: refresh intent/usr/user_guide.md to v2.9.1`), or a single commit if changes are minimal.
- Mark WP-18 Done via `intent wp done ST0035/18`.

## Acceptance Criteria

- [ ] Review memo in Implementation Notes section captures per-file keep / update / throw decision with rationale.
- [ ] Each `intent/usr/*.md` file is either refreshed, deleted, or explicitly confirmed current.
- [ ] No `intent/usr/*.md` file references the deleted `elixir` subagent, the retired `intent/llm/AGENTS.md` path, or pre-v2.9.0 worker-bee state.
- [ ] `README.md` cross-references to `intent/usr/*.md` all resolve to files that still exist post-WP.
- [ ] Any retired file is reflected in `intent/llm/MODULES.md` (removed) and `CHANGELOG.md` v2.9.1 entry (noted).
- [ ] Any refreshed file cross-references `intent/docs/working-with-llms.md` where appropriate (to avoid content duplication — Highlander).
- [ ] Markdown linter clean on any updated file.
- [ ] No stale code examples — every `intent ...` command invocation shown in a user doc actually runs at v2.9.1.

### Tests to add

- None directly (prose docs).
- Consider: add an `intent/usr` presence check to `tests/unit/docs_completeness.bats` if that test exists, otherwise skip.

### Tests to update

- None.

## Dependencies

- **Blocked by**: WP-03 (needs `intent/docs/working-with-llms.md` for cross-reference targets; needed for Highlander-check against `usage-rules.md` and `working-with-llms.md`).
- **Soft-blocked by**: WP-14 (Intent self-dogfood). Ideal ordering is that WP-18's updates describe Intent in its as-dogfooded v2.9.1 state. If WP-14 hasn't landed yet, the audit can still run and tentative decisions can be made; final updates land post-WP-14.
- **Blocks**: WP-17 (verification sweep should confirm no stale user docs ship in v2.9.1).

## Implementation Notes

### Review memo (final)

All three files share the same disposition: **throw**. Frontmatter on all three reads `intent_version: 2.6.0` (2026-03-05), seven minor versions behind the v2.10.0 canon. Per-file rationale:

**user_guide.md** (877 lines, frontmatter 2026-03-05). Tag distribution: ~70% canon-stale (predates `/in-*` skills, `critic-*` subagents, rule library, extension system, `.claude/settings.json` hooks, `.git/hooks/pre-commit`, three-file root canon, `intent/.config/` directory layout); ~25% duplicated by `README.md` quick-start + `intent/docs/working-with-llms.md` narrative + `intent help <cmd>` + `usage-rules.md` DO/NEVER list; ~5% unique-but-still-stale (the "I just got Intent, what now?" onboarding narrative — replaced by README's quick-start which is now adequate). Decision: **THROW**. Replace the README link with a pointer to `intent/docs/working-with-llms.md` (canon narrative) + `intent help` (commands).

**reference_guide.md** (1370 lines, frontmatter 2026-03-05). Tag distribution: ~80% canon-stale (every command's flags/output drifted; the `intent upgrade` example shows a backup-dir flag that no longer exists; the directory tree shows the `intent/eng/` subtree which is largely empty in v2.10.0 projects); ~15% duplicated by `intent help <cmd>` (the canonical command reference); ~5% unique (architectural concepts, but those are now in `intent/docs/working-with-llms.md` D1-D10 and `intent/llm/MODULES.md`). Decision: **THROW**. Replace the README link with a pointer to `intent help` + `AGENTS.md`.

**deployment_guide.md** (619 lines, frontmatter 2026-03-05). Tag distribution: ~60% canon-stale (the `~/intent` clone path predates the v2.10.0 install convention, the per-project alias example is outdated, the plugin/subagent deployment section predates the canon-installer); ~30% duplicated (installation is in README's quick-start; upgrade is in `intent/docs/migration-v2.10.0.md`); ~10% unique (some integration prose for CI environments — but the CI story is genuinely thin and would need a fresh write, not an update). Decision: **THROW**. The README quick-start + `intent/docs/migration-v2.10.0.md` cover the actual canonical install path.

### Cross-reference cleanup applied

- `README.md`: Documentation section (lines 188-192) and Getting Help section (line 447) — replaced four `intent/usr/*` links with pointers to `intent/docs/working-with-llms.md`, `intent help`, and `intent/docs/migration-v2.10.0.md`.
- `docs/blog/0005-getting-started-with-intent.md`: line 490 ("Reference Guide" pointer) — replaced with `intent help`.
- `intent/docs/migration-v2.10.0.md`: line 40 ("intent/usr/" in the unchanged-subdirs list) — removed from the list; the directory itself is gone.
- `CHANGELOG.md` v2.10.0 entry — added a Removed bullet noting the retirement and the redirect targets.
- `intent/llm/MODULES.md`: no entries pointed at `intent/usr/*` (hand-authored docs were never registered as modules); no edit needed.
- Backup files at `.backup/backup-*/intent/usr/*` are not in source control (`.backup/` is gitignored); no action.
- Historical references in `CHANGELOG.md` (pre-v2.10.0 entries) and `DEPRECATIONS.md` left intact — they document the former state, which is correct.

## Risks and Edge Cases

- **Risk**: update churn drags WP into the next release. **Mitigation**: audit-first, decision-second posture keeps scope tight. A "throw" decision is cheaper than an "update" one.
- **Risk**: updating leaves a gap when a new canon feature (say, a future WP adding a sub-command) is not yet in the user doc. **Mitigation**: refresh to as-at-v2.9.1 canon only; explicitly mark later-to-be-added features as out of scope.
- **Risk**: Highlander violation if an updated user doc re-duplicates content from `working-with-llms.md` or `usage-rules.md`. **Mitigation**: cross-reference aggressively; cite instead of copy.
- **Risk**: the `reference_guide.md`'s command listing drifts from `intent help <cmd>`'s output. **Mitigation**: consider retiring the command reference content and linking to `intent help` output, or auto-generating it (follow-on WP if non-trivial).

## Verification Steps

1. `wc -l intent/usr/*.md` — files updated or deleted; note pre/post line counts in review memo.
2. `grep -rn "elixir subagent\|intent/llm/AGENTS\|_llm_preamble" intent/usr/` returns nothing.
3. `grep -rn "intent/usr/" README.md` — all links resolve.
4. Read updated files end-to-end; confirm no placeholder text.
5. `intent doctor` passes unchanged.

## Size and Estimate

- **Size**: M (Medium). Audit is 1–2 sessions; updates depend on outcome of audit.
  - Session 1: Audit all three files; produce review memo; per-file decisions.
  - Session 2+: Apply decisions (update content or delete), test links, commit.
- Could shrink to S if all three are thrown or all confirmed current. Could stretch to L if all three need substantial rewrites.

## Exit Checklist

- [ ] Review memo finalised in Implementation Notes.
- [ ] Each file: kept, updated, or thrown — decision applied.
- [ ] Cross-references in `README.md` and elsewhere validated.
- [ ] `CHANGELOG.md` v2.9.1 entry reflects user-doc state.
- [ ] `MODULES.md` reflects any retirements.
- [ ] WP committed, tests green, WP-18 marked Done.
