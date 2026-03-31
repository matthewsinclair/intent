# Design - ST0028: TCA v3.0 -- Process Doc Update + Skill Suite

## Design Decisions

1. **P2 split: YES** -- 5-tier scheme (P0/P1/P2a/P2b/P3) replaces 4-tier (P0/P1/P2/P3)
2. **Confidence field: YES** -- HIGH/MEDIUM/LOW on each audit finding
3. **Rust/Swift rules: FULL REPLACEMENT** -- validated rules from Conflab replace hypothetical ones
4. **Ash rules: PHASE 0.1** -- A1-A5 as first-class supplemental rules
5. **5 skills**: init, audit, synthesize, remediate, finish
6. **3 scripts**: tca-init.sh, tca-progress.sh, tca-report.sh
7. **Doc kept + updated** as reference; skills are the operational interface

## Architecture

### Stream A: Doc Update (v2.0 -> v3.0)

~330 net new lines across all sections:

- Phase 0.1: Rule precision, validated Rust/Swift, Ash A1-A5
- Phase 0.2: Effective file count model, Phase 0.5 pre-filtering
- Phase 1: Confidence field, file manifest verification, General Opus agent
- Phase 2: Cluster dedup, P2a/P2b split, 5-tier scheme
- Phase 4: Main conversation remediation, test optimization
- Appendices: Polyglot expansion, validated Rust/Swift, Ash prompts, Example C, new lessons

### Stream B: Skill Suite

5 skills at `intent/plugins/claude/skills/in-tca-*/`:

| Skill             | Script          | Purpose                    |
| ----------------- | --------------- | -------------------------- |
| in-tca-init       | tca-init.sh     | Provisioning + WP creation |
| in-tca-audit      | tca-progress.sh | Component audit execution  |
| in-tca-synthesize | (none)          | Cross-WP synthesis         |
| in-tca-remediate  | (none)          | Batched remediation        |
| in-tca-finish     | tca-report.sh   | Wrap-up + feedback report  |

## Alternatives Considered

- **Single mega-skill**: Rejected -- TCA phases are too distinct and each requires different context
- **Scripts only (no SKILL.md)**: Rejected -- the process guidance is the primary value, not the automation
- **Separate doc per phase**: Rejected -- one reference doc with skills as operational interface is cleaner
