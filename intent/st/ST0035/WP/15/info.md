---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-15
title: "Canary rollout to Conflab, Lamplight, Laksa"
scope: Medium
status: Not Started
---

# WP-15: Canary rollout to Conflab, Lamplight, Laksa

> **Coordination note (ST0036/WP-09)**: this canary now carries both ST0035 (LLM canon) and ST0036 (directory relocation) concerns. `intent upgrade` invokes `migrate_v2_9_0_to_v2_10_0` which performs the relocation + canon-apply in a single pass. Verification has 12 points (10 ST0035 + 2 ST0036). See `intent/st/ST0036/impl.md` for the bundled-release rationale.

## Objective

Apply ST0035 canon to three canary projects — Conflab, Lamplight, Laksa — before sweeping the rest of the fleet. These three are all full-spectrum Elixir projects with existing `intent/llm/` content and (in Conflab's case) pre-installed `.claude/skills/`. They exercise every canon artefact; if they come through clean, the fleet sweep (WP16) is low-risk.

## Context

Canary discipline (from ST0034's rollout pattern): apply to a small representative subset, verify, then sweep. The three chosen projects have different characteristics:

- **Conflab** — Elixir, full `intent/llm/` (MODULES.md + DECISION_TREE.md + AGENTS.md), 2 installed skills in `.claude/skills/`. Tests preservation of installed skills through the upgrade.
- **Lamplight** — Elixir, full `intent/llm/`, ~20 deps usage-rules.md files (heaviest `mix usage_rules.sync` interaction). Tests Elixir ecosystem compatibility.
- **Laksa** — Elixir, full `intent/llm/`, contains Sites as a subdir (which is excluded from rollout). Tests the subdir handling.

If any canary project reveals an issue, fix it in the canon (earlier WPs), re-apply to Intent (WP14), re-canary. No fleet sweep until canary is clean three-for-three.

## Deliverables

1. **Canon applied to all 3 canary projects** with a coherent commit each:
   - `cd ~/Devel/prj/Conflab && intent upgrade && intent claude upgrade --apply` → commit.
   - `cd ~/Devel/prj/Lamplight && intent upgrade && intent claude upgrade --apply` → commit.
   - `cd ~/Devel/prj/Laksa && intent upgrade && intent claude upgrade --apply` → commit.
2. **Per-project verification report** in `intent/st/ST0035/WP/15/canary-reports/<project>.md` with:
   - Dry-run output.
   - Apply output.
   - 12-point checklist results.
   - Any issues encountered.
   - Outcome: pass / fix-required / blocked.
3. **Aggregate canary report** at `intent/st/ST0035/WP/15/canary-summary.md` with findings across all three.
4. **Issue tickets** filed for any bugs discovered in the canon (may not be any).
5. **Decision to proceed / halt** on fleet sweep (WP16) — clearly documented.

## Approach

### For each of the three projects:

1. Pull latest from remote (`git pull`).
2. Ensure clean working tree.
3. `intent doctor` — baseline clean.
4. `intent upgrade --dry-run` — view plan.
5. `intent claude upgrade --dry-run` — view canon plan.
6. `intent upgrade --apply` — execute stamp bump.
7. `intent claude upgrade --apply` — execute canon apply.
8. Review `git diff` / `git status`.
9. Run 12-point verification:
   - config.json at 2.10.0 (in `intent/.config/config.json`, post-ST0036 location).
   - Root AGENTS.md real file, not symlink.
   - intent/llm/AGENTS.md absent.
   - Root usage-rules.md present.
   - .claude/settings.json hooks present.
   - .git/hooks/pre-commit executable.
   - .intent_critic.yml present.
   - SessionStart reminder observed in a Claude Code session.
   - Pre-commit blocks on staged violation.
   - `intent critic <lang>` produces report.
   - **ST0036 (11)**: `[ -d intent/.config ]` -- new layout present.
   - **ST0036 (12)**: `[ ! -d .intent ]` -- legacy directory absent (no leftover).
10. For Conflab specifically: verify pre-existing `.claude/skills/` installs survive the upgrade.
11. For Lamplight specifically: verify `mix usage_rules.sync` (if run) still produces correct output and doesn't conflict with Intent's refreshed root `usage-rules.md`.
12. For Laksa specifically: verify Sites subdir is untouched.
13. Commit: `chore: apply ST0035 + ST0036 canon (v2.10.0 rollout canary)`.
14. Push to `local` remote (Dropbox) as per project convention.
15. Document in canary report.
16. Return to Intent repo and update summary.

### Cross-project

- If issue found in Conflab, pause. Investigate. Fix canon in Intent. Re-canary.
- If all three pass clean, proceed to WP16.

## Acceptance Criteria

- [ ] All three projects have `intent_version: 2.10.0` in `intent/.config/config.json` (post-ST0036 location).
- [ ] All three projects have no leftover `.intent/` directory after upgrade.
- [ ] All three projects pass the 12-point verification checklist (10 from ST0035 + 2 from ST0036).
- [ ] All three projects have their changes committed and pushed.
- [ ] Per-project reports written to `intent/st/ST0035/WP/15/canary-reports/<project>.md`.
- [ ] Aggregate summary at `intent/st/ST0035/WP/15/canary-summary.md`.
- [ ] Zero rollbacks during canary.
- [ ] Conflab's pre-existing `.claude/skills/` installs survive the upgrade (verified by listing before and after).
- [ ] Lamplight's `mix usage_rules.sync` (if run) produces a sane AGENTS.md with both Intent's and deps' rules visible.
- [ ] Laksa's Sites subdir is unchanged.
- [ ] Any issues discovered are either (a) fixed in the canon before WP15 closes, or (b) filed as follow-up tickets with explicit approval to proceed.
- [ ] Commit messages follow Intent conventions, no Claude attribution.

### Tests to add

None.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP14 (Intent self-apply must be clean).
- **Blocks**: WP16 (fleet).

## Implementation Notes

- **Commit discipline per project**: each canary project gets its own commit in its own repo. Don't batch.
- **Push to `local` remote**: per user's convention (Dropbox remote is `local`). Don't push to `upstream` (GitHub) from the canary — that's deferred to the fleet sweep post-verification.
- **Observation over measurement**: some canary checks (SessionStart reminder injection) require human observation. Document as "observed" with a brief note.
- **Conflab skills preservation**: list `.claude/skills/` before, after. Diff. Should be identical set of installed skills.
- **Lamplight mix.exs interaction**: `mix usage_rules.sync` is an Elixir tool that reads `deps/*/usage-rules.md` and generates content. Intent's root `usage-rules.md` is hand-authored. They coexist — don't conflict. Verify by running both tools and inspecting output.
- **Laksa Sites subdir**: `~/Devel/prj/Laksa/Sites/` (or similar) is a subdirectory that should not be touched by `intent claude upgrade --apply`. Confirm post-apply that the Sites dir is unchanged.

## Risks and Edge Cases

- **Risk**: Canon apply breaks one of the canary projects. **Mitigation**: git reset available; no pushes to upstream until verified.
- **Risk**: Conflab's installed skills are wiped by the upgrade. **Mitigation**: WP11 should preserve `.claude/skills/` contents; test explicitly.
- **Risk**: Lamplight's `mix usage_rules.sync` emits conflicting AGENTS.md. **Mitigation**: Intent's root AGENTS.md is written by `intent agents sync`; `mix usage_rules.sync` writes to a separate path by default. Confirm they don't collide.
- **Risk**: Laksa's Sites subdir gets touched. **Mitigation**: `intent claude upgrade --apply` operates on the project root; subdirs are not in scope unless explicitly named.
- **Edge**: Canary project has uncommitted work. **Mitigation**: fail the canary for that project; ask user to commit/stash before retrying.

## Verification Steps

See acceptance criteria. Each project produces its own report; the aggregate summary rolls them up.

## Size and Estimate

- **Size**: M (Medium). 2–3 sessions.
- Session 1: Conflab canary; first real-world exercise of canon. Document carefully.
- Session 2: Lamplight canary; Elixir ecosystem stress-test.
- Session 3: Laksa canary + aggregate summary + decision to proceed to WP16.

## Exit Checklist

- [ ] All three canary reports complete.
- [ ] Aggregate summary documents outcomes.
- [ ] Zero unresolved issues.
- [ ] Decision to proceed to WP16 explicitly noted.
- [ ] All three project commits pushed to `local`.
