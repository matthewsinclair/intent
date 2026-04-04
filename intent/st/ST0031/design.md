# Design - ST0031: Agentic Coding Course

## Approach

### D1: Atom-based content architecture (ACI model)

The fundamental unit is an Agentic Coding Insight (ACI) -- a markdown file with structured frontmatter (id, name, category, tags, difficulty, source). Each ACI has: Thesis, Story, Evidence, Application, Anti-Pattern sections. ACIs are composed into course days via audience-specific pathways.

**Rationale**: Atoms are composable. Two audiences need different sequences through shared content. A chapter-based approach would require duplicating content or awkward branching. Atoms let us build different molecules from the same elements.

### D2: Six extraction lenses

Internal insights are extracted via 6 lenses applied to each repo:

1. Rule Archaeology (CLAUDE.md/MEMORY.md git evolution)
2. Plan-Outcome Delta (steel thread design vs implementation)
3. Correction Mining (session log human corrections)
4. Architecture Forensics (major refactors in git)
5. Methodology Evolution (cross-repo timeline)
6. Failure Archaeology (cancelled STs, failed projects, removed rules)

**Rationale**: Different lenses find different kinds of lessons. No single approach captures the full range. The lenses are independent -- each can be applied to any repo.

### D3: Parallel internal + external research

WP-02 (internal forensic extraction) and WP-03 (external landscape research) run in parallel. They converge in WP-04 (taxonomy + course structure).

**Rationale**: Internal and external research are independent. Running them in parallel saves sessions. The merge point in WP-04 ensures coherent integration.

### D4: Dual audience pathways, not dual courses

One ACI pool. Two pathways (individual/advanced and enterprise). Pathways differ in: sequencing, framing, depth, and supplementary materials (enterprise gets ROI/governance).

**Rationale**: Maintaining two separate courses doubles the work for marginal benefit. The insights are the same -- only the presentation order and framing change.

### D5: MeetZaya as centerpiece, not appendix

The failed project is a Day 5 case study ("The Reckoning"), not a sidebar. Failure teaches what success can't -- especially for enterprise audiences concerned about risk.

**Rationale**: Honest failure analysis builds credibility and provides uniquely valuable anti-pattern lessons.

### D6: Landscape + evaluation framework as bookends

Day 1 opens with the state of the art (landscape). Day 5 closes with the evaluation framework (how to keep learning). This grounds the course in the real world and gives attendees a durable meta-skill.

**Rationale**: Without landscape context, the course feels like a sales pitch. Without an evaluation framework, attendees can't continue learning independently.

## Architecture

### Course structure (5-day arc)

| Day | Theme            | Theory Focus                                                         | Practical Focus                        |
| --- | ---------------- | -------------------------------------------------------------------- | -------------------------------------- |
| 1   | The Landscape    | State of the art, tools, frameworks, mental models                   | Set up tooling, first agentic task     |
| 2   | The Conversation | Communication with agents, prompting, corrections                    | Live agent interaction, session review |
| 3   | The Architecture | Code/project structure for agents, CLAUDE.md                         | Refactor for agent-friendliness        |
| 4   | The Process      | Planning, verification, debugging, review, lifecycle                 | Execute a steel thread, run autopsy    |
| 5   | The Reckoning    | MeetZaya failure study, cross-project patterns, evaluation framework | Build personal evaluation rubric       |

### Content file structure

```
docs/course/
  templates/aci-template.md
  insights/ACI-NNN-slug.md ...
  landscape.md
  evaluation-framework.md
  taxonomy.md
  outline.md
  days/day-N-individual.md, day-N-enterprise.md
  exercises/
  meetzaya-case-study.md
```

### New files

| File                                    | Purpose                      | WP  |
| --------------------------------------- | ---------------------------- | --- |
| `docs/course/templates/aci-template.md` | ACI format definition        | 01  |
| `docs/course/insights/`                 | Individual ACI files         | 02+ |
| `docs/course/landscape.md`              | External landscape survey    | 03  |
| `docs/course/evaluation-framework.md`   | Tool/method evaluation guide | 03  |
| `docs/course/taxonomy.md`               | ACI category definitions     | 04  |
| `docs/course/outline.md`                | 5-day structure + pathways   | 04  |

### No infrastructure changes

This ST produces course content documents only. No new skills, no code changes, no test changes.

## Alternatives Considered

**Chapter-based course**: Traditional linear structure. Rejected because two audiences need different sequences.

**Intent-only focus**: Course about Intent methodology specifically. Rejected because the landscape section makes the course more credible and useful, and the client needs broader context.

**Appendix treatment for MeetZaya**: Bury the failure in a sidebar. Rejected because failure analysis is the most valuable and unique content.

**Automated extraction only**: Build tools to find lessons automatically. Rejected as primary approach -- the interpretation step (raw finding -> insight) requires human judgment. Tools assist but don't replace.
