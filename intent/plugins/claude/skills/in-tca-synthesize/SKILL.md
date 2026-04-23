---
description: "TCA synthesize: cross-component deduplication, priority classification, and fix batch ordering"
chains_to: ["in-tca-remediate"]
---

# TCA Synthesize

Performs cross-component synthesis of all completed audit findings. Deduplicates violations by root cause, classifies into 5 priority tiers, and produces a prioritized remediation backlog.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Read all socrates.md files

Read every completed WP's `socrates.md`. Each contains a verbatim critic report (CRITICAL / WARNING / RECOMMENDATION / STYLE sections + `Summary:` line + `Rules applied:` line) plus the cross-WP Highlander notes added by the audit wrapper. If any component WPs are incomplete, stop and run `/in-tca-audit` first.

### 2. Aggregate statistics

Build a per-WP summary table from each WP's critic `Summary:` line:

```markdown
| WP    | Component | Critical | Warning | Recommendation | Style | Total |
| ----- | --------- | -------: | ------: | -------------: | ----: | ----: |
| WP-01 | {name}    |        X |       X |              X |     X |     X |

...
| **Sum** | **All** | **C** | **W** | **R** | **S** | **T** |
```

The critic severity tiers (Critical / Warning / Recommendation / Style) map directly to the synthesis priority tiers (P0 / P1+P2a / P2b / P3) per §5 below.

### 3. Rule distribution

Count findings per IN-\* rule ID across all WPs:

```markdown
| Rule ID              | Slug                 | Count | Dominant WPs   |
| -------------------- | -------------------- | ----: | -------------- |
| IN-AG-HIGHLANDER-001 | highlander           |   ~XX | All / specific |
| IN-EX-CODE-002       | tagged-tuple-returns |    XX | WP-04, WP-07   |

...
```

The IN-\* IDs come straight from the critic reports — no translation needed. Sort by count descending. This reveals **systemic issues** (rules violated everywhere) vs **localized issues** (one bad module). Cross-reference each ID at `intent/plugins/claude/rules/<lang>/<cat>/<slug>/RULE.md` for the Detection heuristic and the canonical fix pattern.

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

Classify each unique violation into one of five tiers. The default mapping from critic severity to TCA priority:

| Critic severity | TCA tier  | Notes                                                         |
| --------------- | --------- | ------------------------------------------------------------- |
| CRITICAL        | P0        | Bugs and crash risks                                          |
| WARNING         | P1 or P2a | Highlander duplications -> P1; mechanical quality gaps -> P2a |
| RECOMMENDATION  | P2b       | Minor refactoring requiring design decisions                  |
| STYLE           | P3        | Style and convention                                          |

This mapping is the default. A WARNING that names `IN-AG-HIGHLANDER-001` (or any `IN-*-HIGHLANDER-*` concretisation) is always P1 — the synthesis dedup may collapse multiple WARNING findings into one P1 cluster. A WARNING on a mechanical rule (e.g. `IN-EX-CODE-003` `@impl true`) is P2a.

| Tier | Name                    | Character                                    |
| ---- | ----------------------- | -------------------------------------------- |
| P0   | Bugs and crash risks    | Incorrect behaviour or crashes in production |
| P1   | Highlander duplications | Cross-cutting code duplication               |
| P2a  | Mechanical quality gaps | Bulk-fixable, no design decisions needed     |
| P2b  | Minor refactoring       | Requires design decisions, single-module     |
| P3   | Style and convention    | Lowest impact mechanical fixes               |

**P0 examples**: bare `=` match on fallible calls, `String.to_atom` on user input, debug artifacts in production paths, non-exhaustive `with` clauses (typically `IN-EX-CODE-005` and similar CRITICAL rules)

**P1 ranking criteria**: copy count, inconsistency risk, fix scope. Driven by the critic's IN-AG-HIGHLANDER-001 findings and their language concretisations (e.g. `IN-EX-CODE-006`).

**P2a examples**: missing `@impl true` (`IN-EX-CODE-003`), missing tagged-tuple returns (`IN-EX-CODE-002`), unambiguous boolean operator fixes

**P2b examples**: thick coordinator extraction, multi-head function conversion, component extraction (typically RECOMMENDATION-tier critic findings)

**P3 examples**: naming conventions, pipe operator adoption, access control tightening (STYLE-tier)

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

For polyglot projects (multiple critic dispatches per WP):

1. **Within-ecosystem pass**: Synthesise findings per language using that language's IN-_ rule pack (e.g. `IN-EX-_`Elixir findings,`IN-RS-\*` Rust findings).
2. **Cross-ecosystem pass**: Look for `IN-AG-*` agnostic-rule violations that span ecosystems — Highlander duplications across language boundaries, identical anti-patterns concretised differently, shared coordinator-thinness gaps. The agnostic rule's `concretised_by:` list shows which language IDs to expect on each side.

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

- Cluster by root cause and fix, not by rule ID. Multiple distinct IN-_ findings can collapse to one fix; one IN-_ finding can split into multiple fixes by data-type context.
- Split clusters when the same rule has different data types or contexts (the FP guidance section in design.md should already enumerate these).
- The P2a/P2b distinction matters: mechanical fixes are parallelizable, refactoring is not.
- Always present synthesis for user review before remediation.
- For polyglot: run two-pass synthesis (within-ecosystem then cross-ecosystem).
- The critic's `Rules applied:` line tells you whether `.intent_critic.yml` filtered any rules. If a key IN-\* is unexpectedly absent from the synthesis tally, check the project config for an unintended `disabled:` entry.
