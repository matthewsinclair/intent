---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-11
title: "Release and fleet upgrade"
scope: Medium
status: Not Started
---

# WP-11: Release and fleet upgrade

## Objective

Finalise Intent v2.9.0 release artifacts (`VERSION`, `CHANGELOG.md`, release notes, git tag, GH release) and upgrade all 16 fleet projects to v2.9.0 in three batches (canary + batch 2 + batch 3) with halt-on-error discipline. Update `intent/wip.md` and `.claude/restart.md` with the release confirmation.

## Context

This is the gate WP. Every prior WP must be green before WP11 runs. The fleet upgrade is the post-release acceptance: if any canary project fails `intent doctor` or falls out of version alignment, the release is suspect and rollback (via v2.9.1 fix release) is preferred over re-tagging.

Intent's release discipline, refined through v2.5.0 - v2.8.2, includes:

- Double remote push (local Dropbox + upstream GitHub)
- Force-push tags with care; prefer new version over re-tag
- No vanity metrics in release notes / CHANGELOG
- No Claude attribution in commits
- Canary batch before full fleet rollout
- Session wrap: update `wip.md` and `restart.md`

v2.8.2 had a slipstream (tag force-moved once). R8 risk mitigation says prefer v2.9.1 over repeating the slipstream.

## Deliverables

### Release artifacts

- `VERSION` file updated to `2.9.0`
- `CHANGELOG.md` — v2.9.0 entry finalised (from WP10 draft)
- `docs/releases/2.9.0/RELEASE_NOTES.md` — finalised (from WP10 draft)
- Release commit: `Release v2.9.0 -- Agentic Software Engineering Suite (ST0034)` (no Claude attribution)
- Tag `v2.9.0` on both remotes (local, upstream)
- GH release synced via `gh release create v2.9.0 --notes-file ...`

### Fleet upgrade

16 projects upgraded to v2.9.0 in three batches:

**Canary batch (5 projects)**: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab

**Batch 2 (6 projects)**: Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts

**Batch 3 (4 projects)**: Multiplyer, Prolix, Utilz, Courses/Agentic Coding

A3/\* skipped per existing user direction.

### Post-release updates

- `intent/wip.md` — updated to "Intent v2.9.0. No active steel threads. Clean working tree." plus Recent entry for 2.9.0
- `.claude/restart.md` — updated with v2.9.0 summary and fleet-upgrade confirmation

## Approach

### Pre-release checks

1. Confirm every WP01-WP10 exit checklist is green.
2. `git status` clean; `git log --oneline -10` shows expected WP commits.
3. `tests/run_tests.sh` all green.
4. `intent doctor` clean on Intent repo.
5. `intent claude subagents list` shows the new critic-\* family and no elixir / canon worker-bee.
6. `intent claude rules validate` passes (all rules).
7. `intent ext validate` passes on the worker-bee seed.
8. Canary dry-run (WP09) completed without issues.

### Release commit

1. Edit `VERSION` to `2.9.0`.
2. Finalise `CHANGELOG.md` v2.9.0 entry (date-stamp it today).
3. Finalise `docs/releases/2.9.0/RELEASE_NOTES.md`.
4. Update `intent/wip.md` and `.claude/restart.md` with Pre-release state.
5. Stage: `git add VERSION CHANGELOG.md docs/releases/2.9.0/ intent/wip.md .claude/restart.md` plus any content WP changes still staged.
6. Commit: `git commit -m "Release v2.9.0 -- Agentic Software Engineering Suite (ST0034)"` — no Co-Authored-By, no Claude attribution.
7. Verify: `git log -1 --format='%B'` shows commit message; `git show HEAD --stat` shows expected file set.

### Tag and push

1. `git tag -f v2.9.0 HEAD`.
2. Push main: `git push local main` and `git push upstream main`.
3. Push tag: `git push -f local v2.9.0` and `git push -f upstream v2.9.0`.
4. Verify tag on both remotes: `git ls-remote --tags local v2.9.0` and `git ls-remote --tags upstream v2.9.0`.

### GH release

