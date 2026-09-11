# Intent Rules

Rules are atomic, cite-able coding standards. Each rule is a small Markdown file with structured frontmatter that captures one principle or antipattern, with bad/good examples and a Detection heuristic. Rules are the single source of truth that skills reference and Critic subagents enforce.

This document is the user-facing guide: what rules are, how they are structured, how to author one, and how they are consumed. The authoritative schema reference lives at `intent/plugins/claude/rules/_schema/rule-schema.md` — read that when authoring tooling. Read this when authoring rules.

## What rules are (and are not)

A **rule** is one atomic coding standard. It has a stable ID, a Detection heuristic, and bad/good examples. Critics enforce rules; skills cite rules; humans link to rules.

A **skill** is a procedural guide loaded on demand. Skills can list rule IDs in frontmatter ("when this skill is active, the following rules apply"), but a skill is never the source of truth for a rule's content — the RULE.md file is.

A **subagent** is a focused worker with its own context window and tool loadout. Critics are subagents that read rules and apply them; the rule library is upstream of the Critics.

Rule, skill and subagent are separate layers, with no duplication:

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
├── prose/style/<slug>/RULE.md
├── author/{craft,style}/<slug>/RULE.md
├── content/{craft,style}/<slug>/RULE.md
├── index.json              # Nothing regenerates or reads it; see "Index regeneration"
└── index.json.template     # Shape for the unbuilt `intent claude rules index`
```

Agnostic rules omit example files and cite `concretised_by:` language-specific rules. Elixir rules have runnable `.exs` examples. Rust / Swift / Lua / Shell rules are textual-only — examples are fenced code blocks inside `RULE.md`. See `_schema/CI-LIMITATIONS.md` for the rationale.

## Rule ID scheme

Format: `IN-<LANG>-<CAT>-<NNN>`

| Segment  | Values                                                                                                                            |
| -------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `IN-`    | Fixed prefix. Distinguishes Intent rules from upstream `ETC-*` rules.                                                             |
| `<LANG>` | `AG` (agnostic), `EX` (Elixir), `RS` (Rust), `SW` (Swift), `LU` (Lua), `SH` (Shell), `PR` (prose), `AU` (author), `CO` (content). |
| `<CAT>`  | Uppercase category code: `CODE`, `TEST`, `ASH`, `PHX`, `LV`, `ARCH`, `HIGHLANDER`, etc.                                           |
| `<NNN>`  | Zero-padded 3-digit suffix, unique within the `<LANG>-<CAT>` bucket.                                                              |

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

| Field          | Purpose                                                                                                        |
| -------------- | -------------------------------------------------------------------------------------------------------------- |
| `id`           | Stable identifier. Format above.                                                                               |
| `title`        | Human-readable one-line name. Matches the H1 heading.                                                          |
| `language`     | One of `agnostic`, `elixir`, `rust`, `swift`, `lua`, `shell`, `prose`, `author`, `content`.                    |
| `category`     | Kebab-case. Matches the directory under `<lang>/`.                                                             |
| `severity`     | `critical`, `warning`, `recommendation`, or `style`.                                                           |
| `summary`      | One or two sentences. Printed with the whole file by `intent claude rules show <id>`; `list` does not show it. |
| `principles`   | Short-name principles this rule embodies.                                                                      |
| `applies_when` | Natural-language circumstances under which the rule fires.                                                     |

### Common optional frontmatter

| Field            | Purpose                                                                                                                                                                                       |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `applies_to`     | Glob patterns that narrow the file set. Used by Critics for file-level gating.                                                                                                                |
| `references`     | Other rule IDs this rule depends on (typically the agnostic rule it concretises).                                                                                                             |
| `concretised_by` | Required on a PATTERN agnostic rule; lists ≥2 language-specific rules that demonstrate it. A PROCEDURAL agnostic rule carries none and discharges the same obligation through `applies_when`. |
| `upstream_id`    | Slug of the upstream `elixir-test-critic` rule this borrows from. See attribution.                                                                                                            |
| `aliases`        | Previous slugs. Lets a rule rename without changing its ID.                                                                                                                                   |
| `status`         | `active` (default), `draft`, `deprecated`. Critics skip non-active rules.                                                                                                                     |

The full field reference, including every optional field and its consumer, lives at `_schema/rule-schema.md`.

### Required sections

Every RULE.md has these H2 sections, in this order:

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

Empty sections are explicitly marked `N/A` rather than omitted, so readers can tell the author considered and rejected the section. Section headings are load-bearing: the headless critic finds its proxy only under an exact `## Detection` heading, and the critic subagents read sections by name. `intent claude rules validate` does not check body sections.

## The agnostic ↔ language pattern

Agnostic rules state a cross-language principle. Language rules concretise it.

```
IN-AG-HIGHLANDER-001  "There can be only one"
├── concretised_by: IN-EX-CODE-006   "Module Highlander" (Elixir)
├── concretised_by: IN-RS-CODE-002   "Crate Highlander" (Rust)
└── ...
```

Every agnostic rule must be pinned to something concrete — this prevents agnostic rules from drifting into vague wisdom — and **which concrete thing depends on what kind of rule it is.**

- A **pattern** agnostic rule governs a code shape, and pins itself with at least two `concretised_by:` language rules that demonstrate it.
- A **procedural** agnostic rule governs an ACTION rather than a code shape, so there is no language-specific spelling of it to point at. **It discharges the same obligation through `applies_when`, which must name the SITUATIONS the rule fires in — never virtues.** _Do not fabricate authority_ is vague wisdom; _any moment a row is blocking you and closing it would unblock you_ is a situation a reader can recognise they are standing in.

