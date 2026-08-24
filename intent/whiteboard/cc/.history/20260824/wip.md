# Control Claude (cc) -- archived 2026-08-24

**Sections for 2026-08-21, 2026-08-22 and 2026-08-23, rolled out of the live board at the 2026-08-24 localfold.** Live board keeps the header, the model, all of 2026-08-24, intent#0070, TODO/BLOCKED and every Watch-out.

## Mine -- what I actually did, 2026-08-21 afternoon

**THIS SESSION IS THE BOUNCE. `87048274` != `ef9e17d5`** -- the one discriminating field, observed about MYSELF, which is the only way it can be observed. **vc's later correction matters and is now the rule: the field is ONE-WAY.** id CHANGED means genuinely new; **id UNCHANGED means resumed OR compacted**, because `/compact` does not rotate it (vc drove it across a compact, byte-identical). A binary answer two causes both produce is not evidence -- ic's rule, applied to the instrument we adopted to replace `ListAgents`.

### LANDED

- **`d8dd6dc6` -- the roster widening (hv's ruling).** `runner_roster_check.sh` population goes from 18 to 51 (now 52), from ONE directory and ONE filename shape to every `.sh` under `intent/st/*/parity/tools/` across both threads. **11 gated / 24 manual / 17 not-an-instrument.** Third kind added, reason required on EVERY kind.
- **`f6face5f` -- restored the executable bit** that `d8dd6dc6` silently dropped, 100755 -> 100644.
- **ST0057 `AT-03.6` GREEN. THE GATE IS 64 OF 67 AT HEAD `6edbd24f`, DRIVEN CLEAN-ROOM** -- detached worktree, no store present, `sync --to-store` ingesting HEAD canon across 57 threads, then the verbs: ST0057 49/51, ST0056/03 15/16. **Store and HEAD AGREE now that ic landed both uncommitted greens at `6edbd24f`.** **Before that commit it was 64 store / 62 HEAD, and the split is kept here because the lesson outlives it: a bare gate figure is a defect, and the store is gitignored so `ac status` can answer about state no clone can reach.** Criterion count moved 48 -> 49 when I greened AT-03.6, so the row WAS the last one for AC-03.6. **ic DECLINED to publish 64-at-HEAD by inference from `no canon is dirty` -- inference from a correct premise is what produced 63 twice -- and handed me the drive. That refusal is worth more than the number.**
- **AT-03.6's note rewritten** because it opened `STILL RED` on a green row and carried two more dead claims.

### THE FINDING THAT MADE THE WIDENING WORTH DOING

**The roster's population and the filename convention were THE SAME POPULATION** -- the declared set was exactly the 18 `*_check.sh` files, both `comm` directions empty. **So the guard was DEFINITIONALLY blind, not accidentally so, and no amount of adding rows under the old needle could have revealed it.** Its real needle was narrower still: 10 of 51 adjudicated. ST0057's whole toolset was invisible, **including `canon_ignore_dispatch_rig.sh`, whose verdict gates AC-01.5 -- a row still blocking the gate.**

### THE BOUNDARY IS MINE AND HAS NO RULING BEHIND IT -- CHALLENGE IT

An instrument **answers a question about the estate and returns a verdict**. Everything else -- sourced libs, generators, extractors, capture drivers, stubs, transformers -- is `not-an-instrument`. **vc's citation test is the mechanical form and it BEAT MINE: does the file's own header cite an AT or AC row it covers?** Driven over all 33: 12 cite, 21 do not. **I tried exit-1 presence first and it MISFIRES** -- `gen_dispatch_table.sh` and `gen_pertest.sh` carry `exit 1` as a usage refusal. **The citation test caught three of my own misclassifications** (`estate_census`, `estate_corpus`, `of_n_population` all cite criteria); all three would have been silently deleted from the population. **Under uncertainty the row goes `manual`: a wrong `manual` costs a reason and stays visible, a wrong `not-an-instrument` generates no signal ever again.**

### AC-03.14 -- BUILT, GREEN, AND THE ROW IS SATISFIED

**THE COVERING ROWS ARE `AT-03.15` AND NOW `AT-03.19`. NOT `AT-03.14`** -- systematic offset in this WP from `.11` onward, `AT-03.N` covers `AC-03.(N-1)`, and `AT-03.14` is a GREEN row about AC-03.13. Looking up the same number returns a green row for a different criterion.

**`intent-cli/tests/cli_write_moves_only_what_changed.rs` -- 13 CLI verbs, real binary via `CARGO_BIN_EXE_intent`, fixture built BY the binary (`init` + six `st new`), estate snapshotted `(path -> bytes, mtime)` around each verb.** Six threads rather than one deliberately: on a one-thread estate _rewrote everything_ and _wrote what changed_ are the same set, so the assertion would hold against the very defect it exists to catch.

**ONLY THREE OF THE TEN NAMED VERBS WRITE.** `st sync --write`, `todo done`, and `todo update` -- which was never on the list and is the todo family's actual mutator. The other seven write nothing, driven:

| verb                                        | rc  | what driving it showed                                                                                                       |
| ------------------------------------------- | --- | ---------------------------------------------------------------------------------------------------------------------------- |
| `st bootstrap`, `st repair`, `st dehydrate` | 2   | `is a known command that is not implemented yet`                                                                             |
| `at lint --fix`                             | 1   | `not implemented in v3` -- the flag that earns `at lint` its `mutate` classification does not exist                          |
| `todo`, `todo list`                         | 0   | driven with `intent/todo.md` ABSENT they print the view and do not create it, which is the write their own `--help` promises |
| `ingest`                                    | 0   | reads, creates no store, driven from a storeless start                                                                       |
| `todo notdone`, `todo toggle`               | 1   | refuse in EVERY state driven -- triage thread, wip thread, not-started WP, and a genuinely Completed thread                  |

**SO THE TEN DID NOT REDUCE BY BEING EXCUSED. THEY REDUCED BY BEING MEASURED, and the measurement is a population defect in the dispatch table.** Six shipped `mutate` entries write nothing at all. New bucket `MUTATE_BUT_WRITES_NOTHING` in the facade instrument, membership DRIVEN not asserted: the day one starts writing, the file reds and forces the re-bucket.

**THE RATCHET MOVED DOWN WITH THE SHRINK: `UNPROVEN.len() <= 32` became `<= 22`.** A bound left at its old value permits the exact regrowth it was installed to prevent while still reading as a ratchet. **Tightening is part of discharging, not a tidy-up afterwards.** The 22 that remain are exactly the ones AT-03.15's note already ruled out of subject.

**AND THE DERIVED ROSTER MOVED 64 -> 68 ON ITS OWN** as the dispatch table changed under it. A hand-authored denominator would have stayed at 64 and said nothing.

### THE GUARD FIRED ON MY OWN CASE, UNPLANTED, ON THE FIRST RUN

`todo done` was declared a writer, exited 1 on `st.done is not a legal transition for ST0002, which is triage`, and left the estate untouched. **A snapshot-only driver reports that as a clean pass** -- the verb wrote nothing, and _wrote nothing_ is what passing looks like. Both limbs caught it: the rc limb and the must-have-written limb. **The fix was a fixture prep, never a relaxed assertion.**

**A SECOND UNEARNED CLAIM IN THE SAME FILE, CAUGHT THE SAME WAY:** the `todo list` case's stated reason asserted store materialisation while its prep never removed the store. The reason was TRUE and the case did not drive it. **A `why` that describes a measurement the case does not perform is the row-promising-more-than-it-delivers shape, inside the instrument built to catch it.**

**AND AN rc IS NOT A REASON.** `rc=2` is both the unimplemented arm AND clap's missing-argument refusal, so `st dehydrate` driven bare would have passed on the parser's refusal while claiming the verb's. Every unimplemented case now pins the stderr phrase as well as the code.

### ic's CATCH: A LOAD-BEARING PROPERTY LIVING ONLY IN A DOC COMMENT

`flag_reachability.rs` duplicates the unwired phrase as a literal **deliberately, so that no copy is authoritative** -- and its comment said _duplicated from `declared_but_unwired.rs`_, naming that file as the origin. **Retiring it would not have removed a copy. It would have PROMOTED the survivor to authoritative -- the exact outcome the reasoning existed to prevent.** Fixed: the comment names the sibling copies and states the invariant (**at least two survive any retirement**) rather than an origin.

**NOTHING WE OWN TRACKS A DEPENDENCY RECORDED IN A DOC COMMENT.** `at lint` sees AT-row citations and sees nothing here. **A documented coupling is worse than an undocumented one, because the documentation is what persuades the next reader not to re-check it.**

### SEQUENCING WITH ic, AND THE RULE IS ic's

**`st dehydrate` is driven HERE BEFORE `declared_but_unwired.rs` retires, never after.** An overlap is visible, self-healing, and fails loud if either driver is wrong; **a gap is silent and every suite inside it is green.** `fixed is not a state` applied to a two-node sequence: there must be no instant where the claim is nobody's.

### AC-08.5 -- BUILT AND LANDED `3f9b2907`, ic COVERS

**`put` on `Thread{id}` splits on EXISTENCE.** Update lands; create-by-id keeps the `this id is server-assigned` refusal, which is correct for it; `Threads` and `Issue{..}` untouched. **ST0011's `completed` has a write path for the first time.**

**CHILD KEYS ARE REFUSED BY NAME, hv's user ruling.** `wps`/`criteria`/`tests`/`attachments` are all `#[serde(default)]`, so parse-and-replace turns a body that did not MENTION `tests` into a thread with none -- the criterion's own second limb, arriving inside the change meant to satisfy it. **Ignoring the keys is the same failure pointed the other way**: a caller who sent `tests` and got a success was told their write landed.

**THE FIELD NAME IS NOT THE ADDRESS SEGMENT AND MY FIRST DRAFT PRINTED AN ADDRESS THAT DOES NOT PARSE.** Model says `tests`, grammar says `at`. `threads/<id>/tests/<AT>` reads perfectly and sends an operator to a parse error **from the tool that just sent them there**. Now mapped explicitly, with a driven arm asserting every printed remedy parses.

**`put` DOES NOT CONSULT THE TRANSITION MACHINES -- hv RULED THE VERBS OWN THEM.** The note is in `facade.rs` because the dispatch table describes VERBS and `put` is not one. **The consistency argument carried it, not safety**: `at.put` already writes `status` this way. **And the note says plainly that the exposure is zero because nothing calls it, NOT because the write is checked** -- 17 `.put(` sites, every one a test. A future reader who finds a production caller has found a live gap.

**STILL OPEN ON AC-08.5, stated so ic can overrule rather than inherit:** the attachment canon record has no narrow setter -- **`put` has NO `Attachment` arm at all**, and `put`'s format guard exempts attachments from the markdown refusal for a path that then falls into `other => has no write path yet`. **A guard defending a door that does not exist.**

### MY TWO TIMING ROWS -- dc's 50ms HYPOTHESIS DOES NOT TRANSFER, DRIVEN

Clean harness (one python3, `perf_counter` around `subprocess.run`), floor **5.2ms**, at `bb3dce99`:

| tool                        | recorded               | re-measured                        |                    |
| --------------------------- | ---------------------- | ---------------------------------- | ------------------ |
| `thread_view_skew_check.sh` | 130-150ms @ `f0c2805c` | **median 167**                     | HIGHER, not lower  |
| `view_skew_check.sh`        | 2860-2940ms            | **median 3806** (spread 3409-4275) | HIGHER, 25% spread |

**A 50ms harness artefact would have made a clean re-measurement LOWER. Both came back slower.** So the figures were not harness-inflated -- **but they are stale on a different axis: this machine runs four Claude sessions and cargo builds, and a 25% spread is the signature of CONTENTION rather than of the tool.** Neither figure is wrong; they answer different questions. **The ratio the roster exists to support survives: ~23x against the recorded ~20x.**

**THE GENERAL DEFECT: every timing row here carries a revision and a machine and NO LOAD CONDITION** -- which is uncomparable on a machine shared by four sessions all day. Not fixable by re-timing. **Two rows to edit, and dc goes first: they are already blocked behind vc's canon line and I will not be a second writer into `runner_roster_check.sh`.**

### CHECK D -- AN INSTRUMENT SHIPS EXECUTABLE (`4c9d9d3e`)

**The file held the convention in PROSE and never checked it.** Four rows read _sourced, not executed; ships 644 and defines functions only_ -- so 644 vs 755 is how this roster tells a library from an instrument, in its own words, **with nothing watching the join.** That is the class the whole file exists to close, sitting inside it.

| arm                                               | result                                                         |
| ------------------------------------------------- | -------------------------------------------------------------- |
| every `gated` + `manual` is `100755` in the INDEX | measured **35 of 35** before asserting                         |
| `not-an-instrument` NOT checked                   | measured **4 at 644, 13 at 755** -- no invariant exists        |
| control                                           | green                                                          |
| a gated tool -> 644 in the index                  | fires by name, rc=1                                            |
| a not-an-instrument lib -> 755                    | **correctly SILENT** -- the arm proving it is scoped, not loud |

**Read from git, never `stat`: the mode git RECORDS is what a clone gets**, and a file can be 755 on this disk and 644 in the index -- exactly what a rename over an already-staged file leaves. **Mutation-proven in a DETACHED WORKTREE, never the shared index**, because `git update-index` writes `.git/index` and peers commit from it.

**AND THE PROOF WAS RE-DRIVEN AFTER THE SHELL CRITIC FORCED A REWRITE.** It blocked `set -- $row` (IN-SH-CODE-001) and was right; `read` now splits the fields. **A proof taken before an edit describes a revision that is not the one shipping** -- and the first proof was already in hand, which is exactly when it is tempting not to re-run it.

**dc SWEPT THE ESTATE AND THERE IS NO LIVE INSTANCE: 284 parity files from the INDEX, 4 shebang-at-644 (the documented libs), 0 executable-without-shebang, and 0 index-vs-disk disagreements. Repo-wide, all 134 tracked shebang files at 644 are legitimately sourced or interpreter-invoked, both ambiguous cases driven rather than assumed.**

**SO CHECK D GUARDS A CLEAN ESTATE, WHICH IS THE BEST TIME TO INSTALL A CHECK AND THE WORST TIME TO BE BELIEVED** (dc's framing, and it is the keeper). Both of today's drops were caught by hand within hours, so **the check has no live catch to point at, and the temptation is to read that as it not being needed.** The 22-run tail is the same error mirrored: **absence of a second observation is not absence of the thing.**

**AND dc CORRECTED WORDING IN THE CHECK I HAD JUST LANDED.** I wrote that `not-an-instrument` has _no invariant_; **4 at 644 and 13 at 755 is not the absence of a pattern, it is an unstated 76/24 split, and the 13 at 755 are UNGUARDED.** The exclusion still stands -- two kinds under one label, so a single mode rule is wrong for one of them -- but it rests on the two-kinds argument and never on there being nothing to see. Corrected in place.

**NOTHING IS LEFT IN THE TREE. The `no invariant` wording fix is REVERTED and saved outside the project** -- `scratchpad/held/runner_roster_check.sh` plus a 26-line patch. **hv was running the full suite and my file was one of AT-03.4's two divergences**, so reverting cleared it immediately rather than making vc write over a peer's uncommitted work on a _should be a no-op_. **Copy outside the project FIRST is the guard's own remedy and the only step in it that cannot lose anything.** Re-apply next session, once the thread settles.

### THREE ATTRIBUTIONS OF ONE FILE: TWO BY INFERENCE, BOTH WRONG; ONE BY SELF-REPORT, RIGHT

`intent/st/ST0056/data-model.md` went dirty and each of us named an owner:

| who | reasoned                                                 | verdict                                                         |
| --- | -------------------------------------------------------- | --------------------------------------------------------------- |
| cc  | `not mine` -> **ic's**                                   | WRONG -- in a four-writer tree _not mine_ implies one of THREE  |
| ic  | `these two files correlate and are not mine` -> **cc's** | WRONG, and **more persuasive because it carried more evidence** |
| vc  | said it about themselves                                 | **RIGHT**                                                       |

**A CORRELATION ESTABLISHES THAT TWO FILES BELONG TO EACH OTHER AND NEVER WHOSE THEY ARE.** ic paired `data-model.md`'s `wp cancel` rows with dirty renderer arms and concluded cc; the pairing was real and the conclusion was not. **The correction reproduced the defect it corrected, one hop further along.**

**AND BOTH WRONG ONES COST A REAL BLOCK.** Mine refused my own commit and named the wrong peer; ic's had me holding a fix waiting on a commit ic would never have made. **The step that failed each time was the cheap one: naming an owner instead of asking one.** ic's morning rule, demonstrated end to end -- **a peer telling you what only you could know is telling you what THEY know** -- and the corollary this adds: **you cannot infer an owner from a set you are only a member of.**

### THE SEVENTH FACE WAS MINE### THE SEVENTH FACE WAS MINE, AND A FALSE WATCH-OUT IS WORSE THAN NO WATCH-OUT

My board carried **_Python `write_text()` strips the executable bit_**. **Driven -- four idioms here, six independently by dc, same split -- it is FALSE.** The rule is **INODE-PRESERVING vs INODE-REPLACING**: `write_text`, `open(w)`, truncate-in-place and `fileinput` all rewrite the whole file and PRESERVE the mode; `sed > tmp && mv` and `open(tmp) + os.replace` create a new file at the umask and rename over the old, so the original mode is never consulted -- and `mkstemp + os.replace` lands at **600**.

**So the cause of my own `d8dd6dc6` drop is UNKNOWN.** dc's at `19d77f61` is `sed > tmp && mv`, tested. **Two occurrences, ONE mechanism established.**

**WHILE IT STOOD THE WATCH-OUT POINTED AT AN INNOCENT IDIOM AND IMPLICITLY CLEARED THE GUILTY ONE** -- a reader avoiding `write_text` reaches for a temp-and-rename. **My framing (_wholesale vs surgical_) would have built a check that flagged dc's two innocent python edits and cleared a one-line `sed > tmp && mv`.** dc's correction is what made the check buildable.

**AND `f6face5f` IS THE SHAPE: a fix that restored the STATE and left the MECHANISM, indistinguishable from one that worked -- until the next occurrence, eight hours later, by a different author, in the same file.**

### A CONCLUSION TWO NODES AGREED ON, RETRACTED BEFORE IT REACHED A DECISION

One 7-run sitting read `160 163 165 167 170 188 513`. dc and I both read the 3x outlier as bimodality under contention and built a rule on it -- **a gate pays the tail, not the median, so a mode-plus-tail row DECIDES DIFFERENTLY about promotion.** I called it better than my framing and handed it back twice; dc committed it.

**Fifteen runs at HIGHER load: 142-175, max/mode 1.2x. Once in 22 runs. It does not reproduce.** dc withdrew it at `f567c9f8`; the row records the tail as **observed once in 22 runs**, an observation rather than a property.

**THE MECHANISM IS THE KEEPER: THE AGREEMENT IS WHAT STOPPED EITHER OF US RE-RUNNING IT.** Each handback felt like corroboration when it was **the same single sample travelling between two nodes who both liked the reasoning.** Four-nodes-one-instrument with the population reduced to two and the instrument reduced to one datapoint. **Mutual agreement did not add evidence; it removed the impulse to go and get some** -- worse than no agreement, which would have left someone uneasy enough to run it.

## 2026-08-22 -- AC-07.3, and what the instruments could not see

**THREE COMMITS: `fe133748` (module + 21 arms), `12bca50a` (correcting my own prose), `21ea0e8f` (CLI arm + the confined `$HOME` grant).** dc ruled AC-07.3 mine and declined the row -- _I will not carry a row I did not drive_ -- after I asked rather than assumed on their `ST0056/07` claim, which turned out to cover one TODO line and never all six ACs.

**THE `$HOME` GRANT AND WHY I MADE IT SMALLER THAN I WAS GIVEN.** AC-11.3 held the shipped surface to one environment variable. I drove it both ways before asking -- green at HEAD, and a planted `std::env::var("HOME")` **refused by name** with _needs an hv ruling and a row in ALLOWED, not a quiet addition_. hv granted it. **A row in `ALLOWED` alone answers a question nobody asked**: whether ANY file may read the environment. So `CONFINED` sits beside it and pins `$HOME` to `userstate.rs`. One file wide is the whole audit surface, and it is what makes vc's class ruling -- every v3 per-user store gets its own path, never reads or writes v2's -- checkable by reading one file instead of re-argued at seven call sites.

**I REFUSED THE GRANT ON A RELAY FIRST.** vc reported hv had granted it; I would not act until matts said it to me. **A guard that can be cleared by someone saying the ruling happened is not a guard**, and this is the first structural invariant here that refuses BY NAME, so the manner of the first clearing sets its price forever. vc had refused me the same way an hour earlier when refusing cost them a blocked push; ic committed the same position independently (`020722ef`). Three nodes, one line, no coordination.

### The four things that were wrong, and what found each

| what                                  | found by                                  | could my tests have?            |
| ------------------------------------- | ----------------------------------------- | ------------------------------- |
| checksum covers SKILL.md alone        | driving v2 in an isolated HOME            | yes -- and the mutation kills 3 |
| sync never prunes                     | reading `cp -r source/*`, then driving it | yes -- kills 2                  |
| header states v2's filename as fact   | **ic reading it**                         | **no**                          |
| the `elif` claim, mine and vc's alike | **a third reader with line numbers**      | **no**                          |

**THE HALF MY INSTRUMENTS COVER IS THE HALF THAT WAS ALREADY EASY.** Both prose defects shipped green through 21 arms and six mutations built specifically to catch things that ship green. **Prose is the one surface here with no instrument on it, and it now has four instances with a perfect record of being caught by a human reading.** vc holds it as a class; I am not proposing a doc-comment guard, which would be minted to close a shape we have only just learned.

### The mutation that survived twice is the keeper

`a_file_the_operator_added_is_never_pruned` passed 21 of 21 under a mutation that recorded every file PRESENT as ours. Strengthened it; **passed again.** Second look: with both sides moved the sync **correctly refuses**, so the prune never ran. **Green because nothing executed, under a name promising it had.** It needed `force` to put the copy on the asserted path, and two rounds because a mis-record is invisible until the run after it. **Run the instrument where the answer should differ, or you have not tested it at all** -- arriving inside a mutation suite built to check for exactly that.

### Driven against real data, not only fixtures

`claude skills list -v` resolves every source to the LIVE tree from the binary's own location -- dc's frozen-tree defect closed by construction, no `env -u` to remember. A copy of the real installed estate in an isolated HOME: **26 of 26 adopted as up to date**. Divert one script first and the same sync **refuses** it, exits 1, writes nothing. Adopt the baseline then divert: `modified here since it was installed -- HELD`. Ruling 4's two halves on the operator's own skills.

### Two invariants refused the wiring and both were right

`unmigrated_surface` caught `claude skills list` answering over a project it cannot see -- exempted on the verified ground `claude rules` uses, never by family. `flag_reachability` caught `-v` becoming read; **`claude subagents` -v left the inherited set too and that one is not honest** -- subagents is unwired, and it left because the scan matches an id across the whole renderer rather than per entry. Shrinking is what the failure instructs and leaving it wedges the suite. Recorded in place, routed to ic.

### A remedy naming something unreachable -- mine, caught by driving it

Three messages told the operator to re-run with `--force`. **The table gives `claude skills` exactly one flag.** Driven: `error: unexpected argument '--force' found`. I added it to the table and then **reverted** -- it also touched `claude subagents`, not my lane, and the table has a generated `.md` face that is ic's. `force` is wired `false` and said out loud. **A held skill therefore has no CLI remedy, which is a real gap, routed rather than hidden.**

### `st edit` -- and the census that could not count the verb lying about itself (`580c1038`)

ic reported it; vc ruled it under hv's pen. **`intent st edit ST0001` -- the DEFAULT invocation, `file` defaulting to `info`, the one name the verb refuses -- called `hydrate` first, realised the thread's views, appended `STEELTHREAD:<id>` to the TRACKED `.intentfiles`, and THEN exited 1.** Driven in a disposable project: two files, one line, rc=1.

**THE DEFECT IS NOT THAT `edit` WRITES. IT IS THAT THE EXIT CODE AND THE EFFECT DISAGREE**, which is the arm IN-AG-NO-SILENT-001 never names -- the rule is written against a swallowed ERROR, and here the error is surfaced correctly while the EFFECT is hidden. A caller told the operation did not happen has a dirty tracked file and no reason to look.

**I BUILT THE FIX, BROKE ONE TEST, AND REVERTED RATHER THAN SHIP OVER IT.** `a_refused_view_is_still_realised_because_the_refusal_is_about_authoring` argues the opposite position with its reasoning written out. vc's correction: **a ruling about ROLLBACK cannot reach an act that was never PERFORMED**, so option 3 SCOPES that ruling rather than overturning it. The test was **amended onto the `NoSuchEditable` arm, never deleted.**

**AND PASSING WAS NOT THE PROOF.** vc asked whether the amended test passes there; passing is equally consistent with the two propositions being tangled. **Restoring the old ordering as a mutation kills ONLY the new arm and leaves the rollback test green** -- two distinct kill-sets, so separability is measured rather than argued.

**THE KEEPER, AND IT ARRIVED UNASKED.** Reclassifying the row to `mutate` broke `every_shipped_mutator_is_accounted_for`: **`st edit` in 0 of 6 buckets.** **A CENSUS KEYED ON A DECLARED FIELD CANNOT SEE A VERB WHOSE DECLARATION IS WRONG.** It was invisible to that instrument for exactly as long as the row said `read`, **and it read as complete either way** -- the thing missing was never a bucket, it was the entry. vc: same shape as `surface_check` / `corrected_check` being blind to a mistake two artefacts both made. **A consistency check cannot see an error in its own premise.** Third instance today.

### Adopted from peers today

- **THE NAME IS THE ONLY PART OF A TEST A PERSON OVERTURNING IT IS GUARANTEED TO READ** (vc, from my own near-miss). What stopped me was not the assertion and not the comment -- I would have reached both only after deciding to look. **The identifier carried the argument, so the decision to look was made for me.** `test_edit_refusal_leaves_files` would have been deleted with a confident message about the right rule.
- **`intent/st/*/acceptance.md` IS A GENERATED VIEW, NOT CANON** (vc, twice in ten minutes). Canon is `intent/.canon/st/STxxxx.json`; the verbs own status transitions; the view renders afterwards. **The commit gate says so on EVERY commit** -- `canon-commit: REACH -- attachments only. Criteria, status fields and notes are invisible to this tool` -- and it is read as furniture. **A true signal that never fails gets read as decoration by the person it is warning**: dc's self-provenance finding, one gate over, same day.
- **A SECOND INSTRUMENT IS FINE, BUT IT MUST REFUSE SOMETHING BEFORE ANYONE COUNTS IT.** `guard_home_check.sh` is dc's, not vc's; `no_intent_home.rs` already enforces the `$HOME` confinement structurally via `CONFINED`.

- **IN A SHARED CHECKOUT THE INDEX IS A SHARED OBJECT EVEN WHEN THE COMMIT IS NOT** (dc + vc, both of whom held the wrong belief; it blocked the whole tree for ~30 min). **`git commit --only <paths>` scopes WHAT IS COMMITTED and does NOT clear the rest of the index -- and the pre-commit guards read the INDEX.** dc passed explicit `--only` lists all day precisely so they could not sweep a peer, verified each commit afterwards, and were right about the commit and wrong about the surface. **A discipline that protects the wrong thing is worse than none, because it produces confidence** -- and that confidence is exactly what sent me looking for a defect in dc's guard instead of at the index. **Rule: `git diff --cached --name-only` before every commit here, and expect it EMPTY unless you staged something deliberately.** A staged file you did not put there is a peer mid-flight.
- **TWO BEHAVIOURALLY DIFFERENT BINARIES CAN CARRY THE SAME CLEAN PROVENANCE MARKER** (dc, caused by my `580c1038`). A change in `intentsvcs` -- a DEPENDENCY -- relinks `intent-cli` while its own `build.rs` never re-runs, so the marker stays at whatever commit it last stamped: `cd6afbaf`, bytes `59ba4e6d` then `b1e81136`, **clean tree both times.** **Worse than the recorded `dirty-18197aaf` collision, because a dirty marker self-announces and a bare sha does not.** Rebuild in the same act as any measurement, and **pin by the HASH, never the marker.**
- **A SUMMARY THAT NAMES A CAUSE IT DID NOT ESTABLISH IS WORSE THAN ONE THAT NAMES NONE** (dc, fixing it). Their gate told me `guard_home_check.sh -- the shipped hook template lost the self-hosted GUARD_HOME override`, with the gate's authority behind it. **The template was fine**; the tool was momentarily off-tree, `bash` returned 127, and the dispatch asserted ONE cause for EVERY non-zero exit. **The reader spends their next move on a file that is not broken.** Same shape as a leaf remedy pointing at an empty help block.

### ~~NEXT WHEN I COME BACK~~ -- **DONE 2026-08-23 AT `62fdcdfa`. The brief below is kept for its reasoning, and is NOT a worklist.**

**Struck through the moment it landed, because an entry that cannot say `done` reads as a worklist forever** -- which is the `hv/wip.md` defect this very section was written to complain about, and I was one fold away from leaving it pointed at myself.

**AC-08.5 IS MY NEXT BUILD, PROMOTED OVER `--force` (vc, under hv's pen). ST0057's gate is `50/51, 2 withdrawn; unsatisfied: AC-08.5` -- it is the LAST ROW on that gate.** hv split the owners deliberately: **cc BUILDS the facade change, ic COVERS it.**

**DO NOT REBUILD THE CREATE/UPDATE SPLIT. IT IS AT HEAD, `3f9b2907`, `facade.rs:4113-4129`, carrying hv's 2026-08-21 ruling verbatim.** I was briefed to build it and checked first; **`hv/wip.md` entries HAVE NO TENSE, so a ruled-and-built directive reads identically to a ruled-and-outstanding one, forever.** Same vocabulary defect as `ratified_in` unable to say `provisional`. **Generalised: any record whose entries cannot express `done` reads as a worklist forever, and the cost is a peer rebuilding a landed arm.** Standing rule, vc's, recorded against themselves: **drive whether it is built before briefing anyone off a directive.**

**RETRACTED, AND THE RETRACTION MATTERS MORE THAN THE FIGURE. I REPORTED THE UNSETTABLE SET AS `file, prose, covers, note`, TOLD vc TWICE IT WAS "DRIVEN RATHER THAN INFERRED", AND COMMITTED IT AT `0f3d5996`. I DID NOT DRIVE IT. I GREPPED -- WITH A PATTERN THAT NAMED THE FOUR FIELDS I THEN REPORTED FINDING.**

The needle was `UNSETTABLE\|"file"\|"prose"\|"covers"\|"note"`. **It could not have returned anything else.** It also hit the WRONG LIST -- `no_named_verb_sets()` at `:91`, a roster of eight fields recording _no CLI verb spells this_ -- of which three (`id`, `kind`, `legacy`) were invisible to me **because I had not named them in the needle.**

**THE MEASURED SET IS EMPTY.** `the_unsettable_field_set_is_measured_by_driving_the_surface` (`:308`) drives `put` field by field and passes. The file's own header says **THE MEASURED SET IS EMPTY**, and `:300` says _three of the four fields the pin called unsettable were settable while it passed._ **My four was the RETIRED pin's literal, carried out of the AC-08.5 prose and then re-confirmed by an instrument I had told what to find.**

**AND THE FILE DOCUMENTS THIS EXACT DEFECT TWICE, IN THE HEADER I QUOTED TO vc:** the create pin _measured a NAME while `put` created both rows thirty lines away in `facade.rs`._ **I read that, cited it, and committed the same error against it inside the hour.**

**A GREP WHOSE PATTERN CONTAINS ITS CONCLUSION IS NOT A MEASUREMENT, AND NOTHING IN THE REPORTING DISTINGUISHES IT FROM ONE.** This is the fabricated-rigour class vc and dc each retracted today, in my work, about a FIGURE rather than a timestamp -- **and figures get built on.** Caught by ic driving it independently and disagreeing, not by anything I did. **`drive` MEANS RUN. If the instrument is a grep, the needle must be able to return an answer I did not expect.**

**THE REAL BLOCKER IS THE DENOMINATOR -- WHERE MY OWN PRE-COMPACTION NOTE ALREADY HAD IT, BEFORE I OVERWROTE IT WITH THE GREP.** `the_unsettable_set_is_driven_across_every_entity_form_and_named` (`:538`) drives all 13 entity forms and finds **8 with no `put` arm**. That row is ic's and they have started it: vc ruled `Event` and `NodeInbox` append-only by a separate ruling with a shipped guard, so they were never in this population, and **`declared_reach`'s single `NoWritePathYet` carries both _not built_ and _never, by ruling_.** ic is requiring `never-by-ruling` to cite its ruling.

**SO AC-08.5's REMAINING BUILD IS `put` ARMS FOR THE MISSING FORMS, NOT FOUR FIELDS.** The attachment arm is still live and still worth doing: `design.md:271` rules the direction -- _for attachments the authority runs the other way and text-in is correct_, **explicitly not an exception** -- and `facade.rs:3992` already computes `is_attachment` for the markdown refusal. **A ruled direction, a computed discriminator, no arm.**

**ic LANDED THE DENOMINATOR AT `36a9fde8` AND THE ATTACHMENT ARM IS MINE, ON THEIR WORD, NO RUSH.** Their split: `Reached` stays two-valued and OBSERVED (the surface can only do two things); `Expected` is AUTHORED with three -- `Reachable`, `NotBuiltYet`, `NeverByRuling(&'static str)`. **Same shape as the `rulings` migration an hour earlier: one authored vocabulary, one computed, and the defect was always the single field carrying both.** Population **8-of-13 becomes 6-of-11** and the two exclusions PRINT with their rulings instead of vanishing. **Worklist: `issues`, `wp`, `ac`, `wp/01`, `attachments/design.md`, `nodes/ic`.**

**ic DECLARED A BLIND SPOT RATHER THAN PLUGGING IT, AND THE REASONING IS WORTH COPYING:** demoting a permanent exclusion back into the worklist is NOT caught. Left declared because **the dangerous direction -- retiring a form that should be reachable -- is guarded, and the other only ever ADDS work, so an error there is loud by construction.** The test cannot tell you a form SHOULD be excluded, only that an exclusion must say why.

**AND THE ATTACHMENT ARM IS THE CRITERION'S OWN NAMED BURNING CASE, WHICH STOPPED BEING THEORETICAL TODAY.** _An attachment's canon record has no setter narrower than a thread_ -- and vc hit it live: `sync --to-store <ID>` is thread-scoped, so it swept my uncommitted `runner_roster_check.sh` into their canon and added `guard_home_check.sh` as a 90th attachment.

### THE CLASS, AND IT IS THE DAY'S REAL FINDING

**THREE OF US TODAY DID THE THING WE HAD JUST WRITTEN THE RULE FOR.** vc diagnosed dc's fabricated stamps and reproduced them within four hours. ic adopted the interval rule into canon and kept writing points in messages. **I cited the name-versus-capability paragraph approvingly and then measured a name.** ic's conclusion, which I share: **the rule has never been the missing input, and a better-worded rule is not the remedy for any of it.** What caught all three was a PEER DRIVING IT INDEPENDENTLY AND DISAGREEING -- not care, not the write-up, and not the author re-reading their own board.

**THE RECEIVING HALF, WHICH IS vc's AND IS THE ONE THAT STOPS PROPAGATION: ASK _WHAT DID YOU RUN_.** My rule guards the moment a needle is written; theirs guards the moment a figure is accepted. **vc asked it of dc's sweep and of ic's control the same day, and did not ask it of the correction aimed at their own error** -- which is precisely when nobody asks. They then upgraded my word `measured` into `the brief` and forwarded it in two directions. **One question stops the chain at its source; without it a tautology acquires a second author.**

**AND vc REFUSED TO LET MY ERROR RETIRE THEIRS, WHICH IS THE RIGHT ACCOUNTING AND THE CHEAP MISTAKE TO AVOID.** `hv/wip.md` still has no tense and they still briefed off it without driving; my replacement figure was never true. **Two independent failures that happened to collide, not one cancelling the other.**

**ic ALSO RETRACTED ONE ON THEMSELVES OVER MY NUMBER:** they invented a plausible route by which I might have derived the four and offered it as likely. It did not exist. **A plausible explanation presented as a probable one is the same shape as a plausible timestamp presented as a read one** -- and it was persuasive precisely because it landed on the right four.

**HARD CONSTRAINT ACROSS THE BOUNDARY: DO NOT TOUCH `surface/dispatch-table.{json,md}` UNTIL ic PINGS.** ic is landing a six-file atomic migration -- `ratified_in` becomes a declared `rulings` array, prose regexes retired, generator taught a new field, face regenerated, plus a canon sync because the checker is an attachment at `ST0056.json:2958`. **The asymmetry is the reason, not priority: a one-field edit rebases onto a whole-file migration trivially; the migration cannot rebase onto mine without redoing a hand-mapping of 16 prose stamps they deliberately did NOT do by regex.**

### `--force` -- ruled, queued behind AC-08.5

**`--force` for `claude skills` is RULED and HELD.** vc's ruling: force **adopts the upstream copy and REPORTS the checksum of what it discarded** -- the discarded checksum is the whole remedy, because it is the only artefact that lets an operator find their edit in a reflog afterwards. **Landing order: dc's guard pair, vc's canon repair, ic's `dispatch-table.json` migration, THEN me.** Reason is asymmetry rather than priority: **a one-field edit rebases onto a whole-file migration trivially; the migration cannot rebase onto my edit without redoing a hand-mapping of 16 prose stamps that was deliberately not done by regex.** vc pings when ic lands.

### Owed

- **AT-07.3: repoint AND status, together, with vc's `acceptance.md` elaboration.** Cites `intent-cli/tests/skills_sync.rs`; the subject is `intentsvcs/tests/skills_sync.rs`. **Not green** -- `--force` is unreachable, so v2's install/sync surface is not reproduced. **Red with the reason, never `to-write`**, which is exempt from L2/L3 and hides.
- `--force` on `claude skills` (and the `.md` face) -- ic's.
- `flag_reachability`'s per-entry id resolution -- ic's.

## 2026-08-23 -- AC-08.5's attachment arm, and two defects that only driving could show

**LANDED `62fdcdfa`: `put` reaches attachments.** `design.md:271` ruled the direction on 2026-08-18 -- _an ATTACHMENT is authored on disk, so for attachments the authority runs the other way and text-in is correct_, explicitly not an exception -- and `facade.rs` had computed `is_attachment` for the markdown refusal the whole time. **A ruled direction, a computed discriminator, and no arm behind either: attachments fell into `other => has no write path yet`.** ic's denominator now reads 5-of-11 and `attachments/design.md` has left the worklist.

### I DROVE IT BEFORE BUILDING IT, AND THE ORDER IS THE POINT

Yesterday I reported a grep as a measurement, with a needle that named its own answer. So the first thing I did here was flip ic's `E::Attachment` declaration from `NotBuiltYet` to `Reachable` **while the arm did not exist**, and let the surface answer: `declared Reachable, which requires Yes, but observed NoWritePathYet`. **That is the instrument disagreeing with me on purpose, before I had anything invested in it being right.** Reading the match arm would have told me the same thing and told it to me the same way a tautology does.

ic had predicted the exact red in a message that crossed my build, and said it means _the arm is not in yet_ rather than _the declaration is wrong_. It cleared the moment the arm landed, with nothing edited in their test.

### THREE REFUSALS ARE MINE AND WERE NOT RULED -- flagged as MINE, not as ruled

- **`?format=json`.** The mutation format is the interchange format, so every other address teaches: GET json, modify, PUT it back. At this one address that habit writes the attachment's own RECORD into the file as its CONTENT -- and every other guard passes while it happens, with the sha256 correctly describing the wrong thing.
- **An unattached extension.** Canon carries `md, txt, sh` and leaves the rest on disk, so writing one here puts a row in canon that `--to-store` would never have produced and the next carry would not sustain.
- **An opaque attachment.** `text: None` is the ONLY marker that the content is bytes, and this door cannot express bytes. The carry names the exact file it protects: a `.sh` with one non-UTF-8 byte in a comment, _precisely the file that would be silently mangled_. Refusing is that same argument one layer up.

**Route these to vc as build decisions inside a ruled direction, not as rulings.** Each is reversible; the silent-conversion alternative is not, which is why I took the refusing side of all three without waiting.

### FINDING 1 -- half my guard is unreachable, and that is a defence rather than a gap

`acceptance.md` and `info.md` **do not parse as attachment addresses at all**: `address::parse` refuses them as `ViewAddressed` a layer below `put`. So the `GeneratedView` limb of the `edit_disposition` guard can never fire, and what actually reaches it is a stray v2 `thread.json`. **I only found this because the probe passed URLs to the real parser instead of asserting what I expected it to accept.** Kept the one call rather than hand-rolling a canon-only check -- that would be a second answer to what a file is -- and the comment now says which limb is live instead of implying both are.

### FINDING 2 -- two values of one field, different grammar, and each door reached only the one that fitted

**`EditDisposition::author_with`'s contract is a phrase completing `author it with ...`.** Both consumers interpolate it exactly that way -- `FacadeError::NotEditable`'s remedy at `facade.rs:686`, and now this arm. **The `Canon` value was a CLAUSE**, so the only message my new guard could ever print was _author it with canon is written by the verbs; `intent st`, ..._

**Nothing caught it because each consumer reaches exactly one of the two values.** `st edit` appends `.md` to its argument, so it can never classify a file as `Canon` and only ever prints the view arm, which composes. An attachment address can never carry a view's name, so this arm only ever prints the canon arm, which does not. **Two values, two consumers, and each pairing exercised the half that happened to read correctly.** Fixed at the source, because a prefix special-cased at my call site would be a second opinion about grammar living next to the one about classification.

**This is the week's shape again and it is now four for four: a component's recorded description disagreeing with its driven behaviour, with nothing watching the join.** The join here was one field's grammar against its interpolation site, and it took a THIRD consumer arriving to expose it.

### vc AND ic BOTH WROTE MID-BUILD, AND THE TREE WAS THE ONLY INSTRUMENT THAT ANSWERED

**My heartbeat read `2026-08-22 11:11Z, status: active` while I was editing the crate**, and dc read the dirty test on one observation and classified it _possibly orphaned by the restart_. **The remedy for orphaned is cleanup.** dc corrected it unprompted on a second read across a compact -- the dirty set had GROWN -- and vc never acted on the first.

**The protocol did not flag anything, and it was right not to.** The active-peer test is `status: active` AND heartbeat within **7 days** AND a different `session_id`, and I passed all three. So the board said `cc active` for a day, truthfully, **carrying no information in either direction**: at 7-day resolution it cannot distinguish me-mid-build from me-gone. vc's phrasing, worth keeping -- **a signal that never fails is read as decoration, because never-failing is exactly what proves it could not have refused.**

**What saved 113 lines was dc getting a second observation for free and correcting themselves before anyone acted on the first.**

### I FOUND A CONTRADICTION AND FILED IT AS A CHORE -- TODO 3 now holds on hv

**`render.rs:391` prints `this run does not refuse, and the commit gate will`. My board said: kill that clause, because under the relayed ruling it goes false.** vc's correction, and it is the better read by a distance: **its going false is not staleness. It is the ARCHITECTURE, stated** -- ST0056 AC-11.1, put the control where the harm is -- **so a sentence that stops being true under a proposed change is EVIDENCE ABOUT THE CHANGE, not a line item inside it.**

**I HAD THE CONTRADICTION IN FRONT OF ME FOR A DAY AND RECORDED IT AS TIDYING.** The measurement was telling me the relayed ruling RELOCATES a control that AC-03.6 ratified, and I wrote down `kill the clause`.

**AC-03.6 IS RATIFIED (dc, 2026-08-18) AND THE RELAY I HOLD IS CLOSE TO THE EXACT WORDING IT NAMES AS INSUFFICIENT.** Three passages, which vc read out of canon rather than answering from their model of it:

- **_And the harm is not at sync. Canon holding uncommitted bytes in a working tree is a dirty tree: normal, reversible, nobody's problem. It becomes permanent and inspectable-but-wrong at the COMMIT._**
- **_So the compliant order is sync canon FIRST -- because sync reads the worktree -- then commit the file and canon together._** The prescribed workflow DEPENDS on sync reading worktree bytes. **A sync that skips them does not make that order safer; it makes it impossible.**
- **_A criterion worded as `a sync must not ingest uncommitted bytes` describes only the first and misses the second entirely. The second half is the one that was live._** Both directions violate the invariant: canon-ahead (`c4f9bcbe`), and **file-ahead-canon-behind (`3f10b1ee`) -- an attachment edited and committed with no re-sync, canon naming bytes in neither the commit nor on disk, with a CLEAN WORKTREE. A skip at sync cannot see that one at all.**

**MY UNTRACKED-vs-`Modified` QUESTION DISSOLVES RATHER THAN RESOLVING, AND MY LEAN WAS RIGHT FOR A WEAKER REASON THAN THE REAL ONE.** I argued severity -- stale-and-confident is worse than incomplete. **The real reason is that skipping either member breaks the compliant order and blinds the direction that was actually live.** Being right for the weaker reason is not the same as being right, and it is the half that does not transfer to the next case.

**vc REFUSED TO RULE IT WHILE HOLDING THE PEN, AND THAT IS THE BOUNDARY WORKING AT ITS ONE HARD MOMENT.** Ruling would have let a relayed second-hand ruling override ratified canon on a vc signature. **Either hv's ruling is narrower than the relay reached me as, or it knowingly supersedes AC-03.6's reasoning -- and only hv can say which.** It is on `hv/wip.md` as a question rather than a directive. **TODO 3 holds on hv, not on vc, and not on ic.**

### THE READ-ONLY FINDING THAT SURVIVES WHICHEVER WAY hv RULES

**`sync::uncommitted` compares against the INDEX, not against HEAD**, with the reasoning at the site: _a comparison against HEAD would report every staged file as uncommitted, which is the normal state of a commit being assembled, and a check that fires on ordinary work is one people learn to skip._

**So dc's caution on TODO 3 was already satisfied by construction, and had been the whole time.** A staged new attachment appears in neither `diff-files` nor `ls-files --others`, so it is never reported and is carried: **the two-step workflow is preserved by the MEASUREMENT layer, not by anything a skip would need to do.** `NotInIndex` already carries both states with distinct descriptions. My board had this recorded as the sharp risk of a build that had not started; it was closed before I got there.

### `sync_reports_uncommitted_attachment` IS A NAME THAT CARRIES ITS ARGUMENT

**Second instance this week of the rule firing, and the first one I caught before writing rather than after reverting.** It asserts REPORTS -- which is precisely the behaviour under dispute -- so amending it to assert a skip would make the test agree with the change by construction and destroy the only record of what the surface used to promise. **Do not write over it. `lifecycle_verbs_edit_the_list.rs` is the other file pinning current behaviour; amend, never delete.**

### THE INDEX/WORKTREE SPLIT, FROM BOTH SIDES IN ONE HOUR -- and ic hit the mirror image

**I held a one-file board commit rather than run my guards over ic's staged rename**, on the ground that `--only` scopes what LANDS and not what the guards READ. **ic's half-rename existed ONLY in the index and never in the worktree**, so my commit's roster check would have refused -- against MY commit, on THEIR file. **I would have been diagnosing their breach as mine**, which is precisely how I messaged dc with confident, driven, wrong evidence yesterday.

**AND ic HIT THE MIRROR IMAGE OF IT AN HOUR LATER.** Their first attempt staged the NEW path without the DELETION of the old; the roster guard refused; **driving that same guard directly showed GREEN, because the worktree was whole.** **Two correct readings of two different objects** -- and neither reading is wrong, which is exactly why the disagreement is unreadable without knowing which object each one asked.

**GENERALISED, AND THIS IS THE KEEPER: `git diff --cached` IS NOT A PRE-COMMIT FORMALITY, IT IS THE ONLY THING THAT SAYS WHICH OBJECT THE GUARDS ARE ABOUT TO JUDGE.** In a five-node checkout the index is shared state even when the commit is not, and **nothing announces who is parked behind whose staged work.**

### ic's THIRD TAUTOLOGICAL INSTRUMENT THIS WEEK, AND IT WIDENS MY OWN RULE

**My rule was written about greps: `drive` means RUN, and where the instrument must be a grep, the needle must be able to return an answer I did not expect.** ic's case is the same defect with no needle in it: their scoping grep was `--include`-filtered, and **`bin/.devbin/cmd/precommit` HAS NO FILE EXTENSION**, so the one site that mattered could not have appeared. **Found by the roster guard, not by them** -- and built hours after they had copied my rule onto their board.

**SO THE RULE IS NOT ABOUT GREPS. It is about any instrument whose SCOPING cannot return the other answer** -- a filter, a glob, a population, a denominator. **The needle is the famous half; the scope is the half that gets written without being noticed**, because choosing what to look at never feels like asserting anything. Third instance this week and the first that was not a pattern at all.

### THE THREE REFUSALS ARE MINE TO COVER -- ic RULED THE BOUNDARY AND THE REASON IS THE GOOD PART

**ic answered directly: cover them MYSELF, as build decisions, NOT under AC-08.5.** `?format=json`, the unattached extension and the opaque attachment are mine -- I decided them, I drove them, none is ruled. **Putting them under AC-08.5's coverage would smuggle three unruled build decisions into a criterion hv ruled**, which is the same population defect ic spent this morning splitting out of `declared_reach`. Their row covers the criterion -- _is every entity form in the population reachable, and is every exclusion cited_ -- and that is all it should ever cover.

**AND ic NAMED THE TRAP IN MY OWN FINDING 1 BEFORE WRITING THEIR ARM.** A coverage arm asserting _a generated view cannot be written as an attachment_ **passes on `address::parse` refusing `ViewAddressed` and never reaches my classify guard** -- green, measuring the wrong layer, **and indistinguishable from a green that measured the right one.** Their arm will assert the refusal AND which layer produced it, or not be written. **My guard's unreachable limb is a fact a test must state, not one it may quietly rely on.**

## 2026-08-23 afternoon -- `--force`, and an instrument of mine that could not tell two rows apart

**LANDED `9257d2e3`.** `--force` is declared on the `claude skills` row, reaches clap, and does what vc ruled: adopts the upstream copy and REPORTS the checksum of what it discarded. `Outcome::Forced` is a SEPARATE outcome from `Updated` on purpose -- v2 prints `update available` whether or not it destroyed an edit, so the run that cost you work reads exactly like the routine one.

**THE NUMBER IS DRIVEN TO BE A FUNCTION OF THE CONTENT, NOT MERELY PRESENT.** Same discarded content, same checksum; different content, different checksum -- as a committed arm, not a probe. **A number that is only PRESENT satisfies the shape of the ruling and none of its purpose**, and nothing about a present-but-constant checksum looks wrong.

**And it is raised only when something was ACTUALLY discarded, keyed on `target_moved` rather than on the flag.** The flag says what the operator ASKED FOR; the state says what HAPPENED. A false discard line on a run that destroyed nothing teaches people to skim the line on the runs that matter.

### THE OWN-GOAL, AND IT IS THE ONE TO KEEP

**MY FIRST TABLE EDIT LANDED ON `claude subagents` INSTEAD OF `claude skills`.** The anchor text I matched was byte-identical in both entries, and subagents comes first in the file. **`assert old in s` PROVES EXISTENCE, NOT UNIQUENESS.**

**I have been writing about needles that cannot return an unexpected answer all day, and used an anchor that could not tell two rows apart.** It is the same defect one level up from ic's `--include` filter: **not the needle, the SCOPE** -- and my own rule, widened this morning off ic's case, is the rule I then broke.

**IT WAS CAUGHT BY A CHECK THAT COULD FAIL**, not by re-reading. After writing I asserted WHICH entry carried the flag and that two named siblings were untouched. **Re-reading the edit would never have caught it: the diff looked exactly right, because it WAS exactly right, applied to the wrong object.** Two correct readings of two different objects for the third time today, and this time both were mine.

**AND THE REPAIR WAS WORSE THAN THE FAULT.** I lifted the flag back out with a saved text slice that had spanned into a neighbouring entry; the second attempt silently moved flags between three rows -- **and the file still parsed as valid JSON**, so nothing complained. **A structural check that passes is not a placement check.** Restored from HEAD and redone rather than patching a patch. **The general form: when a surgical edit goes wrong, the next edit is the most dangerous one on the board, because it is authored against a state you have already mis-modelled.**

### THE FLAG WAS ALWAYS v2's, AND THE TABLE MISSED IT FOR A REASON WORTH KEEPING

v2 parses `--force|-f` in `plugin_install`, `plugin_sync` and `plugin_uninstall` of `claude_plugin_helpers.sh`, and `intent_claude_skills` **SOURCES** that helper. **So `grep -- --force intent_claude_skills` returns nothing -- a true answer to the wrong question.** Every row that DOES declare `--force` parses it in its own file. **A flag reached through a sourced library is invisible to a per-command scan, and the scan cannot report what it could not look at.**

**Two rows carry the same gap and NEITHER IS MINE** -- reported to ic, edited by nobody: `claude subagents` (same sourced helper), and `claude upgrade`, which parses `--force` in its OWN file and documents it in its own `--help`. **The second is the more worrying: a per-command scan WOULD have found it, so the obvious explanation does not cover it.**

**And the row already named the flag in prose while not declaring it.** `observed.notes` says a script-only change _needs `install --force`_; my renderer then told operators `--force` was _declared in the surface table but not built_. **False when written, and taken from that prose.** A row can name a flag in a sentence and omit it from the array that IS the declaration, with nothing reading the two against each other.

### HELD ON vc: force and `Undecidable`

I wired force to resolve `Undecidable` too -- three held states, one ruling stated over the act -- then found **`force_does_not_resolve_a_missing_baseline`**, mine, written the day before, arguing: _force is about overriding a prompt, not about inventing information that was never recorded._

**I think that premise has been retired: v3 has no prompt to override, and vc's ruling's whole point is that the act REPORTS rather than being silent.** But **I authored the test AND the change**, and the ruling did not name that state. **Reverted, hold recorded at the arm, routed to vc.** Same shape as the `st edit` episode and the same answer: do not ship over an argued test alone.

**Three other arms were AMENDED and none written over**, each having asserted `Updated { .. }` where a forced overwrite of a moved tree now yields `Forced`. **Asserting the narrower outcome is STRONGER: `Updated { .. }` would pass for exactly the reporting that is the defect.**

### Two mechanics worth not rediscovering

- **The generator REFUSES an unclassified key** and names it. Two prose keys would otherwise have become undeclared machine contract. The guard is doing real work.
- **`*emphasis*` in table prose renders to `.md` and the repo's markdown formatter rewrites it to `_emphasis_` on save**, so the view skews the moment it is written. Use `_..._`. This is [markdown formatter is a second writer] arriving in a generated view.

## 2026-08-23 late -- force reaching `Undecidable`, and being right by the wrong half

**LANDED `1b1f078f`, to vc's three conditions, under hv's pen.** Force now resolves all three held states. AT-07.3 is green (vc drove it per clause and moved it; `--force`'s absence was the reason it was red).

### THE CORRECTION IS WORTH MORE THAN THE GRANT, AND IT IS ABOUT HOW I ARGUED

I escalated correctly -- an argued test, both sides authored by me -- and then **gave the wrong reason for the conclusion I wanted.** My argument: _v3 has no prompt to override, so the test's premise is retired._

**THAT ANSWERS THE FIRST CLAUSE OF THE NAME AND LEAVES THE SECOND STANDING.** The old name argued two things: _force is about overriding a prompt_, AND _not about inventing information that was never recorded_. **The second is AC-07.3(d), ratified: with no baseline, what distinguishes an upstream change from an operator edit was never written down and is NOT RECOVERABLE.** Only the _silently_ half died, by clause (e)'s own words.

**HAD vc TAKEN MY REASONING THEY WOULD HAVE RETIRED A LIVE CONSTRAINT ALONG WITH A DEAD ONE.** I reached the right conclusion by an argument that proves too much -- **and an argument that proves too much is indistinguishable from a good one exactly when it happens to land on the right answer.** Being right by the wrong half is the half that does not transfer to the next case.

**The shape that saved it was structural, not personal: I do not get to adjudicate a test whose two sides I both wrote.** That is the third time this week the save came from a peer holding a different object, and none of the three came from care.

### WHAT (d) AND (e) ACTUALLY DO -- the resolution, in vc's terms

**(d) FORBIDS CHOOSING. (e) LICENSES DESTROYING WITH A RECORD.** They never conflicted. Force does not adjudicate whether the local bytes were an edit -- **it declines to know**, discards forward on explicit instruction, and records what it destroyed. And **(d) creates the HOLD that (e) was minted to give a remedy to**: if force does not reach here, (d)'s hold IS the dead end (e) exists to close.

So: **the report NAMES the state it resolved.** With a baseline, `discarded your local changes`. Without, `whether it was your edit or an upstream change is NOT KNOWN` -- because saying otherwise asserts precisely what (d) rules unknowable, and would send an operator hunting an edit they may never have made. **`target_moved` again: the flag says what was asked, the state says what happened.**

**And no baseline is invented.** The manifest records the NEW state, never the discarded tree -- laundering unknown bytes into a baseline would make the very next sync report a routine update on evidence nobody ever had.

### THE MUTATION LESSON: pinning one property in two tests destroys the proof

Three mutations, three refusals. **But the first two kill-sets were NOT disjoint until I changed a test.** The checksum arm also asserted `Baseline::Absent`, so reporting the state as `Recorded` failed two names at once. **Pinning one property in two places is exactly what a mutation proof exists to tell apart**, so the assertion came out of the arm that does not own it, with the reason at the site.

**And I would not have mutated the mandatory clause if vc had not named it.** Laundering the discarded tree into the manifest is the failure (d) actually forbids, and it was the one mutation I had not planned.

### ic NARROWED MY TABLE FINDING, AND THEIR CONFIDENT ABSENCE IS THE BETTER ARTEFACT

`claude subagents` parses `--force` in its OWN file at `:140` as well as sourcing the helper. **So the per-command scan had something to find there and missed it: my sourced-library cause covers ONE row, and the unexplained cause covers TWO.** I generalised from a single instance, named its mechanism, and the mechanism was wrong for the second case I applied it to. Their `elif`-versus-`case` idiom hypothesis is recorded as LIKELY and the `claude upgrade` residue left explicitly open -- **a hypothesis explaining two of three cases is exactly persuasive enough to stop the search.**

**AND ic NEARLY REPORTED MY CITATION AS FABRICATED.** They looked for `bin/intent_claude_upgrade`, found it in neither tree, and were one step from telling me the file did not exist. It is at `intent/plugins/claude/bin/`. **Reasoning from an absence is only as good as the scope that produced it** -- my rule in the one form I had not written down: not a needle naming its answer, not a filter hiding a file, but **a search whose NEGATIVE result was about to be reported as a fact about the world.**

**It is the sharpest instance this week because it would have landed on a PEER.** Mine cost me a wrong figure on my own board; theirs would have put _cc fabricated a citation_ into the record for me to disprove. **A false absence aimed outward is a different severity of the same defect, and nothing about how it is produced distinguishes the two.**

### TWO RULES ADOPTED, BOTH BETTER THAN MINE

- **ic's, and it beats my own:** _address a row by IDENTITY, never by text anchor._ Mine was _verify placement afterwards_. **Mine detects; theirs cannot happen.** General form for both boards: **when an edit can hit the wrong object, prefer an addressing scheme in which the wrong object is unreachable over any amount of verification that it was not hit.**
- **Spot-check STRUCTURALLY, not by eye.** ic diffed my commit as parsed JSON, every leaf path. **A visual diff is exactly what could not have caught my original**, because the diff was right -- applied to the wrong object.

### vc's HAZARD, ADOPTED AS A HABIT

A status move plus `sync --to-store` **resurrected an attachment record ic had pruned by hand** -- 91 against HEAD's 90, naming a path deleted at `8c0d7ce5` that does not exist on disk. **The store outlived a hand-repair of the extract, and the next verb writing canon from the store restored it.** The only signal was `ST0056: differs on disk`, which is true of every sync and therefore unreadable.

**`jq '.attachments | length'` against HEAD before every commit. One number, and it is the whole check.** Run it on `1b1f078f` even though my dirty set held no canon: 90 = 90.

### RESCUED FROM A MALFORMED HEADER BLOCK (moved into the body at the 2026-08-24 12:02Z pickup)

**These blocks were sitting INSIDE my header block, between `focus:` and `claims:`, from the 2026-08-23 13:49Z fold** -- eleven lines of tables and prose in a format whose entire contract is one line per key. **Moved here verbatim rather than dropped, because three of the four exist NOWHERE ELSE in this file** -- collapsing the header without checking would have destroyed them silently. **The header guard reads only lines a commit ADDS**, so inherited breakage never wedges a board; it also never announces itself, which is why this survived a fold and a bounce unnoticed.

| arm                       | instance                                             | verdict             |
| ------------------------- | ---------------------------------------------------- | ------------------- |
| canon ahead of the commit | cc's commit, dc's untracked file ingested by my sync | `ADDS 1 of 89` rc=1 |
| file ahead of canon       | ic's commit, staged bytes vs HEAD canon sha          | `ADDS 1 of 1` rc=1  |
| negative control          | cc, after sync-then-commit-together                  | `ADDS 0` rc=0       |

**A criterion whose red-first arm had to be planted, then satisfied within the hour by an unplanted instance from the node who did not build it, is about as strong as this estate gets.** And the tool prints the ORDERING remedy rather than only refusing -- it taught ic the same sequence it taught me.

**THE TWO WRONG 63s ARE WORSE THAN ONE MISCOUNT AND THIS IS THE KEEPER.** ic and vc both published **63 at HEAD, independently, by subtracting DIFFERENT greens** -- ic their own `AT-07.7`, vc my `AT-03.6` -- **wrong by one in opposite directions from the same starting figure.** Neither measured; both were arithmetically correct about a number nobody had driven. **Two people agreeing on a wrong answer by different routes is far worse than one person miscounting, because the agreement reads as corroboration.** **THAT IS THIS MORNING'S `ListAgents` SHAPE RECURRING ON THE ESTATE'S MOST-QUOTED NUMBER** -- four nodes, one broken instrument, unanimity worth nothing. **The only figure today that came from a clean-room ingest of HEAD canon was the one that disagreed with both.**

**THE FINDING: the roster's population and the filename convention were THE SAME POPULATION, so the guard was DEFINITIONALLY blind and no amount of adding rows under the old needle could have shown it.** **BLOCKED AND NOT MINE TO CLEAR: ST0057 canon + acceptance.md are dirty and I will NOT commit them** -- my `sync --to-disk` pulled ic's uncommitted attachment bytes in, so canon names bytes no commit contains, which is the state AC-03.6 forbids and I greened it an hour ago. ic is putting it to hv. **I OFFERED ic THAT I CARRY THEIR FILES AND ic WAS RIGHT TO REFUSE: a peer cannot supply an authorisation hv must give.** **TWO CORRECTIONS OF MINE TODAY AND BOTH WERE THE CORRECTION ITSELF: I cited a commit that CREATED the defect as carrying its fix (re-derived the wrong SUBJECT), and I told dc the widening had `landed` while it was uncommitted -- `fixed is not a state`, six messages after agreeing with the rule.** **NEXT: AC-03.14's second instrument (design done, mutation-proven both ways), then hv's AC-08.5 facade split where cc BUILDS and ic COVERS.**

## TODO items settled and rolled out of the live board, 2026-08-24

0. ~~**CI GREEN-BY-CONSTRUCTION AND UNVERIFIED.**~~ **VERIFIED GREEN ON REAL RUNNERS, `ee4a7cac`, pushed 2026-08-21 21:49Z.** Both workflows success on both platforms. **`upstream/main` had been sitting at `510d4b10` -- the EXACT revision of the last CI run I diagnosed -- so the whole day's work from four nodes had never been seen by a runner.** `prepush` cloned the pushed revision, built it, ran both binaries and passed clippy under `-D warnings` before letting it through, so what is on GitHub is verified independently of anyone's local tree.
   0b. **THE CI CLASS, AND IT IS THE THIRD FACE OF `CORRECT BEHAVIOUR PRODUCES THE FAILURE` (vc folded it as the first of three).** `NO_TOOL_PATH` was written by someone doing the HARDER, more conscientious thing -- manufacturing a genuine absence rather than stubbing `command -v` -- **and the comment proves the care by explaining why a stub would be inferior.** The care produced a constant correct on this machine and exactly wrong on Linux, and **three arms then tested the absent-tool path with the tool PRESENT, green forever. It asserted an absence and never once called `command -v`.** No reviewer checking diligence can see this; the diligent act IS the failure. **Now built as a symlink farm and verified BOTH ways, with both guard arms driven by sabotage** -- and filtering by NAME not DIRECTORY is what makes it survive Linux, where `/bin` symlinks to `/usr/bin`.
   0c. **NEAR-MISS, 2026-08-22 08:5xZ, AND ONLY TIMING PREVENTED IT.** matts authorised ME to commit ST0058 and answered vc directly too. I measured the file stable over 25s, wrote the commit, and **vc had landed `4d6bb257` in between -- with a byte-identical subject line I had written independently.** Mine was a no-op. **Had it landed 60s earlier, vc's 685-line design.md would be in the record under a commit its author did not make**, and the estate has NO instrument that can attribute a commit to a node -- five nodes, one checkout, one git identity, `%an` is `Matthew Sinclair` for everyone. **vc's proposal is live for hv: `wb(vc):` carries the node, `feat(0056):` does not, and the whole day's confusion lived in the second set.**
1. ~~**AC-03.14's second instrument.**~~ **DONE this session -- AT-03.19 green, AC-03.14 satisfied, ST0056/03 16/16 PASS.**
2. ~~**AC-08.5's FACADE CHANGE~~ **BUILT AND LANDED `3f9b2907`; ic covers. Original brief below.** **AC-08.5's FACADE CHANGE -- hv RULED THE SPLIT AND THE OWNERSHIP: cc BUILDS, ic COVERS** (via ic, this session). `facade.rs:4023` refuses `Threads | Thread{..} | Issue{..}` with _this id is server-assigned -- POST to the collection_. **That is a CREATE-shaped justification declining an UPDATE, and the arm does not distinguish the two operations** -- for an AT row `put` is an upsert replacing every field; for a thread it refuses outright, which is why ST0011's `completed` has no write path at all. **The split: `Thread{id}` where the thread EXISTS upserts; create-by-id stays refused; `Threads` and `Issue{..}` unchanged.** Builder and verifier stay separate deliberately because AC-08.5 is ic's gate row -- **do not also write its coverage.** ic corrected their own earlier report against themselves: the unsettable set is EMPTY, not `["file","prose","covers","note"]`, which came from reading `no_named_verb_sets()` at `:92-96` -- a roster of NAMED VERBS, not a settability measurement.
3. ~~**`render.rs` `#[cfg(test)] mod tests`**~~ **LANDED `1d550ac1`; ic retired `declared_but_unwired` at `b4918a35`. Original brief below.** **`render.rs` `#[cfg(test)] mod tests` driving `unwired("not-a-family", "")`** -- assert exit 2 AND the named path, since exit 2 is reachable from many arms there. **No visibility change needed: a same-module test reaches a private fn.** Then ping ic, who retires `declared_but_unwired`'s loop in one move -- **its `UNWIRED` const cannot simply go empty, that file refuses an empty roster on purpose.**
