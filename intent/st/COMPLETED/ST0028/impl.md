# Implementation - ST0028: TCA v3.0 -- Process Doc Update + Skill Suite

## Implementation

### Stream A: TCA Doc Update (v2.0 -> v3.0)

Updated `intent/docs/total-codebase-audit.md` from 918 to 1144 lines (~226 net new).

Key changes:

- Phase 0.1: Rule precision boundaries, validated Rust/Swift rules (replacing hypothetical), Ash A1-A5
- Phase 0.2: Effective file count model with weight table, Phase 0.5 pre-filtering via grep
- Phase 1: Confidence field (HIGH/MEDIUM/LOW), file manifest verification, General Opus agent option
- Phase 2: 5-tier priority (P0/P1/P2a/P2b/P3), cluster dedup by root cause, dedup benchmarks
- Phase 4: Main conversation remediation model, test optimization with mix test --failed
- Appendix A: Polyglot considerations (X-rules, two-pass synthesis, effective file count)
- Appendix B: Validated Rust/Swift prompt additions, new Ash-specific prompts
- Appendix C: Updated Quick-Start Checklist with skill references and new steps
- Appendix D: Example C (polyglot: 256 files, 59% dedup rate)
- Appendix E: Anti-hallucination and R5 over-reporting lessons
- Appendix F: Remediation agent failure modes, R7 false positives, 3-column metrics table

### Stream B: TCA Skill Suite

Created 5 skills with 3 automation scripts:

| Skill             | Script          | Lines (SKILL.md) | Lines (script) |
| ----------------- | --------------- | ---------------: | -------------: |
| in-tca-init       | tca-init.sh     |             ~100 |           ~120 |
| in-tca-audit      | tca-progress.sh |              ~80 |           ~100 |
| in-tca-synthesize | (none)          |              ~90 |            N/A |
| in-tca-remediate  | (none)          |              ~70 |            N/A |
| in-tca-finish     | tca-report.sh   |              ~60 |           ~150 |

All scripts tested on temp directories. All skills installed via `intent claude skills install`.

### Version Bump

- v2.6.0 -> v2.7.0
- Updated: CLAUDE.md, .intent/config.json, CHANGELOG.md, MODULES.md, wip.md, restart.md, .claude/restart.md, MEMORY.md

## Technical Details

- All scripts are bash 3.x compatible (no associative arrays, no ${VAR^})
- Scripts use `set -euo pipefail` and cleanup traps where needed
- tca-progress.sh parses Summary tables from socrates.md files using case pattern matching
- tca-report.sh generates a markdown template pre-populated with audit statistics
- tca-init.sh creates WP directories with templated info.md, last WP is always synthesis

## Challenges & Solutions

- **Parallelization**: Both streams were independent, so Stream A (doc update) and Stream B (skill creation) were executed in parallel using agents
- **No em dashes**: All files checked for em dashes (none found) per project convention
- **Markdown linter**: Tables auto-aligned by linter on save, changes included in commit
