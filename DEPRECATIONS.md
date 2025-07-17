---
verblock: "17 Jul 2025:v0.2: Matthew Sinclair - Updated for Intent v2.0.0"
intent_version: 2.0.0
---
# Intent Deprecations

This document tracks features, files, and functionality that have been deprecated in Intent (formerly STP).

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

Intent version 2.0.0
