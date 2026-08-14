# Lamplight AT-row baseline, taken before the v2.19.0 consumer sweep

**Taken**: 2026-08-14T10:51Z, by vc
**Subject**: `~/Devel/prj/Lamplight` at git `15dbccc92`, Intent 2.18.0 (pre-`intent upgrade`)
**Why**: a sweep that rewrites an estate must be measured against git before it is trusted. The v2.19.0 `at lint --fix` sweep destroyed 87 test-name links in Intent's own contracts and nobody noticed for a day; the loss was recoverable only because `f28938c^` still held them. This is the count to diff against after cc runs the sweep.

## The estate

| Measure                    | Count    |
| -------------------------- | -------- |
| `acceptance.md` files      | 149      |
| of those, carrying AT rows | 97       |
| **AT rows total**          | **1639** |

**Our own notes said 314 AT rows. That figure is stale by a factor of five** and should not be used for planning. The CHANGELOG's `[2.19.0]` entry cites "a consumer estate of 1642 AT rows" — that is this same estate, measured a day or two earlier, and it reconciles.

## What the rows carry today

This is the half that matters: a lossy sweep destroys these, and the loss is invisible unless it was counted first.

| Shape                                 | Rows           | `--fix` behaviour                                               |
| ------------------------------------- | -------------- | --------------------------------------------------------------- |
| Backticked reference of any kind      | 616            | varies                                                          |
| `path::"name"` citation               | 975            | **REFUSES** — two-ended migration                               |
| Multi-file `pathA + pathB` citation   | 508            | **REFUSES** — grammar admits one file                           |
| Both shapes on one row                | 325            | refuses                                                         |
| **Union: shapes `--fix` must refuse** | **1158 (70%)** | reported by name, never guessed                                 |
| Remainder: candidates for `--fix`     | 481            | of which 268 migrate mechanically (CHANGELOG's measured figure) |

**Expect roughly 70% of this estate to be reported as residue needing a human.** That is the fix working, not a failure — every one of those rows was already contributing no verifiable coverage. But it is a large number and whoever runs the sweep should know the scale before, not after.

## AT status vocabulary — healthier than our notes suggested

AT-row-scoped, 1639 rows:

- **in vocabulary** (`green` / `red` / `to-write` / `n/a`): **1630**
- **out of vocabulary**: **9**, across two values — `green.` (8, a trailing full stop) and `:degraded\`` (1)

**Correcting our own record**: `intent/wip.md` and the whiteboard boards named "four known bad-status contracts (ST0276 `**green` x11, ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`)". Measured today: ST0276 has no `acceptance.md` at all; ST0298, ST0270 and ST0198 do carry non-vocabulary `status:` values (`BUILT`, `Done`, `Done;`, `WIP`) **but those are on AC rows, not AT rows** — a different state model and a different question, outside `at lint`'s L1. The AT-row picture is 9 rows, not four contracts' worth.

A first pass of this measurement reported "30+ distinct status values" and was wrong: the grep was not AT-row-scoped and was counting AC rows and prose. Recorded because the corrected number is the one to act on, and because an unscoped grep producing an alarming number is exactly the kind of finding that gets acted on before it is checked.

## How to measure the delta after the sweep

Re-run the same AT-row-scoped counts and require:

1. **`AT rows total` unchanged at 1639** — a sweep must not delete rows.
2. **`::name` and multi-file counts DROP ONLY where a name survived into a trailing note.** A row that loses its `::name` without gaining the name somewhere else is destroyed data, not a migration. This is precisely what happened to Intent's own 87 rows.
3. **`backticked reference` count does not fall.** A row that had a reference and now has none has lost its only link.

Anything that fails 2 or 3 is recoverable from git at `15dbccc92` — but only if someone looks.

## Method

Counts are AT-row-scoped (`^- AT-\d`), taken with a Python pass over `intent/st/**/acceptance.md`, not with line-oriented greps over whole files. That distinction produced the corrected status figure above and is the reason the numbers here should be reproduced the same way.
