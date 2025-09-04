# Intent v2.3.2 Release Notes

## Overview

Intent v2.3.2 enhances the Elixir subagent with comprehensive antipattern detection capabilities. This release helps Elixir developers write cleaner, more maintainable code by automatically detecting and providing remediation for 24 common antipatterns sourced from the official Elixir documentation.

## Key Features

### 🔍 Comprehensive Antipattern Detection

The Elixir Doctor now detects and helps remediate 24 common Elixir antipatterns across four categories:

#### Code-related Antipatterns (9 patterns)

- Comments overuse
- Complex `else` clauses in `with`
- Complex extractions in clauses
- Dynamic atom creation
- Long parameter list
- Namespace trespassing
- Non-assertive map access
- Non-assertive pattern matching
- Non-assertive truthiness

#### Design-related Antipatterns (6 patterns)

- Alternative return types
- Boolean obsession
- Exceptions for control-flow
- Primitive obsession
- Unrelated multi-clause function
- Using application configuration for libraries

#### Process-related Antipatterns (4 patterns)

- Code organisation by process
- Scattered process interfaces
- Sending unnecessary data
- Unsupervised processes

#### Meta-programming Antipatterns (5 patterns)

- Compile-time dependencies
- Large code generation
- Unnecessary macros
- `use` instead of `import`
- Untracked compile-time dependencies

### 📊 Enhanced Code Review Process

- **Integrated Detection**: Antipattern checking is now part of the systematic code review workflow
- **Detailed Reports**: Clear reporting with line numbers and specific remediation suggestions
- **Prioritized Fixes**: Antipatterns are categorized and prioritized by impact
- **Prevention Principles**: Key principles to help avoid antipatterns in future code

## What's New

### Added

- Comprehensive antipattern detection section in Elixir subagent
- Full antipattern documentation at `intent/plugins/claude/subagents/elixir/antipatterns.md`
- Antipattern review workflow with systematic approach
- Example usage commands and report formats
- Key principles for antipattern prevention
- Migration function for v2.3.1 to v2.3.2

### Changed

- Enhanced Elixir subagent with antipattern detection capabilities
- Updated systematic review template to include antipattern analysis
- Elixir Doctor now automatically checks for antipatterns during code reviews

### Technical Improvements

- Better code quality guidance through antipattern detection
- More comprehensive code review process
- Proactive detection of common Elixir mistakes

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
intent upgrade  # Automatically handles 2.3.1 to 2.3.2 migration
intent claude subagents sync  # Update installed agents
```

## Using Antipattern Detection

### Check a Single File

```bash
# Ask the Elixir agent to check for antipatterns
"Check lib/my_app/user.ex for antipatterns"
```

### Review an Entire Module

```bash
# Review a module for antipatterns
"Review MyApp.Accounts for common antipatterns"
```

### Focus on Specific Categories

```bash
# Check for specific antipattern types
"Check for process-related antipatterns in lib/my_app/"
```

### Combined with Full Review

```bash
# Complete Elixir Doctor review including antipatterns
"Apply Elixir Doctor and check for antipatterns in MyApp.Users"
```

## Example Antipattern Report

The Elixir Doctor provides detailed reports:

```
## Antipattern Analysis

Found 4 antipatterns in MyApp.Users:

### Code Antipatterns (2)
1. **Non-assertive map access** (line 45)
   - Using `user[:email]` when email is required
   - Remediation: Use `user.email` for required fields

2. **Long parameter list** (line 78)  
   - Function has 7 parameters
   - Remediation: Group related params into maps/structs

### Design Antipatterns (1)
1. **Boolean obsession** (line 123)
   - Using `admin: true, editor: true` options
   - Remediation: Use `:role` atom instead

### Process Antipatterns (1)
1. **Scattered process interfaces** (lines 200-250)
   - Direct GenServer.call/2 usage in multiple places
   - Remediation: Centralize in single interface module
```

## Testing

After upgrading, verify the new functionality:

```bash
# Check version
intent --version  # Should show 2.3.2

# Update agents
intent claude subagents sync

# Verify elixir agent is updated
intent claude subagents show elixir | grep antipattern
```

## Migration Notes

This release includes a smooth migration path from v2.3.1:

- Version configuration is automatically updated
- Existing Elixir subagent installations are updated via `intent claude subagents sync`
- No breaking changes or manual intervention required

## Documentation

- **Full Antipattern Reference**: `intent/plugins/claude/subagents/elixir/antipatterns.md`
- **Elixir Agent Documentation**: Run `intent claude subagents show elixir`
- **Intent Help**: Run `intent help` for general command documentation

## Support

For issues or questions:

- GitHub Issues: <https://github.com/matthewsinclair/intent/issues>
- Documentation: Run `intent help` for command documentation

## Contributors

This release was developed by Matthew Sinclair with the antipatterns documentation sourced from the official Elixir documentation at hexdocs.pm.

---

*Intent v2.3.2 - Structured Development Process with Enhanced Elixir Code Quality*
