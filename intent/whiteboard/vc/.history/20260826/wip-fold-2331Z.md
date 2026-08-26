---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 22:48Z
status: active
focus: "BOSS-VC. SERIAL PORT of the fleet to v3 on hv's order, controlled from this session on main at HEAD, hv watching every command. Closed: Anvil, arca_notionex, Baize, Cdsync, arca_config. Blocked on cc (three legacy.rs defects): Conflab (multibyte panic), arca_cli (prose covers as ids; `path, describe` kept whole as file). Pair pinned at 43cee2be (the advisory fix, mine, landed as 01dec831+43cee2be). Next: Courses."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**THE SERIAL PORT (hv, first-hand, this session, 2026-08-26 between my clock reads 21:45:23Z and 22:24:23Z).** hv's tiers, verbatim headings: **Important** (_"Full live port, no detritous, pristine 'intent doctor'"_): Arca/arca_config, Arca/arca_cli, Baize, Cdsync, Conflab, Courses (_"its done at the Courses/ dir, not subdirs"_), Devbin, Intent, Laksa, Lamplight, llm-tropes, MicroGPTEx, Molt, Pplr, Prolix, Riffle, **Utilz** (_"a miss. That's definitely in the Important category"_). **Port** (_"to Intent3, run doctor, check in and commit, then move on"_): A3, Anvil. **Ignore** (_"Not Intent projects"_): Molt-flynn, Molt-matts. **Purposely held at v2:** Intentv2. **Content** (_"Not ported, subdirs are used in other projects"_): Sites/**, Wrightings/**. Open with hv: llm-tropes and Pplr have NO intent/ tree (an adoption, not a port); arca_notionex (closed) sits in no tier. **The ledger, the procedure and the LEARNED bullets are in `cutover-runbook.md` under PORT LEDGER.** Rulings in force: buckets to `intent/history/`; `deps/`, `_build/`, `node_modules/`, `.worktrees/` are not projects; a WP recorded WIP over a passing gate is `wp done` (Baize ST0025/02); the backup schedule is configuration and is NEVER silenced (hv to dc, direct); doctor advisories are printed, never counted (hv: _"Fix. Do it here"_ -- done, `01dec831`+`43cee2be`).

**THE PAIR:** `$S/pair-43cee2be/{intent,intentd}`, built CLEAN in a detached worktree's private target (stamp `43cee2be` both halves; `cargo clean -p intentd` was needed -- cargo does not rerun a build script whose crate did not change, so the previous stamp survives a rebuild). hv rebuilt `target/release` at 22:5xZ on a clean main; `~/.local/bin/intent` and `~/bin/intent` follow it, **bare `intent` is still the keg `80d8b2ca` (PATH 1) and prints the pre-fix doctor** -- told hv; `int local use dev` or 3.0.1's keg is hv's call. THE FLIP HAPPENED 2026-08-26 14:00:46Z on my word (tag `v3.0.0` = `80d8b2ca`).

**STATE PER NODE:** dc -> `bc6c1637` in (backup schedule from `backup.schedule`, back-fill through `upgrade`, verified on arca_config: `"backup": {"schedule": "daily"}`), `f06d444b` Intent's own config by hand; ic -> `c14aa9bf` in (watermark is an instant), and ic's D53 question is HELD for hv below; cc -> holding, THREE legacy.rs defects waiting on hv's word to assign: (1) `unread_field_keys` slices at a non-char boundary (Conflab, `✓` in `ST0121/acceptance.md:72,83`); (2) prose `covers` clauses carried as ids (arca_cli ST0011 AT-07.3 "the gate itself", AT-11.6 "the reachability half of AC-11.1", AT-13.4 "the seam itself"); (3) `path, describe "…"` kept whole as `file` (arca_cli ST0011, 16 rows -> 16 "cites a file that does not exist" -> five WPs Done-but-BLOCKED). After cc's pair: re-convert ST0011 from the v2 source, then arca_cli's port; rehearse Conflab on the clone again, then port it.

## CRITICAL PATH

1. hv's word on cc for the three; then Conflab + arca_cli.
2. Courses (pre-read done), Devbin, Intent (ST0055's five `wp done`, the PASSES-branch remedy text, my two shell reds), Laksa, Lamplight, MicroGPTEx, Molt, Prolix, Riffle, Utilz; A3 (Port tier); llm-tropes / Pplr on hv's ruling.
3. THERE IS NO 3.0.2. 3.0.1 collects: the three legacy.rs fixes, `config get|set`, `st repair`, the daemon snapshot or a manual ruling, the PASSES-branch remedy text, D53's answer.

## HELD FOR hv

- **ic's D53 question (verbatim options):** the two constant reds `a_flush_survives_a_machine_that_has_no_database` and `doctor_does_not_report_a_flushed_view_as_hand_edited` ask "does a flush survive a git clone?" -- under D53 (`f42987c7` took `events.jsonl` out of the tree) it does not. (a) it does not travel and the tests are rewritten to that contract -- every fresh clone of a flushed project reports `todo.md` hand-edited, permanently; (b) the committed `## DONE:<T>` heading is read as a FALLBACK when there is no store -- on a store-less machine doctor can no longer catch a hand-edited watermark. Not mine to answer.
- The daemon's scheduled snapshot vs manual; `intent config get|set` and `st repair` are stubs; `finding.rs:348` puts the BLOCKED remedy under the PASSES finding.
- Lamplight: 22 non-UTF-8 files refusing `--to-store`; the two stale `.worktrees/{cc,ic}` at 2.19.0; ST0270's covers reading.
- Mine, at a quiet moment: `verify-canonical.sh`'s `.backup/` arm reads the root only.

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
