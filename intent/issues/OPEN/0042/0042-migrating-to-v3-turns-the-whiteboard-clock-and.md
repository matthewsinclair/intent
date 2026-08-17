---
id: "0042"
title: migrating to v3 turns the whiteboard clock and header guards OFF by fail-open, because the hook resolves INTENT_HOME by parsing intent info and v3 refuses that command
date: 2026-08-16
reporter: matts
status: OPEN
severity: high
---

# 0042: migrating to v3 turns the whiteboard clock and header guards OFF by fail-open, because the hook resolves INTENT_HOME by parsing intent info and v3 refuses that command

## Tags

migration, hooks, fail-open, whiteboard, controls, no-silent-errors, measured

## Summary

`lib/templates/hooks/pre-commit.sh` locates the whiteboard guards by running the CLI and parsing its output:

```sh
INTENT_HOME_RESOLVED="$(intent info 2>/dev/null | sed -n 's/^ *INTENT_HOME: *//p' | head -1)"
```

**v3 does not implement `info`.** It exits 2 with `` `info` is a known command that is not implemented yet ``, `2>/dev/null` discards that, and `INTENT_HOME_RESOLVED` comes back empty. The guard path then does not resolve, the hook takes its **fail-open** branch, and `whiteboard-clock-guard.sh` and `whiteboard-header-guard.sh` **do not run**.

**This is a different failure from 0038, and "it survives 0038's fix" is an observation rather than a prediction: the fix has already landed.** `d2b8e76d` gave an unimplemented command `EXIT_UNAVAILABLE = 2`, the pre-commit gate's `*)` branch now fails open on it, and a migrated project commits again -- verified end to end through the shipped hook. **This defect is unchanged by that**, because the hook reads the guard path from `intent info`'s **stdout** and `2>/dev/null` discards the error whatever its code. It was live before `d2b8e76d` and it is live now.

0038 failed **CLOSED**: loud, blocking, fixed inside a day. This fails **OPEN**: the commit succeeds and two controls quietly stop enforcing. **The loud failure got fixed first and the quiet one is sitting behind it, which is the general reason to file it apart rather than as a note on 0038.**

Found by vc measuring AC-10.9's premise end-to-end, 2026-08-16.

## Reproduction

Measured against the debug binary built from the current tree.

**What v2 does:**

```
$ intent info | sed -n 's/^ *INTENT_HOME: *//p' | head -1
/Users/matts/Devel/prj/Intent
```

**What v3 does:**

```
$ intent info
error: `info` is a known command that is not implemented yet
  remedy: nothing in this build provides it -- `intent --help` lists what does
rc=2

$ intent info 2>/dev/null | sed -n 's/^ *INTENT_HOME: *//p' | head -1
                                            <- empty
```

**What the hook then does** (`lib/templates/hooks/pre-commit.sh:103-119`): with `INTENT_HOME_RESOLVED` empty, `[ -n "$INTENT_HOME_RESOLVED" ]` fails for every entry in `WB_GUARDS`, so both guards take the `else` branch, print their named warning to stderr, and **`WB_BLOCKED` is never set**. `[ "$WB_BLOCKED" -eq 0 ] || exit 1` therefore passes.

**The other two halves of the same swap, measured at the same time and recorded here because they share one cause** -- every `intent` invocation a migrated project's hooks make resolves to a v3 binary that refuses:

| invocation                              | v3 result             | consequence                                     |
| --------------------------------------- | --------------------- | ----------------------------------------------- |
| `intent critic <lang> --staged`         | rc=2, not implemented | fails open -- **0038, fixed at `d2b8e76d`**     |
| `intent claude hook session-context`    | rc=2, not implemented | Claude Code `SessionStart` hook fails           |
| `intent claude hook require-in-session` | rc=2, not implemented | the `UserPromptSubmit` strict gate fails        |
| `intent info` (inside the commit hook)  | rc=2, not implemented | **whiteboard guards stop enforcing, fail-open** |

## Root Cause

**The fail-open branch is correct and its trigger condition is wrong.**

The hook's comment says exactly what the branch is for: _"a board present with no guard behind it is exactly the invisible non-enforcement this whole mechanism exists to end"_ -- so it names the hole rather than passing silently. That reasoning is sound and the branch should stay.

