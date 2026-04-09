---
description: "TCA synthesize: cross-component deduplication, priority classification, and fix batch ordering"
chains_to: ["in-tca-remediate"]
---

# TCA Synthesize

Performs cross-component synthesis of all completed audit findings. Deduplicates violations by root cause, classifies into 5 priority tiers, and produces a prioritized remediation backlog.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Read all socrates.md files

Read every completed WP's `socrates.md`. If any component WPs are incomplete, stop and run `/in-tca-audit` first.

### 2. Aggregate statistics

Build a per-WP summary table:

```markdown
| WP    | Component | Total | High | Medium | Low |
| ----- | --------- | ----: | ---: | -----: | --: |
| WP-01 | {name}    |     X |    X |      X |   X |

...
| **Sum** | **All** | **T** | **H** | **M** | **L** |
```

### 3. Rule distribution

Count violations per rule across all WPs:

```markdown
| Rule | Name            | Count | Dominant WPs   |
| ---- | --------------- | ----: | -------------- |
| R6   | Highlander Rule |   ~XX | All / specific |

...
```

This reveals **systemic issues** (rules violated everywhere) vs **localized issues** (one bad module).

### 4. Cross-cutting deduplication

The most important synthesis step. Cluster violations by **root cause and fix**, NOT by rule number:

**Deduplication signals**:

- Same function name reported in multiple WPs
- Same root cause (e.g., "no shared utility module")
- Same fix recommended across WPs

**Split signals** (do NOT merge these):

- Same rule but different data types (e.g., Map.get on struct vs Map.get on plain map)
- Same pattern but different contexts requiring different fixes
- Same function name but genuinely independent implementations

**Benchmarks**:

- Expect 40-60% dedup rate on large projects
- Projects with strong existing architecture have lower dedup rates

### 5. Priority classification (5 tiers)

Classify each unique violation into one of five tiers:

| Tier | Name                    | Character                                   |
| ---- | ----------------------- | ------------------------------------------- |
| P0   | Bugs and crash risks    | Incorrect behavior or crashes in production |
| P1   | Highlander duplications | Cross-cutting code duplication              |
| P2a  | Mechanical quality gaps | Bulk-fixable, no design decisions needed    |
| P2b  | Minor refactoring       | Requires design decisions, single-module    |
| P3   | Style and convention    | Lowest impact mechanical fixes              |

**P0 examples**: bare `=` match on fallible calls, `String.to_atom` on user input, debug artifacts in production paths, non-exhaustive `with` clauses

**P1 ranking criteria**: copy count, inconsistency risk, fix scope

**P2a examples**: missing `@impl true`, missing SAFETY comments, missing `@doc`/`@spec`, unambiguous boolean operator fixes

**P2b examples**: thick coordinator extraction, multi-head function conversion, component extraction

**P3 examples**: naming conventions, pipe operator adoption, access control tightening

### 6. Fix batch ordering

Group into batches:

```
Batch A: P0 critical bugs (small, targeted, test after each)
Batch B: P1 highest-impact Highlander fix (extract shared module)
Batch C: P1 second-highest Highlander fix
Batch D: P1 domain-scoped dedup (fixes within one domain)
Batch E: P2a mechanical fixes (grep-and-fix in bulk)
Batch F: P2b thick coordinator refactoring (one per batch)
Batch G: P2b multi-head conversions (incremental)
Batch H: P3 style fixes (batch by rule, one commit per rule)
```

**Interleaving**: P2a can be interleaved with P1 since they touch different files. P2b should wait until P1 shared modules exist.

### 7. Polyglot two-pass synthesis

For polyglot projects:

1. **Within-ecosystem pass**: Synthesize findings per ecosystem (Elixir violations, Rust violations, etc.)
2. **Cross-ecosystem pass**: Look for X-rule violations and patterns that span ecosystems (e.g., API contract mismatches, shared Highlander patterns across language boundaries)

### 8. Present for review

Before writing the synthesis, present key findings to the user:

- P0 items: any false positives?
- P1 priority order: which Highlander fixes matter most?
- P2b scope: which refactorings are worth doing now vs deferring?
- Overall dedup rate: does it match expectations?

### 9. Write synthesis

Write the complete synthesis to the final (synthesis) WP's `socrates.md`:

- Aggregate statistics table
- Rule distribution table
- Deduplicated violation list with priority classification
- Fix batch ordering with dependency notes
- False positive notes (violations reviewed and rejected)

### 10. Commit

```bash
git add WP/{synthesis}/socrates.md
git commit -m "audit: synthesis -- {X} unique violations across {N} WPs"
```

Proceed to review (Phase 3), then `/in-tca-remediate` for execution.

## Important Notes

- Cluster by root cause and fix, not by rule number
- Split clusters when same rule has different data types or contexts
- The P2a/P2b distinction matters: mechanical fixes are parallelizable, refactoring is not
- Always present synthesis for user review before remediation
- For polyglot: run two-pass synthesis (within-ecosystem then cross-ecosystem)
