---
node: ic
name: Interface Claude
role: interface
session_id: 6e1c92e1-44be-4a97-b2bb-69a3a25e8f04
heartbeat_at: 2026-08-21 13:42Z
status: paused
focus: "**FOLDED, DELTA-FOLDED AND HOLDING FOR A REAL BOUNCE. The 12:55Z fold stands; this is the delta, not a redo.** **ZERO OF FOUR NODES BOUNCED -- all four are resumes, and all four independently concluded three of four bounced but not me, each reading ListAgents SOCKET age as SESSION age.** Four correct self-reports, one unanimous wrong population; my board never carried the figure. **AND A MISATTRIBUTION ARRIVED IN THE FOLD INSTRUCTION: I was asked to fold a zsh path-clobber that did not happen in this session, and I declined to adopt it -- my PATH is intact at 23 entries.** **ROOT CAUSE FOUND BY vc WITHIN THE HOUR AND IT IS BIGGER THAN THE INSTANCE: the incident is LAMPLIGHT-ic's, and MONIKERS ARE ESTATE-SCOPED WITH NOTHING MARKING THEM.** Same class as intentdb, arriving in a fold. AC-07.7 still unstarted by instruction; three calls still open with hv."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. NOTHING STARTED TODAY, AND THAT WAS THE INSTRUCTION RATHER THAN THE DAY RUNNING OUT.** The session booted at 09:29Z, researched AC-07.7 to the point of a written plan, put three calls to matts, and was folded before a line of code. **The plan is in the TODO rows below, not in anyone's memory.**

**THE PRE-FOLD BOARD AND vc's INBOX ARE AT `.history/20260821/`** -- `wip-prefold-1254Z.md` and `inbox.vc.md` verbatim. Every rule below is stripped to the rule; the instance is in the archive.

## ON RESUME -- the part that existed nowhere but in conversation

### 1. D57-8's amended fence has a DURABLE home, and it is not the disk file

**`intent/st/ST0057/design.md` is an ATTACHMENT of ST0057 in canon**, not thread prose and not a generated view. `intent/.canon/st/ST0057.json` carries `attachments[].path == "design.md"` with the **full text (33538 chars), `bytes: 33538`, `sha256: abc9a20584606699378953b73310c9775bf67f34876e0cb537784b3e049e1e49`**, and the disk byte count matches exactly. **So reading the fence off disk is reading the projection THIS THREAD EXISTS TO MAKE OPTIONAL.**

The fence is the **SECOND** fenced block under `## D57-8` (`design.md:228` is the heading). The first is the nine entity forms; the second is the four collections, added 2026-08-20.

**THE RULING I TOOK: parse the fence out of CANON, and cross-check canon's recorded `sha256` against the disk bytes when the disk file is present.** An attachment is authored on disk so canon can lag until `sync --to-store`; the cross-check turns _canon is stale_ into a named failure instead of a silently short denominator, and it degrades correctly when the file is dehydrated. **Precedent, already ruled in this estate: `preconditions.rs` reads canon and never `acceptance.md`, and REFUSES on finding nothing rather than passing vacuously.** Scoped to this one attachment and this one test -- the general class is `canon_commit_check.sh` and it is cc's.

**THE ONE THING NOT TO DO IS HAND-COPY THE FOUR.** `d57_8_forms()` is a hand-copy and D57-8's own amendment names that hand-copy as the mechanism that went four short.

### 2. AT-07.7's singularity arm ALREADY EXISTS, and writing a second is the Highlander defect

**`no_second_resolver_exists` at `address_resolution_single_home.rs:138`** scans `intent-cli` and `intentd` sources for the scheme literal. **Its subject is _does any other crate spell the scheme_, which is FORM-INDEPENDENT -- so it already covers the four collections.** A second scheme-scan in a new file is two answers to one question.

**Proposal, NOT taken, open with matts:** lift `collect_scheme_hits` into `intentsvcs/tests/common/` and call it from both. **It has no population over forms, so it cannot move AC-07.1's meaning** -- unlike `d57_8_forms()`, which stays untouched. Flagged rather than assumed because it edits AT-07.1's file.

### 3. THE FINDING: one enum, two matches, and only one of them was built to catch a tenth variant

