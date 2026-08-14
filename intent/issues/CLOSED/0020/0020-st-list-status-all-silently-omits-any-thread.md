---
id: "0020"
title: st list --status all silently omits any thread whose status is not one of ten hardcoded literals, and sync --write now composes that view into the canonical index
date: 2026-08-14
reporter: matts
status: CLOSED
severity: medium
---

# 0020: st list --status all silently omits any thread whose status is not one of ten hardcoded literals, and sync --write now composes that view into the canonical index

## Tags

steel-threads, listing, index, silent-failure, vocabulary

## Summary

`intent st list --status all` does not show all threads. The `all` branch iterates a **hardcoded ordered list of ten status literals** and emits only rows whose status string matches one of them exactly. A thread whose `status:` is anything else is dropped from the output with no diagnostic, no count, and no exit-code change -- so a view that names itself `all` is silently partial.

Since issue 0019, `intent st sync --write` composes exactly this view into `intent/st/steel_threads.md`, the canonical committed index whose own preamble says it holds every steel thread. **The omission therefore now propagates from a transient view into tracked project state.** That is the same defect 0019 just fixed one layer up, surviving at the layer below it.

Found on Laksa while relaying a different report (a node read an empty `st list` as a broken reader; the empty table was in fact the correct WIP-only default, and this turned up on the way to proving that).

## Reproduction

Measured on Laksa, 2026-08-14, against Intent at `77e40fc`:

```
$ find intent/st -name info.md -maxdepth 3 | wc -l
96                                     # thread documents on disk

$ intent st list --status all | grep -cE '^ST[0-9]{4}'
94                                     # rows the "all" view emits

$ find intent/st -name info.md -maxdepth 3 | xargs grep -l '^status: \(SUPERSEDED\|COMPLETE\)$'
intent/st/COMPLETED/ST0048/info.md     # status: COMPLETE
intent/st/COMPLETED/ST0008/info.md     # status: SUPERSEDED

$ intent st list --status all | grep -cE '^ST0008 |^ST0048 '
0                                      # both absent, silently
```

Two threads, both real, both with a correct `info.md`, both resolvable by `intent st show`, both invisible to the view that claims to show everything. Exit code 0 throughout.

## Root Cause

`bin/intent_st`, the `list` command's `all` branch:

```bash
if [ "${STATUS_FILTER[0]}" = "all" ]; then
  declare -a status_order=("WIP" "In Progress" "TBC" "Not Started" "HOLD" "On Hold" "COMPLETED" "Completed" "CANCELLED" "Cancelled")
  for group_status in "${status_order[@]}"; do
    for line in "${st_data[@]}"; do
      line_status=$(echo "$line" | cut -d'|' -f3)
      if [ "$line_status" = "$group_status" ]; then
        status_group+=("$line")
      fi
    done
    ...
```

The list is a **presentation ordering** being used as a **membership test**. Rows are collected by walking the allowlist and matching exactly; a row is emitted only if its status appears in the array. Nothing iterates the leftovers, so a status outside the vocabulary is not merely mis-ordered -- it is absent.

Two independent gaps compound it:

1. **`canonical_status` is not applied on this path.** It exists precisely to fold synonyms (`complete` -> `Completed`), and it would have rescued `COMPLETE`. The comment on the branch says "we need exact match since status_order has both forms", which is the workaround that made the normaliser redundant here and the vocabulary brittle instead.
2. **A genuinely unknown status has no home.** `SUPERSEDED` is not a synonym of anything, so even correct normalisation would leave it unplaced. There is no "everything else" group, and no report that anything was skipped.

## Impact

- **The canonical index is silently incomplete.** After 0019, `steel_threads.md` is regenerated from this view on every `st done` / `st new` / `sync --write`. A thread with an out-of-vocabulary status disappears from tracked project state and stays gone, with the regeneration reporting success.
- **The failure is invisible in exactly the way that matters.** No warning, no count, exit 0. The only way to notice is to independently count `info.md` files and compare -- which is how this was found.
- **It reads as a reader fault.** A node hitting a short or empty listing has no way to tell "correctly filtered" from "silently dropped", and both were live on the same estate on the same day.
- Estate exposure is small but real: 2 of 96 threads on Laksa. Intent's own estate is unaffected (all 55 threads carry vocabulary statuses), which is why it survived the v2.19.0 sweep.

## Proposed Fix