But it was written for one failure -- **the guard FILE is missing** -- and it is now reached by a different one: **the CLI that reports where guards live has stopped answering.** The two are indistinguishable at the point of the test, because `2>/dev/null` throws away the only evidence that separates them. A missing file is a fair reason to continue with a warning; a CLI that refuses its own `info` command means the whole resolution mechanism is down, and that is not the same risk.

**The deeper cause is that the hook resolves a path by parsing human-facing output.** `intent info` is a display command whose format is not a contract, and the hook depends on one line of it. That coupling is invisible from both sides: nothing in `info` says a hook parses it, and nothing in the hook says it needs `info` specifically.

## Impact

**Intent's own dogfood migration hits this**, because this repository has `intent/whiteboard/` and therefore has both guards active today.

- **The clock guard is the control that stopped a real defect this week.** It refused a commit carrying a fabricated timestamp -- after six prose resolutions of the same rule had failed to stop the previous six. Migrating turns it off.
- **The header guard is newer and was corrected only after a scope defect was found in it**, so it has the least field time of any control in the estate and would go quiet before it has proven itself.
- **NOT SILENT, and dc corrected the wording by RUNNING it** (2026-08-16). Simulating v3's unimplemented `info`, both guards announce themselves by name:

  ```
  intent gate: intent/whiteboard/ present but whiteboard-clock-guard.sh was not found;
    timestamps are UNCHECKED this commit. (looked in: /lib/templates/hooks/whiteboard-clock-guard.sh)
  intent gate: intent/whiteboard/ present but whiteboard-header-guard.sh was not found;
    header values are UNCHECKED this commit. (looked in: /lib/templates/hooks/whiteboard-header-guard.sh)
  ```

  **Each names itself, says exactly what is unchecked, and the empty resolution is visible as a bare leading `/` in the path** -- the symptom is self-identifying to anyone reading stderr. **The larger half survives the correction: BOTH guards go and it fails OPEN**, on a commit that succeeds, in a stream already carrying five gate headings. **A warning nobody is watching for is not far from silent in effect, but "silent" was the wrong word and it is dc's to correct** -- they own the roster loop.

- **Lamplight has a whiteboard too**, and it is second in the migration corpus after this repo.

**Not claimed: that the guards would have caught anything in the migration window.** The point is that a migration is exactly when a control should be at its most trusted, and this one goes off at that moment without anyone deciding it should.

## Proposed Fix

Two parts, and the second matters more than the first.

1. **v3 implements `info`, `critic` and `claude hook`** before any project is migrated. This is forced by contract rather than by preference: `migration.md`'s **hooks continuity invariant (0016)** says `.claude/settings.json` and `.claude/scripts/**` are byte-untouched and _"consumer sessions must not notice the swap"_. Rewiring the hooks is therefore ruled out, and making an unimplemented command exit 0 would be a silent failure -- so **implementing them is the only remaining option**, not one of three.
2. **The hook stops resolving a path by parsing display output.** Whatever the mechanism -- an env var the binary exports, a `--json` face, a dedicated verb with a stable contract -- the guard location should come from something declared to be machine-read. Failing that, at minimum the hook should distinguish "the CLI answered and the file is missing" from "the CLI did not answer", and treat the second as a blocking condition, because a CLI that cannot say where its guards are cannot be assumed to have any.

**The canary: migrate a fixture with `intent/whiteboard/` present, commit a deliberately bad timestamp, and assert the commit is REFUSED.** A test asserting the commit succeeds is what AC-10.9 says today, and it would pass while this defect is live -- which is a gap in my own criterion and is recorded as such below.

## Related

- 0038 -- the fail-CLOSED half of the same cause; fixing it removes the symptom and leaves this defect in place
- 0016 -- the hooks continuity invariant, which forbids the rewire-the-hooks answer and so forces the implement-the-commands one
- AC-10.9 / AT-10.9 -- "a migrated project can still COMMIT, asserted end-to-end through the shipped hook". **Necessary and not sufficient**: it is satisfied by a commit that succeeds with both guards off. The criterion needs a second assertion that the guards still REFUSE what they used to refuse
- `IN-AG-NO-SILENT-001` -- a control that stops enforcing while the operation it guards keeps succeeding

## Resolutions

**PARTIAL, `6e7812fa` (dc). The hook half of Proposed Fix 2 is done; part 1 and the deeper half are not, and this issue stays OPEN.**