**THIS IS AN EXCEPTION IN FORM, NOT A HOLE:** a procedural rule with an empty or aspirational `applies_when` has failed the requirement exactly as a pattern rule with no `concretised_by:` would. `IN-AG-RED-CONTROL-001` and `IN-AG-FIAT-001` are the procedural members today.

Language rules cite the agnostic rule via `references:`. Together they form a small graph that skills and Critics walk.

When you author a new language-specific rule, check first whether an agnostic rule already covers the principle. If yes, set `references:` to the agnostic rule and add your new rule's ID to the agnostic rule's `concretised_by:`. If no agnostic rule exists, consider whether the principle is genuinely cross-language — if so, author the agnostic rule first.

## Authoring a new rule

1. **Decide the bucket.** Use `language` + `category` to find the directory. See `intent/llm/DECISION_TREE.md` for placement guidance.
2. **Assign an ID.** Use the next free `IN-<LANG>-<CAT>-<NNN>` for that bucket. Never reuse a removed rule's suffix.
3. **Copy the exemplar.** `intent/plugins/claude/rules/elixir/test/strong-assertions/` is the canonical full-form example (frontmatter + sections + runnable examples). For agnostic rules, copy any directory under `rules/agnostic/`.
4. **Fill the frontmatter** per the schema above.
5. **Write the H2 sections.** Be substantive in `## When This Does Not Apply` — this is what prevents Critic noise.
6. **Author examples.** Elixir rules get runnable `good_test.exs` / `bad_test.exs` (or `good.exs` / `bad.exs` for code rules). Rust / Swift / Lua / Shell rules embed examples inline as fenced code blocks.
7. **Validate.** `intent claude rules validate <id>` checks the frontmatter: declared and required keys, id shape, duplicate ids, cited ids, attribution rows. It does not check sections or run examples.
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
- Every key in the `### Required fields` table of `_schema/rule-schema.md` is present and non-empty (`id`, `title`, `language`, `category`, `severity`, `summary`, `principles`, `applies_when`)
- `id` starts `IN-`, ends in a three-digit number, and has at least two uppercase-or-digit segments between (no language-code list is checked)
- If `upstream_id:` is set (and not `null`), some `_attribution/*.md` has a row pairing this rule's id with that slug
- Every id cited in `references:`, `concretised_by:`, `related_rules:` or `conflicts_with:` is declared by a rule in the library
- Every top-level frontmatter key is declared in `_schema/rule-schema.md` (an undeclared key is an error)
- No two rule files declare the same `id`

Findings go to stderr as `error: <path>: <message>`. Stdout prints `<N> ok`, or `<E> error(s), <W> warning(s) across <N> rule(s)` with exit 1.

Run before every rule commit. The validator is fast (whole library validates in well under a second) so there is no excuse to skip it.

## Index regeneration

`intent claude rules index` is declared and not built: it exits 2 with `is a known command that is not implemented yet`. Nothing regenerates or reads `intent/plugins/claude/rules/index.json`, so the committed file is not kept in step with the RULE.md files; `intent claude rules list` reads the RULE.md files directly. Whether the verb and the file retire is an open hv decision.

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
3. **Extension rules**: not read in v3 (`userstate::ext_base()` answers `None`), so `intent claude rules list`/`show` serve canon only.
4. **Upstream interop** (Elixir only): if `~/.claude/plugins/elixir-test-critic/rules/` exists, its rules are deduped against Intent rules by `upstream_id:`. Absence is silent.

For each loaded rule, the Critic applies the Detection heuristic from the `## Detection` section to the target files. Findings cite rule IDs; the report groups by severity.

The full Critic contract — modes, ambiguity handling, report format, `.intent_critic.yml` — lives at `intent/docs/critics.md`.

## Adding rules via user extensions

Extension rule packs at `~/.intent/ext/<name>/rules/<lang>/<category>/<slug>/RULE.md` are not read by v3 (`userstate::ext_base()` answers `None`), so they neither join discovery nor shadow a canon rule.

`intent ext` (including `ext new --rule-pack`) is declared and not built in this release; it exits 2 with `is a known command that is not implemented yet`. See `intent/docs/writing-extensions.md` for the extension layout.

`intent claude rules validate <path>` validates a RULE.md at any path against the canon corpus. Extension packs under `~/.intent/ext` are not validated, and the verb prints a `note:` saying so.

## Attribution policy

Intent's rule schema is intentionally compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, copyright 2026 Manuel Zubieta), pinned at commit `1d9aa40700dab7370b4abd338ce11b922e914b14`. Upstream rules drop into Intent's discovery unchanged.

Tiers of borrowing:

| Tier | When                                                                 | What's required                                                            |
| ---- | -------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 1    | Topical overlap only; no principle or wording lifted                 | No attribution                                                             |
| 2    | Principle or Detection heuristic lifted; rewritten in Intent's voice | `upstream_id:` in frontmatter; row in `_attribution/elixir-test-critic.md` |
| 3    | Substantial portion (paragraph-scale prose, literal example code)    | All of Tier 2 + full MIT notice inline in the rule body                    |

The design is to rewrite in Intent's voice rather than copy upstream prose. If a future rule crosses into Tier 3, add the inline MIT notice and update the attribution file.

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
- `intent/plugins/claude/rules/_schema/index-generator.md` -- spec for `intent claude rules index`, which is declared and not built (exits 2); whether it retires is an open hv decision
- `intent/docs/critics.md` — Critic subagent contract and report format
- `intent/docs/writing-extensions.md` — author rule packs as extensions
- `intent claude rules --help` -- `intent claude rules` command reference
