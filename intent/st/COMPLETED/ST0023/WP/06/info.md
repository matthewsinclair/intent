# WP06: Tests - Remove/Update Test Files

## Scope

Delete backlog test files, update remaining tests.

## Files to DELETE

- tests/unit/bl_commands.bats
- tests/unit/task_commands.bats
- tests/unit/migration.bats

## Files to Edit

- tests/core_functionality.bats - Remove backlog tests/assertions
- tests/unit/project_commands.bats - Remove backlog test
- tests/unit/bootstrap.bats - Remove backlog config assertion
- tests/unit/agent_commands.bats - Update subagent assertions
- tests/lib/test_helper.bash - Remove mkdir backlog

## Acceptance Criteria

- Test suite drops from 17 to 14 files
- All remaining tests pass
