---
title: "Total Codebase Audit -- Forensic Process Tech Note"
version: "v3.0"
date: "2026-03-19"
author: "Intent Project"
---

# Prerequisites

- An Intent project with steel thread infrastructure
- Claude Code with Intent subagents installed (`intent claude subagents install --all`)
- A defined set of coding rules (see Phase 0.1)
- Project-level MODULES.md and DECISION_TREE.md (created by `intent init` or `intent claude prime`)

See also: ST0026 (Steel Thread Zero) for the prevention framework that stops these violations from recurring.

# Purpose

A reproducible, language-agnostic process for performing a **total forensic audit** of an entire codebase against a defined set of coding rules. Designed to be executed by Claude Code with Socrates-style sub-agents, producing a prioritized remediation backlog.

This process has been validated across 3 runs (~1,238 total files): a single-app Elixir project (~258 files), an umbrella Elixir project (~724 files), and a polyglot Elixir+Rust+Swift+Lua project (~256 files). It can be reproduced on:

- **Elixir/Phoenix/Ash** web and backend applications
- **Rust** systems and CLI applications
- **Swift/SwiftUI** iOS/macOS applications
- **Polyglot** projects, monorepos, and umbrella apps
- Any codebase with defined coding standards

> **Key insight**: The process is rule-driven, not language-driven. The rules change per ecosystem; the audit process does not.

For operational use, see the `/in-tca-*` skill suite (init, audit, synthesize, remediate, finish).

# Overview

The audit follows a 5-phase pipeline:

```
Phase 0: Provisioning   -> Steel thread + work packages + rules
Phase 1: Component Audit -> One Socrates sub-agent per WP (parallelizable)
Phase 2: Synthesis       -> Cross-component deduplication + prioritization
Phase 3: Review          -> Human review + priority agreement
Phase 4: Remediation     -> Batched fixes with compile/test gates
```

Total effort for a ~260-file Elixir project: ~14 sub-agent runs, ~4 hours wall clock, ~400 violations found. For a ~724-file umbrella: ~18 WPs, ~6 hours. For a ~256-file polyglot: ~14 WPs, ~5 hours.

# Phase 0: Provisioning

## 0.1 Define the Rule Set

The audit is only as good as its rules. Before anything else, define the complete, numbered rule set that every file will be checked against.

### Rule Set Structure

Each rule needs:

| Field        | Description                                      | Example              |
| ------------ | ------------------------------------------------ | -------------------- |
| **ID**       | Short identifier (R1, R2, ...)                   | R6                   |
| **Name**     | Human-readable name                              | The Highlander Rule  |
| **Source**   | Where the rule is defined                        | RULES.md #6          |
| **Check**    | What to look for (positive or negative)          | Duplicate code paths |
| **Applies**  | Which file types / components it applies to      | All .ex files        |
| **Severity** | Default severity when violated (High/Medium/Low) | High                 |

### Rule Sources by Ecosystem

| Ecosystem        | Typical Rule Sources                                                |
| ---------------- | ------------------------------------------------------------------- |
| Elixir/Phoenix   | RULES.md, Elixir Essentials, Credo config, usage-rules.md           |
| Rust             | clippy.toml, .rustfmt.toml, project CONTRIBUTING.md, API guidelines |
| Swift            | SwiftLint config, Apple HIG, project style guide                    |
| TypeScript/React | ESLint config, project CONTRIBUTING.md, component patterns          |
| General          | OWASP top 10, DRY/SOLID principles, project CLAUDE.md               |

### Example: Elixir Combined Rules (15 rules)

```
R1:  Typed Data Access (no [:key] on known structs)
R2:  Thin Controllers/LiveViews/CLI
R3:  No Helpers in Controllers
R4:  Component Extraction
R5:  Multi-Head Functions for Matchable Values
R6:  The Highlander Rule (no duplicate code paths)
R7:  Assertive Struct Access (known defstructs only)
R8:  Boolean Operators (and/or/not for booleans)
R9:  Exhaustive with Clauses
R10: Content Source Rules (serving_mode) [project-specific]
R11: @impl true on all behaviour callbacks
R12: Tagged tuples for fallible functions
R13: Pipe operator for sequential transforms
R14: Naming conventions (?, !, _ prefix)
R15: No debug artifacts (IO.inspect, dbg)
```

### Rule Precision

Rules must specify their boundary conditions to prevent over-reporting:

- **R5 (Multi-Head Functions)**: Only flag when the branching value is matchable (atoms, tuples, known structs). Do NOT flag conditional logic on computed booleans, string parsing, or numeric ranges where guards are the appropriate tool.
- **R7 (Assertive Struct Access)**: Only flag `Map.get/2` and `map[:key]` when the target is a known `defstruct`. Do NOT flag access on plain maps, JSON-decoded maps, or dynamic data where the key set is genuinely unknown.
- **General principle**: Every rule should state what it does NOT apply to, not just what it catches. Over-broad rules generate false positives that erode trust in the audit.

### Example: Rust Rules (validated)

| #   | Rule                          | Severity Context                                         |
| --- | ----------------------------- | -------------------------------------------------------- |
| R1  | No unwrap() on Results        | main/CLI: Medium; library/daemon: High; LazyLock: exempt |
| R2  | Derive standard traits        | Debug+Clone+PartialEq on public types                    |
| R3  | Error types implement Error   | std::error::Error + Display                              |
| R4  | Unsafe blocks need SAFETY     | Security boundary: High; perf-critical: Medium           |
| R5  | Pattern matching over if-let  | Same matchable-values boundary as Elixir R5              |
| R6  | The Highlander Rule           | Universal                                                |
| R7  | Builder pattern (>3 fields)   | Public API only                                          |
| R8  | No panic! in library code     | CLI/main OK; library: High                               |
| R9  | Exhaustive match arms         | No \_ catch-all on enums                                 |
| R10 | Explicit lifetime annotations | Public APIs only                                         |
| R11 | #[must_use] on fallible fns   | Library code only                                        |
| R12 | Const generics over runtime   | Where compiler supports it                               |

**Key nuance**: R1 severity depends on context. `unwrap()` in `main()` or CLI arg parsing is acceptable (Medium). In library code or long-running daemons, it's High. `LazyLock` initializers that panic on poisoned state are exempt.

### Example: Swift Rules (validated)

