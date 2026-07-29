---
id: "0008"
title: intent agents sync writes an unconditional Bash 4.0+ prerequisite into every AGENTS.md, contradicting projects that target bash 3.2
date: 2026-07-29
reporter: matts
status: CLOSED
severity: medium
---

# 0008: intent agents sync writes an unconditional Bash 4.0+ prerequisite into every AGENTS.md, contradicting projects that target bash 3.2

## Tags

agents, agents-sync, generator, prerequisites, consumer-project, bash-compat

## Summary

`intent agents sync` emits `- Bash 4.0+, POSIX-compliant shell` into the **Prerequisites** section of every generated `AGENTS.md`, unconditionally. It is the only prerequisite in that block not gated on detection, and the version floor it states is wrong for at least one consumer project.

Utilz targets **bash 3.2** deliberately -- macOS ships 3.2.57 as `/bin/bash`, CI exercises it, and the project's `CLAUDE.md` and steel-thread design notes both state the constraint (no namerefs, no `${var,,}`). Its `AGENTS.md` nevertheless tells every agent that reads it that the floor is 4.0+.

This is not cosmetic. The generator's own preamble, four lines above the offending block, states the stakes: _"This is the primary tool-agnostic config file for AI coding agents working on this project. Every major agentic CLI (Claude Code, Codex, Cursor, Copilot, Aider, Continue, Cline, Gemini CLI) reads AGENTS.md as the canonical project contract."_ An agent that trusts the contract will reach for bash-4 constructs that break on the platform the project actually ships to.

## Reproduction

Observed on Intent v2.17.3, across three consumer projects:

```
$ grep -n "Bash 4.0" ~/Devel/prj/{Utilz,Lamplight,Baize}/AGENTS.md
/Users/matts/Devel/prj/Utilz/AGENTS.md:13:- Bash 4.0+, POSIX-compliant shell
/Users/matts/Devel/prj/Baize/AGENTS.md:14:- Bash 4.0+, POSIX-compliant shell
/Users/matts/Devel/prj/Lamplight/AGENTS.md:14:- Bash 4.0+, POSIX-compliant shell
```

Lamplight is Elixir / Rust / Swift / Lua / author / content. Baize is Elixir / Swift / Rust. Neither has a `.bats` file. Both are told they need bash 4.0+.

The contradiction is sharpest in Utilz, which is the shell project:

```
$ cat ~/Devel/prj/Utilz/VERSION
2.4.0
$ /bin/bash --version | head -1
GNU bash, version 3.2.57(1)-release (arm64-apple-darwin25)

$ grep -n "bash 3.2" ~/Devel/prj/Utilz/CLAUDE.md ~/Devel/prj/Utilz/README.md
README.md:497:- Bash 3.2+ or Zsh (bash 3.2 is what macOS ships, and is the floor the framework targets)

$ grep -n "Bash 4.0" ~/Devel/prj/Utilz/AGENTS.md
13:- Bash 4.0+, POSIX-compliant shell
```

The whole Utilz framework was verified running under `/bin/bash 3.2.57` with `set -euo pipefail` on 2026-07-29 (ST0009): dispatcher, `doctor`, `list`, `generate`, and the library seams. 3.2 is not aspirational there, it is tested.

Regenerating does not help, because the string is not derived from project state:

```
$ cd ~/Devel/prj/Utilz && intent agents sync
ok: AGENTS.md updated at project root.
$ grep -n "Bash 4.0" AGENTS.md
13:- Bash 4.0+, POSIX-compliant shell
```

## Root Cause

`intent/plugins/agents/bin/intent_agents`, the Prerequisites block (~lines 310-323):

```bash
  if [ "$has_mix_exs" = true ]; then
    echo "- Elixir / Erlang / OTP (see \`mix.exs\` for version)"
  fi
  if [ "$has_cargo_toml" = true ]; then
    echo "- Rust toolchain (see \`Cargo.toml\` for edition)"
  fi
  if [ "$has_package_json" = true ]; then
    echo "- Node.js + npm/yarn"
  fi
  if [ "$has_bats" = true ]; then
    echo "- Bats testing framework"
  fi
  echo "- Bash 4.0+, POSIX-compliant shell"
```

Two distinct problems, both visible in that one excerpt:

1. **The bash line is the only ungated `echo` in the block.** Four siblings immediately above it are detection-gated; the pattern is right there and this line breaks it. Every project gets the claim whether or not it contains a line of shell.

2. **The floor is a hardcoded constant, so it cannot be right for every project.** Even where a project genuinely is a shell project, the generator has no way to know its target. `4.0+` is a particularly unfortunate default: macOS has shipped 3.2.57 as `/bin/bash` since 2007 (a GPLv3 licensing decision, not an oversight), so on the platform these projects are developed on, the stated prerequisite is not met by the system shell.

A related structural point, offered as context rather than as part of the defect: detection here is entirely filesystem probes (`has_mix_exs`, `has_cargo_toml`, `has_package_json`, `has_bats`). `intent_agents` never reads the `languages` array from `intent/.config/config.json`, even though Intent elsewhere treats that array as the authoritative declaration of languages-in-use -- the `/in-session` skill states it explicitly: _"Languages-in-use is a configuration decision, not filesystem detection."_ Two mechanisms answer the same question, which is a Highlander smell independent of this bug. All three projects surveyed happen to declare `shell`, so this issue is not blocked on it, but a fix that gates on `languages` would resolve both at once.

