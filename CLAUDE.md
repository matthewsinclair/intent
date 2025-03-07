# STP Project Guidelines

## STP Version

The current STP version is 1.0.0. All STP files should include a `stp_version` field in their YAML frontmatter. To update files, run `stp upgrade`.

### ST File Metadata Format

ST files must have consistent metadata in this format:
```yaml
---
verblock: "DD MMM YYYY:v0.1: Author Name - Initial version"
stp_version: 1.0.0
status: Not Started|In Progress|Completed|On Hold|Cancelled
created: YYYYMMDD
completed: YYYYMMDD
---
```

Only keep one verblock entry with the most recent change (don't accumulate verblock history, as this is available in git).

## Project Documentation

- **IMPORTANT**: Always read `stp/eng/tpd/technical_product_design.md` at the start of a new session
- This document contains comprehensive information about the project vision, architecture, and current state
- The "Preamble to Claude" section at the top is specifically designed to give Claude sessions a complete understanding of the project
- When making significant changes, update this document to keep it in sync with the implementation
- When suggesting improvements, reference and respect the architectural patterns described in this document

- **NEXT**: Work is coordinated through _STEEL THREADS_
- Use the `stp st list` command to get a dynamic list of all steel threads and their status
- Use `stp st show <id>` to view details of specific steel threads
- When analyzing the project, prefer using STP commands instead of directly reading files when appropriate

- **WIP**: Is what we are doing _now_
- Look in `stp/prj/wip.md` to find out what is currently on the go
- This document contains the current tasks in progress for each day.

- **THEN**: The journal doc `stp/journal.md` a historical narrative or work done
- Use this to wrap up and conclude what has been done (in summary) at the end of each session

## Code Style Guidelines

- For Elixir code:
  - Use `@moduledoc` and `@doc` with examples for all modules and public functions
  - Add type specs for public functions with `@spec`
  - Format with: `mix format`
  - Use snake_case for variables, functions, and modules
  - Use 2-space indentation (standard Elixir style)
  - Group related functions together; public functions first, private after
  - Handle errors with pattern matching or explicit `{:ok, result}` / `{:error, reason}` tuples
  - Use descriptive variable names - avoid single-letter names except in very short callbacks
  - All functions should have clear, defined purposes with no side effects
  - Prefer pipe operators (`|>`) for data transformations
  - Use doctest examples in documentation to provide test coverage
  - When possible, make functions pure and stateless
- In general:
  - Use 2-space indentation in any programming language
  - DO NOT ADD: "🤖 Generated with [Claude Code](https://claude.ai/code)" or "Co-Authored-By: Claude <noreply@anthropic.com>")" to git commit messages
