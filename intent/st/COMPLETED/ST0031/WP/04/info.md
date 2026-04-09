---
verblock: "09 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-04
title: "Rename --st-dir to --tca-dir, audit metadata line, chains_to"
scope: Small
status: Done
---

# WP-04: Rename --st-dir to --tca-dir, audit metadata line, chains_to

## Objective

Mechanical cleanup WP. Rename the misleading `--st-dir` flag to `--tca-dir` across the whole TCA skill suite (it takes the TCA's steel thread directory, not an arbitrary ST's), add the audit-metadata recording instruction that the Lamplight feedback report identified as missing, and encode the skill chain via `chains_to:` frontmatter on all 5 TCA skills so the workflow the feedback report implicitly assumes becomes discoverable to Claude.

Addresses Lamplight feedback items P2 (rename), P3 (metadata line), and the Bonus chain-encoding opportunity.

## Deliverables

1. Rename `--st-dir` to `--tca-dir` and `ST_DIR` to `TCA_DIR` across all 6 TCA files:
   - `in-tca-init/scripts/tca-init.sh` (flag + variable + help text)
   - `in-tca-init/SKILL.md` (flag in step 5 invocation)
   - `in-tca-audit/scripts/tca-progress.sh` (flag + variable + help text)
   - `in-tca-audit/SKILL.md` (flag in step 1 and step 7 invocations)
   - `in-tca-finish/scripts/tca-report.sh` (flag + variable + help text)
   - `in-tca-finish/SKILL.md` (flag in step 3 and step 6 invocations)
2. Help text description updated from "Steel thread directory" to "TCA steel thread directory" on all three scripts so the flag description reflects the TCA scope.
3. New metadata-line instruction in `in-tca-audit/SKILL.md` Post-WP section: each component audit must record `**Agent**: {type}; **Turns**: N; **Raw hits**: N; **FPs**: N` at the top of the WP's socrates.md. This feeds the "Sub-Agent Effectiveness" section of the final feedback report. On the Lamplight ST0121 run, this was "not tracked" because there was no instruction to record it.
4. `chains_to:` frontmatter added to all 5 TCA SKILL.md files encoding the workflow:
   - `in-tca-init` chains to `["in-tca-audit"]`
   - `in-tca-audit` chains to `["in-tca-synthesize"]`
   - `in-tca-synthesize` chains to `["in-tca-remediate"]`
   - `in-tca-remediate` chains to `["in-tca-finish"]`
   - `in-tca-finish` chains to `["in-finish"]`

## Acceptance Criteria

- [x] Zero `--st-dir` or `ST_DIR` occurrences remain anywhere in `intent/plugins/claude/skills/in-tca-*/` (verified via grep)
- [x] All three scripts pass `bash -n` syntax check after the rename
- [x] `tca-init.sh --help` shows `--tca-dir PATH    TCA steel thread directory`
- [x] `tca-report.sh --tca-dir PATH -o FILE` successfully generates a template (verified)
- [x] `tca-report.sh --tca-dir PATH --check-only` fires the guards correctly (verified)
- [x] Old `--st-dir` flag returns "unknown option" error (verified)
- [x] `intent claude skills list` successfully lists all 22 skills including the 5 TCA skills (verified)
- [x] Metadata-line instruction is present in `in-tca-audit/SKILL.md` Post-WP section as the first post-completion step
- [x] `chains_to:` is present on line 3 of all 5 TCA SKILL.md files (verified via grep)
- [x] No em dashes in any edited file
- [x] No Claude attribution in commit message

## Dependencies

Depends on WP-03 because `tca-report.sh` was heavily edited in WP-03 to add the `--check-only` mode and guards. Running the rename on top of that stable post-guard code keeps the diff reviewable.

WP-04 is the final WP of ST0031. After this lands, the steel thread can be closed.
