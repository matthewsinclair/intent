---
verblock: "04 Feb 2026:v0.2: matts - WP01 complete, as-built"
wp: "01"
title: "intent treeindex CLI command"
status: Complete
---
# WP01: `bin/intent_treeindex` CLI Command

## Status

Complete (2026-02-04). 612 lines bash, 38 bats tests, 250/250 full suite.

## Objective

Create the `intent treeindex` CLI command that walks directories bottom-up and uses `claude -p` (headless mode) to generate `.treeindex` summary files stored in a centralized shadow directory.

## As-Built CLI Interface

```
intent treeindex [OPTIONS] DIR

DIR is required and must be a subdirectory of the project root.

OPTIONS:
  -d, --depth N     Depth to traverse (default: 2)
  --check           Check staleness only, do not generate
  --force           Regenerate ignoring fingerprints
  --model MODEL     Claude model to use (default: haiku)
  --dry-run         Show what would be generated without doing it
  -h, --help        Show help

EXAMPLES:
  intent treeindex lib/my_app/           # dir + 2 levels deep
  intent treeindex lib/ --depth 3        # 3 levels deep
  intent treeindex lib/ --check          # report stale indexes
  intent treeindex lib/my_app/ --force   # regenerate everything
```

## As-Built Core Functions

1. **Argument parsing** -- mandatory DIR, validate directory exists and is under project root
2. **`.treeindexignore` loading** -- auto-create default, parse dir/file patterns
3. **Directory walking** -- collect directories to specified depth, sort deepest-first, prune ignored dirs
4. **Fingerprint computation** -- `treeindex_fingerprint()` using filenames + file sizes, excluding ignored files
5. **Staleness checking** -- compare stored vs computed fingerprint
6. **File gathering** -- collect source files, skip ignored patterns via `EXCLUDE_FILE_ARGS` array
7. **Claude invocation** -- pipe prompt to `claude -p` with Haiku model and format spec
8. **Output writing** -- prepend fingerprint header, write to centralized shadow directory
9. **Progress reporting** -- `[3/14] lib/my_app/accounts/ -- generated`

## Deviations from Original Design

| Aspect         | Original Design                  | As-Built                               |
|----------------|----------------------------------|----------------------------------------|
| DIR argument   | Optional, defaults to `.`        | Mandatory, rejects project root        |
| Default depth  | 1                                | 2                                      |
| Model          | sonnet                           | haiku (Claude Haiku 4.5)               |
| Budget cap     | $0.02/dir                        | $0.50/dir                              |
| Ignore config  | Hardcoded lists                  | `.treeindexignore` file                |
| Scope          | Global command (no project req)  | Requires Intent project context        |
| Estimated size | 250-350 lines                    | 612 lines                              |
| Tests          | Not specified                    | 38 bats tests                          |
| Bash compat    | Not specified                    | Bash 3.2 required (macOS constraint)   |

## Files Created

| File                                       | Action  | Lines |
|--------------------------------------------|---------|-------|
| `bin/intent_treeindex`                     | Created | 612   |
| `tests/unit/treeindex_commands.bats`       | Created | ~580  |
| `intent/.treeindex/.treeindexignore`       | Created | 27    |

## Acceptance Criteria

- [x] `intent treeindex --help` shows usage
- [x] `intent treeindex <dir>` generates `.treeindex` for dir and children
- [x] `intent treeindex <dir> --depth N` respects depth limit
- [x] `intent treeindex <dir> --check` reports staleness without generating
- [x] `intent treeindex <dir> --force` regenerates ignoring fingerprints
- [x] `intent treeindex <dir> --dry-run` shows plan without executing
- [x] Skips up-to-date directories (fingerprint match)
- [x] Handles directories with no source files gracefully
- [x] Progress and summary output to stderr, `.treeindex` content to files
- [ ] ~~Works as a global command~~ -- Changed: requires project context for shadow directory
