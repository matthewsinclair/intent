---
verblock: "05 Sep 2025:v0.1: matts - Implementation details for ash-expert agent"
intent_version: 2.3.2
---
# Implementation - ST0019: Ash-Expert Agent for Modern Ash Framework Code Quality and Architecture

## Implementation

The ash-expert agent was implemented as a comprehensive Ash Framework specialist with two main components:

### 1. Claude Code Agent Integration
- **Agent Definition**: `intent/plugins/claude/subagents/ash-expert/agent.md` with extensive system prompt (164 lines)
- **Metadata Configuration**: `intent/plugins/claude/subagents/ash-expert/metadata.json` with tool specifications
- **Installation**: Integrates with Intent's agent management system via `intent claude subagents install ash-expert`

### 2. 4-Tier Expertise System
Four structured capability tiers provide escalating levels of Ash Framework guidance:
- **Tier 1**: Critical quality gates for immediate fixes
- **Tier 2**: Modern pattern promotion for architectural guidance  
- **Tier 3**: Development quality best practices
- **Tier 4**: Advanced scenarios for expert-level implementations

## Code Examples

### Agent System Prompt Structure
```markdown
---
name: ash-expert
description: Modern Ash 3.0+ specialist for code quality, best practices, and architectural guidance
tools: Bash, Read, Write, Edit, Grep, Glob, LS
---

You are a specialized Ash Framework expert with deep expertise in modern Ash 3.0+ patterns...

## Your Role - The "Strict but Helpful Mentor"

When working with developers, you should:
1. **Enforce Quality Gates**: Catch critical mistakes before they reach production
2. **Promote Modern Patterns**: Suggest Ash 3.0+ approaches over legacy patterns
3. **Provide Concrete Examples**: Show actual code transformations, not abstract advice
```

### Agent Usage Patterns
```elixir
# Resource Quality Review
Task(
  description="Review Payment resource for Ash best practices",
  prompt="Analyze lib/my_app/resources/payment.ex for anti-patterns, suggest modern Ash 3.0+ improvements",
  subagent_type="ash-expert"
)

# Query Optimization
Task(
  description="Optimize Ash query performance", 
  prompt="Review PaymentService.list_payments/1 - fix post-filtering issues and show proper Ash.Query patterns",
  subagent_type="ash-expert"
)
```

### Critical Anti-Pattern Detection
The agent flags these issues immediately:
```elixir
# ❌ Direct Ecto bypass
Repo.all(Payment)
Ecto.Changeset.change(payment, %{status: :paid})

# ✅ Proper Ash patterns
Ash.read!(Payment)
Ash.Changeset.for_update(payment, :mark_paid)
```

### Modern Ash 3.0+ Pattern Promotion
```elixir
# ❌ Legacy loop patterns
for user <- users, do: update_user_status(user, :active)

# ✅ Modern bulk operations
Ash.bulk_update!(User, :activate, %{status: :active})
```

## Technical Details

### Agent File Structure
```
intent/plugins/claude/subagents/ash-expert/
├── agent.md                  # Claude Code agent definition (164 lines)
└── metadata.json             # Agent configuration (14 lines)
```

### Metadata Configuration
```json
{
  "name": "ash-expert",
  "version": "1.0.0", 
  "description": "Modern Ash 3.0+ specialist providing comprehensive code quality enforcement...",
  "author": "Intent Development Team",
  "tools": ["Bash", "Read", "Write", "Edit", "Grep", "Glob", "LS"],
  "tags": ["ash", "ash-framework", "elixir", "code-quality", "performance", "domain-driven-design"],
  "context_sources": [
    "intent/docs/ref/ash/",
    "intent/docs/ref/ash/ash_usage_rules.md",
    "intent/docs/ref/ash/deps/ash_postgres/usage-rules.md",
    "intent/docs/ref/ash/deps/ash_phoenix/usage-rules.md"
  ]
}
```

### 4-Tier Capability Mapping
Each tier targets specific developer needs and code complexity levels:

