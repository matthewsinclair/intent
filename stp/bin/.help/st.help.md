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
  new <title>                       Create a new steel thread
  done <id>                         Mark a steel thread as complete
  list [--status <status>] [--width N] List all steel threads
  sync [--write] [--width N]        Synchronize steel_threads.md with individual ST files
  show <id>                         Show details of a specific steel thread
  edit <id>                         Open a steel thread in your default editor

Options for 'list':
  --status <status>        Filter steel threads by status
                           Valid statuses: Not Started, In Progress, Completed, On Hold, Cancelled
  --width N                Set the output table width in columns (defaults to terminal width)

Options for 'sync':
  --write                  Update the steel_threads.md file (without this flag, output is sent to stdout)
  --width N                Set the output table width in columns (defaults to terminal width)

Examples:
  stp st new "Implement User Authentication"    # Create a new steel thread
  stp st done ST0001                            # Mark ST0001 as complete
  stp st list --status "In Progress" --width 100  # List all in-progress steel threads
  stp st sync --write --width 100               # Update steel_threads.md with current ST state
  stp st show ST0001                            # Show details of ST0001
  stp st edit ST0001                            # Open ST0001 in your default editor

Steel Thread Metadata:
  Steel threads can store metadata in two formats:
  
  1. YAML frontmatter (at the top of the file):
     ---
     status: In Progress
     created: 20250307
     completed: 
     ---
     
  2. Document body (in the main content):
     - **Status**: In Progress
     - **Created**: 2025-03-07
     - **Completed**: 
     
  For full details on steel thread document formats, see the reference guide.
