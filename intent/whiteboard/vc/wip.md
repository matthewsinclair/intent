---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 21:02Z
status: active
focus: "**BOSS-VC for the v3 fleet cutover under hv's pen. HOLD: hv builds HEAD on main and runs the full suite while every workstream waits. 2b green at 03470c5a; d6ddb874 (stamp subject = the build's inputs) and a8fc134b (todo parity, three tests red and named) on top; B1 two proofs unarmed, re-run owed on hv's go; the third migration defect in cc's ruled order for 3.0.1; Laksa carry running on a pinned pair; Lamplight carry HELD for hv. THERE IS NO 3.0.2; WIP ONLY.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**LOCALFOLD on hv's word at the stamp in the header; HOLD: hv is doing a clean build of HEAD on main and then the full suite while every workstream waits. Nothing commits, nothing builds, nothing writes under native/rust or surface, no long run starts, until hv says go.** Executable state: `cutover-runbook.md` (LEARNED, in order -- read the last thirty bullets). Every long run of mine uses the PINNED copy at `scratchpad/pair-03470c5a/` (sha256 7d26ff62..., marker 03470c5a) -- my scripts refuse any other binary (`0c5ac8c6`).

**WHERE THE PAIR STANDS:** 2b green at `03470c5a` (dc, five reads verbatim, 20:24Z). Since then HEAD carries dc's stamp fix `d6ddb874` (DIRT_SCOPE = native/rust + surface; identity over the same scope; empty-rev-list guard; arm 6c) and ic's todo parity restore `a8fc134b` (the DONE watermark grafted onto HEAD's nesting; `todo done --flush/--prune` back; TodoWindow removed; the two B1 proof fixes) -- **committed RED on purpose with three failures NAMED** (`a_flush_survives_a_machine_that_has_no_database`, `doctor_does_not_report_a_flushed_view_as_hand_edited`, `the_structured_buckets_and_the_rendered_view_say_the_same_thing`: ic's comment asserted a limit the restored tests disprove -- `export.rs` still ships events.jsonl; follow-up on hv's go, never a rewrite); 13 more fail only in a detached worktree (they need an installed `intent` on PATH; hv's real-project run is the control); clippy NOT run on that sha. **hv's own `todo done --prune` on the rebuilt binary produced v2's behaviour** (54 completed items streamed, watermark advanced) and its `todo.md` is committed (`5c4e9e01`). **hv WILL MEET TWO NEW THINGS IN THE BUILD: (a) since `d6ddb874` a dirty `surface/` stamps `dirty-` for the FIRST TIME EVER -- the fix working, not breakage; (b) a dirty `native/rust`/`surface` does NOT block the build, it REDIRECTS it to `target/private/release` at rc 0 and leaves the shared pair at 03470c5a -- so the tree must be clean at build start.** At the fold cc's residue-class set (`finding.rs`, `legacy.rs` at cc's restored sha d6199fc7, `tests/legacy_unread_field.rs`) is still uncommitted: my commit of it was BLOCKED by the gate, and `intent/st/ST0056/migration.md` went dirty in the same minute -- handed back to cc to land with that fix.

**B1 SECOND-OPERATOR VERDICT at 03470c5a:** ARMED AC-11.1 (both tests), AC-11.2/present, AC-11.5; **PROOF UNARMED** AC-11.2/tty (the test read only the exit code) and AC-11.4 (the fixture's manifest was present, so the verb never acted). ic's fixes are in `a8fc134b`; **RE-RUN OWED by me on hv's go**, ic's named mutations: tty guard -> `if false` must red on the REASON text; `declare_default` calling `organize(Mode::Apply)` after the write must red now the test starts from an absent manifest; `views::in_done_bucket` `completed >= mark` -> `true` must red `todo_watermark.rs`'s passing membership arms. AC-11.1/11.3/11.4 now read WIP ONLY in canon (`f6791a47`, round-tripped through the store).

**THE THIRD MIGRATION DEFECT (cc, 3.0.1, order ruled under my pen):** (1) unknown-field residue as the CLASS -- cc's step (1) is built (`FindingClass::UnreadField`, `field_key` + `unread_field_keys`, AT scan bounded at `-- status:` because 118 of 124 unknown AT keys live inside note's region; 5 tests; 3 of 4 mutations driven); (2) arm 1 + arm 2 TOGETHER -- `legacy.rs:1275` literal `(non-test)` prefix discards 39 hand-set satisfactions on Lamplight; `withdrawn:`/`descoped-to:` are fields never read, 22/22 dispositions fleet-wide arrived as prose; refuse never reclassify; `by`/`on` carried never minted; acceptance ST0346 gates 24/24; (3) the covers `+` parser against 12 fixtures. Fleet arm 1 after the field-presence check: Lamplight 17, Laksa 1, Baize 0. Also on cc's list: bare `sync --to-disk` re-emits only what it touched (per-id sync after any git revert of the extract); doctor has no canon-vs-store arm; the ingest walk refuses an estate on a PNG (22 non-UTF-8 files block Lamplight's `--to-store`); `sync --to-disk <id>` resolves ids as steel threads only; `issues add` has no body door; `st attach` grows the store ~2 MB per small file and needs a BULK door; `is_criterion_id` accepts `AC-0.2`; the attachment cap has no per-thread total.

