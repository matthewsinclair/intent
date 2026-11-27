# Intent Rules

Rules are atomic, cite-able coding standards. Each rule is a small Markdown file with structured frontmatter that captures one principle or antipattern, with bad/good examples and a Detection heuristic. Rules are the single source of truth that skills reference and Critic subagents enforce.

This document is the authoring guide: what rules are, how to author one, and how they are consumed. The references it points to live in `intent/plugins/claude/rules/_schema/`: `rule-schema.md` (every field and its consumer), `id-scheme.md`, `attribution-policy.md` and `CI-LIMITATIONS.md`.

## What rules are (and are not)

A **rule** is one atomic coding standard. It has a stable ID, a Detection heuristic, and bad/good examples. Critics enforce rules; skills cite rules; humans link to rules.

A **skill** is a procedural guide loaded on demand. Skills can list rule IDs in frontmatter ("when this skill is active, the following rules apply"), but a skill is never the source of truth for a rule's content: the RULE.md file is.

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
├── _schema/                # Schema reference, ID scheme, attribution policy, CI limitations
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
└── content/{craft,style}/<slug>/RULE.md
```

Agnostic rules omit example files. Elixir `code` and `test` rules have runnable `.exs` examples. The Elixir framework categories (`ash`, `phoenix`, `lv`) and the other packs are textual-only: their examples are fenced code blocks inside `RULE.md` (`_schema/CI-LIMITATIONS.md` says why).

## Rule IDs

Format: `IN-<LANG>-<CAT>-<NNN>`, eg `IN-AG-HIGHLANDER-001`, `IN-EX-TEST-001`, `IN-RS-CODE-005`, `IN-SH-CODE-002`. The language and category codes are in `_schema/id-scheme.md`.

The numeric suffix is permanent: once a rule ships, that suffix belongs to it for life. A rename changes the slug and records the old slug in `aliases:`; the ID never changes. A removed rule leaves a gap, and its suffix is not reused. This keeps external citations (release notes, blog posts, skill files) valid across releases.

## Anatomy of a RULE.md

Each rule lives in its own directory, alongside its example files:

```
rules/elixir/test/strong-assertions/
├── RULE.md          # The rule itself
├── good_test.exs    # Runnable example demonstrating the correct pattern
└── bad_test.exs     # Runnable example demonstrating the antipattern (still exits 0)
```

The RULE.md file has YAML frontmatter and a fixed sequence of Markdown sections. The required frontmatter:

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

`_schema/rule-schema.md` lists every field, required and optional, with its type and its consumer.

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

## The agnostic and language pattern

Agnostic rules state a cross-language principle. Language rules concretise it.

```
IN-AG-HIGHLANDER-001  "There can be only one"
├── concretised_by: IN-EX-CODE-006   "Module Highlander" (Elixir)
├── concretised_by: IN-RS-CODE-002   "Ownership before clone" (Rust)
└── ...
```

Every agnostic rule must be pinned to something concrete, so that it cannot drift into vague wisdom. What it is pinned to depends on the kind of rule:

- A **pattern** agnostic rule governs a code shape, and pins itself with `concretised_by:`, which lists every language rule that cites it in `references:`, and at least two.
- A **procedural** agnostic rule governs an ACTION rather than a code shape, so there is no language-specific spelling of it to point at. It discharges the same obligation through `applies_when`, which must name the SITUATIONS the rule fires in, never virtues. _Do not fabricate authority_ is vague wisdom; _any moment a row is blocking you and closing it would unblock you_ is a situation a reader can recognise they are standing in.

A procedural rule with an empty or aspirational `applies_when` fails the requirement exactly as a pattern rule with no `concretised_by:` would. `IN-AG-RED-CONTROL-001` and `IN-AG-FIAT-001` are the procedural members.

When you author a new language-specific rule, check first whether an agnostic rule already covers the principle. If yes, set `references:` to the agnostic rule, name its principle in `principles:`, and add your new rule's ID to the agnostic rule's `concretised_by:`. `intent claude rules validate` refuses the rule, naming the file to edit, while any of the three disagrees; `references:` is the home of record, and the short-name is the principle's directory (`highlander` for `IN-AG-HIGHLANDER-001`). If no agnostic rule exists, consider whether the principle is genuinely cross-language, and if so author the agnostic rule first.

## Authoring a new rule

1. **Decide the bucket.** Use `language` + `category` to find the directory. See `intent/llm/DECISION_TREE.md` for placement guidance.
2. **Assign an ID.** Use the next free `IN-<LANG>-<CAT>-<NNN>` for that bucket. Never reuse a removed rule's suffix.
3. **Copy the exemplar.** `intent/plugins/claude/rules/elixir/test/strong-assertions/` is the canonical full-form example (frontmatter + sections + runnable examples). For agnostic rules, copy any directory under `rules/agnostic/`.
4. **Fill the frontmatter** per `_schema/rule-schema.md`.
5. **Write the H2 sections.** Be substantive in `## When This Does Not Apply`: it is what prevents Critic noise.
6. **Author examples.** Elixir `test` rules get runnable `good_test.exs` / `bad_test.exs`, and Elixir `code` rules `good.exs` / `bad.exs`. Both must exit 0 and start with `# EXPECTED: passes`, and the rule's slug goes in `runnable_code_rules` or `runnable_test_rules` in `tests/unit/rule_pack_elixir_runnable.bats`, or its examples never run. Every other rule embeds its examples inline as fenced code blocks. Two-space indentation is mandatory throughout (`_schema/rule-schema.md`, "Formatting invariants").
7. **Validate.** Run `intent claude rules validate <id>` (see Validation).
8. **Wire it up.** If a skill should reference the new rule, add the ID to the skill's `rules:` frontmatter list. A Critic picks the rule up from `intent claude rules list` with no further wiring.
9. **Attribution.** If the rule borrows from `elixir-test-critic`, set `upstream_id:` and add a row to `_attribution/elixir-test-critic.md` (see Attribution).

