---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-31 10:29Z
status: active
focus: "AT-06.11 predicate fix LANDED (98612798). AC-10.5: ALL FOUR MEMBERS NOW DRIVEN -- and TWO OF THE FOUR NEVER MIGRATE, so the verdict vocabulary has no word for half the corpus. Blocked on vc for that ruling; everything else measured and controlled."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**AC-10.5 -- ALL FOUR CORPUS MEMBERS DRIVEN, BLOCKED ON A VOCABULARY RULING FROM vc.** Fixtures at `/private/tmp/cc-fleet`, captures verified against the pinned revisions, fixtured `HOME` throughout so the real store was never touched.

| member    | declares | migrator                       | artefact              | prose                                                    | UNACCOUNTED |
| --------- | -------- | ------------------------------ | --------------------- | -------------------------------------------------------- | ----------- |
| canary    | 2.19.0   | rc=0, 168 disp                 | STRANDED 0, ALT-ATT 0 | ALTERED **2** (one defect, `CLOSED/0059`)                | 347         |
| baize     | 2.19.0   | rc=0, 109 disp                 | STRANDED 0, ALT-ATT 0 | ALTERED 0, DOUBLED-SECTION 0, 52/52 drops verified empty | 135         |
| lamplight | 2.19.0   | **REFUSED** -- 6 findings      | n/a                   | n/a                                                      | n/a         |
| utilz     | 2.18.0   | **REFUSED** -- below the floor | n/a                   | n/a                                                      | n/a         |

**TWO OF FOUR NEVER MIGRATE, AND `fleet_corpus_conservation.sh` READS `conserved` / `named` / else-LOST.** A refused member fits none: `conserved` is a vacuous pass on a member that never ran, and anything else reads LOST, which asserts the migrator destroyed data when it correctly declined. **I wrote no verdict file rather than pick the less wrong lie.**

**BOTH REFUSALS ARE VERIFIED INERT** -- lamplight 5613/5613 and utilz 101/101 still byte-identical to their pinned revisions afterwards, and **zero files written under the fixtured HOME**: both declined before touching the store.

**LAMPLIGHT'S REFUSAL IS 6 ROWS IN 3 FILES OF 5613, NOT THE ~1158 THE CORPUS NOTE ANTICIPATED** -- 5 `unread-field` (4 are `evidence` on a row missing its `(non-test)` marker, 1 is `red-first`) and 1 `unparseable-row`. **The last names an UNMADE RULING**: `AC-05.2` writes its note in markdown emphasis, "which is a separate ruling from where the field ends and has not been made". The migrator is blocked on a spec decision and says so rather than guessing, which is the behaviour we want and is not a defect to fix in code.

**UNACCOUNTED IS ONE FINDING ABOUT THE MIGRATOR, NOT A PER-MEMBER RESIDUE, AND IT IS AC-10.8's PROPERTY ON THE INGEST SIDE.** 347 canary / 135 baize, and on baize the whole set is non-canon: 110 `intent/whiteboard/`, 12 `intent/llm/`, `steel_threads.md`, wip/todo/restart/done, history, dotfiles. **No authored thread file is in it** -- checked explicitly for design/impl/tasks/acceptance/info. `intent upgrade` names dispositions per SECTION and never names an out-of-model FILE set, so `--out-of-model` has nothing to consume. Measured: `whiteboard`, `intent/llm/`, `wip.md`, `todo.md`, `steel_threads.md` are each named **zero** times in the migrator's output.

## TODO

