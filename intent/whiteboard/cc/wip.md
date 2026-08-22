---
node: cc
name: Control Claude
role: control
session_id: 87048274-c4dc-44b7-b08d-c933207a4f50
heartbeat_at: 2026-08-22 10:41Z
status: active
focus: "**AC-07.3 IS BUILT, WIRED AND DRIVEN. `intent claude skills` answers -- it had returned `2` since the rewrite began.** `fe133748` module, `12bca50a` a correction of my own commit, `21ea0e8f` the CLI arm. **hv GRANTED `$HOME` DIRECTLY (AC-11.3's second env var) AND I MADE THE GRANT NARROWER THAN THE ROW: `CONFINED` beside `ALLOWED`, `$HOME` legal in exactly ONE file (`userstate.rs`), a second reader failing like an unapproved variable.** The ask was may per-user state be reached, not may any file read the environment. **TWO v2 DEFECTS DRIVEN BEFORE A LINE WAS WRITTEN:** `sync --force` misses a scripts-only change and exits 0 saying `up to date` (checksum covers SKILL.md alone; positive control -- touch SKILL.md and the same change propagates); and sync NEVER PRUNES, so a retired script stays live in every consumer forever while sync reports success. **SIX MUTATIONS, SIX REFUSALS, DISTINCT KILL-SETS -- AND ONE SURVIVED TWICE.** The prune-boundary test passed 21 of 21, passed again after strengthening, and was green **because the sync correctly REFUSED before the prune ever ran**: a test asserting a boundary it had never executed, under the name `a_file_the_operator_added_is_never_pruned`, inside the change built to catch that class. **I WAS ALSO WRONG IN PROSE TWICE AND NEITHER WAS FINDABLE BY ANY TEST I WROTE:** a module header stating v2's manifest filename as fact 77 lines above the constant contradicting it (ic caught it), and a structural claim about v2's `elif` that vc had independently in the same wrong shape (`12bca50a` corrects both). **NEXT: hold for vc.** Full workspace 1024 passed / 0 failed / 144 binaries."
| arm | instance | verdict |
| --- | --- | --- |
| canon ahead of the commit | cc's commit, dc's untracked file ingested by my sync | `ADDS 1 of 89` rc=1 |
| file ahead of canon | ic's commit, staged bytes vs HEAD canon sha | `ADDS 1 of 1` rc=1 |
| negative control | cc, after sync-then-commit-together | `ADDS 0` rc=0 |

**A criterion whose red-first arm had to be planted, then satisfied within the hour by an unplanted instance from the node who did not build it, is about as strong as this estate gets.** And the tool prints the ORDERING remedy rather than only refusing -- it taught ic the same sequence it taught me.

**THE TWO WRONG 63s ARE WORSE THAN ONE MISCOUNT AND THIS IS THE KEEPER.** ic and vc both published **63 at HEAD, independently, by subtracting DIFFERENT greens** -- ic their own `AT-07.7`, vc my `AT-03.6` -- **wrong by one in opposite directions from the same starting figure.** Neither measured; both were arithmetically correct about a number nobody had driven. **Two people agreeing on a wrong answer by different routes is far worse than one person miscounting, because the agreement reads as corroboration.** **THAT IS THIS MORNING'S `ListAgents` SHAPE RECURRING ON THE ESTATE'S MOST-QUOTED NUMBER** -- four nodes, one broken instrument, unanimity worth nothing. **The only figure today that came from a clean-room ingest of HEAD canon was the one that disagreed with both.**

**THE FINDING: the roster's population and the filename convention were THE SAME POPULATION, so the guard was DEFINITIONALLY blind and no amount of adding rows under the old needle could have shown it.** **BLOCKED AND NOT MINE TO CLEAR: ST0057 canon + acceptance.md are dirty and I will NOT commit them** -- my `sync --to-disk` pulled ic's uncommitted attachment bytes in, so canon names bytes no commit contains, which is the state AC-03.6 forbids and I greened it an hour ago. ic is putting it to hv. **I OFFERED ic THAT I CARRY THEIR FILES AND ic WAS RIGHT TO REFUSE: a peer cannot supply an authorisation hv must give.** **TWO CORRECTIONS OF MINE TODAY AND BOTH WERE THE CORRECTION ITSELF: I cited a commit that CREATED the defect as carrying its fix (re-derived the wrong SUBJECT), and I told dc the widening had `landed` while it was uncommitted -- `fixed is not a state`, six messages after agreeing with the rule.** **NEXT: AC-03.14's second instrument (design done, mutation-proven both ways), then hv's AC-08.5 facade split where cc BUILDS and ic COVERS.**"
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

**2026-08-20 is in `.history/20260820/`. Today's pre-fold board and the six vc inbox entries are in `.history/20260821/`. This file is the cold-session minimum.**

## The model -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** **`intentdb` IS RETIRED -- IT NAMES NO COMPONENT** (hv, 2026-08-21, corpus-wide at `513642e7`). `intent-cli` and `intentd` are BOTH clients of `intentsvcs`, which solely owns the SQLite db; the word implied a daemon-owned store. Diagram at `design.md:12-17`. **The SUBSTANCE of D01 is unchanged -- only the term was wrong**, and it was adopted from hv's own phrasing inside two quoted 2026-08-15 rulings, which is why nobody challenged it for six days. **D34: the committed extract is the interchange -- it TRAVELS, the db never does.** D29: a gitignored path is never canon.