## Validation

`intent claude rules validate` is the authoring gate.

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
- A rule's `references:`, the pattern principle's `concretised_by:` and the principle's short-name in `principles:` agree
- Every top-level frontmatter key is declared in `_schema/rule-schema.md` (an undeclared key is an error)
- No two rule files declare the same `id`

It does not check sections or run examples. Findings go to stderr as `error: <path>: <message>`. Stdout prints `<N> ok`, or `<E> error(s), <W> warning(s) across <N> rule(s)` with exit 1. A path validates a RULE.md anywhere against the canon corpus. Run it before every rule commit.

## How skills reference rules

Skills cite rules by ID. The rule file owns the prose; the skill is a thin pointer. Example from `in-elixir-essentials/SKILL.md`:

```markdown
| Rule ID          | Slug                              | What it enforces                                     |
| ---------------- | --------------------------------- | ---------------------------------------------------- |
| `IN-EX-CODE-001` | `pattern-match-over-conditionals` | Multi-clause heads beat nested `if`/`case` on shape. |
| `IN-EX-CODE-002` | `tagged-tuple-returns`            | `{:ok, v}` / `{:error, r}` instead of bare `nil`.    |
```

The skill says "here are the rules that apply when this skill is loaded; read the RULE.md file when the situation matches". The skill never restates a rule's prose: that would create a Highlander violation between the skill and the rule.

`tests/unit/rule_reference_skills.bats` checks that each rule-pointer skill (`in-elixir-essentials`, `in-elixir-testing`, `in-ash-ecto-essentials`, `in-phoenix-liveview`) cites a fixed list of rule ids written into the test, so a rule added to a pack is caught only when that list is edited; and `tests/unit/highlander_audit.bats` checks a proxy for restated rule prose over those skills: they carry no fenced code blocks and stay thin.

## How Critics consume rules

A Critic re-reads the rule library on every invocation, through the installed tool: `intent claude rules list --lang <lang>` and `--lang agnostic`, then `intent claude rules show <id>`. It applies each rule's `## Detection` heuristic to the targets and reports findings by rule ID, grouped by severity. The load order, the modes, the report format and `.intent_critic.yml` are the Critic contract, `intent/docs/critics.md`.

## Extensions

Extension rule packs at `~/.local/share/intent/ext/<name>/rules/<lang>/<category>/<slug>/RULE.md` are not read in this build (`userstate::ext_base()` answers `None`), so they neither join discovery nor shadow a canon rule, and `intent claude rules validate` prints a `note:` saying it did not validate them. `intent ext` (including `ext new --rule-pack`) is declared and not built: it exits 2 with `is a known command that is not implemented yet`. See `intent/docs/writing-extensions.md` for the extension layout.

## Attribution

Intent's rule schema is compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, copyright 2026 Manuel Zubieta). A rule that lifts a principle or Detection heuristic from it sets `upstream_id:` and has a row in `_attribution/elixir-test-critic.md`; `_schema/attribution-policy.md` gives the tiers and the steps. Upstream rules are not part of Intent's discovery, and no Critic loads them, even when the upstream plugin is installed under `~/.claude/plugins/elixir-test-critic/`. A copied upstream rule keeps an `ETC-*` id that `intent claude rules validate` refuses.

## See also

- `intent/plugins/claude/rules/_schema/rule-schema.md`: authoritative schema reference (every field, every consumer, schema evolution)
- `intent/plugins/claude/rules/_schema/id-scheme.md`: full ID scheme reference
- `intent/plugins/claude/rules/_schema/attribution-policy.md`: full attribution policy
- `intent/plugins/claude/rules/_schema/CI-LIMITATIONS.md`: runnable (Elixir) vs textual (other languages) examples
- `intent/docs/critics.md`: the Critic contract (`intent claude rules contract` prints it from the install)
- `intent/docs/writing-extensions.md`: author rule packs as extensions
- `intent claude rules --help`: the `intent claude rules` command reference
