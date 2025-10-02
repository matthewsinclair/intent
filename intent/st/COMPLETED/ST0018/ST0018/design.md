---
verblock: "20 Aug 2025:v0.1: matts - Initial design"
intent_version: 2.3.0
---
# ST0018: Design - AGENTS.md Support

## Architecture Overview

The implementation introduces a plugin architecture for Intent, with both AGENTS.md support and Claude subagents implemented as plugins.

### Plugin Structure
```
intent/plugins/
├── agents/           # AGENTS.md plugin
│   ├── bin/         # Plugin commands
│   ├── templates/   # AGENTS.md templates
│   └── config.json  # Plugin configuration
└── claude/          # Claude plugin
    ├── subagents/   # Subagent definitions
    └── bin/         # Plugin commands
```

## Command Structure Changes

### Before (v2.2.0)
```bash
intent agents init
intent agents install
intent agents list
```

### After (v2.3.0)
```bash
# AGENTS.md commands (new)
intent agents init        # Create AGENTS.md
intent agents sync        # Update AGENTS.md
intent agents validate    # Check compliance

# Claude subagents (renamed)
intent claude subagents init
intent claude subagents install
intent claude subagents list
```

## Plugin Dispatch

The main `bin/intent` script now handles plugin dispatch:

1. Detects plugin commands (agents, claude)
2. Routes to appropriate plugin bin script
3. Maintains backward compatibility for core commands

## AGENTS.md Implementation

### File Location
- Real file: `intent/llm/AGENTS.md`
- Symlink: `./AGENTS.md` → `intent/llm/AGENTS.md`

### Generation Logic
The AGENTS.md generator:
1. Detects project type (Node.js, Python, etc.)
2. Extracts build/test commands
3. Includes Intent-specific information
4. Lists installed Claude subagents
5. References steel threads and backlog

### Template System
- Default template provided
- Framework-specific templates planned
- Customizable sections

## Migration Strategy

### Clean Break Approach
- No deprecation period
- Clear error messages guide users to new commands
- intent_upgrade handles v2.2.0 → v2.3.0 migration

### Migration Tasks
1. Move agents/ → intent/plugins/claude/subagents/
2. Update command references
3. Generate initial AGENTS.md
4. Update configuration

## Benefits

1. **Standards Compliance**: Supports universal AGENTS.md spec
2. **Plugin Architecture**: Extensible for future features
3. **Clean Separation**: Core vs. plugins clearly defined
4. **Better Organization**: All extensions in intent/plugins/
5. **Future-Proof**: Easy to add new plugins

## Testing Strategy

1. Unit tests for each plugin command
2. Integration tests for plugin dispatch
3. Migration tests for upgrade path
4. Validation tests for AGENTS.md generation

## Success Criteria

- ✅ Plugin architecture implemented
- ✅ AGENTS.md generation working
- ✅ Claude subagents relocated and functional
- ⏳ All tests passing
- ⏳ Documentation updated
- ⏳ Migration path tested