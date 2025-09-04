# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.3.2] - 2025-09-04

### Added

- Comprehensive antipattern detection to Elixir subagent
  - Detects and remediates 24 common Elixir antipatterns
  - Antipatterns categorized into Code (9), Design (6), Process (4), and Meta-programming (5)
  - Full documentation in `intent/plugins/claude/subagents/elixir/antipatterns.md`
  - Antipatterns sourced from official Elixir documentation
- Antipattern review workflow integrated into Elixir Doctor
- Example usage commands and report formats for antipattern detection
- Key principles for antipattern prevention

### Changed

- Enhanced Elixir subagent with antipattern detection capabilities
- Updated systematic review template to include antipattern analysis
- Elixir Doctor now automatically checks for antipatterns during code reviews

### Technical Improvements

- Better code quality guidance through antipattern detection
- More comprehensive code review process
- Proactive detection of common Elixir mistakes

## [2.3.1] - 2025-08-29

### Added

- Worker-bee agent for Worker-Bee Driven Design (WDD) in Elixir applications
- Resources directory structure for agents with templates and Mix tasks
- Worker-bee agent includes comprehensive WDD validation and scaffolding tools

### Changed

- Enhanced agent system to support resource directories
- Improved subagent installation and management

## [2.3.0] - 2025-08-20

### Added

- Plugin architecture for Intent
- Claude subagents system (renamed from agents)
- AGENTS.md universal AI agent instructions
- Support for multiple AI platforms through AGENTS.md
- New `intent agents` commands for AGENTS.md management
- New `intent claude subagents` commands (replacing old `intent agents`)

### Changed

- Renamed `intent agents` commands to `intent claude subagents`
- Moved subagents to `intent/plugins/claude/subagents/`
- Updated project structure to support plugins

### Technical Improvements

- More flexible agent system architecture
- Better separation of concerns with plugin system
- Universal agent instructions format

## [2.2.1] - 2025-08-11

### Added

- Centralized version management through VERSION file
- `get_intent_version()` function in intent_helpers for consistent version retrieval
- Comprehensive tool dependency checking in `intent doctor`
- Platform-specific installation instructions for all required tools
- Better error handling for missing jq dependency across all commands

### Changed

- Steel threads now start with 'WIP' status instead of 'In Progress' when using `intent st start`
- Tool dependencies categorized as required, core, and optional in doctor command
- Enhanced jq error messages with clear installation instructions
- All scripts now read version from centralized VERSION file

### Fixed

- `intent upgrade` now preserves existing CLAUDE.md files instead of overwriting them
- Silent failures when jq is missing during agent operations
- Missing error messages for required tool dependencies
- Inadequate installation guidance for different platforms
- Version number inconsistencies across different scripts

### Technical Improvements

- Single source of truth for version management
- Reduced maintenance overhead for version updates
- Improved fallback behavior when tools are missing
- Better user experience with actionable error messages

## [2.2.0] - 2025-08-05

### Added

- `intent fileindex` command for systematic file tracking and progress management
- Check functionality (`-C` flag) to explicitly mark files as checked [x] in the index
- Uncheck functionality (`-U` flag) to explicitly mark files as unchecked [ ] in the index
- Toggle functionality (`-X` flag) to switch files between checked/unchecked states
- Flexible operation modes - works both within Intent projects and standalone
- Enhanced Elixir agent with systematic code review workflow using fileindex
- Support for both Elixir module names and filesystem paths in the Elixir agent
- Comprehensive test suite for fileindex command (47 tests including check/uncheck)
- Demo mode (`--demo`) to showcase fileindex functionality

### Changed

- Updated all version references from 2.1.0 to 2.2.0
- Enhanced `intent upgrade` to support 2.1.0 → 2.2.0 migrations
- Improved upgrade path handling for incremental version upgrades
- Updated Elixir agent documentation with systematic review workflow
- Added fileindex to global commands list

### Fixed

- Bash compatibility issues on macOS (associative arrays, readarray command)
- Local variable declarations at global scope in shell scripts
- Missing `assert_output` function in test framework
- Test expectations for error messages

### Technical Improvements

- Replaced bash associative arrays with parallel arrays for macOS compatibility
- Replaced `readarray` with portable while loops
- Added proper error handling for edge cases in file operations
- Enhanced test helper with assert_output function

## [2.1.0] - 2025-07-27

### Added

- `intent agents init` command to initialize agent configuration
- Support for upgrading from Intent v2.0.0 to v2.1.0
- Enhanced agent manifest management with proper initialization
- Improved agent setup workflow with explicit initialization step

### Changed

- Updated all version references from 2.0.0 to 2.1.0
- Enhanced `intent upgrade` to support 2.0.0 → 2.1.0 migrations
- Improved agent installation workflow to require initialization first
- Updated documentation to reflect v2.1.0 features

### Fixed

- Agent directories not being properly created during upgrade
- Missing agent initialization when upgrading from older versions
- Agent manifest not being created in fresh installations
- Incorrect creation of `agents/` directory at project root instead of `intent/agents/`
- Upgrade process incorrectly preserving root-level agent directories

## [2.0.0] - 2025-07-17

### Added

