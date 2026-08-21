---
node: vc
name: Validation Claude
role: validation
session_id: 575f9585-0b9a-47fe-9d3b-24b2a561827c
heartbeat_at: 2026-08-21 10:29Z
status: active
focus: "**GATE CORRECTED TO 62 OF 67 AT `14298e6b` AND BROADCAST -- three files said 63 and three peers woke on it.** The wrong digit was not the finding: all three said do-not-hand-tally while naming two calls that cannot reach 67. **The third call, `ac status ST0056/03`, is a WP-scoped STID nothing in this estate ever wrote down.** Suite green at `706db8ee`, attributable, subject untouched -- **and structurally unable to go red for the undispatched-instrument class that holds three gate rows.** Five gate rows swept CITATION-CLEAN before cc and ic spend context on them. **My one HIGH finding today was wrong and my own grep population is what made it wrong.** 71G of build artefacts found; disposal is hv's."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

**P0 DONE AND COMMITTED (`14298e6b`), BROADCAST TO cc/ic/dc.** Gate 62 of 67, procedure repaired to the three calls that actually compose it. **P2 DONE:** five gate rows citation-clean; `AT-07.7` cites nothing correctly under `11762943`. Four escalations filed to hv and surfaced live. **BLOCKED, NOT DROPPED: removing my orphaned 4.3G worktree was refused by the session permission classifier -- correctly. hv runs it.**

## TODO

1. **ST0057 AC-01.5, AC-03.6 (cc), AC-07.7, AC-08.5 (ic); ST0056 AC-03.14 (cc).** None mine to build; all mine to verify on close.
2. **GATED ON hv, RULED AND EVIDENCED, DO NOT START:** dc's two roster admissions (`canon_commit_check.sh`, `thread_view_skew_check.sh`), and `test_helper.bash:93`'s `3.0.0` default. **hv instructed dc directly to hold; that outranks my "land it" and I was wrong to issue one.**
3. `declared_but_unwired` adequacy. The heartbeat-currency note for hv. cc's eleven-copies filing.
4. **RESOLVED, NOT DROPPED: the 49 commits went to both remotes 2026-08-20 23:51 local; `local` and `upstream` are each 0/0 against `main`.** The `upstream` freeze question is moot for that push and stays live for the next one -- confirm with hv.

## WATCH-OUTS -- RULES ONLY

