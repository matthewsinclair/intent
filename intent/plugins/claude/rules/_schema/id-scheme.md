# Rule ID Scheme

Every rule in Intent's rule library has a stable, cite-able identifier.

## Format

`IN-<LANG>-<CAT>-<NNN>`

Three fixed segments separated by hyphens:

| Segment  | Values                                                                                                                   |
| -------- | ------------------------------------------------------------------------------------------------------------------------ |
| `IN-`    | Prefix. Fixed. Distinguishes Intent rules from `ETC-*` (elixir-test-critic) when both are loaded.                        |
| `<LANG>` | Language code. One of: `AG` (agnostic), `EX` (elixir), `RS` (rust), `SW` (swift), `LU` (lua), `SH` (shell — bash + zsh). |
| `<CAT>`  | Category code. Short abbreviation in uppercase (`CODE`, `TEST`, `ASH`, `PHX`, `LV`, `ARCH`, `MOCK`).                     |
| `<NNN>`  | Zero-padded 3-digit sequence, starting at `001`. Scope: unique within a `<LANG>-<CAT>` prefix.                           |

## Examples

| ID                     | Parsed                                                   |
| ---------------------- | -------------------------------------------------------- |
| `IN-AG-HIGHLANDER-001` | Intent / agnostic / Highlander principle category / #001 |
| `IN-EX-CODE-001`       | Intent / Elixir / code category / #001                   |
| `IN-EX-TEST-001`       | Intent / Elixir / test category / #001                   |
| `IN-EX-ASH-002`        | Intent / Elixir / Ash category / #002                    |
| `IN-EX-PHX-003`        | Intent / Elixir / Phoenix category / #003                |
| `IN-EX-LV-001`         | Intent / Elixir / LiveView category / #001               |
| `IN-RS-CODE-001`       | Intent / Rust / code / #001                              |
| `IN-RS-TEST-002`       | Intent / Rust / test / #002                              |
| `IN-SW-CODE-001`       | Intent / Swift / code / #001                             |
| `IN-LU-CODE-003`       | Intent / Lua / code / #003                               |

## Language codes

Fixed, two-letter, uppercase. Extending this list requires a schema bump.

- `AG` — agnostic (cross-language principles)
- `EX` — Elixir
- `RS` — Rust
- `SW` — Swift
- `LU` — Lua
- `SH` — Shell (bash + zsh both fall under this code; per-rule frontmatter tags distinguish bash-specific, zsh-specific, or both)

Why two letters: short enough to read inline in Critic reports; distinct enough to grep.

Why not `EL` for Elixir: `EX` matches the `.ex` / `.exs` file extensions and reads as "Elixir" in context.

Why not `BA`/`ZS` for bash/zsh separately: about 80% of shell rules (quoting discipline, `$()` over backticks, no-parse-`ls`, no `eval` on untrusted input) apply identically to both. Splitting into two language codes would force Highlander violations for every shared rule. Shell-dialect divergence is real (`set -e` vs `setopt err_exit`, 0- vs 1-based array indexing, word-splitting defaults) and is handled by splitting THAT concern into two separate `IN-SH-*` rules with distinct slugs, tagged `bash-specific` or `zsh-specific`. The language code stays `SH`.

## Category codes

Upper-case short slug. Categories are established per-language as the pack grows. Some are shared conventions:

| Code         | Meaning                             | Typical languages       |
| ------------ | ----------------------------------- | ----------------------- |
| `CODE`       | Production code rules               | all                     |
| `TEST`       | Test rules                          | all                     |
| `ARCH`       | Architecture-level principles       | agnostic                |
| `ASH`        | Ash framework                       | elixir                  |
| `PHX`        | Phoenix framework                   | elixir                  |
| `LV`         | LiveView                            | elixir                  |
| `MOCK`       | Mocking / test doubles              | elixir (future: others) |
| `HIGHLANDER` | Highlander rule                     | agnostic                |
| `PFIC`       | Pure Function / Impure Coordination | agnostic                |
| `THIN-COORD` | Thin Coordinator                    | agnostic                |

Categories are not strictly hierarchical. A rule fits one `<LANG>-<CAT>` bucket; that bucket is what appears in its ID.

## Numbering rules

### Numeric suffix is permanent

Once a rule ships with `IN-<LANG>-<CAT>-NNN`, **that numeric suffix belongs to that rule for the life of the rule**.

- If the rule is renamed (slug changes), the ID does not change.
- If the rule is removed, its suffix is **not reused**. Gaps in the numbering are expected and acceptable.
- If the rule is superseded by a different rule, the new rule gets the next free suffix; it does not inherit the old one.

This invariant allows external citations (skill rule lists, release notes, blog posts) to remain valid across Intent releases.

### Starting sequence

