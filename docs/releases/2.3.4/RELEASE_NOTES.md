# Intent v2.3.4 Release Notes

**Release Date:** February 4, 2026

## Overview

Intent v2.3.4 introduces the `intent treeindex` command for generating LLM-optimized directory summaries, along with bug fixes and Elixir subagent enhancements.

## New Features

### Treeindex CLI (ST0019 WP01)

The `intent treeindex DIR` command generates concise `.treeindex` summary files for each directory, designed to help LLMs navigate unfamiliar codebases without reading every file.

**Key capabilities:**

- **Bottom-up indexing**: Leaf directories first, then parents (child summaries feed into parent context)
- **Claude Haiku 4.5**: Fast, cost-effective summarization via headless `claude -p`
- **Centralized shadow directory**: Index files stored at `intent/.treeindex/` to keep the source tree clean
- **Fingerprint-based staleness**: Detects adds/removes/size changes without mtime dependency (stable across git clones)
- **`.treeindexignore`**: Gitignore-style config for excluding files and directories from indexing
- **Platform compatible**: Handles macOS/Linux `stat` differences, works with Bash 3.2

**Usage:**

```bash
intent treeindex lib/          # Index lib/ and subdirectories (depth 2)
intent treeindex --depth 3 src/  # Index deeper
intent treeindex --check lib/    # Report staleness without regenerating
intent treeindex --dry-run lib/  # Preview what would be generated
intent treeindex --force lib/    # Regenerate even if up-to-date
```

**CLI options:**

| Option          | Description                              |
| --------------- | ---------------------------------------- |
| `--depth N`     | Directory traversal depth (default: 2)   |
| `--check`       | Report staleness without generating      |
| `--dry-run`     | Preview plan without writing files       |
| `--force`       | Regenerate even when fingerprint matches |
| `--model MODEL` | Claude model to use (default: haiku)     |
| `--verbose`     | Show detailed progress                   |

### CLAUDE.md Convention (ST0019 WP03)

CLAUDE.md now instructs Claude to check `.treeindex` files before exploring unfamiliar directories, avoiding redundant Glob/Grep/Read operations. The `intent treeindex <dir>` command is listed under Core Commands.

## Bug Fixes

- **`intent init` version display**: Now reads from VERSION file instead of showing hardcoded 2.0.0
- **`--sync` flag**: Fixed bug in steel thread synchronization

## Enhancements

- Expanded Elixir subagent with architectural principles, Ash/Phoenix patterns, and testing guidance
- Standardized abbreviations throughout documentation

## Upgrade Notes

### From v2.3.3

To upgrade from v2.3.3 to v2.3.4:

```bash
intent upgrade
```

If you have the Elixir subagent installed, update it:

```bash
intent claude subagents sync
```

### Migration Details

The v2.3.3 -> v2.3.4 migration:

1. Updates project version in `.intent/config.json` and `VERSION`
2. No structural changes required
3. The treeindex command is available immediately after upgrade
4. Elixir subagent enhancements are available via `intent claude subagents sync`

## Compatibility

- Fully compatible with Intent v2.3.0 through v2.3.3
- No breaking changes
- Existing projects can upgrade seamlessly
- Bash 3.2+ required (macOS default is compatible)

## Files Changed

- `bin/intent_treeindex` (new, 612 lines)
- `tests/unit/treeindex_commands.bats` (new, ~580 lines, 38 tests)
- `intent/.treeindex/.treeindexignore` (new, default ignore patterns)
- `VERSION` (2.3.3 -> 2.3.4)
- `.intent/config.json` (version fields updated)
- `bin/intent_helpers` (added migration functions)
- `bin/intent_upgrade` (added v2.3.4 upgrade paths)

## For More Information

- **Full Changelog**: See CHANGELOG.md
- **Steel Thread**: `intent/st/ST0019/` (Treeindex design and implementation)
- **Treeindex Design**: `intent/st/ST0019/design.md`
- **Treeindex Implementation**: `intent/st/ST0019/impl.md`
