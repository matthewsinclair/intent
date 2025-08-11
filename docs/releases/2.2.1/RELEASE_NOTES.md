# Intent v2.2.1 Release Notes

## Overview

Intent v2.2.1 is a maintenance release that significantly improves tool dependency management and error handling. This release addresses silent failures when required tools are missing and provides comprehensive guidance for users to resolve dependency issues.

## Key Improvements

### 🔧 Centralized Version Management

- **Single Source of Truth**: Version is now managed through a `VERSION` file at the project root
- **Consistent Updates**: All scripts dynamically read from the VERSION file, eliminating version inconsistencies
- **Easier Maintenance**: Future version bumps only require updating one file

### 🛠️ Comprehensive Tool Dependency Checking

The `intent doctor` command now provides detailed dependency analysis:

- **Categorized Tools**: Dependencies are organized into required, core, and optional categories
- **Platform-Specific Instructions**: Installation commands tailored for macOS, Linux distributions, and other systems
- **Clear Severity Levels**: Distinguish between critical errors, warnings, and informational messages

### 🚫 No More Silent Failures

- **jq Dependency Handling**: All commands that require jq now fail gracefully with clear error messages
- **Agent Operations**: Fixed silent failures during agent installation, sync, and management when jq is missing
- **Actionable Error Messages**: Every error now includes specific steps to resolve the issue

## What's New

### Added
- `VERSION` file for centralized version management
- `get_intent_version()` function in intent_helpers
- Comprehensive tool dependency checking in `intent doctor`
- Platform-specific installation instructions for all tools
- Better error handling for missing dependencies

### Changed
- Tool dependencies categorized as required, core, and optional
- Enhanced error messages with installation instructions
- All scripts now use centralized version management

### Fixed
- Silent failures when jq is missing during agent operations
- Missing error messages for required tool dependencies
- Inadequate installation guidance for different platforms
- Version number inconsistencies across scripts

## Installation & Upgrade

### For New Users
```bash
git clone https://github.com/matthewsinclair/intent.git
cd intent
export PATH="$PATH:$(pwd)/bin"
intent bootstrap
```

### For Existing Users
```bash
cd /path/to/intent
git pull origin main
intent doctor  # Check for any missing dependencies
```

## Tool Requirements

### Required Tools
- **bash**: Shell interpreter
- **sed**: Text processing
- **grep**: Pattern matching
- **mkdir**: Directory creation
- **jq**: JSON processing (critical for configs and agents)

### Optional Tools
- **backlog**: Task management system
- **bats**: Test framework
- **sha256sum/shasum**: Checksum verification for agents

Run `intent doctor` to check your environment and get installation instructions for any missing tools.

## Testing

After upgrading, verify your installation:

```bash
# Check version
intent --version  # Should show 2.2.1

# Run diagnostics
intent doctor

# Test in verbose mode
intent doctor --verbose
```

## Migration Notes

No breaking changes in this release. The version management improvements are backward compatible.

## Support

For issues or questions:
- GitHub Issues: https://github.com/matthewsinclair/intent/issues
- Documentation: Run `intent help` for command documentation

## Contributors

This release was developed by Matthew Sinclair with automated assistance.

---

*Intent v2.2.1 - Structured Development Process with Improved Reliability*