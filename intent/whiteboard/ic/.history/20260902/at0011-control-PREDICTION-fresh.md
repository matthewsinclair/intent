# AT-00.11 positive control -- FRESH re-drive PREDICTION (before the run), 2026-09-01, ic

Re-driving at0011-control-driver.sh: defective canon_commit_check.sh (f2a2675f) vs
repaired (8bb47e49). FRESH because the POPULATION moved (relocation + target
consolidation + devbin rollout since 2026-08-19); the ARMS are the same code (body
byte-identical from `set -u`). Prior art, cited not restated:
.history/20260819/at0011-control-{PREDICTION,SCORE,sweep-120} and at0011-mutants-*.

Revision set (a defined subset of the archived sweep's 17 confirmed-overstated rows):
48d48d20 5a0d0e00 bf6654fb fd2e4067 69033e4e 92a51134

P1 -- all 6 still OVERSTATE: old(f2a2675f).scoped > new(8bb47e49).scoped, STRICTLY.
The defective tool's superset filter over-counts; the fix's `comm -12` does not. The
arm that catches the founding defect still fires at the current estate.
P2 -- the driver CLOSES: overstated + agreed + no-verdict + unreadable = driven.
P3 -- NO UNDERSTATED row. old < new would falsify the defect's one-directional sign.

Then the CLEAN result, of_n_closes over the AT-00.11 population at HEAD (banked ONLY
after the control above fires):
P4 -- CLOSES; Arm C DIVERGED nominations expected (two trees are two revisions, the
confound REACH #6 names); ZERO Arm A/B findings (the defect is repaired at HEAD);
some UNMEASURED (a tool absent at a historical rev, or correctly refusing). A clean
sweep means nothing without P1 having fired first -- that is the whole rule this row
inherits.
