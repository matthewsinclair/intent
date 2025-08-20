# Socrates Agent - CTO Review Mode

## Overview

The Socrates agent implements "CTO Review Mode" - a structured approach to technical decision-making through Socratic dialog between two expert personas. This methodology transforms technical reviews from checkbox exercises into genuine exploration.

## Purpose

Technical decisions often suffer from single-perspective analysis, unexplored edge cases, and insufficient challenge to assumptions. The Socrates agent addresses this by facilitating a structured conversation between:

- **Socrates (The CTO)**: A strategic thinker with 30+ years experience
- **Plato (The Tech Lead)**: An implementation expert with deep technical knowledge

## When to Use

### Ideal For:
- Architecture decisions (microservices vs monolith, database selection)
- Technology selection (framework choices, build vs buy)
- Complex refactoring strategies
- API design choices
- Performance optimization approaches
- Integration planning

### Not Necessary For:
- Simple bug fixes
- Routine updates
- Well-established patterns
- Emergency hotfixes

## How It Works

The agent facilitates a dialog through five phases:

1. **Opening**: CTO asks fundamental questions about the problem
2. **Exploration**: Multiple rounds of questions and detailed analysis
3. **Challenge**: CTO challenges assumptions, Tech Lead defends complexity
4. **Synthesis**: Both work toward pragmatic recommendations
5. **Conclusion**: Agreement on approach, trade-offs, and next steps

## Usage with Claude Code

### Direct Invocation

Simply ask Claude to use CTO Review Mode for your technical decision:

```
"I need to decide between PostgreSQL and DynamoDB for our new service. 
Can you conduct a CTO Review Mode dialog exploring this decision? 
Context: 5-person team, expecting 1M users in year one, 
strong PostgreSQL experience but no NoSQL experience."
```

### Using with Task Tool

When delegating to the Socrates agent as a sub-agent:

```javascript
Task(
  description="Review authentication architecture",
  prompt="Conduct CTO Review Mode dialog for our authentication system redesign. We need to support enterprise SSO, maintain 15-minute token expiry for compliance, and ensure zero-downtime migration.",
  subagent_type="socrates"
)
```

## Integration with Intent

The Socrates agent is Intent-aware and integrates seamlessly:

- References steel threads in dialogs
- Outputs can become part of steel thread design documentation
- Dialog conclusions can generate backlog tasks
- Facilitates team alignment through shared decision records

## Example Output Structure

```markdown
# CTO Review Mode: [Decision Topic]

## The Socratic Dialog

**Socrates (CTO):** What's driving the need for...?

**Plato (Tech Lead):** We're facing three key constraints...

[Dialog continues through exploration]

## Recommendation
[Clear, actionable recommendation]

## Key Trade-offs
- Performance vs simplicity
- Time to market vs technical debt
- [etc.]

## Next Steps
- [ ] Create steel thread for implementation
- [ ] Document in design.md
- [ ] Create backlog tasks
```

## Benefits

1. **Comprehensive Analysis**: Forces exploration from multiple angles
2. **Decision Documentation**: Creates permanent record of reasoning
3. **Challenge Assumptions**: Natural questioning reveals blind spots
4. **Educational Value**: Junior developers learn from dialog format
5. **Async Reviews**: Team members can contribute asynchronously

## Origin

This methodology was developed by Matthew Sinclair and successfully used across multiple projects including MeetZaya, Anvil, and Laksa. The approach is documented in detail in:

- [MeetZaya Blog: CTO Review Socratic Dialog AI](https://github.com/meetzaya/meetzaya/blob/main/intent/docs/blog/B002_cto_review_socratic_dialog_ai.md)
- [Technical Note TN027](https://github.com/meetzaya/meetzaya/blob/main/intent/docs/notes/TN027_cto_review_socratic_dialog.md)

## Installation

```bash
# Install the Socrates agent globally
intent agents install socrates

# Verify installation
intent agents list
```

## Tips for Best Results

1. **Provide Context**: The more context you provide, the better the dialog
2. **Be Specific**: Include real constraints (team size, timeline, existing tech)
3. **Iterate**: Don't hesitate to ask for deeper exploration of specific points
4. **Document**: Save important dialogs as part of your project documentation
5. **Share**: Use dialogs as starting points for team discussions

## Customization

You can create project-specific versions by:

1. Copying this agent to your project's `intent/agents/` directory
2. Modifying the personas to match your organization's roles
3. Adding domain-specific knowledge and constraints
4. Including references to your specific tech stack and standards

## Support

For questions or improvements to the Socrates agent, please open an issue in the Intent repository or contribute directly via pull request.