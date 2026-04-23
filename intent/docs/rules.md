# Intent Rules

Rules are atomic, cite-able coding standards. Each rule is a small Markdown file with structured frontmatter that captures one principle or antipattern, with bad/good examples and a Detection heuristic. Rules are the single source of truth that skills reference and Critic subagents enforce.

This document is the user-facing guide: what rules are, how they are structured, how to author one, and how they are consumed. The authoritative schema reference lives at `intent/plugins/claude/rules/_schema/rule-schema.md` — read that when authoring tooling. Read this when authoring rules.

## What rules are (and are not)

A **rule** is one atomic coding standard. It has a stable ID, a Detection heuristic, and bad/good examples. Critics enforce rules; skills cite rules; humans link to rules.

A **skill** is a procedural guide loaded on demand. Skills can list rule IDs in frontmatter ("when this skill is active, the following rules apply"), but a skill is never the source of truth for a rule's content — the RULE.md file is.

A **subagent** is a focused worker with its own context window and tool loadout. Critics are subagents that read rules and apply them; the rule library is upstream of the Critics.

Three things in three layers, no duplication:

| Layer    | Lives at                                           | Owns                                      |
| -------- | -------------------------------------------------- | ----------------------------------------- |
| Rule     | `intent/plugins/claude/rules/<lang>/<cat>/<slug>/` | What the standard is + how to detect it   |
| Skill    | `intent/plugins/claude/skills/<slug>/SKILL.md`     | When and how to apply rules in a workflow |
| Subagent | `intent/plugins/claude/subagents/<name>/agent.md`  | The agent that consumes rules and reports |

If two of these duplicate the same prose, the duplicate is the bug. The rule file wins.

## Library layout

```
intent/plugins/claude/rules/
├── _schema/                # Schema reference, ID scheme, attribution policy, critic contract
├── _attribution/           # MIT notices for upstream-derived material
├── agnostic/               # Language-agnostic principles (Highlander, PFIC, Thin Coordinator, ...)
│   └── <slug>/RULE.md
├── elixir/
│   ├── code/<slug>/RULE.md
│   ├── test/<slug>/RULE.md
│   ├── ash/<slug>/RULE.md
│   ├── phoenix/<slug>/RULE.md
│   └── lv/<slug>/RULE.md
├── rust/
│   ├── code/<slug>/RULE.md
│   └── test/<slug>/RULE.md
├── swift/<...>/<slug>/RULE.md
├── lua/<...>/<slug>/RULE.md
├── shell/<...>/<slug>/RULE.md
├── index.json              # Generated; sorted, pretty-printed, deterministic
└── index.json.template     # Reference shape for index.json
```

Agnostic rules omit example files and cite `concretised_by:` language-specific rules. Elixir rules have runnable `.exs` examples. Rust / Swift / Lua / Shell rules are textual-only — examples are fenced code blocks inside `RULE.md`. See `_schema/CI-LIMITATIONS.md` for the rationale.

## Rule ID scheme

Format: `IN-<LANG>-<CAT>-<NNN>`

| Segment  | Values                                                                                  |
| -------- | --------------------------------------------------------------------------------------- |
| `IN-`    | Fixed prefix. Distinguishes Intent rules from upstream `ETC-*` rules.                   |
| `<LANG>` | `AG` (agnostic), `EX` (Elixir), `RS` (Rust), `SW` (Swift), `LU` (Lua), `SH` (Shell).    |
| `<CAT>`  | Uppercase category code: `CODE`, `TEST`, `ASH`, `PHX`, `LV`, `ARCH`, `HIGHLANDER`, etc. |
| `<NNN>`  | Zero-padded 3-digit suffix, unique within the `<LANG>-<CAT>` bucket.                    |

Examples: `IN-AG-HIGHLANDER-001`, `IN-EX-TEST-001`, `IN-RS-CODE-005`, `IN-SH-CODE-002`.

The numeric suffix is permanent: once a rule ships, that suffix belongs to it for life. Renames change the slug (and add the old slug to `aliases:`); the ID never changes. Removed rules leave gaps — the suffix is not reused. This invariant keeps external citations (release notes, blog posts, skill files) valid across releases.

Full reference: `intent/plugins/claude/rules/_schema/id-scheme.md`.

## Anatomy of a RULE.md