- **RUN A SUITE THROUGH ITS RUNNER, NEVER AROUND IT.** An entry point carries fixes and defences a lower-level invocation silently loses.
- **A POPULATION THAT EXCLUDES THE ANSWER RETURNS A CONFIDENT ZERO.** _N of N files lack X_ is worthless if the file that has X is not an N. **Ask what would hold X before counting what does not.**
- **AND THE EXCLUSION FILTER IS WHERE THE PREMISE HIDES.** 2026-08-21: I greped for guard dispatch across `.githooks`, `bin/.devbin` and `lib` while EXCLUDING `lib/templates`, reasoning that templates ship to consumers rather than run here -- and the delegated-roster design exists precisely because **this repo consumes its own template.** Confident zero, HIGH finding, said out loud to hv before the self-refute killed it. **A population is a claim about what could hold the answer, and `--exclude` is that claim in its least visible form.**
- **`grep -c` COUNTS LINES, NOT MENTIONS, AND MY OWN CHEAP SPLIT PENALISES THE BEST-FORMED CITATION.** `# ST0057 AT-01.5, covering AC-01.5` scores **1** -- identical to a sloppy single-id mention -- because thread, AT and AC are on ONE line, which is exactly what good practice looks like. **"2 hits means ready" is line-arithmetic wearing evidence's clothes.** Zero still means wrong. Anything else is READ, never counted.
- **A GREEN SUITE CANNOT GO RED FOR AN UNDISPATCHED INSTRUMENT -- STRUCTURALLY, NOT INCIDENTALLY.** Nothing runs, so nothing fails. Three gate rows are red for exactly that reason while the suite reports 100%. **The two are the same fact seen from opposite sides, and only one of them is visible from inside the estate.**
- **A COUNT OVER AN INCOMPLETE FILE IS NOT A MEASUREMENT.** Not a partial one -- not one. Same class as reading a `tail` as a total, and as taking `$?` from a pipe.
- **A TRUE MEASUREMENT OF A DIFFERENT PROPERTY, OFFERED AS PROOF.** Being real and driven is what makes it persuasive. **A green is only ever about the question the instrument asked, never the one the reader hears.**
- **FOUR WAYS A ROW PROMISES MORE THAN IT DELIVERS:** stale citation, uncited coverage, vacuous predicate, **title broader than body**. The first three leave a trace. **The fourth leaves none and no instrument can ever see it** -- the row is internally consistent and reads as covered to anyone who does not open it.
- **THE DISCRIMINATOR -- _what does satisfying this row completely still leave broken?_ -- IS ASKED AGAINST THE BODY AND NEVER THE TITLE.** Against a title it returns nothing, and nothing is also what a correctly-covered row returns. **A discriminator whose failure mode is silence must have its subject named every time.**
- **A DENOMINATOR BELONGS TO A FILE, NOT TO A TOPIC.** Name the file whenever you report a coverage defect. Adjacent bullets read as cause and effect while neither claims it.
- **NO INSTRUMENT CATCHES AN EXPIRED CITATION -- ONLY A BUILDER TRYING TO SATISFY THE ROW.** `at lint` exempts `to-write` from L2/L3, correctly. **The cheap split: does the cited file carry the row's own literal id?** Hits = ready to green; zero = wrong citation, or a declared handoff if the row says so.
- **A CRITERION IS SATISFIED WHEN A ROW THAT COVERS IT IS GREEN** -- never because its subject matter is tested somewhere. **Under the AND gate a second covering row holds the criterion open: when you green a row, watch the criterion count; if it does not move, the row was not the last one.**
- **A CHANGE THAT WOULD CONVENIENTLY GREEN YOUR OWN WORK IS THE ONE TO STOP AND ROUTE.**
- **A DOCUMENT CAN GO STALE AGAINST ITSELF, AND SOURCING IT CORRECTLY IS THEN WHAT DELIVERS THE WRONG ANSWER.** Nothing here checks a document's clauses against each other.
- **A RULING THAT EXISTS BUT IS NOT REACHABLE FROM THE SITE THAT NEEDS IT IS AN OPEN QUESTION IN PRACTICE.** Declining to guess is what makes it findable.
- **A COMMENT PROMISING THE FIX, ABOVE THE CALL THAT DEPENDS ON IT, STOPS THE READER WHO WOULD HAVE FOUND IT.** Worse than stale when it was never true.
- **A CORRECT FINDING CAN CARRY A REMEDY THAT READS FINE AND MAKES IT WORSE** -- and the remedy is the part nobody checks, because the finding was sound.
- **A GITIGNORED ARTEFACT IS INVISIBLE TO EVERY INSTRUMENT WE USE**, so a diff-based blast radius under-reports damage to exactly the artefact the model says matters most. **Not priceable as attention.**
- **`sync --to-store` IS DISK-AUTHORITATIVE FOR ATTACHMENTS; a typed field takes canon.** Prettier rewrites and re-stages **inside** the commit window, so the order is **FORMAT, then SYNC, then COMMIT** -- and idempotence is what makes that work. _Sync last_ is ambiguous and means the worse thing.
- **ZERO FAILURES ACROSS A WORKSPACE DOES NOT PROVE A BINARY RAN.** Confirm each subject in the `Running` list before moving a row.
- **A CRITIC SWEEP DRIVEN TO ZERO WITHOUT READING EACH SITE BREAKS WORKING CODE.** A greppable proxy cannot tell intentional word-splitting from accidental.
- **AN ADVISORY THAT REQUIRES SOMEONE TO REMEMBER IS NOT A CONTROL.** A guard nothing dispatches is indistinguishable from a guard that passes.
- **hv's WORD OUTRANKS MINE AND A PEER MUST NEVER READ MINE AS HIS.** I hold the pen he hands me, not his authority.
- **MECHANICAL:** `--no-fail-fast` always. `st list --status all`. `grep` is ugrep here, BSD grep in a `#!/bin/bash` script -- `-E` throughout; `grep -c` exits 1 on zero. **zsh does NOT word-split unquoted `$var`** -- 178 paths went in as one argument today. Never `$?` after a pipe. Absolute paths; the shell cwd persists. `CARGO_TARGET_DIR` inside the checkout. Read the clock, then PASTE.

## DECISIONS

- **2026-08-20 17:36Z** -- suite green under hv's one-off authority; **credo N/A verified (no `mix.exs`, no `.credo.exs`), not assumed.**
- **2026-08-20** -- D57-8 amended to carry four collection addresses; **AC-07.1 NOT reopened**; AC-07.7 minted.
- **2026-08-20** -- AT-11.6's re-citation onto `prepush` **withdrawn**; deliverable restored to dc.
- **2026-08-20** -- AC-10.3's `project_id` limb **stands**; the value is a UUID, ruled by design three times over.
- **2026-08-20** -- AC-10.4 is _same criterion, better instrument_; **AC-10.15 minted** because AC-10.4's title was broader than its body.
- **2026-08-20** -- AC-00.8's clauses 4 and 5 **disambiguated, not re-covered**; documented and exercised are two criteria.
- **2026-08-20** -- ST0057/WP-04 and WP-06 closed on driven verification, not on their owners' reports.
