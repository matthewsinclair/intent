# Phase 2: Migration Implementation Plan

## Overview

Phase 2 focuses on implementing the migration functionality that will transform projects from any STP version to Intent v2.0.0. This includes the upgrade command and initial project initialization.

## Objectives

1. Implement `intent_init` for creating new v2.0.0 projects
2. Implement `intent_upgrade` for migrating existing projects
3. Create version detection logic with clear error handling
4. Implement backup mechanism
5. Build migration logic for each version
6. Test migrations on example projects

## Task Breakdown

### 1. Implement intent_init Command

**File**: `/Users/matts/Devel/prj/STP/bin/intent_init`

This command creates a new Intent v2.0.0 project structure:
- Create `.intent/config.json` with project settings
- Create `intent/` directory structure (flattened)
- Create `backlog/` directory with config.yml
- Initialize first steel thread (optional)
- Set up `.gitignore` appropriately

Key features:
- Use loaded configuration for defaults
- Allow customization via flags
- Create clean v2.0.0 structure (no legacy)
- Integration with Backlog.md

### 2. Implement intent_upgrade Command

**File**: `/Users/matts/Devel/prj/STP/bin/intent_upgrade`

This is the core migration command that handles all version upgrades:

#### 2.1 Version Detection
```bash
detect_stp_version() {
  # Check .intent/config.json (v2.0.0)
  # Check stp/.config/version (v1.2.0+)
  # Check .stp-config (v0.0.0)
  # Check directory structure patterns
  # Return version or error
}
```

#### 2.2 Backup Creation
```bash
create_backup() {
  local backup_dir=".stp_backup_$(date +%Y%m%d_%H%M%S)"
  # Copy all relevant directories
  # Create manifest of backed up files
  # Return backup location
}
```

#### 2.3 Migration Functions
```bash
migrate_v0_0_0_to_v2_0_0() {
  # Convert .stp-config to .intent/config.json
  # Move stp/prj/st/* to intent/st/
  # Flatten directory structure
  # Update file metadata
}

migrate_v1_2_0_to_v2_0_0() {
  # Convert YAML configs to JSON
  # Move file-based steel threads
  # Update directory structure
}

migrate_v1_2_1_to_v2_0_0() {
  # Move directory-based steel threads
  # Convert configs to JSON
  # Update frontmatter
}
```

#### 2.4 Command Options
- `--dry-run`: Show what would be done without changes
- `--yes`: Skip confirmation prompts
- `--verbose`: Show detailed progress
- `--backup-only`: Create backup without migrating

### 3. Update intent Main Script

**File**: `/Users/matts/Devel/prj/STP/bin/intent`

Add routing for new commands:
- `intent init [project-name]`
- `intent upgrade [options]`

### 4. Create Helper Functions

**File**: `/Users/matts/Devel/prj/STP/bin/intent_helpers`

Shared functions for migration:
- `convert_yaml_to_json()` - Convert YAML frontmatter
- `update_frontmatter()` - Change stp_version to intent_version
- `flatten_directory()` - Remove prj/ nesting
- `create_directory_structure()` - Standard v2.0.0 layout

### 5. Testing Strategy

#### 5.1 Test intent_init
- Create new project in temp directory
- Verify all directories created
- Check config.json is valid
- Ensure backlog integration works

#### 5.2 Test intent_upgrade
For each example project (v0.0.0, v1.2.0, v1.2.1):
1. Copy to temp directory
2. Run upgrade
3. Verify:
   - Backup created
   - Files moved correctly
   - Configs converted to JSON
   - No data lost
   - Commands work post-migration

#### 5.3 Edge Cases
- Empty projects
- Projects with custom structures
- Projects with invalid configs
- Interrupted migrations

## Implementation Order

1. **intent_helpers** - Shared functions
2. **intent_init** - New project creation
3. **intent_upgrade** - Migration command
4. **Update intent** - Add new command routing
5. **Test with examples** - Verify all migrations work

## Success Criteria

1. ✓ New projects created with clean v2.0.0 structure
2. ✓ All example projects migrate successfully
3. ✓ Backups created before any changes
4. ✓ Clear error messages for unknown versions
5. ✓ No data loss during migration
6. ✓ Dry-run mode shows accurate preview
7. ✓ All tests pass

## Risk Mitigation

1. **Comprehensive Backups**: Always create timestamped backup
2. **Dry Run First**: Allow preview before changes
3. **Version Detection**: Fail clearly if version unknown
4. **Atomic Operations**: Use temp files, then move
5. **Test Coverage**: Test each migration path thoroughly

## Notes

- This phase prepares for the actual repository restructuring in Phase 3
- We're building the tools that will perform the migration
- The commands work with the current structure but prepare for the new one
- Focus on getting migrations working perfectly before moving files