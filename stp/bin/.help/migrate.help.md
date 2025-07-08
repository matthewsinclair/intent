@short:
Migrate embedded tasks from steel threads to Backlog

@description:
The migrate command helps transition from embedded task lists (checkboxes)
in steel thread documents to individual Backlog task files. This provides
better task management capabilities while preserving task state.

@usage:
stp migrate [options] <ST####>

@arguments:
ST####                     Steel thread ID to migrate

@options:
--all-active               Migrate all active steel threads
--dry-run                  Show what would be migrated without creating tasks

@examples:
# Migrate a single steel thread
stp migrate ST0014

# Preview migration without making changes
stp migrate --dry-run ST0014

# Migrate all active steel threads
stp migrate --all-active

@notes:
- Extracts tasks from the ## Tasks section
- Preserves checkbox state (done/not done)
- Updates steel thread to reference Backlog tasks
- Creates task files in backlog/tasks directory
- Task IDs are assigned automatically by Backlog