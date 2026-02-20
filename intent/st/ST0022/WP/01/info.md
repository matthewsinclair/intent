---
verblock: "20 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-01
title: Special Character Handling
scope: Small
status: Not Started
---

# WP-01: Special Character Handling

## Objective

Make `intent st new` safe for any title string including `/`, `&`, `\`, `$`, backticks, quotes, and Unicode characters.

## Deliverables

- `escape_sed_replacement()` helper function in `bin/intent_st`
- Apply escaping to the sed-template path (lines 361-369) for all `$TITLE` substitutions
- Fix the heredoc-fallback path (lines 374-400) to avoid shell expansion of `$` and backticks in the title
- Escape title in the `update_steel_threads_index()` call (line 427)
- BATS tests covering all special character cases

## Current State

The `new` command interpolates `$TITLE` directly into sed replacement strings:

```bash
sed -e "s/\[Title\]/$TITLE/g" "$template" > "$output_file"
```

Characters `/`, `&`, `\`, and `$` break this substitution. The heredoc fallback uses an unquoted delimiter (`<< EOF`) which causes shell expansion of `$` and backticks in the title.

## Target State

Any valid string can be used as a steel thread title. The created info.md contains the exact title verbatim, regardless of special characters.

## Acceptance Criteria

- [ ] `intent st new "Features/Improvements"` succeeds (slash)
- [ ] `intent st new "Costs & Benefits"` succeeds (ampersand)
- [ ] `intent st new 'Price is $100'` succeeds (dollar sign)
- [ ] `intent st new "Back\slash"` succeeds (backslash)
- [ ] Created info.md contains the exact title verbatim in all cases
- [ ] BATS tests cover all four special character types

## Dependencies

- None (foundation for WP-02)
