@short:
Create and manage file indexes with checkbox states for tracking file processing

@description:
The fileindex command creates indexes of files matching specified patterns, with support
for checkbox states to track which files have been processed or reviewed. It can work
both as a standalone tool and integrate with Intent projects for enhanced functionality.

@usage:
intent fileindex [OPTIONS] [STARTDIR] [FILESPEC]

@options:
-r                  Recurse through subdirectories
-v                  Verbose mode (show processing details and summary)
-f FILE            Output to file instead of stdout
--file FILE        Output to file instead of stdout (alternative syntax)
-i FILE            Use index file to maintain checked states
--index FILE       Use index file to maintain checked states (alternative syntax)
-X FILE            Toggle the checked state of FILE in the index
--toggle FILE      Toggle the checked state of FILE in the index (alternative syntax)
-C FILE            Set FILE to checked [x] state in the index
--check FILE       Set FILE to checked [x] state in the index (alternative syntax)
-U FILE            Set FILE to unchecked [ ] state in the index
--uncheck FILE     Set FILE to unchecked [ ] state in the index (alternative syntax)
--index-dir DIR    Specify default directory for index files
--intent-dir DIR   Specify Intent project directory explicitly
--no-intent        Disable Intent integration even if in a project
-h                 Show help message

@arguments:
STARTDIR           Directory to search in (defaults vary by context)
FILESPEC           File pattern to match (eg "*.py", "*.{ex,exs}")

@defaults:
When run within an Intent project:
  STARTDIR:        lib/
  FILESPEC:        *.{ex,exs}
  INDEX_DIR:       .intent/indexes/

When run standalone:
  STARTDIR:        . (current directory)
  FILESPEC:        *.{ex,exs}
  INDEX_DIR:       . (current directory)

@examples:
# List all Elixir files in the current directory
intent fileindex

# Recursively list all Elixir files
intent fileindex -r

# Create an index file for tracking progress
intent fileindex -r -i project.index

# Search Python files in src/ directory
intent fileindex src "*.py"

# Use verbose mode to see processing details
intent fileindex -rv

# Output to a file instead of stdout
intent fileindex -r -f filelist.txt

# Disable Intent integration in a project
intent fileindex --no-intent

# Toggle a file's checked state (switches between [ ] and [x])
intent fileindex -i project.index -X lib/myapp/user.ex
# Output shows new state:
# [x] lib/myapp/user.ex   (if it was unchecked)
# [ ] lib/myapp/user.ex   (if it was checked)

# Check a specific file (set to [x])
intent fileindex -i project.index -C lib/myapp/user.ex
# Output: [x] lib/myapp/user.ex

# Uncheck a specific file (set to [ ])
intent fileindex -i project.index -U lib/myapp/user.ex
# Output: [ ] lib/myapp/user.ex

# Example workflow: process files one by one
# 1. Create initial index
intent fileindex -r -i review.index
# 2. Review first file and mark as checked
vim lib/myapp/user.ex
intent fileindex -i review.index -C lib/myapp/user.ex
# 3. Continue with next file...
vim lib/myapp/router.ex
intent fileindex -i review.index -C lib/myapp/router.ex
# 4. View current status
cat review.index | grep "^\[.\]"

@index_file_format:
Index files contain:
1. A JSON configuration header with metadata
2. File entries in format: [x] filename or [ ] filename
   - [ ] indicates unchecked/unprocessed
   - [x] indicates checked/processed

Example index file:
```
{
  "generator": "intent-fileindex",
  "version": "1.1",
  "timestamp": "2024-01-15T10:30:00Z",
  "context": "intent_project",
  "config": {
    "startdir": "lib",
    "filespec": "*.{ex,exs}",
    "recursive": true
  }
}

[ ] lib/myapp/application.ex
[x] lib/myapp/router.ex
[ ] lib/myapp/supervisor.ex
```

@features:
- Smart defaults based on context (Intent project vs standalone)
- Persistent checkbox states for tracking file processing
- JSON metadata header for reproducibility
- Handles file additions and removals automatically
- Verbose mode for debugging and progress tracking
- Flexible output options (stdout, file, or index)

@notes:
- When using an index file, the tool preserves checkbox states across runs
- Files that no longer exist are automatically removed from the index
- New files are added with unchecked state
- The index file is updated atomically to prevent corruption
- Toggle mode requires an existing index file with the target file present
- Toggle output shows the new state of the file after toggling
- Check/uncheck modes require an existing index file with the target file present
- Check mode sets files to [x] state regardless of current state
- Uncheck mode sets files to [ ] state regardless of current state
- Check/uncheck operations are idempotent - checking an already checked file keeps it checked