# Tasks - ST0021: Intent Autopsy

## Work Packages

### WP-01: Core Implementation

- [x] Update ST0021 docs (info.md, design.md, tasks.md)
- [x] Create `autopsy.exs` Elixir script with all modules
- [x] Create `banned-words.txt` default file
- [x] Create `intent-autopsy/SKILL.md`

### WP-02: Skills Infrastructure

- [x] Extend `intent_claude_skills` install for full directory copy
- [x] Verify existing skills still install correctly (38 tests pass)
- [x] Add Elixir check to `intent doctor`

### WP-03: Testing

- [x] Create `tests/unit/test_autopsy.bats` (19 tests)
- [x] Verify all existing tests still pass (346 total across 17 files)
- [x] Test script against real JSONL sessions

### WP-04: Documentation

- [x] Update CHANGELOG.md
- [x] Update user_guide.md
- [x] Update reference_guide.md
- [x] Update deployment_guide.md
- [x] Update tests/README.md

## Dependencies

- WP-01 must complete before WP-02 (script needs to exist for install testing)
- WP-02 must complete before WP-03 (tests exercise install mechanism)
- WP-01 through WP-03 before WP-04 (docs reflect final state)