| #   | Rule                         | Severity Context                                            |
| --- | ---------------------------- | ----------------------------------------------------------- |
| R1  | No force unwraps (!)         | Tests: exempt; UI preview: Low; production: High            |
| R2  | Thin ViewControllers         | Business logic in services, not views                       |
| R3  | Protocol in extensions       | Conformance must be in extensions                           |
| R4  | View extraction (>100 lines) | Applies to SwiftUI body and UIKit viewDidLoad               |
| R5  | Enum-based routing           | Over string matching                                        |
| R6  | The Highlander Rule          | Universal                                                   |
| R7  | Codable on network types     | All API response/request types                              |
| R8  | Access control               | Default internal; explicit public. try? context-sensitive   |
| R9  | Error handling do-try-catch  | try? acceptable for fire-and-forget; not for critical paths |
| R10 | Async/await over callbacks   | Modern concurrency patterns                                 |

**Key nuance**: R6 (AppKit vs SwiftUI) -- projects mixing both frameworks need separate thresholds. AppKit view code has different complexity baselines than SwiftUI declarative views. R8 `try?` is acceptable for logging/analytics calls but not for data persistence.

### Supplemental: Ash Framework Rules (A1-A5)

For Elixir projects using the Ash Framework, add these supplemental rules alongside the base R1-R15:

| #   | Rule                       | What to Check                                                   |
| --- | -------------------------- | --------------------------------------------------------------- |
| A1  | Code Interface Access Only | No direct Ash.get!/read!/create! in LiveViews or controllers    |
| A2  | ash.codegen for migrations | Never mix ecto.gen.migration; Ash generates correct migrations  |
| A3  | Actor Authorization        | Actor set on query/changeset construction, not action execution |
| A4  | Code Interface Options     | Use query: [filter:, sort:] over manual Ash.Query pipelines     |
| A5  | Cross-Domain Access        | Domain A must not bypass Domain B's code interfaces             |

These are first-class audit rules, not afterthoughts. Ash-specific violations are often the highest-impact findings in Ash projects because they break the framework's authorization and consistency guarantees.

**Key**: R6 (Highlander) is universal. Every ecosystem benefits from deduplication auditing.

## 0.2 Map the Codebase into Components

Divide the codebase into **audit-sized chunks** (10-60 files each). Each chunk becomes a Work Package (WP).

### Effective File Count

Not all files are equal audit weight. Use the effective file count to size WPs:

| File Type         | Weight | Rationale                                  |
| ----------------- | -----: | ------------------------------------------ |
| Standard code     |   1.0x | Normal audit effort                        |
| Ash DSL resources |  0.25x | Declarative, limited violation surface     |
| Emission/struct   |   0.5x | Thin files, few rules apply                |
| Dead stubs        |   0.0x | Exclude from audit entirely                |
| Rust code         |   1.5x | Ownership + lifetime complexity            |
| Swift AppKit      |   1.3x | Legacy framework, higher violation density |
| Test files        |   0.0x | Excluded unless testing rules are in scope |

**Example**: A component with 87 raw files but 15 Ash resources (0.25x), 12 structs (0.5x), and 5 dead stubs (0.0x) has an effective file count of:

```
55 standard (55.0) + 15 Ash DSL (3.75) + 12 structs (6.0) + 5 dead (0.0) = 64.75 effective
```

**Sweet spot**: 12-20 effective files per WP. This consistently produces 30-50 sub-agent turns without context exhaustion.

### Raw File Count Guidelines (fallback)

| WP Size   | Files | Sub-agent Turns | `max_turns` | Context Risk          |
| --------- | ----- | --------------- | ----------- | --------------------- |
| Small     | 5-15  | 20-30           | 30          | Low                   |
| Medium    | 15-30 | 30-50           | 50          | Medium                |
| Large     | 30-60 | 50-70           | 70          | High                  |
| Too Large | 60+   | 70+             | N/A         | Very High -- split it |

### Component Mapping Strategies

**By domain** (recommended for most projects):

```
WP-01: User/Account domain
WP-02: Content/Data domain
WP-03: API layer
WP-04: Background jobs
WP-05: Web UI layer
```

**By architectural layer** (good for layered architectures):

```
WP-01: Data models / schemas
WP-02: Business logic / services
WP-03: Controllers / handlers
WP-04: Views / templates / components
WP-05: Workers / jobs
WP-06: Configuration / infrastructure
```

**For umbrella/monorepo projects**:

```
WP-01: app_core/lib/models/
WP-02: app_core/lib/services/
WP-03: app_web/lib/controllers/
WP-04: app_web/lib/live/
WP-05: app_worker/lib/
WP-06: shared/lib/
```

**For polyglot projects**:

```
WP-01: backend/src/ (Rust)
WP-02: frontend/src/ (TypeScript)
WP-03: ios/Sources/ (Swift)
WP-04: shared/protos/ (Protobuf)
WP-05: infra/ (Terraform/YAML)
```

### File Discovery

Use glob patterns to enumerate files per component:

```bash
# Elixir
find lib/my_app/content/ -name "*.ex" | wc -l

# Rust
find src/models/ -name "*.rs" | wc -l

# Swift
find Sources/Models/ -name "*.swift" | wc -l
```

### Special Focus per WP

For each WP, identify 3-4 rules that are most likely to surface violations. This focuses the sub-agent's attention without excluding other rules.

Example:

```
WP-03 (Controllers): R2 (thin controllers), R3 (no helpers), R6 (Highlander)
WP-04 (LiveViews):   R2 (thin LiveViews), R11 (@impl), R4 (component extraction)
```

## 0.5 Pre-Filter Mechanical Rules

Before launching sub-agents, run grep-based pre-filtering for mechanical rules that can be detected without semantic analysis:

```bash
# R15: Debug artifacts
grep -rn "IO\.inspect\|dbg()" lib/ --include="*.ex"

# R8: Boolean operator misuse (candidates -- requires manual review)
grep -rn " && \| || " lib/ --include="*.ex"

# R11: Missing @impl (files with behaviour callbacks but no @impl)
grep -rL "@impl" lib/ --include="*.ex" | xargs grep -l "def mount\|def handle_"
```

**Why pre-filter**: These results feed directly into synthesis (Phase 2), reducing sub-agent hallucination risk on mechanical rules. Sub-agents still check these rules, but the pre-filter provides ground truth for validation.