1. **Route the `all` branch through `canonical_status`** rather than exact literal matching, so the synonym table has one home and `COMPLETE` / `complete` / `Done` all land in the Completed group. The dual-form entries in `status_order` (`COMPLETED` **and** `Completed`, `CANCELLED` **and** `Cancelled`) then collapse to one each.
2. **Give the leftovers a group.** After the ordered groups are emitted, emit every remaining row under an "Other" heading rather than discarding it. `all` must mean all: a view that cannot classify a row still has to show it.
3. **Say so.** Warn on stderr, naming each thread whose status is outside the vocabulary and quoting the vocabulary -- the same treatment `intent at lint` gives an out-of-vocabulary AT status (issue 0007's precedent). The row appears AND the anomaly is reported; neither substitutes for the other.
4. **Guard**: a fixture with one out-of-vocabulary status must appear in `--status all`, must appear in a `sync --write` index, and must be named in a warning. Mutation-check by restoring the allowlist match.

Deliberately NOT proposed: widening the vocabulary to include `SUPERSEDED`. Whether that is a real status is a separate question for the project that wrote it; the tool's job is to stop silently discarding rows it does not recognise.

## Related

- 0019 -- the canonical index stops being an index of whatever is in flight. That fix made this one consequential: the omission now reaches committed state.
- 0007 -- an out-of-vocabulary AT status was silently read as not-green. Same shape, different reader; its resolution (report the row, name the vocabulary, never guess) is the precedent for item 3 above.
- 0002 -- `canonical_status` relocated to `bin/intent_helpers` as the single synonym table. This path bypasses it.

## Resolutions

**Fixed in v2.19.0 (before the cut, on hv's instruction), `bin/intent_st` `list` command, `all` branch. Filed by matts, executed by vc.**

All four proposed items taken as filed, and the "deliberately NOT proposed" exclusion honoured -- the vocabulary is unchanged and `SUPERSEDED` remains unrecognised, it is merely no longer discarded.

1. **Membership goes through `normalise_status`.** That is the derived filter-comparison seam over `canonical_status` (`bin/intent_helpers:535`), and it is the same comparison the multi-status branch sitting immediately below this one already used -- the two branches of one algorithm had disagreed about how to compare a status. `status_order` collapses from ten literals to the five canonical tokens `WIP TBC HOLD COMPLETED CANCELLED`, which is the identical presentation ordering with the synonym pairs folded, so no listing reorders.
2. **The unplaced rows are emitted after the ordered groups**, reverse-sorted by id. There are no group headings in this view -- `render_table` renders a flat row list and the groups only ever controlled order -- so "Other" is a position, not a heading.
3. **The anomaly is named on stderr** through `warning()` (the lowercase voice, `8aba5ab`), quoting the vocabulary and then each offending id and its status. Exit stays 0: `sync --write` composes this view, so escalating would break index regeneration on precisely the estates that have the problem.
4. **Guard: `tests/unit/st_list_all_vocabulary.bats`**, seven tests over a four-thread fixture spanning the four cases (vocabulary status, second vocabulary status to pin ordering, synonym, unrecognised).

### Verification

**The defect was reproduced before it was fixed, and the repro was proved to measure the right thing.** A scratch project with three threads -- `Not Started`, `COMPLETE`, `SUPERSEDED` -- run against pre-fix code in a throwaway `git worktree` at `fae90dc`: **1 row emitted of 3 on disk, exit 0**, both off-vocabulary threads gone. The same project against the fix: 3 of 3, the synonym placed in its canonical group, the unrecognised row shown last and named on stderr. `sync --write` then wrote 3 index rows for 3 threads on disk, which is the half that reaches committed state.

**Mutation battery, run in a sacrificial worktree so `bin/` was never mutated in place** (`~/.local/bin/intent` symlinks into this repo, so an in-place mutation is live for every other project on the machine). Each mutation applied by exact string replacement, verified applied, syntax-checked, and restored between runs:

| Mutation                                    | Kills                | Reading                                          |
| ------------------------------------------- | -------------------- | ------------------------------------------------ |
| M1 exact-match membership restored (the bug) | 1, 3, 6, 7           | synonym drops; unrecognised still caught by the pass |
| M2 unplaced pass deleted                     | 1, 4, 5, 6, 7        | unrecognised drops; synonym still placed         |
| M3 warnings silenced                         | 5 only               | the stderr test is not vacuous                   |
| M4 presentation order permuted               | 7 only               | the ordering test genuinely pins order           |
| M5 `warning` escalated to `error`            | all 7                | the exit-0 test is not vacuous either            |

M1 and M2 killing complementary sets is the load-bearing result: it shows the two halves of the fix are independently necessary, and that no test is passing for the other half's reason.

**A first attempt at this battery produced a false reading** and is recorded because the correction is the point. M1's `perl` substitution failed to match (shell-metacharacter escaping), the `&&` chain therefore skipped the restore step, and M2 ran on top of a half-mangled file -- reporting that deleting the unplaced pass also broke synonym placement, which it does not. The result was incoherent on its face (M2 cannot affect normalisation), which is what prompted looking instead of believing. Every mutation now hard-fails if the source is unchanged after substitution.

**Collateral:** the eleven test files touching `st list` / `st sync` / `steel_threads.md` plus `helpers.bats` and `set_e_increment_guard.bats` all green; both integration decks green. This repo's own estate still lists 55 rows for 55 threads on disk and emits no warning, confirming the issue's own assessment that the home estate is unaffected.

### Related work not done here

`bin/intent_st:731-741` computes a `CREATED` value in the in-progress arm that no code path reads (the readers at 843/885/914/956 all recompute it locally inside `sync`). It is the residue of the arguments issue 0019 pruned from `update_steel_threads_index`, and it is dead rather than wrong. Left for a tidy after the tag.
