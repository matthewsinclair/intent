---
id: "0059"
title: eight of eleven parity check instruments have no execution site, so a check that never ran is indistinguishable from one that ran green -- and all three AT-05.5 cites are among them
date: 2026-08-17
reporter: matts
status: CLOSED
severity: medium
---

# 0059: eight of eleven parity check instruments have no execution site, so a check that never ran is indistinguishable from one that ran green -- and all three AT-05.5 cites are among them

## Tags

parity-tools, gates, runner, vacuous-green, acceptance, measured

## Summary

`intent/st/ST0056/parity/tools/` holds eleven `_check.sh` instruments. **Three have an execution site. Eight have none.**

Measured across `bin lib scripts .github native/rust/crates` plus the tools directory itself, with mentions filtered from invocations:

| instrument                  | execution site                       |
| --------------------------- | ------------------------------------ |
| `provenance_check.sh`       | `bin/.devbin/cmd/precommit:241`      |
| `view_skew_check.sh`        | `bin/.devbin/cmd/precommit:253,257`  |
| `generator_inputs_check.sh` | `bin/.devbin/cmd/precommit:276`      |
| `class_vocab_check.sh`      | **none**                             |
| `corrected_check.sh`        | **none**                             |
| `drift_check.sh`            | **none**                             |
| `guide_refs_check.sh`       | **none**                             |
| `implemented_check.sh`      | **none**                             |
| `residue_class_check.sh`    | **none**                             |
| `stale_at_check.sh`         | **none**                             |
| `surface_check.sh`          | **none** -- see the correction below |

So `precommit` is the family's home, and eight instruments were written, reasoned about at length in their own headers, cited in `MODULES.md`, and never wired to anything that runs them.

## The correction that matters more than the count

**`surface_check.sh` was credited to `cargo test` on the first pass and it runs from nothing.** `git grep` puts it in `native/rust/crates/intent-cli/tests/dispatch_ssot.rs` at `:327` and `:868`; both hits are `///` doc comments. A file-level grep reports a test file referencing the tool, which reads exactly like the tool being driven by the suite.

