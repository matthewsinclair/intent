# Implementation - ST0024: Add work packages as first class citizens within a steel thread

## Implementation

### bin/intent_wp (345 lines)

New command script with 6 subcommands: `new`, `done`, `start`, `list`, `show`, `help`.

Key functions:

- `parse_wp_specifier()` -- parses `STID/NN` into `ST_ID` and `WP_NUM` globals, supports bare numbers
- `resolve_st_dir()` -- finds ST directory regardless of status subdirectory (root, NOT-STARTED, COMPLETED, CANCELLED)
- `get_next_wp_number()` -- scans existing WP directories for next sequential number (01-99)

Config extraction uses shared `get_config_field()` from intent_helpers instead of inline grep.

### Template

`lib/templates/prj/st/WP/info.md` -- frontmatter with `wp_id`, `title`, `scope`, `status` fields. Sed substitution for `[Title]`, `[Date]`, `[Author]`, `WP-NN` placeholders. Heredoc fallback with `__VERBLOCK__`, `__WPNUM__`, `__TITLE__` substitution.

### Shared helpers added to bin/intent_helpers

- `get_config_field()` -- extracts a field from `.intent/config.json` without jq dependency. Replaces inline `grep -oE` patterns in both `intent_st` and `intent_wp`.

### Documentation

All key docs updated with WP commands, bare number syntax, special character support, and WP directory structure:

- README.md, CLAUDE.md, user_guide.md, reference_guide.md

## Challenges & Solutions

1. **Special characters in WP titles**: Reused `escape_sed_replacement()` from intent_helpers (originally added in ST0022 for st new).
2. **Finding STs across status directories**: `resolve_st_dir()` checks root, NOT-STARTED, COMPLETED, CANCELLED in order.
3. **Heredoc fallback**: Uses quoted heredoc (`<< 'TEMPLATE'`) to prevent premature variable expansion, then sed for substitution.
