---
id: "0003"
title: intent critic rejects declared author/content languages: pre-commit gate errors (exit 2) and fail-opens on every commit
date: 2026-07-13
reporter: matts
status: CLOSED
severity: medium
---

# 0003: intent critic rejects declared author/content languages: pre-commit gate errors (exit 2) and fail-opens on every commit

## Tags

critic, pre-commit-hook, languages, prose-critic

## Summary

When a project declares `author` and/or `content` in its `intent/.config/config.json` `languages` array (via `intent lang init author` / `intent lang init content`), the Intent pre-commit critic gate invokes `intent critic <lang>` once per declared language. `intent critic` only accepts the five CODE languages `elixir | rust | swift | lua | shell` and rejects `author` / `content` with exit 2 (`error: first argument must be a language: ...`). The gate catches the non-zero exit and fail-opens, so every `git commit` in such a project prints an error pair per prose language and the critic gate silently does not run for those languages while still reporting a pass.

## Reproduction

1. In any Intent project, declare a prose language: `intent lang init author` (and/or `intent lang init content`) so `config.json` `.languages` includes `author` / `content`.
2. Stage any file and `git commit`.
3. Observe on stderr, once per prose language:

```
intent critic (author) invocation error (exit 2); fail-open.
error: first argument must be a language: elixir | rust | swift | lua | shell
intent critic (content) invocation error (exit 2); fail-open.
error: first argument must be a language: elixir | rust | swift | lua | shell
```

Direct repro of the underlying reject (no commit needed): `intent critic author` -> `error: first argument must be a language: elixir | rust | swift | lua | shell` (exit 2).

Observed in project Lamplight, `config.json` `languages = ["shell","elixir","lua","swift","rust","author","content"]`, on commits `ab4ca1d90` and `3c2421cd5` (2026-07-13).

## Root Cause

Two Intent surfaces disagree on the language set:

- `intent lang init` accepts `author` / `content` as first-class declared languages, and `/in-session` loads `in-author-essentials` / `in-content-essentials` for them and routes their critique to `critic-prose`.
- The `intent critic <lang>` CLI -- the headless runner the pre-commit gate calls, via `.git/hooks/pre-commit` -> `.git/hooks/pre-commit.intent` -> `intent critic <lang>` per declared language -- hardcodes its accepted argument set to the five CODE languages only.

So the gate iterates the project's declared languages and calls `intent critic author` / `intent critic content`, which the CLI rejects with exit 2. The gate's per-language wrapper treats any non-zero exit as "critic errored, fail-open" -- it cannot distinguish "this language has no code critic" from "the critic ran and failed." The accepted-language set is duplicated (not shared) across `intent lang init`, `intent critic`, and the gate, so the three can drift -- and here they have.

## Impact

- Persistent noise: an error pair on stderr on every commit in any project that declares `author` / `content`. It reads as a failure even though the commit succeeds.
- Silent no-op of the gate: the critic gate does not run for `author` / `content` yet reports a pass. A fail-open quality gate reads as "passed" for a check it never performed. (In practice today prose critique is on-demand, not a commit gate, so no prose check is actually lost -- but the gate is asserting a pass it never made.)
- Low blast radius: fail-open never blocks a commit. The defect is the noise plus the silent-pass, not a broken commit.

## Proposed Fix

Either path closes it; doing both is cleanest:

1. Gate-side (preferred, minimal): the pre-commit critic gate should invoke `intent critic <lang>` only for languages that have a registered CODE critic (elixir / rust / swift / lua / shell) and SKIP prose / on-demand languages (author / content) cleanly -- no invocation, no error, no "fail-open" line. Derive the skip from the same registry `intent critic` validates against so the two cannot drift.
2. CLI-side: teach `intent critic` to accept `author` / `content` and dispatch to `critic-prose`, so `intent critic author <files>` is a valid review (or explicit no-op) rather than an argument error. If prose critique should never gate commits, pair this with fix (1) so the gate does not call it at commit time regardless.

Underlying both: the accepted-language set for `intent critic`, the set `intent lang init` allows, and the set the pre-commit gate iterates should be ONE registry (Highlander) -- declaring a language must never yield a language the gate cannot handle.

## Related

- Surfaced in project Lamplight when `author` / `content` were added to `config.json` `languages` (the prose / web-content lang packs).
- No related Intent issue.

## Resolutions

FIXED (2026-07-13, working tree -- pending matts verify + commit; issue left OPEN). One language registry now lives in `intent/plugins/claude/lib/critic_runner.sh`: `critic_code_languages` (elixir/rust/swift/lua/shell -- the headless CODE critics) and `critic_prose_languages` (author/content -- the on-demand critic-prose subagent), plus `critic_is_code_language` / `critic_is_prose_language` predicates. `bin/intent_critic` derives its argument validation from that registry, accepts `author` / `content` as a clean exit-0 no-op (a message pointing at the critic-prose subagent, not the old exit-2 arg error), and exposes `intent critic --languages` as the inspectable registry surface.

Chosen mechanism (differs from the issue's "preferred" path 1): the pre-commit gate (`lib/templates/hooks/pre-commit.sh`) is UNCHANGED in language logic -- it defers to `intent critic`'s exit code. A declared prose language now returns exit 0, so the gate is silent: no per-commit "invocation error ... fail-open" pair, and no false-pass (prose is an explicit no-op, not a swallowed error, so the gate never claims a pass for a check it could not run). Path 1 (gate queries `intent critic --languages` and skips prose itself) was built and reverted: it made the gate depend on that query returning a clean list -- a broken / older CLI could then silently skip a REAL code critic, worse than the original noise -- and it broke the stub-based `critic_dispatch.bats` model. Exit-code deferral is strictly more robust and keeps the gate free of any language knowledge; the single registry still lives in one place and both the CLI validation and the prose no-op derive from it.

Prose-only-on-content (matts's follow-up): confirmed and guarded, not just asserted. Prose rules declare `applies_to` globs `**/*.md` / `**/*.mdx` / `**/*.html`; the SAME `critic_rule_applies_to_file` gate the code critics use means author/content critique fires only on content files and never on `.ex` / `.sh` / `.lua` -- symmetric to "the Elixir critic fires on `.ex`, not `.lua`."

Guards: `intent_critic.bats` (--languages list excludes prose; author/content no-op exit 0); `pre_commit_hook.bats` (a prose-only project commits clean with no drift noise; a mixed elixir+author project still blocks on the elixir bad-fixture while author is skipped); `critic_runner_applies_to.bats` (prose rules apply to `.md`/`.html`, NOT `.ex`/`.sh`/`.lua`). MODULES.md registers the registry under `critic_runner.sh`. NOTE: `intent lang init`'s accepted set (derived from the `lib/templates/lang/*/` template dirs) is a superset that already includes author/content; the code-vs-prose split is a critic concern owned by `critic_runner.sh`, so the two need no merging -- the gate now handles ANY declared language (code -> runs, prose -> no-op).
