@short:
Sync steel thread status based on Backlog task completion

@description:
The status command helps maintain consistency between steel thread status
and the completion state of associated Backlog tasks. It can show current
status, suggest updates, and generate reports.

@usage:
stp status <command> [options] [arguments]

@commands:
show <ST####>              Show status of steel thread and its tasks
sync <ST####>              Update steel thread status based on tasks
report                     Generate status report for all active threads

@options:
--dry-run                  Show what would be changed without updating

@examples:
# Show current status
stp status show ST0014

# Sync steel thread status
stp status sync ST0014

# Preview changes without updating
stp status sync ST0014 --dry-run

# Generate overall status report
stp status report

@notes:
- Status is determined by task completion:
  - Not Started: No tasks or all tasks in draft
  - In Progress: At least one task todo or in-progress
  - Completed: All tasks done
- Manual status overrides are preserved when appropriate