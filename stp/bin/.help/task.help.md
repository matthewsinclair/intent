@short:
Manage Backlog tasks linked to Steel Threads

@description:
The task command integrates STP with Backlog.md for fine-grained task tracking.
It allows you to create, list, and synchronize tasks associated with steel threads.

@usage:
stp task <command> [options] [arguments]

@commands:
create <ST####> <title>    Create a new task for a steel thread
list <ST####>              List all tasks for a steel thread
sync <ST####>              Sync task status with steel thread

@examples:
# Create a new task
stp task create ST0014 "Update documentation"

# List all tasks for a steel thread
stp task list ST0014

# Sync task status
stp task sync ST0014

@notes:
- Tasks are stored in the backlog/tasks directory
- Task names follow the pattern: ST#### - <description>
- Tasks have status: todo, in-progress, or done
- Use 'backlog' directly for advanced task management