# Intent v2.2.0 Release Notes

## Overview

Intent v2.2.0 introduces the `fileindex` command, a powerful file indexing and tracking system that enables systematic progress tracking through large codebases. This release also enhances the Elixir agent with systematic code review capabilities.

## New Features

### 1. Fileindex Command

The `intent fileindex` command provides a persistent checkbox-based file tracking system:

- **File Discovery**: Automatically find and index files based on patterns
- **Progress Tracking**: Mark files as checked/unchecked with persistent state
- **Flexible Integration**: Works both within Intent projects and standalone
- **Toggle Functionality**: Quick marking/unmarking of files with `-X` flag
- **Explicit State Control**: New `-C` (check) and `-U` (uncheck) flags for setting specific states

#### Key Commands:
```bash
# Create an index of all Elixir files recursively
intent fileindex -r -i project.index

# Check a specific file (mark as completed)
intent fileindex -i project.index -C lib/my_app/user.ex

# Uncheck a specific file (mark as pending)
intent fileindex -i project.index -U lib/my_app/user.ex

# Toggle a file's checked state
intent fileindex -i project.index -X lib/my_app/router.ex

# View current index with verbose output
intent fileindex -v -i project.index
```

### 2. Enhanced Elixir Agent

The Elixir agent now supports systematic code review workflows using the fileindex command:

- **Module-based Reviews**: Review entire Elixir modules systematically
- **Path Flexibility**: Accept both module names (eg `MyApp.Users`) and filesystem paths
- **Progress Tracking**: Automatically track which files have been reviewed
- **Smart Path Mapping**: Intelligent conversion between Elixir module names and file paths

#### Example Usage:
```
Review the MyApp.Users module systematically, checking each file as you complete it
```

## Improvements

### Upgrade Command
- Updated to support migrations to v2.2.0
- Improved version detection and upgrade path handling
- Better support for incremental upgrades (2.0.0 → 2.1.0 → 2.2.0)

### Bash Compatibility
- Fixed macOS compatibility issues with associative arrays
- Replaced `readarray` with portable alternatives
- Improved shell script portability

## Installation

### New Installation
```bash
curl -sSL https://intent.dev/install.sh | bash
```

### Upgrade from Previous Versions
```bash
intent upgrade
```

## Migration Notes

Projects upgrading from v2.1.0 will have their configuration automatically updated to v2.2.0. The fileindex command will be immediately available after upgrade.

## Breaking Changes

None. This release is fully backward compatible with v2.1.0.

## Bug Fixes

- Fixed bash compatibility issues for macOS users
- Improved error handling in test framework
- Better handling of edge cases in file operations

## Credits

This release includes contributions from the Intent community. Special thanks to all testers and users who provided feedback on the fileindex functionality.

## Next Steps

After upgrading to v2.2.0:

1. Run `intent help fileindex` to learn about the new command
2. Try `intent fileindex --demo` to see it in action
3. Use `intent agents list` to see the updated Elixir agent capabilities
4. Run `intent doctor` to verify your installation

For questions or issues, please visit: https://github.com/intent-dev/intent/issues