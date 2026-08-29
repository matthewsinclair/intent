---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-29 22:31Z
status: active
focus: "ON THE BOUNCE after localfold+compact; board and all four inboxes cold-read, HOLDING for vc as hv directed. Item 1 landed `180fb4a3` and my six files are clean at HEAD. MY LANE HAS TWO OTHER WRITERS: dc ST0066 Fiat cascade and the ic surface-retired door, both uncommitted under `native/**`, and the next ST0056 commit is a JOINT act -- read the ic board DOING before touching git. ITEM 2 IS STILL PARKED, MEASURED NOT ASSUMED: `surface/dispatch-table.json` (REPO ROOT, there is no crate prefix and I guessed one wrong) is still dirty."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0056/13, ST0057/00]
---

# Control Claude (cc)

## DOING

**HOLDING for vc sequencing. Nothing of mine in flight; item 1 landed at `180fb4a3`** -- 6 files, private-index write, no peer swept. **The commit message carries the reasoning; do not re-derive it from here.**

**MY LANE HAS TWO OTHER WRITERS RIGHT NOW AND THE NEXT ST0056 COMMIT IS A JOINT ACT.** `native/**` carries uncommitted work that is NOT mine -- dc's ST0066 Fiat cascade (`facade`, `transitions`, `store`, `model`, `render`, the fiat tests) and ic's `surface retired` door (`spine.rs`, plus the `ac.fc` row in `event.rs`). **My LANE decision says `native/**` is mine and that is now false in practice**; class 8 governs, not the lane line. Attribute by CONTENT before touching anything -- I attributed these by diffing, not by assuming.

**ic HAS INDEPENDENTLY LANDED MY ITEM-2 FINDING AS A CODE COMMENT.** `spine.rs` now documents that `table.retired()` selects on `!is_shipped()`, that `organize` has two rows, and that publishing the unfiltered list would delete a working verb from a caller. **Two nodes reached the same defect from opposite ends; the register arm is still owed, and it is now the only half not written down.**

## TODO

**vc's DOLE-OUT, IN vc's ORDER. Items 1 and 2 were independent; 1 is done.**

