# Cost Estimate: Intent

**Analysis Date**: 2026-03-06
**Target Directory**: /Users/matts/Devel/prj/Intent

## Codebase Metrics

- **Total code lines**: 10,665 across 2 languages

| Language | Files | Code Lines | % of Total |
| -------- | ----: | ---------: | ---------: |
| Shell    |    26 |      5,705 |      53.5% |
| Elixir   |    18 |      4,960 |      46.5% |

## Complexity Assessment

- **Architecture**: Plugin-based CLI with callback architecture (8 callbacks per plugin, shared library dispatch). Steel thread methodology as first-class concept.
- **Key frameworks**: BATS (testing), custom Bash plugin system, Credo (Elixir templates)
- **Integration points**: Git CLI, filesystem/JSON manifests, Claude Code skills/subagents install system
- **Specialized domains**: None
- **Test sophistication**: 462 BATS tests across 22 files with helper scaffolding

Note: The Elixir code is primarily Credo check templates and an autopsy script -- not a running Elixir application. The real engineering complexity is in the Bash plugin architecture and CLI dispatch.

## Development Time Estimate

**Adjusted tiers** (shifted ~1,000 lines from complex to moderate -- some plugin scripts are straightforward):

| Tier        | Code Lines | Rate (lines/hr) | Base Hours |
| ----------- | ---------: | --------------: | ---------: |
| Simple      |      5,073 |              40 |        127 |
| Moderate    |      1,491 |              25 |         60 |
| Complex     |      4,101 |              15 |        273 |
| Specialized |          0 |              -- |          0 |
| **Total**   | **10,665** |                 |    **460** |

**Overhead multiplier**: 2.0x (solo dev, lighter process overhead)
**Total development hours**: 920 hours

## Calendar Time

| Org Type   | Efficiency | Coding Hrs/Week | Calendar Weeks | Calendar Time |
| ---------- | ---------: | --------------: | -------------: | ------------- |
| Solo       |        70% |          28 hrs |       33 weeks | ~8 months     |
| Startup    |        60% |          24 hrs |       38 weeks | ~9 months     |
| Scaleup    |        50% |          20 hrs |       46 weeks | ~11 months    |
| Enterprise |        40% |          16 hrs |       58 weeks | ~1.1 years    |

## Cost Estimate

| Scenario |    Rate | Dev Hours | Engineering Cost | Team Multiplier | Full Team Cost |
| -------- | ------: | --------: | ---------------: | --------------- | -------------: |
| Low      | $120/hr |       920 |         $110,400 | 1.0x (solo)     |       $110,400 |
| Mid      | $150/hr |       920 |         $138,000 | 1.0x (solo)     |       $138,000 |
| High     | $200/hr |       920 |         $184,000 | 1.0x (solo)     |       $184,000 |

## Git History

- First commit: 2023-11-15
- Last commit: 2026-03-06
- Total commits: 250 by 3 authors over 28 months

## Agentic Leverage

Session estimates from commit clustering (4-hour gap threshold):

| Metric               |     Value |
| -------------------- | --------: |
| Agentic sessions     |        43 |
| Agentic hours        |        88 |
| Human-equivalent hrs |       920 |
| **Leverage ratio**   | **10.5x** |

| Cost Scenario  | Human Cost | Agentic Hrs | Effective $/hr |
| -------------- | ---------: | ----------: | -------------: |
| Low ($120/hr)  |   $110,400 |          88 |         $1,255 |
| Mid ($150/hr)  |   $138,000 |          88 |         $1,568 |
| High ($200/hr) |   $184,000 |          88 |         $2,091 |

Note: Session hours estimated from commit density per session (1-2 commits = 1hr, 3-5 = 2hr, 6-10 = 3hr, 11+ = 4hr). Intent's lower ratio reflects that it predates heavy AI-assisted development -- earlier commits were largely manual.

## Assumptions

1. Rates based on US market (2025-2026)
2. Full-time equivalent allocation
3. Does not include marketing, legal, hosting, or ongoing maintenance
4. Solo org type: team multiplier is 1.0x (no supporting roles)
5. Elixir code treated as templates/scripts, not a running application
6. Overhead multiplier at low end (2.0x) since solo developer skips formal process
