---
id: "0043"
title: 0038's fix set unimplemented commands to exit 2, which is the UserPromptSubmit BLOCK code, so ANY project carrying the canon hooks blocks every Claude Code prompt the moment v3 is on PATH -- migration is not the trigger -- and cannot run the command that would clear it
date: 2026-08-16
reporter: matts
status: OPEN
severity: critical
---

# 0043: 0038's fix set unimplemented commands to exit 2, which is the UserPromptSubmit BLOCK code, so ANY project carrying the canon hooks blocks every Claude Code prompt the moment v3 is on PATH -- migration is not the trigger -- and cannot run the command that would clear it

## CORRECTION, 2026-08-16: THE TRIGGER IS PATH, NOT MIGRATION

**This issue was filed and confirmed under a precondition that does not hold, and the correction widens it. Found by dc; the evidence was already in my own confirmation and I read past it.**

**`claude` is unimplemented as a FAMILY, so v3 refuses before it looks at project state at all.** Measured: `intent claude hook require-in-session` exits `2` in a **migrated** project, in an **unmigrated v2** project, and **outside any Intent project whatsoever**.

**The confirmation above already proved this and did not notice.** ARMV3 -- the end-to-end arm, the real binary wired the real way -- ran in `$CLAUDE_JOB_DIR/tmp/hookprobe/armv3`, **a throwaway directory with no `intent/.config/config.json` on any ancestor.** It blocked. So the decisive arm was executed with the "migrated project" precondition absent, and was reported as end-to-end confirmation of a claim about migrated projects. **The result was right and the framing around it was wrong**, which is the harder version of this to catch: nothing failed, and the reasoning about scope was never tested by anything.

**What changes:**

- **The blast radius is every Intent project on the machine carrying the canon hooks, migrated or not.** The session hooks are canon, not opt-in, so that is the fleet.
- **The condition to hold is PUBLICATION, not migration.** cc's standing "do not migrate this repo until 0043 is settled" remains right and **is not sufficient** -- migration was never the door.
- **`brew install` is the trigger**, and it puts v3 at PATH position 1 without asking.
- **OPERATIONAL, and it applies to every node working this thread right now: every session in this estate is alive only because `intent` on PATH still resolves to v2.** The moment v3 lands on the PATH of a shell a Claude Code session runs in, that session stops accepting prompts and cannot be recovered from inside itself. **Do not put v3 on PATH.**

Landed by dc as the second hard publication hold in `install.md` (`ad46d014`), beside 0036.

## Tags

migration, hooks, exit-codes, claude-code, lockout, regression, measured, critical

## Summary

**Two consumers read the same exit code and take opposite decisions from it.** (Filed as two. **Measured, it is four** -- see the Root Cause table. These two are the pair that makes it critical.)

- The **pre-commit gate** reads `2` as _"the critic tooling is unavailable"_ and **fails open** -- correct, and the reason 0038 was fixed by moving unimplemented commands from `1` to `2` (`d2b8e76d`, `EXIT_UNAVAILABLE`).
- The **`UserPromptSubmit` hook** reads `2` as _"BLOCK this prompt"_. That is Claude Code's contract and the shipped `require-in-session.sh` uses it deliberately: `:20` documents _"Block (exit 2 + stderr message)"_ and `:71` is a bare `exit 2`.

`.claude/settings.json` wires `UserPromptSubmit` to `intent claude hook require-in-session`. **v3 does not implement `claude hook`, so it exits 2 -- which the hook contract reads as a block.**

**In any project carrying the canon hooks -- migrated or not -- every prompt is refused, and the refusal cannot be cleared from inside the session**, because clearing it means running `/in-session`, which means submitting a prompt. (Filed as "in a migrated project". See the correction above: **`claude` is unimplemented as a family, so v3 refuses before it reads project state, and the trigger is v3 arriving on PATH.**)

