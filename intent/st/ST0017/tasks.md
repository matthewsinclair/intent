# Tasks - ST0017: Add an Intent sub-agent for Claude Code to Intent

## Tasks

### Phase 1: Infrastructure
- [ ] Create agents directory structure in Intent core
- [ ] Create agents/.manifest directory for global manifest
- [ ] Implement JSON manifest parsing functions in intent_helpers
- [ ] Add checksum calculation utility function
- [ ] Implement Claude Code detection function
- [ ] Create project agent directory template

### Phase 2: Core Commands
- [ ] Create bin/intent_agents main command file
- [ ] Implement intent_agents_list subcommand
- [ ] Implement intent_agents_install subcommand
- [ ] Implement intent_agents_sync subcommand
- [ ] Implement intent_agents_uninstall subcommand
- [ ] Implement intent_agents_show subcommand
- [ ] Implement intent_agents_status subcommand
- [ ] Add agents command to main intent router
- [ ] Update help system with agents commands

### Phase 3: Agent Development
- [ ] Create agents/intent directory
- [ ] Write Intent sub-agent system prompt (agent.md)
- [ ] Create Intent agent metadata.json
- [ ] Create agents/elixir directory
- [ ] Write Elixir sub-agent system prompt (agent.md)
- [ ] Create Elixir agent metadata.json
- [ ] Create global-agents.json manifest

### Phase 4: Integration
- [ ] Update intent_init to detect Claude and offer agent installation
- [ ] Add agent checks to intent_doctor
- [ ] Update intent_upgrade to handle agent migration
- [ ] Create agent installation documentation
- [ ] Update main README with agent examples

### Phase 5: Testing
- [ ] Write unit tests for manifest operations
- [ ] Write unit tests for checksum functions
- [ ] Write integration tests for agent installation
- [ ] Write integration tests for sync operations
- [ ] Write end-to-end workflow tests
- [ ] Test cross-platform compatibility (macOS/Linux)
- [ ] Test Claude detection variations

### Phase 6: Documentation
- [ ] Write agent development guide
- [ ] Document agent metadata format
- [ ] Create troubleshooting guide
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
- Requires Intent v2.0.0 or higher
- Must maintain compatibility with existing command structure
- Should integrate with existing helper functions

### Sequencing
1. **Infrastructure first**: Manifest and utility functions
2. **Commands second**: Build on infrastructure
3. **Agents parallel**: Can be developed alongside commands
4. **Integration**: After core functionality complete
5. **Testing throughout**: Unit tests as we go, integration tests at end
6. **Documentation last**: Once implementation is stable