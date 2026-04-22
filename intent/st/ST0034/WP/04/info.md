---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-04
title: "Agnostic rule pack"
scope: Small
status: Not Started
---

# WP-04: Agnostic rule pack

## Objective

Author the four foundational language-agnostic rules as first-class `RULE.md` files: Highlander, PFIC (Pure Function / Impure Coordination), Thin Coordinator, and No Silent Errors. Each is a cross-cutting principle referenced by every language-specific critic subagent and by `in-standards`.

## Context

Intent's `CLAUDE.md` and existing subagent prose encode these four principles informally across many places. This WP extracts them into atomic, cite-able rule files so:

- Skills can reference them by stable ID (`IN-AG-HIGHLANDER-001`, etc.)
- Critic subagents apply them as a first pass before language-specific rules
- Each gets a clear Problem / Detection / When Applies / When Does Not Apply section
- `in-standards` stops being hollow by loading these four rules

The agnostic invariant: every rule must concretise in at least two language packs. This stops the agnostic directory becoming a dumping ground for vague wisdom. If a rule can't be demonstrated in at least two concrete languages, it doesn't belong at this tier.

## Deliverables

### Four rule files

- `intent/plugins/claude/rules/agnostic/highlander/RULE.md`
- `intent/plugins/claude/rules/agnostic/pfic/RULE.md`
- `intent/plugins/claude/rules/agnostic/thin-coordinator/RULE.md`
- `intent/plugins/claude/rules/agnostic/no-silent-errors/RULE.md`

### Proposed rule IDs

- `IN-AG-HIGHLANDER-001` — There can be only one; never duplicate code paths for the same concern.
- `IN-AG-PFIC-001` — Pure Function / Impure Coordination; isolate side effects at boundaries, keep the core deterministic.
- `IN-AG-THIN-COORD-001` — Coordinators (controllers, LiveViews, CLI dispatchers) stay thin; business logic lives in services or domains.
- `IN-AG-NO-SILENT-001` — Every error path is handled explicitly; never swallow errors silently.

### Cross-references

Each rule lists `concretised_by:` IDs pointing at language-specific rules in WP05/WP06. Examples:

- `IN-AG-HIGHLANDER-001` → `[IN-EX-CODE-004, IN-RS-CODE-002]` (or similar — the exact IDs firm up as WP05/06 land; placeholders OK in WP04)
- `IN-AG-PFIC-001` → `[IN-EX-CODE-001, IN-RS-CODE-001]`
- `IN-AG-THIN-COORD-001` → `[IN-EX-PHX-001, IN-SW-CODE-003]` (or similar)
- `IN-AG-NO-SILENT-001` → `[IN-EX-CODE-004, IN-RS-CODE-001, IN-LU-CODE-004]`

### Documentation pointers

Each rule's "Further reading" section points at:

- Relevant Intent project docs (CLAUDE.md, MODULES.md)
- The language-specific rules that concretise it
- External canonical sources where applicable (e.g. "Coalton's pure-core pattern", "Railway-oriented programming", "Joe Armstrong's 'let it crash' for the bounded case")

## Approach

1. **Mine source material.** Every agnostic rule has a home in Intent's existing discipline:
   - Highlander: `CLAUDE.md` Project Rules section, `intent/llm/MODULES.md` discipline
   - PFIC: Elixir subagent's functional-core framing; Worker-bee's 6-layer architecture (pure core surrounded by side-effecting boundaries)
   - Thin Coordinator: `CLAUDE.md` "Thin controllers/LiveViews"; Phoenix idiomatic conventions
   - No Silent Errors: `intent_helpers.sh:error()` discipline; Intent project rule #3

2. **Draft each rule.** Follow the archetype from WP01. For each:
   - Frontmatter: id, slug, title, `language: agnostic`, `category: <principle>`, severity, references, concretised_by
   - Problem: what goes wrong when the principle is violated, with a concrete scenario
   - Detection: cross-language heuristics (grep patterns, structural signals). These are guidance for Critic subagents, not prescriptive regex.
   - When Applies: scenarios where the principle applies
   - When Does Not Apply: edge cases (e.g. Highlander does not apply to localisation files; No Silent Errors does not apply inside top-level supervisors that explicitly want to let processes crash)
   - Further Reading: pointers

3. **Set `concretised_by:`** — even with placeholder IDs. Finalise IDs after WP05/06 complete; update references if any concretising rule IDs shift.

4. **Validate.** `intent claude rules validate` (WP02 deliverable) confirms each rule's frontmatter parses and cross-references resolve. If validate doesn't yet exist when WP04 runs, do manual validation against schema.

## Acceptance Criteria

- [ ] All 4 rule files exist at paths listed above
- [ ] Each rule's frontmatter has required fields per schema: `id`, `title`, `language: agnostic`, `category`, `severity`, `summary`, `principles`, `applies_when`
- [ ] Each rule's `concretised_by:` lists at least 2 language-specific rule IDs (placeholders acceptable if WP05/06 not yet complete; finalise before WP07)
- [ ] Each rule has all 9 structural elements per schema: H1 + tagline + `## Problem`, `## Detection`, `## Bad` (may be `N/A — see concretised_by`), `## Good` (same), `## When This Applies`, `## When This Does Not Apply`, `## Further Reading`
- [ ] Each rule body is under 200 lines (agnostic rules should be concise; detail lives in concretising rules)
- [ ] No two agnostic rules duplicate prose (intra-agnostic Highlander audit)
- [ ] Each rule cites a concrete Intent-project or industry source in Further Reading
- [ ] `intent claude rules validate rules/agnostic/` passes (validator from WP02)

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP04.

