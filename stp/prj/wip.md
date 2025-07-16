---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.2.0
---
# Work In Progress

This file serves as a placeholder for kicking off new sessions.

See the following files for detailed information about the project:

- [Technical Product Design](../eng/tpd/technical_product_design.md)
- [Steel Threads Overview](st/steel_threads.md)

Read CLAUDE.md then wait for instruction.

#### Todo

# Intent v2.0.0 Migration - Session Restart Prompt

## Context

You are working on the Intent project (formerly STP - Steel Thread Process) located at `/Users/matts/Devel/prj/Intent`. This project has been successfully migrated from STP v1.2.1 to Intent v2.0.0.

## Project State

### Migration Completed (ST0016)

- Successfully migrated from STP v1.2.1 to Intent v2.0.0
- All 16 steel threads have been migrated and organized by status
- Project structure has been updated from `stp/` to `intent/`
- Configuration converted from YAML to JSON format

### Current Directory Structure

```
/Users/matts/Devel/prj/Intent/
├── bin/                    # Intent CLI executables
│   ├── intent              # Main entry point
│   ├── intent_*            # Subcommands
│   └── stp -> intent       # Backward compatibility symlink
├── intent/                 # Main project content (migrated from stp/)
│   ├── st/                 # Steel threads organized by status
│   │   ├── COMPLETED/      # 12 completed STs (ST0001-ST0009, ST0012-ST0014)
│   │   ├── NOT-STARTED/    # 3 not-started STs (ST0010, ST0011, ST0015)
│   │   ├── CANCELLED/      # Empty
│   │   └── ST0016/         # Active ST (In Progress)
│   ├── eng/tpd/            # Technical Product Design docs
│   ├── llm/                # LLM guidelines
│   ├── ref/                # Reference materials
│   └── wip.md              # Work in progress tracking
├── lib/                    # Intent libraries and templates
│   └── templates/          # Project templates (migrated from stp/_templ)
├── stp/                    # Legacy STP structure (preserved, can be removed)
├── backlog/                # Backlog.md integration
├── .intent/                # Intent configuration
│   └── config.json         # Project configuration (v2.0.0)
└── CLAUDE.md               # Project guidelines for Claude

### Configuration
- **INTENT_HOME**: `/Users/matts/Devel/prj/Intent`
- **Project Version**: Intent v2.0.0
- **Active Steel Thread**: ST0016 (Rename STP CLI to INTENT)

### Key Commands Available
- `intent` - Main command (requires PATH setup or symlink)
- `intent st` - Manage steel threads
- `intent organise` - Organize steel threads by status
- `intent upgrade` - Upgrade projects to Intent v2.0.0
- `intent doctor` - Check configuration
- `intent help` - Get help

### Recent Work Completed
1. Fixed migration script to properly handle all steel thread directories
2. Created `intent_organise` command to organize STs by status
3. Successfully migrated all 16 steel threads with correct directory structure
4. Preserved backward compatibility through `stp` symlink

### Known Issues Resolved
- Intent command not working from PATH - needs either:
  - Symlink: `sudo ln -sf /Users/matts/Devel/prj/Intent/bin/intent /usr/local/bin/intent`
  - Or PATH: `export PATH="/Users/matts/Devel/prj/Intent/bin:$PATH"`

### Next Potential Tasks
- Remove legacy `stp/` directory if no longer needed
- Test all Intent commands to ensure they work correctly
- Update any remaining documentation for Intent v2.0.0
- Complete ST0016 implementation phases

## Instructions

Please read the CLAUDE.md file to understand the project guidelines, then wait for specific instructions on what to work on next.
