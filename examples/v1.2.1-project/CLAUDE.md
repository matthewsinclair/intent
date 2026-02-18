# . Project Guidelines

This is an Intent v2.0.0 project (formerly STP).

## Project Structure

- `intent/` - Project artifacts (steel threads, docs, work tracking)
  - `st/` - Steel threads organized as directories
  - `docs/` - Technical documentation
  - `llm/` - LLM-specific guidelines
- `backlog/` - Task management (if using Backlog.md)
- `.intent/` - Configuration and metadata

## Steel Threads

Steel threads are organized as directories under `intent/st/`:

- Each steel thread has its own directory (eg ST0001/)
- Minimum required file is `info.md` with metadata
- Optional files: design.md, impl.md, tasks.md, results.md

## Commands

- `intent st new "Title"` - Create a new steel thread
- `intent st list` - List all steel threads
- `intent st show <id>` - Show steel thread details
- `intent doctor` - Check configuration
- `intent help` - Get help

## Migration Notes

This project was migrated from STP to Intent v2.0.0 on 2025-07-17.

- Old structure: `stp/prj/st/`, `stp/eng/`, etc.
- New structure: `intent/st/`, `intent/docs/`, etc.
- Configuration moved from YAML to JSON format

## Author

matts
