---
verblock: "06 Mar 2025:v0.1: Matthew Sinclair - Initial version"
---
# st

@short:
Manage steel threads for the project

@desc:
Steel threads are self-contained units of work that focus on implementing
specific pieces of functionality. The 'st' command helps create, manage,
and track steel threads throughout the development process.

Steel threads provide a structured way to organize development tasks,
making it easier to collaborate with LLMs and track progress over time.

@usage:
stp st <command> [options] [arguments]

Commands:
  new <title>              Create a new steel thread
  done <id>                Mark a steel thread as complete
  list [--status <status>] List all steel threads
  show <id>                Show details of a specific steel thread

Options for 'list':
  --status <status>        Filter steel threads by status
                           Valid statuses: Not Started, In Progress, Completed, On Hold, Cancelled

Examples:
  stp st new "Implement User Authentication"    # Create a new steel thread
  stp st done ST0001                            # Mark ST0001 as complete
  stp st list --status "In Progress"            # List all in-progress steel threads
  stp st show ST0001                            # Show details of ST0001
