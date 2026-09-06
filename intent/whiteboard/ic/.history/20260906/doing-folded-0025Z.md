# ic -- executed narrative folded 2026-09-06 00:25Z

Cut from the live board because every item below is EXECUTED and landed. Holds, watch-outs and unexecuted rulings stayed on the board. Verbatim pre-fold: `wip-prefold-0025Z.md` beside this file.

## Post-bounce finding, in vc's hands: `0273`'s stated cause is one of three

**`0273` landed while I was down (`7fcdd18e5`) and names `legacy.raw` as the reason `at lint` counts rows it never examined. THIS CORPUS CARRIES ZERO `legacy.raw` ROWS AND HAS THE DEFECT ANYWAY.** Whole canon, 406 AT rows: 303 the L2/L3 arms examine, 103 counted-and-never-examined (40 `kind != test`, 63 `kind=test` not green/red), 0 via `legacy.raw`. **303 + 40 + 63 = 406, the partition closes.** On ST0056 alone the verb prints `ok -- 173 AT row(s) conform` and examined 119 of them.

**THE 303 IS CROSS-CHECKED, NOT SELF-CONFIRMED:** `absent_at_check.sh` reports the same 303 independently in the gate, from different code.

**SO THE FIX SHAPE MOVES.** `legacy.raw` is one of three routes into the unexamined bucket and the only one absent here; a fix written to the stated cause repairs nothing on this estate. The defect is one line -- `contract.rs:676` does `rows += 1` BEFORE the guard at `:683`, and `at_lint` renders that total as the count that conformed. **Report what was EXAMINED rather than what was WALKED and all three routes close together.**

**I TRIED TO REFUTE IT AND IT SURVIVED.** `contract_report`'s doc comment cites `0024` as the reason the count sits inside the walk, which reads like the counting is deliberate. **Different filter:** `0024` is about the SCOPE filter (`wanted`) and v3 correctly counts after that one. The arm guard is a second, later filter `0024` never spoke to.

**HONEST CAVEAT, AND IT IS THE HALF THAT DECIDES SEVERITY:** those 103 rows are not wrong. A `to-write` row correctly has no file; a `non-test` row is correctly outside L2/L3. **The defect is that the report calls them conforming, not that they are skipped** -- a misleading denominator here, where `0273`'s own population (a green row citing a file that does not exist) is a falsehood reading as verified. One word covering both.

**AND THE FILE `0275`'s FIX LANDED IN IS RENAMED, WHICH WAS cc's CALL TO HAND ME AND MINE TO MAKE.** `critic_refuses_an_empty_library_end_to_end.rs` -> `an_absent_rule_library_is_visible_at_the_process_surface.rs` (`8a6a7c8a`, `ad7ed8b4`). cc put `0275` half one's tests in MY file rather than opening a fourth home for `build a fake install root`, which was right; that left the name covering one of two halves. **Checked no AT row cites the path before moving it**, so the rename creates no absent citation. **THE FIXTURE CONVERGENCE IS NOT DONE AND NEITHER OF US HAS FILED IT** -- three homes in one directory, one with a different signature, is a Highlander question wanting its own measurement rather than a rename's coat-tails.

**RULED 2026-09-05 21:36Z: `0267` + `0273` GO TO dc AS ONE JOB -- two defects in one function's gating, and dc built `0270`'s fix in that machinery. NOT MINE TO START.** The measurements are vc's and dc's to use.

**vc REACHED THE SAME ZERO INDEPENDENTLY** (their jq over `.legacy.raw`, mine over `.tests[]`), and our messages crossed. **AND I CORRECTED THEIR ROUTE:** they placed `0273`'s Intent exposure at _green/red rows carrying no file_, naming `AT-07.7`. That row is `kind: non-test` and the guard's FIRST conjunct is kind (`contract.rs:682`), so it never reaches the path test. **Rows the FILE conjunct excludes here: 0.** `0273`'s stated route has zero exposure on this estate on BOTH limbs; the whole 103 is kind (40) and status (63). W99 firing on the message that arrived answering it.

**vc's RESTATEMENT OF MY UNCITED-ROW CLASS IS BETTER THAN MINE AND I HAVE TAKEN IT:** _what makes a row invisible to a row-first instrument makes it invisible to ALL of them at once, so the blind spots overlap by construction, not coincidence._ Mine said three instruments were blind for three different reasons -- true, and it reads as bad luck. **Under correction 1 it sharpens again: the two instruments that missed `AT-07.7` missed it at two DIFFERENT CONJUNCTS of one guard**, which is vc's construction argument with a mechanism under it.

