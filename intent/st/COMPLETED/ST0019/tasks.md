---
verblock: "05 Sep 2025:v0.1: matts - Task breakdown for ash-expert agent implementation"
intent_version: 2.3.2
---
# Tasks - ST0019: Ash-Expert Agent for Modern Ash Framework Code Quality and Architecture

## Tasks

### Phase 1: Agent Foundation
- [x] Create agent directory structure in intent/plugins/claude/subagents/ash-expert/
- [x] Design comprehensive system prompt focused on Ash Framework expertise
- [x] Implement "strict but helpful mentor" personality in agent definition
- [x] Create metadata.json with proper tool specifications and context sources
- [x] Test agent installation and basic functionality

### Phase 2: 4-Tier Expertise System
- [x] Design 4-tier capability architecture (Critical, Modern, Quality, Advanced)
- [x] Implement Tier 1: Critical quality gates for anti-pattern detection
- [x] Implement Tier 2: Modern Ash 3.0+ pattern promotion
- [x] Implement Tier 3: Development quality best practices
- [x] Implement Tier 4: Advanced scenarios for expert-level guidance
- [x] Create structured escalation from basic to advanced capabilities

### Phase 3: Anti-Pattern Detection System
- [x] Design critical anti-pattern identification patterns
- [x] Implement Ecto/Ash pattern detection and flagging
- [x] Implement resource definition validation
- [x] Implement query anti-pattern identification (N+1, manual loops)
- [x] Implement action implementation review patterns
- [x] Create immediate flagging system for production-critical issues

### Phase 4: Modern Pattern Promotion
- [x] Design Ash 3.0+ feature promotion system
- [x] Create bulk operation pattern suggestions
- [x] Implement atomic update pattern promotion
- [x] Design domain-driven design boundary enforcement
- [x] Create authorization policy review patterns
- [x] Implement performance optimization identification

### Phase 5: Documentation Integration
- [x] Design documentation reference system for intent/docs/ref/ash/
- [x] Implement context_sources integration with usage rules
- [x] Create ash_postgres usage rules integration
- [x] Create ash_phoenix usage rules integration
- [x] Ensure consistency with existing project patterns
- [x] Add learning resource references in agent responses

### Phase 6: Agent Ecosystem Integration
- [x] Design integration with existing elixir agent
- [x] Design integration with existing worker-bee agent
- [x] Ensure complementary capabilities without overlap
- [x] Test agent interaction patterns with Intent system
- [x] Validate agent follows Intent project conventions

### Phase 7: Quality Assurance & Testing
- [x] Test agent with various Ash resource patterns
- [x] Validate anti-pattern detection accuracy
- [x] Test modern pattern suggestions for effectiveness
- [x] Verify documentation integration works correctly
- [x] Ensure "strict but helpful" personality balance

### Phase 8: Integration & Deployment
- [x] Integrate with Intent's agent management system
- [x] Update global-agents.json manifest
- [x] Update AGENTS.md with ash-expert agent description
- [x] Test agent installation and availability
- [x] Validate agent works within Claude Code sub-agent system

### Phase 9: Intent Project Documentation
- [x] Create steel thread documentation (ST0019)
- [x] Update Intent documentation with ash-expert agent
- [x] Document agent in Intent's available agents list
- [x] Ensure agent follows Intent project conventions
- [x] Complete steel thread files (info.md, design.md, impl.md, tasks.md)

## Task Notes

### Critical Success Factors
- **4-Tier Expertise Balance**: Essential for scaling guidance from beginner to expert levels without overwhelming users
- **Anti-Pattern Focus**: Must catch critical Ecto bypasses and resource definition errors that cause production issues
- **Modern Pattern Promotion**: Agent should exclusively promote Ash 3.0+ patterns to prevent technical debt
- **Documentation Integration**: All responses must reference intent/docs/ref/ash/ for consistency and learning

### Implementation Approach
Tasks were completed in logical sequence with agent foundation first, followed by the core expertise system, then integration and testing. The "strict but helpful mentor" personality was integrated throughout rather than added as an afterthought.

### Quality Assurance
Each phase included validation that the agent provides appropriate guidance for its tier level. Anti-pattern detection was tested with common Ash/Ecto mistakes, and modern pattern promotion was validated against Ash 3.0+ best practices.

## Dependencies

### Prerequisites Completed
- ST0017: Intent Agent System infrastructure (provides agent installation framework)
- Intent v2.3.2 plugin architecture and Claude subagent integration
- Claude Code sub-agent system compatibility
- Existing elixir and worker-bee agents for ecosystem integration

### External Dependencies
- Claude Code sub-agent system for agent execution
- Intent's plugin architecture for agent registration
- Ash Framework knowledge base and documentation
- intent/docs/ref/ash/ documentation structure

### Internal Dependencies
- Agent definition must be complete before testing can begin
- 4-tier system must be designed before individual tier implementation
- Anti-pattern detection requires comprehensive Ash/Ecto pattern knowledge
- Documentation integration requires existing usage rules and patterns

### Sequential Requirements
1. Agent foundation (system prompt, metadata) enables basic functionality
2. 4-tier system design enables structured capability implementation
3. Anti-pattern detection requires deep Ash Framework pattern knowledge
4. Modern pattern promotion requires understanding of Ash 3.0+ features
5. Documentation integration requires all features to reference consistently
6. Ecosystem integration requires understanding of elixir and worker-bee agents

All dependencies were satisfied during implementation, with the agent now providing comprehensive Ash Framework expertise while maintaining consistency with Intent's project methodology and agent ecosystem.

### Completion Status
All tasks completed successfully. The ash-expert agent is fully implemented and integrated with Intent's agent system, providing comprehensive Ash Framework expertise through the 4-tier system while maintaining the "strict but helpful mentor" approach.