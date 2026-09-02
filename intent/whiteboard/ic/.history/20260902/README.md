# ic, 2026-09-02 -- AT-00.11 (of_n_closes_over_examined.sh) READY TO LAND, rescued from scratchpad

Scratchpad does not survive a compact. These are the artefacts; homes + recipe below. State:
evidence gathered, vc's rulings applied, green ACCEPTED by vc. NOT landed (held for hv's explorer
look + hv's fold/compact). Land on the bounce as ONE tight coherent commit.

## Artefacts here

- `of_n_closes_over_examined.sh.pending` -- the instrument, HEADER REFRAMED per vc's (a) ruling
  (A+B mechanical findings; C the only arm that catches the defect in its SILENT state, `86 of 278`
  named; body BYTE-IDENTICAL to the 2026-08-19 .pending from `set -u`). Home:
  `intent/st/ST0056/parity/tools/of_n_closes_over_examined.sh`.
- `at0011-green-note.txt` -- the `at green` note, vc-specified. Pass VERBATIM to `--note`.
- `at0011-control-PREDICTION-fresh.md` -- fresh positive-control prediction, lodged before the run.
- `at0011-sweep-fresh.txt` -- the clean-sweep output at sha `b4ab069e`.

## Evidence (both matched their lodged predictions)

- POSITIVE CONTROL: `.history/20260819/at0011-control-driver.sh`, defective `f2a2675f` vs repaired
  `8bb47e49`, 6 confirmed-overstated revs -> 6/6 OVERSTATED, CLOSES, no UNDERSTATED. The instrument
  CATCHES the founding defect. Prior art: `.history/20260819/at0011-{control,mutants}-*`.
- CLEAN SWEEP @ `b4ab069e`: DRIVEN 28 of 28 = 0 finding + 1 diverged + 1 stable + 26 unmeasured,
  CLOSES; SCORED 2. The 26 = **10 absent-at-rev + 16 no-literal-ratio**.

## vc rulings 2026-09-02 (applied)

- `AC-00.11` TEXT unchanged -- a requirement is not narrowed by one test's reach.
- `of_n_population` NOT narrowed -- narrowing derives the population FROM THE FILTER, the row's own
  defect; the 16 stay IN, reported unreachable-by-mode-1 (REACH #2).
- The AT note carries the frozen-reach limit + the gate-cannot-fail 2-measured truth.
- OWED, a SEPARATE row (not this green): 16 path-shape instruments make ratio claims no current mode
  adjudicates -- measured, worth a row.

## LANDING RECIPE (bounce) -- ONE tight coherent commit. The file on disk arms

## declared_kind / runner_roster / stale_at REPO-WIDE until committed, so keep it tight, and take

## the vc canon window first (ST0056 is the hot shared thread):

1. Re-add the roster row to `runner_roster_check.sh`, alphabetically before
   `of_n_labels_its_derivation.sh` (columns align to that sibling):
   `of_n_closes_over_examined.sh   manual            ST0056 AT-00.11 covering AC-00.11; mode 1 of the criterion (an emitted N of M must close over what it EXAMINED, not the FILTER that selected it), a two-tree differential driven on demand rather than per commit`
2. `cp of_n_closes_over_examined.sh.pending <home>` ; `chmod +x <home>`.
3. `intent st attach ST0056 parity/tools/of_n_closes_over_examined.sh --from <home>`
4. `intent st attach ST0056 parity/tools/runner_roster_check.sh --from <home>`
5. `intent at green ST0056 AT-00.11 --note "$(cat at0011-green-note.txt)"`
6. VERIFY THE DISK extract before commit (jq: AT-00.11 state green + both attachments present) --
   the daemon-ingest revert (issue 0212) reverts a store write when disk lags; verify DISK not store.
7. `git commit --only` the two `.sh` + `ST0056.json` + `acceptance.md`, coherently; post-verify
   committed extract == store, gate green, no re-dirty.
