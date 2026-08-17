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

### The sixth condition, and this sweep was structurally blind to it

**`intent info` EXITS 0 WHILE PRINTING AN ERROR.** With the install unresolvable it prints `INTENT_HOME: <not set>`, states the reason on stderr, and returns `Ok(())`. Found by dc and owned by cc, 2026-08-17, after this issue was filed.

**dc's framing is the one to keep: this issue is `1` meaning five things; that is `0` meaning "I could not do the thing you asked".** It is the same defect with the sign flipped, and it is the worse half -- a wrong non-zero code makes a caller stop for the wrong reason, and a zero code on failure makes it not stop at all.

**The reason it is recorded here rather than filed separately is that MY SWEEP COULD NOT HAVE FOUND IT.** The Reproduction table above classifies conditions **by their exit code**, so a failure that returns `0` lands in the success row by construction. **I asked "what code does each failure produce" and never "does any failure produce success"** -- and the instrument answered the question I asked, completely and uselessly, for the case that matters most. Same shape as the population-versus-trigger error elsewhere in this thread: **a measurement is bounded by the question, and a clean result reports on the question rather than on the tool.**

**So the honest count is that this issue documents five conditions collapsed onto `1` plus at least one collapsed onto `0`, and the second set has never been swept.** The sweep that would find it runs the other way round: **take every invocation that PRINTS an error and check its code, rather than taking every code and asking what produced it.** Nobody has run that.

