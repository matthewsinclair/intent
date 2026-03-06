# Design - ST0027: Add /in-cost-analysis skill

## Skill Directory Structure

```
intent/plugins/claude/skills/in-cost-analysis/
  SKILL.md                    # Procedural guide (~100 lines)
  scripts/cost-metrics.sh     # Bash LOC counter + git analysis, outputs JSON
  data/reference-rates.md     # Rate tables, overhead multipliers, org factors
```

Follows `in-autopsy` pattern (SKILL.md + scripts/). Adds `data/` as new convention -- `cp -r` install handles it automatically.

## Design Decisions

1. **Drop ROI**: Original candidate had Claude ROI analysis (~100 lines). Dropped to keep the skill focused on cost estimation only.
2. **Bash metrics script**: Automated LOC counting, language detection, and git history analysis. No jq dependency -- manual JSON construction.
3. **Reference data file**: Rate tables, overhead multipliers, and org factors extracted to `data/reference-rates.md` so Claude reads them as context rather than hardcoding in SKILL.md.
4. **Language-agnostic**: Extension-to-language mapping via `case` statement covers 16+ languages. Tier classification uses file path heuristics (test/config = simple, service/worker = complex).
5. **bash 3.x compatibility**: No `declare -A`, no `${VAR^}`, no `readarray`. Uses `case` for maps and temp files for aggregation.

## Architecture

### SKILL.md (Procedural Guide)

6-step procedure:

1. Gather parameters (target dir, org type, exclusions)
2. Run `cost-metrics.sh` via find-based path resolution
3. Read metrics JSON + reference data
4. Analyze complexity (architecture, frameworks, integrations)
5. Calculate hours using reference rates + overhead multipliers
6. Generate cost estimate report (language-agnostic template)

### cost-metrics.sh (Metrics Collector)

- CLI flags: `--dir PATH`, `--exclude DIRS`, `--git`/`--no-git`, `-o FILE`
- LOC counting: total lines, blank lines, comment lines (prefix heuristic), code lines
- Comment detection: `#` for shell/python/ruby, `//` for C-family/Go/Rust/Swift, `--` for Elixir/Haskell/Lua
- Tier classification: heuristic based on file path patterns
- Git history (optional): first/last commit dates, total commits, author count, months active
- JSON output: manual construction, to stdout or `-o` file

### reference-rates.md (Reference Data)

- Productivity rates by complexity tier (simple/moderate/complex/specialized)
- Overhead multipliers (design, debugging, review, docs, testing, learning, devops, PM)
- Organizational efficiency factors (solo/startup/scaleup/enterprise)
- Team composition ratios by company stage
- Market rate ranges (junior/mid/senior/lead, low/mid/high)

## Alternatives Considered

1. **Python script**: More capable but adds a dependency. Bash keeps it zero-dependency like the rest of Intent.
2. **jq for JSON**: Would simplify output construction but intent_helpers already avoids jq dependency for JSON generation.
3. **Include ROI analysis**: The original candidate had Claude ROI. Dropped because it is a different concern and makes the skill too long.
