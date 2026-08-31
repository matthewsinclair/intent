# AC-06.1 -- the 27 UNSTABLE files, resolved to four causes

**The read AC-06.1 was blocked on (cc, 2026-08-31), performed at HEAD `e0cdc389` in a detached worktree.** `coverage_map.sh` refuses to publish because the committed burn baseline no longer covers the on-disk estate, and **its own prescribed remedy does not fix the AC**: re-running `burn.sh` yields 27 UNSTABLE where the baseline had zero, so a quarter of the estate becomes unmeasurable by the method and the criterion is no closer. This document answers the question that was actually in the way -- **not _what is the burn ratio_ but _why are 27 files not green_** -- because until that is answered, reclassifying the affected register rows is eight judgement calls instead of one decision with one reason.

**Nothing here changes a register class.** Classification is a disposition and belongs to vc or hv.

## The instrument is deliberately HALF of burn.sh, and that is how vc's ruling is honoured rather than evaded

`burn.sh` runs each file twice -- default binding, then `INTENT_BIN=/usr/bin/false` -- and the delta is the burn. **UNSTABLE is defined by the FIRST run alone**: a non-zero default failure count means the baseline is not green, so the delta carries no information. Establishing a CAUSE therefore needs only the default half.

That is not a shortcut, it is the point. **vc ruled that the new baseline stays OUT** -- one carrying 27 UNSTABLE records _we do not know_ in a field consumers read as a measurement. Running only the default half means **no baseline is produced at all**, so the ruling is honoured literally instead of being measured around. A tool that had to be run in full, producing an artefact nobody wants, in order to answer a question that needs half of it, is the shape that gets a ruling quietly laundered.

Run in a detached worktree at the measured revision, because three peers are live in the shared checkout.

## Controls, asserted rather than assumed

**The read should be trusted because it survived these, not because the numbers came out tidy.**

1. **The count reproduces across revisions.** 27 at the earlier pin `d978d8c8`, 27 again at `e0cdc389`. Same number, different tree, so it is a property of the estate rather than an artefact of one pin.
2. **The baseline really is zero-UNSTABLE.** `burn-baseline.tsv` is 98 rows with a maximum `DEFAULT_FAIL` of 0 and no row above zero. Every one of the 27 is therefore NEW since 2026-08-15, not ambient estate rot. Checked rather than quoted, because the whole claim rests on this denominator.
3. **The fresh-worktree confound does not apply here, and it was checked rather than dismissed.** `INTENT_BIN` defaults to `${INTENT_PROJECT_ROOT}/bin/intent` -- the committed v2 bash CLI -- so these tests need no build output. The previous drive of this area nearly shipped an isolation artefact as a finding, so the absence of the confound is recorded rather than assumed away.
4. **Two whole classes come back green.** `out-of-scope` (20 of 20) and `retire` (7 of 7). An instrument that reported everything red would be indicted by that result; this one is not.

## The partition -- all 98 register rows, one default run each

| class          | green | UNSTABLE | gone | total |
| -------------- | ----- | -------- | ---- | ----- |
| `keep`         | 23    | 8        | 0    | 31    |
| `pending`      | 20    | 19       | 1    | 40    |
| `out-of-scope` | 20    | 0        | 0    | 20    |
| `retire`       | 7     | 0        | 0    | 7     |

**UNSTABLE is confined to `keep` and `pending`.** The one `GONE` row is `tests/unit/intent_claude_upgrade.bats`, which `coverage_map.sh` already reports as a stale baseline row.

## The four causes

| cause | files                              | what it is                                                                      |
| ----- | ---------------------------------- | ------------------------------------------------------------------------------- |
| **A** | 24 outright, plus 1 of 2 in a 25th | `125f601d` deleted five v2 plugin commands and deleted NO tests                 |
| **B** | `config.bats` t7                   | the SAME prune's deliberate inline trips an ST0042 Highlander guard             |
| **C** | `intent_critic.bats` t19           | `.intent_critic.yml` `disabled:` is not suppressing; root cause NOT established |
| **D** | `no_absolute_home_paths.bats` t5   | a scaffolded project has no `.claude/settings.json`; root cause NOT established |

