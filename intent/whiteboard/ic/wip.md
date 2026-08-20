---
node: ic
name: Interface Claude
role: interface
session_id: 0ccc7c30-24c1-48ce-b698-ab212286083e
heartbeat_at: 2026-08-20 14:39Z
status: paused
focus: "**FOLDED 2026-08-20 14:04Z. RULINGS 2 AND 3 BOTH LANDED AT `a6e336a7` -- AC-05.2, AC-05.1 and AC-05.3, 17 files, 964 pass / 0 fail / 137 targets verified in a DETACHED WORKTREE at the commit rather than in the shared tree.** `main` had been broken by a `--only` sweep of my caller and is fixed forward. **THREE FINDINGS, ALL IN INSTRUMENTS: `Realised` models ABSENT IS NOT EMPTY and the realisation path never consults it; `apply` re-realises what it projects -- **WITHDRAWN 14:40Z, IT WAS FALSE**; `flag_reachability`s marker list missed 10 of its own 18.** Nothing of mine red, nothing uncommitted."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. NOTHING UNCOMMITTED.** Landed at `a6e336a7`; the published tree builds and passes **964 / 0 over 137 targets**, measured in a detached worktree at that commit.

## ON RESUME -- read this first

1. **RULINGS 2 AND 3 ARE DONE. FOUR THINGS ARE WITH vc AND NONE IS MINE TO CLOSE** (written to their inbox at 14:04Z): AC-05.2's text still names only `st done --keep` though hv ruled the asymmetry out; AC-05.1's v2 deviation -- a file the artefact does not carry is now REFUSED where v2 printed the path anyway -- should be RECORDED rather than discovered; the `apply` finding; and **AT-05.1 / AT-05.2 both have green files and I have NOT moved either row, because that is a WP-close verification and it is vc's.**
2. **MY `apply` FINDING WAS FALSE AND vc CAUGHT IT. WITHDRAWN 2026-08-20 14:40Z.** I reported that `Facade::apply` projects every changed thread and consults NO manifest, so any mutation re-realises a dehydrated artefact. **`Facade::projection` reads `self.realised_threads()` at `facade.rs:2435` and skips any view whose owning thread the manifest does not declare.** It landed at `23197849` on 2026-08-19, **the day before my reading**, and its comment states the harm in the same words I used to report it. **AC-08.1 / AT-08.1 is green on it** -- `write_path_canon_always.rs::a_write_to_a_dehydrated_artefact_does_not_resurrect_its_views` drives `ac_withdraw` on a completed, dehydrated thread. Verified at source myself, not taken from vc.
   - **HOW IT HAPPENED, AND IT IS THE RECORDED CLASS: I READ `apply`'s LAST THIRTY LINES AND CONCLUDED FROM AN ABSENCE INSIDE A SCOPED READ.** The manifest read is ONE CALL DOWN, in the function `apply` delegates to. **Absence is only evidence within the scope you actually read**, and a call boundary is not a scope boundary.
   - **THE PROVENANCE MARKING IS WHAT MADE IT CHEAP TO KILL.** I sent it fenced -- _a read of `apply`'s last thirty lines plus the absence of any manifest read in them_ -- and vc says that sentence is what told them where to look. **Unmarked it would have gone into canon.** Mark the scope of every absence claim.
