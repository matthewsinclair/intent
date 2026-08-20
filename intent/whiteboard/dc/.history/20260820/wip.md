# dc -- archived 2026-08-20

Rolled out of the live board at 2026-08-20 09:18Z, after `critic` landed at `5043d0c4`.

## DONE -- the day's work, as the board carried it

**`intent critic` -- BUILT, AT PARITY, UNCOMMITTED.** `critic.rs` (677 lines + 18 tests), the CLI arm, `EXIT_REFUSED`/`Failure::Refused` in the spine, `regex` as a direct dep (already in the graph via `ignore`, so zero new supply chain). **Exit codes built to the CODE and the GATE, never the table: 0 clean / 1 findings / 2 usage / 3 refused.**

**FOUR DEFECTS IN MY OWN WORK, ALL FOUND BY RUNNING IT AGAINST v2 RATHER THAN READING MY PORT, AND EVERY ONE WAS A FALSE CLEAN.**

1. **TOOL-ARMED RULES WERE COUNTED `ran` AND NEVER RUN.** The finding loop was gated on grep patterns, so shell's two shellcheck rules reported ASKED and asked nothing. **The census output was byte-identical to v2's, so a parity check on the REPORT passed while the ACT diverged** -- subject and report swapped, in the one direction that looks like success.
2. **BLOCK-FORM `applies_to` READ AS ABSENT, AND ABSENT MEANS UNIVERSAL.** `frontmatter_list` handled only the inline form, so every scoped rule fired on every file -- `IN-EX-TEST-002` (`test/**/*_test.exs`) reporting against `lib/`. **It did not fail closed, it fired everywhere.** Caught by v2 reporting one finding where I reported two.
3. **AN UNKNOWN LANGUAGE RETURNED 0.** No rules match, empty census, clean exit. **In the gate a typo in a project's declared language list silently disables checking for it.** v2 refuses at 2.
4. **A MUTATION THAT NEVER APPLIED, REPORTED AS A PASS.** A quote-nesting `SyntaxError` in the mutation script; the test ran against unmutated code and printed `ok`. **The probe did not run and the output looked like a result.** Every mutation now asserts its anchor and diffs the file hash before trusting the verdict.

**Mutations confirmed effective (3 of 3, after the above):** findings->2 fails the exit contract; dropping flag validation fails 4 injection-boundary tests; inline-only `applies_to` fails the scoping regression.

**DELIBERATE DEVIATIONS FROM v2, STATED RATHER THAN DRIFTED:** ID lists sorted (walk order is undiffable and meaningless); JSON carries the census in-document rather than pushing it to stderr; an undrivable tool is a REFUSAL, following v2's comment rather than v2's code -- **zero live population for that arm, so it is a fixture and the zero is written into the test.**

## Decisions settled today, archived rather than deleted

- (2026-08-20) **CITED IS NOT OWNED -- vc, correcting me, and the correction is right.** All 45 parity tools are attachments in the store (checked straight at sqlite: 45 rows), so **45 of 45 are owned**; 20 are additionally CITED by an AT row and 12 of those name a file that does not exist yet. My "37 genuinely unowned" partitioned by citation and reported it as ownership. **An AT row pointing at a file is EVIDENCE, NOT TITLE.** What survives is the 12: an artefact naming a path before any file exists there proves **path-naming in canon is not derived from disk**. The mechanism does not fall short inside a thread directory; it stops at the directory boundary, which confirms cc's arity point rather than denting it.
- (2026-08-20) **WP-07 DELIVERABLE 1 IS MIS-SPECIFIED RATHER THAN UNDONE.** No rust-embed anywhere; `rules.rs:17-21` states the design that replaced it -- roots resolve from the executable's own location, never the environment -- and AC-11.3 is green by that route. **A deliverable naming a mechanism the architecture chose against, while the criterion it served is satisfied.** vc's to reword. Row-by-row: deliverable 1 mis-specified, 2 is `claude` at 2 of 8, 3/4/5 (critic, agents, lang) untouched. **`rules.rs:11-14` says the headless critic "reads this module rather than restating any of it" -- critic was designed to sit on what landed last night, which is why it is first.**
- (2026-08-20) **`critic` IS A PRECONDITION OF FULL SELF-HOSTING, NOT A ROW IN THE SWEEP.** Self-hosting means v3 on PATH; `intent critic` answers 2, which the pre-commit gate reads as fail-open, in all five declared languages here and in 15 projects through one symlink. **Build it before anything touches PATH.**
- (2026-08-19) **EVERYTHING THAT MANAGES STEEL THREADS IS DONE; EVERYTHING THAT MANAGES INTENT ITSELF IS NOT.** That is the hosting gap in one line, and the split is WP boundaries rather than accident.
- (2026-08-19) **A REPO-LOCAL HOIST AND A FLEET HOIST ARE DIFFERENT OPERATIONS** (ic). One symlink serves 16 projects, every one declaring v2. Repointing it is not hoisting this repo, it is starting a 16-project migration -- and only the first is a tonight-sized thing.
- (2026-08-19) **WITHDRAW AS A PRECONDITION, BUILD AS CAPABILITY; THE WARRANT DECIDES, NOT THE CODE.** AC-06.4 is one build under two justifications and only one of them gates. **The question was never whether the work is wanted, it is whether a gate should hold on it.**
- (2026-08-19) **A GATE MUST NOT BE A SECOND AUTHORITY OVER A DESTRUCTIVE ACT** (vc, sharper than my Highlander framing). `sync_uncommitted` asks _are there bytes nobody has synced_; the realiser gate asks _can the store reproduce these bytes_. **They disagree BY CONSTRUCTION rather than by drift**, so a second gate refuses work the real authority would allow. The warning earns its place on timing; safety stays where the deletion is.
- (2026-08-19) **`intent organize` PREVIEWS BY DEFAULT AND `--apply` PERFORMS, ONE BODY TAKING A `Mode`** (ic, AC-05.1). **vc previewed every step before firing the 423-file dehydration and says the polarity is what made it safe to fire.**
- (2026-08-19) **AN INFERENCE FROM SHAPE THAT READS A SPELLING IS FORBIDDEN; ONE THAT CALLS THE MODULE OWNING THE FACT IS NOT** (ic, on `address::promote`). The distinction generalises past that row, and it is the test to apply to any "we can just detect it" proposal.
- (2026-08-19) **A ROW DECLARING A VERB THE MODEL CANNOT EXPRESS IS A ROW THAT LIES.** Proven against me tonight -- four hydrate rows the roster cannot bucket because their mechanism does not exist. **Table-leads-reader is right when the behaviour exists and only the surface lags; reversed, it ships a promise.**

## Decisions carried from 2026-08-19 that today closed or superseded

(Yesterday's set archived here in full; the ones still governing tomorrow stay on the live board.)