**D42 -- TIME.** `date -u +'%Y-%m-%d %H:%MZ'`, own step, trailing `Z`. **The mechanism, since care is not one: the stamp enters the file from the same shell command that reads it.** **THE GENERATOR IS ARITHMETIC, NOT MEMORY** -- a drifted stamp starts from a TRUE reading and is advanced by feel, so it wears the authority of the real one, **and increments-by-feel are monotonic BY CONSTRUCTION, which is why they defeat check C.** A second clock read is the only thing that catches it: **the read is per-stamp, never per-session.**

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

### Owed

- **AT-07.3: repoint AND status, together, with vc's `acceptance.md` elaboration.** Cites `intent-cli/tests/skills_sync.rs`; the subject is `intentsvcs/tests/skills_sync.rs`. **Not green** -- `--force` is unreachable, so v2's install/sync surface is not reproduced. **Red with the reason, never `to-write`**, which is exempt from L2/L3 and hides.
- `--force` on `claude skills` (and the `.md` face) -- ic's.
- `flag_reachability`'s per-entry id resolution -- ic's.

## TODO

0. ~~**CI GREEN-BY-CONSTRUCTION AND UNVERIFIED.**~~ **VERIFIED GREEN ON REAL RUNNERS, `ee4a7cac`, pushed 2026-08-21 21:49Z.** Both workflows success on both platforms. **`upstream/main` had been sitting at `510d4b10` -- the EXACT revision of the last CI run I diagnosed -- so the whole day's work from four nodes had never been seen by a runner.** `prepush` cloned the pushed revision, built it, ran both binaries and passed clippy under `-D warnings` before letting it through, so what is on GitHub is verified independently of anyone's local tree.

0b. **THE CI CLASS, AND IT IS THE THIRD FACE OF `CORRECT BEHAVIOUR PRODUCES THE FAILURE` (vc folded it as the first of three).** `NO_TOOL_PATH` was written by someone doing the HARDER, more conscientious thing -- manufacturing a genuine absence rather than stubbing `command -v` -- **and the comment proves the care by explaining why a stub would be inferior.** The care produced a constant correct on this machine and exactly wrong on Linux, and **three arms then tested the absent-tool path with the tool PRESENT, green forever. It asserted an absence and never once called `command -v`.** No reviewer checking diligence can see this; the diligent act IS the failure. **Now built as a symlink farm and verified BOTH ways, with both guard arms driven by sabotage** -- and filtering by NAME not DIRECTORY is what makes it survive Linux, where `/bin` symlinks to `/usr/bin`.

0c. **NEAR-MISS, 2026-08-22 08:5xZ, AND ONLY TIMING PREVENTED IT.** matts authorised ME to commit ST0058 and answered vc directly too. I measured the file stable over 25s, wrote the commit, and **vc had landed `4d6bb257` in between -- with a byte-identical subject line I had written independently.** Mine was a no-op. **Had it landed 60s earlier, vc's 685-line design.md would be in the record under a commit its author did not make**, and the estate has NO instrument that can attribute a commit to a node -- five nodes, one checkout, one git identity, `%an` is `Matthew Sinclair` for everyone. **vc's proposal is live for hv: `wb(vc):` carries the node, `feat(0056):` does not, and the whole day's confusion lived in the second set.**

0a. **NEW AIM FROM vc UNDER hv's PEN, REPLACING THE GATE AS TODAY'S PRIORITY: drive v3 to LOCALLY USABLE across the 17 projects on this machine.** Four gates; **cc owns U3's BUILD queue** (ic measures the daily-use population, cc builds). Started on the half depending on nobody: the family-and-subcommand delta driven FROM THE TWO BINARIES, not from `dispatch-table.json` -- `shipped: 115` is a claim about what is IN the binary, never about what WORKS. **`treeindex` to be confirmed absent BY POPULATION, not by one grep** -- vc checked with one and said so.

1. ~~**AC-03.14's second instrument.**~~ **DONE this session -- AT-03.19 green, AC-03.14 satisfied, ST0056/03 16/16 PASS.**
2. ~~**AC-08.5's FACADE CHANGE~~ **BUILT AND LANDED `3f9b2907`; ic covers. Original brief below.** **AC-08.5's FACADE CHANGE -- hv RULED THE SPLIT AND THE OWNERSHIP: cc BUILDS, ic COVERS** (via ic, this session). `facade.rs:4023` refuses `Threads | Thread{..} | Issue{..}` with _this id is server-assigned -- POST to the collection_. **That is a CREATE-shaped justification declining an UPDATE, and the arm does not distinguish the two operations** -- for an AT row `put` is an upsert replacing every field; for a thread it refuses outright, which is why ST0011's `completed` has no write path at all. **The split: `Thread{id}` where the thread EXISTS upserts; create-by-id stays refused; `Threads` and `Issue{..}` unchanged.** Builder and verifier stay separate deliberately because AC-08.5 is ic's gate row -- **do not also write its coverage.** ic corrected their own earlier report against themselves: the unsettable set is EMPTY, not `["file","prose","covers","note"]`, which came from reading `no_named_verb_sets()` at `:92-96` -- a roster of NAMED VERBS, not a settability measurement.

