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

⏺ Comprehensive Restart Prompt for Intent v2.0.0 Implementation

# Intent v2.0.0 Implementation Session

  I need you to implement the Intent v2.0.0 upgrade as specified in ST0016. This is a major refactoring of the STP (Steel Thread
  Process) CLI tool, renaming it from "stp" to "intent" with significant architectural improvements.

## Critical Context

  1. **Current Location**: `/Users/matts/Devel/prj/STP/`
  2. **Steel Thread**: ST0016 is currently In Progress at `stp/prj/st/ST0016/`
  3. **Approach**: FAIL-FORWARD - No rollback mechanisms, we fix issues as we go
  4. **User**: Single user (Matthew Sinclair) who understands the changes

## Your Primary References

  Read these files FIRST to understand the complete plan:

- `stp/prj/st/ST0016/info.md` - Overview and objectives
- `stp/prj/st/ST0016/design.md` - Detailed design with all phases
- `stp/prj/st/ST0016/impl.md` - Technical implementation details
- `stp/prj/st/ST0016/tasks.md` - Task breakdown by phase

## Implementation Order (CRITICAL - FOLLOW EXACTLY)

### Phase 0: Test Infrastructure (DO THIS FIRST!)

  Before ANY other implementation:

  1. Create `examples/` directory at project root
  2. Create test fixtures:
     - `examples/v0.0.0-project/` - Ancient .stp-config format
     - `examples/v1.2.0-project/` - File-based steel threads
     - `examples/v1.2.1-project/` - Directory-based steel threads
     - `examples/hello-world/` - Clean v2.0.0 structure
  3. Write comprehensive BATS test suite in `tests/upgrade/`
  4. Document expected test outcomes
  5. Create test harness for migration scenarios

### Phase 1: New Commands

  1. Implement `intent bootstrap` command:
     - Auto-detect INTENT_HOME
     - Create global config at `~/.config/intent/config.json`
     - Provide PATH setup instructions
  2. Implement `intent doctor` command:
     - Configuration diagnostics
     - JSON validation
     - --fix mode for auto-repairs

### Phase 2: Configuration System

  1. Implement JSON config parsing (NO external dependencies)
  2. Config loading hierarchy:
     - Global: `~/.config/intent/config.json` (XDG standard)
     - Local: `.intent/config.json`
     - Environment variables override all
  3. Project root detection logic

### Phase 3: Repository Restructuring

  1. Move `stp/bin/*` → `bin/`
  2. Rename executables:
     - `stp` → `intent`
     - `stp_*` → `intent_*`
     - Create `stp` → `intent` symlink
  3. Move `stp/_templ/` → `lib/templates/`
  4. Flatten project structure:
     - `stp/prj/st/` → `intent/st/`
     - `stp/eng/` → `intent/eng/`
     - `stp/usr/` → `intent/ref/`

### Phase 4: Upgrade Command

  1. Implement version detection with clear error handling
  2. Create backup mechanism (timestamp-based)
  3. Migration logic for each version:
     - v0.0.0 → v2.0.0
     - v1.2.0 → v2.0.0
     - v1.2.1 → v2.0.0
  4. Convert configs to JSON format
  5. Update all documentation files

### Phase 5: Command Updates

  Update all commands to use:

- Configured directory names ($INTENT_DIR)
- New flattened structure (st/ not prj/st/)
- JSON config loading

### Phase 6: Documentation

  1. Update README.md
  2. Update CHANGELOG.md
  3. Create migration guide
  4. Update all command docs

## Key Technical Details

### JSON Config Format

  ```json
  {
    "intent_version": "2.0.0",
    "intent_dir": "intent",
    "backlog_dir": "backlog",
    "author": "Matthew Sinclair",
    "editor": "vim"
  }

  Directory Structure (Final)

  $INTENT_HOME/
  ├── .intent/              # Local config for intent-on-itself
  │   └── config.json
  ├── bin/                  # Tool executables
  │   ├── intent
  │   ├── intent_*
  │   └── stp -> intent
  ├── lib/                  # Tool resources
  │   └── templates/
  ├── intent/               # Project artifacts
  │   ├── st/              # Steel threads (flattened)
  │   ├── eng/
  │   ├── ref/             # Was usr/
  │   ├── llm/
  │   └── _archive/
  ├── backlog/             # Backlog.md directory
  ├── examples/            # Test projects
  ├── docs/                # Tool documentation
  └── tests/               # Tool tests

  Critical Implementation Notes

  1. Version Detection: If unable to determine version, FAIL with clear error message
  2. Config Parsing: Use simple sed/grep for JSON (no jq dependency)
  3. Error Messages: Must be clear and actionable
  4. Testing: Create tests BEFORE implementation
  5. Self-Hosting: Remember that intent uses STP on itself (meta usage)

  Session Goals

  1. Complete Phase 0 (test infrastructure) entirely
  2. Begin Phase 1 (new commands) if time permits
  3. Use Backlog.md to track detailed tasks: stp task create ST0016 "<task>"
  4. Update ST0016 files as implementation progresses
  5. Commit changes with clear messages

  Important Reminders

  - The tool is being renamed from "stp" to "intent"
  - The Steel Thread Process (STP) methodology remains unchanged
  - ST#### numbering continues as before
  - This is a fail-forward implementation - no rollback needed
  - Test everything thoroughly before proceeding to next phase

  Start by reading all ST0016/*.md files, then begin with Phase 0. Good luck!

  This restart prompt provides comprehensive instructions for implementing Intent v2.0.0, following the exact specifications in
  ST0016. It emphasizes the critical Phase 0 (test infrastructure first) approach and provides all necessary context for a fresh
  session.