Each rule lives in its own directory, alongside its example files:

```
rules/elixir/test/strong-assertions/
├── RULE.md          # The rule itself
├── good_test.exs    # Runnable example demonstrating the correct pattern
└── bad_test.exs     # Runnable example demonstrating the antipattern (still exits 0)
```

The RULE.md file has YAML frontmatter and a fixed sequence of Markdown sections.

### Required frontmatter

```yaml
---
id: IN-EX-TEST-001
language: elixir
category: test
severity: critical
title: Strong assertions against concrete values
summary: >
  Shape assertions (`assert is_struct`, `assert is_map`, `refute is_nil`) pass for
  any value of the right type. They do not prove the function did what the test
  claims. Assert on concrete fields and return shapes.
principles:
  - honest-data
applies_when:
  - "Any ExUnit test asserting on a return value of a fallible function"
---
```

| Field          | Purpose                                                       |
| -------------- | ------------------------------------------------------------- |
| `id`           | Stable identifier. Format above.                              |
| `title`        | Human-readable one-line name. Matches the H1 heading.         |
| `language`     | One of `agnostic`, `elixir`, `rust`, `swift`, `lua`, `shell`. |
| `category`     | Kebab-case. Matches the directory under `<lang>/`.            |
| `severity`     | `critical`, `warning`, `recommendation`, or `style`.          |
| `summary`      | One or two sentences. Shown in `intent claude rules list`.    |
| `principles`   | Short-name principles this rule embodies.                     |
| `applies_when` | Natural-language circumstances under which the rule fires.    |

### Common optional frontmatter

| Field            | Purpose                                                                            |
| ---------------- | ---------------------------------------------------------------------------------- |
| `applies_to`     | Glob patterns that narrow the file set. Used by Critics for file-level gating.     |
| `references`     | Other rule IDs this rule depends on (typically the agnostic rule it concretises).  |
| `concretised_by` | Required on agnostic rules; lists ≥2 language-specific rules that demonstrate it.  |
| `upstream_id`    | Slug of the upstream `elixir-test-critic` rule this borrows from. See attribution. |
| `aliases`        | Previous slugs. Lets a rule rename without changing its ID.                        |
| `status`         | `active` (default), `draft`, `deprecated`. Critics skip non-active rules.          |

The full field reference, including every optional field and its consumer, lives at `_schema/rule-schema.md`.

### Required sections

Every RULE.md has all nine sections, in this order:

```markdown
# <Title matching frontmatter>

<One-line restatement of the rule.>

## Problem

## Detection

## Bad

## Good

## When This Applies

## When This Does Not Apply

## Further Reading
```

Empty sections are explicitly marked `N/A` rather than omitted, so readers can tell the author considered and rejected the section. Section headings are load-bearing — `intent claude rules validate` greps for them verbatim.

## The agnostic ↔ language pattern

Agnostic rules state a cross-language principle. Language rules concretise it.

```
IN-AG-HIGHLANDER-001  "There can be only one"
├── concretised_by: IN-EX-CODE-006   "Module Highlander" (Elixir)
├── concretised_by: IN-RS-CODE-002   "Crate Highlander" (Rust)
└── ...
```

Every agnostic rule must list at least two `concretised_by:` rules — this prevents agnostic rules from drifting into vague wisdom. Language rules cite the agnostic rule via `references:`. Together they form a small graph that skills and Critics walk.

When you author a new language-specific rule, check first whether an agnostic rule already covers the principle. If yes, set `references:` to the agnostic rule and add your new rule's ID to the agnostic rule's `concretised_by:`. If no agnostic rule exists, consider whether the principle is genuinely cross-language — if so, author the agnostic rule first.

## Authoring a new rule

