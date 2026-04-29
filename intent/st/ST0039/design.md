---
verblock: "29 Apr 2026:v0.1: matts - Initial design"
intent_version: 2.11.2
---

# ST0039 — Design

## Runner contract (post-fix)

`bin/intent_critic <lang> --staged|--files ...` invokes `critic_runner.sh::critic_scan_files` which iterates over enabled rules and applies each rule's headless detection. The contract for what a headless detection can be is now strict.

### What counts as a "simple" greppable proxy

A proxy is **simple** (headless-runnable) iff every non-empty, non-comment line in its fenced bash block matches:

```
grep [-rn][E]? [--include=GLOB ...] '<single-quoted-pattern>' <path>...
```

- Exactly one `grep` invocation per line.
- Optional flags from a fixed set: `-r`, `-n`, `-E`, `--include=...`. No `-L`, `-v`, `-B`, `-A`, `-l`, `-c`, `-o`, `-w`, `-x`.
- Pattern is a single-quoted string (the runner does not attempt to parse multi-line scripts, double-quoted patterns with shell expansion, or unquoted patterns).
- No pipes, no `xargs`, no command substitution, no chained commands (`&&`, `||`, `;`).
- Path argument(s) present after the pattern (the runner ignores them — staged files come from the caller — but their presence is part of the canonical form).

Multiple simple lines in the same block are accepted; results are unioned.

### What the runner does with non-simple proxies

For each non-simple line, the runner emits one stderr line:

```
note: skipping IN-EX-XXX-NNN (proxy not headless-runnable)
```

Tagged with the rule_id. The line is suppressed when `--format json` is set on the _findings_ output (stderr stays separate from stdout). If every line in a rule's proxy is refused, the rule produces no findings.

### Helper signatures (additions to `critic_runner.sh`)

```bash
# True iff $1 is a single, simple grep invocation per the contract above.
critic_proxy_is_simple() {
  local line="$1"
  # ... regex-based predicate, bash 3.x compatible, no declare -A
}

# Walks the fenced bash block from $rule_path, emits one pattern per
# accepted line on stdout, emits stderr diagnostics for each refused line
# tagged with $rule_id. Replaces critic_pattern_from_grep_command.
critic_patterns_from_grep_block() {
  local rule_path="$1"
  local rule_id="$2"
  # ... extracts block, walks line-by-line, prints accepted patterns
}
```

### Call site change in `critic_apply_rule`

```bash
# Before (lines 225-238 of critic_runner.sh):
block="$(critic_extract_greppable_block "$rule_path")"
[ -z "$block" ] && return 0
pattern="$(critic_pattern_from_grep_command "$block")"
[ -z "$pattern" ] && return 0
# ... single grep ...

# After:
block="$(critic_extract_greppable_block "$rule_path")"
[ -z "$block" ] && return 0
patterns="$(critic_patterns_from_grep_block "$rule_path" "$rule_id")"
[ -z "$patterns" ] && return 0
# Loop over patterns; union results; dedupe on (file, line)
while IFS= read -r pattern; do
  [ -z "$pattern" ] && continue
  grep -nE "$pattern" "$file" 2>/dev/null
done <<< "$patterns" | sort -u -t: -k1,1n
```

The `sort -u -t: -k1,1n` deduplicates `(line, excerpt)` pairs so two patterns hitting the same line don't double-report.

## Rule-library disposition

The `## Detection` section of each stripped rule loses its fenced bash block but gains (or already has) the standard ST0038-style note. Template:

> **No greppable proxy is authoritative for this rule.** The structural signal is "<one-line description of what body confirmation looks like>". A per-file regex over `<naive pattern>` would false-positive on <list of canonical idioms>. Apply this rule via the LLM-driven `critic-elixir` subagent during `/in-review`, not in the headless pre-commit gate.

The rest of each RULE.md (frontmatter, prose, Bad/Good examples, "When This Applies" / "When This Does Not Apply", references) is unchanged.

## Test design

### Predicate fixtures (`critic_proxy_is_simple`)