- New `intent` command as the primary CLI (replacing `stp`)
- `intent bootstrap` command for easy global setup
- `intent doctor` command for comprehensive diagnostics
- `intent st repair` command to fix malformed steel thread metadata
- JSON-based configuration system (local and global)
- Full backwards compatibility with STP v1.x projects
- Comprehensive test suite with GitHub Actions CI/CD
- Example projects demonstrating migration paths
- Support for `jq` dependency in workflows
- **Claude Code Sub-Agent Integration**: Complete agent management system
  - `intent agents` command suite (list, install, sync, uninstall, show, status)
  - Intent agent with steel thread methodology knowledge
  - Elixir agent with Usage Rules and Ash/Phoenix patterns
  - Global and project-specific agent support
  - Manifest-based tracking with checksum integrity
  - Seamless integration with intent init, doctor, and upgrade commands

### Changed

- **BREAKING**: Renamed from STP to Intent
- **BREAKING**: Flattened directory structure (intent/ instead of stp/prj/)
- **BREAKING**: Executables moved to top-level bin/ directory
- **BREAKING**: Configuration format changed from YAML to JSON
- Improved error messages and user feedback
- Enhanced migration tools with fail-forward approach
- Streamlined command structure and naming
- Updated all documentation to reflect Intent branding

### Fixed

- GitHub Actions workflow issues with bats libraries
- Symlink issues with stp compatibility command
- Test suite reliability and coverage
- Configuration loading hierarchy
- Path resolution in various environments
- Malformed YAML frontmatter in steel threads after migration
- Legacy field names (stp_version) in steel thread metadata
- Conflicting status values between frontmatter and body content

### Deprecated

- `stp` command (now aliases to `intent` for compatibility)
- Old directory structure (stp/prj/st/ → intent/st/)
- YAML configuration format
- Nested project directory structure

### Migration Guide

#### From STP v1.x to Intent v2.0.0

1. **Automatic Migration**: Run `intent upgrade` to automatically migrate your project
2. **Manual Installation**:

   ```bash
   # Clone Intent repository
   git clone https://github.com/matthewsinclair/intent.git
   cd intent
   
   # Add to PATH
   export PATH="$PATH:$(pwd)/bin"
   
   # Bootstrap global configuration
   intent bootstrap
   ```

3. **Project Structure Changes**:
   - `stp/prj/st/` → `intent/st/`
   - `stp/prj/wip.md` → `intent/wip.md`
   - `stp/eng/` → `intent/eng/`
   - `stp/usr/` → `intent/usr/`

4. **Command Changes**:
   - All `stp` commands now use `intent`
   - Same subcommands and options supported
   - `stp` symlink provided for compatibility

See [Release Notes](./docs/releases/2.0.0/RELEASE_NOTES.md) for complete details.

## [1.2.1] - 2025-07-09

### Added

- Directory-based structure for steel threads (replacing single files)
- New steel thread file types: `info.md`, `design.md`, `impl.md`, `tasks.md`
- Migration script `migrate_st_to_dirs` for upgrading from v1.2.0 to v1.2.1
- Support for editing/viewing specific steel thread files with `stp st show/edit <id> <file>`
- `stp st show <id> all` command to view all steel thread files at once
- Automatic file creation when editing non-existent steel thread files
- Version tracking in `stp/.config/version` file

### Changed

- **BREAKING**: Steel threads are now directories containing multiple files instead of single `.md` files
- Updated `stp_st` script to handle both legacy (file) and new (directory) structures
- Enhanced `stp st new` to create directory structure with all template files
- Modified `stp st done` to move entire directories when completing steel threads
- Updated `stp st list` to read from `info.md` files in directories
- Enhanced `stp st organize` to handle directory-based steel threads
- Improved `stp upgrade` to automatically detect and migrate steel threads to directory structure
- Updated all documentation to reflect new steel thread structure

### Fixed

- Version detection in `stp_st` now properly checks for directory vs file structure
- Steel thread organization now correctly moves directories instead of files

### Migration Guide

#### Upgrading from v1.2.0 to v1.2.1

1. Run `stp upgrade` - it will detect old-format steel threads and offer to migrate them
2. The migration will:
   - Create a backup in `.backup/1.2.1/`
   - Create directories for each steel thread (e.g., `ST0001/`)
   - Split content into separate files based on sections
   - Preserve all existing content and metadata
3. After migration, use `stp st organize --write` to organize by status if desired

#### New Steel Thread Commands

- `stp st show ST0001 design` - Show only the design.md file
- `stp st edit ST0001 impl` - Edit the implementation file
- `stp st show ST0001 all` - View all files for a steel thread

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

### Fixed

- Fixed `stp upgrade` version mismatch (was using 1.0.0 instead of 1.2.0)
- Made file organization in `stp upgrade` optional with new `--organize` flag to prevent unexpected file moves

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
3. Reference the usage rules documentation at `intent/llm/usage-rules.md`

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

[2.2.1]: https://github.com/matthewsinclair/intent/compare/v2.2.0...v2.2.1
[2.2.0]: https://github.com/matthewsinclair/intent/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/matthewsinclair/intent/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/matthewsinclair/intent/compare/v1.2.1...v2.0.0
[1.2.1]: https://github.com/matthewsinclair/intent/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/matthewsinclair/intent/compare/v1.0.0...v1.2.0
[1.0.0]: https://github.com/matthewsinclair/intent/releases/tag/v1.0.0
