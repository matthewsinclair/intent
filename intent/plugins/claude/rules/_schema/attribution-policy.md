# Attribution Policy

Intent's rule library is compatible with — and in places derived from — `iautom8things/elixir-test-critic` (MIT, copyright 2026 Manuel Zubieta). This document defines the attribution discipline: when to declare upstream provenance, when to vendor license text, and how the `upstream_id:` frontmatter field interacts with MIT compliance.

## The upstream reference

- **Repository**: https://github.com/iautom8things/elixir-test-critic
- **License**: MIT
- **Copyright**: 2026 Manuel Zubieta
- **Pinned commit**: `1d9aa40700dab7370b4abd338ce11b922e914b14` (HEAD on main at 2026-04-22, time of WP01)
- **Intent's canonical attribution file**: `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`

The pinned commit is load-bearing. Every rule in Intent with `upstream_id:` references an upstream rule at that exact commit. Upstream changes after the pin do not affect Intent rules; re-pinning requires a new ST.

## What MIT requires

MIT permits fork, vendor, modify, sublicense, commercial use. The single binding requirement is:

> The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

For Intent, this means: anywhere we reproduce or adapt upstream content substantially enough to be a "substantial portion", we include the full MIT notice (copyright line + permission text). The notice lives at `_attribution/elixir-test-critic.md` (single source of truth) and is cited from rules that borrow.

## Three levels of borrowing

Not every rule that mentions upstream is a borrow. The discipline has three tiers:

### Tier 1: No attribution required

The rule is Intent's own work; it happens to overlap an upstream rule topically. No principle, wording, or Detection heuristic was lifted.

- Frontmatter: no `upstream_id:`.
- Attribution file: no entry.
- RULE.md body: no upstream mention unless comparing approaches.

### Tier 2: Principle or heuristic borrow (attribution required)

The rule is written in Intent's voice but the underlying principle or Detection heuristic comes from upstream. This is the common case.

- Frontmatter: `upstream_id: <upstream-slug>`.
- Attribution file: rule appears in the "Rules derived from upstream principles" section with a link to the upstream rule.
- RULE.md body: "Further Reading" cites the upstream rule by slug.
- No verbatim prose copying.

Example: Intent's `IN-EX-TEST-002` (no-process-sleep) lifts the principle from upstream's `no-process-sleep` (ETC-CORE-005) but writes the Problem and Detection sections in Intent's voice. `upstream_id: no-process-sleep` is mandatory.

### Tier 3: Substantial portion (full MIT notice required)

Any case where Intent reproduces or adapts upstream content at paragraph scale or larger — verbatim prose, literal example code, or a complete rule definition.

- Frontmatter: `upstream_id: <upstream-slug>`.
- Attribution file: rule appears in the "Substantially derived rules" section.
- RULE.md body: prefaced with the full MIT notice inline (not just a link).
- The portion that's derived is clearly marked.

**Intent does not currently produce any Tier 3 rules.** The reference-and-recommend design (see `intent/st/ST0034/design.md` D4) is explicitly to avoid Tier 3: Intent rewrites in its own voice, never copies upstream prose.

If a future rule crosses into Tier 3, add the full MIT notice to its RULE.md and update this policy with the rule ID.

## When `upstream_id:` is required

Set `upstream_id: <upstream-slug>` whenever **any** of the following is true:

- The Problem statement's central insight comes from upstream.
- The Detection heuristic matches upstream's (same grep pattern, same AST signal, same structural test).
- The example is morally identical to upstream's (same antipattern, same fix), even if rewritten.
- The principle listed in `principles:` was introduced by upstream (e.g. `assert-not-sleep`, `mock-as-noun`).

Set `upstream_id` to the upstream rule's slug (lower-kebab-case, matching the final path segment of the upstream rule directory). Intent scopes the slug by category via the `category:` frontmatter field, so bare slugs are unambiguous. Examples:

```yaml
upstream_id: no-process-sleep      # upstream rules/core/no-process-sleep  (ETC-CORE-005)
upstream_id: start-supervised      # upstream rules/core/start-supervised
upstream_id: async-by-default      # upstream rules/core/async-by-default
```

Verify the slug exists at the pinned commit before committing:

```bash
curl -fsSL "https://raw.githubusercontent.com/iautom8things/elixir-test-critic/1d9aa40700dab7370b4abd338ce11b922e914b14/rules/core/<slug>/RULE.md" >/dev/null
```

A 404 means the slug is wrong (typo, or upstream uses a different directory layout in that category).

When **not** to set it:

- The rule is language-agnostic (lives in `rules/agnostic/`). Agnostic rules predate upstream.
- The rule is Rust / Swift / Lua. Upstream is Elixir-only.
- The rule is Intent-specific Elixir guidance with no upstream counterpart (e.g. Ash-specific rules that upstream doesn't cover).
- Topical overlap without principle borrow.

## Attribution file structure

Single canonical file: `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`. Structure:

```markdown
# Attribution: elixir-test-critic

## Source

- Repository: https://github.com/iautom8things/elixir-test-critic
- License: MIT
- Copyright: 2026 Manuel Zubieta
- Pinned commit: 1d9aa40700dab7370b4abd338ce11b922e914b14
- Pin date: 2026-04-22

## License (MIT)

<verbatim MIT license text>

## Rules derived from upstream principles

| Intent rule    | Upstream slug    | What was borrowed                                  |
| -------------- | ---------------- | -------------------------------------------------- |
| IN-EX-TEST-002 | no-process-sleep | Principle `assert-not-sleep` + Detection heuristic |
| IN-EX-TEST-003 | async-by-default | Principle (safe concurrency by default)            |
| IN-EX-TEST-004 | start-supervised | Principle (test-supervised process lifecycle)      |

Rows above are illustrative — WP05 authors the actual Intent rules and populates `_attribution/elixir-test-critic.md` with the final table. IDs shown are the planned allocations.

## Substantially derived rules

None in v2.9.0.

## Schema compatibility

Intent's rule schema (see `_schema/rule-schema.md`) adopts the frontmatter shape of
this upstream verbatim, with Intent-specific optional field extensions that upstream
tools ignore. Rules from upstream can be dropped into Intent's discovery unchanged.

## Re-pinning

Re-pinning to a new upstream commit requires a new ST. Process:

1. Fetch new commit SHA.
2. Diff upstream rules against pinned versions.
3. Update each Intent rule with changed upstream slug → verify principle still matches.
4. Update this file's pinned commit and date.
5. Run full rule validation pass.
```

## Re-pinning discipline

The pinned commit is frozen for v2.9.0. Re-pinning is a future-ST concern because:

- Changed upstream rule slugs break every `upstream_id:` that references them.
- Changed upstream principles mean Intent rules that borrowed may need rewriting.
- A new pin requires a full re-audit of every `upstream_id:` in the library.

Re-pinning is not a bug fix. It's a scope-bearing change.

## When upstream removes a rule

If upstream removes a rule between pins, Intent rules with the now-dangling `upstream_id:` keep the field but carry a `_attribution_note:` in this file noting the removal. The Intent rule stands on its own; the attribution remains historical.

## When upstream re-slugs a rule

Upstream renaming a rule without changing its semantics: Intent keeps the old slug in `upstream_id:` until re-pin. Upstream's `aliases:` (if they use one) can be cross-referenced at pin time.

## What to do on a new rule

Before committing a new Intent rule:

1. Check whether the principle or Detection comes from upstream.
2. If yes, set `upstream_id:` to the upstream slug.
3. Add a row to the "Rules derived from upstream principles" table in `_attribution/elixir-test-critic.md`.
4. Cite the upstream rule in the new rule's "Further Reading" section.
5. Verify the rule is Tier 2 (principle borrow) and not Tier 3 (substantial copy). If Tier 3, include the full MIT notice inline in the rule body and flag in the attribution table.

`intent claude rules validate` (WP02) enforces: every rule with `upstream_id:` set has a matching row in the attribution file.

## What to do on a rule deletion

If a rule with `upstream_id:` is deleted:

1. Delete the rule directory.
2. Remove the row from the attribution file's "Rules derived" table.
3. The attribution file's "Source" section (pinned commit, MIT license) remains — the file is canonical for the project, not per-rule.

## FAQ

**Q: Can I cite upstream in a rule's Further Reading without `upstream_id:`?**

Yes. "Further Reading" is open-ended. `upstream_id:` is specifically for the derived-from-principle case. If you're pointing readers at upstream for depth or context but the rule is Intent's own, no `upstream_id:` needed.

**Q: What if I borrow from a different source (blog post, book, conference talk)?**

Cite it in `sources:` frontmatter and "Further Reading". `upstream_id:` is reserved for elixir-test-critic. Other external sources don't have a structured attribution mechanism yet; add one if the pattern grows.

**Q: Does an agnostic rule ever need `upstream_id:`?**

No. Upstream is Elixir-specific. Agnostic rules are Intent's own work, concretised by language-specific rules (some of which may cite upstream).

**Q: Can I use upstream's test-helper code verbatim?**

That's a Tier 3 question. If you're copying more than a few lines, include the MIT notice inline and flag the rule in the "Substantially derived" table. In practice, Intent's runnable examples are simple enough that copying from upstream rarely arises.

## Commit hash verification

When re-pinning or auditing, verify the pinned commit:

```bash
curl -s https://api.github.com/repos/iautom8things/elixir-test-critic/commits/1d9aa40700dab7370b4abd338ce11b922e914b14 \
  | jq -r '.sha'
# Expected: 1d9aa40700dab7370b4abd338ce11b922e914b14
```

If the commit returns 404, it has been force-pushed away. That is a signal for a full re-audit, not a silent re-pin.
