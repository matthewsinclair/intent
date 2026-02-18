---
verblock: "16 Jul 2025:v0.2: Matthew Sinclair - Updated with JSON config and new commands"
stp_version: 2.0.0
---

# ST0016: Design Document

## Overview

This design document details the comprehensive refactoring of the CLI tool from "stp" to "intent", addressing architectural concerns and modernizing the tool structure.

**Key Terminology**:

- **intent**: The command-line tool (lowercase)
- **STP**: Steel Thread Process methodology (unchanged)
- **intent\_\***: Subcommands following the new naming convention
- **Fail-forward approach**: Direct migration to v2.0.0 without incremental steps

## Phase 0: Test Infrastructure (Foundation)

### 0.1 Create Example Projects FIRST

Before any implementation, create comprehensive test fixtures:

```
examples/
├── v0.0.0-project/     # Ancient .stp-config format
├── v1.2.0-project/     # File-based steel threads
├── v1.2.1-project/     # Directory-based steel threads
└── hello-world/        # Clean v2.0.0 structure
```

### 0.2 Test Suite Development

- Comprehensive BATS test suite
- Migration scenario tests
- Self-hosting test cases
- Performance benchmarks
- Error condition tests

### 0.3 Documentation Templates

- Migration guide template
- Troubleshooting guide structure
- Release notes format

## Phase 1: Repository Restructuring

### 1.1 Current Structure Problems

```
stp/
├── bin/        # WRONG: Tool executables mixed with project artifacts
├── prj/st/     # UNNECESSARILY NESTED: Steel threads
├── eng/        # Project artifacts
└── _templ/     # Tool resources
```

Problems identified:

- Tool executables (stp/bin/) mixed with project artifacts
- Unnecessary nesting (prj/st/ instead of just st/)
- Unclear separation of concerns
- Confusing for users and deployment

### 1.2 New Clean Structure

```
$INTENT_HOME/                    # The intent tool repository
├── .intent/                     # LOCAL config for intent-on-itself
│   └── config.yml
├── bin/                        # Tool executables (moved from stp/bin/)
│   ├── intent
│   ├── intent_*
│   └── stp -> intent           # Backwards compatibility
├── lib/                        # Tool resources (was stp/_templ/)
│   └── templates/
├── intent/                     # Project artifacts (was stp/)
│   ├── st/                     # Steel threads (flattened from prj/st/)
│   ├── eng/
│   ├── ref/                    # Reference docs (renamed from usr/)
│   ├── llm/
│   └── _archive/
├── backlog/                    # Existing Backlog.md directory
├── examples/                   # NEW: Example projects for testing
│   └── hello-world/            # Model project with full structure
├── docs/                       # Tool documentation
└── tests/                      # Tool tests
```

Benefits:

- Clear separation: tool (bin/, lib/) vs usage (intent/, backlog/)
- Flattened structure (st/ not prj/st/)
- Intuitive organization
- Easy deployment (just copy bin/ and lib/)

## Phase 2: Configuration System

### 2.1 Config Locations

- **Local**: `.intent/config.json` (project-specific)
- **Global**: `~/.config/intent/config.json` (XDG standard)

### 2.2 Config Format

```json
{
  "intent_version": "2.0.0",
  "intent_dir": "intent",
  "backlog_dir": "backlog",
  "author": "Matthew Sinclair",
  "editor": "vim"
}
```

### 2.3 Config Loading

**Loading Order**:

1. Load global config first (`~/.config/intent/config.json`)
2. Overlay local config (`.intent/config.json`)
3. Apply environment variable overrides (highest priority)

**Project Detection**:

```bash
find_project_root() {
  current_dir=$(pwd)
  while [ "$current_dir" != "/" ]; do
    # New structure
    if [ -f "$current_dir/.intent/config.json" ]; then
      echo "$current_dir"
      return 0
    fi
    # Legacy structures
    if [ -d "$current_dir/stp/.config" ] || [ -f "$current_dir/.stp-config" ]; then
      echo "$current_dir"
      return 0
    fi
    current_dir=$(dirname "$current_dir")
  done
  return 1
}
```

### 2.4 New Commands

#### intent bootstrap

- Initial setup for new installations
- Creates global config directory
- Sets up PATH recommendations
- Detects or uses $INTENT_HOME
- Validates installation

#### intent doctor

- Configuration diagnostics
- Validates JSON syntax
- Checks for missing dependencies
- Suggests fixes for common issues
- Can auto-fix with --fix flag

## Phase 3: Model Project & Testing

### 3.1 Create Example Project

```
examples/hello-world/
├── .intent/
│   └── config.json         # Example config (JSON)
├── intent/
│   ├── st/
│   │   ├── ST0001/         # Example steel thread
│   │   └── ST0002/
│   ├── eng/
│   │   └── tpd/
│   └── ref/
└── backlog/
    └── config.yml         # Backlog.md config (unchanged)
```

This serves as:

- Testing ground for upgrade scenarios
- Example for new users
- Reference implementation
- Regression test baseline

### 3.2 Upgrade Testing Strategy

Test scenarios:

1. **v0.0.0 → v2.0.0**: Ancient .stp-config format
2. **v1.2.0 → v2.0.0**: File-based steel threads
3. **v1.2.1 → v2.0.0**: Directory-based steel threads

Validation tests:

- All files migrated correctly
- No data loss
- Commands work post-migration
- Rollback capability
- Config format conversion

### 3.3 Test Implementation

```bash
# tests/upgrade/comprehensive_test.bats
@test "upgrade from v1.2.1 to v2.0.0" {
  # Setup test project
  cp -r examples/v1.2.1-project "$TEST_DIR/project"
  cd "$TEST_DIR/project"

  # Take snapshot
  find . -type f | sort > before.txt

  # Run upgrade
  run intent upgrade --yes

  # Verify structure
  assert_success
  assert [ -f ".intent/config.json" ]
  assert [ -d "intent/st" ]
  assert [ ! -d "stp/prj/st" ]

  # Verify no data loss
  # ... detailed checks
}
```

## Phase 4: Migration Implementation

### 4.1 Upgrade Command Enhancement

```
intent upgrade [--dry-run] [--yes]
```

Migration steps:

1. **Detect Version**:
   - Check stp_version in known locations
   - If unable to determine: fail with clear error message
   - No assumptions about unknown versions

2. **Backup**: Create .backup/ with timestamp

3. **Migrate Structure**:

   ```
   Old                          New
   stp/bin/*                 → bin/*
   stp/_templ/*              → lib/templates/*
   stp/prj/st/*              → intent/st/*
   stp/eng/*                 → intent/eng/*
   stp/usr/*                 → intent/ref/*
   stp/.config/*             → .intent/* (with format conversion)
   .stp-config               → .intent/config.json
   ```

4. **Update Configs**: Convert to JSON format
5. **Verify**: Run validation checks
6. **Update Documentation**: README, CHANGELOG, etc.
7. **Cleanup**: Remove old structure

### 4.2 Failure Handling

**Fail-forward approach**:

- No rollback mechanism (not needed)
- Clear error messages on failure
- Backup available for manual recovery if needed
- Focus on getting it right the first time through comprehensive testing

## Phase 5: Command Updates

### 5.1 Main Script

Updates required:

- Detect invocation name (stp vs intent)
- Load config with new hierarchy
- Use configured directory names
- Support both old and new structures

### 5.2 All Subcommands

Changes for each command:

- Use `$INTENT_DIR` instead of hardcoded "stp"
- Remove hardcoded path assumptions
- Use flattened structure (st/ not prj/st/)
- Maintain backwards compatibility

## Phase 6: Documentation

### 6.1 Updates Required

- All command examples use "intent"
- Directory structure documentation
- Migration guide with examples
- Troubleshooting section
- Configuration reference

### 6.2 Backwards Compatibility Notes

Important clarifications:

- ST#### numbering remains (Steel Thread Process continues)
- "stp" → "intent" is just the CLI tool name
- Existing projects continue working
- Migration is optional but recommended

## Phase 7: Bootstrap Strategy

### 7.1 New User Flow

```bash
# Clone the repository
git clone https://github.com/user/intent.git
cd intent

# Option 1: Set INTENT_HOME explicitly
export INTENT_HOME=$(pwd)
./bin/intent bootstrap

# Option 2: Let bootstrap detect location
./bin/intent bootstrap
# Bootstrap will crawl up from current location to find intent directory
```

### 7.2 Bootstrap Command Tasks

1. Detect or validate $INTENT_HOME
2. Create global config directory: `~/.config/intent/`
3. Generate initial global config
4. Add bin/ to PATH recommendations
5. Validate installation
6. Run `intent doctor` to verify

## Additional Considerations

### Error Handling Strategy

- Clear, actionable error messages
- Specific version detection failures
- Migration interruption detection
- Config validation errors
- Dependency check failures

### Performance Optimization

- Config caching within session
- Efficient directory traversal
- Minimal overhead on command execution

### Integration Updates

- CI/CD pipeline modifications
- GitHub Actions workflow updates
- Documentation site updates
- Release automation

## Risk Mitigation

1. **Comprehensive Testing**: Test every upgrade path with model projects
2. **Test-First Development**: Create tests before implementation
3. **Backup Everything**: Full project backup before migration
4. **Clear Error Messages**: Fail fast with helpful diagnostics
5. **Documentation First**: Complete docs before release
6. **intent doctor**: Safety net for configuration issues

## Success Criteria

1. **Zero data loss** during upgrades
2. **All existing projects** continue working
3. **Clean separation** of tool vs project artifacts
4. **Intuitive structure** for new users
5. **Robust test coverage** for all scenarios
6. **Smooth migration** experience
7. **Performance maintained** or improved
