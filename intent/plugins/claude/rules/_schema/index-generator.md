# Rule Index Generator

`rules/index.json` is the bash-readable catalogue of the rule library. Bash cannot parse YAML frontmatter; jq cannot walk nested Markdown. The generator bridges the two: read every `RULE.md`, extract the frontmatter, emit a flat JSON document.

This spec defines the generator's contract. The implementation ships in WP02 as `intent claude rules index` (see `intent/plugins/claude/bin/intent_claude_rules`).

## Invariants

- **Deterministic.** Running the generator twice on the same input produces byte-identical output. No timestamps in the JSON; rules sorted by `id`.
- **Idempotent.** No side effects besides writing `index.json`. Running against an unchanged tree regenerates the same bytes.
- **Flat.** All fields at the top level of the per-rule object are scalars or flat arrays. No nested maps. This keeps `jq` queries simple and bash `read` loops reliable.
- **Skip-tolerant.** A malformed `RULE.md` does not abort generation. The bad rule is omitted from `index.json` and reported on stderr. Generation exits non-zero only if zero rules are indexed at all.
- **Scope.** The generator walks `rules/{agnostic,elixir,rust,swift,lua}/**/RULE.md` only. `rules/_schema/`, `rules/_attribution/`, and any other top-level `_*` directories are excluded — the archetype at `_schema/archetype/strong-assertions/RULE.md` is template, not rule.

## Input

- Rule files: every `RULE.md` under the language-pack roots.
- Frontmatter: YAML between `---` delimiters at the start of the file.
- Field set: defined in `rule-schema.md` (required + optional).

## Output shape

A single `rules/index.json` with this top-level structure:

```json
{
  "schema": "intent-rule-index/v1",
  "intent_version": "2.9.0",
  "rule_count": 42,
  "upstream_pin": "1d9aa40700dab7370b4abd338ce11b922e914b14",
  "rules": [
    { "id": "...", "...": "..." },
    { "id": "...", "...": "..." }
  ]
}
```

Each entry in `rules[]` is a flat object mirroring the frontmatter keys the CLI consumes (per the "Field consumers" table in `rule-schema.md`, the `intent claude rules` column):

```json
{
  "id": "IN-EX-TEST-001",
  "title": "Strong assertions against concrete values",
  "language": "elixir",
  "category": "test",
  "severity": "critical",
  "summary": "Shape assertions (...) pass for any value of the right type...",
  "principles": ["honest-data", "public-interface"],
  "applies_to": ["test/**/*_test.exs"],
  "applies_when": [
    "Any ExUnit test asserting on a return value of a fallible function",
    "Assertions on struct fields, map values, or list contents"
  ],
  "does_not_apply_when": [
    "Property-based tests asserting invariants rather than specific values"
  ],
  "upstream_id": null,
  "references": ["IN-AG-HIGHLANDER-001"],
  "related_rules": ["IN-EX-TEST-002"],
  "concretised_by": [],
  "aliases": [],
  "tags": ["elixir", "exunit", "assertions"],
  "status": "active",
  "version": 1,
  "source_path": "intent/plugins/claude/rules/elixir/test/strong-assertions/RULE.md"
}
```

Notes on shape:

- `upstream_id` and string scalars are `null` when omitted from frontmatter, not `""`. Bash consumers use `jq -r '.rules[] | select(.upstream_id != null)'` to filter.
- Array fields are always arrays, empty `[]` when omitted. Consumers do not need null-guards.
- `source_path` is added by the generator (not in frontmatter). Repo-relative, forward slashes.
- `status` defaults to `"active"` when omitted.
- `version` defaults to `1` when omitted.

Optional frontmatter fields not consumed by the CLI (per `rule-schema.md` §Field consumers) are also emitted so `intent claude rules show --json` can round-trip. Specifically: `sources`, `conflicts_with`.

## Pipeline

The generator is a bash + jq + `awk` pipeline. No Elixir, no Ruby, no Python dependency. Bash 3.x compatible (no `readarray`, no `declare -A`, no `${VAR^}`).

Stages:

### 1. Enumerate

```bash
find "${INTENT_HOME}/intent/plugins/claude/rules" \
  -path '*/_*' -prune -o \
  -name 'RULE.md' -print \
  | sort
```

`-path '*/_*' -prune` excludes `_schema/`, `_attribution/`, and any future `_*` sibling directories. Sort produces deterministic order.

### 2. Split frontmatter

For each file, extract the YAML block between the first two `---` lines:

```bash
awk '
  /^---$/ { count++; if (count == 1) next; if (count == 2) exit; next }
  count == 1 { print }
' "$rule_md"
```

Edge cases:

- File without a frontmatter block: report on stderr `skip: <path>: no frontmatter`, continue.
- File with only one `---` (no closer): same treatment.
- Empty frontmatter: reported as malformed.

### 3. Parse YAML

