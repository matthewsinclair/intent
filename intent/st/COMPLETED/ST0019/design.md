---
verblock: "05 Sep 2025:v0.1: matts - Design specifications for ash-expert agent"
intent_version: 2.3.2
---
# Design - ST0019: Ash-Expert Agent for Modern Ash Framework Code Quality and Architecture

## Approach

Implement a specialized Intent agent focused on Ash Framework expertise through:

1. **4-Tier Quality System**: Structured expertise levels from critical fixes to advanced patterns
2. **"Strict but Helpful Mentor" Philosophy**: Enforce quality gates while providing educational guidance
3. **Modern Pattern Focus**: Promote Ash 3.0+ features over legacy approaches
4. **Documentation Integration**: Leverage existing Intent documentation at intent/docs/ref/ash/
5. **Claude Code Sub-Agent**: Integrate with Intent's agent system for specialized task delegation

## Design Decisions

### 4-Tier Expertise Architecture
**Decision**: Structure agent capabilities into four distinct tiers
**Rationale**: Provides clear escalation path from critical fixes to advanced scenarios, ensuring both beginners and experts get appropriate guidance

### "Strict but Helpful Mentor" Personality
**Decision**: Design agent to be opinionated about quality while remaining educational
**Rationale**: Prevents developers from shipping anti-patterns while teaching proper Ash principles through concrete examples

### Anti-Pattern Detection Focus
**Decision**: Prioritize flagging direct Ecto usage and other critical anti-patterns
**Rationale**: Prevents the most common and damaging mistakes that bypass Ash's benefits

### Modern Ash 3.0+ Pattern Promotion
**Decision**: Exclusively recommend current best practices, not legacy approaches
**Rationale**: Ensures codebases use the most maintainable and performant patterns available

### Documentation-Driven Responses
**Decision**: Always reference intent/docs/ref/ash/ documentation in responses
**Rationale**: Maintains consistency with project patterns and provides learning resources

## Architecture

### Agent Structure
```
ash-expert/
├── agent.md                   # Claude Code agent definition (164 lines)
├── metadata.json              # Agent configuration with tool specs
└── [Integration with Intent agent system]
```

### 4-Tier Capability System

**Tier 1: Critical Quality Gates (Must Fix Immediately)**
- Ecto/Ash pattern detection and flagging
- Resource definition validation
- Query anti-pattern identification
- Action implementation review

**Tier 2: Modern Pattern Promotion (Architectural Guidance)**
- Ash 3.0+ feature suggestions
- Domain-driven design enforcement
- Authorization pattern review
- Performance optimization identification

**Tier 3: Development Quality (Best Practices)**
- Migration generation guidance
- Test template generation
- Error handling enforcement
- Documentation integration

**Tier 4: Advanced Scenarios (Expert-Level)**
- Multi-resource transaction review
- Change tracking implementation
- Code interface generation

### Integration Points
- **Intent Project Integration**: Works within steel thread methodology
- **Agent Ecosystem**: Complements elixir and worker-bee agents
- **Documentation System**: References intent/docs/ref/ash/ patterns
- **Quality Enforcement**: Integrates with project quality gates

## Alternatives Considered

### Alternative 1: Extend Elixir Agent with Ash Knowledge
**Rejected**: Would dilute the elixir agent's focus and create knowledge overlap
**Chosen**: Dedicated ash-expert agent with specialized Ash Framework knowledge

### Alternative 2: Simple Pattern Checker
**Rejected**: Would only catch basic issues without providing educational value
**Chosen**: Comprehensive mentor that teaches while enforcing quality

### Alternative 3: Framework-Agnostic Data Layer Agent
**Rejected**: Ash has unique patterns that generic approaches wouldn't handle well
**Chosen**: Ash-specific agent that understands framework philosophy and patterns

### Alternative 4: Documentation-Only Approach
**Rejected**: Static documentation doesn't provide context-aware guidance
**Chosen**: Interactive agent that applies documentation patterns to specific code

### Alternative 5: Flat Expertise Model
**Rejected**: Would be overwhelming for beginners and insufficient for experts
**Chosen**: 4-tier system that scales guidance to developer needs and code complexity