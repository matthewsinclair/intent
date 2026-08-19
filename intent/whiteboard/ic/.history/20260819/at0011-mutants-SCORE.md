# of_n_closes_over_examined.sh -- mutant score, AFTER the run

# Scored against mut/PREDICTION.md, lodged before any arm was driven.

# Mutating the CRITERION, not the exemplar: no arm was tuned until the real estate passed.

M1 impossible `2 of 1` HIT FINDING (IMPOSSIBLE) in both trees, exit 1
M2 remainder `The other -4` HIT FINDING (NON-CLOSING) in both trees, exit 1
M3 honest, identical HIT no finding, counted STABLE
M4 honest, ratio moves HIT no finding, DIVERGED -- reported as a NOMINATION
R1 empty population HIT exit 2, and it does NOT say all instruments match
R2 "flat" tree is nested HIT exit 2, identity established rather than inherited
R3 every subject unmeasurable HIT exit 2, "a sweep that examined nothing must never report that nothing is wrong"
R4 population source absent UNPREDICTED -- exit 2, correct, but I did not predict it. Recorded as
an undriven-until-now arm rather than counted as a hit.

7 of 7 predicted arms scored as predicted. 8 arms driven. The 8th was not predicted and is labelled.

## THE MUTANTS FOUND TWO DEFECTS IN THIS INSTRUMENT, AND THAT IS THE POINT OF THEM

1. THE SUBJECT EXTRACTOR SILENTLY DROPPED SUBJECTS. `[a-z_]+\.sh` misses any name
   carrying a digit, dot or dash. NO REAL TOOL CURRENTLY HAS ONE, so the real estate
   could never have shown this and reading never would have: the population would have
   come back short, closed arithmetically, and looked complete. A subject dropped from
   a population is invisible in exactly the way this row exists to catch.

2. THE PARTITION MIXED TWO POPULATIONS -- THIS ROW'S OWN DEFECT, INSIDE THE INSTRUMENT
   ENFORCING IT. `n_findings` counted EMISSIONS (per tree, per ratio) while diverged,
   stable and unmeasured counted INSTRUMENTS. First real run printed
   `2 findings + 1 + 1 + 1 = 7` over 5 driven. CAUGHT BY THE CLOSURE ASSERTION, which
   refused to bank the verdict and exited 2. Not caught by review, and I wrote both.
   The two counters are now printed as two: N instrument(s), M emission(s).

   That assertion exists because vc ruled this morning that CLOSURE MUST BE ASSERTED IN
   THE OUTPUT rather than left available to a reader who adds up. It paid on the first
   run of the first instrument built after the ruling.

3. A THIRD, FOUND WHILE FIXING THOSE: the TREES line printed the DEFAULT revision
   literals even when --nested/--flat supplied real directories -- naming a subject the
   run was not measured from. The same class as the day's other error, one line down.

## TWO MORE DEFECTS, FOUND AFTER THE MUTANTS PASSED

4. THE DIFFERENTIAL WAS CONFOUNDED. The first build took each side's script from ITS OWN
   tree, so the INSTRUMENT VERSION and the TREE SHAPE varied together and no divergence
   could be attributed to either. That is the bar this row holds others to -- establish
   the identity of what you compare, never inherit it -- broken by the instrument
   enforcing it, and the mutants could NOT catch it: their two copies were identical, so
   a confounded run and a controlled one gave the same answer. Found by reading the code
   after the mutants were green. One version now drives both trees (`--tools-from`).

5. AND THE CONFOUND THAT CANNOT BE REMOVED, found by running the first REAL subject.
   A nested tree and a flat tree are two points in HISTORY. No revision exists in both
   shapes -- the only tree that ever held both was half-migrated and none was committed.
   So an instrument keyed on HEAD examines a DIFFERENT COMMIT on each side.
   canon_commit_check.sh reports `0 of 279` nested against `87 of 280` flat, and that is
   fully explained by the two HEADs with no defect present at all.
   THE METHOD CANNOT SEPARATE SHAPE FROM REVISION. Arms A and B survive it (an impossible
   ratio and a negative remainder are wrong at any revision); ARM C cannot, which is why
   it nominates and does not adjudicate. Stated in the output as REACH 6.