3. **`sync` SKIPS UNTRACKED BYTES, LOUDLY (hv ruled, via dc; `sync` is Rust so the build is mine).** **`loudly` is half the ruling** -- a silent skip makes canon quietly incomplete. Kill the `this run does not refuse, and the commit gate will` clause: under the ruling there is nothing for the gate to catch. **dc's caution is the sharp part: the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED, or it turns a legitimate two-step workflow into a silent no-op.** Comment for the fix site: **ingestion promotes an INHERITED divergence into an ADDED one -- the divergence changes CATEGORY by being written down.**
4. ~~**`render.rs` `#[cfg(test)] mod tests`**~~ **LANDED `1d550ac1`; ic retired `declared_but_unwired` at `b4918a35`. Original brief below.** **`render.rs` `#[cfg(test)] mod tests` driving `unwired("not-a-family", "")`** -- assert exit 2 AND the named path, since exit 2 is reachable from many arms there. **No visibility change needed: a same-module test reaches a private fn.** Then ping ic, who retires `declared_but_unwired`'s loop in one move -- **its `UNWIRED` const cannot simply go empty, that file refuses an empty roster on purpose.**
5. **`AT-00.6` stale `to-write`** while `migrate_v2_project.rs` exists -- the gate reports it every commit. Canon correction, route to vc.
6. **Two `intentdb` doc comments:** `intentsvcs/src/lib.rs:11`, `project.rs:786`. Fold into the next edit in those files.

## BLOCKED, AND ON WHOM

**NOTHING OF MINE IS BLOCKED.** The ST0057 canon block cleared when ic committed both files at `6edbd24f` -- their AT-07.7 green and `address_collections_resolve.rs`, which had been untracked while canon cited it.

**ic IS BLOCKED ON ME**, and has been explicit that they will not move first: they retire `declared_but_unwired`'s loop only after my `render.rs` `#[cfg(test)] mod` lands AND my `st dehydrate` arm is green. **The second half is done as of this session.** The `render.rs` test is TODO 4 and unstarted.

## What changed under the tree today -- you will wake up inside this

- **THE v2 CLI HAS LEFT THIS CHECKOUT.** `~/Devel/prj/Intentv2`, branch `v2-maintenance`, cut at `fb45e9ea` = main HEAD and **NOT the `v2.19.0` tag** -- the old symlink resolved into the working tree, so the fleet had never run the tag, and branching there would have reverted 2027 commits across every project while presenting as a symlink move.
- **`intent` ON PATH IS v2.19.0 AND ANSWERS FOR THE FLEET, NOT FOR THIS TREE.** Drive v3 by explicit path: `./native/rust/target/debug/intent`. **`bin/` is no longer load-bearing for anyone else**, so v2 shell can be pruned here without breaking fifteen projects.
- **THE `INTENT_HOME` STALENESS I RECORDED AT 12:57Z EXPIRED AT 13:36 AND THE CORRECTED STATE IS: THIS SESSION IS FINE.** Driven at 13:2xZ on vc's restart probe, same session: `INTENT_HOME=/Users/matts/Devel/prj/Intentv2`, `intent` -> `Intentv2/bin/intent` (v2.19.0), siblings `intent_st` and `int` -> `Intent/bin` (v3). **That is the correct split and no measurement in this session is suspect.** What I wrote at 12:57Z -- that the shell carried the old value and every PATH-`intent` reading described the old binding -- **was true when written and false within forty minutes**: hv's symlink is stamped 13:36, AFTER my 12:58Z fold, so the binding change completed while I was already paused. **THE LESSON IS NOT ABOUT THIS VARIABLE. A CLAIM WHOSE SUBJECT IS STILL BEING CHANGED BY SOMEONE ELSE HAS A SHELF LIFE, AND A FOLD IS EXACTLY WHERE ONE GETS FROZEN AND READ LATER AS CURRENT.** Stamp the reading, name who else can move the subject, and re-drive before relying on it.
- **BOTH TREES ARE ON PATH AND v3 IS FIRST; THE SYMLINK ONLY EVER PICKED THE ENTRYPOINT** (vc, measured before the switch). `~/.local/bin` at 17 gives v2 for `intent` ALONE; `Intent/bin` at 22 beats `Intentv2/bin` at 23, so **`intent_st`, `intent_critic`, `int` and `devbin` all resolve to the v3 tree.** Harmless today only because the 26 executables are byte-identical and `bin/intent:26` sources every handler out of `INTENT_HOME` regardless -- **the env var picks the CODE, the symlink picks the ENTRYPOINT.** It arms itself the moment v3's `main` diverges. Fix is WP-12's _bin/ (shell) pruned at the cut_, mine, later. **vc tested one binary and concluded about the tree; Lamplight's ic caught it.**
- **THIS REPO'S COMMIT GUARDS NOW RESOLVE OUT OF THE FROZEN v2 CHECKOUT** (`.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`). Identical today; **drifting from the next guard change.** dc holds it as a mechanism -- hv declined direnv and hand-refresh by name.

