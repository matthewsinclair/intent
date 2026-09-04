# Scope sweep: recorded `scope:` against remaining work, ST0056

**Commissioned by vc 2026-09-04, re-framed by vc mid-run to run BOTH directions. Driven by ic. Instrument positive-controlled before use.**

## The headline, and it is about the instrument rather than the WPs

**NEITHER AC-COUNT NOR AC-TEXT-WEIGHT MEASURES REMAINING WORK, AND THE SWEEP PROVES IT ON ITS OWN FOUNDING CASE.**

`WP-17` is what commissioned this: 17 of 18 rows satisfied, and vc established that its single remaining row (`AC-17.6`) carries the `browse` arm, the ES module client, the form-description consumption and the Conflab bridge port -- most of the package. So I added text-weight as a second proxy to catch exactly that shape.

**It does not catch it.** `AC-17.6` is 2,547 characters. That ranks it THIRD among the few-rows-left group, behind `WP-12` (10,276) and `WP-04` (3,451). **The proxy under-ranks the one case we independently know is heavy.**

So the sweep can find DISAGREEMENTS mechanically and cannot adjudicate them. What read `WP-17` correctly was reading the row and knowing the design behind it -- judgement, not an instrument.

## Instrument and control

Satisfaction is resolved the way `intent ac list` resolves it (`state.is == satisfied`, or `computed` with every covering AT `green`/`n-a`). **Positive control before use: the instrument was required to reproduce `WP-17 = 18 rows / 17 satisfied`, independently known from `intent ac list`. It agreed.** Mapping control: `seq` 5 and 16 were confirmed against `intent wp show ST0056/05` and `/16` by title. Zero AC ids failed to parse.

## The table

| WP  | scope | status      | left |  of | unsat chars | title                                    |
| --- | ----- | ----------- | ---: | --: | ----------: | ---------------------------------------- |
| 00  | -     | -           |    9 |  16 |      28,076 | thread-level rows, no WP record          |
| 01  | L     | done        |    0 |   4 |           0 | Design canon                             |
| 02  | L     | done        |    0 |   8 |           0 | Workspace and reified model              |
| 03  | L     | done        |    1 |  17 |       1,910 | Ingest, views and sync engine            |
| 04  | XL    | wip         |    1 |   6 |       3,451 | intentsvcs facade: core command families |
| 05  | L     | wip         |    0 |   7 |           0 | CLI in-process mode and BATS harness     |
| 06  | XL    | wip         |    3 |  13 |       7,137 | CLI parity long tail                     |
| 07  | L     | wip         |    1 |   7 |       1,151 | Canon and claude subsystem               |
| 08  | XXL   | done        |    0 |  12 |           0 | intentd daemon                           |
| 09  | L     | done        |    0 |   6 |           0 | MCP server and agent guide               |
| 10  | XL    | wip         |    1 |  15 |       1,909 | Migration and fleet ingest harness       |
| 11  | M     | wip         |    3 |   7 |      11,884 | Distribution: cargo-dist, Homebrew       |
| 12  | L     | not-started |    2 |   4 |      10,276 | Cutover and v3.0.0 release               |
| 13  | XL    | not-started |    9 |   9 |       1,481 | Project search                           |
| 14  | L     | not-started |   12 |  12 |       4,548 | Coordination model                       |
| 15  | L     | not-started |    4 |   4 |       1,664 | Skills catalogue triage                  |
| 16  | S     | not-started |    4 |   4 |       2,375 | Contract drift                           |
| 17  | XL    | wip         |    1 |  18 |       2,547 | Form DSL: TUI and WEB realisers          |

## Findings

**THE `WP-17` SHAPE IS NOT UNIQUE -- BIG SCOPE, ONE ROW LEFT, HEAVY REMAINDER OCCURS THREE TIMES: `WP-04`, `WP-10`, `WP-17`.** Anyone reading those as nearly-done is making the reading vc nearly made about `WP-17`. **I have NOT adjudicated `04` or `10`; I am flagging the shape, not the verdict.** Their owners are the ones who can say.

**ONE CLEAN STALE STATUS, AND IT IS A `WP-08`-SHAPED CLOSE: `WP-05` IS `L` / `wip` WITH 0 OF 7 ROWS REMAINING.** Every criterion satisfied, zero unsatisfied text, status still `wip`. Confirmed by the independent route (`intent ac list ST0056`, all seven `AC-05.*` read `satisfied: yes`). **It is dc's, and it is theirs to close, not mine.**

**`WP-11` IS THE OTHER DIRECTION AND THE SWEEP CAUGHT IT ONLY BY WEIGHT: `M` with 3 of 7 left and 11,884 characters unsatisfied** -- the heaviest remainder of any live package bar the thread-level rows, under the smallest live scope. The count alone would not have raised it.

**CONTROLS BOTH FIRE, which is what makes the sweep able to say anything.** Scope and remaining work AGREE on `WP-06` (XL, 3 of 13 left), `WP-13` (XL, 9 of 9), `WP-14` (L, 12 of 12), `WP-15` (L, 4 of 4). And `WP-16` runs the other way: `S` with 4 of 4 left, so scope UNDERSTATES there. **A sweep that only found stale scopes could not distinguish _scopes go stale_ from _scopes are always big_; these four say the field is right more often than it is wrong.**

## What this sweep cannot see

- **Which side of a disagreement is wrong.** It finds disagreements. Adjudicating one means reading the remaining rows and knowing the design behind them, which is judgement.
- **Remaining work.** Both proxies are proxies. AC-count fails optimistically (`WP-17`); text-weight fails on the same case.
- **Work with no AC row at all.** Anything not expressed as a criterion is invisible here, in every direction.
- **`WP-00`'s 9 remaining rows** are thread-level and have no WP record, so they carry no scope to disagree with. They are in the table for completeness and out of every finding.
- **Whether a `status` of `wip` versus `not-started` is itself accurate.** Taken as recorded, not verified.