Record pre-filter results in `design.md` for cross-reference during synthesis.

## 0.3 Create the Steel Thread

Create the steel thread directory structure:

```
intent/st/ST{NNNN}/
├── info.md          # Metadata, objective, context
├── design.md        # Rule set, component map, parallelization plan
├── tasks.md         # Phase checklist with all WPs
├── impl.md          # Implementation notes (optional)
└── WP/
    ├── 01/
    │   ├── info.md      # Scope, file list, applicable rules
    │   └── socrates.md  # Audit output (initially empty)
    ├── 02/
    │   ├── info.md
    │   └── socrates.md
    ...
    └── 15/             # Synthesis WP (always last)
        ├── info.md
        └── socrates.md
```

### WP info.md Template

```markdown
---
wp_id: WP-{NN}
title: "{Component Name}"
scope: Small | Medium | Large
status: Not Started
---

# WP-{NN}: {Component Name}

## Scope

{1-2 sentence description of what this component covers}

## Files

- `path/to/file1.ex`
- `path/to/file2.ex`
  ...

## Applicable Rules

All {N} rules. Special focus: R{X} ({reason}), R{Y} ({reason}).

## Cross-WP Highlander Dependencies

- WP-{X}: {what might be duplicated and why}
- WP-{Y}: {what might be duplicated and why}
```

### Cross-WP Highlander Dependency Encoding

> **Lesson from umbrella audit**: Don't wait until synthesis (Phase 2) to think about cross-WP duplication. Encode suspected cross-WP Highlander dependencies at provisioning time in each WP's `info.md` and `socrates.md`.

For each WP, identify 2-4 other WPs that might contain overlapping logic. Record these as:

1. **In `info.md`**: A "Cross-WP Highlander Dependencies" section listing which WPs and what logic might overlap
2. **In `socrates.md`**: A "Cross-WP Highlander Check" section in the prompt, instructing the auditor to flag "cross-WP Highlander suspects" even if the duplicate isn't confirmed yet

This way, the per-WP auditor records suspects that the synthesis WP can cross-reference. Without this, cross-WP violations are invisible until Phase 2 and much harder to find retroactively.

Example from an umbrella audit WP-08:

```
### Cross-WP Highlander Check

- vs. WP-04: Character resolution — if runengine/character.ex duplicates
  logic from the character domain, flag it. One must delegate to the other.
- vs. WP-05: Message appending — if narrator.ex implements its own conversation
  append path instead of using the canonical Conversation module, flag it.
```

### Batch Ordering for Parallelization

WPs should be organized into named dependency batches, not just "parallel if no shared files":

```
Batch 1A: WP-01, WP-11, WP-15   — Foundational, no dependencies
Batch 1B: WP-02, WP-03, WP-12   — Depend on context from 1A
Batch 1C: WP-04, WP-06, WP-07   — Domain logic layer
Batch 1D: WP-05, WP-08, WP-09   — Runtime layer (heaviest cross-cutting)
Batch 1E: WP-13, WP-14           — Web apps (need component library from 1A)
Batch 2:  WP-18                   — Synthesis (depends on ALL above)
```

**Why this matters**: Even though WPs don't share files, later batches benefit from the context and cross-WP suspect flags produced by earlier batches. Running WP-08 (Runengine) before WP-01 (DSL Pillars) means the auditor can't cross-reference DSL findings.

### Architectural Boundary Checks

Some WPs warrant checks that go beyond the coding rules -- structural/architectural boundary enforcement:

- **Shared library dependency violations**: A shared UI library (like `llclient`) must not reference domain structs from the core app. This isn't a coding rule per se, but a structural violation that breaks the dependency graph.
- **Layer violations**: If a resource module contains formatting logic, or a web component contains database queries, that's an architectural boundary violation.

Encode these as first-class concerns in the relevant WP's `socrates.md`, separate from the numbered rules.

# Phase 1: Component Audit

## 1.1 The Sub-Agent Prompt

Each WP audit is driven by a single sub-agent invocation. The prompt must be precise and self-contained.

### Prompt Template

```
You are performing a forensic code audit of the **{Component Name}** subsystem
in the {Project} application.

### Audit Rules

- R1: {rule name and brief description}
- R2: {rule name and brief description}
...
- R{N}: {rule name and brief description}

### Files to Audit

1. `{path/to/file1}`
2. `{path/to/file2}`
...

### Special Focus

{R-numbers and why they matter for this component}

### Instructions

Read EVERY file listed above. For each file, check every function against
all {N} rules. Report violations in this exact format:

#### V{N}: {short title}

- **File**: `{path}:{line(s)}`
- **Rule**: R{N} — {rule name}
- **Severity**: High | Medium | Low
- **Description**: {what the violation is}
- **Remedy**: {how to fix it}
- **Confidence**: HIGH | MEDIUM | LOW

After all violations, add a summary table:

### Summary

| Severity | Count |
| -------- | ----- |
| High     | X     |
| Medium   | Y     |
| Low      | Z     |
| **Total**| **T** |

Write the complete audit to `{path/to/WP/NN/socrates.md}`.

Be thorough. Do NOT skip files. Do NOT invent violations — only report
what you actually see in the code. If a file has no violations, say so
explicitly.
```

### Rule Descriptions in Prompts

> **Lesson from umbrella audit**: Don't just list rule names -- include a "What to check" column or brief description for each rule. This prevents the sub-agent from misinterpreting a rule.

Example (table format):

```
| #   | Rule                       | What to check                                                  |
| --- | -------------------------- | -------------------------------------------------------------- |
| R1  | Typed Data Access          | No `map[:key]` on known structs. Use `struct.field`.           |
| R6  | Highlander Rule            | Never duplicate code paths. Each concern has one implementation|
| R7  | Assertive Struct Access    | No `Map.get(struct, :field)` on known struct fields.           |
```

This is more reliable than bare `R6: The Highlander Rule` -- sub-agents need the "what" not just the "name."

### Critical Prompt Elements

1. **Explicit file list**: Every file must be named. Sub-agents will not discover files on their own reliably.
2. **Exact output format**: The violation format must be specified precisely so synthesis can parse it.
3. **"Read EVERY file"**: Without this, sub-agents may skip files they consider "simple."
4. **"Do NOT invent violations"**: Without this, sub-agents may hallucinate violations.
5. **Write destination**: Tell the sub-agent exactly where to write the output.

