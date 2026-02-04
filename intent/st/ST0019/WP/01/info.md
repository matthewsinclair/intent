---
verblock: "03 Feb 2026:v0.1: matts - Initial version"
wp: "01"
title: "intent treeindex CLI command"
status: Not Started
---
# WP01: `bin/intent_treeindex` CLI Command

## Objective

Create the `intent treeindex` CLI command that walks directories bottom-up and uses `claude -p` (headless mode) to generate `.treeindex` summary files.

## Scope

### CLI Interface

```
intent treeindex [OPTIONS] [DIR]

OPTIONS:
  -d, --depth N     Depth to traverse (default: 1, ie dir + immediate children)
  --check           Check staleness only, don't generate
  --force           Regenerate ignoring fingerprints
  --model MODEL     Claude model to use (default: sonnet)
  --dry-run         Show what would be generated without doing it
  -h, --help        Show help

EXAMPLES:
  intent treeindex lib/my_app/           # dir + 1 level deep
  intent treeindex lib/ --depth 3        # 3 levels deep
  intent treeindex lib/ --check          # report stale indexes
  intent treeindex lib/my_app/ --force   # regenerate everything
```

### Core Functions

1. **Argument parsing** -- handle options, validate directory exists
2. **Directory walking** -- collect directories to specified depth, sort deepest-first
3. **Fingerprint computation** -- `treeindex_fingerprint()` using filenames + file sizes
4. **Staleness checking** -- compare stored vs computed fingerprint
5. **File gathering** -- collect source files, skip ignored patterns
6. **Claude invocation** -- pipe file contents to `claude -p` with format prompt
7. **Output writing** -- prepend fingerprint header, write `.treeindex`
8. **Progress reporting** -- `[3/14] lib/my_app/accounts/ -- generated`

### Ignore Lists

Directories: `_build/`, `deps/`, `node_modules/`, `.git/`, `.elixir_ls/`, `cover/`, `priv/static/assets/`

Files: `.DS_Store`, `.treeindex`, `*.beam`, `erl_crash.dump`, `*.ez`

## Files to Create/Modify

| File                   | Action                             |
|------------------------|------------------------------------|
| `bin/intent_treeindex` | Create (~250-350 lines bash)       |
| `bin/intent` line 41   | Add `treeindex` to GLOBAL_COMMANDS |

## Dependencies

- `claude` CLI must be installed and authenticated
- `shasum` (macOS) or `sha256sum` (Linux) for fingerprinting

## Acceptance Criteria

- [ ] `intent treeindex --help` shows usage
- [ ] `intent treeindex <dir>` generates `.treeindex` for dir and 1 level of children
- [ ] `intent treeindex <dir> --depth N` respects depth limit
- [ ] `intent treeindex <dir> --check` reports staleness without generating
- [ ] `intent treeindex <dir> --force` regenerates ignoring fingerprints
- [ ] `intent treeindex <dir> --dry-run` shows plan without executing
- [ ] Skips up-to-date directories (fingerprint match)
- [ ] Handles directories with no source files gracefully
- [ ] Progress and summary output to stderr, `.treeindex` content to files
- [ ] Works as a global command (no project context required)
