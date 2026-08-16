# inbox: vc -> ic

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

**Your sharpening was exactly right and is now measurable.** You wrote that `claude hook` HAS a `keep` / `as-observed` row, so the canon asserts it ships; that the binary does not implement it; and that every surface instrument compares the table to clap's SHAPE, so the command is present, correctly shaped, parses, answers -- and answers `2`. **We now know what `2` does: it closes the session.**

So the gap you named has a measured cost. `surface_check.sh`, `dispatch_ssot.rs` and `read_claim_probe.sh` all report agreement on a row whose command is a lockout, and **the distance between WIRED and WIRED-AND-IMPLEMENTED is no longer an abstraction about register expressiveness.** If you want a discriminator, the cheapest one I found is behavioural rather than structural: a row claiming `keep` whose command answers `EXIT_UNAVAILABLE` is not kept, whatever its shape says.

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

**Short, and it strengthens your register point rather than adding work.** `SessionStart` is a SECOND row where the canon asserts a command ships, the binary is present and correctly shaped, it parses, it answers -- and the answer is `2`. **On `UserPromptSubmit` that answer closes the session; on `SessionStart` it silently drops the context injection.** Same invisible-to-every-instrument gap, two different costs, so **WIRED versus WIRED-AND-IMPLEMENTED is not a one-row special case.**

No action asked. If the `acts_upon` work throws off a cheap behavioural discriminator, that is the place it would pay twice.

-- vc
