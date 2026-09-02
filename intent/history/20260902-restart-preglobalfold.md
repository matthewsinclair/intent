# Intent -- traps and conventions

**Current as at `66e08a74`, 2026-08-30.** Current work is `intent/wip.md`; the entry point is `.claude/restart.md`; the DONE ledger is `intent/done.md`.

**THIS FILE CARRIES RULES, NOT HISTORY (hv, 2026-08-30).** Every completed-work narrative and every incident that produced a rule has moved to `intent/history/202608-restart-prefold.md`, verbatim. **What survives is what nothing else enforces** -- ic's criterion, and it is better than _is it done_: **a decision that survives only as prose is one nobody can fail**, so anything already encoded in a test, a guard or a generator has no prose home here.

## Where you are standing

**THIS CHECKOUT IS v3 AND IT IS WHAT THE FLEET RUNS.** Measure it rather than trusting this line:

    command -v intent && readlink "$(command -v intent)"
    intent --version
    intent info | sed -n 's/^ *INTENT_HOME: *//p'

**`~/Devel/prj/Intentv2` IS FROZEN AND IS NEVER WRITTEN**, and was never migrated. A census that finds projects by config presence cannot tell a consumer from the tool.

**A BUILD IS THE DELIVERY.** There is no install step: `bin/devbin build all` deletes both binaries to force the provenance embeds, and four symlinks point into that tree, so one rebuild heals all four.

**DURING A REBUILD EVERY ESTATE ON THIS MACHINE HAS NO `intent`, AND THAT BLOCKS COMMITS RATHER THAN WEAKENING THEM.** A commit invokes `intent` twice over -- `intent info` for the guard runner, `intent critic <lang>` per declared language. **In THIS tree the guard half survives and the critic half refuses outright**; in a consuming estate neither runs. **Both statements are true and they are different.** One node rebuilds and announces the opening AND the end -- **the end is the half that gets forgotten, and a holder waiting on a stale announcement is invisible to everyone including themselves.**

**THE PAIR CAN BE SPLIT, AND EVERY CHECK THAT READS ONLY `intent` PASSES ON IT.** The property is that BOTH halves name the same commit; `bin/devbin build all` verifies them as a set, which is why a bare `cargo build --release` is not the door. **`build-support/source_commit.rs` omits `rerun-if-changed` deliberately** -- emitting any would REPLACE cargo's default of re-running on package change and make the embed stale on CODE changes, silently, in the worse direction. **Nobody fixes it and nobody reaches for the bare command.**

**THE MARKER IS PROVENANCE, NOT AN IDENTITY.** Never compare it to `HEAD` -- that differs after every board commit and says the alarming thing on the healthy case. **The deciding test is `git diff --name-only <marker>..HEAD -- native/rust surface`, empty meaning current.** The sha256 distinguishes one build from another.

## Open protocol questions

**hv's BOUNCE QUESTION IS OPEN and the test written for it cannot answer it.** The rule was that a fresh session's `$CLAUDE_CODE_SESSION_ID` differs from the one on its board -- **but `/compact` does NOT rotate it**, so the test reads a compacted session as a resumed one. **A discriminator that fails only in the direction you expected is worse than one that fails both ways, because nobody re-checks the reading they anticipated.**

**DO NOT USE `ListAgents`' `started` COLUMN -- it is SOCKET age, not SESSION age.** All four nodes read it, all four reported _three of four bounced, but not me_, and all four were wrong. **Consensus is not corroboration when every node used the same broken instrument.**

**MONIKERS ARE ESTATE-SCOPED AND NOTHING MARKS THEM.** `ic`, `cc` and `vc` exist in Intent, Lamplight, Laksa and Prolix simultaneously; `/in-whiteboard` defines the roster per-project and is silent about crossing one. **Write the qualifier every time a moniker crosses an estate: `lamplight-ic`, never `ic`.** A fold is where an unchallenged claim becomes permanent.

## Live and unfixed -- read before driving this repo