Intent does not bundle a YAML parser in bash. Two options:

- **Option A** (preferred when available): `yq` (the Go port, v4+) is fast, jq-compatible, and already installed on macOS dev boxes: `yq -o=json`.
- **Option B** (fallback): a minimal shell parser handling the flat-frontmatter invariant. No nested maps in the schema means the parser can be a few dozen lines of `awk` covering: `key: value`, `key: >`-folded multiline, `key:` followed by `- item` list, `[a, b]` inline array, `[]` empty array, quoted strings.

The generator auto-detects: if `yq` is on PATH, use it; else fall back to the bundled parser. Both paths produce the same JSON on valid input.

Output of stage 3: a per-rule JSON fragment with all frontmatter keys present (defaults filled), plus `source_path`.

### 4. Validate and annotate

For each JSON fragment:

- Verify `id` matches `^IN-(AG|EX|RS|SW|LU)-[A-Z][A-Z0-9-]*-[0-9]{3}$` (per `id-scheme.md`).
- Verify required fields exist and match type.
- Fill defaults: `aliases: []`, `status: "active"`, `version: 1`, optional arrays as `[]`, optional scalars as `null`.
- Add `source_path` (repo-relative).

Malformed entries are logged on stderr and skipped from the index. Validation is advisory at this stage — the authoritative validator is `intent claude rules validate` (a separate command).

### 5. Aggregate

Collect per-rule fragments into the final top-level structure:

```bash
jq -s --arg v "$(cat "${INTENT_HOME}/VERSION")" \
       --arg pin "$(extract_pin_from_attribution)" \
       '{
          schema: "intent-rule-index/v1",
          intent_version: $v,
          rule_count: length,
          upstream_pin: $pin,
          rules: sort_by(.id)
        }' <rule-fragments> > "${OUT_PATH}"
```

`extract_pin_from_attribution` reads `rules/_attribution/elixir-test-critic.md` and greps the "Pinned commit" row.

### 6. Byte-exact output

Run `jq -S` (sort keys) + `jq` compact pretty-printing with 2-space indent so `git diff` on `index.json` is readable and stable.

```bash
jq -S . < "${TMP_OUT}" > "${OUT_PATH}"
```

Line endings are LF (no `\r`). No trailing newline beyond the one `jq` emits.

## Consumers

- `intent claude rules list` — jq query by language, severity, category.
- `intent claude rules show <id>` — first loads the index, finds `source_path`, then reads `RULE.md` for full Markdown body.
- `intent claude rules validate` — uses the index as a cheap second pass (after walking files directly).
- `critic-<lang>` subagents — consult the index at invocation time to enumerate rules in their language pack.
- `in-review` skill stage-2 — enumerates the severity-critical-plus-warning subset for the dispatched Critic.

Every consumer reads `index.json` with `jq`. None parses the frontmatter directly. This enforces the Highlander rule: the frontmatter-to-JSON mapping lives in one place (this generator).

## Invalidation

`index.json` is a build artifact. It is checked in (so fresh clones can operate without `jq` installed) but regenerated on every change:

- Developer: run `intent claude rules index` after editing any `RULE.md`.
- Release gate: WP11's pre-release checklist regenerates the index and asserts no diff.
- Future CI: a pre-commit or CI check that runs the generator and diffs against the committed `index.json`.

When `index.json` and the rule files disagree, the rule files win — re-run the generator.

## Error handling

- Exit 0 on success, stderr silent.
- Exit 0 with warnings on stderr when individual rules are skipped for malformation (allows most of the library to be indexed while one rule is broken).
- Exit 1 when zero rules are indexed (empty input or all malformed). This always means something is structurally wrong — e.g. someone moved `rules/` or ran from the wrong directory.
- Exit 2 for generator invocation errors (missing `INTENT_HOME`, missing `rules/` directory, disk-full writing output).

Error messages follow the Intent CLI convention (lowercase prefixes):

```
skip: rules/elixir/test/foo/RULE.md: no frontmatter
skip: rules/agnostic/bar/RULE.md: id "BAD-ID-001" does not match IN-<LANG>-<CAT>-<NNN>
error: no rules found under rules/ — check INTENT_HOME
ok: indexed 42 rules (3 skipped) to rules/index.json
```

## Template

See `rules/index.json.template` for the exact target shape with placeholder values.

## Testing

WP02 acceptance criterion: `tests/unit/rule_index.bats` covers:

- Round-trip: generate, mutate a rule's `severity`, regenerate, diff non-empty.
- Determinism: generate twice, byte-compare.
- Skip behaviour: corrupt one rule's frontmatter, assert it is skipped and others index.
- Empty-tree: rules directory with no RULE.md files exits 1.
- Pin propagation: change the pin in `_attribution/`, regenerate, verify `upstream_pin` updates.
- Bash 3.x: the pipeline runs under macOS default bash (`/bin/bash --version` = 3.2).
