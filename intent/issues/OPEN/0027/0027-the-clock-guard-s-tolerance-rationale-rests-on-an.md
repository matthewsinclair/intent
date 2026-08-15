---
id: "0027"
title: the clock guard's tolerance rationale rests on an error-distribution claim a measured incident falsifies
date: 2026-08-15
reporter: matts
status: OPEN
severity: low
---

# 0027: the clock guard's tolerance rationale rests on an error-distribution claim a measured incident falsifies

## Tags

hooks, whiteboard, clock, guard, measured

## Summary

`lib/templates/hooks/whiteboard-clock-guard.sh` sets `TOLERANCE_SECONDS=120` for check A (a stamp must not postdate the commit that adds it) and justifies it at `:109-113`:

> TOLERANCE: 120s, check A only. Stamps are minute-granular, so one written at 14:59:50 and committed at 15:00:05 is honest. **The errors this catches are +7 minutes and worse, so the tolerance costs nothing.**

**"The errors this catches are +7 minutes and worse" is an empirical claim about the error distribution, and there is now a measured counter-example at exactly +2 minutes** -- which the tolerance passed, at zero margin.

## Reproduction

Measured 2026-08-15 on a real incident, not a constructed one.

vc fabricated a whiteboard stamp of `09:52Z` on an entry to `cc/inbox.vc.md`. The last clock read before writing it was `09:45Z`; the next real read was `09:50Z`. The commit carrying it landed at approximately `09:50Z`.

```
stamp        09:52Z
commit       ~09:50Z
drift        +120s
guard test   [ "$drift" -gt "$TOLERANCE_SECONDS" ]   ->  120 > 120  ->  false
result       PASSED, with zero margin
```

The comparison at `:189` is strictly greater-than, so a drift of exactly the tolerance passes.

## Root Cause

Not a coding defect -- the guard does what it says. The defect is in the **justification**: the tolerance was sized against an assumed error distribution ("+7 minutes and worse") rather than a measured one, and the first documented instance of a fabricated stamp on this project landed at +2 minutes, inside the allowance.

Why a legitimate stamp should rarely need this allowance at all: check A compares a minute-granular stamp against the commit time. A stamp read from `date -u` is always taken BEFORE the commit, so honest drift is normally negative. The `14:59:50` example in the comment is itself a negative-drift case (stamp `14:59Z`, commit `15:00:05`) and therefore does not motivate a positive allowance. The 120s appears to be defensive slack whose size was never tested against a real error.

## Impact

Low, and bounded by three things the guard already states.

The guard's own header says it is **not closed** -- "a fabricated stamp that carries a `Z`, lands in the past, and still increases monotonically passes all three checks. Smaller target, not an empty one." So a gap is not a surprise; what is new is a measured boundary and a falsified rationale.

Check C (an append-only inbox going backwards) is exact and needs no clock, and remains the two-sided test. The commit history remains the trustworthy ordering.

**This matters only until D30/WP-14 lands.** Under D33 the API becomes the only writer of a timestamp and this guard demotes from primary defence to a legacy-file check, at which point the tolerance stops being load-bearing. Until then D33 explicitly holds the guard at full strength, so the window is real rather than theoretical.

## Proposed Fix

Options, for whoever owns the guard -- deliberately not chosen here:

1. **Tighten** `TOLERANCE_SECONDS` (60s would have caught the measured instance) and update the rationale to cite measurement rather than an assumed distribution.
2. **Change the comparison** to `-ge` so drift exactly at the tolerance fails, which alone would have caught this instance.
3. **Leave the value and fix the comment**, replacing the "+7 minutes and worse" claim with the measured +2 minute counter-example, so the next person sizing it is working from evidence.

Whatever is chosen, the sentence stating the error distribution should not survive unamended: it is the part that is now known to be wrong.

## Related

- ST0040 / ST0045 -- the whiteboard protocol and its clock rule
- D33 (ST0056) -- hv's ruling that no node ever authors a timestamp; ends this class by construction and deletes these rules once WP-14 lands
- Raised by cc, whose framing is the reason it was filed: a measurement of the guard's sensitivity belongs with the guard's owner rather than inside the offender's self-assessment.

## Resolutions

{{TBC}}
