---
verblock: "28 Apr 2026:v0.2: matts - Scope and design pointer"
intent_version: 2.11.0
status: Completed
slug: critic-elixir-false-positives-on-canonical-otp
created: 20260428
completed: 20260428
---

# ST0038: critic-elixir false positives on canonical OTP/Mix idioms

## Objective

Stop the headless `intent critic` (which powers the pre-commit gate) from firing on canonical Elixir/OTP/Mix idioms that are correct by construction.

## Context

Reported from Lamplight ST0163/WP-01 commit attempt. The pre-commit gate refused the commit on three rules that all fired against canonical OTP/Mix code:

```elixir
@impl true
@spec start(Application.start_type(), term()) :: {:ok, pid()} | {:error, term()}
def start(_type, _args) do                  # CODE-002 + CODE-006 fire here
  Supervisor.start_link(...)
end

def run(_args) do                            # CODE-002 + CODE-006 + TEST-002 fire here
  {:ok, started} = Application.ensure_all_started(:control)
  Process.sleep(:infinity)
end
```

`Application.start/2` and `Mix.Task.run/1` are behaviour-mandated callbacks; `Process.sleep(:infinity)` in a long-running Mix task is the canonical way to keep the BEAM alive after `Application.ensure_all_started/1`. All three rules misfired.

User direction: "MUST FIX". Bundles into v2.11.0 ship line.

## Diagnosis

Three issues in the headless critic:

1. **`applies_to` is declared in rule frontmatter but never honoured** by `critic_apply_rule` in `intent/plugins/claude/lib/critic_runner.sh`. Every rule applies to every staged file regardless of path. IN-EX-TEST-002 with `applies_to: ["test/**/*_test.exs"]` fires on `apps/control/lib/...`.
2. **Greppable proxies for IN-EX-CODE-002 and IN-EX-CODE-006 are too coarse**. Both extract `def [a-z_]+\(` as the pattern, which matches every public function definition. The rules' actual concerns ("function returns bare nil/false" and "same function name across multiple modules") are not expressible as a single-file regex.
3. **Pattern extractor takes only the first quoted regex** from a multi-pipe greppable command (`critic_pattern_from_grep_command`). Combined with #2 it amplifies the false positives. Pre-existing limitation; deferred.

## Scope

5 items:

1. **Runner-side**: add `critic_file_matches_glob` and `critic_rule_applies_to_file` helpers to `critic_runner.sh`. Modify `critic_apply_rule` to early-return when the staged file doesn't match the rule's `applies_to`. Glob-to-regex with suffix anchor `(^|/)` so umbrella layouts (`apps/<app>/lib/...`, `apps/<app>/test/...`) match the rule's globs.
2. **Rule-side**: strip the greppable proxy from IN-EX-CODE-002 (tagged-tuple-returns). The Detection prose stays; a one-line note explains the pattern is too coarse for a single-file scan and that the LLM critic-elixir subagent applies the rule via `/in-review`.
3. **Rule-side**: strip the greppable proxy from IN-EX-CODE-006 (module-highlander). Same treatment — the rule is fundamentally cross-file.
4. **BATS coverage**: new tests in `tests/unit/critic_runner.bats` covering `applies_to` honoring (positive + negative cases for umbrella layouts). Update `tests/unit/pre_commit_hook.bats` to stage fixtures under `test/<name>_test.exs` to match `IN-EX-TEST-001`'s `applies_to`.
5. **CHANGELOG**: append a "Fixed" entry under `## [2.11.0]` documenting all three fixes.

T-shirt: **S**.

## Related Steel Threads

- ST0037 — v2.11.0 ship line; ST0038 lands as part of the same release because it's a downstream-reported correctness blocker.
- ST0034 — original critic family (v2.9.0). Greppable proxies authored as the headless-fast-path companion to the LLM critic; this ST tightens the contract on which proxies are safe to ship.

## Context for LLM

This is a small focused fix. Read `design.md` for implementation specifics if it exists; otherwise the scope above is the plan.

### How to update this document

1. Update status as work progresses (use `intent st` commands).
2. Mark completion date when finished.
