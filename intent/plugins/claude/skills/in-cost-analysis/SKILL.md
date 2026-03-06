---
description: "Cost analysis: estimates development cost of reproducing a codebase from scratch"
---

# Cost Analysis

Estimates the development cost of reproducing a codebase from scratch. Language-agnostic, uses automated metrics collection and industry reference data to produce a structured cost estimate.

## Procedure

### 1. Gather parameters

Ask the user:

- Target directory (default: current project root)
- Organization type: solo, startup, scaleup, or enterprise (default: startup)
- Directories to exclude (default: vendor, node_modules, \_build, deps, .git)
- Whether to include git history analysis (default: yes if git repo)

### 2. Run the metrics script

The script lives alongside this SKILL.md:

```bash
bash "$(find ~/.claude/skills/in-cost-analysis -name cost-metrics.sh 2>/dev/null | head -1)" \
  --dir /path/to/project \
  -o /tmp/cost-metrics.json
```

Adjust flags based on user input:

- `--dir PATH` for target directory
- `--exclude "dir1,dir2"` for exclusion list
- `--no-git` to skip git history analysis
- `-o FILE` for output file (default: stdout)

### 3. Read metrics and reference data

Read the JSON output from the metrics script. Also read the reference rates file:

```bash
cat "$(find ~/.claude/skills/in-cost-analysis -name reference-rates.md 2>/dev/null | head -1)"
```

This provides productivity rates, overhead multipliers, and organizational factors.

### 4. Analyze complexity

Beyond raw LOC, assess the codebase for factors that affect cost:

- **Architecture complexity**: monolith vs microservices, event-driven, plugin systems
- **Framework depth**: how heavily the code relies on complex frameworks (Ash, Rails, React, Metal, etc.)
- **Integration surface**: external APIs, databases, message queues, third-party services
- **Specialized domains**: GPU programming, real-time systems, cryptography, ML/AI pipelines
- **Test sophistication**: property-based tests, integration suites, E2E automation

Use these factors to adjust the tier distribution from the metrics script. For example, if the script classifies 60% of code as "moderate" but the architecture is highly event-driven, shift some of that to "complex".

### 5. Calculate hours

Using the reference rates from Step 3:

1. **Base coding hours**: For each language, multiply code lines by the productivity rate for its tier
2. **Apply overhead multipliers**: Sum the overhead percentages (design, debugging, review, docs, testing, learning curve, devops, PM) and apply to base hours
3. **Total development hours** = base hours x (1 + total overhead fraction)
4. **Calendar time**: Divide total hours by (40 x efficiency factor for org type)
5. **Full team cost**: Multiply engineering cost by the team multiplier for org type

### 6. Generate cost estimate report

Produce a report with these sections:

```markdown
# Cost Estimate: [Project Name]

**Analysis Date**: [date]
**Target Directory**: [path]

## Codebase Metrics

- **Total code lines**: [N] across [N] languages
- [Per-language breakdown table: language, files, code lines, % of total]

## Complexity Assessment

- Architecture: [description]
- Key frameworks: [list]
- Integration points: [count and description]
- Specialized domains: [list]

## Development Time Estimate

| Tier        | Code Lines | Rate (lines/hr) | Base Hours |
| ----------- | ---------: | --------------: | ---------: |
| Simple      |        [N] |             [N] |        [N] |
| Moderate    |        [N] |             [N] |        [N] |
| Complex     |        [N] |             [N] |        [N] |
| Specialized |        [N] |             [N] |        [N] |
| **Total**   |    **[N]** |                 |    **[N]** |

**Overhead multiplier**: [X]x (design, debug, review, docs, testing, learning, devops, PM)
**Total development hours**: [N] hours

## Calendar Time

| Org Type   | Efficiency | Coding Hrs/Week | Calendar Weeks | Calendar Time |
| ---------- | ---------: | --------------: | -------------: | ------------- |
| Solo       |        70% |          28 hrs |      [N] weeks | ~[N] months   |
| Startup    |        60% |          24 hrs |      [N] weeks | ~[N] months   |
| Scaleup    |        50% |          20 hrs |      [N] weeks | ~[N] years    |
| Enterprise |        40% |          16 hrs |      [N] weeks | ~[N] years    |

## Cost Estimate

| Scenario |    Rate | Dev Hours | Engineering Cost | Team Multiplier | Full Team Cost |
| -------- | ------: | --------: | ---------------: | --------------- | -------------: |
| Low      | $[N]/hr |       [N] |             $[N] | [N]x            |           $[N] |
| Mid      | $[N]/hr |       [N] |             $[N] | [N]x            |           $[N] |
| High     | $[N]/hr |       [N] |             $[N] | [N]x            |           $[N] |

## Git History (if available)

- First commit: [date]
- Last commit: [date]
- Total commits: [N] by [N] authors over [N] months

## Agentic Leverage (if available)

Session estimates from commit clustering (4-hour gap threshold):

| Metric               |    Value |
| -------------------- | -------: |
| Agentic sessions     |      [N] |
| Agentic hours        |      [N] |
| Human-equivalent hrs |      [N] |
| **Leverage ratio**   | **[N]x** |

| Cost Scenario  | Human Cost | Agentic Hrs | Effective $/hr |
| -------------- | ---------: | ----------: | -------------: |
| Low ($[N]/hr)  |       $[N] |         [N] |           $[N] |
| Mid ($[N]/hr)  |       $[N] |         [N] |           $[N] |
| High ($[N]/hr) |       $[N] |         [N] |           $[N] |

Note: Session hours estimated from commit density per session (1-2 commits = 1hr, 3-5 = 2hr, 6-10 = 3hr, 11+ = 4hr). This is a consistent heuristic -- it may over- or under-estimate absolute hours, but produces comparable ratios across projects and over time.

## Assumptions

1. Rates based on US market (2025-2026)
2. Full-time equivalent allocation
3. Does not include marketing, legal, hosting, or ongoing maintenance
```

## Important Notes

- The metrics script requires bash 3.x+ (no other dependencies)
- Git analysis is optional and gracefully degrades without git
- Tier classification is heuristic -- always review and adjust in Step 4
- The report is a starting point for discussion, not a binding quote
