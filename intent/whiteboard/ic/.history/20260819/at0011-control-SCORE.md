# AT-00.11 positive control -- SCORE, written AFTER the run

Scored against PREDICTION.md, which was lodged before any tool was driven.
Subject: canon_commit_check.sh, DEFECTIVE f2a2675f against REPAIRED 8bb47e49.
Run: 120 revisions. DRIVEN 120: 17 overstated + 61 agreed + 42 no-verdict + 0 unreadable = 120. CLOSES.
SCORED 78 of 120 -- 42 refuse at rc 2 and are not scoreable either way.

P1 differential exists, strictly one-directional HIT 17 overstate; ZERO rows where repaired > defective, across all 78 scored.
P2 defect is LATENT, not universal HIT 61 of 78 scored rows agree exactly.
P3 `total` unchanged by the fix HIT identical M on both sides in every scored row.
P4 no negative remainder in the nested layout HIT and it stands as a LIMIT of this control, not a repair.
P5 a substantial fraction unmeasurable HIT -- BUT SEE BELOW. Scored MISS at 10 revisions, HIT at 120.

## P5 is the finding, and it is about the scoring instrument rather than the subject

I scored P5 a MISS on ten revisions and reported that MISS to vc, who relayed it onward
as a fact. At 120 revisions, 42 refuse at rc 2 and the prediction is plainly a HIT.

The ten revisions I sampled were all recent, and all recent revisions carry attachments.
The refusals live in an older era the sample could not reach. So a true prediction was
scored false by an instrument whose REACH was too narrow to see the population the
prediction was about -- and the wrong score came with a clean-looking table behind it.

A SCORE IS A FIGURE AND CARRIES ITS REACH LIKE ANY OTHER. `P5 MISS` named no population,
so it was exactly as unverifiable as the `86 of 278` this whole exercise exists to examine.
Scores here now travel with the driven count.

## What this control CANNOT do -- stated so no pass is read as coverage

1. THE RECORDED `EXAMINED 2 of 1 ... the other -1` IS UNREACHABLE AT ANY COMMITTED REVISION.
   The defective tool driven against the flat tree at 16048f82 refuses cleanly:
   rc=2, `CANNOT MEASURE -- 16048f82 carries no thread.json anywhere under intent/st`.
   That figure needs a HALF-MIGRATED tree and none exists in history. Not manufactured:
   a tree built to produce a finding produces the builder's finding, not the estate's.
2. Nested layout only for the differential. The flat side is a separate, single-tree run.
3. It adjudicates ONE instrument. It is a control for the harness, not an estate sweep.
