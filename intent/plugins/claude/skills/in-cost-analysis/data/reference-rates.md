# Reference Rates for Cost Analysis

## Productivity Rates by Complexity Tier

Lines of code per hour for a senior developer (5+ years experience):

| Tier        | Lines/Hour | Examples                                                |
| ----------- | ---------- | ------------------------------------------------------- |
| Simple      | 30-50      | CRUD, config, boilerplate, simple templates, migrations |
| Moderate    | 20-30      | Business logic, API handlers, standard UI, test suites  |
| Complex     | 10-20      | Concurrency, state machines, complex queries, protocols |
| Specialized | 5-15       | GPU/shaders, crypto, real-time, kernel/driver, ML/AI    |

Use the midpoint of each range for initial estimates. Adjust based on complexity assessment.

## Overhead Multipliers

These percentages are applied on top of base coding hours:

| Category              | Percentage | Notes                                        |
| --------------------- | ---------- | -------------------------------------------- |
| Architecture & design | 15-20%     | Up-front design, ADRs, API design            |
| Debugging             | 25-30%     | Troubleshooting, root cause analysis         |
| Code review           | 10-15%     | Reviewing, responding to feedback, rework    |
| Documentation         | 10-15%     | Internal docs, API docs, READMEs             |
| Testing               | 20-25%     | Unit, integration, E2E, test infrastructure  |
| Learning curve        | 10-20%     | New frameworks, domain knowledge, onboarding |
| DevOps & CI/CD        | 5-10%      | Build pipelines, deployment, infrastructure  |
| Project management    | 5-10%      | Planning, estimation, coordination           |

**Typical total overhead**: 100-145% of base coding hours (i.e., total = 2.0-2.45x base)

For a quick estimate, use 2.2x as the default multiplier.

## Organizational Efficiency Factors

Real developers do not code 40 hours per week. These factors account for meetings, communication, context switching, and administrative overhead.

| Org Type   | Efficiency | Coding Hrs/Week | Typical Overhead Sources                     |
| ---------- | ---------- | --------------- | -------------------------------------------- |
| Solo       | 65-75%     | 26-30 hrs       | Self-management, context switching           |
| Startup    | 55-65%     | 22-26 hrs       | Standups, planning, some meetings            |
| Scaleup    | 45-55%     | 18-22 hrs       | More meetings, cross-team coordination       |
| Enterprise | 35-45%     | 14-18 hrs       | Heavy process, reviews, compliance, meetings |

**Calendar weeks** = Total dev hours / (40 x efficiency factor)

## Team Composition by Company Stage

Engineering does not ship alone. These ratios express supporting role hours as a fraction of engineering hours:

| Role                   | Solo | Startup | Scaleup | Enterprise |
| ---------------------- | ---- | ------- | ------- | ---------- |
| Product management     | 0%   | 15%     | 30%     | 40%        |
| UX/UI design           | 0%   | 15%     | 25%     | 35%        |
| Engineering management | 0%   | 5%      | 15%     | 20%        |
| QA/Testing             | 0%   | 5%      | 20%     | 25%        |
| Project management     | 0%   | 0%      | 10%     | 15%        |
| Technical writing      | 0%   | 0%      | 5%      | 10%        |
| DevOps/Platform        | 0%   | 5%      | 15%     | 20%        |

**Full team multipliers** (sum of all roles + 1.0 for engineering):

| Org Type   | Team Multiplier |
| ---------- | --------------- |
| Solo       | 1.0x            |
| Startup    | 1.45x           |
| Scaleup    | 2.2x            |
| Enterprise | 2.65x           |

## Market Rate Ranges (US, 2025-2026)

Hourly rates for software engineers:

| Level  | Low     | Mid     | High    |
| ------ | ------- | ------- | ------- |
| Junior | $40/hr  | $60/hr  | $85/hr  |
| Mid    | $75/hr  | $100/hr | $135/hr |
| Senior | $120/hr | $150/hr | $200/hr |
| Lead   | $150/hr | $185/hr | $250/hr |

For cost estimates, use the **Senior** row as the baseline. Adjust up for specialized domains (GPU, crypto, real-time) or high-cost markets (SF, NYC).

**Recommended defaults**: Low $120/hr, Mid $150/hr, High $200/hr
