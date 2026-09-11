---
description: "TCA finish: final verification, ST doc updates, feedback report generation, and session wrap-up"
chains_to: ["in-finish"]
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

Once design.md, tasks.md and impl.md are updated, record each in the store: `intent st attach STXXXX <file> --from intent/st/STXXXX/<file>`. Until then `intent organize --verbose` lists them as unclaimed.

**impl.md**: Implementation notes:

- Session count and approximate wall clock times
- Deferred items and why they were deferred
- False positive patterns discovered
- Process improvements for next audit

### 3. Generate feedback report template

Run the report script and write directly to the canonical path at the TCA ST root:

```bash
bash "$(find ~/.claude/skills/in-tca-finish -name tca-report.sh 2>/dev/null | head -1)" \
  --tca-dir intent/st/STXXXX \
  -o intent/st/STXXXX/feedback-report.md
```

This generates a template with the WP list and each WP's Complete/Pending status. Its severity columns parse `| High`/`| Medium`/`| Low` rows, not the critic `Summary:` line, so they read 0 for verbatim critic reports, and the dedup rate prints `?`. Fill both from the synthesis WP. The analytical sections are `[Fill in ...]` placeholders.

### 4. Fill in the feedback report

Open `intent/st/STXXXX/feedback-report.md` and replace every `[Fill in: ...]` placeholder with real analysis:

- **Rule Analysis**: which IN-\* rules had most value, which were noisy, rule-by-rule FP rates. Where a rule had a high FP rate, propose either a `.intent_critic.yml` `disabled:` entry for the project or a Detection refinement for the rule itself (the latter is an edit to the rule's `RULE.md` in the Intent source repo, validated by `intent claude rules validate`).
- **WP Sizing Assessment**: which WPs were appropriately sized, which were too large or too small.
- **Sub-Agent Effectiveness** (the template's name for critic effectiveness): per-WP critic findings counts and FPs noted (use the metadata lines in each WP's socrates.md). Where the critic missed something a human reviewer caught, the gap is a candidate Detection improvement on the relevant RULE.md.
- **Process Improvements**: concrete recommendations for the TCA doc, the rule library, or the skill suite based on what went wrong this audit.

The pre-flight guard refuses while any `[Fill in` placeholder remains, including `[Fill in if applicable]` under Comparison with Previous TCAs.

### 5. Close the acceptance contract

`info.md` and `acceptance.md` are generated views, so tick nothing by hand. List the contract with `intent ac list STXXXX`, satisfy each non-test AC with `intent ac satisfy STXXXX AC-NN --evidence <ref>` (eg `--evidence feedback-report.md`), turn each covering test green with `intent at green STXXXX AT-NN`, then run `intent ac gate STXXXX` -- it exits non-zero with `BLOCKED` while any AC is unsatisfied.

### 6. Pre-flight guard

Run the guard in `--check-only` mode:

```bash
bash "$(find ~/.claude/skills/in-tca-finish -name tca-report.sh 2>/dev/null | head -1)" \
  --tca-dir intent/st/STXXXX \
  --check-only
```

The guard verifies:

- The TCA ST is properly shaped (WP/ directory, and a design.md containing the literal `rule set` -- or `Rule <N>` / `R<N>` -- case-sensitive)
- `feedback-report.md` exists at the canonical location
- The feedback report contains no unfilled `[Fill in:` placeholders
- `info.md` has no `- [ ]` lines -- a v3 `info.md` never has any, so this check always passes; the acceptance contract is checked by `intent ac gate STXXXX` (step 5), and the guard does not replace it

If the guard fails, fix the flagged issue and re-run. **Do NOT hand-edit session docs or run `/in-finish` manually until this guard passes.** The failure mode this guard prevents is the Lamplight ST0121 24-hour window of lying docs (commits 75706c18 to 98616a0c, 2026-04-08) -- closing the TCA before the feedback report exists or before acceptance criteria are actually met.

### 7. Close and commit

After the guard and `intent ac gate STXXXX` both pass, close the thread with `intent st done STXXXX`, then commit:

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