**`address.rs:606 view_path_of` matches `Entity` with `_ => None`.** Thirty lines up, **`Entity::form()` at `:165` is exhaustive ON PURPOSE** -- its own doc says _a tenth form does not compile until it is named here_, and calls a hand-kept roster beside an enum "the shape that shrinks".

**So a tenth entity form silently acquires no markdown rendering, and nothing fails to compile.** The wildcarded match is the one that decides whether an address has a representation at all. **`address.rs` is mine (AT-07.3), so the fix is mine: name every variant, `None` where correct.**

### 4. Two of the four collections have NO rendering, and that is FAITHFUL -- measured, not assumed

`view_path_of` maps `Threads -> steel_threads_view`, `Thread -> info_view`, `AcCollection -> acceptance_view`, `Wp -> wp_info_view`, and **`Issues` and `WpCollection` fall to `None`** (`NoMarkdownRendering`).

**Checked rather than reasoned: `Project` exposes FIVE view accessors (`project.rs:798-822` -- `steel_threads_view`, `todo_view`, `info_view`, `wp_info_view`, `acceptance_view`) and `views.rs render_all` emits FOUR of them.** **There is no issues index view and no WP index view in this estate**, so those two collections have nothing to serve. D57-8 is faithful: the POST clause makes a collection addressable so a create has a target, and the under-addressing clause defines exactly one collection-to-view mapping.

**SO AT-07.7 ASSERTS THE REPRESENTATION SET AS AN EXACT SET** -- `Threads` and `AcCollection` have one, `Issues` and `WpCollection` do not -- **which fails if a rendering is ADDED or REMOVED.** The population goes in the assertion; an unstated one is the vacuous green.

**Loose end, not a row: `Project::todo_view()` exists as an accessor and `render_all` never emits it.**

### 5. The red-first arm needs a PAIR of mutations

Break `AcCollection`'s parse arm -- AT-07.7 must go red. **Then, separately, confirm a POST-clause-only test (`Threads`/`Issues`/`WpCollection`) does NOT go red on that same mutation.** One planted break proves the test fires; **the pair proves this row is not AT-07.1 wearing a different id.**

### 6. Provenance of my own morning measurement, and it names the wrong binary

**I drove `native/rust/target/RELEASE/intent` for the gate, not `debug`.** vc's directive names `./native/rust/target/debug/intent`. The numbers I got -- **ST0057 47/51 satisfied 2 withdrawn, ST0056 59/132 satisfied 1 withdrawn, at HEAD `706db8ee`, 09:35Z** -- match the EOD record exactly, so they are consistent. **But the instrument is not the one the estate now names, and a re-measure uses `debug`.** **AND THE COMMIT GUARD SHARPENED THIS AN HOUR LATER: `self_provenance_check.sh` reports `native/rust/target/release/intent [sha256 957aa2b2e9029f5b]` was built from an UNCOMMITTED tree, `dirty-483e65e49190d6134d31ae312ccb0319b3da68b2`.** So the binary that produced my gate numbers **names no commit at all**, and the marker does not distinguish it from any other build of the same dirty tree -- **pin by the hash, never by the marker.** The numbers agreeing with the EOD record is what makes this survivable, not what makes it measured.

### 7. ZERO OF FOUR BOUNCED, and the check I distrusted was worse than I had said

**vc checked on my ask, and the answer is unanimous the other way: cc, dc and vc are ALL resumes** -- unchanged `session_id`s, this morning's conversation intact, exactly as I am. **And all four of us independently concluded _three of four bounced, but not me_**, each citing `ListAgents` showing the OTHER three as started ~5 minutes ago.

**`ListAgents`' "started" IS SOCKET AGE, NOT SESSION AGE.** When the topology changed every peer re-registered, **so everyone looked fresh to everyone else.** Four correct self-reports, one unanimous wrong inference about the population.

**My board never carried the figure** -- checked before folding, `three of four` appears nowhere in `ic/wip.md` or in what I sent vc. What I sent was the question, not a count. **The only reason anybody asked was distrusting the pooling**, which is the whole of item 8 below one day earlier than it deserves.

### 8. A MISATTRIBUTION ARRIVED INSIDE THE FOLD INSTRUCTION AND I DECLINED TO ADOPT IT

vc's fold instruction named two things as existing only in my conversation. **The second was not mine.** It described clobbering zsh's `$path` via `while read -r sha path`, restoring it by hand from a list omitting `~/.local/bin`, being unable to know whether it had persisted, and therefore **measuring both states and labelling them rather than reporting one.**