- First rule in a bucket: `001`.
- Zero-padded to three digits.
- Scope of uniqueness: `<LANG>-<CAT>`. So `IN-EX-CODE-001` and `IN-EX-TEST-001` are distinct rules in distinct buckets, both starting at `001`.

### When the 3-digit ceiling approaches

Current capacity per bucket: 999 rules. If any bucket reaches 900, treat it as a signal to split the category (not to extend to four digits). Splitting a category requires:

- New category code.
- Migration of rules that fit the new code (with `aliases:` for old IDs in each moved rule).
- Schema-level review.

Four-digit suffixes (`IN-EX-CODE-1000`) are forbidden in v2.9.0. Re-category before hitting the ceiling.

## Renames (allowed)

A rule's slug (`<slug>` segment in the directory path and `slug:` frontmatter field) can change without changing the numeric ID. Store the previous slug in `aliases:`:

```yaml
---
id: IN-EX-TEST-001
slug: strong-assertions
aliases:
  - no-shape-tests
---
```

Downstream references by ID (`IN-EX-TEST-001`) continue to resolve. References by slug (`no-shape-tests`) resolve via the alias list.

The `intent claude rules validate` tool checks that alias lookups still resolve to the expected rule.

## Cross-reference conventions

Within rule bodies and skill rule-lists, cite rules by full ID:

- Correct: `See IN-AG-HIGHLANDER-001 for the cross-language principle.`
- Avoid: `See the Highlander rule.` (ambiguous if multiple versions ship)

Cross-rule references in frontmatter use IDs in arrays:

```yaml
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-EX-TEST-003
concretised_by:
  - IN-EX-CODE-001
  - IN-RS-CODE-002
```

## Comparison with upstream

Upstream (`elixir-test-critic`) uses `ETC-<CAT>-<NNN>` — a two-segment scheme without a language code because upstream is Elixir-only.

Intent's three-segment scheme has the language code because Intent covers Elixir + Rust + Swift + Lua + agnostic. The cost of the extra segment is worth the clarity when rules from five sources appear in one report.

When Intent ports an upstream principle, the Intent rule stores the upstream slug in `upstream_id:`. The IDs are not mechanically convertible — Intent assigns its own sequence. Planned allocations for v2.9.0 (WP05 authors the actual rules):

| Intent ID        | Upstream `upstream_id` | Upstream ID    | Upstream path                 |
| ---------------- | ---------------------- | -------------- | ----------------------------- |
| `IN-EX-TEST-002` | `no-process-sleep`     | `ETC-CORE-005` | `rules/core/no-process-sleep` |
| `IN-EX-TEST-003` | `async-by-default`     | `ETC-CORE-*`   | `rules/core/async-by-default` |
| `IN-EX-TEST-004` | `start-supervised`     | `ETC-CORE-*`   | `rules/core/start-supervised` |

`IN-EX-TEST-001` (strong-assertions) is Intent-original — no upstream counterpart (upstream's `test-shape-not-values` is telemetry-scoped, not a general strong-assertions rule).

See `attribution-policy.md` for the attribution discipline.

## Validation

The `intent claude rules validate` tool (spec in WP02) checks:

- ID matches regex `^IN-(AG|EX|RS|SW|LU|SH)-[A-Z][A-Z0-9-]*-[0-9]{3}$`.
- ID directory path matches ID structure: `rules/<lang>/<category>/<slug>/` where `<lang>` and `<category>` are the lowercase forms of the ID segments.
- IDs are unique across the entire library (no two rules share a full ID, even across language packs).
- `aliases:` do not collide with other rules' current slugs.
- References (`references:`, `related_rules:`, `concretised_by:`, `conflicts_with:`) all resolve to existing IDs or aliases.

## Do / Don't

| Do                                                  | Don't                                           |
| --------------------------------------------------- | ----------------------------------------------- |
| `IN-EX-TEST-042`                                    | `in-ex-test-42` (case, padding)                 |
| `IN-EX-CODE-001`                                    | `IN-EX-CODE001` (missing hyphen)                |
| `IN-EX-CODE-001` after rename                       | `IN-EX-CODE-010` to "make room"                 |
| Reuse `IN-EX-TEST-003` after removing by `aliases:` | Reassign `IN-EX-TEST-003` to a different rule   |
| `IN-RS-CODE-001`                                    | `IN-RUST-CODE-001` (language code is 2 letters) |
| `IN-EX-PHX-001` for Phoenix                         | `IN-EX-PHOENIX-001`                             |

## Evolution

Adding a new language requires:

- New two-letter language code, agreed on.
- Documentation update here.
- Rule directory `rules/<lang>/` created.
- At least one seed rule in the new pack — subsequent rules copy from it.
- `LANG_SUBDIRS` updated in `intent/plugins/claude/bin/intent_claude_rules`.
- Validator regex updated in the same file.

Adding a new category code within an existing language: lighter process. Add the category code to the `rule-schema.md` category table and start numbering from `001`.
