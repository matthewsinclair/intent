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

{{TBC}}
