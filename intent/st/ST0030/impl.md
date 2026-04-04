# Implementation - ST0030: Cherry-Pick Superpowers Patterns for Intent

## Implementation

All 6 WPs completed in a single session.

### New Skills Created (3)

| Skill       | Path                                              | Lines | Content                                           |
| ----------- | ------------------------------------------------- | ----- | ------------------------------------------------- |
| `in-verify` | `intent/plugins/claude/skills/in-verify/SKILL.md` | 50    | Verification gate with 5 rules + types table      |
| `in-debug`  | `intent/plugins/claude/skills/in-debug/SKILL.md`  | 68    | 4-phase debugging + 3-strike rule                 |
| `in-review` | `intent/plugins/claude/skills/in-review/SKILL.md` | 58    | Two-stage review with agent delegation for Elixir |

All 3 include Red Flags tables and `chains_to:` fields.

### Existing Skills Modified (6)

| Skill           | Changes                                                                                                                            |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `in-start`      | Added `chains_to: ["in-plan", "in-next"]` + Skill Chain section                                                                    |
| `in-plan`       | Added `chains_to: ["in-next"]` + Plan Quality Standards (new section 3) + Red Flags + Skill Chain. Renumbered sections 3-5 to 4-6. |
| `in-next`       | Added `chains_to: ["in-plan"]` + Skill Chain section                                                                               |
| `in-finish`     | Added `chains_to: ["in-verify"]` + Red Flags + Skill Chain section                                                                 |
| `in-essentials` | Added Red Flags table (4 entries)                                                                                                  |
| `in-standards`  | Added Red Flags table (3 entries)                                                                                                  |

### MODULES.md Updated

Added "Skills: Superpowers Cherry-Picks" section with 3 entries.

### Verification Results

- 462/462 BATS tests pass
- 22 skills listed (19 existing + 3 new), all INSTALLED
- `intent claude skills sync` updated 6 modified skills successfully
- No em dashes in any files
- All markdown tables column-aligned

## Reference: Superpowers Patterns

Source repo: https://github.com/obra/superpowers (v5.0.7, Jesse Vincent)

### Verification-before-completion (Superpowers pattern)

Key phrase: "NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE"

Rules:

- Must show actual command output, not paraphrase
- Must run verification in current message, not reference prior run
- Must verify the specific change, not just "all tests pass"
- "I verified this works" without evidence is a red flag

### Rationalization table format (Superpowers pattern)

Every workflow skill includes a table of anticipated model shortcuts:

```markdown
| Rationalization           | Reality                                   |
| ------------------------- | ----------------------------------------- |
| "Too simple to test"      | Simple code breaks. Test takes 30 seconds |
| "I'll add tests later"    | Tests after code prove nothing            |
| "This is just a refactor" | Refactors break things. Test first.       |
```

### 3-strike debugging rule (Superpowers pattern)

If 3+ fixes fail for the same issue:

1. Stop attempting fixes
2. Present the failure pattern to the user
3. Question whether the architecture/approach itself is wrong
4. Ask the user to confirm direction before continuing

### Two-stage code review (Superpowers pattern)

Stage 1 -- Spec compliance: Does the code match the plan/design doc?
Stage 2 -- Code quality: Is the code clean, idiomatic, maintainable?

Superpowers dispatches anonymous subagents for each stage. Intent maps this to existing named agents (`diogenes` for spec compliance, `elixir` for code quality) in Elixir projects.

## Technical Details

### Frontmatter `chains_to:` field

YAML list in SKILL.md frontmatter. Claude reads it when skill is loaded. No bash parsing needed.

```yaml
---
description: "Session start: ..."
chains_to: ["in-plan", "in-next"]
---
```

Claude should present chain suggestions at skill completion: "Next suggested skills: `/in-plan` or `/in-next`"

### Context window impact

3 new skills add ~190 lines of SKILL.md content. At ~4 tokens/line, that's ~760 tokens when all three are active simultaneously. Minimal impact.

Rationalization tables in 4 existing skills add ~50 lines total (~200 tokens). Also minimal.

## Challenges & Solutions

(To be filled during implementation)