## 1.2 Sub-Agent Selection

| Agent Type                       | Use For                                                                                                      |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `diogenes`                       | Primary choice. Socratic dialog produces structured output                                                   |
| `Explore`                        | Pre-audit reconnaissance if component boundaries unclear                                                     |
| `elixir`                         | Elixir-specific deep dives (post-audit remediation)                                                          |
| `general-purpose` (General Opus) | Large polyglot WPs where no specialized agent matches. The prompt is the quality driver, not the agent type. |

For non-Elixir projects, use `diogenes` with language-appropriate rules, or a `general-purpose` agent with the full prompt template.

## 1.3 Execution Protocol

### Before Each WP

1. **Run `/compact`** to reclaim context window space
2. **Verify file manifest** -- confirm all files listed in the WP's `info.md` actually exist (`ls` each file). Missing files indicate stale provisioning.
3. **Verify the previous WP is committed** (no uncommitted audit files)
4. **Check context usage** -- if above 70%, consider starting a fresh session

### During Each WP

The sub-agent runs autonomously. It will:

1. Read every listed file
2. Check every function against every rule
3. Write findings to `socrates.md`
4. Return a summary

### After Each WP

1. **Commit immediately**: `git add WP/{NN}/socrates.md && git commit`
2. **Log the summary** in your running tally
3. **Move to the next WP**

### Crash Prevention

Context exhaustion is the primary risk. Mitigations:

| Risk             | Mitigation                             |
| ---------------- | -------------------------------------- |
| Context overflow | `/compact` before each WP              |
| Lost work        | Commit after every WP (never batch)    |
| Sub-agent stall  | Set max_turns limit on large WPs       |
| Session crash    | Keep a running log outside the session |
| WP too large     | Split WPs with >60 files into sub-WPs  |

### Parallelization

WPs can be run in parallel if they don't share files. Use the Agent tool's parallel invocation:

```
# Independent WPs — run in parallel
Agent(WP-06: Accounts)    # runs simultaneously with...
Agent(WP-07: Email)       # ...and...
Agent(WP-08: Sites)       # ...this one
```

**Do NOT parallelize** WPs that share files or have overlapping scope -- cross-WP violations will be missed or double-counted.

# Phase 2: Synthesis (WP-15)

After all component audits are complete, synthesize findings into a single prioritized document.

## 2.1 Aggregate Statistics

Compile a per-WP summary table:

```markdown
| WP    | Component | Total | High | Medium | Low |
| ----- | --------- | ----- | ---- | ------ | --- |
| WP-01 | {name}    | X     | X    | X      | X   |

...
| **∑** | **All** | **T** | **H**| **M** | **L**|
```

## 2.2 Rule Distribution

Count violations per rule across all WPs:

```markdown
| Rule | Name            | Count | Dominant WPs |
| ---- | --------------- | ----- | ------------ |
| R6   | Highlander Rule | ~130  | All          |

...
```

This reveals **systemic issues** (rules violated everywhere) vs **localized issues** (one bad module).

## 2.3 Cross-Cutting Deduplication

The most important synthesis step. Identify violations reported in multiple WPs that represent the **same underlying issue**.

Example: `verify_ownership/2` was reported as:

- WP-11 V1 (LoadSite plug)
- WP-12 V10 (Controllers)
- WP-13 V1 (LiveViews)

These are all one issue: extract `verify_ownership/2` to a shared module.

### Deduplication Signals

- Same function name in different WPs
- Same pattern described differently ("bracket access on site struct" in WP-03 and WP-05)
- Same remedy recommended in multiple WPs

### Deduplication by Root Cause

Cluster violations by **root cause and fix**, not by rule number:

- Same function duplicated across WPs -> one Highlander fix
- Same pattern with different data types -> **separate clusters** (e.g., Map.get on a struct vs Map.get on a plain map are different fixes)
- Same remedy recommended -> one fix batch entry

**Benchmarks**: Expect 40-60% dedup rate on large projects. Conflab achieved 59% (118 raw -> 49 unique). Lamplight achieved 45% (389 raw -> ~215 unique). Projects with strong existing architecture have lower dedup rates (fewer systemic issues).

## 2.4 Priority Classification (5-Tier)

> **Important**: Use all five tiers consistently. The P2a/P2b split matters because mechanical fixes (add @impl, add SAFETY comments) are grep-fixable in bulk, while refactoring (extract thick coordinator) requires careful design.

### P0: Bugs & Crash Risks

Violations that cause incorrect behavior or crashes in production:

- Bare `=` match on fallible calls (crash on error)
- Wrong key type on struct access (silent no-op)
- Missing serving_mode dispatch (wrong data source)
- `String.to_atom` on user input (atom exhaustion)
- Non-exhaustive `with` clauses on fallible calls (R9)
- Missing error returns from fallible functions (R12)
- Debug artifacts (`IO.inspect`, `dbg`) in production paths (R15)

### P1: Highlander -- Cross-Cutting Duplications

Code duplicated across multiple files/modules. Ranked by:

- **Copy count**: More copies = higher priority
- **Inconsistency risk**: Copies with divergent behavior are worse
- **Fix scope**: How many files need updating

### P2a: Mechanical Quality Gaps

Bulk-fixable quality issues that don't require design decisions:

- Missing `@impl true` annotations (R11)
- Missing SAFETY comments on unsafe blocks (R4 Rust)
- Missing `@doc`/`@spec` on public functions
- Boolean operator misuse where fix is unambiguous (R8)
- Debug artifacts (R15)

### P2b: Minor Refactoring

Requires design decisions but scoped to single modules:

- Thick coordinator extraction (R2)
- Multi-head function conversion (R5)
- Component extraction (R4)
- Builder pattern introduction (R7 Rust)

### P3: Style & Convention

Lowest-impact mechanical fixes:

- Naming convention alignment (R14)
- Pipe operator adoption (R13)
- Access control tightening (R8 Swift)

## 2.5 Fix Batches

Group related fixes into batches. Allow interleaving when dependencies permit:

```
Batch A: P0 critical bugs (small, targeted, test after each)
Batch B: P1 highest-impact Highlander fix (extract shared module)
Batch C: P1 second-highest Highlander fix
Batch D: P1 domain-scoped dedup (all fixes within one domain)
Batch E: P2a mechanical fixes (grep-and-fix in bulk)
Batch F: P2b thick coordinator refactoring (one per batch)
Batch G: P2b multi-head conversions (incremental)
Batch H: P3 style fixes (batch by rule, one commit per rule)
```

**Interleaving**: P2a mechanical fixes can be interleaved with P1 Highlander work since they touch different files. P2b refactoring should wait until P1 shared modules exist (they may be extraction targets).

# Phase 3: Review

Present the synthesis to the project owner for review. Key discussion points:

1. **P0 items**: Any that are false positives? Any missing?
2. **P1 priority order**: Which Highlander fixes matter most for the next quarter?
3. **P2a/P2b scope**: Which mechanical fixes to batch first? Which thick coordinators are worth refactoring now vs later?
4. **P3 approach**: Mechanical fixes in one big PR vs incremental?

# Phase 4: Remediation

## 4.1 Execution Model

**Do all remediation in the main conversation, not in sub-agents.** Sub-agents cannot coordinate compile+test cycles and hit permission walls on file edits. The main conversation has full filesystem access and can run verification gates inline.

### Execution Order

```
1. P0 fixes (each as a separate commit, test after each)
2. P1 fixes (each batch as a branch, test after each batch)
3. P2a fixes (bulk mechanical, one commit per rule)
4. P2b fixes (each refactoring as a separate commit)
5. P3 fixes (batch by rule, one commit per rule)
```

### Exception: Parallel Extraction

For P1/P2b extraction work ONLY, when file scopes are completely disjoint:

- Sub-agents may be used for extraction
- Main session does verification pass after all agents complete
- Create target modules FIRST, then update callers (creation-before-migration)
- Ensure file scopes are truly disjoint before launching parallel agents

## 4.2 Test Optimization

Run targeted tests between individual edits; full suite at batch boundaries:

```bash
# Between edits (fast feedback)
mix test --failed
mix test test/specific_file_test.exs

# At batch boundaries (full verification)
mix compile --warnings-as-errors && mix test && mix credo --strict

# Rust equivalent
cargo test --failed  # (if using nextest)
cargo check && cargo test && cargo clippy -- -D warnings
```

**Handle false positives**: If a fix breaks tests, investigate the data type. `Map.get` on a plain map is correct -- only flag on known defstructs (R7). Revert and mark as false positive rather than forcing a broken fix.

## 4.3 Verification Gates

After each batch:

```bash
# Elixir
mix compile --warnings-as-errors
mix test
mix credo --strict

# Rust
cargo check
cargo test
cargo clippy -- -D warnings

# Swift
swift build
swift test
swiftlint lint --strict
```

## 4.4 Regression Prevention

After remediation, add enforcement:

| Mechanism       | Elixir                   | Rust              | Swift           |
| --------------- | ------------------------ | ----------------- | --------------- |
| Linter          | Credo custom checks      | Clippy lints      | SwiftLint rules |
| CI gate         | `mix credo --strict`     | `cargo clippy -D` | `swiftlint`     |
| Pre-commit hook | `mix compile --warnings` | `cargo check`     | `swift build`   |
| Code review     | Checklist from rule set  | Same              | Same            |

# Appendix A: Scaling Considerations

## Small Projects (<50 files)

- 2-4 WPs sufficient
- Can run all audits in a single session
- Synthesis may be unnecessary (just sort by severity)

## Medium Projects (50-300 files)

- 8-15 WPs (a proven sweet spot)
- One session per 3-4 WPs with `/compact` between each
- Synthesis is essential for cross-cutting identification

## Large Projects (300-1000 files)

- 20-40 WPs
- Multiple sessions required
- Consider running WPs in parallel using `isolation: "worktree"`
- May need intermediate synthesis checkpoints

## Monorepos / Umbrella Apps

- Treat each app/package as a top-level component group
- Create WPs within each group -- decompose by **domain** within apps, not just per-app
- Add a cross-app synthesis WP that looks for inter-app duplications
- Use batch ordering to audit foundational/leaf apps first (e.g., shared UI libraries, core domain) before apps that depend on them
- Encode cross-app dependency constraints as architectural boundary checks (e.g., "shared UI library must NOT reference core domain structs")

> **Lesson from umbrella audit**: A 5-app umbrella with ~734 files required 17 component WPs + 1 synthesis, organized into 6 dependency-ordered batches. The largest app (~500 files) was split into 13 domain-based WPs. Without domain decomposition within that app, WPs would have been >60 files and exceeded context limits.

## Polyglot Projects

- Separate rule sets per language
- Group WPs by language first, then by domain
- Cross-language synthesis focuses on R6 (Highlander) and API consistency

### Polyglot Considerations

- **X-rules**: Cross-ecosystem rules (e.g., "API contracts match between Rust backend and Swift client"). Define these as X1, X2, etc. alongside per-ecosystem R-rules.
- **Two-pass synthesis**: First synthesize within each ecosystem, then run a cross-ecosystem pass focusing on X-rules and shared patterns.
- **Effective file count**: Apply ecosystem-specific weights (Rust 1.5x, Swift AppKit 1.3x) when sizing WPs. A 256-file polyglot project with Rust and Swift code may have 300+ effective files.

# Appendix B: Prompt Variations by Language

## Rust-Specific Additions to Prompt

```
Additional context for Rust audits:
- R1 (unwrap): Check context -- main/CLI is Medium, library/daemon is High.
  LazyLock initializers with expect() on static initialization are exempt.
- R4 (unsafe): Severity scales with security boundary. Unsafe in FFI/system
  calls near user input: High. Unsafe in performance-critical inner loops
  with documented invariants: Medium.
- Check trait implementations for missing derive macros
- Look for .unwrap() and .expect() in non-test code
- Verify error types implement std::error::Error + Display
- Check for unnecessary clones (ownership violations)
- Verify unsafe blocks have SAFETY comments explaining the invariant
```

## Swift-Specific Additions to Prompt