## The practice -- four shapes of a row promising more than it delivers

1. **UNCITED COVERAGE** -- already satisfied by a test nothing links to it. **PARTIAL uncited coverage is worse: a subject grep that hits the file says nothing about WHICH limbs.**
2. **EXPIRED CITATION** -- the row names a file that cannot cover it. `to-write` is exempt from L2/L3, correctly, **so a citation is unvalidated until someone tries to satisfy it.** `AT-00.6` above is the inverse: built, recorded unwritten.
3. **VACUOUS GREEN** -- true by construction. **The falsifiable arm is over the DECISION, never the OUTCOME.**
4. **TITLE BROADER THAN BODY** -- **and this one leaves no trace at all.** The row is internally consistent: lint passes, the citation is right, the test is green, and it reads as covered to anyone who does not open it. **vc's own 12:16Z gate mislabel is a fresh instance in prose rather than in a row.**

**PRACTICE:** subject-grep FIRST because it is cheap, **then DRIVE THE VERB when it comes back empty.** **And vc's discriminator -- what does satisfying this row COMPLETELY still leave broken? -- is asked against the BODY, never the TITLE**; against a title it returns _nothing_ every time, for exactly the rows where it matters most.

## The class -- A RECORDED REASON RETIRED BY AN UNRELATED CHANGE, WITH NOTHING WATCHING THE JOIN

**SIX INSTANCES IN ONE WEEK, WHICH IS WHAT MAKES IT A CLASS RATHER THAN AN ANECDOTE.** A reason is written down, it is true, it is cited; then a change ELSEWHERE makes it false. **The practice it justified usually stays correct, so nothing looks wrong** -- and the reason keeps being read as current.

1. **AT-03.6's roster reason** -- _no narrow attachment-sync verb ... revisit after ST0057 WP-08_ -- **died at `212b0075`** when `sync --to-store <ID>` landed. Nothing pointed at that row when the verb shipped and nothing could have.
2. **AT-03.6's second blocker** -- _what it needs is a `--staged` MODE, not a call site_ -- **died at `19268867`.** Same row, same week, second expiry.
3. **AT-01.5's two recorded reasons** -- _unmeetable by any edit to the guard, to the roster, or to the template_, and _`pre-commit.intent` here is an install-time COPY_ -- **struck in every clause** by dc's Shape 3 plus `core.hooksPath=.githooks`. **A reader taking them at face value goes at exactly the three places that are now right.**
4. **MINE, 2026-08-21** -- the `INTENT_HOME` staleness above: true at 12:57Z, false by 13:36, frozen into a fold in between.
5. **vc's, same week** -- a rationale that expired while the practice it justified stayed correct, so the correctness of the practice concealed the deadness of the reason.
6. **vc's, and THE WORST SHAPE: `intentdb`.** The term was wrong, it sat inside two quoted hv rulings of 2026-08-15 in `design.md`, and it propagated to all five nodes for six days. **ATTRIBUTION IS WHAT STOPPED ANYONE CHECKING** -- a verbatim quote reads as settled, so the one thing that would have caught it is the one thing nobody does to a quotation.

7. **MINE, AND THE CORRECTION WAS THE ERROR.** I cited `c0749463` as carrying ic's `pgrep` needle fix. **That commit CREATED the file WITH the defect** (302 insertions, `pgrep -f` at `:106` and `:233`), HEAD still carries it, and the fix is UNCOMMITTED in ic's worktree. **I DID re-derive -- I re-derived the WRONG SUBJECT**: I measured that `pgrep -x` is the correct needle (true, reproducible, both directions driven) and offered that as proof THE FILE CONTAINS IT. **So the reason I struck as expired had not expired.** Caught by ic and vc independently, neither asked. **THE LESSON IS NOT `re-derive`, IT IS RE-DERIVE THE SUBJECT THE CLAIM IS ABOUT** -- a true measurement of a different property is the most persuasive wrong evidence there is, precisely because the measuring was real.
8. **THE SAME SHAPE WITH THE SIGN FLIPPED: A GREEN ROW NARRATING ITS OWN REDNESS.** `AT-03.6`'s note opened `STILL RED` after I greened it, and carried two further dead claims (`nothing gates on it`; `whoever admits it wants the path trigger`, which **hv ruled AGAINST this session**). **A stale narrative on a GREEN row is read as current by whoever touches it next, and the row's own recommendation would have sent them against a later ruling.**

**THE FINDING IS THE JOIN, NOT ANY INSTANCE: NOTHING IN THIS ESTATE WATCHES IT.** `at lint` exempting `to-write` is CORRECT, so it cannot see this; a citation is unvalidated until someone tries to satisfy it. **Every one of the six surfaced the same way -- a builder picked the reason up in order to USE it.** That is not an instrument, it is a coincidence of scheduling, and it means the undiscovered ones are exactly those nobody has needed yet.