- **`intent edit` AND `intent st edit` WRITE ON THEIR rc=1 REFUSAL PATH.** `0144` closed the unknown-id half; **the known-thread refusal still writes** -- `st edit ST0001 design` against a real thread with no realised file returns rc=1 and grows TRACKED `intent/.intentfiles`. Filed as `intent#0145`, **and its remedy is vacuous on that path** (`remedy: this artefact carries: ` with nothing after the colon), which belongs with the vacuous-`doctor`-remedy item as ONE class.
- **U3: four verbs are mandated in canon and unimplemented in v3** -- `lang`, `plugin`, `ext`, `version` -- **all dispositioned `keep`, so UNBUILT rather than retired**, each saying so at rc=2. **DO NOT EDIT THE CANON FOR THOSE FOUR: every one works in v2, so the mandates are correct for 16 of 17 projects. The canon is not defective, it is not VERSION-AWARE.**
- **doctor's GATE/CARRIER ROWS ARE FALSE ON EVERY SHIM ESTATE** (0105, 0106, 0113): a reporter reads the CARRIER and expects the GATE BODY's properties, so the alarm is permanent. **Do not act on those rows.** `bash .githooks/pre-commit.intent --where` is the one-second truth, `guards: N ran` proves reach, and only a refused commit proves bite.
- **`claude upgrade` AND prettier ARE TWO WRITERS OF `AGENTS.md` THAT CANNOT CONVERGE** (0110): apply 6211 -> prettier 6208 -> apply 6211, forever. **The pair rebuild it was waiting on has happened, so the closure is RE-CHECKABLE and has not been re-checked** -- do not read this as still-blocked without driving it. Until then, treat apply's `written: AGENTS.md` as unread and check `git diff --stat`.
- **0111's SYMPTOM IS REAL AND ITS CAUSE IS CONTESTED.** `sync --to-store` compares JSON extract to store and fires on a real divergence. **Read the store text back after any repair-motivated `--to-store`.**
- **SIX SHELL TESTS ARE RED AND HAVE BEEN FOR THREE TO FOUR DAYS**, none of them caused by the v3 work -- see `wip.md`. **The finding is the four days, not the six.**

## Measuring anything here

**`int suite` RUNS THE SUITE IN `prepush`'s SINGLE-WRITER CLONE AND IS ATTRIBUTABLE BY CONSTRUCTION.** It measures HEAD, not the working tree. **WHILE ANYTHING IS UNCOMMITTED, NO SUITE FIGURE CAN NAME A REVISION** -- commit first, then measure.

**OUTPUT TO A FILE, THEN COUNT. `grep -c '^test .* FAILED$'` over a complete file can be zero honestly; a `tail` cannot.** (`2>&1 > file` sends stderr to the TERMINAL -- `> file 2>&1` is the form that captures both.) **A partial read that finds SOMETHING self-corrects; one that finds NOTHING is unfalsifiable without redoing the read**, and a positive survives truncation, which is why the two do not feel different at the time. **A negative from a partial read is not a result.**

**A DETACHED WORKTREE GETS ITS OWN `CARGO_TARGET_DIR`.** `env!("CARGO_MANIFEST_DIR")` is baked at COMPILE time, so a build from a worktree that is later deleted leaves rlibs walking from a directory that is gone -- and the failure surfaces in somebody else's run, pointing at an innocent file. **A stale artefact is not a regression: rebuild before diagnosing.**

**RUN THE BATS SUITE THROUGH `tests/run_tests.sh`.** A direct single-file `bats` run is also safe -- the runner and the helper both read `INTENT_FIXTURE_VERSION` from `VERSION`, so they agree by construction.

## Traps that cost real time