**THAT DID NOT HAPPEN IN THIS SESSION.** I ran vc's check verbatim and nothing else. **Verified rather than remembered: my PATH is intact at 23 entries, `.local/bin` at 17, `Intent/bin` at 22, `Intentv2/bin` at 23** -- which is the true topology vc themselves described, so my check output was measured against an uncorrupted PATH and independently corroborates that vc's restoration was correct.

**IT IS NOT MINE. RESOLVED BY vc WITHIN THE HOUR AND IT IS NOT vc's EITHER -- IT IS `lamplight-ic`'s.** hv pasted a Lamplight session transcript into vc's context (four Lamplight nodes answering the same env check); **vc read it and handed it to `ic`, meaning the INTENT one. Same three letters, different estate, and nothing in the protocol marks the difference.** vc ran my check: no trace of the block on cc's or dc's boards, and every other item they sent was sourced from that node's own reply. **Contained at one.** The MECHANISM is worth keeping and is recorded as vc's in the Watch-outs below. **The OWNERSHIP is not, and adopting it would have written an incident ic never had into ic's permanent record** -- where the next session would have read it as its own experience and had no way to tell. **Same class as `intentdb`: a wrong thing adopted from a trusted peer's phrasing, in a fold, and never challenged.** **The handling -- measuring BOTH states and labelling them rather than reporting one to a waiting hv -- is genuinely good practice, and it is `lamplight-ic` who earned it.** I had credited vc; vc corrected it back to the node that did it, which is the same discipline running in the generous direction.

## TODO

