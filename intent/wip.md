---
verblock: "30 Apr 2026:v0.69: matts - v2.11.3 verified in field; v2.11.4 docs patch"
intent_version: 2.11.4
---

# Work In Progress

## Current State

**v2.11.4 docs patch.** v2.11.3's strict-proxy fix smoke-tested in Conflab (the canonical field witness): previously-misfiring patterns clear (`IN-EX-CODE-004` no longer flags single-step `case ... do`; `IN-EX-TEST-003` no longer flags compliant `use ExUnit.Case, async: true`). Gate still produces real signal on genuine violations (`IN-EX-TEST-001` / `005` / `007`). No stderr diagnostic spam — expected, since stripped rules no longer carry proxy blocks for the runner to refuse.

The smoke also surfaced an architectural clarity gap that v2.11.4 documents: the headless runner and canon rule library load from `$INTENT_HOME` (the Intent install on `$PATH`), not from each project's plugin tree. A fix to the runner or a canon rule applies across the fleet the moment Intent itself updates. Captured in `intent/docs/critics.md` "Code locality" note.

## Next Up

No active steel thread. Optional follow-on, in order of return:

1. **Per-project canon refresh** for fleet members on stale Intent versions (Anvil v2.10.0, Lamplight v2.11.0, MeetZaya v2.10.0, MicroGPTEx v2.10.0, Conflab v2.11.0). Hygiene only — runner+rules already current via `$INTENT_HOME`; refresh keeps each project's `intent/llm/RULES*.md` and skill copies in sync.
2. **`/in-review` Elixir fleet sweep** — Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab. Catalogue findings per project; do not auto-fix without user direction.
3. **Conflab pre-existing test findings** surfaced during the smoke (`IN-EX-TEST-001` / `005` / `007`, 7 hits across `command_parser_test.exs`, `attribution_test.exs`, `layering_invariants_test.exs`, `lensify/client_test.exs`, `rubric_test.exs`). Not v2.11.3-introduced; folds into Conflab's own backlog.
4. **Deferred v2.11.x backlog** (not blocking):
   - `intent claude upgrade` Phase-2 CLAUDE.md substitution audit (regex mangles `v2.0.0` in migration-history paragraph).
   - Homebrew tap (Conflab pattern).
   - `scripts/release` v2 polish (`--rollback`, log-to-file mirror, prettier output).
   - `$N`-in-SKILL.md trap audit on remaining `in-*` skills.
   - `docs/blog/_drafts/####-shell-critic-inception.md` blog draft (now has v2.10.1 + v2.11.3 + v2.11.4 as second / third / fourth dogfood datapoints).

## Recent

- **2026-04-30**: v2.11.4 cut. Docs patch following Conflab smoke test of v2.11.3. `intent/docs/critics.md` "Code locality" note added — the runner and canon rules load from `$INTENT_HOME`, not per-project. CHANGELOG, wip, restart docs refreshed.
- **2026-04-29**: ST0039 / v2.11.3 cut. Strict-proxy contract in `critic_runner.sh`: refuse Greppable proxies the headless runner cannot honour (pipes, `xargs`, `grep -L`, `-v` filters, `-B/-A` context, awk) with a once-per-rule stderr diagnostic instead of silently degrading. Eight Elixir rules with non-mechanical proxies stripped; `IN-EX-CODE-004` surgically edited (counter line removed, `error -> error` forwarder kept). New `tests/unit/critic_runner_proxies.bats`, plus extensions to `intent_critic.bats` (stderr/JSON separation via `run --separate-stderr`) and `pre_commit_hook.bats`.
- **2026-04-29**: Field bug ingest from Conflab post-v2.11.0 upgrade — pre-commit gate emitted false-positive WARNINGs in a 3-file diff; three commits required `--no-verify`. Diagnosis traced two distinct runner bugs (silent first-quoted-arg extraction; no honouring of `-L` / `xargs grep -l` / pipeline filters) plus eight rules whose proxies were never expressible in single-file headless form.
- **2026-04-28**: ST0037 / v2.11.0 cut. Languages-in-use is now an explicit `languages` field in `intent/.config/config.json`, replacing four sites of filesystem-marker probing. `intent lang init / remove` verbs; `migrate_v2_10_x_to_v2_11_0` back-fills from `RULES-<lang>.md` presence. v2.11.1 length-guard hotfix and v2.11.2 `intent upgrade` dispatcher hotfix shipped same day.
- **2026-04-28 morning**: v2.10.1 shipped (gate-firing fix + `scripts/release` orchestrator + four polish items).

## Parked

_(None.)_
