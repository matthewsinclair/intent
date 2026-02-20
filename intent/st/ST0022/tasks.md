# Tasks - ST0022: Harden `st new` -- Special Characters, Slugs, and --start Flag

## Phase 0: Documentation

- [x] Rewrite ST0022 info.md with objective, problem, solution, work packages
- [x] Write ST0022 design.md with design decisions and architecture
- [x] Write ST0022 tasks.md (this file)
- [x] Create WP/01 through WP/04 info.md files
- [ ] Commit Phase 0 documentation

## Phase 1: Special Character Fix (WP-01)

- [ ] Add `escape_sed_replacement()` function to `bin/intent_st`
- [ ] Apply escaping to sed template path (lines 361-369)
- [ ] Fix heredoc fallback path to avoid shell expansion of title (lines 374-400)
- [ ] Escape title in `update_steel_threads_index()` call
- [ ] Add BATS tests for `/`, `&`, `\`, `$` in titles

## Phase 2: Slug System (WP-02)

- [ ] Add `slugify()` function to `bin/intent_st`
- [ ] Add `slug:` placeholder to `lib/templates/prj/st/ST####/info.md`
- [ ] Generate and substitute slug in template-sed path of `st new`
- [ ] Generate and substitute slug in heredoc-fallback path of `st new`
- [ ] Extract slug from frontmatter in `st list` data collection
- [ ] Replace "Title" column header with "Slug" in `st list` display
- [ ] Update `st sync --write` to use slug in index output
- [ ] Fall back to title for pre-existing STs without `slug:` field
- [ ] Add BATS tests for slug generation edge cases
- [ ] Add BATS tests for slug display in `st list`

## Phase 3: `-s|--start` Flag (WP-03)

- [ ] Add flag parsing (`-s|--start`) before positional title arg in `new` command
- [ ] After creation with flag, invoke start logic inline (move dir, update status)
- [ ] Update help text to show `-s|--start` option
- [ ] Add BATS tests for `-s` and `--start` forms
- [ ] Add BATS test for flag-after-title (`intent st new "Title" -s`)

## Phase 4: Documentation and Wrap-up (WP-04)

- [ ] Update `intent/usr/reference_guide.md` with new flag and slug field
- [ ] Update `intent/usr/user_guide.md` with examples
- [ ] Update `CHANGELOG.md` with version entry
- [ ] Update test count in documentation
- [ ] Run full test suite, confirm all pass

## Dependencies

```
WP-01 -> WP-02 -> WP-03 -> WP-04
```