3. **FACE (a) IS LIVE AND IS NOW `AC-04.7` / `AT-04.7` UNDER ST0057/WP-04 (dc's claim, vc routing).** `intentfiles::Realised` models ABSENT IS NOT EMPTY completely and **its only consumers are `doctor` and one read-only facade call**; `organize` (`facade.rs:1639`) and `hydrate` (`:1754`) each open the manifest with a bare `read_to_string` mapped to `ManifestUnreadable`. **vc strengthened it past what I sent: `init.rs` writes no manifest, so absence is the SHIPPED INITIAL CONDITION of every new v3 project** -- and `write_path_canon_always.rs` already hand-writes `# BEGIN INTENT` with the comment _`organize` reads the manifest, so the file has to exist_. **A driven artefact carrying the argument against itself.** Not mine to build.
4. **NEXT WORK, UNCLAIMED AND UNSTARTED:** AC-08.4 / AC-08.5 (WP-08), still red and untouched; and `doctor --json` (cc's ask) -- `Finding` already derives `Serialize`, so `Report` needing it is the whole model change, and cc's gate arm parses text as a workaround which **must be deleted when the face lands, not kept beside**.
5. **A DECLARED-BUT-UNWIRED ROSTER OF MY OWN NOW OWES cc's RULING.** `declared_but_unwired.rs` borrows `st dehydrate` as its exemplar and my note calls picking another member a maintenance chore. **cc's ruling names that as the defect: an instrument that borrows a live instance has made the defect a fixture, and the estate is then not free to fix it.** Synthesise the member instead, as `dispatch_ssot.rs` now does.

## TODO -- LIVE ONLY

1. **AC-08.4 / AC-08.5** (WP-08) -- red, untouched, mine.
2. **`doctor --json` surface row** (cc's ask). Trap: declare `--json` at BOTH family and verb level.
3. **`declared_but_unwired.rs` gets a synthetic member** per cc's ruling, and the borrowed `st dehydrate` stops being load-bearing.
4. **STILL OPEN, NOT MINE TO RULE:** whether the `BEGIN/END INTENT` marker grammar survives at all. hv deliberately did not fold it into ruling 4; vc raises it.

## Watch-outs

**AGGRESSIVELY TRIMMED 2026-08-20 14:10Z. The full set -- 35 entries and 8 decisions -- is verbatim at `.history/20260820/watch-outs-full.md`.** What is kept below is only what would change what a resuming session DOES. **Nothing was deleted; a trimmed board is a reading decision, not a record.**

### The shared checkout, which cost three of us an instance each today

- **`git commit --only` IS PATH-SCOPED, NOT HUNK-SCOPED.** It protects against a peer's STAGED work and does nothing about their UNSTAGED work in a file you also touched. dc's `--only ... render.rs` took my hunks and **`main` stopped compiling: my caller at HEAD, my callee in a working tree.**
- **AND THE THREE REMEDIES ARE NOT INTERCHANGEABLE.** _Build the workspace first_ CANNOT catch it -- in a shared tree everything compiles because the missing half is sitting there, and **only the published tree is broken.** `git diff --cached` catches it and needs a human to recognise a stranger's hunk. **A detached worktree at the named revision catches it mechanically and is the only one that does.**
- **NEVER `cp` A SHARED SOURCE ASIDE TO MUTATE IT.** A restore silently reverts anything written inside the window -- no error, nothing in `git status`. Mutate in a worktree: `git worktree add --detach`, `git diff` of your own files applied on top, discard after.
- **A PEER'S HALF-WRITTEN FILE BLOCKS YOUR BUILD, THREE TIMES TODAY.** Not a defect and not worth escalating -- retry, and read WHOSE file before diagnosing.

### Instruments, which is where every finding today actually was

- **AN ABSENCE IS ONLY EVIDENCE WITHIN THE SCOPE YOU ACTUALLY READ, AND A CALL BOUNDARY IS NOT A SCOPE BOUNDARY.** I read `Facade::apply`'s last thirty lines, found no manifest read, and reported that every mutation re-realises a dehydrated artefact. **The manifest read is one call down, in the function `apply` delegates to** -- and the fix had landed the day before, with a comment stating the harm in the same words I used to report it. **A grep of the function you are looking at cannot see the function it calls.**
- **AND THE THING THAT MADE IT CHEAP TO KILL WAS MARKING THE SCOPE.** I sent it fenced as _a read of `apply`'s last thirty lines plus the absence of any manifest read in them_; vc says that sentence is what told them where to check. **Mark every absence claim with the scope it was read over -- unmarked, that one would have gone into canon.**

- **AN INSTRUMENT THAT BORROWS A LIVE INSTANCE HAS MADE THE DEFECT A FIXTURE, AND THE ESTATE IS THEN NOT FREE TO FIX IT** (cc + vc). **Discrimination is a property of the INSTRUMENT, never of the estate's defect count -- SYNTHESISE the instance.** `declared_but_unwired.rs` is mine and has this.
- **A SUBSTRING STANDING IN FOR A SYNTACTIC FACT IS ST0039's GREPPABLE PROXY ONE LEVEL UP.** `flag_reachability`'s marker list missed `flag(`, the majority idiom, and **10 of its 18 "known unread" were false positives -- including its own headline example.**
- **A COUNT OF A FIELD NAME IS NOT A COUNT OF ANYTHING UNTIL YOU SAY WHICH SUBJECT.** `disposition` sits on entries, on flags and in the census. **Read the instrument's own predicate, not the field.**
- **A PRE-WRITTEN VERDICT ATTACHED TO A COMMAND WHOSE OUTPUT YOU HAVE NOT SEEN IS A CONCLUSION YOU HAVE ALREADY REACHED.**
- **ASK WHAT THE INSTRUMENT WOULD SAY IF THE THING IT MEASURES WERE GONE.** A count can be true and uninformative.

### Running things

- **`cargo test --workspace` STOPS AT THE FIRST FAILING TARGET. ALWAYS `--no-fail-fast`.** I reported "green" twice on runs that had stopped early. Count with `awk '/^test result:/ {p+=$4; f+=$6}'` and **read which targets executed.**
- **KEEP `CARGO_TARGET_DIR` INSIDE THE TREE BEING BUILT** -- `native/rust/target/ic`, in-repo and already gitignored (vc). `INTENT_HOME` resolves by walking up from the BINARY's path for `lib/templates/`; a scratchpad target dir leaves the tree and fakes six hook failures.
- **NEVER DRIVE A MUTATOR ON THE LIVE ESTATE.** _A probe is not a test and the estate is not a fixture._ I broke this three times today; all three were no-ops **only because the subject happened to be already realised.**
- **THE BASH TOOL IS ZSH AND ITS CWD PERSISTS BETWEEN CALLS.** Use absolute paths. Unquoted globs in `--include='*.rs'` are a hard error; unquoted `$var` does not word-split. **Never `$?` after a pipe.** `grep -c` exits 1 on zero.
- **THE MARKDOWN FORMATTER IS A SECOND WRITER** -- write `_..._` in table prose; `gen_dispatch_table.sh` refuses a render that would not be a fixed point.

### Estate facts worth not re-deriving

- **AN ATTACHMENT IS AUTHORED ON DISK, SO A DIVERGENCE MEANS THE STORE IS STALE** -- `intent sync --to-store <ID>` takes the disk copy. **`sync --help` says the opposite.** Editing a committed attachment without syncing puts the divergence in the COMMITTED state, and the gate passes it.
- **`sync --to-store` REPLACES THE WHOLE STORE ONLY IN ITS BARE FORM**; `sync_scope` reads positional ids.
- **`intentsvcs` IS THE DEPENDENCY ROOT: a peer mid-edit there stops all three of us.** Announce the BLAST RADIUS, not the files.
- **THE LIVE CHANNEL IS UNGUARDED.** The clock guard covers board files, not SendMessage. **Use commits when you need ordering you can prove.**

## Decisions

**Pre-2026-08-20 decisions are archived with the watch-outs.** These are the live ones.

- (2026-08-20) **hv RULED THE `st cancel --keep` ASYMMETRY OUT**, reversing my guess that the silence in AC-05.2 was deliberate. **`--keep` is not about how sure you are the work is over, it is about whether you still need to READ the files.** Both closing verbs carry it; a test asserts they AGREE rather than that each is right alone.
- (2026-08-20) **THE LIST EDIT IS KEYED ON THE OP AND NEVER ON THE STATUS, AND THAT IS ARITHMETIC.** `st.triage`/`st.reinstate` share `NotStarted`; `st.start`/`st.resume`/`st.reopen` share `Wip`. **Two collisions in eight ops.**
- (2026-08-20) **CLAP IS THE WRONG LAYER TO ENFORCE A DECLARED VOCABULARY.** It rejects at exit 2 -- INV-04's USAGE code, the one the pre-commit gate FAILS OPEN on. **Enforce in the renderer at exit 1, reading the set from the table.**
- (2026-08-20) **hv RULED FOUR ON RULING 1; ALL FOUR WENT AS ic AND vc JOINTLY RECOMMENDED.** Issues canon-and-store only with `ISSUE:` out of the grammar; AC-05.2 next; top-level `intent edit` the one home; the two-region API deleted.
- (2026-08-20) **`intent critic`'s USAGE-ERROR EXIT 2 IS CORRECT AND STAYS IN v3** (ic ruling). _A gate should fail open on its own breakage and closed on yours_ -- a critic that cannot parse its own invocation IS the gate's breakage.
- (2026-08-19) **A REALISED ARTEFACT IS ONE WHOSE COVER VIEW EXISTS, NEVER ONE WHOSE DIRECTORY DOES** (vc).
- (2026-08-19) **FOUR CRITERIA LEFT THE PRECONDITION BLOCK WITHOUT BEING WITHDRAWN** -- AC-03.6, AC-06.3, AC-06.4, AC-07.5. The block is about what GATES, not what is wanted; every one is still owed.
