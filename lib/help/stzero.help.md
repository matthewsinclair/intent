@short: Steel Thread Zero retrofit for brownfield projects

# intent st zero

Retrofit ST0000 deliverables into existing (brownfield) projects that already have code and custom configuration.

## Synopsis

```
intent st zero install [options]
```

## Description

The `st zero` command audits which ST0000 deliverables are present, missing, or partial in a brownfield project, then offers to install the missing ones. It never overwrites existing files -- CLAUDE.md in particular is never touched if it already exists.

## Commands

### install

Audit deliverables and install missing ones.

```
intent st zero install
intent st zero install --audit-only
intent st zero install --dry-run
intent st zero install --deliverable D3
```

Options:

- `--audit-only` -- Show gap analysis only, make no changes
- `--dry-run` -- Show what would be created, but don't write files
- `--deliverable ID` -- Target a single deliverable (D2-D11)

### help

Display usage information.

```
intent st zero help
```

## Deliverables

| ID  | File                           | Notes                    |
| --- | ------------------------------ | ------------------------ |
| D2  | CLAUDE.md                      | Never overwritten        |
| D3  | intent/llm/MODULES.md          | Auto-generated from code |
| D4  | intent/llm/ARCHETYPES.md       | Elixir only              |
| D5a | credo_checks/\*.ex           | Elixir only              |
| D6  | intent/llm/DECISION_TREE.md    | Template                 |
| D8  | MEMORY.md                      | Via intent claude prime  |
| D9  | Module check hook              | .claude/settings.local   |
| D10 | .intent/learnings.md           | Empty structure          |
| D11 | intent/llm/DEPENDENCY_GRAPH.md | Elixir only              |

## 4-Phase Process

1. **Audit** -- Check each deliverable for PRESENT/MISSING/PARTIAL status
2. **Gap Analysis** -- Display results with project type detection
3. **Proposals** -- Determine what to create or modify
4. **Apply** -- Install missing deliverables (with per-item output)

## Examples

```bash
# Audit only -- see what's missing
intent st zero install --audit-only

# Dry run -- see what would change
intent st zero install --dry-run

# Install everything missing
intent st zero install

# Install only MODULES.md
intent st zero install --deliverable D3
```

## See Also

- `intent help` -- General help
- `intent modules check` -- Check MODULES.md registry
- `intent audit health` -- Health check
