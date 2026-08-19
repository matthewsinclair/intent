# ic, 2026-08-19 -- AT-00.11 work rescued out of session scratchpad

Scratchpad is session-scoped and does not survive a compact. These are the artefacts,
NOT their final homes.

- `of_n_closes_over_examined.sh.pending` -- the AT-00.11 instrument. Its home is
  `intent/st/ST0056/parity/tools/of_n_closes_over_examined.sh`. NOT moved there yet:
  matts's suite is live, and cc was told this session that I have nothing uncommitted
  under `intent/st/`. It goes there when they report.
- `at0011-control-*` -- the positive control. Prediction lodged BEFORE the run, score
  after, and the 120-revision sweep it was scored from. Defective `f2a2675f` against
  repaired `8bb47e49`: 17 overstated + 61 agreed + 42 refused = 120, closes.
- `at0011-mutants-*` -- red-first. 7 of 7 predicted arms as predicted, 4 refusal arms
  at exit 2. Records the five defects these found in my own instrument.
- `at0011-first-real-subject.txt` -- the run that showed the method's unfixable confound.

DO NOT DRIVE THE 18 AGAINST AT-00.11 AS THE ROW STANDS (vc, this session). The row asks
for a tree where the FILTER and EXAMINED populations differ and requires M to follow the
examined one. A two-tree differential cannot deliver that: nested and flat are two points
in HISTORY, so an instrument keyed on HEAD examines a DIFFERENT COMMIT on each side, and a
divergence has an innocent explanation by construction. vc is correcting the row.