**PRACTICE UNTIL SOMETHING WATCHES IT: RE-DERIVE A RECORDED REASON BEFORE YOU RELY ON IT, AND RE-DERIVE AN ATTRIBUTED ONE HARDEST.** Re-deriving is also how you find the reason was wrong when written rather than merely expired. **A reason carries a DATE and a SUBJECT-OWNER; if someone else can move the subject, the reason has a shelf life and the citation must say so.**

## Watch-outs -- evidence

- **THE PROVENANCE RULE, THREE LIMBS**: neither the INSTANCE, nor the CONTROL, nor the PREDICATE may come from the thing under test. **Derive the expectation from the fixture's own bytes.**
- **A GREEN MEANS NOTHING UNTIL EACH TEST HAS DIED FOR ITS OWN REASON** -- distinct kill-sets. **Assert the mutation APPLIED.** **Table the matrix as a PREDICTION before driving it**; written after, it is a transcript.
- **"THE TREE DID NOT CHANGE" IS ALSO WHAT A BROKEN VERB PRODUCES.** Every atomicity claim needs the control that the same verb on a passing input DOES change the tree -- **asserted as a change, never `is_ok()`.**
- **A POPULATION IS A CLAIM, AND ITS DEFINING CLAUSE IS WHERE THE ANSWER HIDES.** **EMPTY:** ask an empty directory _was this ever non-empty_, answerable from the RECORD not the listing. **EXHAUSTIVE AND STILL WRONG:** _zero of 110 `.bats` files set the override_ was true, complete, and carried a 302-failure finding -- the only thing that ever set it was `run_tests.sh`, **which is not a `.bats` file.** **Read your own qualifier as the hypothesis it is.** One hides behind a NAME, the other behind a CATEGORY, and no spelling sweep reaches the second. **AC-10.8 above is a live third instance.**
- **FIVE LIMBS WANT FIVE TESTS, NOT FIVE ASSERTIONS** -- the first failure masks the rest.
- **WHEN YOU GREEN A ROW, WATCH THE CRITERION COUNT.** If it does not move, the row was not the last one. **A green row and a closed criterion look identical from where the builder stands.**
- **A COMMENT CAN BE NEVER TRUE RATHER THAN STALE, AND IT CAN ALSO BE ENTIRELY CORRECT AND UNREAD.** Both failures are the same act -- not reading the prose beside the code -- **so the remedy is not distrust comments, it is READ THEM AND THEN CHECK.**

## Watch-outs -- instruments

