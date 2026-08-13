---
id: "0009"
title: intent agents sync answers 'what languages?' by filesystem probe while the rest of Intent reads the declared languages array
date: 2026-07-29
reporter: matts
status: CLOSED
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

FIXED + CLOSED (2026-08-14), shipped in v2.19.0. Confirmed structural, exactly as filed and as deferred out of issue 0008: `intent_agents` answered "what languages?" by filesystem probe while everything else in Intent reads the `languages` array that ST0037 made authoritative *precisely because* filesystem presence is unreliable evidence. Two mechanisms answered one question and could disagree with nothing to notice -- a stray `mix.exs` told every reader the project needs an Elixir toolchain; a polyglot repo whose markers sit in subdirectories declared `elixir` and was told it needs nothing.

**Why it mattered more than it looks.** AGENTS.md is, by its own preamble, the contract every agentic CLI reads and trusts without cross-checking. The wrong answer was not merely present, it was stated authoritatively to the tools least equipped to doubt it.

**What shipped.** Prerequisites read `has_project_language` (the seam 0008 already created). Build and test commands **keep their probes** -- the issue's own split, and correct: `mix test` genuinely depends on a `mix.exs` being at that path. `lua` and `swift` gain prerequisite lines they never had. The empty-section message changed from "None detected" to "None declared", which is what it now means. **No back-fill migration**: declaration is authoritative, and losing an undeclared project's prerequisite lines is the 0008 precedent already ruled correct.

### Two exceptions, stated rather than left as apparent oversights

- **Node stays on its probe.** Intent's declared-language vocabulary has no name for it, so gating the line on a declaration a project *cannot make* would delete it forever -- a silent loss dressed up as consistency. Completing this means adding a `javascript` language pack, which is its own decision and is left with hv.
- **Bats stays on its probe**, for a better reason: it is a test *runner*, not a language. `.bats` files are the right evidence, and declaring `shell` should not assert that a project needs bats installed.

### Reach on upgrade -- the part that took the work

A generator correction that a consumer must know a command to receive does not reach the fleet (the v2.18.0 lesson, one file over). `intent upgrade` synced subagents and skills and left **the one file the CLIs actually read** untouched. It now converges AGENTS.md, with `intent agents sync --check` reporting staleness without writing -- ignoring the generated-by date, which would otherwise report drift daily and make the step fire on every run forever.

**The convergence runs AFTER the canon apply, and the ordering is load-bearing.** It was first written as an `agents_sync` ledger step, per the work order's own suggestion. Exercised end to end, the step reported "already satisfied" and the upgrade still finished with a stale AGENTS.md: the ledger runs *before* the canon apply, and the canon apply creates files (`usage-rules.md`) that AGENTS.md's own file map lists. Generated content derived from post-canon state has to be generated after canon. A dry run could not have shown this; only running the real path in a sacrificial copy did.

**A related trap, recorded:** `intent upgrade` short-circuits when the project is already at the target version and no ledger step reports work. So "the fix reaches consumers" is true because v2.19.0 is a version boundary for everyone, **not** because upgrade re-provisions canon unconditionally. Any future canon-only correction needs a ledger step with a real state probe, or it will not reach a converged project.

**Two existing tests encoded the old contract and were adapted, not deleted.** `intent_agents.bats` asserted the Elixir prerequisite from a bare `mix.exs` -- exactly the behaviour this issue calls wrong -- so its fixture now declares `elixir`; another asserted the "None detected" wording.
