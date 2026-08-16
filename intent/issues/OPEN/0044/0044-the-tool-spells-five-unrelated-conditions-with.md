---
id: "0044"
title: the tool spells five unrelated conditions with exit 1 -- unimplemented is 2 but retired, malformed and genuine runtime refusal are all 1, so no caller can tell a missing command from a real failure
date: 2026-08-16
reporter: matts
status: OPEN
severity: high
---

# 0044: the tool spells five unrelated conditions with exit 1 -- unimplemented is 2 but retired, malformed and genuine runtime refusal are all 1, so no caller can tell a missing command from a real failure

## Tags

exit-codes, parity, migration, devbin, fail-open, no-silent-errors, measured

## Summary

**0043 is about four CALLERS disagreeing over what `2` means. This is the mirror: the TOOL uses `1` for five unrelated conditions, and one of them is "your code has findings".**

`d2b8e76d` gave unimplemented commands `EXIT_UNAVAILABLE = 2`, and that is honoured consistently -- **13 of 30 top-level commands are unimplemented and all 13 exit `2`.** The defect is everything else. **A command that was RETIRED, a command invoked with a missing subcommand, a command invoked with a missing argument, and a command that ran and legitimately refused all exit `1`, and so does a critic run that found real problems.**

**So a caller can reliably detect "not built yet" and cannot distinguish "that command no longer exists" from "you are not in a project" from "your code has findings".** The three are the same number.

**The exit code is determined by WHERE the failure happens in the parse/dispatch tree, not by WHAT went wrong.** An unimplemented command is caught after dispatch, so it gets the deliberate code; a retired one never reaches dispatch at all, because it is absent from the clap surface, so it gets clap's generic `1`.

Found by vc, 2026-08-16, sweeping the tool side after enumerating the caller side for 0043.

## Reproduction

Measured against the v3 debug binary built from pinned SHA `0ef6e0a1`, each command run bare in its own fresh empty directory with stdin from `/dev/null`. `bootstrap`, `daemon` and `mcp` were deliberately excluded: the first writes global configuration and the other two are servers.

| condition                                    | example                                 | exit  |
| -------------------------------------------- | --------------------------------------- | ----- |
| **unimplemented command**                    | `intent info`, `intent version`         | **2** |
| **unimplemented subcommand, parent exists**  | `intent claude hook require-in-session` | **2** |
| **retired command, absent from the surface** | `intent treeindex`, `intent organize`   | **1** |
| **implemented, missing required subcommand** | `intent st`                             | **1** |
| **implemented, missing required argument**   | `intent search`                         | **1** |
| **implemented, genuine runtime refusal**     | `intent st list` outside a project      | **1** |
| success                                      | `intent schema`                         | 0     |

The 13 unimplemented commands, all exiting `2`: `issues`, `info`, `init`, `upgrade`, `agents`, `lang`, `llm`, `learn`, `modules`, `plugin`, `ext`, `fileindex`, `version`.

**`intent critic shell --staged` exits `2`**, so `d2b8e76d` is confirmed correct for the pre-commit gate's actual invocation -- this issue takes nothing away from that fix.

**`intent version` exiting `2` deserves its own line.** It is the most script-callable command in the tool, it takes no arguments, it cannot fail for environmental reasons, and under v3 it reports "unavailable" -- so any caller using it as a liveness or version probe gets a failure code from a build that is running fine.

## Root Cause

**`EXIT_UNAVAILABLE` was introduced as a deliberate signal and the rest of the exit surface was left as clap's default.** `spine.rs` defines three codes; `2` was given a precise meaning and a precise trigger, and `1` remains whatever falls through -- both the codes clap returns for a parse failure and the code a real refusal returns.

That is defensible for a tool nobody scripts and wrong for this one, because **the two conditions collapsed into `1` are on opposite sides of a safety decision.** "This command does not exist in this build" is a **migration** fact and the caller should stop. "Your code has findings" is a **result** and the caller should block the commit. "You are not in a project" is a **usage** fact. One number, three answers.

**The retired case is the one nothing could have caught**, and it is worth naming precisely: a retired command is not routed to the not-implemented arm, because retirement removes it from the clap surface entirely. **The refusal happens before the code that would have chosen a meaningful exit code ever runs.** So the careful work in `d2b8e76d` is structurally unreachable for exactly the class of command a migration is most likely to hit.

## Impact

