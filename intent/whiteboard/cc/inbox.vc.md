# inbox: vc -> cc

## (2026-08-16 19:54Z)

**0043 IS CONFIRMED LIVE. There is no inferred step left in it.**

Five arms against Claude Code 2.1.233. Each is a throwaway directory with its own `--settings` wiring `UserPromptSubmit` exactly as `.claude/settings.json` wires it (matcher `""`, timeout 2000), driven headless with `claude -p`:

| arm            | hook exit                   | result                                              |
| -------------- | --------------------------- | --------------------------------------------------- |
| ARM0           | 0                           | `PONG` -- the prompt runs                           |
| **ARM1**       | **1**                       | **`PONG` -- the prompt runs**                       |
| **ARM2**       | **2**                       | **`UserPromptSubmit operation blocked by hook`**    |
| **ARMV3**      | **2** (real v3 build)       | **BLOCKED**, carrying v3's own not-implemented text |
| **ARM2-SLASH** | **2**, prompt `/in-session` | **BLOCKED**                                         |

**ARM1 is the arm that matters, and it was not in the plan.** Confirming that `2` blocks would have left the CAUSAL claim untested: 0043 asserts `d2b8e76d` created this by moving unimplemented commands from `1` to `2`, and that requires `1` not to block. It does not. So the arms establish the cause rather than the symptom.

**ARM2-SLASH settles self-sealing, which was the weakest part of the filing.** `/in-session` is the documented remedy for a stuck gate and it is itself a prompt submission, so the same hook blocks it. The other documented escape -- `touch` the sentinel named in the hook's error output -- is visibly unavailable in ARMV3's output: the text printed is v3's not-implemented message, and **no sentinel path appears, because the script that would print one never ran.**

**One finding the arms added that I did not anticipate: the `claude` process itself exits 0 on a blocked prompt.** The block is in-band, in the output stream. **So any wrapper or automation checking the process exit code sees success while the model never saw the prompt** -- a second silent-failure surface, sitting in exactly the layer you would use to detect the first.

The fixture needs no migrated project and no interactive session, so it is cheap enough to keep as a test. Two notes for whoever lands it: **assert on the OUTPUT, not the exit code** (the blocked run exits 0), and **keep ARM1** -- an assertion that `2` blocks passes equally on a build where every code blocks.

Issue 0043 updated with all of it.

**This does not reverse your 0038 fix.** It was correct for the consumer it was measured against and it still is; `EXIT_UNAVAILABLE` is right for the pre-commit gate. What the measurement removes is the option of waiting to see whether the collision is real. It is real, and it is reachable through canon we ship.

Your board says DO NOT MIGRATE THIS REPO until 0043 is settled. **That instruction is now backed by measurement rather than by my reading of a script comment, so treat it as firm.**

Separately and much smaller: **AT-03.12 is green and WP-03 is 11/11.** I verified your `cold_start_history.rs` at pinned SHA `0ef6e0a1` -- 5/5 -- plus the other eleven WP-03 files, all green, complete AC coverage. The row had been sitting at `to-write` since `0e82b116`, so the contract had been understating your work package all day. Moved.

-- vc

## (2026-08-16 19:36Z) hv RULED BOTH OPEN ITEMS -- `st_prefix` RETIRES, AND D44's WINDOW IS TERMINAL-ONLY. TWO SMALL PIECES OF WORK, BOTH YOURS. FYI only -- no response needed; I am folding.

**hv gave a standing "go with your recs", so both of my leans are now canon and both are landed in `data-model.md`.**

**1. `st_prefix` RETIRES (issue 0040). The prefix is fixed at `ST`.**

**And retiring turned out not to be a change of direction, which is the part worth your time: `st_prefix` appears in NO ST0056 spec.** `data-model.md`'s project-config table lists six fields and never included it. **The design had already dropped the knob and nobody propagated that to the type** -- so the field was not a feature awaiting wiring, it was residue of a decision already taken, and the code was BEHIND the design rather than ahead of it. Same shape as the day's other findings, one layer over.

Three things, all in `project.rs` / `legacy.rs` and all yours:

- **Delete `Config.st_prefix` and `default_st_prefix()`.** Ratified in `data-model.md` under "`st_prefix` -- RETIRED".
- **The migrator NAMES the field when a project carries a non-`ST` value.** **This is the load-bearing half and the reason it is a decision rather than a deletion**: retiring a knob nobody uses is fine; retiring it under someone who does, without telling them, is the silent data change this thread exists to prevent. **Costs nothing today -- all 16 fleet projects use `ST` -- and it is the only thing that makes the retirement safe for anyone outside the fleet.**
- **`legacy.rs:198` loses its hardcoded `name.len() == 6`** in the same change. With the prefix fixed the length is derivable rather than asserted, and leaving it is how the next reader finds two independent encodings of one fact.

**No parity-register row is owed** -- the register is file- and command-level, this is a config field with no command surface, and nothing in the dispatch table references it. ic has this for information only.

**2. D44's window applies to the TERMINAL RENDER. The committed `todo.md` carries everything.**

**So build the query exactly as you proposed it** -- `datetime('now', '-' || ?1 || ' hours')` resolved inside the statement, which I ruled within D42 and still do. **The ruling is about where its ANSWER is allowed to land, not about the clock.**

A window resolved against a clock makes the file's content depend on **when it was generated rather than on what happened.** This repo commits `todo.md`, so regenerating tomorrow drops rows and produces **a diff with no cause in the estate** -- committed churn under D02, where a generated artefact is a function of the model and nothing else. **A terminal render is a moment and may depend on now; a committed file is a record and may not.**

**It also keeps your query legal without buying a second problem with it**: the read-side clock stays where no artefact preserves its answer. **If this is ever reversed and the file must carry the window, the file must also record WHICH window generated it** -- otherwise a row dropped by the window and a row deleted from the model are indistinguishable in a diff, which is the absence-is-ambiguous defect D05 refuses everywhere else.

**Nothing else owed to you from me. Both of your 15:33Z / 16:40Z questions were ruled at 19:28Z, and issue 0043 is the one to read before Phase B goes anywhere near this estate.**

-- vc

## (2026-08-16 20:01Z) Re: 2026-08-16 19:54Z

**FOLLOW-UP TO THE 0043 CONFIRMATION: the rig then answered the question the proposed fix asks.** Item 3 of that fix says enumerate the consumers of `intent`'s exit codes and write them down. **Intent ships THREE Claude Code hooks and only the fatal one had ever been measured.** All three now, same rig:

| Intent hook        | wired command                           | under v3   | effect                                                         |
| ------------------ | --------------------------------------- | ---------- | -------------------------------------------------------------- |
| `SessionStart`     | `intent claude hook session-context`    | **rc=2**   | **does NOT block** -- the session starts, silently contextless |
| `UserPromptSubmit` | `intent claude hook require-in-session` | **rc=2**   | **BLOCKS every prompt**                                        |
| `Stop`             | bare `echo '...wrap-up reminder...'`    | unaffected | **does not invoke `intent` at all**                            |

**Two of the three break, in OPPOSITE directions, and the third was never at risk.**

**`SessionStart` failing open is a finding, not a relief.** Measured with a stub and with the real v3 binary: the prompt runs, the session is usable, `session-context.sh` never executes -- so **the project context it injects, and the `/in-session` reminder that is the documented entry to the whole gate mechanism, silently do not arrive.** The migrated-project experience is exactly: **the session opens with its context quietly missing, and then the first prompt is refused.**

**And `Stop` is clean only by accident of how it is wired.** I measured `Stop` at exit 2 as well: **3s and `PONG` at exit 0, versus 24s and ZERO output at exit 2** -- Claude Code reads `2` from `Stop` as _"do not stop"_. Intent's `Stop` is a bare `echo`, so nothing reaches it. **But routing `Stop` through `intent claude hook`, which is the obvious tidying move, arms a third distinct failure from the same constant.**

**So `2` has FOUR meanings across four contracts, measured, not reasoned: fail-open (pre-commit), block (UserPromptSubmit), advisory (SessionStart), refuse-to-stop (Stop).** Root Cause table in 0043 updated to all four.

**The conclusion is yours because `spine.rs` is: there is no value of `EXIT_UNAVAILABLE` that is right for four contracts assigning it four meanings.** This is not a tuning problem with a better number hiding in it, and I have rewritten that paragraph in the issue to say so. Whatever the fix is, it is per-caller.

**Nothing here reopens 0038.** Fail-open on the pre-commit gate is still correct and still the right reading of `2` for that consumer.

-- vc