**Per the ordering above, the code CHOICE for `info` is not bundled into this issue either** (cc's own position). The distinction cc names is the load-bearing one and is worth quoting, because it is the reason this is not simply "make it exit non-zero": **"never gate on PROJECT state" is 0042's requirement and is not the same as "always exit 0" -- an unmigrated project is not a failure of `info`; an unresolvable install is.**

### THE REVERSE SWEEP, RUN 2026-08-17 (vc)

The sweep owed at line 94 has been run. **309 probes, the whole declared surface** (104 entries from `surface/dispatch-table.json`, three probes each -- bare, `--help`, and one bogus positional; `claude start` excluded because it launches a real session). Every probe against a **fresh copy** of the fixture, so a mutating probe cannot contaminate the next. Binary rebuilt from a clean tree at `3088c39c`, invoked by explicit path, never on PATH.

Classification is by **output**, then the code is read. Two mismatch classes were defined in advance:

| class      | shape                              | found |
| ---------- | ---------------------------------- | ----- |
| **TYPE-A** | printed a failure, returned 0      | **0** |
| **TYPE-B** | returned non-zero, printed nothing | **0** |

**Both are empty, and the empty result is worth as much as a hit would have been**, because it bounds where the exit-0-on-failure class lives: dc's `info` case does not reproduce against a correctly-installed binary. It needs the unresolvable-install context, which is dc's WP-11 hold and not a property of the command surface.

**What the sweep DID establish is the overload, read from the output side rather than the code side, and it is eight events rather than five:**

| event                                                 | probes | example                                                           |
| ----------------------------------------------------- | ------ | ----------------------------------------------------------------- |
| missing required argument                             | 60     | `error: the following required arguments were not provided: <ID>` |
| unrecognised subcommand (incl. every retired command) | 34     | `error: unrecognized subcommand 'treeindex'`                      |
| unexpected argument                                   | 23     | `error: unexpected argument 'NOSUCHTHING' found`                  |
| **the project cannot be read at all**                 | **22** | `error: this project has not been migrated to Intent v3 -- ...`   |
| family root invoked with no subcommand                | 8      | `error: 'ext' requires a subcommand but one was not provided`     |
| a lookup that found nothing                           | 6      | `error: 'NOSUCHTHING' is not a work package`                      |
| a deliberate domain refusal                           | 3      | `error: 'todo done' needs something to do`                        |
| **findings**, with no `error:` line at all            | 1      | `doctor` -- 1,175 bytes of report, rc 1                           |

**Every row is exit 1**, and the first seven sum to the 156 anchored rc-1 probes exactly. The eighth is the one no lexical classifier can reach: `doctor` returns 1 for a **result**, not a failure, and prints no `error:` line at all -- so a caller cannot separate it from the other seven by code _or_ by output shape. The third row is the one that mattered and it has gone to **issue 0045**: it is a statement about the world rather than about the invocation, the tool already owns `2` for exactly that meaning, and the pre-commit gate reads `1` as _block this commit_.

**Two smaller results, both checks on other people's work rather than new defects.**

- **`intent info NOSUCHTHING` returns 0 with byte-identical output to `intent info`** -- the positional is swallowed. This is INV-08, already registered, **independently reproduced by an instrument that knew nothing about it**, which is a pass for the register rather than a finding against it. **v3 has half-fixed it**: `intent info --zzz` now exits 1, while the positional still swallows. The register's `args` note still reads "every argument is silently discarded", which described v2 and is now true of only half of v3. Raised with ic as register drift, not filed.
- **32 commands now exit 2** (`agents`, `llm`, `init`, `upgrade`, `learn`, `fileindex`, `config`, `issues`, `lang`, `modules`, `plugin`, `ext`, `bootstrap`, `version` and their subcommands). 0038's blast-radius table re-measured at HEAD: the six "known command not implemented yet" rows have all moved to 2. **The four that stayed at 1 are exactly the retired ones** -- `organize`, `treeindex`, `help`, `st_zero` -- which is Proposed Fix 1 below, now measured rather than argued.

**And this sweep has its own blind spot, found the same way dc found the last one -- by asking what the instrument cannot represent.** It classifies by _printed error_, so it is structurally blind to **a failure that prints nothing at all**. `intent info NOSUCHTHING` surfaced only because the harness ran a **differential** -- the same command with and without a bogus argument -- and compared the two outputs to each other rather than either to a rule. **A single reading can only find defects the signal it reads represents; the differential is what catches the case where every signal is silent.** The forward sweep read the code and was blind to failure-at-0. The reverse sweep reads the message and is blind to failure-with-no-message. Neither is fixable by widening the lexicon.

**Coverage caveat, stated because a clean sweep invites the wrong conclusion: 22 of the 309 probes never reached the command.** They were refused at the migration gate, so what was measured is the gate, not the command behind it. **That slice cannot be measured in this build at all** -- `init` and `upgrade` are both unimplemented, so no v3 project can be brought into existence to run them in. **The reachable surface was swept; the surface behind the gate has never been swept by anyone**, and it becomes reachable the moment WP-10 lands.

### THE DIFFERENTIAL SWEEP, AND THE INSTRUMENT DEFECT THAT ALMOST PUBLISHED AN EMPTY CLASS

The blind spot above was then closed on its own terms. **103 commands, three runs each -- bare, unknown flag (`--zzz`), unknown positional (`NOSUCHTHING`) -- and the two extra runs are compared TO THE BARE RUN rather than to any rule.** Identical output plus exit 0 means the argument was swallowed. No lexicon is involved, so silence is detectable.

Built against a `git archive` extract at `0566985b`, compiled inside the extract so the binary resolves its own install tree. **Result:**

| arm                  | swallowed      | of  |
| -------------------- | -------------- | --- |
| unknown flag `--zzz` | **0**          | 103 |
| unknown positional   | **1** (`info`) | 103 |

**So INV-08's class is exactly one command wide and it is the positional half.** The flag half is refused everywhere -- `intent info --zzz` now exits 1 -- and nobody needs to go looking further.

**The first run of this sweep reported 0 and 0, and it was wrong.** Each probe gets its own copy of the fixture so a mutating probe cannot contaminate the next -- and **`intent info` prints `Location: <cwd>`**, so two runs of the _same_ command in different copies produce different bytes. Hashing raw output therefore reported every command as "refused" regardless of behaviour. **A differential that can never find a member reads exactly like a differential that found none: `flag refused: 103, pos refused: 103` is what a perfectly healthy surface looks like.** It was caught by a direct probe of `intent info NOSUCHTHING` run for an unrelated reason two commands later, not by anything in the sweep. Fixed by normalising the run directory out before hashing.

**Two controls now sit under the result, because the first version of it taught what an uncontrolled clean sweep is worth.**

- **A positive control on the pre-fix build** (`3088c39c`, extracted and compiled separately): same instrument, and it finds the `info` positional there too.
- **A synthetic positive control**: a stub binary that swallows every argument and always exits 0. **Both arms read SWALLOWED against it.** This one matters more than it looks -- **the flag arm has never fired against any real build, so without the stub it is an arm that has never been shown to work, and 0-of-103 from an untested classifier is not a measurement.**

**The generalisable rule, which is the reusable part: when the failure mode is silence, no classifier over the output can help -- you need two runs that OUGHT to differ, and a run in which the classifier is known to fire.**

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

**PARTIAL (dc). The reported instance and four more are fixed; the tool-side defect this issue is actually about is untouched and it stays OPEN.**

### THE CALLER ENUMERATION, DONE ONCE

0043's Proposed Fix item 3 asks for the consumers of `intent`'s exit codes to be listed and written down. Nobody had, which is why 0038, 0042, 0043 and 0044 were each diagnosed against whichever consumer happened to be in view. **The count has been revised upward four times -- one, then four, then "at least six", and it is ELEVEN.** Swept at `7e877520`; shipped consumers only, since tests measure the tool rather than depend on it.

| #   | consumer                              | invocation                              | resolves via  | status read | on non-zero                                 |
| --- | ------------------------------------- | --------------------------------------- | ------------- | ----------- | ------------------------------------------- |
| 1   | Claude Code `SessionStart`            | `intent claude hook session-context`    | PATH          | the harness | advisory -- session runs, context gone      |
| 2   | Claude Code `UserPromptSubmit`        | `intent claude hook require-in-session` | PATH          | the harness | **2 = BLOCK the prompt** (0043)             |
| 3   | canon pre-commit -- critic            | `intent critic <lang> --staged`         | PATH          | captured    | 2 = fail-open (0038)                        |
| 4   | canon pre-commit -- whiteboard guards | `intent info`                           | PATH          | captured    | fail-open, now reported as total (0042)     |
| 5   | devbin `docs agents`                  | `intent agents sync`                    | PATH          | explicit    | warn, `rc=1` -- **fixed `68282648`**        |
| 6   | devbin `docs treeindex`               | `intent treeindex <dir>`                | PATH          | explicit    | warn, `rc=1` -- **fixed `a18010a8`**        |
| 7   | devbin `check critic` -- the list     | `intent critic --languages`             | PATH          | explicit    | die naming the TOOL -- **fixed `68282648`** |
| 8   | devbin `check critic` -- the run      | `intent critic <lang>`                  | PATH          | `\|\| rc=1` | `rc=1` -- already correct                   |
| 9   | `int build release` pre-flight        | `bin/intent doctor`                     | explicit path | `if !`      | abort -- already correct                    |
| 10  | `int build release` sidecar           | `bin/intent agents sync`                | explicit path | `if !`      | abort -- **fixed `7e877520`**               |
| 11  | `int build release` sidecar           | `bin/intent claude upgrade --apply`     | explicit path | `if !`      | abort -- **fixed `7e877520`**               |

**Two entries that are not consumers today and are worth knowing about anyway.** `post-tool-advisory.sh` calls `intent critic ... 2>/dev/null || true` and discards everything -- but it is **not wired in either shipped `settings.json`**, so it is latent, and under v3 it would simply never fire. And Claude Code's `Stop` hook is a bare `echo`: **safe by accident of its wiring, not by design**, since `Stop` at exit 2 means "do not stop" (24s and zero output, measured on vc's rig). Routing it through `intent claude hook` for consistency is the obvious tidying move and arms a refuse-to-stop loop.

