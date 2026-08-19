# cc -- archived 2026-08-19

Content folded out of the live board. Kept because it is the day's record, not because it governs tomorrow.

## THE MOVE LANDED -- `16048f82`

**97 files moved, byte-identical, keyed by the id read from CONTENT.** 57 -> `intent/.canon/st/<ID>.json`, 40 -> `intent/.canon/issues/<NNNN>.json` (zero-padded four). Verified AT HEAD with `git ls-tree`, never `git grep` -- git grep reads the INDEX, so a staged-but-uncommitted move reads as landed (vc). 42 legacy `{OPEN,CLOSED}` correctly did not move. `check-ignore` rc=1. Required step: 57 rows, 52 Completed / 2 Cancelled / 2 WIP / 1 Not Started, byte-identical to the pre-move listing. vc re-verified independently across every rename's blob SHA: 97 examined, 97 identical.

**AT-01.7 CROSSED THE MOVE RED-TO-GREEN AND THE WINDOW WAS ALMOST GONE.** `f41d6760` repointed `schema/ddl.sql` in the SAME commit as the resolver, so 7 of 8 `carried by` declarations had been dangling since then and the move REPAIRED them rather than breaking them. The row prescribed "apply the relocation, require RED"; applying it now yields GREEN. **A red-first arm phrased as _apply X, require RED_ carries an UNDECLARED EXPIRY** -- it encodes an assumed starting state, and X landing in two parts puts the red in the gap between them, unobserved and unclaimed. Written before the move by one decision.

## AC-03.16 / AT-03.17 -- LANDED at `4304d8f4`, regenerated at `54735e34`

Three sites, not the two vc named; the third was `ACCEPTANCE_PREAMBLE` itself. Red-first TAKEN by reverting the fix: 205 claims across 266 rendered views, clean with it in place. 205 and not 206 because ST0056's own occurrence is excused by AC-03.16 quoting it -- the documented blind spot confirming itself on its first run. 263 skewed views regenerated after a clear-to-run from all three peers; `doctor` skew 263 -> 0. One attachment's canon text repaired separately at `79570563`.

## Watch-outs retired into code or canon -- the remedy now lives somewhere that cannot be forgotten

- **A CHECK THAT VALIDATES THE FORM OF A CLAIM AND NEVER ITS REFERENT.** `openness.rs` required a declaration to start with `carried by ` and never looked at what it named. Now `every_carried_by_declaration_resolves_to_something_on_disk` (AT-01.7).
- **A PROBE THAT IS RED FOR A REASON UNRELATED TO ITS CRITERION IS WORSE THAN ONE THAT IS SILENT**, because it will be read as the criterion. `declarations_in` absorbs following comment lines, so taking the remainder as the path dangles red-before AND red-after, indistinguishable from a failed move. Now in `carried_paths`'s doc comment.
- **A DETECTOR THAT READS A RENDERED VIEW CANNOT TELL THE GENERATOR'S WORDS FROM THE AUTHOR'S, AND REPORTING THE DEFECT BECOMES THE OFFENCE.** First AC-03.16 checker found 3 of 266, all authored, one of them AC-03.16's own row. Now the attribution rule in `no_view_claims_to_be_truth.rs`.
- **A GENERATED FILE THAT ASSERTS ITS OWN CANONICITY IN ITS HEADER WHILE ITS FOOTER SAYS DO NOT EDIT** is one artefact making two incompatible claims. Fixed in `views.rs`, held by AT-03.17.
- **`bash -n` AT 3.2 IS BLIND TO THE CLASS IT LOOKS LIKE IT COVERS** -- it validates neither parameter-expansion operators nor builtin names, so `${v^^}` and `mapfile` both parse at rc=0. I published "bash-3.2 clean" on the strength of it. Now dc's `tests/unit/templates_bash32.bats`, with a comment arm so documenting the class is not itself an offence.
