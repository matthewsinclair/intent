# Implementation - ST0027: Add /in-cost-analysis skill

## Implementation

### Files Created

| File                                                                    | Lines | Purpose                           |
| ----------------------------------------------------------------------- | ----- | --------------------------------- |
| `intent/plugins/claude/skills/in-cost-analysis/SKILL.md`                | 139   | Procedural guide for Claude       |
| `intent/plugins/claude/skills/in-cost-analysis/scripts/cost-metrics.sh` | 429   | Bash LOC counter + git analysis   |
| `intent/plugins/claude/skills/in-cost-analysis/data/reference-rates.md` | 84    | Rate tables, multipliers, factors |

### Key Patterns

- Script invocation via find (from in-autopsy): `bash "$(find ~/.claude/skills/in-cost-analysis -name cost-metrics.sh)"`
- bash 3.x compat: `case` for maps, no `declare -A`, no `${VAR^}`
- Manual JSON construction (no jq dependency)
- `cp -r` install mechanism handles data/ and scripts/ subdirectories automatically

## Technical Details

- Comment detection uses line-prefix heuristic per language family
- Tier classification maps file paths to complexity tiers (simple/moderate/complex/specialized)
- Git analysis is optional (--no-git flag) for repos without git history
- JSON output uses temp files for aggregation, cleaned up via trap EXIT

## Challenges & Solutions

1. **macOS awk lacks `asorti`**: Initial `aggregate_lang_json()` used gawk's `asorti` for sorting. Fixed by piping awk output through `sort -t'|' -k1 -rn` and reassembling JSON in a while-read loop.
2. **Script larger than estimated**: Plan called for ~200-250 lines, actual is 429. The extra comes from comprehensive language mapping (25+ extensions), robust comment detection per language family, and the aggregation functions using temp files instead of associative arrays.
3. **Linter table alignment**: Markdown linter reformatted tables in SKILL.md and reference-rates.md on save -- included as-is.
