# Tasks - ST0028: TCA v3.0 -- Process Doc Update + Skill Suite

## Stream A: TCA Doc Update

- [x] A1. Frontmatter + Purpose (bump to v3.0, cite 3 runs, skill suite note)
- [x] A2. Phase 0.1 -- Rule precision, validated Rust/Swift, Ash A1-A5
- [x] A3. Phase 0.2 -- Effective file count model, Phase 0.5 pre-filtering
- [x] A4. Phase 1 -- Confidence field, file manifest verification, General Opus
- [x] A5. Phase 2 -- Cluster dedup, P2a/P2b split, 5-tier scheme
- [x] A6. Phase 4 -- Main conversation remediation, test optimization
- [x] A7. Appendices (B, C, D, E, F updates)

## Stream B: Skill Suite

- [x] B1. in-tca-init (SKILL.md + tca-init.sh)
- [x] B2. in-tca-audit (SKILL.md + tca-progress.sh)
- [x] B3. in-tca-synthesize (SKILL.md)
- [x] B4. in-tca-remediate (SKILL.md)
- [x] B5. in-tca-finish (SKILL.md + tca-report.sh)

## Verification

- [x] Read updated TCA doc end-to-end for coherence
- [x] Verify all cross-references (P2a/P2b, Example C, skill references)
- [x] Confirm no project-specific details leaked into doc or skills
- [x] Test tca-init.sh on a test directory
- [x] Test tca-progress.sh parsing
- [x] Test tca-report.sh template generation
- [x] Install all 5 skills
- [x] Commit and update ST0028 status

## Release + Docs

- [x] Update all relevant docs (user-facing and intent internal)
- [x] Bump version number (v2.6.0 -> v2.7.0)
- [ ] Tag and publish release to both remotes + GitHub release
- [ ] Consider peer language skills (in-rust-essentials, in-swift-essentials) to complement the Elixir-specific ones

## Dependencies

Steps A2-A6 can be parallelized.
Steps B1-B5 can be parallelized (after Stream A core completes).
A7 (Appendices) depends on A2-A6.
A1 (Frontmatter) depends on A7.
