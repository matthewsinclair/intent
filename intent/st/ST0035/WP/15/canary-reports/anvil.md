---
verblock: "27 Apr 2026:v0.1: matts - Anvil canary report"
project: Anvil
wp: ST0035/WP-15
status: pass
date: 2026-04-27
commit: 39c63bd
---

# Anvil canary report

## Outcome

**Pass.** Second canary application of the v2.10.0 canon. Surfaced one canon-installer gap (legacy single-file pre-commit not migrated to chained architecture); fixed in the canon-installer with an additional `MIGRATE_LEGACY_PRE_COMMIT` action and three new BATS scenarios. Two unrelated flyby fixes landed in Anvil (Ash 3.24 dep + code-interface drift). All 12 verification points pass; mix test 192/192.

## Pre-flight

- **Working-tree state at session start**: user had already run `intent upgrade` manually before the session. Working tree had the canon files staged-ish (untracked + the `.intent/` -> `intent/.config/` rename in flight) plus `mix.lock` modified (likely from a partial `mix deps.get` that hit the lazy_html error).
- **HEAD before**: `ed15f41 Drop legacy .intent/version file` (one commit ahead of main of `local`).
- **Doctor (pre)**: clean (Intent CLI v2.10.0 against Anvil v2.8.1; all checks ok).
- **Branch**: `main`. Remotes: `local` (Dropbox), `upstream` (GitHub). Per WP-15 protocol, pushed to `local` only; `upstream` deferred.

## Dry-run summary

`intent claude upgrade` (canon-installer dry-run) reported:

```
.git/hooks/pre-commit: LEGACY (single-file; migrating to chained)

Actions to perform:
  1. MIGRATE_LEGACY_PRE_COMMIT
```

This is a NEW action introduced this session. Pre-Anvil, the canon-installer reported `UP TO DATE` for this state (canon body at `pre-commit`, no `pre-commit.intent`) and took no action -- a fleet-wide bug for any project that ran `intent claude upgrade` before the chaining rewrite landed.

Manual review flagged: `! CLAUDE.md at root is user-authored (no Intent footer marker); not refreshed.` Correct behaviour -- the canon respects authorship.

## Apply summary

`intent claude upgrade --apply` ran the `MIGRATE_LEGACY_PRE_COMMIT` action:

- Moved the existing canon body `pre-commit` -> `pre-commit.intent`.
- Generated a fresh chain stub at `pre-commit` with the `intent-chain-block:start/end` markers.
- Idempotence verified: re-running `--apply` left both files byte-identical.

The chain block now correctly invokes `pre-commit.intent` (the critic gate) on every commit. Confirmed live during the canon commit cycle.

The earlier `intent upgrade` (run by the user before the session) had already done Phase 1 (relocate) + Phase 2 (stamp) + most of Phase 3. This session's `intent claude upgrade --apply` was the missing pre-commit migration.

## 12-point verification

| #   | Check                                                                    | Result |
| --- | ------------------------------------------------------------------------ | ------ |
| 1   | `jq -r .intent_version intent/.config/config.json` -> `2.10.0`           | ok     |
| 2   | `[ -f AGENTS.md ] && [ ! -L AGENTS.md ]`                                 | ok     |
| 3   | `[ ! -e intent/llm/AGENTS.md ]`                                          | ok     |
| 4   | `[ -f usage-rules.md ]`                                                  | ok     |
| 5   | `jq -r '.hooks \| keys[]' .claude/settings.json` returns 3 hooks         | ok     |
| 6   | `[ -x .git/hooks/pre-commit ]` and `[ -x .git/hooks/pre-commit.intent ]` | ok     |
| 7   | `[ -f .intent_critic.yml ]`                                              | ok     |
| 8   | `[ -x .claude/scripts/session-context.sh ]`                              | ok     |
| 9   | `grep -qF 'intent-chain-block:start' .git/hooks/pre-commit`              | ok     |
| 10  | `bin/intent_critic --help` reachable                                     | ok     |
| 11  | `[ -d intent/.config ]`                                                  | ok     |
| 12  | `[ ! -e .intent ]`                                                       | ok     |

## Project-specific notes

- **Pre-commit was the canon body itself** (legacy single-file install pattern from the pre-chaining era). The new `MIGRATE_LEGACY_PRE_COMMIT` action handled this cleanly. This pattern is expected on any project last touched by Intent before the chained architecture landed -- watch for it on the remaining canaries.
- **Anvil has no prettier in its pre-commit.** Unlike Laksa (which carries prettier + mix-format), Anvil's previous pre-commit was just the canon body. After migration, the chain stub is minimal (`#!/usr/bin/env bash; set -u; <chain block>`) and that is the entire pre-commit. The user can add formatter logic later if desired.
- **CLAUDE.md content drift**: existing CLAUDE.md is user-authored (no Intent footer marker). Canon respects authorship and did not modify it. A separate session can refresh by running `intent claude upgrade --force` or by hand-editing against `lib/templates/llm/_CLAUDE.md`.
- **`/AGENTS.md.bak`** added to `.gitignore` (regen safety net) to align with Intent / Laksa housekeeping.

## Issues encountered (and resolutions)

Three issues, two flyby fixes plus one canon-installer hardening:

1. **Canon-installer LEGACY pre-commit detection** (canon bug): the installer reported `UP TO DATE` when `pre-commit` was the canon body verbatim and `pre-commit.intent` was absent. Bug: this state is the legacy single-file install from the pre-chaining era; the installer should migrate it. Fix landed in `intent/plugins/claude/bin/intent_claude_upgrade` plus three new BATS scenarios. **The `INSTALL_PRE_COMMIT` action was also updated** so fresh installs produce the chained architecture from the start (canon body at `pre-commit.intent`, chain stub at `pre-commit`); previously fresh installs produced the legacy state. 788/788 BATS green (was 785).

2. **lazy_html `only: :test` blocked `mix deps.get`** (Anvil bug, Ash 3.24 era): Anvil's `mix.exs` declared `lazy_html` as `only: :test`, but `lucide_icons` (added in some recent dep update) requires `lazy_html` unconditionally. Mix rejected the inconsistent `:only` constraint. Fix: dropped `only: :test` so `lazy_html` is available everywhere `lucide_icons` needs it.

3. **`Anvil.Projects.create/3` undefined** (Anvil bug, code-interface drift): four tests in `test/anvil/policies/project_policy_test.exs` called `Anvil.Projects.create(name, attrs, opts)`. The domain renamed the code interface from `:create` to `:create_project` in commit `ea3f189` (2025-08-20, "Fixed an Ash regression after a hex update"); tests were not updated. Ash 3.24 no longer generates a positional-name `create/3` signature, so the calls produced `UndefinedFunctionError`. Fix: updated all 4 calls to `Anvil.Projects.create_project(%{name: ..., organisation_id: ...}, actor: ...)`. mix test now 192/192 pass.

## Decision

**Proceed.** Second canary clean once the canon-installer migration was added. The fix is fleet-wide -- watch for `LEGACY (single-file)` reports on the remaining canaries; the installer now handles them automatically. Two flyby fixes were necessary to get Anvil compiling and testing under Ash 3.24; both unrelated to the canon work but expected when reanimating a project that has been dormant for several months.

Anvil committed as `39c63bd Intent upgrade to 2.10.0` (user-authored single commit covering canon application + flyby fixes; user opted to bundle rather than split). Pushed to `local`.

Next canary: any of Molt, Utilz, Arca, Prolix, MicroGPTEx, Sites. Conflab + Lamplight still deferred. Pplr out of scope.