```
Additional context for Swift audits:
- R1 (force unwrap): Tests and previews are exempt. Production force
  unwraps on IBOutlets are High (crash on load).
- R6 (Highlander): AppKit and SwiftUI views have different complexity
  baselines. Don't cross-compare line counts between frameworks.
- R8 (try?): Acceptable for fire-and-forget (logging, analytics).
  Not acceptable for data persistence or network calls.
- Check for force unwraps (!) outside test targets
- Verify protocol conformances are in extensions
- Look for massive view bodies (>100 lines in body or viewDidLoad)
- Check for retain cycles in closures (missing [weak self])
- Verify Codable conformance on network types
```

## Ash-Specific Additions to Prompt

```
Additional context for Ash Framework audits:
- A1: Any Ash.get!/read!/create!/load! call outside a domain module's
  code interface is a violation. Severity: High (breaks authorization).
- A3: Actor must be set at query construction time, not at read/action
  time. Check for |> Ash.read!(actor: user) patterns.
- A5: Cross-domain access -- Domain A calling Domain B's resources
  directly (bypassing B's code interfaces) breaks domain boundaries.
- Check for mix ecto.gen.migration (should be mix ash.codegen)
- Verify require Ash.Query at module level (not inside function body)
```

## TypeScript/React-Specific Additions to Prompt

```
Additional context for TypeScript/React audits:
- Check for any types (should be explicitly typed)
- Look for business logic in React components (should be in hooks/services)
- Verify error boundaries around async operations
- Check for missing dependency arrays in useEffect
- Look for prop drilling (>3 levels should use context)
```

# Appendix C: Quick-Start Checklist

For a new project audit, use `/in-tca-init` or follow this manual checklist:

- [ ] Define rule set (15-20 rules, numbered R1-R{N}; add A1-A5 for Ash projects)
- [ ] Specify rule precision boundaries (what each rule does NOT apply to)
- [ ] For polyglot: define X-rules for cross-ecosystem concerns
- [ ] Enumerate all source files (`find . -name "*.{ext}" | wc -l`)
- [ ] Calculate effective file counts using weight table
- [ ] Identify Ash DSL resources, emission/struct files, dead stubs
- [ ] Map files into 8-15 WPs (12-20 effective files each)
- [ ] Create steel thread with info.md, design.md, tasks.md
- [ ] Create WP directories with info.md and empty socrates.md
- [ ] Run Phase 0.5 pre-filtering (grep for R15/R8/R11)
- [ ] Verify file manifests (all listed files exist)
- [ ] For each WP (`/in-tca-audit`):
  - [ ] Run `/compact` first
  - [ ] Launch sub-agent with full prompt template (include confidence field)
  - [ ] Commit socrates.md immediately after completion
- [ ] Write synthesis WP (`/in-tca-synthesize`): cluster by root cause, 5-tier priority
- [ ] Review with project owner (`/in-tca-remediate` for execution)
- [ ] Execute remediation batches in main conversation with verification gates
- [ ] Wrap up (`/in-tca-finish`): feedback report, ST doc updates

# Appendix D: Reference Implementations

## Example A -- Single-App Elixir

```
intent/st/STNNNN/                # Steel thread root
├── design.md               # 15 rules, 14 components, parallelization plan
├── tasks.md                # Phase checklist with violation counts
├── WP/01-14/socrates.md   # Individual component audits
└── WP/15/socrates.md      # Cross-component synthesis
```

Project: Single-app Elixir/Phoenix/Ash (~258 .ex files)
Rules: 15 (R1-R15)
Total violations: 408 (50 High, 150 Medium, 208 Low)
Dominant rule: R6 (Highlander) at 32% of all violations

## Example B -- Umbrella Elixir (5 apps)

```
intent/st/STNNNN/                # Steel thread root
├── design.md                # 15 rules, 17+1 components, 6 batches, execution protocol
├── tasks.md                 # 5-phase checklist (incl. regression prevention)
├── WP/01-17/info.md         # Component scope with cross-WP Highlander dependencies
├── WP/01-17/socrates.md     # Audit prompts with V{N} format + anti-hallucination
└── WP/18/socrates.md        # Cross-component synthesis (4-tier P0-P3 priority)
```

Project: Elixir/Phoenix/Ash umbrella (~724 .ex files across 5 apps)
Rules: 15 (R1-R15) + architectural boundary checks
Innovations over Example A:

- Cross-WP Highlander dependency encoding at provisioning time
- Dependency-ordered batch parallelization (1A->1E->2)
- Explicit violation output format (`V{N}`) for synthesis parsing
- Anti-hallucination instruction in all prompts
- 4-tier priority classification (P0 bugs, P1 Highlander, P2 thick coordinators, P3 style)
- Phase 5: Regression prevention (custom Credo checks, CI gates)

## Example C -- Polyglot (Elixir + Rust + Swift + Lua)

```
intent/st/STNNNN/                # Steel thread root
├── design.md               # 3 ecosystem rule sets + X-rules, 14 components
├── tasks.md                # Phase checklist with per-ecosystem tracking
├── WP/01-13/socrates.md   # Component audits (grouped by ecosystem)
└── WP/14/socrates.md      # Cross-ecosystem synthesis
```

Project: Polyglot application (~256 files: Elixir, Rust, Swift, Lua)
Rules: 3 rule sets (Elixir R1-R15, Rust R1-R12, Swift R1-R10) + X-rules
Total raw violations: 118
Unique after dedup: 49 (59% dedup rate)
Innovations over Examples A and B:

- Per-ecosystem rule sets with ecosystem-specific severity contexts
- X-rules for cross-ecosystem API contract consistency
- Two-pass synthesis (within-ecosystem then cross-ecosystem)
- Effective file count weighting (Rust 1.5x, Swift AppKit 1.3x)
- Higher dedup rate due to cross-ecosystem pattern overlap (same Highlander violation manifesting in different languages)

# Appendix E: Lessons Learned from Single-App Audit + Remediation

## What Worked Well

### Parallel sub-agent execution

Running 4-9 extraction agents in parallel dramatically reduced wall-clock time. Each agent worked in the same repo (no worktree isolation needed since they touched different files). Total extraction of 14 thick coordinators completed in ~30 minutes wall clock.

### Batched remediation with verification gates

Committing after each batch (A through G) with `mix compile --warnings-as-errors` + `mix test` between batches caught integration issues early. The WP-03 agent changed `&&` to `and` on a non-boolean value -- tests caught it immediately.

### Service extraction pattern consistency

Every extraction followed the same pattern: business logic -> service module with tagged tuple returns, coordinator stays thin (parse -> call -> render/assign). This made the work predictable and parallelizable.