**Tier 1: Critical Quality Gates**
- Pattern detection using systematic code analysis
- Immediate flagging of Ecto/Ash anti-patterns
- Resource definition validation preventing cast errors
- Action implementation review for proper Ash usage

**Tier 2: Modern Pattern Promotion**
- Ash 3.0+ feature suggestions with concrete examples
- Domain-driven design boundary validation
- Authorization policy review and security gap identification
- Performance optimization through modern Ash patterns

**Tier 3: Development Quality**
- ash_postgres migration guidance with proper constraints
- Test template generation for Ash-specific testing patterns
- Error handling enforcement using Ash error system
- Documentation integration with intent/docs/ref/ash/

**Tier 4: Advanced Scenarios**
- Multi-resource transaction pattern validation
- Change tracking and audit trail implementation guidance
- Code interface generation for domain-driven APIs

### Integration Architecture
The agent integrates with Intent's ecosystem through:
- **Steel Thread Awareness**: References architectural decisions in steel threads
- **Documentation Integration**: Always references intent/docs/ref/ash/ patterns
- **Agent Ecosystem**: Complements elixir (general Elixir patterns) and worker-bee (WDD architecture) agents
- **Quality Gates**: Integrates with project quality enforcement workflows

## Challenges & Solutions

### Challenge 1: Balancing Strictness with Helpfulness
**Problem**: Agent needed to be opinionated about quality without being discouraging
**Solution**: Implemented "strict but helpful mentor" personality that explains the "why" behind quality requirements and provides concrete examples for improvements

### Challenge 2: Ash Framework Complexity
**Problem**: Ash has many nuanced patterns that generic advice wouldn't handle well
**Solution**: Created 4-tier system that scales from basic anti-pattern detection to advanced transaction patterns, allowing appropriate guidance for different skill levels

### Challenge 3: Integration with Existing Agent Ecosystem
**Problem**: Needed to complement existing elixir and worker-bee agents without overlap
**Solution**: Focused specifically on Ash Framework patterns while referencing other agents for general Elixir (elixir agent) and architecture (worker-bee agent) guidance

### Challenge 4: Documentation Context Awareness
**Problem**: Agent needed to stay current with project-specific Ash patterns and usage rules
**Solution**: Built-in context_sources that reference intent/docs/ref/ash/ documentation, ensuring consistency with established project patterns

### Challenge 5: Modern Pattern Promotion
**Problem**: Needed to promote Ash 3.0+ patterns over legacy approaches without breaking existing code
**Solution**: Agent suggests modern patterns with migration strategies and explains benefits, allowing developers to upgrade incrementally

### Challenge 6: Quality Gate Implementation
**Problem**: Needed to catch critical mistakes without overwhelming developers
**Solution**: Prioritized Tier 1 (critical) issues that prevent production problems while organizing other guidance into structured tiers

## Key Implementation Insights

### "Strict but Helpful Mentor" Success
The agent personality strikes the right balance between quality enforcement and education. Developers receive firm guidance on anti-patterns while understanding the reasoning and getting concrete improvement examples.

### 4-Tier Expertise Scaling
The tiered approach allows the agent to provide appropriate guidance regardless of developer skill level or code complexity. Beginners get critical fixes while experts get advanced pattern guidance.

### Documentation Integration Value
Always referencing intent/docs/ref/ash/ ensures consistency with project standards and provides developers with learning resources beyond the immediate interaction.

### Anti-Pattern Focus Impact
Prioritizing detection of direct Ecto usage and other critical anti-patterns prevents the most damaging mistakes that completely bypass Ash's benefits.

### Modern Pattern Promotion Effectiveness  
Exclusively promoting Ash 3.0+ patterns ensures codebases use current best practices, improving maintainability and performance while preventing technical debt accumulation.

## Files Created

**Total**: 2 files implementing comprehensive Ash Framework expertise
**Core Agent**: agent.md (164 lines of detailed system prompt and usage examples)
**Configuration**: metadata.json (14 lines with comprehensive tags and context sources)

The implementation provides a specialized foundation for Ash Framework quality enforcement while maintaining educational value and integration with Intent's project methodology.