**0038's fix created this.** Before `d2b8e76d` an unimplemented command exited `1`, which `UserPromptSubmit` ignores. The fix was correct for the gate it was measured against and collides with a different consumer that spells the opposite decision with the same number.

Found by vc, 2026-08-16, tracing 0042's session-hook row to what it actually does.

## Reproduction

Measured against the debug binary built from the current tree, in a v2 fixture.

```
$ intent claude hook require-in-session
error: `claude` is a known command that is not implemented yet
  remedy: run `intent claude --help` for the verbs that are
rc=2

$ intent claude hook session-context
error: `claude` is a known command that is not implemented yet
  remedy: run `intent claude --help` for the verbs that are
rc=2
```

**The wiring** (`.claude/settings.json:14-25`): `UserPromptSubmit`, matcher `""` (every prompt), command `intent claude hook require-in-session`, timeout 2000.

**The contract** (`.claude/scripts/require-in-session.sh`):

```
:15  #   - Pass-through (exit 0) when: ...
:20  #   - Block (exit 2 + stderr message) when the sentinel is absent AND ...
:25  # cannot turn this gate hook into a hard abort. The script decides pass/block
:26  # explicitly via exit codes; an unexpected abort would block every prompt.
:71  exit 2
```

**The script's author foresaw this exact failure and defended the only half they could reach.** Line 26 -- _"an unexpected abort would block every prompt"_ -- is a guard against the SCRIPT aborting. It cannot guard against the command that INVOKES the script not existing and returning the same code by a different route.

## Root Cause

**`2` is not one meaning. Measured, it is FOUR, in four contracts, and nothing relates them.**

| consumer                       | reads `2` as         | resulting action              | status                                |
| ------------------------------ | -------------------- | ----------------------------- | ------------------------------------- |
| `pre-commit.sh` critic loop    | tooling unavailable  | **fail open, proceed**        | live, correct, the reason for 0038    |
| Claude Code `UserPromptSubmit` | deliberate refusal   | **block the prompt**          | **live, fatal -- this issue**         |
| Claude Code `SessionStart`     | advisory stderr only | **proceed, hook effect lost** | **live, silent**                      |
| Claude Code `Stop`             | do not stop          | **refuse to end the turn**    | unreached today (`Stop` is an `echo`) |

**The first two were the issue as filed. The third and fourth were found by measuring rather than reasoning**, and the fourth is the instructive one: it is not a defect today and it is one wiring change away from being one.

`spine.rs` has one `EXIT_UNAVAILABLE` for every caller, so a single constant has to satisfy all of them -- and it cannot, because the contracts disagree about what the number means. **Whichever value is chosen, some consumer is wrong: `1` breaks the commit gate (0038), `2` breaks the prompt gate (this).** That is not a tuning problem with a better number in it. **There is no value of the constant that is right for four contracts that assign it four meanings**, which is why the fix below is per-caller rather than a different global.

**This is not a mistake in `d2b8e76d`.** The fix was measured against the pre-commit gate, was right about it, and its reasoning is sound and recorded. The defect is that **the exit code was treated as a property of the tool when it is a property of the CALLER's contract**, and nothing enumerated the callers. `.claude/settings.json` and `pre-commit.sh` are the two shipped consumers of `intent`'s exit codes, and only one was in view.

## Impact

**A migrated project cannot be used with Claude Code at all.**

- **Every prompt is blocked**, on every matcher (`""` matches all), with the message `` `claude` is a known command that is not implemented yet ``.
- **The lockout is self-sealing.** The documented remedy for a stuck gate is to run `/in-session`, or to `touch` the sentinel named in the hook's error output. The first requires submitting a prompt; the second requires the hook to print the sentinel path, and it prints v3's not-implemented message instead. **Neither escape is reachable from inside the session.**
- **It affects every project that has ever run `intent claude upgrade`**, which is the whole fleet -- the session hooks are canon, not opt-in.
- **It lands precisely on hv's stated plan.** The goal is to migrate Intent itself to v3 quickly in order to dogfood it. **The dogfood is conducted through Claude Code sessions, and this closes them at the moment of migration.**
- **It is worse than 0038 in the one way that matters most: 0038 blocked commits and left the tool you would use to fix it working.** This blocks the tool.