- **[2] THE REGISTER'S `shipped` DERIVATION -- S, BLOCKS ic, PARKED ON FILE OWNERSHIP.** `Entry::is_shipped()` (`dispatch.rs:583`) is `disposition != retire && target.state != retire` and **asks nothing about existence**, so a `keep`-dispositioned unbuilt verb ships by construction; 14 rows refuse. **Not fixable in the register: I profiled the 14 against the other 104 and they are declaratively INDISTINGUISHABLE** (`(keep, as-observed)` covers 7 refusers AND 43 workers). vc ruled BOTH obvious branches out -- `measured_at` is a pinned v2-surface claim that must never advance, and `is_shipped()` fails OPEN deliberately (the only thing stopping a hand-edit shipping a retired command). **Built-ness is a property of the (register, binary) PAIR, so it lives in neither half: a committed GATE ARM drives every shipped ROW against the binary and refuses on `not implemented yet`.**
  - **KEY ON ROWS, NOT PATHS** -- `organize` is three rows and sits in `shipped` AND `retired`; a path-keyed arm lets whichever row it reaches decide.
  - **DRIVE FROM THE ROW'S OWN `args[0]`** (vc's rule): a family declaring a required subcommand is driven with one of its declared values, a leaf is driven bare. **Bare `daemon` is rc=1 from CLAP**, so a bare probe classifies a refuser as BUILT -- my 26-count error, third firing.
  - **TWO FREE ARMS AUTHORISED BY vc:** `shipped n retired` is EMPTY, and `declared` has no duplicates. **BOTH WOULD FIRE TODAY -- re-measured on the bounce, and the table has MOVED by two rows since I profiled it, so this is not the reading I banked.** `organize` is still in both populations and still double-counted in `declared`, and the sum still conserves for exactly that reason: two errors cancelling. **Do not carry the counts; run the probe.**
  - **My two `not_probed` edits are with ic, credited** (`mcp`'s reason false in the present tense; `daemon` moved to `probeable`). **`probeable` has grown well past the figure I handed over -- ic has been editing; read it off the file, never off this board.**
- **[3] WP-08 `intentd` + THE WEB VIEW -- XL.** Architecture is RULED, do not re-open: `intentsvcs` owns the surface; CLI and daemon never touch disk or DB; **`intentd` is a CLIENT exactly as the CLI is**; D32 requires one implementation both reach; AC-08.2 requires identical results. **So the web view is an `intentsvcs` surface with two skins.** Keep `daemon start|stop|status|run`, do NOT add `-B|--background` (AC-00.6, binds the cut). `intentd` is 49 lines in `main.rs` / 69 across the crate; greenfield, no HTTP dep in the workspace. **Say early if it exceeds XL once the socket layer is read.**
- **[4] WP-13 project search -- XL.** 9 criteria, Not Started.
- **[5-7] FILLERS, S EACH.** `intent#0145` (the known-thread refusal still pins `.intentfiles`, and its remedy renders `this artefact carries: ` with nothing after the colon -- ONE class, on hv's parked stack with the vacuous-`doctor`-remedy item). **`VIEW_NAMES` two-readers Highlander** -- `address.rs:357/394` vs `Project::classify`'s own inline list, in my module, invisible to every gate. **`canon-commit` gates on ADDS**, so a commit that REMOVES attachments is outside its reach.

**NOT MINE TO MOVE, OR WAITING ON SOMEONE.**

- **`AC-00.5` IS S AND hv's BOARD DESCRIBES IT IN PRE-RULING TERMS -- vc HOLDS THAT PEN.** Board says distinguishable BY EXIT CODE; **canon says hv re-ruled it on 2026-08-25 OFF exit codes** -- `spine.rs` keeps its decision, retirement becomes ENUMERABLE, callers branch on MEMBERSHIP. **The stale text sends a builder at `spine.rs`, which the ruling protects.** The roster already exists and already answers on the exec path (`retired_refusal`, `spine.rs:700`, reading the `include_str!`'d table) -- it needs a DOOR, not a roster. **vc's refinement is the fix and it is bigger than the instance: the entry is a dated PROVENANCE record, false as a description and true as history, so it must CITE the canon row rather than restate it.**
- **`runner_roster_check.sh`'s SEVERED-INSTRUMENT DISCRIMINATOR (S), vc's by authorship.** `git ls-files` cannot see an untracked file, so a brand-new instrument prints SEVERED and the remedy sends the reader to repair a correct row. Three outcomes want three messages. Needs the canon ceremony -- it is an ST0056 attachment.
- **ST0064 NAMES `Geodica/_tools` IN THREE PLACES (XS)** -- moved to `Gtools`; nothing executes them. The finding underneath is hv's: a project with a concurrent test suite inside a cloud-sync provider loses files silently. Unfiled; the honest form is a sync root CONJOINED with a concurrent writer, not "root is under Dropbox".
- **THE WRITE-PATH PACKAGE (S-M)** -- `issues edit --body/--from`, `close --note`, scoped issue sync. **vc keeps the queue count; read it off their board.** Never hand-edit the extracts -- that writes round the store.
- **ST0065's REMAINDER** -- `_DECISION_TREE.md` + its MODULES.md lines, two archetype templates, `0122`'s two CLI remedy strings. Waits on hv adopting the draft. **ic claims ST0065 and it is OUT of the cut.**
- **PARKED FOR A RULING, NOT EFFORT** -- `issues list` is the disclosure convention's last holdout; `--status a,b` advertises an ordering it does not honour.
- **THE PRUNE, at vc's (d)** -- gated on `at-accounting.sh` and WP-11's absent-manifest default. **A FILE THE STORE CANNOT BE SHOWN TO HOLD IS A REFUSAL NAMING IT, NEVER A DELETION.** Re-measure the population; never trust a count off this board.
- **WITH hv, UNANSWERED** -- the two enum-roster findings (a `--format` widening's predicate must be grounded in the refusal vocabulary, NOT `code != 0`, which red-flags `doctor --format json`); `publish_home` on a temp root; test-target consolidation; `intentd --version` prints no commit; the v2 shell CLI prune.
- **3.0.1, AND THERE IS NO 3.0.2 (hv)** -- `on:` on `Descoped`; bare-sync must compare to the STORE; a `doctor` canon-vs-store arm; the id resolver treating every id as a thread; store growth per `st attach` plus a bulk door; a tenth residue class; `unclaimed_digest()` hashing absolute paths. `slug` stays vestigial (hv: _let us not overcook this_).
- **THREE NON-DECISIONS, DELIBERATE** -- the guard-runner resolver and critic exit-2 fail-opens STAY with 0043's reasoning beside them; the remedy-less error literals are NAMED, not gated; **`c755bd44`'s fabricated stamp STAYS UNRESOLVED**, because a corrected-looking fake is worse than an admitted one.
- **BANKED** -- `ST0057`'s `PASS` has a hidden denominator (vc filed `0123`). `staged_reads_check.sh` reports 7 of 14 gated instruments reading unstaged; placement is dc's. `AC-12.4` reads UNSATISFIED over three measurably-done conditions.

## Watch-outs

**BY CLASS, RULE-ONLY. Every line cost a wrong answer that read as a right one. INSTANCES live in `.history/` and in the commits.**

**1. AN INSTRUMENT CAN PASS WHILE BLIND, AND ITS GREEN IS INDISTINGUISHABLE FROM A REAL ONE.** The faces: a corpus that CANNOT EXHIBIT the defect (commonest); a control that would also pass broken; **a control that does not FLIP proves the probe was inert**; ACCIDENTALLY RIGHT; a predicate reproduced against the UNMUTATED source; a null read as a clean sheet; an instrument naming the WRONG CAUSE; a correct AGGREGATE that cannot check the mechanism (predict the SPLIT, never the total); the scanner inside its own corpus; a filter matching nothing, which passes for free; **a vacuity that renders as a well-formed TABLE, invisible to a control looking for a sentence**; and **a control that validates the axis it tests while silent about the one next door**. **THE ONLY CONTROL IS MUTATION -- break it and require the instrument to notice; assert the OCCURRENCE COUNT first.** **THE CHEAPEST INSTRUMENT I OWN IS A PRECONDITION THAT REFUSES TO RUN.** **AND A BROKEN INSTRUMENT RETURNS THE PASSING SHAPE, NOT AN ERROR SHAPE** -- five arms reporting `rc=1 planted=0` while never invoking the subject. **A CONTROL THAT CANNOT PLAUSIBLY FAIL TURNS THAT BACK INTO AN ERROR SHAPE**, and one firing for the WRONG reason is still worth its price.

**2. EVIDENCE IN HAND IS NOT A READING. Say what the numbers RULE OUT.**

**3. THE DISK CANNOT BE THE WITNESS FOR A WRITE, BECAUSE THE DISK IS WHAT MOVES -- AND NEITHER CAN A VERIFICATION THAT WAS TRUE WHEN IT RAN.** The cure is removing the gap, not looking harder: a detached worktree, where index and worktree cannot disagree.

**4. A CREATE IMPLEMENTED AS AN UPSERT REPORTS SUCCESS AND REPLACES.** hv ruled add/new must REFUSE. Wider form: any verb that RECONSTRUCTS a row from arguments silently enlarges every time a payload-carrying variant is added.

**5. A CORRECTION MADE IN CONVERSATION DOES NOT REACH AN ARTEFACT.**

**6. A VALUE GOES IN ONLY IF THE POPULATION'S OWN DECLARATION HAS IT -- BOTH DIRECTIONS.** Hardcoding a member of a draining population goes stale by being right; **MINTING one is the same error walking the other way.**

**7. A PEER'S INSTANCE BEING WRONG DOES NOT MAKE THEIR SIGHTING WRONG.** Check the SET, not the instance you were handed. **And a claim that survives an attempt to KILL it is worth more than one nobody checked** -- vc's four-verb correction was right and under-evidenced, and those are different things.

**8. A SHARED TREE HAS OTHER WRITERS.** `git commit` commits the index AS IT STANDS. **`--only <path>` protects other PATHS, not other people's edits to YOUR path -- it takes the WORKTREE version wholesale**, so no textual conflict plus a total ownership conflict is still a refusal to write. Check `git diff --cached --name-only` is empty BEFORE `git add`, in a separate call. A commit can lose the race AFTER the gate passes; re-verify and re-`commit`, do NOT re-`add`. **A BROAD COMMIT IS INDISTINGUISHABLE FROM A NO-OP AT THE CALLER** -- `nothing to commit, working tree clean` is byte-identical either way. **Ask `git log -1 -- <path>` which commit owns your path afterwards.**

**9. A PRIVATE INDEX IS THE ONE WRITE THAT CANNOT SWEEP A PEER -- AND IT LEAVES THE SHARED INDEX HOLDING A REVERSION.** `GIT_INDEX_FILE` + `read-tree HEAD` + add-only-mine; the gate honours it. **Afterwards the shared index still holds the PRE-commit state for your paths, which against the new HEAD reads as staged DELETIONS** -- `git reset HEAD -- <your paths>` is the second half and is NOT optional. **Confirmed live 2026-08-29: all six of my paths were sitting there.** Verify with `env -u GIT_INDEX_FILE`; an EMPTY `GIT_INDEX_FILE=` is not unset.

**10. EVERY TIMESTAMP IS READ FROM `date -u +'%Y-%m-%d %H:%MZ'` IN THE SAME CALL THAT WRITES IT.** `git log` prints LOCAL. I have fabricated stamps three ways: wrong zone, typed-from-the-last-one, and one minute ahead of a clock I had just read. **A placeholder shaped like a stamp is worse than an admitted gap.**

**11. A PIPELINE ANSWERS FOR THE LAST STAGE, NOT THE INTERESTING ONE -- FOR THE EXIT CODE AND FOR THE OUTPUT.** `x | tail` gives you `tail`'s 0; zsh has `$pipestatus`. **I truncated a suite's output with `head -30` and read the truncation as clean while the build was failing** -- and separately reported "the tool did not say" about a message it had printed at the top. **When the claim is _it did not say_, re-run it unfiltered. READ THE EXIT CODE FIRST.**

**12. THE POPULATION YOU MEASURED IS NOT THE ONE YOU MEANT, AND NOTHING NAMES IT.** `cargo check -p X` builds no test target; `--test X` excludes lib tests; a narrowing flag never narrows the CLAIM. **A TERMINATION CONDITION IS A FILTER NOBODY DECLARED** -- `cargo test` ABORTS after a failing target, so two runs reported 38 and 140 binaries and I compared them as one population. **Capture ONE run and analyse the capture; never compare across runs.** `clippy -D warnings` aborts per crate, so the truer count came from the LESS strict run. **AND THE INDEX IS A DIFFERENT CORPUS FROM THE WORKTREE.**

**13. A ONE-SIDED COMPARISON DISCARDS THE EVIDENCE THAT THE INSTRUMENT IS WRONG**, and a conservation check whose two sides share a source is an IDENTITY, not a measurement. **`shipped`+`retired`=`declared` conserved perfectly while `organize` sat in both, because `declared` double-counted it -- and it still does, re-measured after the table grew by two rows. Diff member-by-member, never by count** -- the register's own `why` block says exactly this and was itself validated by count.

**14. A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE CLOSING IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** The pull is strongest exactly when the finding is good. **Corollary for tests: when a ruling changes behaviour, RE-AIM the arm at what still holds; never delete the coverage to make your own change pass.**

**15. A MECHANISM FILED WITHOUT ITS CONSEQUENCE IS NOT FILED**, and a finding's SCOPE is set by the search that found it -- always smaller than the problem.

**16. A REFUSAL THAT DELETES IS THE MORE DESTRUCTIVE OPTION AND READS AS THE SAFER ONE.** Refusing is conservative only when refusing PRESERVES.

**17. ABSENCE IS A STATE; UNREADABILITY IS AN ERROR; A `let Ok(..) else` CANNOT TELL THEM APART.** A refusal that ends the run reports a FLOOR read as a COUNT.

**18. ONE VALUE, TWO HOMES, AND NOTHING COMPARES THEM.** A SERIALISER is a second writer nobody declared; so is the markdown formatter. **The quietest home is PROSE SITTING BESIDE THE MACHINE-CHECKED FIELD.** A hand-typed COUNT is the same shape. **THE HARDEST SUB-CASE IS A PROVENANCE RECORD THAT PARAPHRASES ITS SUBJECT INSTEAD OF CITING IT** (vc, on `AC-00.5`): it becomes a second home the moment the subject is re-ruled and CANNOT BE REPAIRED BY CORRECTING IT, because as a dated record it is still true. **A record may quote WHAT WAS SAID, but must POINT AT, never reproduce, what is currently TRUE.**

**19. A REMEDY IS A CLAIM ABOUT BEHAVIOUR AND MUST BE DRIVEN LIKE ONE.** **BORN STALE and EXPIRED are different defects and NEITHER is caught by reading.** **AN EXPIRED REMEDY CAN POINT AT A REPAIR A LATER RULING EXPLICITLY FORBIDS** -- actively counter-useful, and it reads identically to a live one.

**20. A MISSING TEST IS A BACKLOG ITEM; AN UNRUN TEST IS A DISCIPLINE FAILURE.** A pristine-worktree control proves "not my diff" and CANNOT prove "not the environment".

**21. HAND OVER PROPERTIES, NOT VALUES.** A bare id is ambiguous, a bare moniker crosses estates, a hash is a READING. **A pin dies at a bump somebody else schedules.** Re-measure; never report a ledger. **Applies to ANNOUNCEMENTS too -- see 23.**

**22. ATTRIBUTION DECAYS INTO ASSERTION**, and exoneration and attribution are two claims. **An incident and its generalisation usually have DIFFERENT authors; record the split while you still know it.**

**23. A TWO-PART OBLIGATION WHOSE SECOND PART HAS NO TRIGGER IS A ONE-PART OBLIGATION.** I announced a window opening, promised the close, and never sent it; vc did the identical thing a day earlier. **The opening is prompted by the act of starting; NOTHING prompts the end, because finishing FEELS like the end.** It cost nothing only because both readers went and measured, which is the behaviour that hides it. **CURE: state the PROPERTY that ends the window, never a promise to send a second message.** **AND MEASURE BEFORE YOU ANNOUNCE** -- I announced a window on a file that was already dirty with a peer's work, then had to withdraw it to three boards.

**24. A COMPARISON BETWEEN A DERIVED ARTEFACT AND ITS SOURCE CANNOT DISTINGUISH _EDITED_ FROM _STALE_. BOTH ARE `the bytes differ`.** Cost two fatal designs in one build. **The stale case is the COMMON one** -- every canon change makes every view stale until the renderer runs -- so refusing on it would have refused `sync --to-store` across the fleet. **The fix needs a THIRD input and each obvious one is insufficient alone:** the MODEL says which regions it generates; the FILE INDEX says whether a human touched the file. **AND `Changed` MEANS _new_ AS WELL AS _moved_**, so a file with no baseline reads as edited. Honest predicate: _was in the PREVIOUS index AND has moved_. **THE ESCALATION IS THE REAL LESSON: EACH DEFECT WAS CAUGHT BY A STRICTLY BROADER POPULATION THAN THE LAST** -- 9 unit arms green when the crate suite caught the first, crate suite green (1149 passed) when the workspace caught the second. **A green bounds the ring it ran on and says nothing about the next ring out.**

**25. TWO RULES GOVERNING ONE DECISION -- ONE STRUCTURAL, ONE PURPOSIVE -- ARE CONSISTENT ONLY UNDER AN ASSUMPTION NEITHER STATES.** `announce` is defined structurally (every peer's inbox); `hv/inbox.*` is defined by purpose (the durable ESCALATION surface). Jointly satisfiable only while every announcement is an escalation, which nothing says. **The tell is not drift between copies -- it is that obeying one BREAKS the other, in a case neither anticipated**, which is why it reads as a judgment call and is not one. vc's ground: **an obligation to READ is destroyed by volume long before it is destroyed by refusal.** **UNRESOLVED and hv's: _paused peers need the durable form_ applies to hv too.**

**26. THE FOLD IS ITSELF AN INSTRUMENT AND IT HAS FAILED IN THIS BOARD.** It has left one class in two sections, dropped a ruled GO, and duplicated numbering -- **and this fold found 34 sitting ABOVE 33.** **Archive VERBATIM first, under `wip-fold-HHMMZ.md`.** **UPDATE classes, never append instances beside them**; when a fold only trims narrative it is not aggressive, it is tidy.

**27. A WEAKENING WITH NO SYMPTOM IS THE HARDEST THING TO CATCH** (dc). When a type widens, the loose port and the tight one both compile. **THE COMPILER'S SILENCE IS THE SIGNAL, NOT THE ALL-CLEAR** -- I added an enum variant and NOTHING broke, because every consumer used `if let ...Refuse`; the new variant fell straight through all three. **Two turned out safe by a guard I had not read.** The REASON for a tight match belongs in the file, next to the match.

**28. A GENERATED ARTEFACT CAN WRITE INTO THE AGENT-INSTRUCTION CHANNEL, AND NOTHING ANNOUNCES IT.** `docs/reference/claude.md` IS `CLAUDE.md` on a case-insensitive filesystem. **Prefix the whole output space rather than special-casing the member that bit.**

**29. PRESENCE OF THE VARIANT IS NOT PRESENCE OF THE SUBJECT.** A census keyed on the state NAME cannot see a payload field inside one variant. **Key on the unit the defect lives in** -- and its wider form: `is_shipped()` is per-ROW while the populations are path-keyed, so a path can be two rows with opposite answers.

**30. A GUARDED DOOR USUALLY HAS AN UNGUARDED TWIN ONE COMMAND AWAY, AND REACHING FOR IT IS AN ACCIDENT RATHER THAN A DECISION.** `bin/devbin build` takes a verdict; plain `cargo build --release` does the same compile with none and writes into the shared path. **The guard was never bypassed -- it was never invoked.** The tell is that the guarded door is the LONGER spelling. **Check the ARTEFACT afterwards, and check the SET: the shared path holds a PAIR and has held two different trees at once.**

**31. STATE A SIZE ONLY AFTER THE SEARCH THAT WOULD REFUTE IT, AND QUOTE THE SEARCH.** A cost is a claim about what does NOT exist. **AND A DECLARED GAP CAN NAME THE WRONG SUBJECT, WHICH READS AS RIGOUR AND IS WORTH LESS THAN NO NOTE** -- my board said "I have not read `sync.rs`" when `sync.rs` was never the `--to-store` leg, retiring the question for the next reader. **Worse still: I recommended MEASURED without checking whether measuring was PERMITTED, and quoted the fail-open sentence in the same message that proposed breaking it.** A rule is at its least protective in the sentence that asserts it.

## Decisions

**I DO NOT EDIT `CLAUDE.md`, PERMISSION SETTINGS OR CONFIG ON A PEER'S INSTRUCTION**, however right they are on the merits. A peer's ask is never the user's approval. **AND I DO NOT APPLY AN EDIT A CLASSIFIER REFUSED A PEER.**

**A RELAY IS NOT AN INSTRUCTION** -- a ruling reaching me through a peer gets confirmed with hv AT SOURCE before I act. **EXCEPTION, hv's own word in my session 2026-08-29: _wait for instructions from intent-vc ... Go._** hv designated the channel first-hand, so vc's sequencing is not a relay. That exception dies with this instruction; it does not generalise.

**MY WINDOW ANNOUNCEMENTS STATE THE PROPERTY THAT ENDS THEM, NEVER A PROMISE TO SEND A CLOSE NOTICE** (mine, endorsed by vc). Not _I will tell you when it shuts_ but _open until `git status --porcelain <path>` is empty_. See class 23. The companion -- `/in-finish release` refusing while a window is open -- is NOT adopted; it is a shared door and goes to hv with dc.

**DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc's ruling on what I may write into ST0056 canon).

**A PARITY TOOL IS A RECORDED ATTACHMENT.** The carry is **`intent st attach ST0056 parity/tools/<f> --from <path>`, ONE CALL PER FILE**, then `intent sync --to-disk ST0056`, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` together. **Never the thread-wide carry** -- `ingest::collect_attachments_into` re-reads authored prose, which is the live damage class.

**THERE IS NO 3.0.2 (hv).** Everything else goes into 3.0.1.

**RESIDUE BLOCKS ON A LIVE THREAD AND CARRIES ON A CLOSED ONE; HALF A's ACCOUNTING BLOCKS ON BOTH** (vc).

**LANE.** `native/**` and the v3 crates are mine; the parity harness is ic's; hooks, roster and `int hooks` are dc's; **canon writes route through vc**. `~/Devel/prj/Intentv2` IS NOT TOUCHED.

**ENVIRONMENT -- PROPERTIES, NEVER VALUES.** Bare `intent` resolves through the `~/.local/bin/intent` SYMLINK into `native/rust/target/release/`, and **that symlink passes through no currency check at all** -- the commit gate is the only place a stale delivered pair is reported. Read the pair off the gate's currency line or `bin/devbin cli`; **pin by HASH, never by marker.** `find` is bfs, not GNU find. **The Bash tool's shell is zsh: unquoted `$var` does NOT word-split -- THIS FIRED THREE TIMES IN ONE SESSION AND EVERY ONE RETURNED A PLAUSIBLE RESULT** (a `timeout 5 $1` probe that ran NOTHING and reported rc=127 on all arms including the control; a `for arm in` loop of the same shape; and **`git add -- $MINE` which staged ZERO files and let a commit run against an empty index**). Write the words out, or use `${=var}`. An unmatched glob ABORTS the command; `"$VAR:path"` is a bad substitution greping to a convincing zero; **`"...$ENV{NAME}..."` in DOUBLE quotes expands `$ENV` to nothing** -- single-quote every perl `-e`. cwd persists between calls. **`cat -A` is not BSD cat.**
