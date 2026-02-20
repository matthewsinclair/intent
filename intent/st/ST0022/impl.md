# Implementation - ST0022: Harden `st new` -- Special Characters, Slugs, and --start Flag

## Implementation Summary

All three features (WP-01 through WP-03) were implemented in a single pass to `bin/intent_st` since they all touch the same `new` command code. The changes were committed together with 15 new BATS tests.

## Key Implementation Details

### WP-01: escape_sed_replacement()

Added at line 272, escapes three characters: `\` (must be first to avoid double-escaping), `/`, and `&`. The `$` character was omitted because it has no special meaning in sed replacement strings -- the plan's concern about `$` was actually a shell-level issue, not a sed issue, and is fully handled by proper variable quoting.

The heredoc fallback was rewritten to use `<< 'TEMPLATE'` (quoted delimiter prevents all shell expansion) with placeholder tokens (`__TITLE__`, `__SLUG__`, `__AUTHOR__`, etc.) that are substituted via `sed -i.bak`. This is safer than the original `<< EOF` which expanded `$` and backticks.

### WP-02: slugify()

Added at line 282. Algorithm:
1. `tr '[:upper:]' '[:lower:]'` for lowercase
2. `sed 's/[^a-z0-9]/-/g'` to replace non-alnum with hyphens
3. `sed 's/--*/-/g'` to collapse runs
4. `sed 's/^-//;s/-$//'` to strip leading/trailing
5. Bash substring `${slug:0:50}` + `${truncated%-*}` for word-boundary truncation

The `st list` display reuses the `TITLE` variable name internally but extracts slug from frontmatter first: `grep -m 1 "^slug:" "$file"`. Falls back to title heading for pre-existing STs.

### WP-03: --start flag

Uses an ARGS array pattern instead of `break` to support flag in any position:
```bash
ARGS=()
while [ $# -gt 0 ]; do
  case "$1" in
    -s|--start) START_FLAG=1; shift ;;
    -*) error "Unknown option: $1" ;;
    *) ARGS+=("$1"); shift ;;
  esac
done
TITLE="${ARGS[0]}"
```

Start logic is inline (not subprocess): updates `status:` and `- **Status**:` via `sed -i.bak`, then moves directory from `NOT-STARTED/` to `intent/st/`, and sets `ST_STATUS="WIP"` so the subsequent `update_steel_threads_index` call uses the correct status.

Both directory-structure and legacy paths handle `--start`.

## Files Modified

| File                                   | Lines changed |
|----------------------------------------|---------------|
| `bin/intent_st`                        | +148/-38      |
| `lib/templates/prj/st/ST####/info.md`  | +2/-1         |
| `tests/unit/st_commands.bats`          | +198          |

## Test Coverage

15 new tests added to `st_commands.bats` (30 existing -> 45 total):

- 4 special character tests (slash, ampersand, backslash, mixed)
- 5 slug tests (basic, slash title, ampersand title, truncation, list display)
- 1 slug fallback test (old ST without slug field)
- 5 start flag tests (-s, --start, without flag, flag after title, slug in started thread)

Full suite: 327 tests across 16 files, all passing.
