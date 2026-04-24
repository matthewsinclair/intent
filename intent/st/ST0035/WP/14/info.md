---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-14
title: "Self-apply canon to Intent repo (dogfood)"
scope: Small
status: Not Started
---

# WP-14: Self-apply canon to Intent repo (dogfood)

## Objective

Run `intent claude upgrade --apply` against Intent's own repo. This is the first real-world use of WP11's upgrade logic and the authoritative dogfood of the entire canon stack. Outcome: Intent itself has the complete canon set (AGENTS.md at root, CLAUDE.md overlay, usage-rules.md refreshed, .claude/settings.json + hooks, .git/hooks/pre-commit, .intent_critic.yml, intent/llm/ without AGENTS.md). Intent is the reference example for every downstream project.

## Context

Before the canary (WP15) or fleet rollout (WP16), Intent must apply its own canon. If the self-apply reveals issues, those are fixed in Intent itself before any fleet project is touched.

This WP executes, not authors. All templates/generators exist (WP01–WP11). WP14 runs them end-to-end on Intent and verifies the result.

## Deliverables

1. **Self-applied canon state** in Intent's repo:
   - Root `AGENTS.md` — real file, enriched content, no symlink.
   - Root `CLAUDE.md` — Claude-specific overlay (from WP13).
   - Root `usage-rules.md` — refreshed (from WP02).
   - `.claude/settings.json` — SessionStart + Stop hooks wired.
   - `.claude/scripts/session-context.sh` — executable.
   - `.git/hooks/pre-commit` — executable, invokes `bin/intent_critic`.
   - `.intent_critic.yml` — default config.
   - `intent/llm/MODULES.md` — present and up to date.
   - `intent/llm/DECISION_TREE.md` — present.
   - `intent/llm/AGENTS.md` — **absent** (deleted via WP10 logic).
   - `.intent/config.json` — `intent_version: 2.9.1`.
2. **Dry-run report** (pre-apply): capture `intent claude upgrade --dry-run` output to `intent/st/ST0035/WP/14/dry-run-report.md` as evidence.
3. **Apply report**: capture `intent claude upgrade --apply` stdout/stderr to `intent/st/ST0035/WP/14/apply-report.md`.
4. **Verification checklist run** against the 10-point per-project verification list in `design.md` — all green.
5. **Commit** all changes as a coherent set: `chore: apply ST0035 canon to Intent itself (v2.9.1 self-dogfood)`.

## Approach

1. Confirm WP01–WP13 are all complete and committed (this WP gates on all of them).
2. From a clean working tree:
   - `bin/intent doctor` — baseline clean state confirmation.
   - `bin/intent claude upgrade --dry-run` — capture output; review.
   - `bin/intent claude upgrade --apply` — capture output.
   - Review `git diff` / `git status` for the expected set of changes.
3. Run the 10-point verification:
   - `cat .intent/config.json | jq .intent_version` → `2.9.1`.
   - `test -f AGENTS.md && ! test -L AGENTS.md` → real file.
   - `test ! -f intent/llm/AGENTS.md` → retired.
   - `test -f usage-rules.md` → present.
   - `jq .hooks .claude/settings.json` → non-empty.
   - `test -x .git/hooks/pre-commit` → executable.
   - `test -f .intent_critic.yml` → present.
   - Open a scratch Claude Code session in Intent — verify SessionStart reminder fires.
   - Stage a known-bad file (craft a small violation), try to commit — confirm pre-commit blocks.
   - `bin/intent critic shell bin/intent_critic` — produces report.
4. Document any surprises in an as-built section.
5. Commit.

## Acceptance Criteria

- [ ] `bin/intent claude upgrade --dry-run` ran cleanly; output captured.
- [ ] `bin/intent claude upgrade --apply` ran cleanly; output captured.
- [ ] All 10 verification checks pass (checklist above).
- [ ] `bin/intent doctor` post-apply returns 0 with no warnings.
- [ ] `tests/run_tests.sh` passes.
- [ ] `intent/st/ST0035/WP/14/dry-run-report.md` exists.
- [ ] `intent/st/ST0035/WP/14/apply-report.md` exists.
- [ ] Changes committed as a single coherent commit.
- [ ] No regressions: Intent's own skills/subagents still work (`intent claude subagents list` clean, `intent claude skills list` clean).
- [ ] Second run (idempotence): `intent claude upgrade --apply` produces no changes.
- [ ] Commit follows Intent conventions.

### Tests to add

None new; this WP exercises existing tests.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP11 (upgrade logic), WP13 (Intent's own CLAUDE.md rewrite), and transitively all earlier WPs.
- **Blocks**: WP15 (canary).

## Implementation Notes

- **Safety-first**: this is a live apply against Intent. Keep the working tree clean before starting (no uncommitted work). Be prepared to `git reset --hard` and retry if something unexpected happens.
- **Scratch-branch option**: consider doing WP14 on a feature branch first (`git checkout -b canon-self-apply`), verifying, then merging to main. Reduces blast radius if anything goes sideways.
- **Manual verification of hooks**: the SessionStart reminder only fires in a real Claude Code session. Document the observation (screenshot-free, just "observed as expected") in the apply-report.
- **Known-bad file for pre-commit test**: stage an obvious rule violation (e.g., a bash file without `set -euo pipefail` if that's a critic rule). Easy to prepare and revert.
- **Idempotence check is critical**: if the second `--apply` reports changes, WP11 has a bug that must be fixed before WP15.

## Risks and Edge Cases

- **Risk**: Self-apply reveals a WP11 bug (e.g., the generator doesn't preserve user-marked sections correctly). **Mitigation**: fix in WP11, re-run WP14. Don't proceed to WP15 until clean.
- **Risk**: Intent's own CLAUDE.md (freshly rewritten in WP13) doesn't match what WP11's upgrade expects. **Mitigation**: WP13 dogfoods the WP09 template; they should match by construction.
- **Risk**: Pre-commit hook blocks Intent's own commits because Intent's bash has rule violations. **Mitigation**: expected and desired (from WP05's definition of done — Intent's own bash should be rule-clean). If violations surface, they're real and must be fixed — file tickets, don't disable rules.
- **Risk**: Self-apply corrupts Intent. **Mitigation**: work on a branch; never apply directly to main without verification.

## Verification Steps

See Acceptance Criteria — WP14 is itself a verification WP.

## Size and Estimate

- **Size**: S (Small). 1–2 sessions.
- Session 1: Run dry-run, apply, verification checks, document.
- Session 2: If issues found, fix in upstream WPs; re-run.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] Dry-run + apply reports captured.
- [ ] 10-point verification all green.
- [ ] Idempotence confirmed.
- [ ] Any discovered issues filed as follow-up tickets or fixed in upstream WPs before this WP closes.
- [ ] Committed.