**THE CARRY:** Laksa RUNNING on the pinned pair (`tasks/bw5wkexia.output`, `$S/ingest4-Laksa`; 278+ of ~622 attached, 0 refused, every file byte-verified; slowed to 4/min under a load of 40); Laksa's store 1.87 -> 2.46 GB. **Lamplight's 3,738 is NOT a tonight job by `st attach` per file (HELD FOR hv, recommend the 3.0.1 bulk door).** Bucket prose carried and proven on Cdsync, Baize, arca_notionex, Utilz, Anvil, arca_cli, Riffle, MicroGPTEx, Courses, Molt, Prolix; arca_config green (devbin-vc). Fleet `--default` sweep: 13/16 `present, unchanged, rc 0` (AC-11.2 live); Prolix/Molt-flynn/Molt-matts refused as dirty, since resolved (Molt manifests committed; Prolix's formatter wart named).

**STATE PER NODE:** cc -> step (1) built, uncommitted at the fold (handed back), holding on its user; ic -> `a8fc134b` in, red-named, frozen; dc -> `d6ddb874` in, smoke arm `53699d66` in, frozen; devbin-vc -> paused (arca_config green, 0098 filed); lamplight-vc -> paused (`close-the-nine-final.sh` gated on hv's go); hv -> building and testing HEAD on main.

## CRITICAL PATH (the bounce checklist)

1. **hv's go after the clean build + full suite.** Read hv's build output: the dir the build named (a redirect line = the tree was dirty at start) and the stamp (`dirty-` on a dirty surface/ = the fix working). Read hv's suite result: the three named watermark tests are ic's follow-up; the 13 PATH-dependent ones should be green in the real project; anything else is new and gets named before anyone builds again.
2. **On hv's go:** cc lands step (1) (+ migration.md's residue class) with a sha and the fourth mutation; ic lands the watermark follow-up; I re-run ic's three named mutations + cc's four at their shas in a worktree (`mutate-at.sh`, private target); dc builds 2c under the QUIET protocol (both scopes 0 dirt, 0 cargo, same minute) and posts the five reads; then Lamplight's triage store-half and carry per hv's rulings.
3. **Laksa's carry result** when it lands: refused 0 / verify-failed 0 / kept dirs named; store size after.
4. 3.0.1 continues in cc's ruled order; dc's staging-dir build (measured); hv's `--default --force` per project at a tty when hv chooses; the cut on hv's go. THERE IS NO 3.0.2.

## HELD FOR hv

- **Rulings on hv's board (HELD, from tonight):** Lamplight's Triage store-half (22 non-UTF-8 files refuse `--to-store`; wait for cc / move them / rule them out); Lamplight's bucket carry sizing (bulk door vs overnight vs history); Conflab NOT MIGRATED (issue 0098); the staging-dir build (dc, measured: 252 refusals); the attachment cap's missing per-thread total; ic's DONE-heading wording deviation; Lamplight decision #5 (symlink on PATH) in the light of the 252.
- **Delegated rulings hv can overrule:** cc's build order; dc's stamp-fix sequencing (now landed); the S guard now / the M guard with the walk; the create door's both-flags-refuse; Class B ranges refused by name.
- **Open:** cc's step (1) commit; ST0270's disputed covers reading; `ST0064/01`.

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
