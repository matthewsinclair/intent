# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2025-07-09

### Added

- New `stp llm usage_rules` command for displaying STP usage patterns to LLMs
- `--symlink` option for `stp llm usage_rules` to create usage-rules.md symlinks in projects
- Comprehensive test suite for the llm command (`stp/tests/llm/llm_test.bats`)
- DEPRECATIONS.md file to track deprecated features
- Help documentation for llm command (`stp/bin/.help/llm.help.md`)
- Archive directory structure for deprecated content (`stp/prj/archive/`)

### Changed

- Renamed `usage_rules.md` to `usage-rules.md` to follow Elixir Hex package conventions
- Updated `stp_upgrade` to handle file renaming during upgrades
- Updated all documentation to reference Backlog for historical tracking instead of journal.md
- Simplified `stp_init` to only create `wip.md` and `steel_threads.md` in the prj directory

### Deprecated

- `journal.md` file - users should migrate to Backlog task tracking for historical project narrative

### Removed

- `journal.md` creation from `stp_init` script
- Journal template from `stp/_templ/prj/_journal.md`
- All references to `journal.md` from documentation (18 files updated across user guides, reference guides, blog posts, and templates)

### Migration Guide

#### For users with existing journal.md files

1. Your existing `journal.md` has been automatically moved to `stp/prj/archive/journal-deprecated.md`
2. Use `stp bl list` to view task history moving forward
3. Track detailed progress in Backlog task descriptions
4. Use steel thread documents for high-level context and decisions

#### For LLM integration

1. Use `stp llm usage_rules` to display usage patterns
2. Create symlinks with `stp llm usage_rules --symlink` for projects expecting usage-rules.md
3. Reference the usage rules documentation at `stp/eng/usage-rules.md`

## [1.0.0] - 2025-06-03

### Added

- Initial release of Steel Thread Process (STP)
- Core script framework for managing steel threads
- Template system for project documentation
- Integration with Backlog.md for task management
- Comprehensive test suite using BATS
- User and reference documentation
- Blog series explaining STP concepts and methodology

### Features

- `stp init` - Initialize STP in a project
- `stp st` - Manage steel threads (new, list, show, edit, done, sync)
- `stp bl` - Backlog.md wrapper for task management
- `stp task` - Create and list tasks linked to steel threads
- `stp status` - Synchronize steel thread status with task completion
- `stp migrate` - Migrate embedded tasks to Backlog
- `stp upgrade` - Upgrade STP files to latest format
- `stp help` - Comprehensive help system

[1.2.0]: https://github.com/matthewsinclair/stp/compare/v1.0.0...v1.2.0
[1.0.0]: https://github.com/matthewsinclair/stp/releases/tag/v1.0.0