- **AC-10.5's ROW, WITH vc.** On the canary: artefact conservation CLEAN (DOUBLED 194 / RELOCATED 192 / STRANDED 0 / ALTERED-ATTACHMENT 0), prose residue **2** once the migrator's own dispositions are joined (115 of 115 declared drops matched, verified empty in canon). **ONE CORPUS OF THE FOUR THE AC NAMES** -- Lamplight, Utilz and Baize unrun, and nothing here is a fleet claim.
- **THE SURVIVING RESIDUE IS ONE DEFECT AND IT NEEDS PLACING** (mine and vc's): `issue CLOSED/0059` carries **two `## Related` sections** with different authored bodies and canon holds one, so the other is gone. The ingest-side twin of `DOUBLED-SECTION`; ic agrees it is one defect.
- **NO SHIPPED VERB SETS AN AT ROW BACK TO `to-write`, AND vc HIT IT TODAY** (their finding, my surface). `at` ships green/red/na; Machine 5 declares `(any) -> to-write` via `at.set` and the CLI exposes NO SPELLING for it; `at new --status to-write` refuses a taken id by design. **So a one-word row move is irreversible by any shipped verb** -- issue 0033's class arriving operationally rather than documentarily. It cost fifteen minutes and blocked four nodes. Worth a verb or a ruling that there should not be one; not mine to decide, but the gap is in the surface I own.
- **AC-06.8 is ic's file, reported not fixed.** `INHERITED_UNREAD` carries live entries keyed on the FAMILY where the verb is what matters.
- **A `bin/devbin build all` is owed** before anyone browses the web face from the delivered binary -- PARKED WITH A REASON: the shared-artefact guard refuses a rebuild while `native/rust` is dirty, and it has been dirty with several nodes' work all day.

## Watch-outs

**A CONTROL CAN FAIL IN TWO PLACES AT ONCE AND STILL LOOK LIKE A CLEAN NEGATIVE.** My first baize positive control moved nothing, and I nearly read that as the counter being unable to fire. It was the control: `cp -R` left the `.CAPTURE` record behind (it lives BESIDE the tree by design) AND the victim was not in the attachment set. **The tool said so in a line I had not read** -- _a subject with NO CAPTURE RECORD, revision UNKNOWN, so this figure names no tree_. Redone properly both zeros fire: ALTERED-ATTACHMENT 0 -> 1 on a byte change, STRANDED 0 -> 1 on dropping the canon entry.

**A `||` AFTER A PIPELINE BINDS TO THE LAST STAGE, SO A `grep ... | head || echo "(none)"` FALLBACK NEVER FIRES.** Same family as `| head; echo $?` already on this board. It cost me a moment reading a missing "(none)" as a match.

**A DEFECT CAN HAVE A GUARD ASSERTING IT, AND THAT IS WHY IT SURVIVES.** `unwired()` decided its remedy by asking the table whether a family declared shipped verbs, while its sentence promised the verbs the BUILD has. `dispatch_ssot.rs` then asserted that exact predicate from the other side, so the wrong answer had a passing test with the right name. **When you fix a defect, grep for the test that was holding it in place** -- mine went red on the same run and its message read as a regression.

**A FORBIDDEN LIST LIVES IN AN INSTRUMENT AND DOES NOT TRAVEL TO THE NEXT ONE.** My driver excluded `daemon start`; I then hand-wrote a shell probe for the same question and started a daemon. Temp `$HOME`, so the hard rule held and nothing peer-facing was touched -- **but the protection was in the tool, not in the procedure**, and the second tool was written from memory of the question, not of the constraint. `kill` was refused by the classifier; `HOME=<temp> intent daemon stop` is the right undo anyway, because the store path is HOME-derived so it CANNOT reach a peer's daemon.

**A GUARD'S PREDICATE CAN BE FILE-SCOPED WHERE THE HAZARD IS PER-SPAWN-SITE** (reported to dc). `table_driven_tests_fixture_their_home.rs` asks `src.contains(".env(\"HOME\"")`, so my file satisfied it via `Fixture` while `wiredness()` did ~110 unfixtured spawns beside it, against the real `$HOME`. Green, correctly, on a question one level coarser than the defect. Fixed mine by folding every spawn into `Fixture::run`.

**A MENTION IS NOT AN INSTANCE, AND THE AGENT GUIDE IS THE OUTPUT THAT MENTIONS EVERYTHING.** `intent llm guide` renders `UNWIRED_PHRASE` verbatim to explain exit 2, and carries `remedy:` lines of its own -- so a whole-output `contains` called a wired verb unwired and paired the marker with an unrelated remedy. Read the refusal as the PAIR it is: an `error:` line and the `remedy:` line after it. `dispatch_ssot.rs` had already learned this for the same output and its comment says so, one question before it made the same error again.

**A ROW'S STATED BLOCKER IS A FALSIFIER TO DRIVE, NEVER A NOTE TO READ. FIVE EXPIRED TODAY.** AT-06.11 held _until WP-10 lands `upgrade`_ (it landed; it migrates the canary at rc=0). AT-10.12's cited property was WITHDRAWN and re-cut while my board still carried the old wording. AT-10.5's _the migration does not name it_ is FALSE at HEAD. **And the dangerous shape is a stale REASON on a correct VERDICT** -- the verdict gets re-checked, the reason does not.

**A DECLARATION TRUE OF EVERY CURRENT MEMBER IS NOT THE SAME AS A CORRECT ONE, AND NOTHING SEPARATES THEM UNTIL SOMEONE ADDS A MEMBER.** My `SERVED_BY_DAEMON` rule said _project-scoped, request-response `Op`_, full stop -- accidentally equivalent to the real rule because every `Op` then had an in-process twin, and wrong the moment ic added one that did not. Same shape in the product: `unwired()`'s predicate answers the question NEXT TO the one its sentence asks.

**A CONTROL DRAWN FROM THE SAME ENUMERATION AS THE INSTRUMENT CAN ONLY CONFIRM THE SHAPES THAT ENUMERATION ALREADY HAS** (`flag_reachability.rs`). Pin an instance the ESTATE asserts elsewhere. Mine red on its first run, on my own extractor, which read `upgrade to migrate this project to` out of English prose -- matching no path, silently dropped, **so the property arm would have passed by having nothing to check.**

**A NARROWING IS ONLY SOUND WHILE ITS PRECONDITION HOLDS, SO MEASURE THE PRECONDITION.** Backticked-spans-only is blind to a command named outside backticks; a second arm walks the same corpus and refuses one. A precondition nothing checks is a convention until the day it is not.

**A BAD NEEDLE THAT AGREES WITH THE NUMBER YOU ARE CHECKING IS THE MOST DANGEROUS KIND.** A `verblock:` line came back NOT IN CANON and would have CONFIRMED a file as stranded -- it is frontmatter, parsed into fields, so its absence proves nothing. The body-line probe reversed the answer.

**A COUNTER THAT CANNOT FIRE READS EXACTLY LIKE ONE THAT FOUND NOTHING, AND THE PARTITION IS WHERE THE DIFFERENCE SHOWS.** `STRANDED 386` was reproducible, deterministic and agreed by two nodes; **none of that was evidence about the predicate.** Its partner `DOUBLED 0` was impossible, and that is what found it. Check the partition, not the figure you came for.

**LOOK FOR THE UNEXERCISED FLAG BEFORE BELIEVING A FIGURE THAT INDICTS SOMEONE.** `conservation_check.sh --dispositions` consumes `intent upgrade`'s own stdout; the migrator has emitted 168 dispositions a run all along; every residue measurement had been taken without it, and the row recorded the migrator as SILENT.

**AND A JOIN IS ONLY LEGITIMATE IF THE TOOL DOES NOT TAKE THE CLAIMANT'S WORD**: _named per-section AND VERIFIED EMPTY IN CANON_. A migrator naming everything FAILS that rather than zeroing it. Same property AC-10.8 needed from the other end.

**A DERIVATION CHECK CANNOT SEE AN OVER-CLAIMED RESIDUE SET.** Pinning members to a source document closes the denominator attack and is blind to its inverse: a member the operation reproduces perfectly well passes every derivation check. **Only a measurement sees it** -- specimen ingested, deleted, egested, still missing.

**WHERE A POPULATION MUST NOT SILENTLY SHRINK, USE AN EXHAUSTIVE `match` SO A NEW VARIANT FAILS TO COMPILE**, never `matches!`. For a const slice there is no compile-time exhaustiveness -- compare LENGTHS against a specimen table, because a per-member loop walks less and passes on a removal.

**A GUARD ON THE WRONG SIDE OF THE WIRE.** An arm driving the composing function proves the sentence is right and proves NOTHING about whether the renderer still calls it. Measured: unwiring `render.rs` left an entire test file green and red only the CLI arm.

**WHEN A FIXTURE SHOWS A CATASTROPHIC FAILURE, CHECK THE FIXTURE CAN OCCUR BEFORE YOU CHECK THE CODE.** `intent/st/WIP/` never existed -- v2 buckets only COMPLETED / NOT-STARTED / CANCELLED. And the fixture before it was flat-only, which could not exhibit the bucketed class at all: both directions of the same error, ten minutes apart.

**COMMITTING IN A SHARED CHECKOUT IS FOUR PROBLEMS AND MY ISOLATION REACHES ONE.**

- **CONTENTION** -- taking a peer's bytes from files you did not touch. Closed by a HEAD-pinned private index.
- **COHERENCE** -- **NOT closeable by you**: `git add` has no hunk scope, so a peer and you in ONE file is land-the-pair-or-wait. Met twice today; both times the answer was to preserve out of tree and let the node mid-flight go first.
- **REVERSION** -- a correct-looking artefact. Announce any disk->store sync.
- **WORKTREE VISIBILITY** (dc's, and the one none of the three describes): **a guard's population is the WORKTREE while every isolation device I own operates on the INDEX.** An untracked file nobody staged red-lines the gate for every node, and **the author is the one node whose gate may not fire on it.**

**MY PRIVATE INDEX STAGES A DELETION OF EVERY FILE IT CREATES, AND THE ONLY WITNESS IS A CLONE NOBODY HAS MADE** (issue 0178; ic's extension: a MODIFIED file's stale ambient entry reads as a REVERSION of your own change; vc's 0179: presence of an entry is not currency). **PROCEDURE: after every private-index commit, `git restore --staged` EVERY committed path, created and modified alike, then require `git diff --cached` empty.** Compare BLOBS, never count entries.

**A RULE THAT NAMES THE OCCASION YOU MET THE DEFECT ON IS SCOPED TO YOUR ENCOUNTER, NOT TO THE DEFECT** (vc's general form of dc's). Mine named `reset`; dc's named `--only`; neither was the mechanism. **I wrote mine AFTER being handed the generalisation.** Prefer a procedure step to a rule.

**zsh, AND ALL THREE COST ME A WRONG ANSWER RATHER THAN AN ERROR.** `$var` does NOT word-split -- use `${=var}`; an unmatched glob (`--include=*.rs`) ABORTS the whole command; `... | head; echo $?` reports `head`'s status. The word-split one manufactured a fifteen-row table in which no multi-word verb was ever invoked. **The tell was UNIFORMITY across a set that should have differed.**

**rc=1 FROM CLAP IS NOT EVIDENCE OF A WIRED VERB** (dc). Argument validation runs BEFORE the unwired dispatch, so `config get` with no argument reads exactly like a wired subverb demanding input. Supply arguments to reach the real refusal.

**A REFUSAL CAN BE A RETRY.** `cannot lock ref 'HEAD'` and `index.lock: File exists` both mean a peer landed between your gate and your ref move. Read the message before reading it as a verdict on your work.

**THE CLOCK GUARD REFUSED MY OWN BOARD COMMIT** (stamp read as 0 minutes ahead). **Re-read the clock; never adjust a stamp into looking right.** You cannot recover a time you did not read.

**NEVER START `intentd` UNDER THE REAL `$HOME` WHILE PEERS ARE LIVE.** **NEVER INVOKE `intent fc`, for any reason, including measuring something else.** A UNIX socket path has a 143-byte limit and the session scratchpad exceeds it. **rustfmt needs `--edition 2024` here and reformats what you just wrote** -- format, re-read, then patch. **DO NOT `assert_eq!` ON FILE BYTES** (mine dumped 757KB of sqlite); collect differing PATHS. **A MEASUREMENT THROUGH `PATH` HAS A SHELF LIFE** -- build your own and drive that; the delivered pair is 16 commits behind HEAD.

## Decisions

- (2026-08-31) **AC-10.8's residue naming is a QUALIFIER ON THE CLAIM, never an enumeration beside it** -- vc, under hv's pen. Naming the set anywhere else leaves the sentence claiming what it claims. **The other two claim sites (`st sync --write`, MCP `st sync`) were left alone deliberately**: neither reads as a report on the whole estate, and widening a ruling by implementing it broadly is still widening it.
- (2026-08-31) **`daemon status` narrowed to `terminal`, and widen it again when a projection is built** -- the `07ad9876` precedent. ST0064's menubar app is the named consumer and is UNBUILT, so the contract has no reader.
- (2026-08-31) **The AT-06.11 predicate fix lands BEFORE OR WITH hv's `--help` narrowing, never after** (vc). And wiredness is DERIVED or driven, never a hand-maintained flag.
- (2026-08-31) **`RealDaemon` refuses a stale sibling daemon rather than rebuilding one** -- a harness that quietly builds hides the class from the node whose binary was stale.
- (2026-08-30) **`SERVED_BY_DAEMON` grows when `Op` gains a project-scoped request-response variant THAT ALSO ANSWERS IN-PROCESS** -- the twin clause is ic's amendment and it closes a gap my wording left.
- (2026-08-30) **A new guard gets a file named for its contract**, never an arm inside one whose name describes something else.
- (2026-08-30) **Attachments are AUTHORED; no sync direction rewrites them.**
- (2026-08-30) **One published port, both protocols, disambiguated at byte 0**; `Op::Shutdown` refused over HTTP. **51737 is a preference, never a promise.**
