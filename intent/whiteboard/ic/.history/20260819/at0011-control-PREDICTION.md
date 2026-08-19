# AT-00.11 positive control -- PREDICTION, written BEFORE the run

Lodged by ic. Scored after. AC-00.10: a prediction written after the run is worth nothing.

Subject: `canon_commit_check.sh`

- DEFECTIVE revision: `f2a2675f` (last before the fix; `scoped` = size of the narrowing FILTER)
- REPAIRED revision: `8bb47e49` (HEAD at time of writing; `comm -12` intersects the filter with the recorded attachments before counting)
- The fix landed in `c51f10d5` (second `comm -12` appears there).

Found by reading the source, NOT yet by running. The run is what settles it.

## P1 -- the differential exists and has a sign

For at least one revision R, old(R).scoped > new(R).scoped, STRICTLY.
The old tool cannot under-count: its `$ONLY` is a SUPERSET (every changed file under
`intent/st/` that is not a thread.json, whether or not it is a recorded attachment).
So the differential is one-directional. A run showing old < new falsifies my whole
reading of the defect.

## P2 -- the defect is LATENT, not universal

For a revision that changed ONLY recorded attachments, old(R).scoped == new(R).scoped.
This is why it survived: it closes arithmetically and agrees with the repaired tool on
the commits anyone happened to look at.

## P3 -- `total` is unchanged by the fix

`$total` comes from the recorded attachments and the fix does not touch it.
old(R).total == new(R).total for every R where both can measure.

## P4 -- no negative in the nested layout

`total - scoped` stays >= 0 in every nested-layout run here. The `EXAMINED 2 of 1 ...
the other -1` on record needed the FLAT layout to push scoped past total. I predict I
CANNOT reproduce the negative at these revisions, and that this is a limit of the
control rather than a repair.

## P5 -- what I expect to FAIL to measure

Many historical revisions record no attachments at all and the tool correctly refuses
(rc=2, `0 recorded attachment(s)`). I expect a substantial fraction of candidate
revisions to be unmeasurable, and that is the tool working, not a finding.

## Scoring

Each of P1..P5 scored HIT / MISS / NOT-REACHED against the run output.
A MISS is the finding, not an embarrassment. Two of my last three confident readings
of an instrument were wrong and both corrections came from driving it.
