# cc fold -- 2026-08-25 23:20Z

Verbatim from `wip.md` before the fold. Cut because git holds the detail, the item was
filed elsewhere, or the item explicitly asked to be cut once its lesson had a home.

## The per-commit LANDED list (git holds every word of this)

**LANDED (git holds the detail; this holds only what git does not):**

- `540d92bb` **`version`** -- both spellings byte-identical because the arm asks clap for `render_version()`. **ST0058 `AC-00.6` is satisfiable and I did NOT satisfy it** -- ic verified it and my half PASSES; the ROW still fails on the OTHER twin (`intent help` rc=2 vs `--help` rc=0), filed by vc as **issue `0086` HIGH with three fixes ruled. Queued, not assigned, and it lands near dispatch-table work.**
- `0d77e337` **`plugin`** -- three entries, `plugin list` byte-identical to v2.
- `e63813a2` **config-write prerequisite** -- `Config::declare_language`/`undeclare_language` + atomic `write_config`. **This was the first mutation route to `config.json` in v3 at all.**
- `b60f9ebb` **`lang`** -- `init`/`remove` declare and undeclare in `config.json`, `show`/`list` answer from a compile-time registry, **`sync` RETIRED on two independent grounds** (nothing to converge; a second home for the `languages` array). **`lang list` BYTE-IDENTICAL to the FROZEN v2 install at 83 bytes** and derived from `rules::declarable()`, not the template scan. Closes the build half of `0068`. **The `lang` sequencing was WRONG BY THREE**: five table edits were really five rows + `legal_pairs` + `populations` + six help strings + a regenerated `dispatch-table.md`. **Price the DERIVED artefacts, not just the rows.**

## TODO 1 -- the marker that asked to be cut

1. **`sync` untracked-bytes has MOVED TO `DOING` -- hv RULED IT IN 2026-08-25 and the `DO NOT BUILD` on this line was true for four hours.** Kept as a marker, not as an instruction: **this is the second time in one evening a resolved item sat open on this board because nothing pushes a ruling back to the question.** The hv-inbox entry at 22:42Z asked for the ruling; the ruling came back over the live channel; the board recording the question learned nothing. **Cut this line at the next fold -- it has no content left once the lesson is in Watch-outs.**

## TODO 3 -- VIEW_NAMES, answered and filed as issue 0087

3. **dc's `VIEW_NAMES` question -- ANSWERED, AND NOW FILED AS ISSUE `0087` (low, open). NOTHING OWED BY ME.** Driven: `VIEW_NAMES` occurs only in `address.rs` (357, 394); `project.rs` references it ZERO times, and `e63813a2` was 250 pure insertions touching `classify` zero times -- **still true, unchanged by my work**, and vc re-drove it independently before filing. **The FILING is the part that mattered**: all four nodes were compacting when it was answered, and an unfiled finding evaporates.

## TODO 4 -- U3's build queue, routed to vc

4. **U3's build queue** -- superseded in substance by the 36-of-105 measurement, which was driven FROM THE BINARY. Routed to vc for globalfold; **do not edit `restart.md`.**

## The gate line as it stood, which was FALSE when folded

Kept verbatim because the lesson is the staleness, not the figures. Driven at 23:20Z:
`ac gate ST0057` -> BLOCKED, rc=1, 51/53, unsatisfied AC-08.6 AC-08.7; ST0058 2/6, not 0/6.
vc reported this and the board still carried the old figure for hours.

**THE GATE: `ST0057` CLOSED 2026-08-25 AT 67 OF 67, AND THAT IS NOT THE RELEASE.** `ac status ST0057` 51/51 + 2 withdrawn; `ac status ST0056/03` 16/16 + 1 withdrawn; `ac gate ST0057` PASS rc=0. **`ac status ST0056` answers 64/134 and is NOT this gate's denominator** -- the third call is the one nobody writes down. **DRIVE THE THREE VERB CALLS; DO NOT TRANSCRIBE THE FIGURE FROM HERE.** `ST0058` is 0/6 and now GATES THE CUT, so the release is further away than the gate closing suggests. **hv OWNS THE RELEASE AND NOBODY TAGS OR PUSHES.**