- **AN INSTRUMENT ANSWERS A DIFFERENT QUESTION THAN THE ONE ASKED, AND ITS OUTPUT LOOKS LIKE AN ANSWER.** The dominant class here by a distance. **Ask it to find something you KNOW is there before believing it when it finds nothing.** Forms: a prefix match resolving to a shorter valid prefix; a substring of a different word; **an enumerator structurally unable to see its own population** -- no grep can see a call site taking a VARIABLE, and one set gave 15, 17 and 24 with the COMPILER the only complete answer; a hand-written control failing for an unrelated reason; a precondition that silently did not hold; **a mutation harness that never asserted the mutation APPLIED**; and **over-kill, where a mutant kills MORE than predicted because it broke the fixtures' premises rather than the property.**
- **A ZERO FROM YOUR OWN INSTRUMENT IS A CLAIM ABOUT THE INSTRUMENT.** Four failure sites: it cannot reach the subject; it matches real strings that are not the thing; its input is not what you think; **or the query is right and returns MORE rows than you assumed, where the COUNT is the finding.** **A query against the wrong FIELD answers with an ABSENCE, and an absence is the one result that never looks like a bug in the query.**
- **THE POPULATION IS THE CLAIM.** A fix, a figure, a roster and a revert list each name a SET, and the set is the part nobody checks. **A remedy asserting completeness is a population claim about the failure modes it covers.**
- **THE GATE, AND ANY N-OF-M, IS COMPUTED BY A VERB.** Hand-tallying produced three wrong numbers in three days. **Highlander applies to a figure in prose exactly as it applies to code.**
- **A TRUE MEASUREMENT OF A DIFFERENT PROPERTY, OFFERED AS PROOF, IS THE HARDEST TO SEE** -- the evidence being real is what makes it persuasive. Correctness and currency are independent and only one is ever checked. **A precondition that holds today is not a property.** Most dangerous wearing rigour.
- **SILENCE AND SUCCESS ARE IDENTICAL UNLESS SOMETHING DISTINGUISHES THEM.** **A gate that cannot say _I could not check_ will eventually say something false instead** -- the tell is a remedy that runs clean and changes nothing. **And a gate that blocks and gives no reason is worse than the bug it was added to report.**
- **THE FAILURE PATH IS THE ONE THAT MUST STILL WORK, AND IT IS THE ONE A GREEN RUN NEVER EXERCISES.** Cleanup written after the assertions is dead code until the day it matters. **No-answer is indistinguishable from still-working** -- an orphaned child holding a test binary's stdout pipe turns a failing test into a hung build naming no test. **An unconditional cleanup can delete a SUCCESSOR's claim**, inverting the failure it prevents.
- **A CI RUN'S SUBJECT IS THE PUSH, NEVER YOUR COMMIT.** Read `git log <lastpush>..HEAD` before attributing a run to anybody.
- **A CHANGE THAT WOULD CONVENIENTLY GREEN YOUR OWN WORK IS THE ONE TO STOP AND ROUTE.** The tell, not the virtue.
- **THE REVISION IS PART OF THE FINDING.** Name revision, clock and dirty count on every measurement. **ZERO FAILURES ACROSS A WORKSPACE DOES NOT PROVE A BINARY RAN** -- confirm each subject appears in the `Running` list.
- **THE SHARED OBJECTS ARE THE INDEX, CANON, AND THE GUARD SCRIPTS THEMSELVES.** `git commit --only <paths>` bounds what is COMMITTED and bounds NOTHING about the GATE, **and it cannot reach an untracked file at all.** **It also builds its tree from HEAD plus the named paths, so a peer's STAGED fix is excluded from every other node's commit** -- the protocol that stops us sweeping each other is what makes one node's in-flight fix inescapable for all. **Staging a guard's own body is a PROJECT-WIDE ACT and nothing about `git add` says so.**
- **`sync` HAS NO UNIT NARROWER THAN A THREAD**, so it welds a peer's bytes into your commit under your signature. **An authorship hazard produces no error ever**, so stopping on one cannot be prompted by a tool. **`sync --to-store` IS DISK-AUTHORITATIVE FOR ATTACHMENTS** -- a canon-only edit to a realised attachment is discarded in silence at rc=0; **for a typed field CANON wins.** Edit canon, `--to-store`, THEN lint, and **check the sync's rc, not its tail.** **Between the store write and the extract write the estate genuinely disagrees with itself.**
- **A FILE WHOSE INTERMEDIATE STATES ARE UNINHABITABLE FOR EVERY PEER NEEDS THE SHORTEST POSSIBLE EDIT WINDOW AND AN ANNOUNCEMENT AT BOTH ENDS.** The dispatch table is compiled IN, so a malformed one is not a bug a user could hit -- **it is a binary that will not start, for everyone building from the shared worktree.**
- **`intent st list` defaults to in-progress and returns 2; `--all` is NOT a flag** -- use `st list --status all`. **But `intent issues list` uses `--kind all`, and the two verbs MIS-TEACH EACH OTHER**: `issues list --status all` exits 1 with EMPTY STDOUT, which reads as a true zero.
- **A ` M` IN `git status` IS A CLAIM ABOUT THE INDEX, NOT ABOUT CONTENT.** A file can present as modified through a stale stat entry with zero changed bytes. `git diff --stat` separates them.
- **A QUOTATION IS TESTIMONY ABOUT A DOCUMENT, NOT THE DOCUMENT.** Quotations are the commoner case BECAUSE THEY LOOK LIKE EVIDENCE. **THE ENVELOPE BEATS THE BYLINE** -- an incident and its generalisation are separable and usually have different authors, so NAME WHICH HALF.
- **A TERMINATION CONDITION IS A FILTER NOBODY DECLARED, AND THE STRICTEST INSTRUMENT REPORTS THE SMALLEST POPULATION.** `cargo clippy -- -D warnings` reported 113 where the truth was 116, because compilation ABORTS PER CRATE. **The flag that makes a run authoritative is the flag that shrinks its denominator, and nothing in the output says the denominator moved.**
- **A RECORDED REASON IS RETIRED BY AN UNRELATED CHANGE, AND NOTHING WATCHES THE JOIN.** Every instance surfaced because a builder picked the reason up in order to USE it. **An answer surviving a change of premise is not the same answer.**
- **A CLAIM OUTLIVES ITS BASIS AND THE MORE LOAD-BEARING IT IS, THE LONGER IT SURVIVES**, because everyone builds on it rather than checking it. **A board decays fastest exactly where the work moved fastest** -- the opposite of where attention goes. **And a rule fails most often in the artefact that states it: reading a warning and applying it are different acts.**
- **A CORRECT REFUSAL IS NOT A SAVE.** A refused `--to-store` leaves the bytes only on disk, so the next `--to-disk` destroys them at rc=0. **The refusal GUARANTEES the loss and arrives wearing the costume of a save.**
- **A REMEDY IS A PROMISE, AND AN UNKEEPABLE ONE IS WORSE THAN A BARE REFUSAL** -- the operator does the one thing they were told to do and is refused for doing it.
- **ORDER IS FORMAT, THEN SYNC, THEN COMMIT.** **A gate may refuse; it must not author** -- the formatter arms check and refuse rather than rewriting and re-staging, so the staged tree and the committed tree are the same object.
- **`FIXED` IS NOT A STATE -- worktree, index, HEAD and pushed are FOUR.** A peer saying _I fixed it_ reports the first while the reader hears the third. **Ask which.**
- **FIVE SESSIONS SHARE ONE WORKING TREE, SO TWO ORDINARY GIT INSTRUCTIONS HAVE NO REFERENT HERE.** _Pull before you read this_ is meaningless, and so is _I will push only my own commits_. **Both fail the same way: an instruction written for many clones, applied to one tree**, and both are correct in the topology they were written for, so neither reads as wrong. **The tell is a POSSESSIVE or a SYNC VERB attached to something the whole estate shares.** **The defect is the possessive, not the obligation** -- a node bound to confirm with hv before pushing is still bound.
- **MECHANICAL.** `--no-fail-fast` always. **Never `$?` after a pipe** -- zsh has `pipestatus`, not `PIPESTATUS`. **An unquoted `$var` does NOT word-split here.** **An apostrophe or a backtick inside a single-quoted string is a hard syntax error** -- run `bash -n` on any edited shell file, and DRIVE it, because a syntax check is not an execution. **`grep` is ugrep here.**
- **FOUR SHELL CRITIC FINDINGS ARE DELIBERATELY NOT FIXED AND MUST NOT BE.** `bin/intent_st:1187`/`:1208` and `bin/intent_treeindex:220` are **intentional word-splitting**; `bin/intent_st:1353` is a fragment of a multi-line `sed` script. **A sweep driven to zero without reading each site breaks three live paths.**

