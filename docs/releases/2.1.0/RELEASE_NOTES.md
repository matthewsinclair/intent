# Intent v2.1.0 Release Notes

## Release Date: July 27, 2025

## Overview

Intent v2.1.0 is a maintenance release that enhances the Agent system introduced in v2.0.0 and fixes critical bugs in the upgrade process.

## What's Changed

### 🐛 Critical Bug Fixes

#### Agent Directory Structure

Fixed a critical bug where the upgrade process was creating agent directories in the wrong location:

- **Problem**: `intent upgrade` was creating `./agents/` at the project root
- **Solution**: Agent directories are now correctly placed in `./intent/agents/`
- **Impact**: Projects upgraded with the buggy version will have an incorrect `agents/` directory at root

**To fix affected projects:**
```bash
# Remove the incorrectly placed directory
rm -rf ./agents

# The correct location is:
# ./intent/agents/  (for project-specific agents)
```

### 🔧 Agent System Improvements

- **`intent agents init`**: Now required before installing agents (introduced in v2.0.0)
- **Project agent initialization**: Fixed to create directories in the correct location
- **Upgrade process**: No longer incorrectly preserves root-level agent directories

### 📋 Updated Components

- `bin/intent_helpers`: Fixed `migrate_v2_0_to_v2_1()` function
- `bin/intent_agents`: Fixed project agent initialization path
- `bin/intent_upgrade`: Removed incorrect agent directory preservation

## Agent Directory Structure (Clarified)

The correct agent directory structure is:

```
$INTENT_HOME/agents/          # Global agents shipped with Intent
./intent/agents/              # Project-specific custom agents  
~/.claude/agents/             # Where Claude Code reads installed agents
~/.intent/agents/             # Intent's tracking of installed agents
```

## Installation & Upgrade

### Upgrading from v2.0.0

```bash
# Update Intent installation
cd /path/to/intent
git pull

# Upgrade your project
cd /path/to/your/project
intent upgrade
```

### For Projects with Incorrect Agent Directories

If you previously ran `intent upgrade` and have an `agents/` directory at your project root:

```bash
# Check if you have the incorrect structure
ls -la ./agents

# If it exists, remove it (the correct location is ./intent/agents/)
rm -rf ./agents
```

## Testing

All 165 tests pass, including:
- Agent directory creation tests
- Upgrade process tests  
- Agent initialization tests

## Known Issues

None at this time.

## Contributors

- Matthew Sinclair (@matts)

## Support

- **Issues**: https://github.com/matthewsinclair/intent/issues
- **Help**: Run `intent help agents` for agent-specific help