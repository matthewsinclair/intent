# Phase 1: New Commands Implementation Plan

## Overview

Phase 1 focuses on implementing the new Intent v2.0.0 commands in the top-level `bin/` directory. These commands provide essential functionality for the new architecture.

## Objectives

1. Create top-level `bin/` directory structure
2. Implement `intent_bootstrap` command for initial setup
3. Implement `intent_doctor` command for diagnostics
4. Create shared `intent_config` library for JSON parsing
5. Ensure all commands work without external dependencies

## Task Breakdown

### 1. Create Directory Structure

```bash
mkdir -p /Users/matts/Devel/prj/STP/bin
mkdir -p /Users/matts/Devel/prj/STP/lib
```

### 2. Implement intent_config Library

**File**: `/Users/matts/Devel/prj/STP/bin/intent_config`

This shared library will provide:
- `parse_json()` function using sed/grep (no jq dependency)
- `load_intent_config()` function for config hierarchy
- `find_project_root()` function for project detection
- Common variables and defaults

Key features:
- Parse JSON without external tools
- Handle global → local → environment variable precedence
- Support legacy STP project detection
- Export configuration for use by other commands

### 3. Implement intent_bootstrap Command

**File**: `/Users/matts/Devel/prj/STP/bin/intent_bootstrap`

Functionality:
1. **Auto-detect INTENT_HOME**:
   - If not set, crawl up from script location
   - Look for bin/intent and lib/ directory
   - Validate the installation

2. **Create global config**:
   - Create `~/.config/intent/` directory (XDG standard)
   - Generate default `config.json` if not exists
   - Use current user and editor from environment

3. **PATH setup**:
   - Display clear instructions for shell configuration
   - Show export commands for INTENT_HOME and PATH

4. **Verification**:
   - Run `intent doctor` to verify setup
   - Display success message

### 4. Implement intent_doctor Command

**File**: `/Users/matts/Devel/prj/STP/bin/intent_doctor`

Checks to perform:
1. **INTENT_HOME**: Set and valid directory exists
2. **Executables**: intent binary exists and is executable
3. **Global config**: Exists and has valid JSON syntax
4. **Local config**: If in project, check syntax
5. **PATH**: Verify $INTENT_HOME/bin is in PATH
6. **Permissions**: Check file permissions
7. **Dependencies**: Verify required tools (bash, sed, grep)

Features:
- Normal mode: Report issues
- `--fix` mode: Attempt automatic repairs
- Summary with error/warning counts
- Exit codes: 0 for success, 1+ for errors

### 5. Create Compatibility Wrapper

**File**: `/Users/matts/Devel/prj/STP/bin/intent`

This will be a copy of the current `stp` script, modified to:
- Load the new config system
- Detect if called as 'stp' for compatibility warnings
- Route to intent_* subcommands
- Support both old and new project structures during transition

### 6. Testing Strategy

After implementing each command:

1. **Unit tests**: Run the BATS tests we created in Phase 0
2. **Integration tests**: 
   - Test bootstrap on clean system
   - Test doctor with various configurations
   - Test config loading hierarchy
3. **Example project tests**:
   - Verify commands work with v2.0.0 hello-world project
   - Ensure legacy detection works with older examples

## Implementation Order

1. **intent_config** (foundation for other commands)
2. **intent_bootstrap** (needed for initial setup)
3. **intent_doctor** (validates bootstrap worked)
4. **intent** (main wrapper, minimal changes from stp)

## File Permissions

All executables will need:
```bash
chmod +x /Users/matts/Devel/prj/STP/bin/intent*
```

## Success Criteria

1. ✓ Bootstrap creates valid global config
2. ✓ Doctor correctly identifies all issues
3. ✓ Doctor --fix repairs common problems
4. ✓ Config loading respects hierarchy
5. ✓ JSON parsing works without jq
6. ✓ All BATS tests pass
7. ✓ Commands work on example projects

## Risk Mitigation

1. **No external dependencies**: Use only bash built-ins and standard Unix tools
2. **Backwards compatibility**: Detect legacy structures
3. **Clear error messages**: Help users understand issues
4. **Atomic operations**: Don't leave system in broken state
5. **Backup before modify**: Doctor --fix backs up files

## Notes

- These commands will initially coexist with stp/bin/* commands
- The actual migration (moving stp/bin/* to bin/) happens in Phase 3
- Focus on getting the new commands working perfectly first
- Use the implementation details from ST0016/impl.md as reference

## Next Steps After Phase 1

Once these commands are working:
- Phase 2: Implement full configuration system
- Phase 3: Repository restructuring with intent_upgrade
- Phase 4: Update all existing commands