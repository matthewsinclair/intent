---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-07
title: "Health Check & Learnings (D7, D10)"
scope: Medium
status: Done
---

# WP-07: Health Check & Learnings (D7, D10)

## Objective

Lightweight periodic health check (D7) for end-of-day use, plus a learnings accumulator (D10) for capturing project-specific knowledge over time.

## Deliverables

### D7: `intent audit health`

```bash
intent audit health             # ~2 min health check
intent audit health --report    # Save markdown report
intent audit health --diff      # Only check files changed since last audit
```

**Checks:**

1. Run automated Credo checks (from D5a/D5b)
2. New modules not in `MODULES.md`
3. Controllers/LiveViews past line threshold (default: 100 lines)
4. Identical function names across modules (Highlander suspects)

**Diff mode**: `git diff --name-only` since last timestamp in `.intent/last-health-check`.

**Report saved to**: `intent/audit/YYYYMMDD-health.md`

### D10: Learnings Accumulator

```bash
intent learn "description"                    # Add footgun (default)
intent learn --category worked "description"  # Pattern that worked
intent learn --category failed "description"  # Pattern that failed
intent learn --list                           # Show all learnings
```

**Storage**: `.intent/learnings.md`

```markdown
# Project Learnings

## Footguns

- 2026-03-04: description

## Patterns That Worked

- 2026-03-04: description

## Patterns That Failed

- 2026-03-04: description
```

**Integration**: Consumed by `intent claude prime` (WP-04) for MEMORY.md "Known Footguns" section.

## Acceptance Criteria

- [ ] `intent audit health` produces useful report under 2 minutes
- [ ] `--diff` only checks changed files
- [ ] `--report` saves timestamped markdown
- [ ] `intent learn` appends to `.intent/learnings.md`
- [ ] Learnings consumed by `intent claude prime`

## Dependencies

- D7 depends on: WP-06 (health check runs Credo checks)
- D10 is independent