### A -- the cutover prune, and the asymmetry that made it visible

`125f601d` (AC-12.1, 2026-08-30) deleted `intent_claude_prime`, `intent_claude_rules`, `intent_claude_skills`, `intent_claude_subagents` and `intent_claude_upgrade`. **It deleted no tests.** `intent/plugins/claude/bin/` now holds `intent_claude_cwi` and `intent_claude_hook` and nothing else.

Every one of the 24 files fails with the same shape -- `error: Plugin command not found: .../intent_claude_<name>` -- and the failures distribute across exactly the five pruned names. These are v2 conformance tests exercising commands **v3 deliberately does not have**, so they are expired `keep` and `pending` rows rather than broken tests, and **the remedy is a disposition, never a repair.**

Two entries in this class do not present as _command not found_ and are recorded separately so the next reader does not pattern-match past them:

- `plugin_commands.bats` fails because the claude plugin's own inventory went `Commands (7):` to `Commands (0):`. Same commit, different mechanism.
- `ext_discovery.bats` t10 asserts a shadow warning through a bare `[[ ]]` rather than `assert_success`, so it carries none of the family's error text while having the family's cause.

### B -- the prune's own inline, against a guard that predates it

`config.bats` t7 is an ST0042 Highlander check, and it fails on a line **the prune added**:

```
intent/plugins/claude/lib/rules_lib.sh:56    echo "${INTENT_EXT_DIR:-$HOME/.intent/ext}"
```

The prune replaced `source "$INTENT_HOME/bin/intent_helpers"` with an inlined copy of the one symbol that file took, correctly, so the survivor carries no `bin/` dependency. The hunk carries a long comment ending:

> the next node running a duplication sweep will find this and be right to ask (vc, 2026-08-30)

**That prediction was right about what would find it and wrong about who.** It was not a node running a sweep. It was a test that has been in the estate since ST0042 -- and **a test cannot read the comment written to answer it.** The justification is sound, the guard is sound, and nothing joins them.

This is the same class as issue `0199`: **a claim addressed to a human reader, left in the one place only an instrument looks.** Recorded here as a class and not as one row, because the repair for the row (exempt the file, or take the red as a recorded deviation) does not touch the class.

### C and D -- established as NOT-A, and no further

- **C.** `.intent_critic.yml` carrying `disabled: [IN-EX-TEST-001]` does not suppress it; the rule fires four times at CRITICAL and the run exits 1. The fixture lives outside the temporary project the test builds, which makes project-root resolution a plausible mechanism. **Plausible is not established, and it is not recorded as though it were.**
- **D.** A scaffolded project has no `.claude/settings.json`, although `lib/templates/.claude/settings.json` exists.

Both files were green in the committed baseline, so both are regressions rather than long-standing reds. **Neither root-cause commit is named, because neither was measured.**

## What this changes, and what it deliberately leaves

**For the register:** the 8 `keep` rows have no unknowns left in them. All 8 expired at one commit, on one deliberate act, so the reclassification is one decision carrying one reason. It was previously two known causes and six unknowns, which is why it was refused.

**For `AC-06.3`:** eight `keep` rows now measurably differ from v2 behaviour. Under the ruled form -- _a recorded deviation or a filed defect, never silence_ -- that is an obligation, and the row cannot go green while it is unmet.

**Left to others, by ownership rather than by preference:** the register classes are vc's or hv's; **B is dc's**, whose prune and whose comment it is; C and D are unowned and belong to whoever the release scope puts them on.

## Two corrections this read forced on its own author

1. **`plugin_commands.bats`'s mechanism was reported wrong.** It was recorded as failing on the `subagents` prune like its neighbour. It has no _command not found_ in its output at all. The mechanism had been taken from the adjacent file rather than from its own log -- the same first-matching-line error this estate has now recorded three times.
2. **The generalisation was right and would have been wrong in exactly two places.** The earlier refusal to extrapolate from 2 known causes to 8 was correct discipline, and the causes did in fact generalise. **They generalise to 25 of 27 and not to 27**: `config.bats` and `intent_critic.bats` are different causes, and an extrapolation would have buried both under a commit that did not produce them.