This is the **third instance in one day, by this reporter, of a comment counted as data** (the others: `enum_str` call sites in `render.rs`, where an import and a comment inflated 34/4 to 42/6; and the coverage needle's two false zeros). It is recorded here rather than in a fold because the rate is the argument: three in a day, from someone who has written the rule down twice, is a case for a mechanical filter rather than more care.

## Reproduction

At `0f87fc2c`, clean on the paths measured:

```
for s in $(ls intent/st/ST0056/parity/tools/ | grep '_check\.sh$'); do
  git grep -n -F "$s" -- bin lib scripts .github native/rust/crates \
      'intent/st/ST0056/parity/tools' \
    | grep -v "tools/$s:" \
    | grep -vE ':[0-9]+: *#' \
    | grep -E 'bash |sh |\$\(|`|Command::new|\.arg\('
done
```

The `-vE ':[0-9]+: *#'` arm is what separates this measurement from the wrong one. Without it the answer is eight of eleven wired, which is the answer the first pass gave.

## Root Cause

An instrument's existence and an instrument's execution are recorded in different places and nothing joins them. `MODULES.md` registers the tool; `precommit` invokes it; **no artefact carries both**, so a tool can be registered, documented, mutation-tested, cited by an acceptance row, and never invoked, with every individual record correct.

## Impact

**The load-bearing case is AT-05.5, which is `green`.** Its note names three instruments as closing it -- `corrected_check.sh`, `class_vocab_check.sh`, `surface_check.sh` -- and **all three are in the no-runner set.** The row is green on three instruments that execute only when a node types their name.

**This is not theoretical and the demonstration is already on the record.** ic, 2026-08-17: `corrected_check.sh` had been dying at exit 2 since that morning -- for all twenty corrected rows, not the two that caused it -- because a ratification was written into an invented `voice_ruling` field where the script hard-refuses on `ratified_in == null`. The refusal was correct, loud, and by design. **It sat through a fold, a compact, and a board reading "nothing blocked", because nothing ran it.**

ic's general form, which is the sentence this issue exists to record:

> **A refusal and a silence look the same to anyone not looking.**

**Sibling of 0057, and deliberately not folded into it.** There, `int check critic` seals a zero-byte `.errors` from a zero-file scan and four readers define green as exactly that emptiness -- a check that RAN over nothing. Here there is no seal at all, so "never ran" and "ran green" are the same absence. Same class -- a verdict that cannot carry its own subject -- and the fixes differ at each end: 0057 needs the population stamped into the seal, this needs an invocation.

## Proposed Fix

**Wire the eight into `precommit` beside their three siblings**, in the same report-never-gate posture the family already uses (`corrected_check.sh` reports; only its own inability to measure refuses). That is the mechanical half and it is one edit per tool.

**The half that closes the class rather than the instance: make registration and invocation checkable against each other.** A tool in `parity/tools/` matching `*_check.sh` with no execution site anywhere is either unwired or deliberately manual, and today those are indistinguishable -- so the check is a roster with a declared disposition per tool (`gated` / `manual`, with a reason), refusing on a tool present in the directory and absent from the roster. Same posture `transitions.rs` takes for the graph, and the same reason: the roster fails on the day a tool is added, with nobody having to remember this issue exists.

**Do not simply add them to the pre-commit gate without the disposition field.** Some of these are slow or estate-scanning, and a gate nobody keeps is worse than an unwired check that is honestly labelled -- which is the whole finding.

## Related

- ST0056 -- Intent v3.0.0
- AC-05.5 / AT-05.5 -- the acceptance row whose green rests on three uninvoked instruments
- 0057 -- the sibling: a seal that cannot distinguish an empty population from a clean one
- `bin/.devbin/cmd/precommit:241,253,276` -- the three that are wired, and the model for the rest
- `parity.md` -- the measurement-rules section; ic's sentence lands there

## Resolution -- CLOSED 2026-08-17, ON BOTH HALVES, AND THE CLASS HALF WAS MUTATION-PROVED IN BOTH DIRECTIONS

Built by ic. Verified by vc at `ceffcd49`, by execution rather than by reading the roster.

### The mechanical half

**Was 3 of 11. Is 9 of 13**, with the remaining four declared `manual` and each carrying its reason:

```
$ grep -oE '\$TOOLS/[a-z_]+_check\.sh' bin/.devbin/cmd/precommit | sort -u | wc -l
9
$ ls intent/st/ST0056/parity/tools/ | grep -c '_check\.sh$'
13
```

| declared `manual`      | the reason the roster gives                                            |
| ---------------------- | ---------------------------------------------------------------------- |
| `drift_check.sh`       | compares a STAMPED inventory against live canon                        |
| `guide_refs_check.sh`  | takes required prose-file arguments, so there is no bare invocation    |
| `implemented_check.sh` | invokes every declared row in a fresh throwaway project                |
| `surface_check.sh`     | probes `--help` across 100+ paths, so every commit pays a full sweep   |

**This is the shape this issue asked for and specifically warned against getting wrong**: the Proposed Fix said *do not simply add them to the pre-commit gate without the disposition field*, because a gate nobody keeps is worse than an unwired check that is honestly labelled. Four are unwired and labelled.

### The class half, which is the part worth having, and it discriminates

The roster refuses in **both** directions and the refusals were fired rather than read -- in a `git archive HEAD` extract, never in the shared tree, because planting a fake instrument while three nodes run gates makes one node's test another node's incident:

```
M1  a tool on disk with no roster row
    -> exit 1: "zz_unrostered_check.sh exists in the tools directory and has NO roster
       row -- declare it gated or manual, with a reason"

M2  a roster row flipped to `manual` while the runner still invokes it
    -> exit 1: "stale_at_check.sh is rostered MANUAL and the runner invokes it --
       the roster is wrong in the direction that reads as safe"
```

**M2's message names the DIRECTION of the error, which is the detail that makes it a closure rather than a check.** The dangerous disagreement is not symmetric: a tool wrongly marked `gated` produces a loud missing-invocation failure, while one wrongly marked `manual` reads as a deliberate exemption and is exactly this issue's defect wearing a declaration.

**And the check canaries its own needle**, which is the discipline this issue exists to enforce applied to the enforcer:

- it dies if the runner stops defining `TOOLS=`, because the needle is anchored on that variable;
- it dies if the needle finds **zero** invocations, on the stated grounds that some are known to be wired, so an empty result is the needle failing rather than the gate emptying.

That is the answer to "what other state would produce this same output?" asked of the instrument itself, and it was asked before anyone had to.

### WHAT DOES NOT CLOSE, STATED PRECISELY BECAUSE THE ISSUE'S HEADLINE NAMED IT

**AT-05.5 cites three instruments; two are now gated and `surface_check.sh` is `manual`.** So the row still rests, in part, on an instrument nothing runs.

**The difference is real and it is the whole point of the fix, so it should not be overstated either way.** Before: the row rested on three instruments and nothing anywhere recorded that they were uninvoked. Now: it rests on two that run every commit and one that is *declared* uninvoked, with a reason a reader can weigh. **"Never ran" and "ran green" are no longer the same absence** -- which is this issue's title -- but a green AT-05.5 is still not evidence that `surface_check.sh` passed.

**Left as a note on the row rather than filed**, because the remedy is a judgement about the AT's wording rather than a defect: an AT note that cites an instrument should cite its disposition with it, so nobody has to open the roster to know whether a citation implies a run.

### TWO CORRECTIONS TO THIS ISSUE'S OWN CONTENTS

**1. The reproduction in this issue now returns a FALSE 13-of-13 and must not be re-used as written.** `runner_roster_check.sh` holds the roster as an inline table naming every tool, so the grep counts a roster row as an execution site. Run today it reports every instrument wired, including the four that deliberately are not:

```
drift_check.sh   1   <- intent/st/ST0056/parity/tools/runner_roster_check.sh:90
                        "drift_check.sh   manual   compares a STAMPED inventory..."
```

**The fix that made this issue closeable is what broke its reproduction**, and in the same direction the issue is about: a name counted as an invocation, by the same instrument that could not previously tell a comment from data. The measurement that survives is narrower and should replace it -- count `$TOOLS/<name>` occurrences in the runner alone, which is exactly the needle `runner_roster_check.sh` uses and canaries.

**2. A mutation of mine had the wrong subject and was caught by its own error text.** The first M2 removed a tool's invocation with `sed` and broke the runner, so `runner_roster_check.sh` answered `error: the runner refused --list-guards` -- a true report about a runner I had destroyed, not about a missing invocation. **The tell was that the message named a capability rather than a disposition.** Redone as a roster-row flip, which changes one declaration and leaves the runner intact.

## Related

- 0057 -- the sibling at the artefact end: a seal that cannot carry its population. **Still open; this fix does not touch it.**
- AT-05.5 -- partially discharged, as set out above
- `intent/st/ST0056/parity/tools/runner_roster_check.sh` -- the roster, and the check that reads it
- `bin/.devbin/cmd/precommit` -- the nine gated invocations
