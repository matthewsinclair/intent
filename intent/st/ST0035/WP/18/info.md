---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail (late addition)"
wp_id: WP-18
title: "Review and update (or retire) intent/usr/*.md"
scope: Medium
status: Not Started
---

# WP-18: Review and update (or retire) intent/usr/\*.md

## Objective

Review the three user-facing docs under `intent/usr/` against the v2.9.1 canon and make a per-file keep / update / throw decision, applying the decision before the v2.9.1 release ships. No user doc should survive into v2.9.1 with a stale surface description (missing skills, missing critics, missing extensions, missing hooks).

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

### Review memo

_To be populated during audit (step 1). Tentative format:_

```
### user_guide.md (877 lines, 2026-04-12)
- Canon-current: §X, §Y paragraphs
- Canon-stale: §Z (references elixir subagent), §W (missing /in-* skills)
- Duplicated: §A overlaps README.md quick-start
- Unique + current: §B (onboarding narrative not in README)
Decision: UPDATE. Gaps: <list>.

### reference_guide.md (1370 lines, 2026-04-12)
...

### deployment_guide.md (619 lines, 2026-03-06)
...
```

### Why not just delete all three

The `README.md` + `AGENTS.md` + `usage-rules.md` + `intent/docs/working-with-llms.md` set covers a lot — but not everything. Specifically, the onboarding narrative (_"I just got Intent, what now?"_) and the deployment guide (_"how do I install Intent on a new machine?"_) don't have natural homes in the canon files. Throwing all three wholesale would create gaps. The per-file audit determines the right answer.

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