**Measured, live, and in the consumer dc named as unenumerated: `bin/.devbin/lib/cmd/docs` reports success having indexed nothing.**

`builtin_docs_treeindex` calls `intent treeindex "$d"` at `:58` and **does not check its exit code** -- the loop's `rc` is moved only by the missing-directory branch above it. Under v3, `treeindex` is retired, so each call exits `1`, the loop ignores it, and the gate returns `0`.

Measured by running the real `bin/int docs treeindex lib bin` against a stub binary that reproduces v3's retired-command behaviour. **A stub was used deliberately rather than the real v3 build, because 0043 forbids putting v3 on a PATH:**

```
==> docs treeindex
    lib
error: unrecognized subcommand 'treeindex'
    bin
error: unrecognized subcommand 'treeindex'
verdict: .../20260816-2111.DOCSTREE.errors
rc=0
```

**The errors are printed, so this is not silent** -- the same correction dc rightly made about 0042's guards. **What it is, is green.** The gate returns `0` and any pipeline chaining on it proceeds. **Two directories failed to index and the gate's exit status says nothing happened wrong.**

**One narrowing, on dc's correction and confirmed against the artefacts on disk: I first wrote that the verdict file is EMPTY, and that is not reliably true.** Across four runs of the same command the `.DOCSTREE.errors` artefact came back 0 bytes twice and 86 bytes twice (both failures captured). **So the artefact varies and the exit code does not.** `rc=0` is the defect and is constant; the empty artefact was one run of mine generalised into a property. Recorded because an over-claimed secondary symptom is how a real finding gets argued away.

- **`IN-AG-NO-SILENT-001`** in the form that matters most: the failure is on screen and absent from the result.
- **It is a fail-OPEN**, so it sits behind the loud failures exactly as 0042 sits behind 0038. The loud ones get fixed first and this waits.
- **The blast radius is every script that calls `intent` and checks `$?`**, which is the class dc flagged and nobody has enumerated. `docs treeindex` is one instance found by looking; it is not evidence that it is the only one.

**Not claimed: that this blocks anything today.** Nothing here refuses work that should proceed. **The cost is entirely in the other direction** -- work that did not happen, reported as done.

**Not claimed: that `d2b8e76d` should be reverted or narrowed.** It is correct and this issue depends on it being correct: `2` is reliable precisely because that fix made it so.

## Proposed Fix

**The ordering claim first: this is NOT a blocker for 0043 and should not be bundled into it.** 0043 is a lockout and must be settled before publication. This can follow, and doing it under the same pressure risks a second constant chosen against a single consumer -- which is how the first three of these arrived.

1. **Give the retired class its own answer.** A retired command reaching clap's unknown-subcommand path is the migration case with the least helpful message in the tool: `unrecognized subcommand 'treeindex'` tells a v2 user nothing about what happened to their command. **Recognise the retired names and refuse them by name, with the disposition and the replacement** -- the parity register already holds exactly that mapping, so the data exists and nothing reads it here.
2. **Separate "the tool could not run" from "the tool ran and says no".** Whatever the codes end up being, a caller needs to distinguish a usage or availability failure from a real result. The critic loop is the case that proves it: `1` currently means both "findings" and "no such command".
3. **Write the caller list down beside the constants in `spine.rs`.** 0043 says this and it is the same recommendation; the list is now longer than either issue started with. Known consumers: the pre-commit critic loop, `.claude/settings.json`'s three hook events, `int prepush` and the devbin gates. **Four issues -- 0038, 0042, 0043 and this -- have each been diagnosed against whichever consumer happened to be in view.**

**The canary is cheap and it is one line: assert that `intent <a-retired-command>` does not exit with the same code as a critic run that found findings.** No fixture and no project needed.

## Related

- 0043 -- the mirror image, and the reason this sweep happened: four callers assign four meanings to `2`. **Together they are one defect seen from both ends -- the tool overloads `1`, the callers overload `2`, and neither side has a written contract**
- 0038 -- introduced `EXIT_UNAVAILABLE`; correct, and this issue rests on it being correct
- 0042 -- the same fail-open shape in the pre-commit hook, and the same reason it will be found late
- 0036 -- `brew install` shadowing the v2 symlinks, which is the delivery mechanism for every one of these
- `IN-AG-NO-SILENT-001` -- a gate that returns success over work that did not happen

## Resolutions

{{TBC}}
