---
description: "TCA finish: final verification, ST doc updates, feedback report generation, and session wrap-up"
---

# TCA Finish

> **Invariant (load-bearing)**: This skill refuses to touch high-level session docs (`intent/wip.md`, `intent/restart.md`, `.claude/restart.md`, audited ST docs) until the pre-flight guard passes. The guard is enforced by `tca-report.sh --check-only` and cannot be bypassed without editing the script. It exists to prevent the Lamplight ST0121 premature-close-out incident (2026-04-08) from recurring. See `intent/docs/total-codebase-audit.md` section 0.0.

Wraps up a Total Codebase Audit: runs final verification, updates steel thread documents, generates a feedback report, verifies completion via pre-flight guard, and performs standard session cleanup.

The feedback report lives at `$TCA_DIR/feedback-report.md` as a top-level artifact of the TCA steel thread. It is NOT a work package -- a report about all the WPs should not itself be a WP.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Final verification

Run the full verification suite:

```bash
# Elixir
mix compile --warnings-as-errors && mix test && mix credo --strict

# Rust
cargo check && cargo test && cargo clippy -- -D warnings

# Swift
swift build && swift test
```

All must pass before proceeding.

### 2. Update steel thread documents

**tasks.md**: Mark all phases complete with final counts:

```markdown
- [x] Phase 0: Provisioning ({N} WPs created)
- [x] Phase 0.5: Pre-filtering ({N} mechanical findings)
- [x] Phase 1: Component audit ({N} WPs, {total} raw violations)
- [x] Phase 2: Synthesis ({unique} unique after {dedup_rate}% dedup)
- [x] Phase 3: Review (confirmed with owner)
- [x] Phase 4: Remediation ({fixed} fixed, {fp} false positives, {deferred} deferred)
```

**design.md**: Update to as-built state:

- Actual WP count (may differ from planned)
- Actual batch ordering (may have been reordered)
- Rules that were added or removed during audit
- Effective file count accuracy assessment

**impl.md**: Implementation notes:

- Session count and approximate wall clock times
- Deferred items and why they were deferred
- False positive patterns discovered
- Process improvements for next audit

### 3. Generate feedback report template

Run the report script and write directly to the canonical path at the TCA ST root:

```bash
bash "$(find ~/.claude/skills/in-tca-finish -name tca-report.sh 2>/dev/null | head -1)" \
  --st-dir intent/st/STXXXX \
  -o intent/st/STXXXX/feedback-report.md
```

This generates a pre-populated template with audit data (WP breakdown, per-WP counts, dedup rate estimate). The analytical sections are left as `[Fill in: ...]` placeholders.

### 4. Fill in the feedback report

Open `intent/st/STXXXX/feedback-report.md` and replace every `[Fill in: ...]` placeholder with real analysis:

- **Rule-by-rule analysis**: which rules had most value, which were noisy, rule-by-rule FP rates
- **WP sizing assessment**: which WPs were appropriately sized, which were too large or too small
- **Sub-agent effectiveness**: turns used per WP, FP rate per agent type (use the metadata lines in each WP's socrates.md)
- **Process improvements**: concrete recommendations for the TCA doc or skill suite based on what went wrong this audit

The pre-flight guard will refuse to close the audit while any `[Fill in:` placeholders remain in the report.

### 5. Close acceptance criteria

Open `intent/st/STXXXX/info.md` and close every `- [ ]` checkbox under Acceptance Criteria. An unchecked box signals that the TCA has not actually finished -- the pre-flight guard will refuse to close the audit if any remain unchecked.

### 6. Pre-flight guard

Run the guard in `--check-only` mode:

```bash
bash "$(find ~/.claude/skills/in-tca-finish -name tca-report.sh 2>/dev/null | head -1)" \
  --st-dir intent/st/STXXXX \
  --check-only
```

The guard verifies:

- The TCA ST is properly shaped (WP/ directory, design.md with rule set)
- `feedback-report.md` exists at the canonical location
- The feedback report contains no unfilled `[Fill in:` placeholders
- `info.md` has zero unchecked `- [ ]` acceptance criteria

If the guard fails, fix the flagged issue and re-run. **Do NOT hand-edit session docs or run `/in-finish` manually until this guard passes.** The failure mode this guard prevents is the Lamplight ST0121 24-hour window of lying docs (commits 75706c18 to 98616a0c, 2026-04-08) -- closing the TCA before the feedback report exists or before acceptance criteria are actually met.

### 7. Commit everything

```bash
git add intent/st/STXXXX/
git commit -m "TCA finish: STXXXX complete -- {unique} violations, {fixed} fixed"
```

### 8. Standard session wrap-up

Only after the pre-flight guard has passed in step 6, run `/in-finish` for standard session cleanup:

- Update `intent/wip.md`
- Update `intent/restart.md`
- Update `.claude/restart.md`

## Important Notes

- Always run full verification before declaring the audit complete
- The feedback report is essential for improving future TCAs
- Include both what worked and what did not work in the report
- Compare metrics with previous TCAs if applicable
- Deferred items must be explicitly listed with reasons
- **The `--check-only` pre-flight guard is load-bearing.** If it fails, do NOT hand-edit session docs or run `/in-finish` manually. Fix the underlying issue (missing feedback report, unfilled placeholders, unchecked acceptance criteria, or non-TCA-shaped ST) and re-run the guard.
- **The feedback report lives at `$TCA_DIR/feedback-report.md`**, not in a "Feedback WP". A report about all WPs should not itself be a WP, and the pre-flight guard expects the canonical path.
- **The Lamplight ST0121 incident is the reason this guard exists.** Commit 75706c18 wrote "ST0121 complete" into wip.md, intent/restart.md, .claude/restart.md, and impl.md before feedback-report.md existed, producing a 24-hour window of lying docs that required commit 98616a0c to repair. Do not repeat this.