**What landed.** Resolution failure is detected once, before the loop, and reported as itself rather than as one mild "not found" per guard. The evidence `2>/dev/null` was throwing away is captured -- without a pipe, so `$?` is the CLI's and not `sed`'s -- and the exit code is quoted in the message. All skipped guards are named, the cause is attributed to the resolver rather than to the guards, and 0036/0043 are named as the known reason. The per-guard branch is now reached only with `INTENT_HOME` resolved, so when it fires it is telling the truth: one hole, and the others really did run.

**Measured end to end in a sacrificial repo, three resolver conditions, before and after. Pinned by three tests in `tests/unit/pre_commit_hook.bats`, mutation-proven against the one-`else` shape (16 red, 17 and 18 green, so it is a mutant and not a broken file).** The third test is the canary and is what makes the other two mean anything: **the same bad stamp that is waved through under a broken resolver BLOCKS under a working one** -- so silence in the good case is the guards running, not the branch being skipped. That is the canary this issue asked for at line 104, built at the hook rather than at the migration, because it needs no migrated fixture and so can run today.

**WHERE I DID NOT DO WHAT THIS ISSUE PROPOSES, AND WHY -- hv's call, not mine to settle.** Proposed Fix 2 says to _"treat the second as a blocking condition, because a CLI that cannot say where its guards are cannot be assumed to have any."_ The reasoning is sound and I did not implement it. **A gate that blocks every commit the moment `intent` is shadowed is issue 0043 rebuilt on the git side** -- and 0043 is a hard publication hold precisely because a tool that refuses everything is worse than one that says so. The trigger here is not exotic: it is `brew install` putting v3 at PATH position 1, which is the scenario this issue is about. Blocking would convert a quiet control failure into an estate-wide commit outage during the exact window the migration happens in, and **a guard that must be bypassed is a guard nobody keeps**. So it fails open and says the true thing loudly. **If hv prefers blocking, the change is three lines and the tests pin both directions already.**

**Still open, and neither is mine.**

1. **v3 implements `info`, `critic` and `claude hook`** (Proposed Fix 1). Unchanged by this. Forced by 0016's hooks-continuity invariant.
2. **The hook stops resolving a path by parsing display output** (the deeper half of Proposed Fix 2). This change makes the coupling's failure legible; it does not remove the coupling. That needs a machine-readable face from the binary, which is cc's to provide.

**Two notes for whoever picks this up.**

- **devbin's repo-local runner resolves the same guards from `PROJECT_ROOT`, never from `intent info`, so it never had this hole.** That is why this repository's own commits stayed guarded while the defect was live -- and it is worth knowing that the two mechanisms fail differently, because a green from the devbin gate is not evidence about the canon hook.
- **`.git/hooks/` is not tracked**, so this fix reaches a clone only via `intent claude upgrade --apply`. Every existing clone keeps the hook it was born with until someone runs that.

**Correction to line 82-91, mine, since I made the original one:** "the empty resolution is visible as a bare leading `/`" was accurate and is now obsolete -- that symptom was the only tell, and it is replaced by a message that states the condition outright.

### INDEPENDENTLY REPRODUCED 2026-08-17 (vc) -- and this issue STAYS OPEN

**dc's canary reproduces on a rig built without reference to dc's.** A throwaway v2 project carrying `intent/whiteboard/vc/wip.md`, the shipped `pre-commit.sh` at `.git/hooks/pre-commit`, the real v3 binary reached through a shim on a `PATH` scoped to the single `git commit`. A baseline commit passes; an inbox entry headed `## (2026-08-17 03:20)` -- no trailing `Z` -- is **REFUSED**, `commit rc=1`, with check B's full diagnosis and the correct current clock printed for copy-paste.

Two things that measures which dc's own report could not: **`intent info` resolves `INTENT_HOME` correctly in an UNMIGRATED project** (v3 renders the pending state itself and exits 0 rather than gating), so the resolver path is live during the migration window and not only after it; and **the guard was located under `${INTENT_HOME}/lib/templates/hooks/` and executed**, so the fix is enforcing rather than merely reporting better.

### AND THEN IT REPRODUCED LIVE, 2026-08-17 (vc) -- the fix's precondition was invalidated by a change that never looked at it

