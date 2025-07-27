# Design - ST0017: Add an Intent sub-agent for Claude Code to Intent

## Approach

Implement a sync-based agent management system that:

1. **Maintains agents within Intent's structure** for version control and distribution
2. **Syncs agents to Claude Code's configuration** when needed
3. **Tracks installation state** using manifest files
4. **Supports both global and project-specific agents**

The system will integrate seamlessly with existing Intent commands and respect the separation between Intent core (global) and project-specific configurations.

## Design Decisions

### 1. Sync vs Symlinks

**Decision**: Use file sync instead of symbolic links
**Rationale**:

- Claude Code's symlink support is uncertain
- Cross-platform compatibility (Windows symlinks differ)
- Explicit sync provides validation opportunity
- More predictable behaviour

### 2. Manifest-Based Tracking

**Decision**: Use JSON manifests to track agent state
**Rationale**:

- Clear record of what's installed vs available
- Enables clean uninstall of Intent-managed agents
- Supports modification detection via checksums
- Allows versioning and updates

### 3. Dual-Level Agent System

**Decision**: Support both global (Intent-wide) and local (project-specific) agents
**Rationale**:

- Global agents ship with Intent (intent, elixir)
- Projects can define custom agents
- Clear separation of concerns
- Flexible deployment options

### 4. Agent Structure

**Decision**: Agents as directories with metadata
**Rationale**:

- Richer than single markdown files
- Supports versioning and dependencies
- Enables future extensions
- Clear organization

## Architecture

### Directory Structure

```
$INTENT_HOME/                          # Global Intent installation
├── agents/                            # Global agents repository
│   ├── .manifest/
│   │   └── global-agents.json         # Available global agents
│   ├── intent/
│   │   ├── agent.md                   # Claude sub-agent definition
│   │   └── metadata.json              # Version, description, etc.
│   └── elixir/
│       ├── agent.md
│       └── metadata.json

$PROJECT_DIR/                          # User's project
├── intent/
│   └── agents/                        # Project-specific agents
│       ├── .manifest/
│       │   └── installed-agents.json  # Tracks installations
│       └── custom-agent/
│           ├── agent.md
│           └── metadata.json
└── .claude/
    └── agents/                        # Claude Code reads from here
        ├── intent.md                  # Synced from global
        └── custom-agent.md            # Synced from local
```

### Manifest Schemas

#### Global Agents Manifest

```json
{
  "version": "1.0.0",
  "agents": [
    {
      "name": "intent",
      "version": "2.0.0",
      "description": "Intent-aware development assistant",
      "path": "intent",
      "checksum": "sha256:abc123...",
      "tools": ["Bash", "Read", "Write", "Edit"],
      "min_intent_version": "2.0.0"
    }
  ]
}
```

#### Installed Agents Manifest

```json
{
  "version": "1.0.0",
  "project": "my-project",
  "installed": [
    {
      "name": "intent",
      "source": "global",
      "source_path": "$INTENT_HOME/agents/intent",
      "version": "2.0.0",
      "installed_at": "2025-01-27T10:00:00Z",
      "checksum": "sha256:abc123...",
      "modified": false
    }
  ]
}
```

### Command Structure

```bash
# Core commands
intent agents list         # Show available and installed agents
intent agents install      # Install agent(s) to Claude config
intent agents sync         # Update modified agents
intent agents uninstall    # Remove Intent-managed agents
intent agents show         # Display agent details

# Additional commands
intent agents status       # Check installation health
intent agents update       # Update to newer versions
```

### Integration Flow

```
1. Developer runs: intent agents install intent
   ↓
2. Intent reads: $INTENT_HOME/agents/.manifest/global-agents.json
   ↓
3. Copies: $INTENT_HOME/agents/intent/agent.md
   ↓
4. To: $PROJECT_DIR/.claude/agents/intent.md
   ↓
5. Updates: $PROJECT_DIR/intent/agents/.manifest/installed-agents.json
   ↓
6. Claude Code can now use the Intent sub-agent
```

## Alternatives Considered

### 1. Direct .claude Management

**Approach**: Let users manually copy agents to .claude/agents/
**Rejected because**:

- No tracking of Intent-managed vs user agents
- No update mechanism
- Poor user experience

### 2. Symbolic Links

**Approach**: Symlink from .claude/agents/ to intent/agents/
**Rejected because**:

- Uncertain Claude Code support
- Platform compatibility issues
- Security concerns

### 3. Single Global Manifest

**Approach**: One manifest in Intent core tracking all projects
**Rejected because**:

- Violates project isolation
- Intent core shouldn't track project state
- Scaling issues

### 4. Embedding in Intent Binary

**Approach**: Include agents directly in Intent commands
**Rejected because**:

- Not how Claude sub-agents work
- Would require Intent to act as intermediary
- Loses benefits of Claude's agent system
