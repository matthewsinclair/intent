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

⏺ Restart Prompt for New Claude Code Session

# Intent v2.0.0 - Technical Product Design Update Session

## Project Context

  You are working on the Intent project (formerly STP - Steel Thread Process), which has undergone a major rebrand and
  restructuring to v2.0.0. The project is located at `/Users/matts/Devel/prj/Intent`.

## Current State (as of 2025-01-17)

- **Intent v2.0.0 migration is COMPLETE** (ST0016)
- **Blog posts have been restored** to `docs/blog/` with all STP references updated to Intent
- **Old stp/ directory has been removed**
- **86 tests passing** (down from 186 - see tests/RECOVERY_PLAN.md)
- **Project is self-hosting** - using Intent v2.0.0 structure

## Recent Accomplishments

  1. Restored 7 blog posts from git history to `docs/blog/`
  2. Updated all "STP" references to "Intent" throughout blog posts
  3. Fixed internal blog post links (e.g., `0000-motivation-for-stp.md` → `0000-motivation-for-intent.md`)
  4. Deleted entire `stp/` directory (contained only old tests)
  5. Updated README.md blog links to point to new location
  6. Fixed YAML frontmatter conversion bug in migration script
  7. Implemented `backlog_list_status` filtering for `intent bl list` command

## Key Implementation Details

- **Commands**: All renamed from `stp_*` to `intent_*`
- **Config**: JSON-based (`.intent/config.json`) instead of YAML
- **Structure**: Flattened to `intent/` instead of nested `stp/prj/`
- **Backlog**: Enhanced with status filtering via `backlog_list_status` config
- **Bootstrap**: New `intent bootstrap` command for global setup
- **Doctor**: `intent doctor` with `--fix` capability
- **Upgrade**: `intent upgrade` for migrating legacy projects

## Your Mission

  **FORENSICALLY update the Technical Product Design (TPD)** at `intent/eng/tpd/technical_product_design.md` to reflect the FULL
  as-built status of the Intent v2.0.0 framework. The TPD is currently outdated and references the old STP structure.

## Key Areas to Document in TPD

  1. **Actual v2.0.0 implementation** vs original plan
  2. **New features added** (bootstrap, doctor, upgrade commands)
  3. **Configuration system** (JSON format, hierarchy, new fields like `backlog_list_status`)
  4. **Directory structure changes** (before/after comparison)
  5. **Command naming conventions** (intent_* pattern)
  6. **Backlog.md integration enhancements**
  7. **Test coverage status** (86 tests, what was lost)
  8. **Migration tooling** (how upgrade works)
  9. **Self-hosting success** (Intent built with Intent)
  10. **Blog series completion** (ST0013)

## Important Notes

- Always refer to the tool as "Intent" not "STP"
- The methodology is still "Steel Thread Process" but the tool is "Intent"
- Check CLAUDE.md for project-specific instructions
- This is a fail-forward implementation - no rollback mechanisms

## First Steps

  1. Read the current TPD at `intent/eng/tpd/technical_product_design.md`
  2. Compare it with actual implementation in `bin/` directory
  3. Review test coverage and what's actually working
  4. Create a comprehensive update plan before making changes
