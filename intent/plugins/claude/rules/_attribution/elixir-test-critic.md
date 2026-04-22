# Attribution: elixir-test-critic

Intent's rule library adopts schema and principle content from the upstream `elixir-test-critic` project. This file is the single canonical source of MIT attribution: copyright notice, pinned commit, and the list of Intent rules that derive from upstream.

Schema and discipline for this file are defined in `../_schema/attribution-policy.md`.

## Source

| Field                  | Value                                               |
| ---------------------- | --------------------------------------------------- |
| Repository             | https://github.com/iautom8things/elixir-test-critic |
| License                | MIT                                                 |
| Copyright              | 2026 Manuel Zubieta                                 |
| Pinned commit          | `1d9aa40700dab7370b4abd338ce11b922e914b14`          |
| Pin date               | 2026-04-22                                          |
| Upstream branch at pin | `main`                                              |
| Intent release at pin  | v2.9.0                                              |

The pinned commit is load-bearing. Every Intent rule with `upstream_id:` references an upstream rule at this exact commit. Upstream changes after the pin do not affect Intent rules; re-pinning is a dedicated future-ST concern (see "Re-pinning" below).

## License (MIT)

The following license text is reproduced verbatim from the `LICENSE` file at the pinned commit. It applies to all upstream content that Intent adapts, references, or substantially reproduces.

```
MIT License

Copyright (c) 2026 Manuel Zubieta

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Rules derived from upstream principles

These Intent rules borrow principle and/or Detection heuristic from upstream rules at the pinned commit. Each Intent rule is re-written in Intent's voice; the `upstream_id:` frontmatter field points to the source slug.

| Intent rule | Upstream slug | Upstream path | What was borrowed |
| ----------- | ------------- | ------------- | ----------------- |

_No rules populated yet._

This table is populated by WP05 (Elixir rule pack) as each rule lands. The `intent claude rules validate` tool (WP02) enforces the invariant that every rule with `upstream_id:` set has a matching row here.

Per `../_schema/attribution-policy.md`:

- **Tier 2 (principle borrow, default case)**: row in this table; `upstream_id:` in the rule's frontmatter; "Further Reading" cites upstream by slug.
- **Tier 3 (substantial portion)**: row in the "Substantially derived rules" table below AND the full MIT notice inlined in the rule's `RULE.md` body.

## Substantially derived rules

None in v2.9.0.

The reference-and-recommend integration design (see `intent/st/ST0034/design.md` D4) explicitly avoids Tier 3 by rewriting upstream content in Intent's voice. If a future rule crosses into Tier 3, add a row here and inline the full MIT notice above into that rule's `RULE.md` body.

| Intent rule | Upstream slug | What was copied | MIT notice location |
| ----------- | ------------- | --------------- | ------------------- |

_No rows._

## Schema compatibility

Intent's rule schema (see `../_schema/rule-schema.md`) adopts upstream's frontmatter shape. Intent adds these optional fields that upstream tools ignore:

- `upstream_id:` — pointer to source slug
- `language:` — Intent covers agnostic + Elixir + Rust + Swift + Lua
- `applies_to:` — machine-readable glob patterns
- `references:`, `related_rules:`, `concretised_by:`, `conflicts_with:` — Intent cross-references
- `aliases:` — previous slugs for stable ID resolution

Upstream rules can be dropped into Intent's discovery unchanged. The reverse (Intent → upstream) requires stripping Intent-specific fields — out of scope for v2.9.0 since Intent does not publish rules upstream.

Upstream uses `## When This Applies` / `## When This Does Not Apply` for Markdown section headings; Intent adopts the same wording verbatim for compatibility.

## Re-pinning

Re-pinning to a new upstream commit is a scope-bearing change, not a patch or slipstream. Process:

1. Fetch the new commit SHA from upstream.
2. Diff upstream rules at the new SHA against the pinned versions at `1d9aa40700dab7370b4abd338ce11b922e914b14`.
3. For each Intent rule with `upstream_id:`:
   - If upstream slug unchanged and principle unchanged: no action.
   - If upstream slug renamed: update the rule's `upstream_id:` and add a row to "Attribution notes" below.
   - If upstream principle changed materially: review the Intent rule for drift; rewrite if needed.
   - If upstream rule removed: keep Intent rule; add entry to "Dangling upstream references" below.
4. Update the `Pinned commit` and `Pin date` fields in the Source table.
5. Update the `curl` commit-verification snippet's SHA below.
6. Run `intent claude rules validate` full pass.
7. Commit in a dedicated ST — never bundled with unrelated work.

## Commit hash verification

Before re-pinning or auditing, verify the pinned commit has not been force-pushed away:

```bash
curl -fsSL https://api.github.com/repos/iautom8things/elixir-test-critic/commits/1d9aa40700dab7370b4abd338ce11b922e914b14 \
  | jq -r '.sha'
# Expected: 1d9aa40700dab7370b4abd338ce11b922e914b14
```

Fetch a specific rule's RULE.md at the pin to verify a slug:

```bash
curl -fsSL "https://raw.githubusercontent.com/iautom8things/elixir-test-critic/1d9aa40700dab7370b4abd338ce11b922e914b14/rules/core/no-process-sleep/RULE.md"
```

If either returns 404 or an unexpected SHA, upstream has rewritten history. That is a signal for a full re-audit, not a silent re-pin.

## Dangling upstream references

None at the v2.9.0 pin.

If a future re-pin finds that upstream has removed a rule that Intent currently references, list it here. The Intent rule stands on its own; the attribution remains historical.

| Intent rule | Upstream slug (removed) | Removed at upstream SHA | Date | Note |
| ----------- | ----------------------- | ----------------------- | ---- | ---- |

_No rows._

## Attribution notes

Free-form log for non-trivial attribution events: re-pin discoveries, upstream slug renames, contested borrows, tier reassessments. One entry per event, newest first.

- _No entries yet._