1. `gh release create v2.9.0 --notes-file docs/releases/2.9.0/RELEASE_NOTES.md` (if release doesn't exist)
2. OR `gh release edit v2.9.0 --notes-file docs/releases/2.9.0/RELEASE_NOTES.md` (if re-syncing)
3. Verify: `gh release view v2.9.0`.

### Fleet upgrade — canary batch

For each of Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab:

```bash
cd ~/Devel/prj/<project>
git status  # confirm clean (or stash user changes first)
intent upgrade --apply
# Post-upgrade acceptance:
cat .intent/config.json | jq .intent_version  # expect "2.9.0"
intent doctor                                  # expect clean
intent claude subagents list                   # no elixir; no canon worker-bee
ls ~/.intent/ext/worker-bee/                   # seeded after first canary run
ls ~/.claude/agents/elixir.md 2>/dev/null      # expect absent
ls ~/.claude/agents/worker-bee.md 2>/dev/null  # expect absent
```

Halt batch on any failure. Diagnose. Fix. Re-run canary before moving on.

### Fleet upgrade — batch 2

After canary passes, repeat for batch 2. Expected no surprises since canary exercised all paths.

### Fleet upgrade — batch 3

Same. A3/\* skipped.

### Post-fleet update

1. Update `intent/wip.md` with "v2.9.0 released; fleet at 2.9.0" entry.
2. Update `.claude/restart.md` similarly.
3. Commit with message `Session finish: v2.9.0 wrap-up` (no Claude attribution).
4. Push.

## Acceptance Criteria

### Pre-release gate

Per `intent/st/ST0034/design.md` §Testing Strategy §Release gate. Every item below must be green before `git tag -f v2.9.0 HEAD`:

- [ ] All WP01-WP10 exit checklists green
- [ ] `./tests/run_tests.sh` exits 0 with no skips attributable to ST0034, total test count ≥ 469 (baseline) + WP additions documented in each WP's impl.md
- [ ] `intent doctor` clean on the Intent repo
- [ ] `intent claude rules validate` exits 0 across the whole rule library (agnostic + elixir + rust + swift + lua)
- [ ] `intent claude rules index` produces a byte-identical `rules/index.json` on re-run (deterministic)
- [ ] `elixir intent/plugins/claude/rules/_schema/archetype/strong-assertions/good_test.exs` exits 0
- [ ] `elixir intent/plugins/claude/rules/_schema/archetype/strong-assertions/bad_test.exs` exits 0
- [ ] Canary dry-run (WP09) passed

If any gate fails: do not tag. Fix root cause, re-run from gate 1.

### Release commit

- [ ] `VERSION` says "2.9.0"
- [ ] CHANGELOG has dated v2.9.0 entry
- [ ] Release notes file exists at `docs/releases/2.9.0/RELEASE_NOTES.md`
- [ ] Neither file contains vanity metrics (no counts of rules/skills/subagents/tests)
- [ ] Commit message: `Release v2.9.0 -- Agentic Software Engineering Suite (ST0034)` (no Claude attribution, no Co-Authored-By)
- [ ] Git working tree clean after release commit

### Tag and push

- [ ] Tag `v2.9.0` exists on both `local` and `upstream` remotes
- [ ] `gh release view v2.9.0` shows release with synced notes
- [ ] Release notes thank elixir-test-critic for schema inspiration

### Fleet upgrade — canary batch

- [ ] Each canary project at `intent_version: 2.9.0`
- [ ] Each canary: `intent doctor` clean
- [ ] Each canary: no `elixir` or `worker-bee` in `~/.claude/agents/`
- [ ] `~/.intent/ext/worker-bee/` exists and validates
- [ ] Critic subagents discoverable via `intent claude subagents list`
- [ ] `.intent_critic.yml` sample documented

### Fleet upgrade — batches 2 and 3

- [ ] All 11 remaining projects (6 + 5 minus canary's 5) at `intent_version: 2.9.0`
- [ ] All projects: `intent doctor` clean
- [ ] Zero rollbacks required

### Post-release

- [ ] `intent/wip.md` updated
- [ ] `.claude/restart.md` updated
- [ ] Post-release commit pushed to both remotes
- [ ] Session wrap documented

## Dependencies

- **WP01-WP10** all complete and green.

## Implementation Notes

### No Claude attribution

Per global rule: no `Co-Authored-By` lines, no Claude signatures, no AI attribution in commit messages. Enforced by:

- Manual discipline at commit time
- MEMORY.md note (`feedback_no_claude_commits`)
- CLAUDE.md rule

### Slipstream avoidance

If a bug is found post-tag:

- **Prefer v2.9.1** (new version, new tag, no force-move).
- **Accept re-tag only** if the bug is caught within minutes of the initial push and no one has pulled yet.

Reason: repeated slipstreams (we had one in v2.8.2) erode user trust.

### Canary selection rationale

- **Anvil**: large established project with Elixir + complex STs; canary-first because regression risk is highest there.
- **Arca/arca_cli, arca_config, arca_notionex**: three related smaller projects; test fleet cohesion.
- **Conflab**: active development, representative state.

Canary batch size of 5 balances coverage (enough variety) with speed (each can be verified in ~5 minutes).

### Halt-on-error discipline

If ANY canary fails:

1. Capture error state (log, `intent doctor` output, config file).
2. Do not proceed to batch 2.
3. Diagnose root cause.
4. Fix in Intent repo (may require v2.9.1 or commit-to-main-then-re-run).
5. Re-run canary end-to-end.

Do not cherry-pick "well, these 4 worked" and proceed. The 5th failure is representative.

### Fleet upgrade parallelism

Projects can be upgraded in parallel within a batch (independent `cd` + `intent upgrade --apply` runs). However, for canary batch, do serially to catch first-failure signal cleanly. For batches 2-3, parallel is acceptable.

### Git workflow precision

```bash
# Stage
git add VERSION CHANGELOG.md docs/releases/2.9.0/ intent/wip.md .claude/restart.md

# Also add any content not yet committed from earlier WPs
git status  # review

# Commit
git commit -m "Release v2.9.0 -- Agentic Software Engineering Suite (ST0034)"

# Tag
git tag -f v2.9.0 HEAD

# Push
git push local main
git push upstream main
git push -f local v2.9.0
git push -f upstream v2.9.0

# GH release
gh release create v2.9.0 --notes-file docs/releases/2.9.0/RELEASE_NOTES.md
```

### Release notes format

Follow v2.5.0 as reference. Narrative prose for each major capability. Breaking changes section clear. Acknowledgements section for elixir-test-critic. No counts of rules/skills/tests.

## Risks and Edge Cases

### Fleet project has uncommitted changes

User has work-in-progress in one of the fleet projects. `intent upgrade --apply` could run on top. Mitigation: check `git status` before upgrade; stash or warn user.

### Fleet project is at unexpected version

One of the 16 might be at v2.7.0 or older. The chain handles this (per WP09), but log-level check recommended.

### A3/\* projects

Explicitly skipped per existing user direction. Document in WP11 execution log.

### `gh release create` conflicts with existing release

If the release exists (from prior attempts), use `gh release edit` instead. Decision at WP11 runtime.

### Force-push failure

Upstream might have branch protection rules. Mitigation: user runs the `gh` commands interactively; not automated.

### Post-release hotfix

If a critical bug is found within hours of release, prepare v2.9.1 process: new tag, new release, chain migration through (identity function if no real fix).

### Backwards-compat complaints

User base is the author and 16 projects; complaints unlikely but possible from third-party observers noticing the GH release. Release notes and acknowledgements are clear about breaking changes.

### Linter diffs during release commit

Markdown linter may re-format CHANGELOG or release notes on save. Include linter changes in the same commit per MEMORY.md guidance.

## Testing Approach

This WP is verification-heavy rather than code-writing.

### Pre-release regression

```bash
cd /Users/matts/Devel/prj/Intent
tests/run_tests.sh     # all green
intent doctor          # clean
```

### Release artifact checks

```bash
cat VERSION            # "2.9.0"
grep "2.9.0" CHANGELOG.md
ls docs/releases/2.9.0/RELEASE_NOTES.md
grep -E "^- [0-9]+ (tests|rules|skills|subagents|critics)" CHANGELOG.md docs/releases/2.9.0/  # zero hits
```

### Tag sanity

```bash
git tag -l | grep "^v2.9.0$"
git show v2.9.0 --format=oneline -s
git ls-remote --tags local | grep v2.9.0
git ls-remote --tags upstream | grep v2.9.0
```

### Canary script

Authored in WP09 as `scripts/ci/canary-dry-run.sh`; reused here for live fleet.

```bash
for proj in Anvil Arca/arca_cli Arca/arca_config Arca/arca_notionex Conflab; do
  (cd ~/Devel/prj/$proj && intent upgrade --apply && intent doctor) || exit 1
done
```

## Size and Estimate

- **Size**: M (Medium, 2-3 sessions).
- **Session 1**: Pre-release checks, VERSION bump, CHANGELOG + release notes finalisation, release commit, tag, push.
- **Session 2**: Canary batch upgrade + full verification. If clean, continue to batch 2.
- **Session 3**: Batch 3 + post-release wip.md/restart.md updates.

## Exit Checklist

- [ ] All acceptance criteria met
- [ ] v2.9.0 tagged on both remotes
- [ ] GH release published
- [ ] 16 fleet projects at v2.9.0 (all green on `intent doctor`)
- [ ] No rollbacks performed
- [ ] `intent/wip.md` and `.claude/restart.md` updated and pushed
- [ ] ST0034 `info.md` status updated to "Done" with completion date
- [ ] Session wrap commit on both remotes
- [ ] User confirms release
