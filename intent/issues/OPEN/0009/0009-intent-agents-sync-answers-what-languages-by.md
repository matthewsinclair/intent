---
id: "0009"
title: intent agents sync answers 'what languages?' by filesystem probe while the rest of Intent reads the declared languages array
date: 2026-07-29
reporter: matts
status: OPEN
severity: low
---

# 0009: intent agents sync answers 'what languages?' by filesystem probe while the rest of Intent reads the declared languages array

## Tags

agents, agents-sync, generator, highlander, languages, config

## Summary

`intent agents sync` decides what a project is built in by probing the filesystem -- `package.json`, `mix.exs`, `Cargo.toml`, a `.bats` file under `tests/`. Everything else in Intent answers that question from the `languages` array in `intent/.config/config.json`, which ST0037 made authoritative precisely because filesystem presence is unreliable evidence. Two mechanisms answer one question, which is a Highlander violation independent of any bug either of them currently causes.

Raised by hv inside issue 0008, explicitly as context rather than as part of that defect ("a fix that gates on `languages` would resolve both at once"). Filed separately because acting on it changes every consumer's generated `AGENTS.md`, which is a decision in its own right rather than a detail of the bash-prerequisite fix.

## Reproduction

Not a failure, so there is nothing to reproduce as a break. The divergence is visible by reading `intent/plugins/agents/bin/intent_agents` against the `/in-session` skill:

```
$ grep -n 'has_mix_exs=true' -B1 intent/plugins/agents/bin/intent_agents
264:  if [ -f "$PROJECT_ROOT/mix.exs" ]; then
265:    has_mix_exs=true
```

versus the skill's own statement of the contract: _"Languages-in-use is a configuration decision, not a filesystem detection."_

A project can therefore be told it needs an Elixir toolchain on the strength of a stray `mix.exs`, or not told despite declaring `elixir`, and nothing reconciles the two views.

## Root Cause

`intent/plugins/agents/bin/intent_agents`, the detection block (~lines 258-290), sets `has_package_json` / `has_mix_exs` / `has_cargo_toml` / `has_makefile` / `has_bats` from `[ -f "$PROJECT_ROOT/<marker>" ]` probes. It never reads the `languages` array. ST0037 replaced exactly this style of detection elsewhere in the tool but did not reach this generator.

The probes do double duty -- they also derive `test_command` and `build_command` (`mix test`, `cargo build`, ...) -- which is why this is not a one-line substitution. A build command genuinely does depend on the marker file being there; a _prerequisite_ depends on the declared language. The two uses have been conflated into one flag each.

## Impact

Low today. The probes are mostly right, because a project that declares `elixir` usually does have a `mix.exs`. The cost is structural rather than behavioural:

- Two sources of truth for one question, so they can disagree with no mechanism to notice.
- The generated `AGENTS.md` is the contract every agentic CLI reads, so when they do disagree the wrong answer is stated authoritatively -- which is what made 0008 worth fixing.
- A project that declares a language with no marker file (a shell project, a docs project, a polyglot repo whose markers sit in subdirectories) is invisible to the generator.

## Proposed Fix

Separate the two questions the flags currently conflate:

1. **Prerequisites** come from `has_project_language <lang>` (added in `bin/intent_helpers` under 0008). This is the declared-intent question.
2. **Test / build commands** keep their filesystem probes, because they genuinely depend on the marker file existing at a known path.

Sizing note: this changes the Prerequisites block of every consumer's `AGENTS.md` on their next `intent agents sync`. Projects that never ran `intent lang init` have an empty `languages` array and would lose their prerequisite lines entirely, so the change wants either a migration that back-fills `languages` from the current probes, or an explicit decision that an undeclared project gets no prerequisites listed. That decision is the substance of this issue.

## Related

- 0008 -- fixed the one concrete defect this smell caused (the ungated `Bash 4.0+` prerequisite), and added `has_project_language`. hv flagged the structural point there and it was deliberately not folded in.
- ST0037 -- made the `languages` array authoritative and replaced filesystem-marker detection; this generator was not migrated.
- `IN-AG-HIGHLANDER-001` -- the principle this violates.

## Resolutions

{{TBC}}
