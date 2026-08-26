---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 22:06Z
status: active
focus: "BOSS-VC. SERIAL PORT of all 22 Intent projects to v3 on hv's order: controlled from this session, on main at HEAD, nothing parallel, every write by the pinned pair by absolute path, hv watching every command. #1 Anvil CLOSED 442c563. #2 Conflab BLOCKED: the migration PANICS at legacy.rs:1844 on a multibyte slice -- found on a throwaway clone, real tree untouched; cc's fix. dc has the backup-schedule fix (hv: config, never a literal; back-fill)."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**THE SERIAL PORT (hv, first-hand, this session, between my clock reads 21:45:23Z and 22:00:44Z on 2026-08-26):** _"systematically go thru EVERY INTENT-USING PROJECT here, one by one, and port them COMPLETELY and COMPREHENSIVELY to Intent3 ... We are NOT GOING TO PARALLELISE ANY OF THIS ... SERIALISE the port and control it from this Claude ... on MAIN on HEAD right here ... not going to run the full test suite UNLESS I SAY SO ... the relevant subset ... I want to watch everything forensically."_ Basis: devbin-vc's audit (issue 0098), re-enumerated: 22 projects. **The ledger, the procedure and the order are in `cutover-runbook.md` under PORT LEDGER.** Rulings in force: buckets go to `intent/history/` (hv); `deps/`, `_build/`, `node_modules/`, `.worktrees/` are never enumerated and never get a verb (hv); the backup schedule is configuration, never a literal, and is back-filled into every config (hv); `doctor 0` is a 48h reading until a scheduler exists or hv rules backups manual.

**THE PAIR:** pinned `$S/pair-a1af59f3/{intent,intentd}` = hv's own clean build of HEAD (`fdf7178d`; the stamp reads the last `native/rust`+`surface` commit, `a1af59f3`, which is the fix working). Bare `intent` on PATH is the brew keg `3.0.0_1` = tag `v3.0.0` = `80d8b2ca`, 205 behind, ZERO rules. **THE FLIP HAPPENED 2026-08-26 14:00:46Z on my word** (`brew link intent` rc 0, `brew test` rc 0, keg install 13:59:29Z by brew's own record, link mtimes 14:03:59Z) -- my folds dropped this line and dc had to ask; it stays here now.

**STATE PER NODE:** dc -> the backup-schedule fix (assigned after Anvil's close; XS; subset tests; reports a sha; I re-pin and re-run `upgrade` on ported projects for the back-fill); ic -> `todo done --prune` on hv's order; cc -> holding, and the Conflab panic is cc's the moment hv says so (`a1af59f3` is cc's step (1), `unread_field_keys` walks bytes and slices at a non-boundary); devbin-vc, lamplight-vc -> paused on hv's word.

## CRITICAL PATH

1. **Conflab:** cc fixes `legacy.rs:1844` (`char_indices` / `is_char_boundary`, a fixture with `✓` inside an AC row -- Conflab `ST0121/acceptance.md:72,83` -- and the mutation that removes the guard must red); I re-pin at cc's sha, rehearse on the clone again, then port the real tree.
2. **dc's fix lands:** re-pin; `upgrade` re-run on every project already closed so the `backup` block back-fills through the tool.
3. **Order:** #3 Arca/arca_notionex, #4 Baize, #5 Courses/002, #6 Intent (with ST0055's five `wp done`, the remedy text, my two shell reds), #7 Laksa (312 attached; the rest to history under the ruling), #8 Lamplight (3740 to history is a `git mv` now; the 10 row losses by hand), #9-#21 the clean 13 verify-only.
4. THERE IS NO 3.0.2. 3.0.1 collects: cc's migration steps (2)(3), the backup schedule, `config get|set`, the daemon snapshot or a manual ruling, `st repair`, the PASSES-branch remedy text, the unread-field fixture.

## HELD FOR hv

- The daemon's scheduled snapshot vs "backups are manual" (the finding returns 48h after every `intent backup` until one of them exists).
- `intent config get|set` and `st repair` answer "known command not implemented yet"; `finding.rs:348` puts the BLOCKED remedy under the PASSES finding.
- Lamplight: the 22 non-UTF-8 files that refuse `--to-store`; the two stale `.worktrees/{cc,ic}` at 2.19.0 (prune or leave); ST0270's covers reading.
- Mine, at a quiet moment: `verify-canonical.sh`'s `.backup/` arm reads the root only -- Anvil's `intent/.backup/` was untracked-not-ignored under a PASS.

## Watch-outs

**These are vc's OWN -- durable cautions, standing, not archived.**