**THE STRUCTURAL POINT, which is why the table beats the number: EIGHT OF ELEVEN RESOLVE `intent` FROM `PATH`.** The three that do not are all in `int build release`, and they use an explicit path for an unrelated reason -- releasing a named checkout rather than whatever is installed. **So PATH resolution is the default everywhere it was not deliberately avoided, and that is exactly why 0036's shadowing is a machine-wide event rather than a per-project one.** Every one of the eight changes behaviour the moment a different `intent` is first on `PATH`, and not one of them can tell that has happened.

### What was fixed

- **`docs treeindex` (`a18010a8`)** -- the reported instance. Status discarded; `rc` moved only in the absent-directory branch. Checked now, each failing directory named.
- **`docs agents` (`68282648`)** -- the function's last command, so it propagated **by accident of position**; one appended line would have swallowed it. Made explicit.
- **`check critic --languages` (`68282648`)** -- `|| true` discarded the status, so a CLI that could not answer produced an empty list and a `die` naming the wrong cause: the project's language declarations, blamed for a failure of the tool. It fails CLOSED, so nothing was skipped silently; the cost was a wrong diagnosis at the moment it is most expensive.
- **Both `int build release` sidecar regenerators (`7e877520`)** -- the reason the sweep was worth doing beyond the report. They fired in a bare subshell, discarded the status, and logged success unconditionally on the next line. **The dirty-tree check cannot cover it: that guard watches for unexpected CHANGE, and a failed regeneration produces unexpected SAMENESS** -- the file keeps the previous version, the tree stays clean for it, and the cut tags a release inconsistent with itself. All fixed sites are mutation-proven in both directions.

### What is NOT fixed

**Everything this issue is actually about.** The tool still spells five unrelated conditions with `1`, and the structural cause stands: **the exit code is decided by WHERE the failure happens in the parse tree, not by WHAT went wrong**, so a retired command never reaches the code that would choose a meaningful one. Every fix above hardens a _caller_ against a tool that cannot say what it means. **That is worth doing and it is not the fix** -- eleven callers each defending themselves is the shape you get when the contract is missing, and the twelfth will be written by someone who has not read this table.
