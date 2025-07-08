@short:
STP wrapper for Backlog.md task management

@description:
The backlog command (also available as 'bl') provides a streamlined interface
to Backlog.md that's optimized for STP workflows. It automatically handles
common issues like git fetch errors and provides shortcuts for task creation.

@usage:
stp backlog <command> [options] [arguments]
stp bl <command> [options] [arguments]

@commands:
init                       Initialize backlog in current project
task <subcommand>          Task management (create, list, edit, etc.)
list                       List all tasks (alias for 'task list --plain')
create <ST####> <title>    Create a task linked to a steel thread
board                      Display tasks in Kanban board
config                     Manage backlog configuration
browser                    Open browser interface

@features:
- Automatically adds --plain to list/board commands to prevent git errors
- Disables remote operations for local STP projects during init
- Provides shortcuts for common operations
- Maintains full backlog functionality

@examples:
# Initialize backlog for your project
stp bl init

# List all tasks without git fetch errors
stp bl list

# Create a task linked to a steel thread
stp bl create ST0014 "Add validation logic"

# Edit a specific task
stp bl task edit task-5

# View tasks in Kanban board
stp bl board

# Open browser interface
stp bl browser

@notes:
- This wrapper configures backlog for local use (no git operations)
- Use 'backlog' directly if you need remote git functionality
- Task status values: "To Do", "In Progress", "Done"
- For full backlog documentation, run: backlog help