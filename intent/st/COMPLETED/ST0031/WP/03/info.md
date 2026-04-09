---
verblock: "09 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-03
title: "Finish guard: pre-flight check to block premature close-out"
scope: Small
status: Done
---

# WP-03: Finish guard: pre-flight check to block premature close-out

## Objective

Make the Lamplight premature-close-out failure mode mechanically impossible. The Lamplight operator wrote "ST0121 complete" into all session docs 24 hours before the feedback report existed. Guidance in SKILL.md was not enough -- an eager operator skipped past it. Replace guidance with a bash guard that exits 1 when preconditions are unmet, and restructure the finish skill so the guard must pass before any session docs are touched.

Addresses Lamplight feedback item P0.3 and lands the D1 decision (canonicalize `feedback-report.md` to `$TCA_DIR/feedback-report.md` as a top-level artifact rather than a "Feedback WP" socrates.md).

## Deliverables

1. New `--check-only` mode in `tca-report.sh` that runs pre-flight guards and exits 0 on pass or 1 on fail, without generating the report.
2. Four pre-flight guards in `tca-report.sh`:
   - Guard 3 (shape, always runs): WP/ directory exists, design.md exists, design.md contains a recognizable rule set. Fires in both normal and check-only modes so the script refuses to operate on non-TCA steel threads.
   - Guard 1a (check-only only): `$ST_DIR/feedback-report.md` exists.
   - Guard 1b (check-only only): feedback-report.md contains no `[Fill in:` placeholders.
   - Guard 2 (check-only only): info.md has zero unchecked `- [ ]` acceptance criteria.
3. Guards 1a/1b/2 scoped to check-only mode because normal mode (template generation) runs before the report exists -- running those guards in normal mode would be chicken-and-egg.
4. Pure-shell counters (while-loop + case) for guards 1b and 2 instead of `grep -c` or `grep | wc -l` pipelines, because the latter interact badly with `set -euo pipefail` (grep returning 1 on zero matches kills the pipeline silently on assignment).
5. Restructured `in-tca-finish/SKILL.md` flow:
   - New top-of-file invariant callout flagging the guard as load-bearing.
   - Step 3 now writes the template directly to `$TCA_DIR/feedback-report.md` via `-o`.
   - Step 4 (old: "Write feedback WP") deleted; new step 4 is "Fill in the feedback report" as a manual operator step.
   - New step 5: close acceptance criteria in info.md (manual).
   - New step 6: run `tca-report.sh --check-only` pre-flight guard (must pass).
   - Steps 7-8 (commit, /in-finish wrap-up) explicitly gated on the guard passing.
6. Updated Important Notes with three new bullets: guard is load-bearing, feedback-report.md is a top-level artifact not a WP, and a Lamplight incident reference.

## Acceptance Criteria

- [x] `tca-report.sh` accepts `--check-only` flag
- [x] `tca-report.sh` shape check (guard 3) runs in both modes and refuses non-TCA STs
- [x] Guards 1a/1b/2 run only in check-only mode (verified via smoke tests)
- [x] Smoke test S1: check-only with no report fails with guard 1a error (verified)
- [x] Smoke test S2: template generation works in normal mode (verified)
- [x] Smoke test S3: check-only with unfilled placeholders fails with guard 1b error (verified)
- [x] Smoke test S4: check-only with unchecked criteria fails with guard 2 error (verified)
- [x] Smoke test S5: check-only with everything fixed passes (verified: "ok: pre-flight guards passed")
- [x] Pure-shell counters do not interact with `set -euo pipefail` (verified)
- [x] `bash -n` passes on `tca-report.sh`
- [x] `in-tca-finish/SKILL.md` step 3 writes to `$TCA_DIR/feedback-report.md`
- [x] "Feedback WP" step deleted from skill
- [x] Pre-flight guard is a required skill step before commit and wrap-up
- [x] No em dashes in SKILL.md
- [x] No Claude attribution in commit message

## Dependencies

Depends on WP-01 (docs) because the SKILL.md invariant callout and script comments reference `total-codebase-audit.md` section 0.0 which WP-01 created.

Does NOT depend on WP-04 (rename) -- WP-03 uses the current `--st-dir` flag name throughout. WP-04 will sweep the rename across all 6 files in one mechanical pass.
