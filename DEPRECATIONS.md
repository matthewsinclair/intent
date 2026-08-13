---
verblock: "14 Aug 2026:v0.5: Matthew Sinclair - Updated for Intent v2.19.0"
intent_version: 2.19.0
---

# Intent Deprecations

This document tracks features, files, and functionality that have been deprecated in Intent (formerly STP).

## August 2026 (v2.19.0): free-form acceptance-test references

### What was deprecated

The AT row's reference field had no grammar. In practice that admitted a test name, a bare filename, a `path::name` selector (taught by the tool's own doc comment and by the shipped `acceptance.md` template), a bare parenthetical status note, and — because the field was defined as _whatever sits inside the first pair of backticks_ — an empty string. All of them are now rejected. A row must match one of two anchored shapes:

```
- AT-<gg>.<n> `<repo-relative-path>` -- covers <AC-id>[, <AC-id>...] -- status: to-write|red|green[ -- <free note>]
- AT-<gg>.<n> (non-test) <prose> -- covers <AC-id>[, <AC-id>...] -- status: n/a[ -- <free note>]
```

### Why it was deprecated

A field with no grammar cannot fail to parse; it can only be partially recovered, silently, one piece at a time. On the estate that surfaced this, that produced five mutually incompatible reference forms across 314 rows and two live `green` acceptance tests citing CSS utility classes as their test files, with no diagnostic anywhere. Coverage that cannot be resolved was still being counted as coverage.

### Migration path

`intent at lint <ID> --fix` migrates the mechanical half (backticking a bare path, stripping a `::name` suffix, converting `and` separators to commas, delimiting a trailing note). Rows needing a human judgement are reported by name and never guessed at. `intent upgrade` runs the same sweep, so a consumer is migrated by upgrading rather than by knowing the command exists.

### Impact

- A contract written against the old convention gates **BLOCKED** until it is swept. This is deliberate: every row the linter names was already contributing no coverage.
- `intent at green|red` refuse a citation that does not resolve, at the moment it goes load-bearing rather than at the next gate.
- The `path::name` form is gone from `lib/templates/prj/st/ST####/acceptance.md` and from the tool's own doc comment. Historical steel-thread documents still contain it, as the record of what was true at the time.

## February 24, 2026: Backlog.md integration removed

### What was deprecated

The Backlog.md integration and all associated commands (`intent bl`, `intent task`, `intent migrate`) have been removed.

### Why it was deprecated

Backlog.md task management added complexity without sufficient value. Steel threads and their associated task files (`tasks.md`, `done.md`) provide adequate tracking for Intent's workflow. Removing the Backlog integration simplifies the codebase and reduces maintenance burden.

### Migration path

Users who were using Backlog.md should:

1. Move any active tasks into steel thread `tasks.md` files
2. Archive historical task data if needed
3. Remove `Backlog.md` from project roots

### Impact

- The `intent bl` command has been removed
- The `intent task` command has been removed
- The `intent migrate` command has been removed
- Associated test files (`bl_commands.bats`, `task_commands.bats`, `migration.bats`) have been removed
- Documentation updated to remove Backlog references

### Version deprecated

Intent version 2.5.0

## July 9, 2025: journal.md

### What was deprecated

The `stp/prj/journal.md` file and associated functionality.

### Why it was deprecated

With the integration of Backlog.md for task management, the journal.md file became redundant. Backlog provides:

- Better structured task tracking with metadata (status, priority, dependencies)
- Automatic linking to steel threads
- More flexible historical tracking through task history
- Integration with modern development workflows

### Migration path

Users who were using journal.md for historical tracking should:

1. Use `stp bl list` to view task history
2. Use steel thread documents for high-level context and decisions
3. Use Backlog task descriptions for detailed implementation notes

### Where to find the deprecated content

The original journal.md file has been archived at `stp/prj/archive/journal-deprecated.md` with a deprecation notice.

### Impact

- The `stp init` command no longer creates journal.md
- Documentation has been updated to reference Backlog for historical tracking
- The journal.md template has been removed from `_templ/prj/`

### Version deprecated

STP version 1.0.0

## July 17, 2025: STP → Intent Rebrand

### What was deprecated

The entire STP (Steel Thread Process) command and naming convention has been deprecated in favour of Intent.

### Why it was deprecated

The name "Intent" better reflects the tool's core purpose of capturing and preserving the intention behind software development decisions. The rebrand includes:

- Better alignment with the tool's philosophy
- Clearer separation between tool and methodology
- Improved directory structure with flattened hierarchy
- Modern JSON-based configuration system

### Migration path

Users migrating from STP to Intent should:

1. Run `intent upgrade` to automatically migrate existing projects
2. Update PATH to point to the new bin/ directory
3. Use `intent` command instead of `stp` (symlink provided for compatibility)
4. Update any scripts or documentation referencing `stp` commands

### Specific deprecations

#### Commands

- `stp` → `intent` (all subcommands remain the same)
- `stp init` → `intent init`
- `stp st` → `intent st`
- `stp bl` → `intent bl`
- `stp task` → `intent task`
- `stp status` → `intent status`
- `stp migrate` → `intent migrate`
- `stp upgrade` → `intent upgrade`

#### Directory Structure

- `stp/prj/st/` → `intent/st/`
- `stp/prj/wip.md` → `intent/wip.md`
- `stp/eng/` → `intent/eng/`
- `stp/usr/` → `intent/usr/`
- `stp/bin/` → `bin/` (moved to top level)

#### Configuration

- YAML format → JSON format
- `.stp/config.yml` → `.intent/config.json`
- No global config → `~/.config/intent/config.json` (XDG standard)

### Where to find deprecated content

- Original STP executables remain in the repository for reference
- Migration is handled automatically by `intent upgrade`
- Backwards compatibility maintained through symlinks

### Impact

- All new projects should use Intent commands and structure
- Existing projects can continue using `stp` via compatibility symlink
- Documentation has been updated to use Intent terminology
- Repository renamed from `stp` to `intent`

### Version deprecated

Intent version 2.1.0
