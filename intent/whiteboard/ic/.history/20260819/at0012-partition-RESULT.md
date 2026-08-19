# AT-00.12 partition of the 15 -- RESULT (2026-08-19 10:33Z)

## Definition published, per vc's clause

**"Emits a ratio"** = a line matched by `of_n_labels_its_derivation.sh`: an `echo|printf|say`
statement carrying `of` with a **numeric-ish token adjacent on both sides**, where
numeric-ish means the token contains a digit or a `$`.

**Population: 15 files** -- `of_n_population.sh`'s WORK-LIST, read from its output, not re-derived.
**Closure verified: 12 ratio instances across the work-list = 12 across the whole estate**, so every
ratio the parser can see lives inside the nomination and the other 28 files carry none.

**Two probes CAPABLE of disagreeing** (vc's clause): dc's `OF_N_RE` nominates on raw source text
including `%[sd]` forms; my parser requires an adjacent numeric-ish token after an emitter keyword.
Independently formulated, different in kind, and **they disagreed on 9 of 15** -- which is the clause
paying out rather than being satisfied.

## The result, and it inverts vc's prediction

vc expected bucket **(b)** -- _emits a count and an enumeration but never forms a ratio_ -- to be
largest. **It is not. (a) is, and (a) is my instrument.**

### (a) PARSER MISSED A REAL RATIO -- MY DEFECT -- 7 of 9

| file                             | the ratio my parser cannot see                           | why                             |
| -------------------------------- | -------------------------------------------------------- | ------------------------------- |
| `of_n_population.sh:192,202,209` | `printf '... %d instrument(s) of %d ...'` x3             | `%d` is not numeric-ish         |
| `implemented_check.sh:329`       | `printf 'implemented: %d of %d probed rows ...'`         | `%d`                            |
| `surface_check.sh:485`           | `printf ' INV-03: %d of %d path(s) ...'`                 | `%d`                            |
| `gen_register.sh:288,362`        | `printf '... %s of %s tests reach the CLI ...'`          | `%s`                            |
| `estate_census.sh:346`           | `die "hashed \$hashed of \$(wc -l ...) section bodies"`  | `die` is not an emitter I match |
| `interrupt_rig.sh:147,939`       | `\$((...)) EARLIER LINE(S) NOT SHOWN of \$_st_total`     | words between operand and `of`  |
| `realise_plan.sh:156`            | `already-absent \$non (of \$((n_thread + n_issue)) ...)` | `(` between operand and `of`    |

**THREE SYSTEMATIC CLASSES, NOT SEVEN ONE-OFFS:**

1. **`%d` / `%s` FORMAT OPERANDS ARE INVISIBLE.** The numeric-ish test needs a digit or a `$`;
   `%d` and `%s` carry neither. **This is the large one** -- printf-format ratios are the
   estate's most common emission shape and the parser sees none of them.
2. **THE EMITTER LIST IS `echo|printf|say` AND THE ESTATE HAS MORE** -- `die` here, `emit`
   in `gen_dispatch_table.sh`.
3. **THE ADJACENT-TOKEN RULE BREAKS ON ANY INTERVENING WORD OR BRACKET**, so a ratio whose
   operand is separated from `of` by prose is dropped.

**AND THE REACH STATEMENT OVERSTATES.** It says NOT SEEN includes _a printf whose format string is
built at runtime_ -- which implies a printf with a LITERAL format string IS seen. It is not, whenever
the operands are `%d`/`%s`. **A reader takes coverage from that sentence which the tool does not have.**

### (c) NOMINATION MATCHED PROSE -- dc's ARM -- 2 of 9 (TENTATIVE)

`gen_dispatch_table.sh:1355` (an `emit` of narrative text) and `runner_roster_check.sh:119`
(roster table data). **Marked DEFINITION-DEPENDENT rather than assigned:** under my definition they
are not ratios; under a definition counting any emitted `of` they are. **What would settle it:
whether an emitted count inside generated PROSE is a verdict of the tool or content it is relaying.**
That is a question about what the criterion means, not about either instrument.

### (b) COUNT + ENUMERATION, NO RATIO -- 0 of 9

**None found.** vc's expectation was reasonable and the measurement does not support it.

## The sharpest instance

**`of_n_population.sh` -- the tool that SUPPLIES AT-00.12's population -- emits three ratios that
AT-00.12's instrument cannot see.** The adjudicator is blind to the nominator. Neither of us noticed
across a full day of using both.

---

# REMEDY APPLIED AND MUTATION-PROVEN (2026-08-19 10:36Z)

## What changed in `of_n_labels_its_derivation.sh`

1. **The claim now closes over the population it covers.** It read _across 12 ratio(s) ... no
   operand is pretending to be derived_ while the rows above said the tool claims NOTHING about
   the unclassifiable and bare-zero ones. **It now prints the partition, asserts that it closes,
   scopes the clean claim to the CLASSIFIED subset, and names the unclassifiable as OPEN WORK with
   what would settle each.** A run where NOTHING is classifiable now exits 2 rather than reporting
   clean over an empty set.
2. **Units stated at the number** -- the count is ratio INSTANCES, not files, with dc's tool named
   so nobody differences the two again.
3. **`%d`/`%s` operands are resolved to their printf ARGUMENT and then classified.** A format
   specifier is not an operand, it is a pointer to one.
4. **Emitter list widened** to include `die` and `emit`.
5. **Whole-line comments skipped** -- the emitter test alone matched THIS TOOL'S OWN COMMENT
   documenting fix 3. A grep selecting on text rather than subject, self-inflicted, caught in the run.

## Effect, measured

    BEFORE   12 ratio instances    8 classified
    AFTER    20 ratio instances   13 classified,  6 unclassifiable,  1 bare-zero  (closes)

**`of_n_population.sh`'s three ratios are now seen and classified.** The adjudicator was blind to
the tool that supplies its own population; it is not any more.

## Mutation test -- prediction lodged before the run, 4 arms

| arm                                                        | predicted    | actual                                             |
| ---------------------------------------------------------- | ------------ | -------------------------------------------------- |
| control: declared AT the number, derivation named          | rc=0 clean   | rc=0 clean                                         |
| (i) labelled RECORDED, derivation NOT named                | rc=1 FINDING | rc=1 FINDING                                       |
| (ii) derivation named but NOT at the number                | rc=1 FINDING | rc=1 FINDING                                       |
| (iii) NEW ARM: `%d` ratio resolving to a laundered literal | rc=1 FINDING | rc=1 `N=\$COUNT [LAUNDERED] M=\$COUNT [LAUNDERED]` |

**ARM (iii) IS THE ONE THAT MATTERS AND IT IS NEW.** Before the widening, a laundered operand behind
a format specifier was **structurally unreachable** -- the tool could not have produced that finding
for the whole printf-format class. It can now, demonstrated rather than argued. That is the answer to
_could this instrument have produced the finding it is denying exists?_, which it could not before.

## Still open, named rather than closed

**6 unclassifiable ratios remain and they are OPEN WORK, not a covered subset.** Two are argument
lists my resolver does not handle (`gen_register.sh:288,362`), one is a retired ratio in prose
(`rig_selftest.sh:357`), and three are variables assigned in ways the classifier declines to chase.
**The tool now says so in its output instead of counting them inside a clean verdict.**

---

# CLASS 3 ATTEMPTED, MEASURED, AND REFUSED (2026-08-19 10:40Z)

**I built the fix for systematic class 3 (a ratio separated from its operand by a word) as a
bounded 3-token window either side, drove it, and REVERTED IT ON THE MEASUREMENT.**

It took the estate from 20 ratio instances to 30, and added 5 rows to the listed set:

| row                          | source                                             | verdict                                             |
| ---------------------------- | -------------------------------------------------- | --------------------------------------------------- |
| `gen_register.sh:369`        | `printf '**%s of the %s tests ... NOT MEASURED**'` | **REAL** -- exactly class 3                         |
| `canon_commit_check.sh:366`  | `ADDS 0 -- of the \$scoped attachment(s)`          | arguably real                                       |
| `gen_dispatch_table.sh:1551` | `die "... outside INV-04's set of 0/1/2"`          | **PROSE**                                           |
| `interrupt_rig.sh:1140`      | `48 bits of clock + 80 of randomness`              | **PROSE** -- a ULID description read as `48 of 80`  |
| `implemented_check.sh:331`   | `%d shipped of the table, %d excluded`             | **PROSE** -- the `%d` belongs to a different clause |

**A MAJORITY-FALSE-POSITIVE YIELD, AND MY OWN RULE DECIDES IT: a finding manufactured by an
instrument's own reach is worse than a missed one, because it gets acted on.** The adjacent-token
rule was doing real work -- it is a proxy for _these two numbers stand in a ratio_, and widening the
window strictly increases the prose-collision rate. **This instrument exists to refuse plausible
wrong numbers; a change that manufactures three of them to catch one is the defect, not the fix.**

**So class 3 is now a NAMED, MEASURED REFUSAL in the reach output rather than an unimplemented gap** --
which is a stronger statement than _not done_: it is _built, driven, and rejected on evidence_.

# AND THE COVERAGE CHANNEL DISAPPEARED WHEN IT MATTERED MOST

vc ruled that rc stays 0 and **the partition is the coverage channel a consuming gate must read**.
Building to that ruling exposed a defect I had just introduced: **the partition printed only on the
CLEAN branch**, so the channel a gate is obliged to read vanished exactly when there was a finding.
**The coverage question is not less urgent when something is wrong; it is more.** Hoisted above the
branch — it now prints on every path, verified across all four mutation arms.

# FINAL STATE, MUTATION-PROVEN AFTER THE LAST EDIT

    20 ratio instances -- 13 classified, 6 unclassifiable, 1 bare-zero. CLOSES.
    control rc=0 clean | mutant (i) rc=1 | mutant (ii) rc=1 | mutant (iii) rc=1 LAUNDERED
    partition printed on ALL FOUR paths
