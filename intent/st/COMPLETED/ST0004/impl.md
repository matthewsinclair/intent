# Implementation - ST0004: Steel Thread Commands

## Implementation Notes

### Command Structure

The steel thread command subsystem includes the following commands:

- `stp st new <title>` - Create a new steel thread with the given title
- `stp st list [--status <status>]` - List all steel threads, optionally filtered by status
- `stp st show <id>` - Display the contents of a specific steel thread
- `stp st done <id>` - Mark a steel thread as complete
- `stp st edit <id>` - Open a steel thread in the default editor

### Steel Thread ID Format

Steel threads follow a consistent ID format:

- IDs are in the format ST#### (eg ST0001)
- Numbers are padded to 4 digits with leading zeros
- IDs are automatically assigned in sequence

To improve usability, commands accept abbreviated IDs:

- Just the number (eg `1`)
- The number with or without leading zeros (eg `0001`)
- The full ID (eg `ST0001`)

### Steel Thread Index

The system maintains a steel thread index file (`steel_threads.md`) that:

- Lists all steel threads with their status, creation date, and completion date
- Is automatically updated when creating or marking threads as complete
- Provides a Markdown table for easy viewing
- Includes links to the individual steel thread files

### Cross-Platform Support

The `edit` command is designed to work across platforms:

- Uses `open` on macOS
- Uses `xdg-open` on Linux
- Uses `start` on Windows
- Falls back to environment variables or vi as a last resort