**Not claimed: that `git commit` breaks.** It does not -- 0038's fix works, and this is the same number arriving at a different consumer.

**CONFIRMED LIVE, 2026-08-16, and nothing in this issue is now inferred.** The confirmation this section asked for has been run: five arms against Claude Code 2.1.233, each a throwaway directory with its own `--settings` wiring `UserPromptSubmit` exactly as `.claude/settings.json` wires it (matcher `""`, timeout 2000), driven headless with `claude -p`.

| arm            | hook command                                       | hook exit | Claude Code result                               |
| -------------- | -------------------------------------------------- | --------- | ------------------------------------------------ |
| **ARM0**       | script                                             | **0**     | `PONG` -- the prompt runs                        |
| **ARM1**       | script                                             | **1**     | `PONG` -- **the prompt runs**                    |
| **ARM2**       | script                                             | **2**     | **`UserPromptSubmit operation blocked by hook`** |
| **ARMV3**      | `intent claude hook require-in-session` (v3 build) | **2**     | **BLOCKED**, carrying v3's own message           |
| **ARM2-SLASH** | script, prompt `/in-session`                       | **2**     | **BLOCKED**                                      |

**ARM1 is the load-bearing arm and it was not in the original filing's plan.** Confirming that `2` blocks would have left the causal claim untested -- the issue asserts `d2b8e76d` CREATED this by moving unimplemented commands from `1` to `2`, and that requires `1` not to block. It does not. So the arms establish the cause, not merely the symptom.

**ARMV3 is the end-to-end case**, the real binary wired the real way, and it blocks with its own text:

```
UserPromptSubmit operation blocked by hook:
[.../intent claude hook require-in-session]: error: `claude` is a known command that is not implemented yet
  remedy: run `intent claude --help` for the verbs that are

Original prompt: Reply with exactly the word PONG and nothing else.
```

**ARM2-SLASH settles the self-sealing claim, which was the weakest part of the filing.** `/in-session` is the documented remedy for a stuck gate, and it is itself a prompt submission: it is blocked by the same hook. **The escape route is closed from inside the session**, measured rather than argued. The second documented escape -- `touch` the sentinel named in the hook's error output -- is visibly unavailable in ARMV3's output above: the message is v3's not-implemented text, and **no sentinel path is printed because the script that would print it never ran.**

**AND THE RIG THEN ANSWERED THE QUESTION THE PROPOSED FIX ASKS.** Item 3 below says to enumerate the consumers of `intent`'s exit codes and write them down. **Intent ships THREE Claude Code hooks, not one**, and until now only the fatal one had been measured. All three, same rig, same day:

| Intent hook        | wired command                           | under v3   | effect                                                         |
| ------------------ | --------------------------------------- | ---------- | -------------------------------------------------------------- |
| `SessionStart`     | `intent claude hook session-context`    | **rc=2**   | **does NOT block** -- the session starts, silently contextless |
| `UserPromptSubmit` | `intent claude hook require-in-session` | **rc=2**   | **BLOCKS every prompt** -- the lockout above                   |
| `Stop`             | bare `echo '...wrap-up reminder...'`    | unaffected | **does not invoke `intent` at all**                            |

**Two of the three break, in OPPOSITE directions, and the third was never at risk.**

**`SessionStart` fails OPEN and that is a finding in its own right, not a relief.** Measured with both a stub and the real v3 binary: the prompt runs, the session is usable, and `session-context.sh` never executes -- so **the project context it injects, and the `/in-session` reminder that is the documented entry to the whole gate mechanism, silently do not arrive.** Same family as 0042: the guarded operation succeeds while the control stops running. So the migrated-project experience is precisely: **the session opens with its context quietly missing, and then the first prompt is refused.**