1. **Decide the bucket.** Use `language` + `category` to find the directory. See `intent/llm/DECISION_TREE.md` for placement guidance.
2. **Assign an ID.** Use the next free `IN-<LANG>-<CAT>-<NNN>` for that bucket. Never reuse a removed rule's suffix.
3. **Copy the exemplar.** `intent/plugins/claude/rules/elixir/test/strong-assertions/` is the canonical full-form example (frontmatter + sections + runnable examples). For agnostic rules, copy any directory under `rules/agnostic/`.
4. **Fill the frontmatter** per the schema above.
5. **Write the nine Markdown sections.** Be substantive in `## When This Does Not Apply` — this is what prevents Critic noise.
6. **Author examples.** Elixir rules get runnable `good_test.exs` / `bad_test.exs` (or `good.exs` / `bad.exs` for code rules). Rust / Swift / Lua / Shell rules embed examples inline as fenced code blocks.
7. **Validate.** `intent claude rules validate <id>` checks frontmatter, sections, references, and the runnable-example contract.
8. **Wire it up.** If a skill should reference the new rule, add the ID to the skill's `rules:` frontmatter list. If a Critic loads it automatically (every Critic auto-loads its language pack), no further wiring is needed.
9. **Attribution.** If the rule borrows from `elixir-test-critic`, set `upstream_id:` and add a row to `_attribution/elixir-test-critic.md`. See attribution policy below.

### Two-space indentation everywhere

Two-space indentation is mandatory in the rule library, regardless of the target language's ecosystem default. All fenced code blocks (Rust, Swift, Lua, YAML, JSON, Bash) and all runnable `.exs` files use two spaces. The check `grep -nE '^    [^ ]' <rule-dir>/RULE.md` should return nothing — four leading spaces on a non-comment line is a violation.

### The runnable-example contract (Elixir)

Both `good_test.exs` and `bad_test.exs` must exit 0 when run. The `bad` example demonstrates what a Critic would flag; ExUnit itself is not the enforcer. The first non-empty line of each file is `# EXPECTED: passes` (upstream convention).

Critics work statically — they read the source and apply the Detection heuristic. Many antipatterns compile and pass ExUnit while still being wrong (shape assertions, `Process.sleep` for synchronisation, missing `async: true`). If runtime failure were required, every rule would need awkward contrivances.

### Textual examples (Rust / Swift / Lua / Shell)

These languages are textual-only in v2.9.0. Examples live as fenced code blocks inside `## Bad` and `## Good` sections of `RULE.md`. No sibling `good.rs` / `bad.swift` files. Runnable examples for these languages are a future-work item — they would require a Rust / Swift / Lua / Shell CI environment in the Intent repo.

## Validation

`intent claude rules validate` is the canonical authoring gate.

```bash
intent claude rules validate                        # Validate every rule
intent claude rules validate IN-EX-TEST-001         # Validate one by ID
intent claude rules validate path/to/RULE.md        # Validate one by path
```

What it checks:

- YAML frontmatter parses
- All required scalar fields present (`id`, `title`, `language`, `category`, `severity`, `summary`)
- `id` matches `^IN-(AG|EX|RS|SW|LU|SH)-[A-Z][A-Z0-9-]*-[0-9]{3}$`
- `severity` is one of the four allowed values
- All nine required H2 sections present
- H1 heading present and matches `title`
- If `upstream_id:` set, `_attribution/elixir-test-critic.md` exists
- Every `references:` ID resolves to another known rule
- Agnostic rules have `concretised_by:` with ≥2 entries
- Language rules do not have `concretised_by:`

Run before every rule commit. The validator is fast (whole library validates in well under a second) so there is no excuse to skip it.

## Index regeneration

`intent claude rules index` regenerates `intent/plugins/claude/rules/index.json` from the canon RULE.md files. The output is sorted by `id` and pretty-printed (`jq -S`), so running the command twice on an unchanged tree produces byte-identical output. This is what `tests/unit/rule_index.bats` enforces.

```bash
intent claude rules index
```

Extension rules are not included in the canon index — ext-shipped rules are discovered at runtime (see "Extensions" below).

## How skills reference rules

Skills cite rules by ID. The rule file owns the prose; the skill is a thin pointer. Example from `in-elixir-essentials/SKILL.md`:

```markdown
| Rule ID          | Slug                              | What it enforces                                     |
| ---------------- | --------------------------------- | ---------------------------------------------------- |
| `IN-EX-CODE-001` | `pattern-match-over-conditionals` | Multi-clause heads beat nested `if`/`case` on shape. |
| `IN-EX-CODE-002` | `tagged-tuple-returns`            | `{:ok, v}` / `{:error, r}` instead of bare `nil`.    |
```

The skill says "here are the rules that apply when this skill is loaded; read the RULE.md file when the situation matches". The skill never restates a rule's prose — that would create a Highlander violation between the skill and the rule.

