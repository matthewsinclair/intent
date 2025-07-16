# Test Recovery Plan

## Current Situation
- Had: 186 tests across 22 test files
- Have: 41 tests across 8 test files  
- Lost: 145 tests (78% of coverage!)

## Critical Tests to Recover

### Steel Thread Tests (st_test.bats - 13 tests)
- ✓ st new creates a new steel thread
- ✓ st new creates sequential IDs
- ✓ st done marks as complete
- ✓ st list shows all threads
- ✓ st list --status filters
- ✓ st show displays content
- ✓ st show works with numbers
- ✓ st sync updates files
- More...

### Task Management (task_test.bats - 9 tests)
- task create <ST####> <title>
- task list <ST####>
- task sync <ST####>
- Task status tracking
- Integration with backlog

### Backlog Integration (bl_test.bats - 9 tests)
- bl init
- bl create
- bl list
- bl task operations

### Init Command (init_test.bats - 9 tests)
- init creates structure
- init handles existing projects
- init with custom names
- init creates config files

### Help System (help_test.bats - 6 tests)
- help shows usage
- help <command> shows command help
- help handles unknown commands

### Doctor/Diagnostic (intent_doctor_test.bats - 13 tests)
- doctor checks installation
- doctor checks config
- doctor fixes issues
- doctor detects legacy projects

## Recovery Strategy

1. **Immediate Priority** (Core functionality):
   - st_test.bats → Update for Intent paths
   - task_test.bats → Update for Intent
   - init_test.bats → Critical for new users
   - help_test.bats → User experience

2. **Secondary Priority** (Integration):
   - bl_test.bats → Backlog integration
   - doctor_test.bats → Diagnostics
   - migrate_test.bats → Legacy support

3. **Update Pattern**:
   - Change STP_* to INTENT_*
   - Update paths: stp/prj/st → intent/st
   - Update commands: stp → intent
   - Fix test helper references
   - Ensure v2.0.0 compatibility

## Expected Outcome
- Restore to ~150+ tests
- Cover all major functionality
- Maintain Intent v2.0.0 compatibility