**`Stop` is clean today only by accident of how it is wired**, and it is worth writing down before someone changes that. The rig measured `Stop` exiting `2` as well: **3s and `PONG` at exit 0, versus 24s and ZERO output at exit 2.** Claude Code reads `2` from `Stop` as _"do not stop"_. Intent's `Stop` hook is a bare `echo`, so nothing in the estate reaches it -- **but routing `Stop` through `intent claude hook`, which is the obvious tidying move, would arm a third distinct failure from the same constant.** That is the fourth meaning of `2`, on the fourth contract, and it is the one that is cheap to walk into.

**One finding the arms added that the filing did not anticipate: the `claude` process itself exits 0 on a blocked prompt.** The block is in-band, reported in the output stream. **So any wrapper or automation checking the process exit code sees success** while the model never saw the prompt -- a second silent-failure surface, in the layer that would be used to detect the first.

**The contract half was already not a single-source inference**, and the measurement now supersedes it. Intent's own canon narrative states it twice, independently of the script:

```
intent/docs/working-with-llms.md:306
  ... soften it by editing `require-in-session.sh` to exit 0 (advisory stdout)
  instead of exit 2 (blocking) when the sentinel is absent ...

intent/docs/working-with-llms.md:580
  ... soften `require-in-session.sh` to exit 0 (advisory) instead of exit 2
  (blocking) ...
```

So three in-repo sources agreed that `2` from `UserPromptSubmit` blocks -- the script's comment, the script's implementation, and the published narrative that tells users how to turn the blocking off -- and **the fourth source is now the tool itself.** Nothing in this issue rests on a reading.

## Proposed Fix

**The ordering matters more than the mechanism: this must be settled before WP-10 migrates anything, including this repository.**

1. **Implement `claude hook` in v3** -- the same conclusion 0042 reaches from the other direction, and forced by the same 0016 invariant that forbids rewiring hooks. With `claude hook` implemented, the collision is unreachable through the shipped canon.
2. **Stop letting one constant answer to two contracts.** The unimplemented-command exit should be chosen per caller contract, not globally. A command invoked as a hook has a different contract from one invoked in a gate loop, and the tool currently cannot tell them apart.
3. **Enumerate the consumers of `intent`'s exit codes and write them down.** That fact belongs in `spine.rs` beside `EXIT_UNAVAILABLE`, where the next person choosing a number will see it. A comment naming only the pre-commit gate is how this happened.

   **And the list is longer than two** (dc, 2026-08-16). **0038, 0042 and 0043 are three consumers reaching three different decisions from one exit code, each fixed against the only consumer in view** -- and dc names **a fourth nobody has: `int prepush` and the devbin gates shell out to `intent` as well.** **Worth one person listing every caller ONCE, rather than a fourth issue arriving by the same route as the first three.**

**The canary: a fixture whose `UserPromptSubmit` command is the v3 binary, asserting the hook's exit code is NOT 2.** It is a one-line assertion and it would have failed the moment `d2b8e76d` landed.

**That fixture has now been built and run** (vc, 2026-08-16 -- the five arms above). It is a throwaway `--settings` file plus a hook script per arm, so **it needs no migrated project and no Claude Code session to reproduce** -- which makes it cheap enough to keep. **Two notes for whoever lands it as a test.** First, it must assert on the OUTPUT and not on the process exit code, because the blocked run exits 0. Second, **it needs ARM1 as well as ARM2**: an assertion that `2` blocks passes equally on a build where every code blocks, and only the `1`-does-not-block arm ties the failure to this change.

