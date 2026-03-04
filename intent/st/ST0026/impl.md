# Implementation - ST0026: Steel Thread Zero

## Implementation Status

Not yet started. This document will be updated with as-built details as each WP is completed.

## Execution Order

| Order | WP    | Title                    | Deliverables | Status      |
| ----- | ----- | ------------------------ | ------------ | ----------- |
| 1     | WP-01 | Skill Rename             | D13          | Not Started |
| 2     | WP-02 | Workflow Skills          | D14          | Not Started |
| 3     | WP-03 | LLM Templates            | D2, D3, D6   | Not Started |
| 4     | WP-05 | Archetype Templates      | D4           | Not Started |
| 5     | WP-04 | Memory Injection         | D8           | Not Started |
| 6     | WP-06 | Automated Enforcement    | D5a, D5b     | Not Started |
| 7     | WP-07 | Health Check & Learnings | D7, D10      | Not Started |
| 8     | WP-08 | Guardrails               | D9, D11      | Not Started |
| 9     | WP-09 | Retrofit Installation    | D12          | Not Started |
| 10    | WP-10 | Integrator Command       | D1           | Not Started |
| --    | WP-11 | TN004 Tech Note          | --           | Not Started |

## Key Technical Details

### Memory Path Computation (WP-04)

Claude Code computes the memory directory from the project's absolute path:

- Input: `/Users/matts/Devel/prj/laksa-web`
- Transform: replace `/` with `-`, prepend `-`
- Output: `~/.claude/projects/-Users-matts-Devel-prj-laksa-web/memory/MEMORY.md`

### Skill System (WP-01, WP-02)

Skills are directories under `intent/plugins/claude/skills/` containing at minimum a `SKILL.md` file. Installation copies the directory to `~/.claude/skills/`. The manifest at `~/.intent/skills/installed-skills.json` tracks installations with SHA256 checksums.

Two skill types:

- **Enforcement** (existing): `## Rules` sections, always-on
- **Procedural** (new workflow skills): `## Procedure` sections, invoked on demand

### Command Scripts

New commands follow the established pattern:

```bash
#!/bin/bash
set -e
INTENT_HOME="${INTENT_HOME:=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
source "$INTENT_HOME/bin/intent_helpers"
load_intent_config
require_project_root
```

Plugin commands additionally register in `plugin.json` and dispatch from `bin/intent`.

## As-Built Notes

[Will be populated as WPs are completed]

## Challenges & Solutions

[Will be populated during implementation]
