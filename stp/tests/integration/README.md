# STP-Backlog Integration Tests

This directory contains integration tests that verify STP and Backlog.md work together correctly.

## Running the Tests

```bash
# Run all integration tests
bats integration/stp_backlog_integration_test.bats

# Run a specific test
bats integration/stp_backlog_integration_test.bats --filter "stp bl create"
```

## What These Tests Verify

### Core Integration Points

1. **Task Creation** - Tasks created through `stp bl create` are properly stored in Backlog format
2. **Task Listing** - Both `stp bl list` and `stp task list` correctly display tasks
3. **Task Naming** - Tasks follow the `task-<number>` naming convention required by Backlog.md
4. **Status Sync** - Task completion status properly syncs with steel thread status
5. **YAML Structure** - Created tasks have correct YAML frontmatter
6. **Special Characters** - Task titles with quotes and special characters are handled correctly
7. **Git Error Prevention** - The STP wrapper successfully prevents git-related errors
8. **Task Counting** - Task counts are accurate across different commands
9. **Error Handling** - Invalid steel thread IDs are properly rejected

### Known Limitations

1. **Task ID Format** - Backlog.md requires `task-<number>` format and cannot be customized
2. **Steel Thread Validation** - `stp bl create` doesn't validate if steel thread exists (by design)
3. **Migration Specifics** - `stp migrate` requires a specific "## Tasks" section format
4. **Browser Launch** - `stp bl board` opens a browser which can't be fully tested in CI

## Test Results Summary

All 13 tests pass consistently, verifying the core integration between STP and Backlog.md works correctly.

## Key Findings

- The integration works well for the core use cases
- Task naming must follow Backlog's conventions (`task-<number>`)
- The STP wrapper successfully prevents git-related errors
- Status synchronization between tasks and steel threads functions correctly
- Special characters in task titles are handled properly

## Future Improvements

1. Add tests for draft task functionality
2. Test concurrent task creation
3. Verify task deletion and archiving
4. Test edge cases like very long task titles
5. Add performance tests for large numbers of tasks