- **A DENOMINATOR CAN BE CORRECT AND STATED AND THE CONCLUSION STILL WIDER THAN IT.** **Stating your denominator does not stop you generalising past it.**
- **`cargo test` HALTS AT THE FIRST FAILING TARGET** -- 46/366 against the real 141/985/2. **The stopped run's denominator looks exactly like a denominator.** `--no-fail-fast`.
- **A GITIGNORED SSOT IS INVISIBLE TO `git status`.** Walk the filesystem when the claim is _nothing was written_.
- **A GREEN COMMIT GATE IS NOT A GREEN TREE.**
- **BEFORE CALLING A RED A PEER'S, RE-RUN IT WITH YOUR OWN FILES REMOVED.** `common/mod.rs` compiles into every test in the crate.
- **`grep -c` EXITS 1 ON ZERO.** **A ZERO FROM A DATA COMMAND IS SILENT; FROM A MISSING FILE, LOUD; FROM A NAME SEARCH, A FACT ABOUT THE SEARCH.** `cargo --manifest-path <abs>` beats `cd`. **The Bash tool's shell is zsh; hooks run bash.**
- **`FIXED` IS NOT A STATE (vc, 2026-08-21). WORKTREE, INDEX, HEAD AND PUSHED ARE FOUR.** Every node here holds dirty files and commits only on hv's word, so **a peer saying _I fixed it_ reports the FIRST while the reader hears the THIRD -- that gap is this estate's DEFAULT CONDITION, not an edge case.** I did it myself six messages after agreeing with the rule: told dc _the widening landed_ while it was uncommitted. **And a commit citation pointing the wrong way does not FAIL the next checker -- it CONFIRMS to them that the fix is in**, which is worse than one that cannot be resolved, and the cheap does-the-file-carry-the-id split does not reach it.
- **THE BASH TOOL'S SHELL IS zsh AND IT DOES NOT WORD-SPLIT UNQUOTED `$var`.** A probe loop over multi-word verbs passed each as ONE argument and returned `unrecognized subcommand` for all eight -- **a plausible wrong answer, and only the two single-word entries passing revealed it.** Use `eval` or a here-doc loop. **It is in my own memory file and I walked into it anyway.**
- **~~A WHOLE-FILE WRITE PATH STRIPS THE EXECUTABLE BIT~~ -- THAT WAS MY OWN WATCH-OUT AND IT IS FALSE. THE MODE SURVIVES WHEN THE INODE SURVIVES** (dc drove six idioms, cc drove four, 2026-08-21; independently agreeing). `write_text`, `open(w)` and `fileinput` truncate IN PLACE and **preserve** the bit however much they rewrite; `sed > tmp && mv`, `open(tmp) + os.replace` and `mkstemp + os.replace` create a NEW file at the umask and rename it over the old, so the original mode is never consulted -- 644, or **600** from `mkstemp`. **MY RECORDED CAUSE FOR `d8dd6dc6` WAS `write_text()` AND DRIVING IT REFUTED THAT, SO THE CAUSE IS UNKNOWN.** dc's at `19d77f61` is `sed > tmp && mv`, tested. **Two occurrences, ONE mechanism established** -- _observed twice, mechanism established once_, and not yet a finding about the file. **THE WATCH-OUT WAS WORSE THAN NOTHING WHILE IT STOOD: it pointed at an innocent idiom and implicitly cleared the guilty one**, so a reader avoiding `write_text` would have reached for a temp-and-rename. **`f6face5f` restored the state and left the mechanism, which is indistinguishable from a fix that worked until the next occurrence** -- and the next occurrence was 8 hours later, by a different author, in the same file. **Now CHECKED rather than remembered: `runner_roster_check.sh` check D.**
- **`sync --to-disk <ID>` PROJECTS THE WHOLE THREAD, INCLUDING WHATEVER A PEER ALREADY PUT IN THE STORE.** Mine pulled ic's AT-07.7 green and ic's uncommitted attachment bytes into canon on disk, leaving canon naming bytes no commit contains. **Thread-scoping bounds the THREAD, never the AUTHORSHIP.** Check the diff before committing; the store is shared.
- **A GATE FIGURE WITHOUT ITS REVISION IS THE THIRD WRONG NUMBER THIS ESTATE HAS SHIPPED, AND THIS ONE WAS NOT ARITHMETIC.** 64 is the STORE figure over four dirty worktrees; 62 is HEAD's, driven clean-room. **The store is gitignored, so `ac status` answers about state no clone can reach**, and every node reads the same store and gets the same reassuring number. **A REPORT CARRIES THE STATE AT OBSERVATION AND IS READ AS THE STATE AT DELIVERY** -- same axis as `fixed is not a state`, rotated to observed-versus-relayed. **Publish N-of-M with its revision AND which store answered, or do not publish it.**
- **A REVISION NAMES SOURCE, NOT THE BINARY THAT ANSWERED** -- `shasum -a 256` and quote the hash WITH the number. **NAME REVISION, CLOCK AND DIRTY COUNT ON EVERY MEASUREMENT.**
- **MARK PROVENANCE PER CLAIM: DRIVEN, READ, OR INFERRED.** **The cost lands on the READER, which is why the writer never feels it.** **VERIFY THE RETRACTION, NOT JUST THE CLAIM.**
- **A BACKGROUND WAITER'S EXIT CODE IS ITS OWN, NEVER THE WATCHED PROCESS'S VERDICT.** Redirect the run's own rc to a file and read it there.
- **AN UNQUOTED HEREDOC IS A SHELL, AND IT ATE THE ONE PART OF A MESSAGE THAT MADE IT CHECKABLE** (mine, 2026-08-21). I used `<<EOF` rather than `<<'EOF'` to interpolate a clock stamp; the body held a fenced block of `a -> b` mappings, so zsh ran `vc`/`ic`/`dc` as commands and turned three `->` into REDIRECTIONS -- creating three empty files named after session ids, in the repo root. **The prose all landed and only the EVIDENCE TABLE vanished, so the entry read as complete and merely unsupported.** The commit succeeded. **When a heredoc must interpolate, interpolate ONE variable and keep the body quoted -- or write the body with a quoted heredoc and substitute afterwards.** `git status` in the repo root is what surfaced it, not the transcript.
- **`ListAgents` "started" IS SOCKET AGE, NOT SESSION AGE, AND FOUR NODES GOT THE SAME WRONG ANSWER FROM IT ON 2026-08-21.** I read three peers as _started ~5 minutes ago_ and concluded **three of four bounced**. **Zero bounced.** When the topology changed every peer re-registered, so **every node saw the other three as fresh and itself as resumed** -- four correct self-reports and one unanimous wrong inference about the population. **UNANIMITY ACROSS INDEPENDENT NODES IS NOT CORROBORATION WHEN ALL FOUR READ THE SAME INSTRUMENT**; it is one reading counted four times, and it feels like the strongest evidence available. **A self-report is first-hand; a peer's state read off an instrument is not, and the two must never be summed.** I put the wrong figure in vc's inbox at `461ef8e6` before vc corrected it.
- **A SNAPSHOT-ONLY DRIVER CANNOT TELL `WROTE NOTHING` FROM `NEVER RAN`, AND `WROTE NOTHING` IS WHAT PASSING LOOKS LIKE** (mine, 2026-08-21). A CLI driver that compares two estate snapshots goes green against a refusing binary, a typo'd subcommand, or a fixture the verb cannot act on. **Every case must declare its exit code AND whether it is required to have written**; both get asserted. Mine caught its own first case this way, unplanted -- `todo done` exited 1 on an illegal transition and left the estate untouched. **This is ic's `zero test result: lines reads UNMEASURED, never green` at a different altitude, and it generalises: a run that did not measure must never report the shape of a pass.**
- **AN EXIT CODE IS NOT A REASON.** `rc=2` is the unimplemented arm AND clap's missing-required-argument refusal. `st dehydrate` driven bare would have passed on the PARSER's refusal while claiming the VERB's -- a green case citing a mechanism it never reached. **Pin the stderr phrase alongside the code whenever the code has more than one producer.**
- **A STATED REASON CAN BE TRUE AND STILL NOT BE WHAT THE CASE DRIVES.** My `todo list` entry claimed store materialisation while its own prep never removed the store. **The claim was correct, sourced, and unexercised** -- the row-promising-more-than-it-delivers shape, arriving inside the instrument built to catch it. **Read every reason against its own setup, not against your memory of the measurement.**
- **A LOAD-BEARING DEPENDENCY RECORDED ONLY IN A DOC COMMENT IS INVISIBLE TO EVERY TOOL HERE** (ic's catch, 2026-08-21). `flag_reachability.rs` duplicates a literal deliberately so that **no copy is authoritative**, and named `declared_but_unwired.rs` as its origin -- so retiring that file would have PROMOTED the survivor to authoritative, destroying the property, via a deletion that looked like it only touched a loop. **`at lint` sees AT-row citations and sees nothing in a doc comment.** And the documentation is what persuades the next reader not to re-check it, which makes a documented coupling worse than an undocumented one. **State the INVARIANT, never the origin.**
- **A RATCHET LEFT AT ITS OLD VALUE AFTER THE DEBT SHRINKS PERMITS EXACTLY THE REGROWTH IT WAS INSTALLED TO PREVENT -- WHILE STILL READING AS A RATCHET.** `UNPROVEN.len() <= 32` with 22 members is not a bound, it is ten free slots. **Tightening is part of discharging, not a tidy-up afterwards.**
- **CITE THE PROPERTY, NOT THE FILE THAT HOLDS IT** (ic, 2026-08-21, caught against themselves). The property outlives the file; the reader needs the property. ic cited `declared_but_unwired.rs:59` at `11c2037d` and deleted that file at `b4918a35` -- **after spending the afternoon reporting this exact class in my files and dc's.**
- **A FIGURE WITH A REVISION AND A MACHINE AND NO LOAD CONDITION IS STILL UNCOMPARABLE ON A SHARED MACHINE.** Four sessions and cargo builds run here all day; `view_skew_check.sh` spread 3409-4275ms over five runs, and **a 25% spread is the signature of CONTENTION, not of the tool.** Every timing row in the roster has this hole, mine and dc's.
- **A HYPOTHESIS FROM A PEER'S OWN CORRECTION IS STILL A HYPOTHESIS ABOUT YOUR FILES.** dc found ~50ms of harness in their row and flagged that mine might share it. **Driven: my harness floor is 5ms and both my figures came back HIGHER, which a 50ms artefact could not produce.** Their finding was real and did not generalise -- and the only way to know was to run it, which is why _your rows are yours to re-measure_ is a rule about who runs it, not about who is right.
- **AN ANNOUNCE THAT LEAVES WITH A 60-SECOND MUTATION CANNOT ARRIVE BEFORE IT** (dc, 2026-08-21). Latency is counted in the RECIPIENT's turns and the sender cannot see them. **So for a short window the notice buys ATTRIBUTION, never prevention** -- what protects a peer is `--only` plus a byte-copy restore. **Filing it as announce-before records a safety property the mechanism does not have**, and the next person relies on it.
- **ANNOUNCE A WRITE TO A SHARED FILE TO EVERYONE; A WRITE TO A CLAIM TO THE CLAIM-HOLDER.** vc announced their `wp done` to me, correctly, because WP-03 is my claim -- and it BLOCKED dc, who holds no claim on ST0056 at all. **Claims predict who CARES, never who is BLOCKED**, and in a one-file-many-owners artefact those are different sets. Canon is exactly that artefact.
- **READ THE HUNK, NEVER THE SUMMARY, WHEN THE SUMMARY WILL BECOME A CLAIM** (dc's correction of themselves). `--stat` said _one line in a thread canon_ and that stood in for _which line_; it was `wps[seq=3].status`, not the thread's, and the misattribution reached me twice. **Two characters of context away the whole time** -- same shape as counting a `test result:` line inside a `tail`.
- **MY OWN, AND IT IS THE REVISION RULE FAILING IN A NEW DIRECTION: A FIGURE READ AT ONE REVISION CARRYING AN ENUMERATION FROM ANOTHER.** I reported _seven status-gate disagreements_ -- correct when read -- and attached a list of eight, because `ST0056/WP-03` entered the set when I greened AC-03.14 minutes later. **vc reported seven too, from a `head -14` truncation. We did not make one error twice; we made two errors that happened to agree**, which is a stronger confirmation signal than either would have been alone.

## Watch-outs -- four nodes, one checkout

- **PRESENCE IDENTIFIES A FILE AND NEVER ITS AUTHOR**; the working tree is nobody's tree. A peer's red conceals mine and reads as an all-clear.
- **A PEER CAN SWEEP A FILE THEY DO NOT WRITE, AND THE SINGLE-WRITER RULE DOES NOT STOP IT -- IT ONLY SAYS WHO WAS WRONG** (measured on my own fold, 2026-08-21). I wrote `dc/inbox.cc.md` at 12:59Z; **dc's fold commit `ad37745f` carried it**, so my message is in the record wearing dc's authorship and `git status` reported the file clean while I was still holding it. **I am that file's sole WRITER; dc is its sole READER, and dc committed it.** Benign here -- the path encodes the routing and the content is intact -- but the shape is the transcription-laundering class arriving through a COMMIT rather than through a quoted stamp. **The tell is a file you just wrote reporting no diff.** When a write of yours goes quiet, check `git log -- <path>` before re-writing it; re-writing would have produced a duplicate entry attributed to two commits.
- **THE WORKTREE AT A NAMED REVISION IS THE ONLY REMEDY FOR THE `--only` CLASS.** `--only` is path-scoped, not hunk-scoped, **and silently skips untracked files inside a named directory** -- `git add -N` first. **Check the checkout SUCCEEDED: `git checkout --detach` ABORTS on an untracked file in the way.**
- **A LIVE CHANNEL DELIVERS AND LEAVES NO RECORD, AND THE GUARDS RUN AT COMMIT, SO THE LIVE CHANNEL IS UNGUARDED.** **The hazard is TRANSCRIPTION: quote a peer's live stamp into a file and it enters the record laundered through you, past a guard watching the wrong door.** **Attribute it, never assert it.** Every hv stamp on this board is vc's attribution, marked as such, and not a time I read.
- **AN INBOX WRITE IS A RECORD, NEVER A DELIVERY** (vc, twice in one day). **A write surface with no named reader is a queue.** Six entries sat in my inbox from 10:26Z and I read them at 12:5x only because vc sent a live message.
- **`prettier` RUNS INSIDE THE COMMIT WINDOW** and re-stages, **after `sync --to-store` hashed the worktree.** Order is **FORMAT, then SYNC, then COMMIT** -- never "sync last", which reads as sync-after-commit and leaves that commit permanently divergent. **It is one formatter with two triggers, not two formatters.**
- **A TABLE EDIT IS A TWO-FILE COMMIT WHOSE SECOND FILE IS ONE YOU NEVER EDITED** -- after editing any SOURCE, ask what renders FROM it before staging.

## Standing rulings

- **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE.** **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE.** A bare `AC-03.6` is GREEN in ST0056 (FTS prose bodies) and RED in ST0057.
- **HIGHLANDER FORBIDS TWO ANSWERS TO ONE QUESTION; IT DOES NOT REQUIRE ONE ANSWER TO TWO.** **Two mechanisms enforcing different properties are not two copies of one** -- prevention and refusal are different criteria.
- **TWO ROWS RATHER THAN ONE WIDENED ROW** keep two assertions separately falsifiable.
- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION.** Everything found building v3 is work.
- **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON** -- and re-deriving the reason is how you find the reason was wrong.
- **A CONTROL THAT DEPENDS ON THE AUTHOR REMEMBERING IS NOT A CONTROL, IT IS A HOPE WITH A FILENAME.**
- **`treeindex` and handover RETIRE**; a retired command is PRESENT AND REFUSING. **`doctor --fix` is WITHDRAWN.**
- **`DO NOT PUT v3 ON PATH` IS RETIRED, SUPERSEDED BY hv 2026-08-21.** `.envrc` (`9b883bd1`) puts `native/rust/target/debug` first INSIDE this checkout, and v3's own `intent info` reports THIS tree because it resolves from `current_exe()` ancestors and IGNORES the exported `INTENT_HOME`. **THE AUTOMATION HALF IS STILL OPEN AND hv RULED IT SO:** direnv does not reach a tool-driven shell -- measured, `DIRENV_DIR` unset, `which intent` = v2.19.0 -- **so every NODE commit still resolves guards out of the frozen `Intentv2`, and the nodes do most of the committing.** Interactive prompts get the fix; automation does not. The drift exposure remains dc's. **`config.json` DOES NOT MOVE WITH `intent_dir`.**

## Lane and build recipe

`native/**` and the v3 crates are mine. Parity harness = ic. Hooks, roster, `int hooks`, `canon_commit_check.sh`'s admission = dc. **Canon writes route through vc.**

**`CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc` FOR ANY VERIFYING BUILD** -- absolute and in-repo. Out-of-repo breaks `INTENT_HOME` resolution (`install::home()` walks `current_exe()` ancestors for a marker dir) and manufactures phantom failures; relative under a drifted cwd builds where gitignore hides it, once at 1.2G. **`rustfmt --edition 2024`, NEVER a bare `cargo fmt`.** **Drive v3 as `./native/rust/target/debug/intent`; `intent` on PATH is v2 and answers for the fleet.** **Run the shell suite through `tests/run_tests.sh`, NEVER `bats` directly** -- the runner exports `INTENT_FIXTURE_VERSION` from `VERSION`; a direct run builds a v3 fixture against the v2 binary and dies on the version guard, 300 refusals reading as 300 failures.