### P0-first ordering

Fixing crash risks before refactoring prevented the "fix creates a new bug" cascading failure mode.

### Anti-hallucination instruction effectiveness

Including "Do NOT invent violations -- only report what you actually see in the code" in every sub-agent prompt reduced false positives by ~30% compared to prompts without this instruction. The most common hallucination mode is reporting violations in functions that don't exist or attributing behavior to the wrong file.

**Lesson**: Anti-hallucination is not optional. Include it in every prompt, even for experienced operators. Sub-agents under context pressure will fabricate findings to appear thorough.

### R5 over-reporting in polyglot audits

When R5 (Multi-Head Functions) lacks the "matchable values only" boundary, sub-agents flag every `if` statement as a potential multi-head conversion. In one polyglot audit, R5 was the most-reported rule (23 raw violations) but had the highest false positive rate (>60%) because agents flagged conditional logic on computed booleans and string parsing.

**Lesson**: Rule precision boundaries are not optional refinements -- they are load-bearing constraints that determine whether a rule produces signal or noise.

## What Didn't Work / Hard Lessons

### Incomplete Batch D

Batch D (Content Pipeline Dedup) was planned for P1-3 through P1-14 but only completed P1-5,6,7,8,9. Items P1-3 (FileWriter), P1-4 (parse_status), P1-10 (blog content type filter) were never done and remain outstanding. **Lesson**: Track completion at the individual item level, not just the batch level.

### The `and` vs `&&` trap

One agent mechanically changed `&&` to `and` in `put_last_modified_header/2` where the left operand was a DateTime (truthy value), not a boolean. This compiled fine but crashed at runtime. **Lesson**: Boolean operator fixes (R8) require semantic understanding of the values involved, not just grep-and-replace. Include explicit guidance in agent prompts: "Only change `&&` to `and` when BOTH operands are known booleans."

### Error message drift during extraction

A controller extraction changed "You must be signed in to subscribe" to the generic "You must be signed in", breaking a test. **Lesson**: When extracting to services, preserve exact user-facing strings. Tests are the safety net here -- always run them.

### Future-dating Highlander violation

The most insidious bug: file-path code used the `date` frontmatter field (date granularity -- visible all day), while DB code used `published_at` column (datetime granularity -- hidden until exact time). Both were "correct" independently but produced different results for the same content. **Lesson**: When there are two code paths for the same concern (Highlander violation), they WILL diverge in subtle ways. The fix is always elimination, not synchronization.

### Context exhaustion across sessions

The full audit + remediation spanned 3+ sessions. Knowledge was lost at each boundary. The plan file (fluttering-wondering-journal.md) was essential for continuity but required manual re-reading. **Lesson**: This is exactly why Steel Thread Zero's memory injection (D8) is needed -- learnings should persist automatically.

## Metrics

### Single-App Audit Phase

- 14 component WPs + 1 synthesis WP
- ~258 .ex files audited
- 408 violations found (7 P0, 14 P1, ~50 P2, ~337 P3)
- Wall clock: ~4 hours for audit, ~2 hours for remediation batches A-D,F,G

### Single-App Extraction Phase

- 14 work packages (+ 1 production bug fix)
- 12 new service modules created
- Net coordinator reduction: ~2,100 lines removed from thick coordinators
- Net new service code: ~2,300 lines (slightly more due to @doc/@spec additions)
- Highlander violations fixed as bonus: 6 (display_site_host, create_preview_recipient, subscription status, blog post fetching, param preparation, test email builder)
- Wall clock: ~1 hour (9 agents in parallel)

### Remaining Violations

After the single-app audit + extraction, 3 P1 Highlander violations remained unfixed:

- P1-3: `build_file_path`/`build_file_content` duplication (FileWriter vs SyncService)
- P1-4: `parse_status/1` in 3 modules with incompatible behavior
- P1-10: `content_type == "blog.post"` filter scattered across 8+ files

## Prevention Recommendations

The violations found in both audits were preventable. See **Intent ST0026 (Steel Thread Zero)** for a comprehensive prevention framework that addresses each root cause:

1. **Rules from commit one** -> CLAUDE.md template with rules baked in
2. **Canonical module registry** -> MODULES.md with ownership declarations
3. **Archetype templates** -> pre-wired thin coordinators
4. **Automated enforcement** -> custom Credo checks for mechanical rules
5. **Decision tree** -> "where does this go?" flowchart
6. **Memory injection** -> `intent claude prime` for session knowledge
7. **Periodic health checks** -> `intent audit health` for drift detection

# Appendix F: Lessons Learned from Umbrella Project Remediation

The umbrella audit covered a 5-app Elixir umbrella (~734 files, ~126k LOC) and remediated 389 violations across Phases A-E. This was 3x the scale of the single-app audit and revealed additional failure modes.

## What Worked Well

### Phased remediation with priority ordering (A->B->C->D->E)

Strict ordering by priority (P0 bugs -> P1 Highlander -> P2 thick coordinators -> P3 style -> E verification) was essential. P0 fixes were small and targeted, making them safe to do first. P1 Highlander extractions created the shared modules that P2 coordinator refactoring then delegated to. Reversing this order would have created circular dependencies.

### Parallel sub-agents for remediation (not just audit)

Launched 4 `elixir` sub-agents simultaneously for extraction work, each touching different files. Phase B batch 2 completed 5 extraction items in ~10 minutes wall clock. Phase C ran 5 coordinators in parallel. Phase D ran 4 style fix agents in parallel. Total remediation wall clock for 389 violations: ~2 hours.

### `defdelegate` as Highlander consolidation pattern

For functions duplicated across many files (e.g., `truncate/2` in 9 files, `get_action/1` in 3 files), the pattern was: extract to a canonical module (`MyApp.Helpers`), then replace each copy with `defdelegate truncate(str, max), to: MyApp.Helpers`. This is the lowest-risk consolidation -- callers don't change their API, just their implementation source.

### Cross-WP Highlander dependency encoding at provisioning time

Each WP's `socrates.md` included a "Cross-WP Highlander Check" section listing suspected overlaps with other WPs. This meant the per-WP auditor flagged suspects that the synthesis WP could cross-reference. Without this, 13 cross-component patterns would have been invisible.

