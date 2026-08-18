---
verblock: "16 Jul 2025:v0.2: Matthew Sinclair - Updated with Phase 0 and new commands"
stp_version: 2.0.0
---

# ST0016: Task Tracking

## Overview

Task tracking for ST0016 is managed through Backlog.md integration. Use the following commands to create and track tasks:

```bash
# Create tasks for this steel thread
intent task create ST0016 "Create test infrastructure"
intent task create ST0016 "Implement config loading system"
intent task create ST0016 "Build upgrade command"

# View all tasks
intent task list ST0016

# Or use backlog directly
intent bl list | grep ST0016
```

## High-Level Task Breakdown

### Phase 0: Test Infrastructure (CRITICAL - DO FIRST)

- [ ] Create examples/v0.0.0-project with .stp-config format
- [ ] Create examples/v1.2.0-project with file-based STs
- [ ] Create examples/v1.2.1-project with directory-based STs
- [ ] Create examples/hello-world with v2.0.0 structure
- [ ] Write comprehensive BATS upgrade test suite
- [ ] Create test harness for migration scenarios
- [ ] Document expected test outcomes
- [ ] Setup CI/CD for automated testing

### Phase 1: New Commands

- [ ] Implement intent bootstrap command
- [ ] Implement intent doctor command
- [ ] Add bootstrap detection logic for INTENT_HOME
- [ ] Create doctor diagnostic checks
- [ ] Add --fix mode for doctor
- [ ] Write tests for new commands

### Phase 2: Configuration System

- [ ] Implement JSON parser for shell
- [ ] Create config loading functions
- [ ] Add project root detection logic
- [ ] Implement config overlay (global → local → env)
- [ ] Use ~/.config/intent/ for global config (XDG)
- [ ] Write comprehensive config tests

### Phase 3: Repository Restructuring

- [ ] Move stp/bin/\* to bin/
- [ ] Rename executables (stp → intent, stp*\*→ intent*\*)
- [ ] Create backwards compatibility symlinks
- [ ] Move stp/\_templ/ to lib/templates/
- [ ] Update all template references
- [ ] Test executable paths

### Phase 4: Upgrade Command

- [ ] Implement version detection with error handling
- [ ] Add clear error messages for unknown versions
- [ ] Create backup mechanism
- [ ] Build migration logic for each version
- [ ] Add dry-run mode
- [ ] Add progress reporting
- [ ] Convert configs to JSON format
- [ ] Update documentation files
- [ ] Write comprehensive upgrade tests

### Phase 5: Command Updates

- [ ] Update main intent script
- [ ] Update intent_init for new structure
- [ ] Update intent_st for flattened paths
- [ ] Update all other subcommands
- [ ] Add deprecation warnings for stp usage
- [ ] Test all commands with both structures

### Phase 6: Documentation

- [ ] Update README.md
- [ ] Create migration guide
- [ ] Update all command documentation
- [ ] Create troubleshooting guide
- [ ] Update examples and tutorials
- [ ] Write release notes

### Phase 7: Release Preparation

- [ ] Run full test suite
- [ ] Test on real projects (with backups)
- [ ] Create release branch
- [ ] Tag v2.0.0
- [ ] Prepare announcement
- [ ] Update website/docs

### Phase 8: Bootstrap & Installation

- [ ] Update installation instructions
- [ ] Create bootstrap script documentation
- [ ] Test new user flow
- [ ] Update CI/CD pipeline
- [ ] Create getting started guide
- [ ] Test PATH setup instructions

## Task Management

When starting work:

1. Create detailed tasks in Backlog.md for current phase
2. Link tasks to ST0016
3. Update task status as work progresses
4. Document any issues or decisions in this steel thread

## Dependencies

- Requires bash 3.2+ (standard on most systems)
- Requires standard Unix tools (sed, awk, grep)
- No external dependencies for core functionality
- JSON parsing done with sed/grep (no jq required)
