# Tasks - ST0030: Cherry-Pick Superpowers Patterns for Intent

## WP-01: Skill Dependency Chains (High Priority)

- [x] Add `chains_to:` field to `in-start/SKILL.md` frontmatter
- [x] Add `chains_to:` field to `in-plan/SKILL.md` frontmatter
- [x] Add `chains_to:` field to `in-next/SKILL.md` frontmatter
- [x] Add `chains_to:` field to `in-finish/SKILL.md` frontmatter
- [x] Add "Skill Chain" section at bottom of each modified skill explaining the chain
- [ ] Test: invoke `/in-start` in Claude Code, verify chain suggestion appears

## WP-02: Rationalization Tables (High Priority)

- [x] Add "Red Flags" table to `in-essentials/SKILL.md`
- [x] Add "Red Flags" table to `in-plan/SKILL.md`
- [x] Add "Red Flags" table to `in-finish/SKILL.md`
- [x] Add "Red Flags" table to `in-standards/SKILL.md`
- [x] Verify no em dashes in any added content
- [ ] Test: invoke `/in-essentials` and verify rationalization table is in context

## WP-03: in-verify Skill (High Priority)

- [x] Create `intent/plugins/claude/skills/in-verify/SKILL.md`
- [x] Register in MODULES.md
- [x] Install: `intent claude skills install in-verify`
- [ ] Test: invoke `/in-verify` in Claude Code
- [ ] Test: verify skill catches "tests pass" claims without evidence

## WP-04: Plan Granularity Standards (Medium Priority)

- [x] Add "Plan Quality Standards" section to `in-plan/SKILL.md`
- [x] Include no-placeholder rule, step sizing, file specificity, verification commands
- [ ] Test: invoke `/in-plan`, verify granularity standards are enforced

## WP-05: in-debug Skill (Medium Priority)

- [x] Create `intent/plugins/claude/skills/in-debug/SKILL.md`
- [x] Implement 4-phase debugging procedure
- [x] Implement 3-strike architectural review rule
- [x] Register in MODULES.md
- [x] Install: `intent claude skills install in-debug`
- [ ] Test: invoke `/in-debug` in Claude Code

## WP-06: in-review Skill (Medium Priority)

- [x] Create `intent/plugins/claude/skills/in-review/SKILL.md`
- [x] Implement two-stage review: spec compliance then code quality
- [x] Document agent delegation for Elixir projects (diogenes + elixir)
- [x] Register in MODULES.md
- [x] Install: `intent claude skills install in-review`
- [ ] Test: invoke `/in-review` in Claude Code

## Verification

- [x] All 462 BATS tests still pass (`tests/run_tests.sh`)
- [x] `intent claude skills list` shows 3 new skills (in-verify, in-debug, in-review)
- [x] `intent claude skills sync` works with all modified skills (6 updated)
- [x] No em dashes in any new or modified skill files
- [x] All markdown tables are column-aligned
- [ ] Commit and update ST0030 status

## Dependencies

- WP-01, WP-02, WP-03: Independent of each other (can be parallelized)
- WP-04: Can be done alongside WP-01/02/03 (modifies `in-plan`, coordinate with WP-01 and WP-02 if done in same session)
- WP-05, WP-06: Independent of each other, no hard dependencies on WP-01/02/03
- Verification: After all WPs complete
