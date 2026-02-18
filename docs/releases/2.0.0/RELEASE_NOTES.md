# Intent v2.0.0 Release Notes

## Release Date: July 17, 2025 (Updated: July 27, 2025)

## Overview

Intent v2.0.0 marks a major milestone in the evolution of the Steel Thread Process tooling. This release represents a complete rebrand from STP to Intent, reflecting the tool's core mission of capturing and preserving the intention behind software development decisions.

## Major Changes

### 🤖 Claude Code Sub-Agent Integration

Intent v2.0.0 includes full integration with Claude Code's sub-agent system, revolutionizing AI-assisted development:

- **Intent Agent**: Pre-built agent that understands steel threads, Intent commands, and project structure
- **Elixir Agent**: Specialized agent for Elixir development with Usage Rules and Ash/Phoenix patterns
- **Agent Management**: Complete suite of commands for installing, syncing, and managing agents
- **Seamless Integration**: Agents work automatically with Claude Code sessions

### 🚀 Complete Rebrand: STP → Intent

The project has been renamed from "STP" (Steel Thread Process) to "Intent" to better communicate its purpose. While the Steel Thread methodology remains unchanged, the tooling now has a name that immediately conveys its value proposition.

### 📁 Simplified Directory Structure

The project structure has been flattened and simplified:

**Before (STP v1.x):**

```
stp/
├── bin/        # Executables mixed with project
├── prj/        # Nested project directory
│   ├── st/     # Steel threads
│   └── wip.md  # Work in progress
├── eng/        # Engineering docs
└── usr/        # User docs
```

**After (Intent v2.0.0):**

```
.
├── agents/     # Claude Code sub-agents (global)
│   ├── intent/ # Intent methodology agent
│   └── elixir/ # Elixir code doctor agent
├── bin/        # Tool executables (top-level)
├── intent/     # Project artifacts (flattened)
│   ├── agents/ # Project-specific sub-agents
│   ├── st/     # Steel threads
│   ├── eng/    # Engineering docs
│   ├── usr/    # User docs
│   └── wip.md  # Work in progress
└── tests/      # Test suite
```

### 🔧 New Commands

- **`intent bootstrap`**: One-command global setup with clear instructions
- **`intent doctor`**: Comprehensive diagnostics to troubleshoot issues
- **`intent agents`**: Complete agent management system:
  - `intent agents list` - Show available and installed agents
  - `intent agents install` - Install agents to Claude configuration
  - `intent agents sync` - Update agents while preserving modifications
  - `intent agents uninstall` - Remove Intent-managed agents
  - `intent agents show` - Display agent details and metadata
  - `intent agents status` - Check agent health and integrity

### 📋 JSON Configuration

Configuration has moved from YAML to JSON format with proper local/global hierarchy:

- Local: `.intent/config.json`
- Global: `~/.config/intent/config.json` (follows XDG standard)

### ✅ Full Backwards Compatibility

- `stp` command symlinked to `intent`
- Automatic migration via `intent upgrade`
- All existing projects continue to work

## Installation & Migration

### New Installation

```bash
# Clone the repository
git clone https://github.com/matthewsinclair/intent.git
cd intent

# Add to PATH
export PATH="$PATH:$(pwd)/bin"

# Bootstrap global configuration
intent bootstrap

# Verify installation
intent doctor
```

### Migration from STP v1.x

```bash
# From your existing STP project
intent upgrade

# The upgrade will:
# 1. Detect your current version
# 2. Create backups
# 3. Migrate directory structure
# 4. Update configuration format
# 5. Preserve all content
```

## Breaking Changes

While we maintain backwards compatibility, these changes affect the underlying structure:

1. **Directory paths have changed**:
   - `stp/prj/st/` → `intent/st/`
   - `stp/prj/wip.md` → `intent/wip.md`
   - Configuration in `.intent/` not `.stp/`

2. **Configuration format**:
   - YAML → JSON
   - New global config location

3. **Repository location**:
   - GitHub: `matthewsinclair/stp` → `matthewsinclair/intent`

## New Features

### Claude Code Agent Integration

- **Pre-built Agents**: Intent and Elixir agents ready to use
- **Agent Management**: Full lifecycle commands (install, sync, uninstall)
- **Project & Global Agents**: Support for both scopes
- **Integrity Checking**: Checksums track modifications
- **Seamless Claude Integration**: Agents automatically enhance Claude sessions

**Quick Start with Agents:**

```bash
# Install the Intent agent
intent agents install intent

# Check agent status
intent agents status

# Now Claude understands Intent methodology!
```

### Enhanced User Experience

- **Better error messages**: Clear, actionable feedback
- **Improved help system**: Context-aware help
- **Streamlined commands**: Consistent interface
- **Progress indicators**: Visual feedback during operations

### Developer Experience

- **Comprehensive test suite**: Full coverage with BATS
- **GitHub Actions CI/CD**: Automated testing
- **Example projects**: Migration demonstrations
- **Enhanced documentation**: Updated for Intent

### Technical Improvements

- **Robust migration**: Fail-forward approach
- **Better path handling**: Works in more environments
- **Dependency management**: Clear requirements (jq, backlog.md)
- **Configuration validation**: Catches errors early
- **Agent manifest tracking**: JSON-based with checksums
- **50 new tests**: Complete agent system test coverage

## Fixed Issues

- GitHub Actions workflows now properly exclude library tests
- Symlink handling improved for cross-platform compatibility
- Test suite reliability enhanced
- Configuration loading hierarchy properly implemented
- Path resolution works correctly in all scenarios

## Known Issues

None at this time. All tests passing on Ubuntu and macOS.

## Upgrading

### From v1.2.1 to v2.0.0

1. **Backup your project** (automatic, but always good practice)
2. Run `intent upgrade` from your project root
3. Review the migration summary
4. Update any custom scripts to use `intent` instead of `stp`
5. Update your PATH if you had hardcoded the old location

### Command Equivalents

All commands remain the same, just replace `stp` with `intent`:

| Old Command       | New Command          |
| ----------------- | -------------------- |
| `stp st new`      | `intent st new`      |
| `stp bl list`     | `intent bl list`     |
| `stp task create` | `intent task create` |
| `stp help`        | `intent help`        |

## Future Roadmap

With the rebrand complete and Claude Code integration shipped, Intent is positioned for:

- **Q3 2025**: Extended agent ecosystem (language-specific agents)
- **Q4 2025**: Team collaboration features
- **2026**: Enterprise scalability and custom agent marketplaces

## Support

- **Documentation**: Updated user and reference guides
- **Issues**: Report bugs at https://github.com/matthewsinclair/intent/issues
- **Help**: Run `intent help` or `intent doctor`

## Contributors

- Matthew Sinclair - Project creator and maintainer
- Claude (Anthropic) - Development assistance

## Thank You

Thank you to all early adopters of STP. Your feedback shaped Intent into what it is today. The Steel Thread Process remains at the heart of Intent, now with tooling that better reflects its purpose.

---

**Start capturing intention today with Intent v2.0.0!**

```bash
intent st new "My first intentional development"
```