## Impact

`AGENTS.md` is, by the generator's own description, the canonical contract every agentic CLI reads. A false prerequisite there is a false instruction to every agent working the project:

- **In shell projects it invites broken code.** An agent told the floor is 4.0+ may use namerefs (`local -n`), `${var,,}`, associative arrays, or `mapfile` -- all bash 4 features, all silent failures or syntax errors under macOS `/bin/bash`. Utilz already carries hard-won notes about exactly these constructs; `AGENTS.md` contradicts them.
- **In non-shell projects it is noise in the one file meant to be authoritative.** Lamplight and Baize list a shell version requirement in projects whose prerequisites are Elixir, Rust, and Swift toolchains.
- **It is unfixable from the consumer side.** Editing `AGENTS.md` directly is explicitly forbidden (it is generated; the next `intent agents sync` reverts it), so a downstream project that notices the error has no remedy but to raise it here. Utilz corrected the same wrong claim in ten hand-authored files on 2026-07-29 and had to leave `AGENTS.md:13` standing.

Severity is medium rather than high: nothing breaks at generation time, and an attentive reader cross-checks against `CLAUDE.md`. It is above low because the failure mode is an agent confidently writing platform-incompatible code, and because the file's whole purpose is to be trusted without cross-checking.

## Proposed Fix

Gate the line, and stop asserting a floor the generator cannot know.

Minimal, consistent with the four siblings:

```bash
  if [ "$has_shell" = true ]; then
    echo "- Bash or Zsh (see the project's own docs for the target version)"
  fi
```

Preferred, and resolving the dual-mechanism smell noted above -- gate on the declared `languages` array rather than a new filesystem probe:

```bash
  if intent_lang_declared "shell"; then
    echo "- Bash or Zsh (see the project's own docs for the target version)"
  fi
```

Either way, drop the version number. The generator has no basis for `4.0+`, and any hardcoded constant will be wrong for some project; pointing at the project's own documentation is both correct and maintenance-free. If a floor really is wanted in the generated output, it should come from a project-level declaration, not a literal in the tool.

Whichever shape is chosen, `intent agents sync` should be re-run across the consumer projects afterwards so the corrections propagate -- Utilz, Lamplight, and Baize all currently carry the bad line.

## Related

- 0005 -- same class of defect: `intent lang init` writing a path into a consumer project's `intent/llm/RULES.md` that is correct only relative to the Intent install. Both are the generator asserting something into a downstream repo that is true where the tool lives and false where it lands.
- Utilz ST0009 -- corrected the same "Bash 4.0+" claim in ten hand-authored files (`README.md`, `help/utilz.md`, eight utility READMEs, and `.github/workflows/README.md`) and verified the framework green under bash 3.2.57. `AGENTS.md:13` is the one instance that could not be fixed downstream, which is why this issue exists.

## Resolutions

FIXED + CLOSED (2026-07-29), shipped in v2.17.4, together with 0005 (its generator-into-consumer twin). The preferred fix is implemented -- gated on the declared `languages` array, with no version floor.

**Both problems closed.** The line is now gated like its four siblings, and it no longer states a floor:

```bash
  if [ "$has_shell" = true ]; then
    echo "- Bash or Zsh (see the project's own docs for the target version)"
  fi
```

`has_shell` is set from `has_project_language shell "$PROJECT_ROOT/intent/.config/config.json"`, hoisted next to the other detection flags so the emit block stays uniform. Pointing at the project's own documentation is both correct and maintenance-free: a generator cannot know a downstream project's target, and any hardcoded constant will be wrong somewhere.

**`has_project_language` is new in `bin/intent_helpers`** (registered in MODULES.md first) and is THE declared-language predicate. Three sites were hand-rolling the same jq read, so it earns a helper on Highlander grounds independently of this issue. Absent config, absent array and absent jq all answer false -- the safe direction: a generator that cannot prove a language is in use should say nothing about it rather than assert a default.

**One thing the fix broke, and closed.** With every prerequisite line now conditional, the section can come out empty -- which the unconditional bash line used to mask. A bare `### Prerequisites` heading reads as "none needed" when it means "nothing declared or detected", so the block now says `- None detected. Declare the project's languages with 'intent lang init <lang>'.` when nothing fires.

**Structural point deferred, not dropped.** The Highlander smell noted in the report -- `intent_agents` answering "what languages?" by filesystem probe while the rest of Intent treats the `languages` array as authoritative -- is real and is filed as its own issue rather than folded in here. Migrating all four probes changes every consumer's generated `AGENTS.md` and deserves its own decision; this issue gets the bash line only, which is what it asked for.

**Consumer sweep.** Intent's own `AGENTS.md` is regenerated in this commit and now reads `- Bash or Zsh (...)`. Utilz, Lamplight and Baize still carry the bad line and need `intent agents sync` re-run after they take v2.17.4 -- separate repos, hv's. Lamplight and Baize will lose the line entirely unless they declare `shell`, which is the correct outcome: neither is a shell project.

**Guards.** Two tests added to `tests/unit/intent_agents.bats` (no shell declared -> no prerequisite; declared -> the line appears, and never with a version floor; empty block says so) and three to `tests/unit/helpers.bats` for the predicate, including whole-entry matching so `she` and `shellscript` do not match `shell`. Full unit suite (1105 tests) and both integration files green.