**And the register cannot see it, which is why no instrument caught it** (ic, 2026-08-16). **`claude hook` HAS a dispatch-table row, `keep` / `as-observed`** -- so the canon asserts the command survives into v3 exactly as it behaved in v2. **The binary does not implement it, and the gap between those two facts is invisible to every surface instrument**, because they compare the table against clap's SHAPE: `claude hook` is present, correctly shaped, takes its `<NAME>`, and parses. **It answers. It answers `2`.** The register can distinguish `keep` from `retire`; it cannot distinguish **wired** from **wired and implemented**, and this issue is what that costs. `surface_check.sh`, `dispatch_ssot.rs` and `read_claim_probe.sh` all report agreement on a row whose command is a lockout.

## Related

- 0038 -- the fix that introduced this; correct for its own consumer, and its constant collides with another's
- 0042 -- the other half of the same swap: `intent info` unimplemented turns the whiteboard guards off by fail-open. Same cause, opposite failure direction, and the two together are the argument for enumerating consumers rather than fixing them one at a time
- 0016 -- hooks continuity: `.claude/**` byte-untouched and _"consumer sessions must not notice the swap"_. **This is the invariant's own headline case, and it is currently violated in the most direct way available**
- AC-10.4 -- asserts `.claude/settings.json` and `.claude/scripts/**` are byte-identical pre/post migration. **They are, and the sessions are still dead** -- the same byte-identity-cannot-see-a-semantic-break finding that made AC-10.9 a new criterion rather than a widening
- AC-10.9's ordering note -- _"WP-10 landing before WP-07 puts every migrated project in this state"_, written about the commit gate and now true of the prompt gate as well

## Resolutions

**CLOSED 2026-08-16. The instance is fixed; the class is not, and the class has its own issue.**

**Fixed by cc**: `claude hook` and `info` are implemented. **Verified independently by dc and by vc**, at `native/rust/target/release/intent`:

```
rc=0  intent info                                Intent: The Steel Thread Process
rc=0  intent claude hook require-in-session
rc=0  intent claude hook session-context         Intent project: Intent
```

**The prompt gate passes through, so the lockout is gone.** dc lifted `install.md`'s 0043 publication hold at `61724664`, leaving 0036 as the only hold. **The section is KEPT rather than deleted** -- dc's reasoning, and it is the right one: _"a document that erases a hold once it lifts teaches nobody why it was there."_

**Against the three parts of the proposed fix:**

1. **Implement `claude hook` in v3 -- DONE and verified.**
2. **Stop letting one constant answer to two contracts -- NOT done, and deliberately carried.** This is the class, and it is now **issue 0044**, which measures the mirror image: the tool spells five unrelated conditions with `1` while the callers assign four meanings to `2`. Fixing it under this issue's time pressure is how the first three of these arrived.
3. **Enumerate the consumers beside the constants in `spine.rs` -- DONE by cc**, citing the five-arm measurement. **And the enumeration found a SIXTH consumer that none of the three of us had named**: `.claude/scripts/post-tool-advisory.sh:73` invokes `critic <lang>` with `|| true`, so `2` there means nothing at all and the advisory is suppressed. **That is the enumeration earning its place on the day it was written** -- four issues had each been diagnosed against whichever consumer happened to be in view, and the first act of listing them all turned up another one.

**Two findings from this issue that outlive its fix**, both recorded in `install.md` beside the kept hold:

- **A blocked prompt exits the `claude` process with `0`.** The block is in-band, so a wrapper checking the process exit code sees success while the model never saw the prompt.
- **`Stop` at exit `2` means "do not stop"** -- measured at 24s and zero output against 3s and `PONG`. Intent's `Stop` hook is a bare `echo` and therefore safe **by accident of its wiring**, and routing it through `intent claude hook` for consistency is the most natural tidying move available.

**One thing this issue got wrong and corrected in place**: it was filed and confirmed as a _migrated-project_ defect. dc measured that the trigger is **PATH, not migration** -- and the proof was already in this issue's own ARMV3 arm, which ran in a directory that was not an Intent project at all. **The result was right and the framing around it was never tested by anything.**