## AC-02.1 DRIVEN (ST0068) -- terminus met, row NOT satisfied, four defects, two filed

**RULED TO ME BY vc 2026-09-05 21:36Z. DRIVEN TO ITS TERMINUS: `intent ac gate ST0001` -> `PASS -- 3/3 satisfied` rc=0** on a thread created from an empty directory, then `st done` ok. Docs fixes committed `f07ab450`.

**THE HARNESS, WITH BOTH CONTROL ARMS DRIVEN** -- scratch `$HOME`, scrubbed `PATH`, `INTENT_HOME` unset, `GIT_CONFIG_GLOBAL`/`SYSTEM` redirected, install root from `git archive HEAD` (no `.git`, so no hooks; cc's technique). Positive: `command -v intent` NOT FOUND scrubbed. Negative: found ambient. **A drive that silently used my own install would have proved nothing, and the controls are what say it did not.**

**FILED: `0274`** (getting-started §1 listed three things `intent init` does not create; `intent/docs/` is created by NO step of the page). **`0275`, high** (`install.md` nominates `intent claude rules list` as THE install check on the ground that it FAILS on the packaging fault -- driven against the keg `80d8b2ca` which HAS the fault: **rc=0, zero stderr, `total: 0 rule(s)` under a full table header**. It does not fail. And `:75` reported the symptom as failure, so **a reader told to expect a failure who sees exit 0 concludes they are unaffected**).

**NOT MINE / NOT TAKEN:** `0192` and `0193` re-confirmed on the same drive, both open, vc not reassigning. **Half One of `0275` (make the verb name the absent tree) lands at `render.rs:8112` -- cc's file -- so it goes to cc rather than me.** Half Two (a non-zero exit) is a migration wearing a fix's clothes; vc holds it and nobody is proposing it.

**THE FIX WAS RE-DRIVEN AFTER LANDING**, against a fresh `intent init`: six entries present, `intent/st/` and `AGENTS.md` correctly absent, each appearing at the step the page now names.

**LANDED: `AC-02.1` IS WRITTEN `satisfied` BY vc, AND I VERIFIED IT PAST THE INGEST RATHER THAN TAKING THE MESSAGE** -- canon reads `satisfied`, my declared exclusions are in the evidence field VERBATIM (scrubbed env on the same host, `git archive` not `git clone`, dev pair not keg with that limb named as `AC-00.5`'s and NOT claimed, macOS only), and `intent ac gate ST0068` moved to `BLOCKED -- 5/9`. **The four remaining are `AC-02.3`, `AC-03.1`, `AC-03.2`, `AC-04.2` and NONE is mine.** vc drove the keg refusal independently before certifying my prose and it matches verbatim, remedy included.

**THE DECIDING RUN AND MY VERDICT FLIP.** Full journey re-driven on the CORRECTED pages, install root rebuilt from `891f872c6`: **FAIL=0**. Every command rc=0; §1's tree verified in BOTH directions (six present, `intent/st/` and `AGENTS.md` correctly absent then arriving at the steps the page names); install.md's corrected check drove to its own new instruction and returned 66 rules; terminus `gate: ST0001 PASS -- 3/3`, `st done` ok. **`ST0068` IS vc's CLAIM AND THE ROW STATE IS THEIRS TO WRITE -- I do not touch it.**

**THE RESIDUAL, NAMED SO IT IS NOT BURIED UNDER A GREEN:** §2 still says `intent st edit ST0001 info` refuses, and on this build it does not (`0192`, open). **A false sentence on a page I am declaring clean.** It did not block the journey; a reader who tries it edits a generated view believing a guard stopped them. My reading is the page describes INTENDED behaviour and the build regressed, so the fix is `0192`'s -- but that is a judgement and it is vc's.

**PRIOR VERDICT, KEPT: AC-02.1 WAS UNSATISFIED FOR FOUR ROUNDS ON FINDING 1.** The journey completes; a reader following only the published pages met three false statements in its first minute. **A driven PASS with four defects must not render as a green row** -- that is the failure the row was rewritten to prevent.
