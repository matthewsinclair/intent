# Implementation - ST0017: Add an Intent sub-agent for Claude Code to Intent

## Implementation Plan

### Phase 1: Infrastructure (Days 1-2)

1. Create directory structures
   - Add `agents/` to Intent core
   - Create `.manifest/` subdirectories
   - Set up project agent locations

2. Implement manifest management
   - JSON parsing/writing functions
   - Checksum calculation
   - Manifest validation

3. Add Claude Code detection
   - Check for `.claude` directory
   - Verify `claude` command availability
   - Handle missing Claude gracefully

### Phase 2: Core Commands (Days 3-4)

1. Implement `intent_agents` base command
   - Command routing
   - Help system integration
   - Error handling framework

2. Build core subcommands
   - `list` - Show available/installed agents
   - `install` - Copy agents to Claude config
   - `sync` - Update modified agents
   - `uninstall` - Remove managed agents
   - `show` - Display agent details

### Phase 3: Agent Development (Days 5-6)

1. Create Intent sub-agent
   - System prompt for Intent awareness
   - Steel thread methodology knowledge
   - Command reference

2. Create Elixir sub-agent
   - Elixir best practices
   - Usage rules integration
   - Functional programming focus

### Phase 4: Integration & Testing (Days 7-8)

1. Integration with existing commands
   - Auto-install on `intent init`
   - Doctor command checks
   - Help system updates

2. Comprehensive testing
   - Unit tests for manifest operations
   - Integration tests for sync
   - End-to-end workflow tests

## Technical Details

### Command Implementation Structure

```bash
# intent_agents main command
#!/bin/bash
source "$INTENT_BIN/intent_helpers"

case "$1" in
  list)     shift; intent_agents_list "$@" ;;
  install)  shift; intent_agents_install "$@" ;;
  sync)     shift; intent_agents_sync "$@" ;;
  uninstall) shift; intent_agents_uninstall "$@" ;;
  show)     shift; intent_agents_show "$@" ;;
  status)   shift; intent_agents_status "$@" ;;
  *)        intent_agents_help ;;
esac
```

### Manifest Operations

```bash
# Read manifest
read_manifest() {
  local manifest_file="$1"
  if [ -f "$manifest_file" ]; then
    cat "$manifest_file" | jq '.'
  else
    echo '{"version": "1.0.0", "agents": []}'
  fi
}

# Calculate checksum
calculate_checksum() {
  local file="$1"
  if command -v sha256sum >/dev/null; then
    sha256sum "$file" | cut -d' ' -f1
  else
    shasum -a 256 "$file" | cut -d' ' -f1
  fi
}
```

### Claude Code Detection

```bash
detect_claude() {
  if [ -d "$HOME/.claude" ] || command -v claude >/dev/null 2>&1; then
    return 0
  else
    return 1
  fi
}

# Auto-install prompt
if detect_claude && [ "$1" = "init" ]; then
  echo "Claude Code detected. Install Intent agents? [Y/n]"
  read -r response
  if [[ "$response" =~ ^[Yy]?$ ]]; then
    intent agents install --all
  fi
fi
```

### Agent Metadata Format

```json
{
  "name": "intent",
  "version": "2.0.0",
  "description": "Intent-aware development assistant",
  "author": "Intent Contributors",
  "tools": ["Bash", "Read", "Write", "Edit", "Grep", "WebFetch"],
  "tags": ["project-management", "steel-threads", "development"],
  "min_intent_version": "2.0.0",
  "min_claude_version": null
}
```

### Conflict Resolution

```bash
handle_conflict() {
  local target="$1"
  local source="$2"

  if [ -f "$target" ]; then
    local target_sum=$(calculate_checksum "$target")
    local source_sum=$(calculate_checksum "$source")

    if [ "$target_sum" != "$source_sum" ]; then
      echo "Warning: Agent already exists and has been modified"
      echo "Target: $target"
      echo "[O]verwrite, [S]kip, [D]iff, [B]ackup?"
      read -r choice

      case "$choice" in
        [Oo]) cp "$source" "$target" ;;
        [Ss]) return 1 ;;
        [Dd]) diff "$target" "$source" | less ;;
        [Bb])
          backup="$target.backup.$(date +%Y%m%d_%H%M%S)"
          cp "$target" "$backup"
          cp "$source" "$target"
          ;;
      esac
    fi
  else
    cp "$source" "$target"
  fi
}
```

## Testing Strategy

### Unit Tests

- Manifest reading/writing
- Checksum calculation
- Path resolution
- JSON validation

### Integration Tests

- Agent installation flow
- Sync with modifications
- Uninstall cleanup
- Claude detection

### End-to-End Tests

```bash
# Test full workflow
test_agent_workflow() {
  # Setup
  intent init test-project
  cd test-project

  # Install
  intent agents install intent
  assert_file_exists ".claude/agents/intent.md"

  # Verify manifest
  assert_json_contains "intent/agents/.manifest/installed-agents.json" \
    '.installed[0].name == "intent"'

  # Sync
  touch .claude/agents/intent.md
  intent agents sync

  # Uninstall
  intent agents uninstall intent
  assert_file_not_exists ".claude/agents/intent.md"
}
```

## Rollout Plan

### Release Strategy

1. **v2.1.0-beta**: Initial release with agent support
   - Core commands functional
   - Intent and Elixir agents included
   - Documentation complete

2. **v2.1.0**: Stable release
   - Bug fixes from beta feedback
   - Performance optimizations
   - Additional agents based on demand

### Migration Steps

1. No breaking changes - additive feature
2. Existing projects gain agent commands automatically
3. Optional auto-install on first use
4. Clear documentation in release notes

### Documentation Updates

- Update main README with agent examples
- Add agents section to user guide
- Create agent development guide
- Include in `intent help` system

## Challenges & Solutions

### Challenge 1: Cross-Platform Compatibility

**Issue**: Checksum commands differ between macOS/Linux
**Solution**: Detect available command and use appropriate syntax

### Challenge 2: Claude Installation Variations

**Issue**: Claude might be installed in different ways (homebrew, direct, etc.)
**Solution**: Multiple detection methods, graceful fallback

### Challenge 3: User Customization Preservation

**Issue**: Users might modify agents after installation
**Solution**: Checksum tracking, conflict resolution options

### Challenge 4: Backwards Compatibility

**Issue**: Need to support projects without agent capability
**Solution**: Additive design, no breaking changes to existing commands
