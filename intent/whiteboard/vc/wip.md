---
node: vc
name: Validation Claude
role: validation
session_id: b8e50395-2c15-45b8-800b-d97acece15c5
heartbeat_at: 2026-08-20 17:36Z
status: active
focus: "**FOLD 6 -- RULES ONLY; every instance is in `.history/20260820/`.** Suite driven to green on hv's one-off authority: rust fmt/clippy/test all 0 (142 targets, 995 passed), bats **1440 passed / 0 failed** through `tests/run_tests.sh`'s contract, critics clean, credo N/A verified. **Gate: ST0057 47/51, ST0056 59/132, both COMPUTED by `intent ac status`.** Nine rows moved, four criteria minted, four closed. **My headline finding of the sweep was an invocation error and a peer caught it.**"
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## DOING

Nothing. All four inboxes at their sentinel, mine and hv's. Peers folded and released: cc `490eda36`, dc `a72c9e40`, ic `99ba499f`.

## TODO

1. **ST0057 AC-01.5, AC-03.6 (cc), AC-07.7, AC-08.5 (ic); ST0056 AC-03.14 (cc).** None mine to build; all mine to verify on close.
2. **GATED ON hv, RULED AND EVIDENCED, DO NOT START:** dc's two roster admissions (`canon_commit_check.sh`, `thread_view_skew_check.sh`), and `test_helper.bash:93`'s `3.0.0` default. **hv instructed dc directly to hold; that outranks my "land it" and I was wrong to issue one.**
3. `declared_but_unwired` adequacy. The heartbeat-currency note for hv. cc's eleven-copies filing.
4. **49 commits unpushed to `local`. `upstream`'s freeze reads as lifted in `prepush`; confirm with hv before any push there.**

## WATCH-OUTS -- RULES ONLY

- **RUN A SUITE THROUGH ITS RUNNER, NEVER AROUND IT.** An entry point carries fixes and defences a lower-level invocation silently loses.
- **A POPULATION THAT EXCLUDES THE ANSWER RETURNS A CONFIDENT ZERO.** _N of N files lack X_ is worthless if the file that has X is not an N. **Ask what would hold X before counting what does not.**
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