1. **AN INSTRUMENT'S OUTPUT READ AS THE SUBJECT'S ANSWER.** Today in my hands: a display alias (`{evidence: (.evidence // .note)}`) read back as the schema; a process grep matching dc's WATCH LOOP and reported as hv's release for four minutes; a catch-all in `legacy.rs` inverting eight hv sign-offs at exit 0 (ic). **A classifier whose DEFAULT BUCKET absorbs the unrecognised case cannot report that it failed.**
2. **MECHANISM BEATS A NOTE.** zsh does not word-split (four times, mine); BSD awk has no `\s`; `${PIPESTATUS[0]}` is empty; an rc read through `tail` is the pipe's. **Every rc to a file; every loop over a newline list in a bash FILE; every chain gated on the script's rc** -- my Prolix collapse printed `stop=1` and I committed.
3. **A PARITY SUITE PROVES TWO BINARIES AGREE; IT CANNOT PROVE EITHER IS USEFUL TO ITS CALLER** (dc, `0085`).
4. **A CLOSED LIST IS SAFE WHEN IT DECLARES WHY THE THINGS NOT IN IT ARE NOT IN IT** (dc). **AN ABSENT FIELD MUST BE REFUSED, NEVER RENDERED** (`0086`); its sibling: a known token with an EMPTY value substitutes silently where an unknown token refuses loudly (`rootfiles.rs:335`, devbin-vc).
5. **A BIDIRECTIONAL CLAIM IMPLEMENTED IN ONE DIRECTION IS GREEN FOREVER ON THE SIDE IT DOES NOT WALK** (dc).
6. **A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** A pair sha issued in advance is valid only until the next refusal; the body carries the pair that PERFORMED the migration, read at the time. **A number that travels through a message has a transcription hop nothing checks** (a digest wrong at character 17; a scope difference read as a mismatch). Full-length, by command, against the file.
7. **SHARED CHECKOUT: `--only` SEPARATES FILES, NOT AUTHORS; two reads in one command or they are two facts** (dc). A dirty `native/rust` REDIRECTS a build to a private target the wrapper never reads. **AND A `git status` LINE NAMES PATHS, NOT AUTHORS**: I read four dirty paths and assigned all four to ic; two were cc's live migrator mid-gate, and "park ic's four" would have parked them. Nobody parks a path they did not write; the owner names the tree clean by their own hand.

8. **AGREEMENT IS NOT CORROBORATION WHEN EVERY INSTRUMENT ASKS THE SAME QUESTION** (ic). **A change observed is not a landing. Only the flip earns "done"** (devbin-cc, hv). **A loop over a collection the defect EMPTIES cannot witness the defect** (cc) -- assert the length first.
9. **CANON IS THE SSOT FOR ROWS, NOT FOR PROSE.** Delete only a line-subset; otherwise keep. **The store's refusal is the oracle; nobody edits a ratified contract off a grep's prediction.** And the tail after a verdict token is OPAQUE -- read the verdict from it, never decompose and reassemble it (cc's withdrawn `status:` fix rewrote authors' bytes with no red anywhere).
10. **EVERY TIMESTAMP IS READ IN THE SAME COMMAND THAT WRITES IT. A NUMBER IN THE TERMINAL IS NOT A CLOCK** (cc). A fold parked outside the repo carries `2026-08-26 13:47Z` and stamps at apply.
11. **A PERMISSION BOUNDARY IS PER SESSION AND IS NOT ROUTED AROUND BY ANYONE, INCLUDING THE DIRECTOR.** Three refusals today were right: mine on taking dc's `--force`, mine on killing dc's process, mine on editing a gate's test without hv's word. **A relayed STOP is honoured on the relay; a relayed approval is not an approval; a lift carries hv's words verbatim; a peer's push authority is not mine to hand on.**
12. **A SUITE READING A LIVE ESTATE MEASURES A MOVING TARGET** (ic); only the failure TEXT tells a peer's mid-write from a regression. And the corpus cannot exercise the half the real data never reaches -- synthetic refuse cases, or the green proves nothing (cc, 1910 of 1910).

13. **UNTIL 2b'S PAIR IS BUILT, NO `intent` WRITE VERB AT ALL ON A RE-CONVERTED TREE; UNTIL 3.0.1'S KEG, WRITE VERBS ONLY ON THE VERIFIED PAIR BY ABSOLUTE PATH** (Lamplight, 2026-08-26, local `stat` 19:45:00 BST -- the Z form I wrote earlier was DERIVED from that local read, not read, and is retracted as a stamp; settled 19:03Z): READ verbs on the keg (`st list`, `ac gate`, `todo`) write nothing -- proven on clones of Lamplight and Cdsync and on dc's fixtures with a write control. What rewrote 167 tracked views on Lamplight at 19:45:00 BST by `stat` (local; not read in UTC) was the KEG via a WRITE verb: cc showed statically that the current tree cannot emit `(no reference)` since `e696de15`, that there is ONE emitter in production (`views::render_all`), and that `strings` finds the literal in `/opt/homebrew/bin/intent` (`80d8b2ca`) and not in `bin/intent3`. Which verb is the filer of Lamplight's `.canon/issues/0005.json`'s to say (a real defect report, untracked, a peer's live work -- hands off). `bin/intent3` refuses whenever native/rust moves past its stamp (the guard working). Check `git status` after the first call, whatever the verb.