### Compile-after-every-batch discipline

`mix compile --warnings-as-errors` after every batch caught issues immediately. When agents introduced undefined function references or dependency graph violations, they were caught within minutes, not hours later in a test suite.

## What Didn't Work / Hard Lessons

### Linter interference with sub-agent output

The project's code formatting hook (linter) ran automatically after every file edit. When sub-agents extracted functions from LiveViews, the linter sometimes:

1. **Removed too aggressively** -- deleted `defp` functions it thought were unused (because callers had been updated to use the helper module), but some functions were still called locally without the module prefix
2. **Created invalid syntax** -- tried to rewrite `defp function_name(...)` as `defp ModuleName.function_name(...)` which is invalid Elixir
3. **Removed code the agent hadn't finished with** -- agent edits file A, linter cleans file A, agent tries to edit file A again and finds its previous changes gone

**Lesson**: Linters and sub-agents are adversaries in a concurrent editing scenario. Options: (a) disable formatting hooks during batch remediation, (b) use `isolation: "worktree"` to keep agent work separate until verified, (c) have the agent do a final compile check after all edits. We used (c) but (a) would have been better.

### `String.to_existing_atom` is not always the right fix

Phase A changed `String.to_atom` -> `String.to_existing_atom` across the board as a P0 safety fix. This broke 10 tests because:

- `parse_character_id` in the markdown importer tried to convert tags like `"@ASIDE"` to existing atoms -- but these atoms are created by the parser itself, they don't pre-exist
- `collect_mentions` in the input parser tried to convert user-typed character names -- these may be new atoms that don't exist yet

**Lesson**: `String.to_existing_atom` is the right fix for LLM output (untrusted, could be anything). It is the WRONG fix for controlled inputs (markdown tags, user-typed names in a known domain). The remediation agent needs context about the trust boundary of each call site, not a blanket rule.

### Umbrella dependency graph violations are invisible until they aren't

The agents consolidated `humanize_id/1` from `llclient` (the leaf UI library) into a shared location. But one caller was in the largest app (core domain), which cannot depend on `llclient`. This compiled fine in development (all apps loaded) but would fail under strict dependency enforcement.

**Lesson**: Umbrella apps need an explicit dependency graph check as part of the audit rules. Add a rule: "Module X in app A must not import/alias modules from app B unless A depends on B in mix.exs."

### Sub-agents creating references to non-existent modules

One extraction agent updated callers to use `DashHelpers.compute_stats/2` and removed the original `defp` -- but never created the `DashHelpers` module. The agent's transcript showed it planned to create it, but hit its turn limit before doing so.

**Lesson**: Set `max_turns` high enough for extraction work (at least 40). Better yet, have agents create the target module FIRST, then update callers. Creation-before-migration is safer than the reverse.

### Agents editing the same file concurrently

Phase B batch 2 had one agent (B.2, helpers expansion) and the main session both editing the same module. The main session added a `defdelegate truncate`, then the agent tried to edit the same file and got a "File has been modified since read" error.

**Lesson**: Before launching parallel agents, ensure their file scopes are truly disjoint. If in doubt, handle the overlapping files in the main session after agents complete.

### Remediation agent failure modes

When remediation was attempted in sub-agents (rather than main conversation), three failure modes emerged:

1. **Permission walls**: Sub-agents hit "file has been modified since read" errors when concurrent agents edited overlapping files
2. **Compile-test cycles**: Sub-agents cannot run `mix compile && mix test` inline -- they need to return to the orchestrator, breaking flow
3. **Context loss**: Sub-agents doing remediation lose the synthesis context, leading to fixes that address the symptom but not the root cause

**Lesson**: Remediation belongs in the main conversation. The only exception is parallel extraction work where file scopes are completely disjoint and the target module is created FIRST.

### R7 false positives on plain maps

R7 (Assertive Struct Access) without the "known defstructs only" boundary causes false positives on:

- JSON-decoded maps (keys genuinely unknown at compile time)
- Config maps loaded from external sources
- Dynamic maps built from user input

**Lesson**: R7 must specify that it only applies to known `defstruct` types. `Map.get(plain_map, :key)` is correct code, not a violation.

## Metrics

### Umbrella Audit Phase

- 17 component WPs + 1 synthesis WP (18 total)
- ~724 .ex files audited across 5 umbrella apps
- 389 violations found across all priority levels
- Wall clock: ~6 hours for audit (5 batches, 3 sessions)

### Umbrella Remediation Phase

- 6 commits across Phases A-E
- ~30 new modules created (shared helpers, action modules, service modules, LiveView helpers)
- 22 files modified for style/convention fixes
- All 3,912 tests passing, zero warnings, credo clean
- Wall clock: ~2 hours for remediation (heavily parallelized)

### Violation Distribution by Rule

- R6 (Highlander): 88 -- dominant issue (23%)
- R9 (Missing else): 45 -- highest-risk systemic pattern
- R3 (Helpers in controllers): 35 -- web app hygiene
- R11 (Missing @impl): 28 -- mechanical
- R5 (Multi-head): 25 -- refactoring opportunities
- Remaining rules: 168 combined

### Scale Comparison

| Metric              | Example A (single-app) | Example B (umbrella) | Example C (polyglot) |
| ------------------- | ---------------------- | -------------------- | -------------------- |
| Files audited       | ~258                   | ~724                 | ~256                 |
| Violations found    | 408                    | 389                  | 118 (49 unique)      |
| WPs                 | 14 + 1                 | 17 + 1               | 13 + 1               |
| New modules created | 12                     | ~30                  | N/A (audit only)     |
| Test suite size     | ~600                   | 3,912                | ~400                 |
| Wall clock (audit)  | ~4 hrs                 | ~6 hrs               | ~5 hrs               |
| Wall clock (fix)    | ~3 hrs                 | ~2 hrs               | TBD                  |
| Dedup rate          | N/A                    | ~45%                 | 59%                  |

**Notable**: The umbrella project had FEWER violations despite being 2.8x larger, because it already had stronger architectural patterns (Ash resources, domain modules, shared helper layers). The violations it did have were more deeply embedded in LiveView and REPL layers -- areas that grew organically with feature work.

**Also notable**: Remediation was FASTER on the larger project because the parallel agent pattern was refined from the single-app audit's experience. More agents, better scoping, fewer conflicts.
