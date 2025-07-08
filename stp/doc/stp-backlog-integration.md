---
verblock: "08 Jul 2025:v0.1: Matthew Sinclair - Initial version"
stp_version: 1.0.0
---
# STP + Backlog.md Integration Guide

## Overview

This document describes the integration between STP (Steel Thread Process) and Backlog.md for enhanced task management. The integration maintains STP's strength in intent capture while leveraging Backlog.md's powerful task tracking capabilities.

## Architecture

### STP Responsibilities
- **Intent Capture**: High-level objectives and context in steel thread documents
- **Design Documentation**: Detailed design specifications (ST####_design.md)
- **Implementation Records**: As-built documentation (ST####_impl.md)
- **Process Coordination**: Overall workflow and steel thread lifecycle

### Backlog.md Responsibilities
- **Task Management**: Individual task tracking with rich metadata
- **Status Tracking**: Granular task states (draft, todo, in-progress, done)
- **Task Organisation**: Labels, priorities, dependencies, and subtasks
- **Visualisation**: Kanban board and browser interface

## Naming Conventions

### Backlog Task Naming
Tasks linked to steel threads follow this pattern:
```
ST#### - <task description>
```

Example:
```
ST0014 - Create directory structure
ST0014 - Update ST commands for new paths
ST0014 - Add unit tests
```

### File Organisation
- Steel thread documents remain in `/stp/prj/st/`
- Backlog tasks are stored in `/backlog/tasks/`
- Task files are named: `task-<id> - <title>.md`

## Workflow Integration

### 1. Creating a New Steel Thread

```bash
# Create the steel thread
stp st new "My New Feature"
# Returns: Created ST0015

# Create associated tasks in Backlog
stp task create ST0015 "Design API structure"
stp task create ST0015 "Implement core logic"
stp task create ST0015 "Write unit tests"
```

### 2. Task Lifecycle

1. **Draft Phase**: Ideas and potential tasks
   ```bash
   backlog draft create "ST0015 - Consider caching strategy"
   ```

2. **Active Development**: Move to active tasks
   ```bash
   backlog draft promote <task-id>
   backlog task edit <task-id> --status in-progress
   ```

3. **Completion**: Mark tasks done
   ```bash
   backlog task edit <task-id> --status done
   ```

4. **Archival**: Archive completed tasks
   ```bash
   backlog task archive <task-id>
   ```

### 3. Status Synchronisation

Steel thread status is determined by task states:
- **Not Started**: No tasks created or all tasks in draft
- **In Progress**: At least one task in todo/in-progress state
- **On Hold**: Manual designation when work is paused
- **Completed**: All tasks done or archived
- **Cancelled**: Manual designation with tasks archived

Use `stp status` to sync:
```bash
stp status sync ST0015
```

### 4. Viewing Tasks

```bash
# View all tasks for a steel thread
stp task list ST0015

# View in Kanban board
backlog board

# View in browser
backlog browser
```

## Steel Thread Document Structure

With Backlog integration, steel thread documents focus on intent and context:

```markdown
---
verblock: "08 Jul 2025:v0.1: Author Name - Initial version"
stp_version: 1.0.0
status: In Progress
created: 20250708
completed: 
---
# ST0015: Feature Title

## Objective
High-level goal and business value

## Context
Background information and rationale

## Approach
Strategic approach and key decisions

## Tasks
Tasks are tracked in Backlog. View with: `stp task list ST0015`

## Implementation Notes
Key technical decisions and learnings

## Results
Outcomes and metrics (completed threads)
```

## Migration from Embedded Tasks

For existing steel threads with embedded task lists:

```bash
# Migrate a specific steel thread
stp migrate ST0014

# Migrate all active threads
stp migrate --all-active
```

This will:
1. Extract checkbox tasks from the markdown
2. Create corresponding Backlog tasks
3. Update the steel thread document
4. Preserve task completion status

## Best Practices

### Task Granularity
- Keep tasks focused and achievable in 1-2 days
- Use subtasks for complex items
- Create separate tasks for research vs implementation

### Labeling Strategy
- Always include steel thread ID in task title
- Use additional labels for cross-cutting concerns:
  - `bug`, `feature`, `refactor`, `docs`
  - `blocked`, `waiting-review`
  - Team or component labels

### Dependencies
- Use Backlog's dependency features for task ordering
- Document external dependencies in task notes
- Link related tasks across steel threads

### Regular Maintenance
- Run `stp-status sync` regularly
- Archive completed tasks weekly
- Review and promote drafts in planning sessions

## Command Reference

### stp task
```bash
stp task create <ST####> <title>     # Create a task
stp task list <ST####>               # List tasks for a thread
stp task sync <ST####>               # Sync task status
```

### stp status
```bash
stp status show <ST####>             # Show thread/task status
stp status sync <ST####>             # Update thread from tasks
stp status report                    # Overall status report
```

### stp migrate
```bash
stp migrate <ST####>                 # Migrate one thread
stp migrate --all-active             # Migrate all active
stp migrate --dry-run <ST####>       # Preview migration
```

## Troubleshooting

### Common Issues

1. **Task ID Conflicts**
   - Backlog assigns unique IDs automatically
   - Don't manually edit task IDs

2. **Status Mismatch**
   - Run `stp status sync` to update
   - Check for tasks in unexpected states

3. **Missing Tasks**
   - Check drafts folder
   - Verify task wasn't archived

### Getting Help

- Run `stp help` for STP commands
- Run `backlog help` for Backlog commands
- Check `/stp/doc/` for additional guides

## Testing

The integration includes comprehensive test coverage:

```bash
# Run all integration tests
cd stp/tests
./run_tests.sh task
./run_tests.sh status  
./run_tests.sh migrate

# Or run specific test files
bats task/task_test.bats
bats status/status_test.bats
bats migrate/migrate_test.bats
```

Test files are located in:
- `stp/tests/task/task_test.bats` - Task command tests
- `stp/tests/status/status_test.bats` - Status command tests  
- `stp/tests/migrate/migrate_test.bats` - Migration command tests