| Input                                       | Expected                            |
| ------------------------------------------- | ----------------------------------- |
| `grep -rnE 'pattern' lib/`                  | accept                              |
| `grep -rnE 'pattern' --include=*.ex lib/`   | accept                              |
| `grep -rn 'literal' test/`                  | accept                              |
| `grep -rnE 'pattern' \| wc -l`              | reject                              |
| `grep -rnL 'pattern' test/`                 | reject (`-L`)                       |
| `grep -rnE 'p1' lib/ \| grep -v 'filter'`   | reject (`-v`)                       |
| `grep -rnE 'p1' lib/ \| xargs grep -l 'p2'` | reject (`xargs`)                    |
| `awk '/pattern/ {print}' lib/foo.ex`        | reject (`awk`)                      |
| `grep -rnE -B5 'pattern' lib/`              | reject (`-B`)                       |
| `# comment line`                            | skip (not refused, just not a grep) |
| empty line                                  | skip                                |

### Multi-pattern union fixture

`IN-EX-CODE-005` (no-silent-failures) ships two grep lines:

```bash
grep -rnE 'rescue _ -> (\:ok|nil)' lib/
grep -rnE '^[[:space:]]+_ = [a-z_]+' lib/
```

Test: a fixture file containing both patterns produces two findings (deduped on line if they hit the same line). A fixture file with only one pattern produces one finding.

### Stripped-proxy regression

For each of the 9 stripped rules, assert `grep -c 'Greppable proxy' RULE.md` is 0. This fails loudly if a future PR re-adds an over-broad proxy.

### False-positive reproductions

Two BATS scenarios mirror the field bug:

1. **IN-EX-CODE-004**: stage a `lib/foo.ex` with a single-step `case Repo.get(...) do nil -> {:error, :not_found}; user -> {:ok, user} end`. Run `intent critic elixir --files lib/foo.ex`. Assert exit 0, no findings.
2. **IN-EX-TEST-003**: stage a `test/foo_test.exs` with `defmodule FooTest do\n  use ExUnit.Case, async: true\n  ...end`. Run `intent critic elixir --files test/foo_test.exs`. Assert exit 0, no findings.

### Positive controls

- `IN-EX-TEST-002`: a `test/bar_test.exs` containing `Process.sleep(100)` produces a CRITICAL finding (rule's severity from frontmatter). Pre-commit hook blocks the commit.
- `IN-EX-CODE-005`: a `lib/baz.ex` containing `rescue _ -> :ok` produces a CRITICAL finding.

## Files touched

| File                                                 | Edit shape                                                           |
| ---------------------------------------------------- | -------------------------------------------------------------------- |
| `intent/plugins/claude/lib/critic_runner.sh`         | ~60 LOC added, ~15 deleted; two new helpers; one rewritten call site |
| 9 × `intent/plugins/claude/rules/elixir/.../RULE.md` | strip ~5 LOC fenced block + add ~2 LOC note                          |
| `tests/unit/critic_runner_proxies.bats`              | new file, ~80 LOC                                                    |
| `tests/unit/intent_critic.bats`                      | ~15 LOC added                                                        |
| `tests/unit/pre_commit_hook.bats`                    | ~25 LOC added                                                        |
| `intent/docs/critics.md`                             | ~10 LOC rewrite at "Mechanical subset only" paragraph                |
| `CHANGELOG.md`                                       | new `[2.11.3]` section                                               |
| `VERSION`                                            | `2.11.2` → `2.11.3`                                                  |

## Verification

End-to-end before tag:

1. `bash intent/scripts/run-tests` — all BATS green; new test file exercised.
2. `bin/intent_critic elixir --files <fixture>` repros: single-step `case`, compliant async test, `Process.sleep` test.
3. Stage the field-bug repro fixture; run pre-commit hook directly; assert hook does not block on the false-positive cases.
4. Sanity-check `bin/intent_critic elixir --staged` against an unrelated repo on its current head — no spurious findings.
5. `intent doctor` clean, `intent claude upgrade --apply` self-applies cleanly.
6. `scripts/release --dry-run --patch` previews the v2.11.3 cut.
