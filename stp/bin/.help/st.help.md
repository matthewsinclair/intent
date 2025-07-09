---
verblock: "09 Jul 2025:v0.2: Matthew Sinclair - Updated for directory structure"
---
# st

@short:
Manage steel threads for the project

@desc:
Steel threads are self-contained units of work that focus on implementing
specific pieces of functionality. The 'st' command helps create, manage,
and track steel threads throughout the development process.

Starting with STP v1.2.1, steel threads are organized as directories containing
multiple files, allowing better separation of concerns and richer documentation.

Steel threads provide a structured way to organize development tasks,
making it easier to collaborate with LLMs and track progress over time.

@usage:
stp st <command> [options] [arguments]

Commands:
  new <title>                       Create a new steel thread directory
  done <id>                         Mark a steel thread as complete
  list [--status <status>] [--width N] List all steel threads
  sync [--write] [--width N]        Synchronize steel_threads.md with individual ST directories
  show <id> [file]                 Show details of a specific steel thread file
  edit <id> [file]                 Open a steel thread file in your default editor
  organize [--write]                Organize steel thread directories by status

Options for 'list':
  --status <status>        Filter steel threads by status
                           Valid statuses: Not Started, In Progress, Completed, On Hold, Cancelled
  --width N                Set the output table width in columns (defaults to terminal width)

Options for 'sync':
  --write                  Update the steel_threads.md file (without this flag, output is sent to stdout)
  --width N                Set the output table width in columns (defaults to terminal width)

Options for 'show' and 'edit':
  file                     Specific file to show/edit (optional, defaults to 'info')
                           Valid files: info, design, impl, tasks, results, all

Options for 'organize':
  --write                  Actually move directories (without this flag, shows preview)

Examples:
  stp st new "Implement User Authentication"    # Create a new steel thread directory
  stp st done ST0001                            # Mark ST0001 as complete
  stp st list --status "In Progress" --width 100  # List all in-progress steel threads
  stp st sync --write --width 100               # Update steel_threads.md with current ST state
  stp st show ST0001                            # Show info.md for ST0001
  stp st show ST0001 design                     # Show design.md for ST0001
  stp st show ST0001 all                        # Show all files for ST0001
  stp st edit ST0001                            # Edit info.md for ST0001
  stp st edit ST0001 impl                       # Edit impl.md for ST0001
  stp st organize --write                       # Organize directories by status

Steel Thread Structure (v1.2.1+):
  Steel threads are organized as directories containing multiple files:
  
  ST####/
  ├── info.md      # Main information (metadata, objective, context)
  ├── design.md    # Design decisions and approach
  ├── impl.md      # Implementation details
  ├── tasks.md     # Task tracking
  └── results.md   # Results and outcomes
  
  The info.md file contains the primary metadata:
  ---
  verblock: "Date:v0.1: Author - Description"
  stp_version: 1.2.1
  status: In Progress
  created: 20250307
  completed: 
  ---
  
  For full details on steel thread formats and migration, see the reference guide.