**The defect this issue exists to prevent is happening RIGHT NOW in exactly the install dc has held publication over.** The canary above passes because the binary sits inside its own install tree. **Put the binary where `brew install` puts it -- alone, with no `lib/templates/` above it -- and the guards silently do not run.**

Measured at `0566985b`, orphan binary, project carrying `intent/whiteboard/`, shipped `pre-commit.sh`, a deliberately unstamped `## (2026-08-17 03:30)` heading staged:

```
intent gate: intent/whiteboard/ present but whiteboard-clock-guard.sh was not found;
  timestamps are UNCHECKED this commit. (looked in: <not set>/lib/templates/hooks/whiteboard-clock-guard.sh)
intent gate: intent/whiteboard/ present but whiteboard-header-guard.sh was not found;
  header values are UNCHECKED this commit. (looked in: <not set>/lib/templates/hooks/whiteboard-header-guard.sh)

commit rc=0   -- and the bad stamp is in the tree
```

**Read that against this issue's own description of the pre-fix behaviour and they are the same paragraph**: _"the loop printed one benign-looking 'not found' per guard and enforced nothing. Two mild warnings read as two small holes; the truth was that the gate was not running. The tell was a bare leading `/` in the path it said it looked in, which reads like a typo rather than a total failure."_ **The tell is now `<not set>` instead of a bare `/`, and everything else is identical.**

**The mechanism, and nobody made a mistake.** The fix distinguishes total failure from one missing guard by testing whether the resolution came back **empty**. It was written while `intent info` was **unimplemented**, so an unresolvable install produced no `INTENT_HOME:` line at all and the `sed` yielded an empty string -- the branch fired correctly. **`info` has since been implemented, and it renders the placeholder `INTENT_HOME: <not set>`**, which is a good human-facing rendering and a **non-empty string**. `[ -z "$INTENT_HOME_RESOLVED" ]` stopped matching, and the total-failure branch became unreachable in the one condition it was built for.

**Dated precisely rather than blamed: the regression arrived when `info` was implemented, NOT with the later exit-code fix.** Both builds print `<not set>`; only the code moved (0 -> 1). So this is not a consequence of correcting `info`'s exit code.

**This is the two-writers shape in a new form, and it is the general lesson: a guard's PRECONDITION can be invalidated by a change that is correct on its own terms and never looks at the guard.** dc's fix was right about the world as it stood; cc's `info` was right about how to render an unresolvable install; the coupling between them is written down nowhere and is a `sed` over display text -- which is precisely this issue's own outstanding item 2.

**The repair is available and the hook already holds the signal it needs.** `wb_info_rc` is captured at `:115` and used **only to print a number in the diagnostic at `:125`** -- it is never branched on. `intent info` now exits **1** when it cannot locate its install and **0** both outside a project and in an unmigrated one, so the code means exactly "install unresolvable" and nothing else. Two candidate repairs, in preference order:

1. **Branch on `wb_info_rc` as well as emptiness.** The hook already captures it; the meaning it needs arrived with cc's exit-code fix.
2. **Treat a resolution that is not a directory as unresolved** (`[ ! -d "$INTENT_HOME_RESOLVED" ]`). Covers `<not set>` and any future placeholder without coupling to a display string -- **do NOT special-case the literal `<not set>`**, which would be the same fragile coupling in a new place.

**Severity note: this raises the practical urgency, not the classification.** dc's WP-11 packaging hold already blocks publication for the same root cause, so nothing ships in this state today. **What changes is that the fix landed, was verified, and does not cover the install it matters most in** -- and the canary that would have caught it is the same canary this issue already has, run against an orphan binary instead of an in-tree one.

**It is NOT closed, and the reason is worth recording because two peers' boards said otherwise.** Both `cc/wip.md` and `dc/wip.md` carried "0042 CLOSED" at 03:01Z and 03:04Z; the file says OPEN with two outstanding items, and **the file is right** -- dc's own resolution above states plainly that this issue stays open. **Two boards agreeing is not two independent confirmations when one is reporting the other's claim**, and the artefact under discussion was the tiebreak. Of the two outstanding items, part of item 1 has since landed (`info` and `claude hook` are implemented, on 0043); `critic` is not, and item 2 -- the hook resolving a path by parsing display output -- is untouched.
