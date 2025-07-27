# Tasks - ST0017: Add an Intent sub-agent for Claude Code to Intent

## Tasks

### Phase 1: Infrastructure
- [x] Create agents directory structure in Intent core
- [x] Create agents/.manifest directory for global manifest
- [x] Implement JSON manifest parsing functions in intent_helpers
- [x] Add checksum calculation utility function
- [x] Implement Claude Code detection function
- [x] Create project agent directory template

### Phase 2: Core Commands
- [x] Create bin/intent_agents main command file
- [x] Implement intent_agents_list subcommand
- [x] Implement intent_agents_install subcommand
- [x] Implement intent_agents_sync subcommand
- [x] Implement intent_agents_uninstall subcommand
- [x] Implement intent_agents_show subcommand
- [x] Implement intent_agents_status subcommand
- [x] Add agents command to main intent router
- [x] Update help system with agents commands

### Phase 3: Agent Development
- [x] Create agents/intent directory
- [x] Write Intent sub-agent system prompt (agent.md)
- [x] Create Intent agent metadata.json
- [x] Create agents/elixir directory
- [x] Write Elixir sub-agent system prompt (agent.md)
- [x] Create Elixir agent metadata.json
- [x] Create global-agents.json manifest

### Phase 4: Integration
- [x] Update intent_init to detect Claude and offer agent installation
- [x] Add agent checks to intent_doctor
- [x] Update intent_upgrade to handle agent migration
- [x] Create agent installation documentation
- [ ] Update main README with agent examples

### Phase 5: Testing
- [x] Write unit tests for manifest operations
- [x] Write unit tests for checksum functions
- [x] Write integration tests for agent installation
- [x] Write integration tests for sync operations
- [x] Write end-to-end workflow tests
- [x] Test cross-platform compatibility (macOS/Linux)
- [x] Test Claude detection variations

### Phase 6: Documentation
- [ ] Write agent development guide
- [x] Document agent metadata format
- [x] Create troubleshooting guide
- [ ] Update release notes for v2.1.0

## Task Notes

### Critical Path Items
1. Manifest infrastructure must be complete before command implementation
2. Agent content should be developed in parallel with commands
3. Testing should begin as soon as basic commands work

### Testing Considerations
- Use mock Claude installations for CI/CD
- Test both with and without Claude installed
- Verify backwards compatibility with existing projects

## Dependencies

### External Dependencies
- `jq` for JSON parsing (already required by Intent)
- `sha256sum` or `shasum` for checksums (standard on all platforms)

### Internal Dependencies
- Requires Intent v2.1.0 or higher
- Must maintain compatibility with existing command structure
- Should integrate with existing helper functions

### Sequencing
1. **Infrastructure first**: Manifest and utility functions
2. **Commands second**: Build on infrastructure
3. **Agents parallel**: Can be developed alongside commands
4. **Integration**: After core functionality complete
5. **Testing throughout**: Unit tests as we go, integration tests at end
6. **Documentation last**: Once implementation is stable