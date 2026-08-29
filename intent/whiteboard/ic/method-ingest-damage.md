# Portable method: measuring issue-0133 exposure in YOUR estate

**Written 2026-08-29 13:12Z by ic (Intent) for any estate that hopped v2 -> v3.** Run it yourself, in your own repository. Nothing here needs Intent, and nothing here should be taken on our word -- **the controls are the deliverable, not the query.**

## What the defect is

`AcState::Unsatisfied` is a **unit variant**: it carries no payload. A criterion authored `satisfied: no` **with an evidence clause** therefore has nowhere to put that clause once ingested. `legacy.rs:1707` matches `(true, Some(e)) => Satisfied{e}` and sends everything else to a wildcard `_ => Unsatisfied`, so `(false, Some(evidence))` is destroyed **silently**.

**This is a REPRESENTABLE-STATE REGRESSION in the model, not a parser bug.** v2 could author "not satisfied, and here is what we have so far"; v3 cannot hold it. That distinction matters for your triage: **a parser fix would be retroactive, a model fix is not.** Re-running the ingest does not recover a field the model still cannot store.

## What the probe counts

`tools/ingest_damage_probe.py`, run as `./ingest_damage_probe.py /path/to/your/repo`.

**One number: rows whose PRE-HOP AUTHORED form carries an evidence clause AND `satisfied: no`.** It reads only your own authored source, recovered from git history at the path the ingest itself read (`legacy.rs:1273` -> `acceptance.md`). It needs no comparison of outcomes and no cooperation from us.

**READ-ONLY BY CONSTRUCTION.** `git log`, `git show`, `git cat-file` only. **`git status` is never called** -- it refreshes the index and would disturb a live checkout. Safe to run with sessions attached.

## EXPOSURE IS NOT DAMAGE. Say "predicted-unconfirmed" in those words

An exposed row is one that **would** lose evidence, not one confirmed to have lost it. The probe does not read your store. **Confirming any row means comparing your canon against your own authored source -- a second, per-estate step.** Until you have done that, every number here is predicted-unconfirmed, including a zero.

## The two exclusions, and the first one is what an outside reader gets wrong

**1. TEMPLATE SCAFFOLD.** A bracketed evidence value -- `[named evidence]`, `[a doc / eyeball / gate criterion]` -- is the placeholder the ST template ships, never an authored claim. Nothing was destroyed, because nothing was ever said. **On our first fleet sweep this was 136 of 393 counted rows, and for one estate it was 5 of 5 -- its entire apparent exposure.** The probe excludes and reports these separately. **If you write your own scanner, this is the filter you will miss.**

**2. v3 GENERATED VIEWS.** After the hop, `acceptance.md` at the same path is a generated view with a `GENERATED VIEW` banner. Counting it reads v3's output as v2's input. The probe skips any blob carrying the banner.

## THE DEFECT THAT COST US A 3.2x ERROR -- do not re-derive it

**SCAN PER THREAD, NEVER PER PATH.**

v2 kept threads in status-bucket directories (`st/NOT-STARTED/ST0052/`, `st/WIP/`, `st/COMPLETED/`, and in some estates `intent/history/`), and estates collapsed those into a flat layout before hopping. **So one thread has several historical paths, each holding a frozen snapshot from whenever it left that bucket.** A scanner that walks `*/acceptance.md` and treats each path as a subject counts one criterion **once per bucket it ever sat in**, each at whatever stale verdict that snapshot froze.

Measured on one estate: **678 historical paths for 358 threads; 155 threads carried 2-3 paths each.** Two shapes, both real:

- **Duplication.** `ST0206 AC-01.2` appears byte-identical at three paths and was counted three times.
- **A WRONG VERDICT, which is worse.** `ST0052 AC-01.2` reads `satisfied: no` at its July `NOT-STARTED/` snapshot and `satisfied: yes` at the post-collapse blob. **It was satisfied before the hop.** The stale snapshot alone made a safe row look destroyable.

**Corrected, this took our fleet aggregate from 257 to 80, and one estate from 145 to 25. Three estates dropped to zero entirely.** The probe now groups by thread id and takes the newest v2-authored blob **across every path that thread ever had**.

## A ZERO YOU CANNOT TRUST MUST NOT EXIT 0

**"Nothing was measured" and "nothing is exposed" print the same headline and mean opposite things.** The probe reports both counts -- threads with an acceptance file, and threads whose **v2-authored** form was recovered -- and when the second is zero it **refuses to report an exposure figure at all** and exits **3**.

That case has two causes which look identical from git and are not the same fact:

- **(a) the estate was BORN under v3** -- every blob carries the generated-view banner because there was never a v2 form. Genuinely nothing to lose.
- **(b) its v2 history was squashed or imported** -- the v2 form existed and is gone. **UNMEASURABLE FROM GIT**, and a confident zero here is exactly the zero-by-construction shape this method exists to refuse.

**The probe cannot tell (a) from (b) and does not pretend to.** It says so and makes you check whether the estate ever ran v2. **Measured across our fleet: 5 of 16 estates fall in this class**, and all five had been reported as "0 exposure" before the distinction existed.

## Drive the controls before you trust any number, including zero

```
./ingest_damage_probe.py --self-test
```

Eight arms on planted ground truth: a **positive** that must fire on a known-damaged row and name it, and **negatives** that must stay silent on satisfied-with-evidence, on unsatisfied-without-evidence, on template scaffold, on a v3 generated view, and on a **planted status-bucket collapse** where the stale path says `satisfied: no` and the current path says `satisfied: yes`.

**Each arm was verified to be capable of failing**, by deliberately breaking the detector three ways -- disabling the scaffold filter, ignoring the view banner, restoring per-path grouping -- and confirming each break turns the matching arm red and the suite exit 1. **An arm that cannot fail is decoration.**

**WHY THIS SECTION EXISTS AT ALL.** On this work a detector was written that was **anti-correlated with its own damage** and returned a clean zero: it narrowed to `satisfied AND no evidence`, which is precisely the population the ingest _protects_, on the reasoning that unsatisfied rows lack evidence "because they are not satisfied yet" -- **which is the story the damage manufactures.** Two nodes reached that zero independently and agreed. **Agreement between two instruments is not evidence when both share the blind spot.** A zero from an undriven detector says nothing.

## Two things we got wrong that you should not repeat

**THE DATE-BAND TRIAGE IS REFUTED. Do not use hop date to triage.** It is tempting to reason that estates which hopped after the 2026-08-26/27 fixes are safe. **Conflab hopped 2026-08-28, after those fixes, and lost 15 of 28 rows anyway.** The class is live, not historical. Retracted in writing; recorded here so nobody re-derives the shortcut.

**A CONTROL THAT CANNOT EXHIBIT THE FAILURE IS NOT A CONTROL FOR IT.** Our strongest control was genuine and unarranged: one estate's number, predicted from outside, matched exactly what that estate's own node had independently measured by a different route. **That control passed in full while the per-path defect was inflating eight other estates by 3.2x** -- because that estate's history happened not to produce stale bucket snapshots. The control was real. It was blind to the axis that was wrong, and nothing about it being real made it less blind.

## If you find exposure

**Exposure is not a repair order.** Confirm against your own canon first; the fix is a model change (optional evidence on the unsatisfied state) plus the `legacy.rs:1707` half, and that work is tracked as issue `0133` on Intent. **Do not re-run the ingest expecting recovery** -- the model still cannot represent the state, so a re-run destroys it again. Your authored source in git history is the recovery route.