- [ ] `tests/unit/rule_pack_agnostic.bats` — presence of all 4 rules, frontmatter well-formed, `concretised_by:` ≥ 2 (invariant gate — prevents `agnostic/` from becoming a dumping ground)

### Tests to update

- [ ] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

## Dependencies

- **WP01** (schema): required. Rule files must conform to the schema.

Soft dependency: **WP05 and WP06** for finalising `concretised_by:` IDs. Initial WP04 pass uses placeholder IDs or tentative numbering; update when language packs complete.

## Implementation Notes

### Highlander rule draft structure

```yaml
---
id: IN-AG-HIGHLANDER-001
slug: highlander
title: There can be only one
language: agnostic
category: architecture
severity: critical
applies_to: ["**/*"]
references: []
concretised_by:
  - IN-EX-CODE-004
  - IN-RS-CODE-002
aliases: []
version: 1
---

## Problem
Duplicate code paths for the same concern create drift over time. Two implementations of "validate email" diverge; one gets a bug fix, the other does not. Eventually neither is authoritative and the system behaves inconsistently depending on which path executes.

## Detection
Look for modules, functions, or configurations that appear multiple times with the same responsibility. Signals:
- Two modules named `<app>.Validators.Email` and `<app>.Utils.EmailCheck`
- A function in module A calling its own implementation while module B calls a near-identical implementation in module C
- Same regex or validation logic repeated across files

Language-specific heuristics are in `concretised_by:` rules.

## When it applies
Any code concern: validation, formatting, error handling, configuration loading, I/O wrappers, data transformations.

## When it does not apply
- Localisation files (same key translated to different languages is not duplication)
- Test fixtures (repeated setup patterns are sometimes clearer than extracted helpers)
- Third-party integrations where each vendor's client module is genuinely its own concern

## Further reading
- Intent `CLAUDE.md` Project Rules section
- Intent `MODULES.md` — the Highlander enforcer registry
- [list concretising rules]
```

### PFIC rule draft structure

Similar shape. Problem: mixing pure logic with I/O makes testing hard, makes parallel/concurrent execution unsafe, makes reasoning local. Detection: functions that both compute and perform I/O; modules that claim to be domain logic but reach out to databases or networks. When Applies: any language with the concept of function purity. When Does Not Apply: intentional boundary adapters (HTTP clients, database repositories, OS shims).

### Thin Coordinator rule draft structure

Problem: business logic in controllers/LiveViews/CLI dispatchers couples transport concerns to domain concerns; makes the domain untestable without invoking HTTP/GUI/argv parsing. Detection: controller actions longer than ~20 lines; CLI commands that do validation + coordination + presentation inline; LiveView handle_event callbacks that compute changes. When Applies: any layered application. When Does Not Apply: trivial scripts, prototype sketches.

### No Silent Errors rule draft structure

Problem: swallowed errors (empty rescue, ignored `Result`, unchecked `{:error, _}` tuples) hide failures and degrade the system silently. Detection: `rescue _, do: nil`, `try/catch` without action, `_ = some_call()` where `some_call` returns fallible. When Applies: library code, coordinators, long-running processes. When Does Not Apply: best-effort telemetry, fire-and-forget logging, top-level supervisor "let it crash" at process boundaries.

### File layout

```
intent/plugins/claude/rules/agnostic/
  highlander/
    RULE.md
  pfic/
    RULE.md
  thin-coordinator/
    RULE.md
  no-silent-errors/
    RULE.md
```

No `good.*` / `bad.*` files at the agnostic tier (per schema). Concretising rules in language packs carry the runnable examples.

## Risks and Edge Cases

### Vague rule text

Agnostic rules risk being too abstract to act on. Mitigation: the `concretised_by:` invariant forces each rule to have language-specific counterparts that anchor it.

### Concretising rule IDs shift after WP05/06

If WP05 renumbers a rule, WP04's `concretised_by:` goes stale. Mitigation: the cross-reference validator (WP02) catches this. Also: agnostic rules can use `aliases:` on the language-side rules to preserve the reference.

### Highlander applied to itself

Agnostic rules themselves could duplicate prose. Mitigation: explicit cross-reference via `references:` (e.g. PFIC references Thin Coordinator for coordinator-layer purity discussion).

### "Agnostic" disputed for some principles

Some might argue Thin Coordinator is Elixir/Phoenix-specific. The concretised_by invariant is the test: if we can list a Rust equivalent (e.g. "Axum handlers stay thin, domain logic in services") and a Swift equivalent, it's agnostic. If not, it's language-specific.

## Testing Approach

### Schema validation

- `intent claude rules validate rules/agnostic/` passes (once WP02 validator exists)
- Manual: each frontmatter field matches schema spec

### Cross-reference validation

- After WP05/06 complete: every `concretised_by:` ID resolves to an existing rule file
- Circular reference check: agnostic rule A references B; B does not reference A transitively

### Content review

- Each rule's Problem section is specific, not vague
- Each rule's Detection section gives concrete signals, not handwaves
- Each rule's "When Does Not Apply" is substantive (prevents blanket application)

## Size and Estimate

- **Size**: S (Small, 1 session).
- Concise rules, finite source material, shared frontmatter shape.

## Exit Checklist

- [ ] All 4 rule files committed
- [ ] All acceptance criteria met
- [ ] Schema validation passes
- [ ] Cross-references at least provisionally populated (finalised after WP05/06)
- [ ] Each rule has a clear "When Does Not Apply" section (prevents critic noise)
- [ ] Registered in MODULES.md as part of the rules library