The `tests/unit/rule_reference_skills.bats` test enforces that every rule ID a skill cites resolves to a real RULE.md, and `tests/unit/highlander_audit.bats` enforces that no skill duplicates rule prose.

## How Critics consume rules

Critics are thin orchestrators. On invocation, a Critic re-reads the rule library — no caching. The load order:

1. **Agnostic rules**: `intent/plugins/claude/rules/agnostic/*/RULE.md`.
2. **Language rules, mode-filtered**: `intent/plugins/claude/rules/<lang>/<code-or-test>/**/RULE.md`. For `critic-elixir` in `code` mode, this expands across `code/`, `ash/`, `phoenix/`, and `lv/`.
3. **Extension rules** (if present): `~/.intent/ext/*/rules/<lang>/**/RULE.md`. Extension rules override canon by ID; the Critic prints a shadow warning at the top of the report when a collision occurs.
4. **Upstream interop** (Elixir only): if `~/.claude/plugins/elixir-test-critic/rules/` exists, its rules are deduped against Intent rules by `upstream_id:`. Absence is silent.

For each loaded rule, the Critic applies the Detection heuristic from the `## Detection` section to the target files. Findings cite rule IDs; the report groups by severity.

The full Critic contract — modes, ambiguity handling, report format, `.intent_critic.yml` — lives at `intent/docs/critics.md`.

## Adding rules via user extensions

User extensions can ship rule packs at `~/.intent/ext/<name>/rules/<lang>/<category>/<slug>/RULE.md`. Extension rules participate in Critic discovery alongside canon rules. When an extension rule shares an `id:` with a canon rule, the extension wins and the Critic emits a shadow warning. This lets a project override a canon rule's Detection or examples without forking Intent.

To author a rule pack as an extension, scaffold with:

```bash
intent ext new my-rules --rule-pack
```

The generated extension has the right manifest shape and a placeholder rule. See `intent/docs/writing-extensions.md` for the full extension authoring guide.

Extension rules are validated by `intent claude rules validate` the same as canon rules — pass a path or ID and the validator handles either provenance.

## Attribution policy

Intent's rule schema is intentionally compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, copyright 2026 Manuel Zubieta), pinned at commit `1d9aa40700dab7370b4abd338ce11b922e914b14`. Upstream rules drop into Intent's discovery unchanged.

Three tiers of borrowing:

| Tier | When                                                                 | What's required                                                            |
| ---- | -------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 1    | Topical overlap only; no principle or wording lifted                 | No attribution                                                             |
| 2    | Principle or Detection heuristic lifted; rewritten in Intent's voice | `upstream_id:` in frontmatter; row in `_attribution/elixir-test-critic.md` |
| 3    | Substantial portion (paragraph-scale prose, literal example code)    | All of Tier 2 + full MIT notice inline in the rule body                    |

Intent currently ships zero Tier 3 rules — the design is to rewrite in Intent's voice rather than copy upstream prose. If a future rule crosses into Tier 3, add the inline MIT notice and update the attribution file.

The full attribution policy, including the FAQ and re-pinning discipline, lives at `intent/plugins/claude/rules/_schema/attribution-policy.md`. The canonical attribution file is `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`.

## Schema evolution

- Adding optional fields: minor schema bump, backwards compatible.
- Adding required fields: major bump, migration needed (rare).
- Removing fields: major bump, migrate existing rules first.
- Renaming sections: forbidden without a validator update and a migration pass.

Schema changes are proposed via a new ST, not a WP inside an existing ST.

## See also

- `intent/plugins/claude/rules/_schema/rule-schema.md` — authoritative schema reference (every field, every consumer)
- `intent/plugins/claude/rules/_schema/id-scheme.md` — full ID scheme reference
- `intent/plugins/claude/rules/_schema/attribution-policy.md` — full attribution policy
- `intent/plugins/claude/rules/_schema/critic-contract.md` — how Critics consume rules
- `intent/plugins/claude/rules/_schema/CI-LIMITATIONS.md` — runnable (Elixir) vs textual (other languages) examples
- `intent/plugins/claude/rules/_schema/index-generator.md` — `index.json` generation pipeline
- `intent/docs/critics.md` — Critic subagent contract and report format
- `intent/docs/writing-extensions.md` — author rule packs as extensions
- `intent help rules` — `intent claude rules` command reference
