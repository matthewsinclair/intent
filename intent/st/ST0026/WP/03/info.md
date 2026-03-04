---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-03
title: "LLM Templates (D2, D3, D6)"
scope: Medium
status: Done
---

# WP-03: LLM Templates (D2, D3, D6)

## Objective

Create the three foundational LLM guidance templates that form the core of ST0000's prevention strategy. These documents get baked into every project from commit one.

## Deliverables

### D2: Enhanced CLAUDE.md Template

Replace `lib/templates/llm/_CLAUDE.md` (currently v2.2.0, basic) with a comprehensive version:

- Full rule set (not just a reference to RULES.md)
- Module registry reference (points to MODULES.md)
- Decision tree reference (points to DECISION_TREE.md)
- Session start checklist (files to read on every context reset)
- Project-specific patterns section (placeholder)
- After Context Reset section
- Variable substitution: `[[PROJECT_NAME]]`, `[[AUTHOR]]`, `[[DATE]]`, `[[INTENT_VERSION]]`

Also update `create_claude_md()` in `bin/intent_helpers` and the heredoc in `bin/intent_init` to use the enhanced template.

### D3: MODULES.md Template

Create `lib/templates/llm/_MODULES.md`:

- Header explaining purpose and the Highlander Rule
- Table format: `| Concern | THE Module | Notes |`
- Placeholder sections for common domains (Auth, Content, Core, etc.)
- Instructions: "ALWAYS check this file before creating a new module"
- Instructions: "Register new modules here FIRST, then create the file"

### D6: DECISION_TREE.md Template

Create `lib/templates/llm/_DECISION_TREE.md`:

- Elixir/Phoenix decision tree (primary)
- Generic decision tree (non-Elixir projects)
- Each node: question -> answer -> target location
- Anti-patterns: "If tempted to put X in Y, it belongs in Z"
- Cross-reference to MODULES.md for "does it already exist?" checks

## File Locations

| Template                              | Installed to                  |
| ------------------------------------- | ----------------------------- |
| `lib/templates/llm/_CLAUDE.md`        | `CLAUDE.md`                   |
| `lib/templates/llm/_MODULES.md`       | `intent/llm/MODULES.md`       |
| `lib/templates/llm/_DECISION_TREE.md` | `intent/llm/DECISION_TREE.md` |

## Acceptance Criteria

- [ ] All 3 templates created in `lib/templates/llm/`
- [ ] Templates use variable substitution where appropriate
- [ ] `intent init` creates all 3 files in a new project
- [ ] Templates are self-documenting
- [ ] CLAUDE.md template is significantly richer than current v2.2.0

## Dependencies

- None (templates are standalone)
- Blocks: WP-04 (memory injection reads these templates)
