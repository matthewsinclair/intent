---
description: "TCA finish: final verification, ST doc updates, feedback report generation, and session wrap-up"
---

# TCA Finish

Wraps up a Total Codebase Audit: runs final verification, updates steel thread documents, generates a feedback report, and performs standard session cleanup.

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

### 3. Generate feedback report

Run the report script:

```bash
bash "$(find ~/.claude/skills/in-tca-finish -name tca-report.sh 2>/dev/null | head -1)" \
  --st-dir intent/st/STXXXX
```

This generates a feedback report template pre-populated with audit data. Fill in the analytical sections:

- **Rule-by-rule analysis**: Which rules had most value? Which were noisy?
- **WP sizing assessment**: What worked? What was too large/small?
- **Sub-agent effectiveness**: Turns used, false positive rate per agent type
- **Process improvements**: Recommendations for TCA doc updates

### 4. Write feedback WP

Create a final WP for the feedback report:

```bash
intent wp new STXXXX "Feedback Report"
```

Write the completed feedback report to this WP's `socrates.md`.

### 5. Commit everything

```bash
git add intent/st/STXXXX/
git commit -m "audit: TCA complete -- {unique} violations, {fixed} fixed"
```

### 6. Standard session wrap-up

Run `/in-finish` for standard session cleanup:

- Update `intent/wip.md`
- Update `intent/restart.md`
- Update `.claude/restart.md`

## Important Notes

- Always run full verification before declaring the audit complete
- The feedback report is essential for improving future TCAs
- Include both what worked and what did not work in the report
- Compare metrics with previous TCAs if applicable
- Deferred items must be explicitly listed with reasons
