# Done - ST0022: Harden `st new`

All 4 work packages completed. Released as part of v2.4.0.

## Phase 0: Documentation

- [x] Rewrite ST0022 info.md with objective, problem, solution, work packages
- [x] Write ST0022 design.md with design decisions and architecture
- [x] Write ST0022 tasks.md
- [x] Create WP/01 through WP/04 info.md files
- [x] Commit Phase 0 documentation

## Phase 1: Special Character Fix (WP-01)

- [x] Add `escape_sed_replacement()` function to `bin/intent_st`
- [x] Apply escaping to sed template path
- [x] Fix heredoc fallback path to use quoted heredoc + sed substitution
- [x] Escape title in legacy template path
- [x] Add BATS tests for `/`, `&`, `\` in titles

## Phase 2: Slug System (WP-02)

- [x] Add `slugify()` function to `bin/intent_st`
- [x] Add `slug:` placeholder to `lib/templates/prj/st/ST####/info.md`
- [x] Generate and substitute slug in template-sed path of `st new`
- [x] Generate and substitute slug in heredoc-fallback path of `st new`
- [x] Extract slug from frontmatter in `st list` data collection
- [x] Replace "Title" column header with "Slug" in `st list` display
- [x] Update `st sync --write` to use slug in index output (via list)
- [x] Fall back to title for pre-existing STs without `slug:` field
- [x] Add BATS tests for slug generation edge cases
- [x] Add BATS tests for slug display in `st list`

## Phase 3: `-s|--start` Flag (WP-03)

- [x] Add flag parsing (`-s|--start`) before positional title arg in `new` command
- [x] After creation with flag, invoke start logic inline (move dir, update status)
- [x] Update help text to show `-s|--start` option
- [x] Add BATS tests for `-s` and `--start` forms
- [x] Add BATS test for flag-after-title (`intent st new "Title" -s`)

## Phase 4: Documentation and Wrap-up (WP-04)

- [x] Update `intent/usr/reference_guide.md` with new flag and slug field
- [x] Update `intent/usr/user_guide.md` with examples
- [x] Update `CHANGELOG.md` with version entry
- [x] Run full test suite, confirm all pass