## Design rules

- **MAKE THE BAD STATE UNREPRESENTABLE RATHER THAN CHECKED FOR.** Bind-and-publish as ONE call makes publishing an address nobody is listening on unexpressible -- a category retired before any test exists.
- **WHERE A PROPERTY BELONGS TO A SYSCALL OR A DEPENDENCY DEFAULT, THE OUTCOME HOLDS UNDER ANY IMPLEMENTATION THAT STILL CALLS IT, SO WHAT IS TESTABLE IS THAT YOU STILL CALL IT.**
- **A PREDICATE IS NOT SOUND IN ITSELF -- IT IS SOUND RELATIVE TO WHAT IS DONE WITH THE ANSWER.** One liveness probe fed routing (a false negative costs a redundant run) and eviction (the same false negative unlinks a live daemon's socket).
- **HIGHLANDER GOVERNS IMPLEMENTATIONS, NOT WITNESSES.** Consolidating witnesses is DELETING THE MEASUREMENT: a test that imports the value it asserts has stopped testing. **The discriminator is whether the thing is the SUBJECT of that file or INCIDENTAL to it.** **And derive over test wherever possible -- a test compares two authored values and can only fire after somebody writes the second one.**
- **CHANGING A PUBLISHED FIELD'S MEANING WITHOUT CHANGING ITS SHAPE IS THE WORST VERSION OF THAT CHANGE**: every consumer keeps parsing and the correct ones become silently wrong. **A name that is wrong in a way that WORKS is worse than one that is obviously wrong.**
- **A CRITERION IS OWNED BY WHOEVER CAN SATISFY IT, AND MUST BE ABLE TO FAIL.** A row with two owners has none. Unfalsifiable forms: an unbounded set; a checklist of names; a count of a kind the instrument does not count at all; a criterion restating its source instead of citing it.
- **REMOVING A SWALLOW MEANS FINDING OUT WHAT IT WAS SWALLOWING, AND THERE IS NO WAY TO FIND OUT EXCEPT TO REMOVE IT.** A swallow that has survived is usually load-bearing for something, and that something is invisible BECAUSE it was swallowed.
- **A RULE THAT CATCHES A FAILURE MODE IT WAS NOT DESIGNED FOR IS THE STRONGEST EVIDENCE FOR IT**, and so is a decision that reaches a question it was not about. Neither can be designed for.
- **THE CONSOLIDATION IS THE WORK RATHER THAN THE SHORTENING.** A shape cannot be seen while its instances are filed separately, so **folding is a measurement instrument, not tidying.** And **a duplicate number is invisible to anyone arriving to look up a rule rather than to audit the list** -- check the numbering.

## The clock, and it has THREE generators with three different remedies

**Every stamp is READ FROM A CLOCK: `date -u +'%Y-%m-%d %H:%MZ'`.** A stamp you did not read is fabricated data, not an approximation. `git log` and `stat` both print LOCAL -- reading one and appending a `Z` produces a stamp wrong by exactly the local offset and looking perfect.

The commit guard catches a **future** stamp, a **missing `Z`**, and an **inbox going backwards**. It does not close the class, and the three generators are not the same defect:

1. **ARITHMETIC** -- read the clock ONCE, then advance by feel. **Monotonic BY CONSTRUCTION, so it satisfies the backwards check perfectly.** Remedy: **read per stamp, never per session.**
2. **FABRICATION WITH THE CORRECT VALUE PRESENT** -- typing a plausible number while a correct one sits four lines above. **No read defeats it, because the read already happened.** Remedy: **never type the value -- substitute `$(date -u ...)` into the edit so the hand is out of the path.** **This generator fired twice in one session in 2026-08-30, by a node that had read this paragraph**, and the second firing was inside the commit recording the first. **A discipline that fails twice in a session is one being asked to do a tool's job.**
3. **A STALE REFERENCE, WHICH ONLY EVER ACCUSES THE OTHER PARTY.** Judging a peer's stamp against your own last read makes their correct stamp indistinguishable from a fabricated one. **Judging a stamp IS a stamp-sized act and owes its own read.**

**THE LIVE CHANNEL HAS NO DOOR AT ALL** -- all three checks run at commit. **Attribute a peer's live stamp; never assert it.**

## Conventions

T-shirt sizing only. **ALWAYS use the intent CLI for ST/WP.** NEVER manually wrap markdown. **NO Claude attribution in commits**; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. **Commit to `main` only when matts asks; always `git commit --only <paths>`.** matts runs the full suite externally and is the acceptance verifier. **NEVER `--no-confirm` on the release.** Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/int build release` date them -- **and a defect introduced and fixed inside one unreleased cycle gets NO entry, because there is no reader for it.**

**EACH PROJECT-WIDE DOCUMENT GETS ONE JOB: `.claude/restart.md` is the entry point, `intent/wip.md` is DOING and TODO, `intent/done.md` is the DONE ledger, `intent/history/` is the archive, and this file is traps and conventions.** **If you find yourself writing a supersedes banner, DELETE WHAT IT SUPERSEDES INSTEAD** -- three copies of one narrative is how the gate figure came to have three homes carrying three values.

**THE PUSH GATE NO LONGER RUNS `fmt` OR `clippy`, AND CI IS THE ONLY HOME FOR THEM.** `prepush` still clones, builds and runs the pair. **Do not re-add the arms without re-answering the cadence question.**

**EM DASH IN PROSE PAGES; `--` IN GENERATED REFERENCE PAGES.** The generated pages render CLI text a reader copies, so ASCII throughout means a paste out of them is always safe.

**A SURFACE CLAIM TRAVELS WITH WHAT MAKES IT CHECKABLE -- AND FOR A VERB, THE REGISTER ANSWERS THAT ONLY WHEN THE VERB IS NEW.** The register answers _does this exist_; it cannot answer _does it still do what the last document said_. **For a behaviour change the check is the source or the test**, and the generated reference regenerates identically, reading as current.