14. **AN INSTRUMENT'S DEFECTS FAIL TOWARD CALM** (devbin/vc, four in one day, zero false alarms): a run that measured nothing must print a verdict that is NOT a pass; a pass is a `VERDICT:` line at rc 0 and nothing else; a self-test proves each arm in both directions. And mine: an anchored edit (`^7\. `) hit a numbered watch-out instead of the step I meant -- verify the last step's effect (a grep count, a sha) after every multi-step call, because a zsh parse error or a wrong anchor runs NOTHING and prints little.

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **A CONTROL THAT MISSES ITS SUBJECT IS WORSE THAN NONE, BECAUSE IT WOULD BE BELIEVED.**
- **A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED** (hv). Files committed with a v2 binary on PATH is not a migration.
- **STATE THE INVARIANT, NOT THE MECHANISM.** **A CONVERGER IS NOT THE CURE FOR A SECOND HOME; ONE HOME IS.** **RECORD THE MENU, NOT ONLY THE SELECTION.**
- **A LATER FIRST-HAND RULING FROM THE SAME PRINCIPAL ON THE SAME SUBJECT SUPERSEDES. hv outranks the pen; the pen sequences.** hv's own rulings go on hv's board with their menus; rulings under delegation go to hv's inbox.
- **THE PEN MOVED THE AUTHORITY, NOT THE AIM -- AND NOW THE PUSH, SCOPED TO TWO REPOS.** The confirm gate stays human (hv fires the cut); `publish` is the one verb with its refusals; the flip is on my word; I tell hv before anyone tags if a gate does not close.
- **LEAD WITH THE DECISION THAT MAKES THE ENDGAME REACHABLE.** I put hv's own publish as item 2 of 6 in an inbox hv was not reading; hv stopped the fleet on an empty tap and was right.
- **A SWEEP TAKES ITS POPULATION FROM THE ARTEFACT'S OWN DECLARATION, NEVER BY TRAVERSAL; A SUBSET MEASURED IS NOT THE POPULATION; A DIRECTORY SUMMED IS NOT A THREAD.**
- **A SAMPLE THAT CANNOT EXHIBIT THE DEFECT IS NOT A WORST CASE. A SECOND OPERATOR ON A SECOND PROJECT IS A CONTROL.**
- **A CONVERGER AT THE WRONG FIXED POINT IS BLESSED BY THE IDEMPOTENCE CONTROL.** Only a COUNT sees a doubled block; only "is it THIS block?" sees a substituted one.
- **AN ARM THAT DEMANDS WHAT THE TOOL CANNOT PRODUCE IS A SPEC DEFECT; AN ARM THAT ACCEPTS WHAT THE TARGET STATE FORBIDS IS A FALSE GREEN.** Presence is not provenance; provenance (the footer) is not content (an empty H1).
- **`core.hooksPath` MAKES `.git/hooks/` INERT**; resolve the way git does. **A GUARD OVER ADDED LINES CANNOT TELL A RE-RENDER FROM A CLAIM** (dc): every first-render of a generated view trips it exactly once.
- **HOP 2 IS THE ORACLE.** It refuses atomically and names the pair. **A NARROWED HALT IS A MEASURED ONE.**
- **RULE ON THE MEASURED SIZE.** Three rulings today were made on a size cc gave me and re-made cheaply when cc measured before building; the one that was a schema migration did not ship on release day.
- **SOURCE COMMITS COST AN APPLE ROUND TRIP; TEST COMMITS DO NOT STALE THE PAIR.** Batch source after the fleet; one final re-cut. The migration-correctness fixes and the version bump were the exceptions.
- **READING THE WRITE-UP OF A CLASS IS NOT PROTECTION FROM IT.** The remedy is never care.
- **RULED UNDER THE PEN (vc, 2026-08-26 16:3xZ, delegated, `authority: vc`): hop 2's shortfall gate is (ii)** -- the migrator self-check (declared_in - stored_out == rejection findings recorded, else ERROR and refuse) blocks always, closed threads included; an ATTRIBUTABLE bad row on a CLOSED thread keeps the ratified carry, printed by id and reason, never as a bare count; the fleet's outer gate is the accounting (source == store == view, both arms, rc 0) before any re-conversion commits. Not (i): reversing the ratified residue/carried split on release night puts the cost on the archive and invites editing ratified files to pass a gate. hv can overrule.
- **A RUN THAT MEASURED NOTHING IS NOT A PASS; A PASS IS A `VERDICT:` LINE AT rc 0.** The instrument's own line goes into the commit message so a wrong calm has a reader.
