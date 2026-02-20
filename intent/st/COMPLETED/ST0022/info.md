---
verblock: "20 Feb 2026:v0.4: matts - Completed"
intent_version: 2.4.0
status: Completed
slug: harden-st-new
created: 20260220
completed: 20260220
---

# ST0022: Harden `st new` -- Special Characters, Slugs, and --start Flag

## Objective

Fix three shortcomings in `intent st new`:

1. Special characters in the title break `sed` substitution during creation
2. There is no slug -- a URL/filename-safe identifier derived from the title -- making listings less scannable
3. Creating then immediately starting a steel thread is two commands when it should be one

## Problem

**Special characters**: Titles containing `/`, `&`, `\`, `$`, or backticks cause `sed` failures because the title is interpolated directly into `sed -e "s/\[Title\]/$TITLE/g"` without escaping. The heredoc fallback path also expands `$` and backticks in the title. Any user who puts a slash or ampersand in their title gets a broken info.md or a hard error.

**No slugs**: `st list` and `steel_threads.md` show the full title, which can be long and hard to scan. There is no short, machine-friendly identifier beyond the ST number. A slug (e.g. `my-cool-feature`) would make listings more readable and provide a stable human-friendly label.

**Two-step create+start**: Creating a new steel thread and immediately starting it requires `intent st new "Title"` followed by `intent st start ST####`. This is friction for the common case where you know you want to start working immediately.

## Solution

Three independent fixes to `bin/intent_st`, phased to avoid conflicts:

1. **Escape function** -- `escape_sed_replacement()` that escapes `/ & \` in sed replacement strings; fix the heredoc fallback to use a quoted heredoc (`<< 'TEMPLATE'`) with sed substitution
2. **Slug system** -- `slugify()` function (lowercase, collapse non-alnum to hyphens, max 50 chars); add `slug:` field to frontmatter template and info.md; show slug instead of title in `st list` and `steel_threads.md`
3. **`--start` flag** -- Parse `-s|--start` before the positional title argument; after creation, call existing start logic inline

## Work Packages

- **WP-01**: Special Character Handling -- make `st new` safe for any title string
- **WP-02**: Slug Generation System -- auto-generate slugs, store in frontmatter, display in listings
- **WP-03**: `-s|--start` Flag -- create and immediately start a steel thread in one command
- **WP-04**: Documentation and Wrap-up -- update reference guide, user guide, CHANGELOG, run full test suite

## Sequencing

```
Phase 0 (documentation): ST0022 docs -- NO CODE until committed
Phase 1 (special chars):  WP-01
Phase 2 (slugs):          WP-02
Phase 3 (--start):        WP-03
Phase 4 (wrap-up):        WP-04
```

Linear: WP-01 -> WP-02 -> WP-03 -> WP-04

## Related Steel Threads

- ST0020: Modernizing Intent's Elixir Support (established skills/subagent infrastructure)

## Key Decisions

- Slug replaces Title in the index table -- the table is already tight at 80 cols, adding a column would overflow
- Slug is frontmatter-only, not an alternative ST identifier -- no `intent st show my-cool-feature`
- Max slug length 50 chars, truncated on word boundary where possible
- `--start` reuses existing start logic inline, not via subprocess
- Escape function handles `/`, `&`, `\` -- the three characters that break sed replacement strings (`$` is not special in sed replacement context)