1. **AT-07.7 / AC-07.7 -- THE FOUR COLLECTION FORMS RESOLVE.** Mine, minted `c5320329`, **still not started, now by instruction.** Build it from ON RESUME 1-5 above. New file `intentsvcs/tests/address_collections_resolve.rs`; **NOT** `address_resolution_single_home.rs`, and **`d57_8_forms()` is not its home** -- growing that list to thirteen would silently move AC-07.1's population and make a satisfied criterion mean something it was never assessed against.
2. **THREE CALLS OPEN WITH matts, PUT THIS MORNING AND UNANSWERED WHEN THE FOLD CAME.** (a) Is the two-of-four representation reading accepted -- parse-only, or parse plus representation? **I flagged this AGAINST MYSELF: the parse-only reading is the one that conveniently greens my own row.** The tell, not the virtue. (b) May the scheme scanner be lifted into `tests/common/`, which edits AT-07.1's file? (c) Order -- AT-07.7 first, or clear cc's two asks first?
3. **`intentdb` RETIREMENT -- MY TWO SITES, AND vc DELIBERATELY DID NOT TOUCH THEM.** `surface/dispatch-table.md` (D30/D31 target prose) and `surface/dispatch-table.json` (the generated face) both carry _"... into durable state in the intentdb"_. **Two sites, one string, and WHICH IS AUTHORED AND WHICH IS GENERATED IS MY CALL** -- a hand-edit to a generated face and its source is exactly the skew my own arm exists to catch, so the regeneration is mine to sequence. hv retired the word corpus-wide (vc reports ~12:00Z, standing directive on `hv/wip.md`): **there is no `intentdb`; `intentd` and `intent-cli` are BOTH clients of `intentsvcs`, which solely owns the SQLite db.** The SUBSTANCE of D01 is unchanged -- only the term was wrong.
4. **`no_daemon_required.sh` IS DEFECTIVE AND IT IS IN MY WP (AT-07.5 / AC-07.5, ST0057 WP-07). AC-07.5's GREEN IS NOT IN DOUBT; THE INSTRUMENT IS.** The needle at `:106` and `:233` is `pgrep -f 'intentd'`, and **`-f` matches the whole command line as an unanchored substring, so it matched `intentdb` in every MAAC node's `--append-system-prompt`.** vc drove it at `49be1059` 11:55Z: arm A refuses at exit 2 saying _an intentd process is already running_ **with no intentd running** -- three `claude` processes matched. **Under MAAC this arm refuses 100% of the time and blames a daemon that does not exist, so it fails precisely when the most nodes are working and is invisible to anyone running alone.** Fix: anchor on the executable (`pgrep -x intentd`) or the binary path; **keep the refusal semantics exactly as they are -- refusing at 2 rather than passing is the only reason this was findable.** **MY CALL whether it takes a row.** **AND RETIRING THE WORD DOES NOT FIX IT** -- `intentd` is legitimate and will always be in the corpus. Two fixes; do not let the first look like it closed the second.
5. **`doctor --json`** (cc's ask): **measured today -- `doctor::Report` at `doctor.rs:53` derives `Debug, Clone, Default` and NOT `Serialize`; `Finding` already has it.** Plus a row in the dispatch table. **`thread_view_skew_check.sh` parses doctor's TEXT as a dated workaround and must be DELETED when the face lands, not kept beside it.** Traps: `--json` is declared at both family and verb level where a family has a bare form, and the tool spells this two ways already (`--json` vs `critic --format json`) -- **raise the divergence, do not rule it.**
6. **`declared_but_unwired.rs:65` gets a SYNTHETIC member** (cc's ruling): **unchanged today -- still `UNWIRED: &[&str] = &["st dehydrate"]`**, which borrows a live defect as its fixture, so it reds for a good change.
7. **AC-08.5 -- ONLY THE THIRD BURNING CASE IS MINE.** ST0011's `completed` (a THREAD field, no setter) and the attachment's narrow canon setter are service-layer and **route, do not assume**. **Mine: no CLI verb creates an AC or an AT** -- `Facade::put` already creates both (`facade.rs:2469`, `:2511`), so this is a SURFACE gap over an existing capability, which is my lane. **AT-08.5 stays RED and red is the criterion's own verdict**, so this does not green on the CLI verb alone.
8. **NOT MINE TO RULE:** whether the `BEGIN/END INTENT` marker grammar survives. hv left it out of ruling 4; vc raises it.

## Watch-outs

### Mechanical

- **`ListAgents`' "started" IS SOCKET AGE, NOT SESSION AGE.** A topology change re-registers every peer, so **all peers look freshly started to each other.** Never infer another node's session lifetime from it -- **ask for `CLAUDE_CODE_SESSION_ID`, which is the discriminating field and costs one `echo`.** Measured 2026-08-21: four nodes, four correct self-reports, one unanimous wrong population.
- **`read -r sha path` SILENTLY DESTROYS zsh's `$path`** (**`lamplight-ic`'s incident, 2026-08-21 -- NOT mine and NOT vc's; corrected by vc, who declined the credit rather than let it accrue to them through the same collision running the other way.** Recorded because nobody rediscovers it cheaply; the ownership is the point of ON RESUME 8). `path` is a zsh special tied to `PATH`, so using it as a loop variable clobbers the shell's search path, **and a hand restoration is likely to omit an entry.** If it happens mid-measurement, measure BOTH states and label them rather than reporting one.
- **`grep -rn` OVER `native/rust` TIMES OUT AT 120s** -- it walks `target/`, which holds four build dirs. **Scope to `native/rust/crates`.** Cost one backgrounded call this morning.
- **THE DISPATCH TABLE IS AT REPO-ROOT `surface/dispatch-table.json`**, not under `native/rust/crates/intent-cli/`. There is no `surface/` dir in the crate. Three calls to find it.
- **`intent` ON PATH IS v2.19.0 AND ANSWERS FOR THE FLEET, NOT THIS TREE** -- it refuses at exit 2, which is correct and not a tree problem. **Drive v3 explicitly: `./native/rust/target/debug/intent`.** The v2 CLI left this checkout to `~/Devel/prj/Intentv2` (branch `v2-maintenance`, cut at `fb45e9ea` = main HEAD, NOT the `v2.19.0` tag). **`bin/` is no longer load-bearing for anyone else.**
- **THE GATE IS 62 OF 67 AND IT TAKES THREE VERB CALLS, THE THIRD OF WHICH NOTHING IN THIS ESTATE EVER MENTIONED:** `ac status ST0057` (47/51) plus **`ac status ST0056/03` (15/16) -- a WP-SCOPED STID the verb accepts.** `ac status ST0056` is the WHOLE THREAD (59/132) and there is no path from it to 67. **The guard against hand-tallying was the vector for it** (vc's finding, in a fold vc wrote).
- **62 of 67 IS ST0057's CLOSURE GATE, NOT THE 3.0.0 RELEASE.** The release is ST0056 WP-12, dependent on all prior WPs, and **ST0056 is 59/132 with SEVEN WPs Not Started.** Read as release progress it says 93% where ST0056 is at 45%.
- **`git commit --only` is PATH-scoped, not hunk-scoped** -- it takes a peer's unstaged work in any path it names, and two bodies of work in one file cannot be split. **Only a detached worktree AT the revision sees a broken published tree.** Never `cp` a shared source aside to mutate it.
- **The Bash tool is zsh and its cwd PERSISTS between calls** -- absolute paths always. Unquoted globs in `--include='*.rs'` are a hard error; unquoted `$var` does not word-split; `grep -c` exits 1 on zero; **never `$?` after a pipe.** **The last two compound**: a redirect plus a pipe turns a wrong-cwd miss into `rc=0` and no output, which reads as _searched and found nothing_. **An instrument that cannot say WHERE it looked cannot report an absence.**
- **`cargo test --workspace` stops at the first failing target -- always `--no-fail-fast`**, and **never pipe the log through `tail` before counting.** Write the whole log; count with `awk '/^test result:/'`.
- **Keep `CARGO_TARGET_DIR` inside the tree being built** (`native/rust/target/ic`, gitignored) -- `INTENT_HOME` walks up from the binary's path.
- **A clean `git apply --3way` is not a correct rebase** -- it reports on TEXT; only the suite reports on meaning. It also STAGES, so the following diff must be `git diff HEAD`.
- **Never drive a mutator on the live estate.** A probe is not a test and the estate is not a fixture.
- **The markdown formatter is a second writer** -- `_..._` in table prose.

### Estate

- **MONIKERS ARE ESTATE-SCOPED AND NOTHING MARKS THEM.** `ic`, `cc` and `vc` exist in **Intent, Lamplight and Laksa simultaneously**, and the protocol has no qualifier -- the roster is per-project, so within one board `ic` is unambiguous and ACROSS estates it is not. **Cross-estate relay is live, not hypothetical:** hv pastes transcripts between estates and vc messages `lamplight-vc` directly. **Write the qualifier every time -- `lamplight-ic`, never `ic`.** This is the root cause of the 2026-08-21 misattribution and it is a protocol gap, not a slip: **nothing on either side of the relay carries the estate, so the collision is silent in both directions.**
- **An attachment is authored on disk, so a divergence means the STORE is stale.** `sync --to-store <ID>` takes the disk copy and **`sync --help` says the opposite**; it TAKES ids, thread-scoped.
- **`design.md` IS AN ATTACHMENT, AND CANON CARRIES ITS FULL TEXT PLUS A sha256.** So a design document has a durable machine-readable home, and a test whose denominator is the DISK copy is a test whose denominator this thread exists to make optional.
- **THIS REPO'S COMMIT GUARDS NOW RESOLVE OUT OF THE FROZEN v2 CHECKOUT** (`.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`). Identical today; **drifting from the next guard change.** dc holds it as a mechanism -- hv declined direnv and hand-refresh by name.
- **`intentsvcs` is the dependency root** -- announce the blast radius, not the files, and **announce before adding an enum variant.**
- **The live channel is unguarded** -- use commits when you need ordering you can prove. **Attribute a peer's live stamp; never assert it.**

### Judgement

- **ASK WHAT THE INSTRUMENT WOULD SAY IF THE THING IT MEASURES WERE GONE, AND COULD THIS HAVE COME BACK THE OTHER WAY.** Everything below is an instance.
- **A change that would conveniently green your own work is the one to stop and route.** The tell, not the virtue.
- **A BINARY ANSWER THAT TWO DIFFERENT CAUSES BOTH PRODUCE IS NOT EVIDENCE, AND POOLING IT ACROSS NODES CONVERTS IT INTO A CONFIDENT WRONG ANSWER.** _Is the new heading in your system prompt_ cannot separate **the rewrite failed to reach a relaunched session** from **this session was never relaunched**, and those have OPPOSITE remedies -- a delivery defect versus a node the bounce missed. **Add the discriminating field.** The generalisation is worth more than the instance.
- **A FOLD INSTRUCTION IS NOT A TRUSTED SOURCE ABOUT YOUR OWN HISTORY.** A peer telling you what only you could know is telling you what THEY know; check it against your own record before adopting it, because a fold is exactly where an unchallenged claim becomes permanent.
- **ONE ENUM CAN CARRY TWO MATCHES, ONE EXHAUSTIVE BY DESIGN AND ONE WILDCARDED, AND THE WILDCARDED ONE IS THE ONE THAT DECIDES SOMETHING.** Proximity is not protection: `Entity::form()` and `view_path_of` are thirty lines apart in one file.
- **A denominator belongs to a FILE, not to a topic** -- and adjacent bullets read as cause and effect with neither one claiming it.
- **A row's TITLE can promise more than its BODY, and no instrument here reaches it.** So the discriminator _what does satisfying this row completely still leave broken?_ **must be asked against the BODY, never the title** -- against a title it returns _nothing_ every time, for exactly the rows where it matters, **and that nothing is indistinguishable from a correct answer.**
- **One planted write proves a test fires; a PAIR proves neither row is the other wearing a different id.**
- **A cross-check reconciles when both sides share the same error, and it then reads as confirmation** -- and it self-heals. **A number measured in a shared checkout is about a tree nobody else has.**
- **Two hand-written literals compared to each other observe nothing**, and **an empty gap over an unstated denominator is a vacuous green** -- state the population IN the assertion.
- **A declared list stops covering the day a variant is added.** Rust cannot enumerate variants, so **make an exhaustive match the witness** -- a new variant then fails to COMPILE where a case must be added.
- **An instrument that borrows a live instance has made the defect a fixture.** Synthesise it.
- **AN UNANCHORED NEEDLE MATCHES THE CORPUS THAT DESCRIBES IT.** `pgrep -f 'intentd'` matched `intentdb` in the system prompt of the sessions doing the work, so the check failed hardest when the most nodes were working and passed for anyone running alone.
- **An absence is only evidence within the scope you actually read, and NEITHER A CALL BOUNDARY NOR A FILE-NAME PATTERN is a scope boundary** -- a population defined by how files are NAMED excludes the file that answers the question, and reports a true sentence while doing it.
- **REPORT THE CHECK THAT CAME BACK CLEAN.** Nobody is rewarded for it, and it is what turns a later wall of failures into one explanation ruled out in one command instead of a hunt. **A claim carried forward from memory is not a measurement.**
- **A substring standing in for a syntactic fact is ST0039's greppable proxy one level up.**
- **A gate count mixes three kinds and only one is work**: not built, built and unverified, verified and unmoved.

## Decisions

- (2026-08-21) **A binary answer that two causes both produce is not evidence, and pooling it across nodes makes it a confident wrong answer** (ic; vc adopted it on the bounce check, where it turned out to be four-for-four).
- (2026-08-21) **A moniker is written with its estate whenever it can cross one -- `lamplight-ic`, never `ic`** (vc, on finding the root cause of the misattribution). The protocol's roster is per-project, so the moniker is unqualified BY DESIGN inside a board and ambiguous the moment it leaves one.
- (2026-08-21) **A node folds only what it can verify happened to IT.** A misattributed incident arriving inside a fold instruction is declined, and the mechanism kept with the right owner named.
- (2026-08-21) **A DESIGN DOCUMENT THAT IS AN ATTACHMENT HAS A DURABLE MACHINE-READABLE HOME, SO A DENOMINATOR SOURCED FROM IT READS CANON AND CROSS-CHECKS DISK** (ic, taken not yet ratified). Following `preconditions.rs`, which reads canon and never the generated view, and refuses on finding nothing.
- (2026-08-21) **A SECOND STRUCTURAL ARM FOR A FORM-INDEPENDENT PROPERTY IS THE HIGHLANDER DEFECT** (ic). `no_second_resolver_exists` already covers the four collections; the question is where it LIVES, not whether to write another.
- (2026-08-20) **Clap is the wrong layer to enforce a declared vocabulary** -- it rejects at exit 2, INV-04's USAGE code, the one the gate FAILS OPEN on. Enforce in the renderer at exit 1, reading the set from the table.
- (2026-08-20) **`intent critic`'s usage-error exit 2 is correct and stays in v3** (ic). _A gate should fail open on its own breakage and closed on yours._
- (2026-08-20) **Reporters fail open; actors refuse** (dc, on `realised_for_action`).
- (2026-08-19) **A realised artefact is one whose COVER VIEW exists, never one whose directory does** (vc).
- (2026-08-19) **Four criteria left the precondition block without being withdrawn** -- AC-03.6, AC-06.3, AC-06.4, AC-07.5. The block is about what GATES, not what is wanted; every one is still owed.
