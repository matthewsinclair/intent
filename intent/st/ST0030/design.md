# Design - ST0030: Cherry-Pick Superpowers Patterns for Intent

## Status

As-built matches design. All 6 WPs completed with no deviations. See `impl.md` for details.

## Approach

Cherry-pick 6 patterns from Superpowers into Intent-native skills and skill enhancements. Each work package is self-contained. High-priority WPs (01-03) can be done in any order. Medium-priority WPs (04-06) depend on WP-01 (frontmatter infrastructure).

## Design Decisions

### D1. New skill: `in-verify` (verification-before-completion)

Create a standalone skill, not a section in `in-finish`. Rationale: verification applies mid-session (after any task completion), not just at session end. `in-finish` can chain to it, but `in-verify` must be independently invocable.

**Skill content**: Mandate that before any claim of "done", "tests pass", or "working", the agent must:

- Show the actual command output (not paraphrase it)
- Run tests/build in the current message (not reference a prior run)
- Verify the specific change, not just "all tests pass"

### D2. Rationalization tables in existing skills

Add "Red Flags" and "Common Rationalizations" sections to 4 existing skills: `in-essentials`, `in-plan`, `in-finish`, `in-standards`. These are adversarial self-correction prompts that anticipate how the model will try to skip requirements.

Format (from Superpowers, adapted):

```markdown
### Red Flags

| Rationalization                     | Reality                                          |
| ----------------------------------- | ------------------------------------------------ |
| "Too simple to need a steel thread" | Every piece of work gets tracked. No exceptions. |
| "I'll update docs later"            | Later never comes. Update now.                   |
```

Do NOT add rationalization tables to domain-specific skills (Elixir, Ash, Phoenix) -- those are reference material, not workflow enforcement.

### D3. Skill dependency chains via frontmatter

Add optional `chains_to:` field to SKILL.md frontmatter. This is a hint displayed when the skill completes, suggesting next skills. It does NOT auto-activate anything.

```yaml
---
description: "Session start: ..."
chains_to: ["in-plan", "in-next"]
---
```

Implementation: Pure documentation in the skill files. No infrastructure changes to `intent_claude_skills` or `claude_plugin_helpers.sh` in this ST. The `chains_to:` field is read by Claude when the skill is loaded -- Claude sees the frontmatter and suggests the next skill. No bash parsing needed.

Rationale: Keep it simple. If the field proves valuable, a future ST can add infrastructure support (e.g., `intent claude skills show in-start` displaying chain targets).

### D4. Plan granularity standards in `in-plan`

Add a "Plan Quality Standards" section to `in-plan` that enforces:

- No "TBD", "TODO", or "handle edge cases" placeholders
- Each step specifies which files will be modified
- Steps are small enough to verify independently
- Verification command for each step (test, build, or manual check)

### D5. New skill: `in-debug` (systematic debugging)

Create a procedural skill for debugging that follows Superpowers' 4-phase approach:

1. Reproduce and gather evidence (read errors, check recent changes)
2. Pattern analysis (compare against working code, check for similar past bugs)
3. Hypothesis testing (test one variable at a time, never change two things)
4. Implementation (failing test, fix, verify)

Key addition: the **3-strike rule** -- if 3 fixes fail, stop and question the architecture. Present the failure pattern to the user and ask whether the approach itself is wrong.

### D6. New skill: `in-review` (two-stage code review)

Create a procedural skill that chains two review passes:

1. **Spec compliance**: Does the implementation match the plan/design doc? (Maps to `diogenes` agent for Elixir test specs)
2. **Code quality**: Is the implementation clean, idiomatic, and maintainable? (Maps to `elixir` agent for Elixir code)

For non-Elixir projects, both passes are done inline (no specialized agent). The skill provides the checklist; the agents provide domain expertise.

## Architecture

### New Files

| Path                                              | Type      | Size Est  |
| ------------------------------------------------- | --------- | --------- |
| `intent/plugins/claude/skills/in-verify/SKILL.md` | New skill | ~50 lines |
| `intent/plugins/claude/skills/in-debug/SKILL.md`  | New skill | ~80 lines |
| `intent/plugins/claude/skills/in-review/SKILL.md` | New skill | ~60 lines |

### Modified Files

| Path                                                  | Change                                                       |
| ----------------------------------------------------- | ------------------------------------------------------------ |
| `intent/plugins/claude/skills/in-essentials/SKILL.md` | Add rationalization table (~15 lines)                        |
| `intent/plugins/claude/skills/in-plan/SKILL.md`       | Add plan quality + rationalization (~25 lines), `chains_to:` |
| `intent/plugins/claude/skills/in-finish/SKILL.md`     | Add rationalization table (~10 lines), `chains_to:`          |
| `intent/plugins/claude/skills/in-standards/SKILL.md`  | Add rationalization table (~10 lines)                        |
| `intent/plugins/claude/skills/in-start/SKILL.md`      | Add `chains_to:` field                                       |
| `intent/plugins/claude/skills/in-next/SKILL.md`       | Add `chains_to:` field                                       |

### No Infrastructure Changes

The `intent_claude_skills` script and `claude_plugin_helpers.sh` do NOT need modification. The `chains_to:` field is purely semantic -- Claude reads it from the SKILL.md frontmatter. No manifest changes, no new CLI commands.

## Alternatives Considered

### Install Superpowers alongside (rejected)

Auto-activation conflicts with Intent's explicit invocation model. TDD absolutism conflicts with Ash declarative patterns. Two parallel documentation systems (steel threads vs `docs/superpowers/`). Too much context window overhead (33 skills).

### Migrate Intent to native Claude Code plugin format (rejected)

Intent skills are tightly coupled to the `intent` CLI. Skills reference commands that wouldn't exist for non-Intent users. Significant effort for unclear benefit. Not pursuing unless marketplace distribution becomes a goal.

### Add auto-activation to Intent skills (rejected)

Superpowers' "even 1% chance" auto-activation is aggressive and conflicts with Intent's philosophy. Intent trusts the developer to invoke skills explicitly. Skill chains provide gentle guidance without forcing activation.
