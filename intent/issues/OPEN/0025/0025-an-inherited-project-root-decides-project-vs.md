---
id: "0025"
title: an inherited PROJECT_ROOT decides project-vs-global without any resolution, so intent writes into whatever tree a parent process names
date: 2026-08-14
reporter: matts
status: OPEN
severity: high
---

# 0025: an inherited PROJECT_ROOT decides project-vs-global without any resolution, so intent writes into whatever tree a parent process names

## Tags

plugins, project-resolution, environment, silent-wrong-target, devbin

## Summary

Several code paths treat the presence of a `PROJECT_ROOT` environment variable as proof that the caller is inside an Intent project, and branch on it without ever resolving anything. `PROJECT_ROOT` is a generic name that any parent process can export — a Makefile, direnv, CI, or devbin, which exports it on **every** invocation. When one does, Intent takes the in-a-project branch and operates on the tree that variable names, from any working directory, with no error and no mention.

This is the 0024 family: an instrument that accepts an ambient input and then answers confidently about the wrong subject. Found while adopting devbin, which made it fire on every command rather than occasionally.

## Reproduction

`intent claude subagents install` chooses where the manifest lives:

```bash
# intent/plugins/claude/bin/intent_claude_subagents:24
plugin_get_manifest_path() {
  if [ -n "${PROJECT_ROOT:-}" ]; then
    echo "$PROJECT_ROOT/intent/plugins/claude/subagents/.manifest/installed-agents.json"
  else
    echo "$HOME/.intent/agents/installed-agents.json"
  fi
}
```

Measured against the suite, same tree, same file, three invocations:

```
./tests/run_tests.sh tests/unit/agent_commands.bats                  ->  0 failing
env PROJECT_ROOT=<any dir> ./tests/run_tests.sh tests/unit/...       -> 13 failing
bin/int test shell tests/unit/agent_commands.bats                    -> 13 failing
```

The 13 failures are the suite asserting against `$HOME/.intent/agents/installed-agents.json` while the code wrote into the tree named by the ambient variable. Isolated by injecting each variable devbin exports, one at a time: `INTENT_ROOT`, `BIN_DIR` and `DEVBIN_LIB` all leave the suite green; only `PROJECT_ROOT` breaks it.

## Root Cause

Two distinct shapes, both trusting the environment where they should resolve:

1. **Branch on presence** — `intent_claude_subagents:24` (and `:142`, `:383`) read `${PROJECT_ROOT:-}` to decide project-local versus `$HOME`. Nothing calls `find_project_root`.
2. **Presence suppresses resolution** — `intent/plugins/agents/bin/intent_agents` guarded its config load with `[ -z "${PROJECT_ROOT:-}" ] && ...`, so an inherited value meant `load_intent_config` never ran and every path below used the ambient tree. Five occurrences. **Fixed in this change** by resolving unconditionally; `load_intent_config` assigns from `find_project_root` and is idempotent, so it cannot be fooled from outside.
3. **`require_project_root`** (`bin/intent_helpers:604`) returns success on any non-empty `PROJECT_ROOT`, so as a guard it asks "did anyone set a variable?" rather than "am I in a project?".

`bin/intent` itself is safe: `load_intent_config` (`bin/intent_config:33`) assigns `PROJECT_ROOT=$(find_project_root || true)` unconditionally, overwriting anything inherited. The exposure is in plugin bins that read the variable without going through that.

## Impact

A user whose shell or tooling exports `PROJECT_ROOT` — which devbin now does on every `bin/int` invocation — gets subagent manifests written into that tree instead of `$HOME`, from any directory, silently. The failure is invisible at the time and shows up later as a manifest that is missing, or one that belongs to a project the user was not working in.

Shape 2 is the worse of the three, because it does not merely pick a branch: it **suppresses resolution entirely**, so no amount of being in the right directory corrects it.

## Proposed Fix

- Shape 1: resolve rather than read. `plugin_get_manifest_path` and its siblings should call `find_project_root` and branch on the result, not on the variable.
- Shape 3: `require_project_root` should resolve, or be renamed to say what it actually checks.
- Consider whether Intent should defensively clear an inherited `PROJECT_ROOT` at the top of `bin/intent` rather than relying on every path to overwrite it. That is one line and removes the whole class.

Guard: inject `PROJECT_ROOT=/some/other/tree` and assert the manifest still lands in `$HOME`. The suite could not have caught this before, because it inherited whatever the runner was given — which is why the runner now scrubs it (see below).

## Related

- 0024 — the same family: an instrument that accepts a narrowing or ambient input and answers the wider or wrong question, reading exactly like a correct answer.
- ST0056 — v3 resolves the project once in `intentsvcs` rather than in each surface, which makes this class unconstructible rather than guarded.

## Resolutions

**Landed with this change:**

- `intent/plugins/agents/bin/intent_agents` — the five `[ -z "${PROJECT_ROOT:-}" ] &&` guards removed; config resolution is now unconditional.
- `tests/run_tests.sh` — `unset PROJECT_ROOT INTENT_ROOT BIN_DIR` before the script computes its own, and its own assignment is deliberately not exported, so child `intent` processes resolve the project themselves. A suite that inherits ambient project state measures the machine it happens to run on. This is why `bin/int test all` was red on adoption and is green now.

**Deliberately NOT fixed here**, because they are the wider surface and want their own pass rather than being rushed alongside a tooling change: shapes 1 and 3 above — `intent_claude_subagents`'s three reads, and `require_project_root`'s definition. Both are recorded with their file and line. The suite no longer inherits the variable, so nothing masks them; they simply are not yet fixed.

{{TBC}}
