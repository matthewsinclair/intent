# Intent v2.5.0 Release Notes

**Release Date**: 24 February 2026

## Overview

Intent v2.5.0 adds work package management (`intent wp`), removes the Backlog.md integration, and applies a Highlander audit to eliminate ~350 lines of duplicated plugin code via a shared helper library.

## What's New

### Work Package Management (ST0024)

Work packages break a steel thread into smaller units of work. Each WP lives in a numbered subdirectory under `STXXXX/WP/NN/`.

**New command**: `intent wp`

```bash
intent wp new <STID> "Title"   # Create next WP (auto-assigns 01-99)
intent wp list <STID>          # Table with WP, Title, Scope, Status
intent wp start <STID/NN>      # Mark WP as WIP
intent wp done <STID/NN>       # Mark WP as Done (hints when all complete)
intent wp show <STID/NN>       # Display WP info.md
intent wp help                 # Show usage
```

Specifiers accept bare numbers: `5` = `ST0005`, `5/01` = `ST0005/01`. Titles can contain special characters (`/`, `&`, `\`) safely.

### Highlander Audit: Shared Plugin Helper Library

The skills and subagents scripts shared ~80% identical code for install/sync/uninstall/manifest operations. A new shared library eliminates the duplication:

- **Created** `intent/plugins/claude/lib/claude_plugin_helpers.sh` -- shared install, sync, uninstall, and manifest operations using a callback pattern
- **Refactored** `intent_claude_skills` (654 -> 299 lines) -- defines 8 callbacks, delegates install/sync/uninstall to shared functions
- **Refactored** `intent_claude_subagents` (1015 -> 613 lines) -- defines 8 callbacks, keeps init/list/show/status as script-specific

Each plugin script defines a small set of callbacks describing its source/target/copy semantics, then delegates operations to shared functions. Bug fixes and new features (e.g., adding `--dry-run`) now only need to be made once.

### Shared Config Extraction

Added `get_config_field()` to `bin/intent_helpers` -- a single function replacing inline `grep -oE` patterns duplicated across `intent_st` and `intent_wp`.

```bash
# Before (duplicated in intent_st and intent_wp):
AUTHOR=$(grep -oE '"author"[[:space:]]*:[[:space:]]*"[^"]+"' .intent/config.json | cut -d'"' -f4)

# After (one shared function):
AUTHOR=$(get_config_field "author" "$USER")
```

### Backlog.md Removal (ST0023)

All backlog commands and configuration removed:

- Removed `intent bl`, `intent task`, `intent status`, `intent migrate` commands
- Removed backlog configuration keys and `intent init` backlog directory creation
- Removed Node.js dependency from CI pipeline
- Deleted 3 test files; test suite reduced from 17 to 14 files

## Documentation Updates

- README.md: Added bare number syntax and special character support for WP commands
- CLAUDE.md: Added WP specifier syntax, directory structure, and special character notes
- user_guide.md: Added special character handling for WP titles and template location
- reference_guide.md: Added special character support, template location, and WP directory structure

## Testing

- 29 new BATS tests for work package commands
- Test sandbox updated to include `lib/` directory for shared helpers
- Full test suite: **339 tests** across **17 test files**
- All tests passing on macOS

## Breaking Changes

- Backlog.md commands removed (`intent bl`, `intent task`, `intent status`, `intent migrate`)
- `backlog_dir` and `backlog_list_status` configuration keys no longer recognized

## Migration

No migration required. Work package commands are additive. If you were using Backlog.md commands, those are no longer available -- use work packages (`intent wp`) as the replacement workflow.
