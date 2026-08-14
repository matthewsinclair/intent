---
id: "0025"
title: an inherited PROJECT_ROOT decides project-vs-global without any resolution, so intent writes into whatever tree a parent process names
date: 2026-08-14
reporter: matts
status: CLOSED
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

**Fixed properly, at the class rather than the instances.**

The defect is that `PROJECT_ROOT` was read as an answer. The fix makes resolution the only authority and gives it a name:

- **`resolve_project_root` (`bin/intent_helpers`) is THE project-root authority.** It ASSIGNS `PROJECT_ROOT` from the filesystem, overwriting anything inherited. Registered in MODULES.md as the seam every reader must come through.
- **`require_project_root` resolves, then refuses.** It used to return success for any non-empty value, which made it a guard that asked "did someone set a variable?" while its name promised "am I in a project?". Its message also moves to the lowercase 0023 voice, a site that ruling did not reach.
- **The three plugin bins that never resolved now do**, at load: `intent_claude_subagents`, `intent_claude_prime`, `intent_claude_upgrade`. This matters because `bin/intent` execs plugin commands BEFORE it loads config ("let the plugin handle it", `bin/intent:187`), so anything they saw came from outside Intent entirely.
- **`intent_agents`** had the inverse shape -- five guards where an inherited value SUPPRESSED the config load -- fixed in the preceding commit.
- **`bin/intent` clears an inherited `PROJECT_ROOT` at entry.** One line, and it makes the default fail SAFE rather than fail dangerous: a future reader that forgets to resolve now sees empty, which routes to the `$HOME` branch or an honest refusal, instead of silently naming a stranger's tree.

**Guard**: `tests/unit/ambient_project_root_guard.bats`, 4 tests. The decoy is a REAL, valid Intent project -- a decoy that could never have been selected would certify nothing, because it is exactly the tree that receives the write when the ambient value is honoured.

**Mutation matrix, including the mutation that killed nothing** -- reported because a mutation failing to produce an expected red is itself a finding:

| Mutation                                          | Red        | Reading |
| ------------------------------------------------- | ---------- | ------- |
| M1: dispatcher `unset` removed                    | **none**   | the per-reader `resolve_project_root` covers every path the guard exercises |
| M2: `resolve_project_root` trusts inherited value | 3, 4       | the unit assertions; 1 and 2 survive because the scrub left nothing to trust |
| M3: BOTH removed                                  | 1, 3, 4    | the original defect reproduces -- the manifest lands in the decoy |

Restored: 4/4 green.

**What M1 actually tells us, stated rather than glossed:** the dispatcher scrub is redundant against every reader that exists today, because each of them also resolves. It is kept deliberately as the fail-safe default for readers nobody has written yet, and this table is the record that no current test can fail on it. Test 2 survives even M3, held by a third mechanism that predates this change -- `load_intent_config` (`bin/intent_config:33`) assigns unconditionally, which is why the regular project-command path was never exposed and only the early-exec plugin path was.

**Not deferred this time.** The earlier note said shapes 1 and 3 were left for their own pass; this is that pass.
