---
verblock: "04 Mar 2026:v0.1: matts - Initial version"
wp_id: WP-09
title: "Retrofit Installation (D12)"
scope: Large
status: Done
---

# WP-09: Retrofit Installation (D12)

## Objective

Implement `intent st zero install` for brownfield projects. laksa-web and Lamplight both need this -- they have months of existing code and CLAUDE.md customization that must not be destroyed.

## Command Interface

```bash
intent st zero install                      # Full retrofit with gap analysis
intent st zero install --audit-only         # Gap analysis without applying
intent st zero install --deliverable D3     # Install specific deliverable
intent st zero install --dry-run            # Show what would change
```

## Retrofit Process

### Phase 1: Audit Existing State

1. Scan `lib/` for all modules and responsibilities
2. Detect existing patterns (controller styles, service conventions)
3. Parse existing CLAUDE.md rules
4. Compare existing RULES.md against ST0000 rule set

### Phase 2: Gap Analysis

```
Gap Analysis for: laksa-web

[PRESENT]  CLAUDE.md -- exists (167 lines, 12 sections)
[MISSING]  MODULES.md -- not found (will generate from scan)
[MISSING]  DECISION_TREE.md -- not found
[PARTIAL]  RULES.md -- exists but missing R11, R14, R15
[MISSING]  ARCHETYPES.md -- not found
[MISSING]  Memory priming -- no MEMORY.md found
```

### Phase 3: Generate Proposals

For each missing/partial deliverable:

- MODULES.md: Auto-populate from codebase scan with confidence markers
- CLAUDE.md: Generate diff for new sections (NEVER overwrite)
- RULES.md: Show new vs existing vs conflicting rules

### Phase 4: Apply with Confirmation

Each proposal applied individually with user confirmation:

```
Proposal 1: Create intent/llm/MODULES.md (47 modules detected)
  Apply? [y/N]
```

### Module Auto-Discovery Algorithm

1. Find all `.ex` files in `lib/`
2. Extract `defmodule` declarations
3. Group by top-level namespace
4. Infer concerns from path, name suffixes, behaviour implementations
5. Generate MODULES.md with confidence: `[HIGH]`, `[MED]`, `[LOW]`

## Acceptance Criteria

- [ ] `--audit-only` produces accurate gap analysis
- [ ] Auto-discovered MODULES.md is >80% correct grouping
- [ ] Existing CLAUDE.md NEVER overwritten
- [ ] Each change individually confirmable
- [ ] `--dry-run` shows changes without applying

## Dependencies

- Depends on: WP-03, WP-04, WP-06 (most deliverables must exist first)
- Late-stage WP
