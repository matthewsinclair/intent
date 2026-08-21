---
node: cc
name: Control Claude
role: control
session_id: ef9e17d5-a705-4547-b749-807a30ba16b5
heartbeat_at: 2026-08-21 09:29Z
status: active
focus: "**PICKUP 2026-08-21. Nothing in flight, nothing owed, all four inboxes at sentinel, tree clean at `706db8ee`.** Opening on the three gate rows that are mine -- ST0057 AC-01.5, ST0057 AC-03.6, ST0056 AC-03.14 -- plus AC-10.8, whose first job is not a test. Plan tabled to matts before any build."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

**All of 2026-08-20 is in `.history/20260820/`. This is only what a cold session needs.**

## The model -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** The old "committed JSON durable / DB rebuildable" wording is VOID. **D34: the committed extract is the interchange -- it TRAVELS, the DB never does.** D29: a gitignored path is never canon. D54: 33 + 3 + 23 = 59.

**D42 -- TIME.** `date -u +'%Y-%m-%d %H:%MZ'`, own step, trailing `Z`. I fabricated four stamps in one day while carrying this in bold. **The mechanism, since care is not one: the stamp enters the message from the same shell command that reads it.** **AND THE GENERATOR IS ARITHMETIC, NOT MEMORY** -- vc's disclosure, 2026-08-20: _`17:36Z`, `17:39Z`, `17:44Z`, `17:46Z`: the first was read and the rest were arithmetic._ **A drifted stamp does not start from nothing; it starts from a TRUE reading and is advanced by feel, so it wears the authority of the real one.** **This is the sequence that defeats check C**, the two-sided test that needs no clock: increments-by-feel are monotonic BY CONSTRUCTION, so every stamp in a drifting run passes the one check designed to work when the clock is unavailable. **A second clock read is the only thing that catches it, which is why the read is per-stamp and never per-session.**

## Mine

**STATE `f31aa924`. WP-10 green and mutation-proved:** AT-10.14 `2244926a`, AT-10.2 `28b3610b`, AT-10.4 `4126b2f7`, AT-10.15 `bc522897`, AT-10.3 `98ef78f2`+`783c2e79`, AT-10.16 `f9b318a4`, AT-10.17 `f31aa924`. **Moves are vc's; I have written no canon.**

**NEXT IS `egest_estate.rs` (AC-10.8) AND ITS FIRST JOB IS NOT A TEST:** read `data-model.md`'s enumeration, prove the out-of-model population NON-EMPTY, **and then read the enumerating clause itself as a hypothesis** -- non-empty is necessary and is not sufficient, because a population can be non-empty, exhaustively measured, and still exclude the answer by its own definition (vc, below). Not started under a deadline on purpose -- that control limb is the first thing time pressure removes.

**AC-10.5 wants matts' call** on Lamplight/Utilz/Baize revisions. **AC-10.6 is `n-a` and wants DOING -- it is the verb that zeroed the event log. AT-10.12 stays HELD** on an unexplained trim asymmetry (detail in canon and with vc since 2026-08-19): building before it is explained encodes a guess inside a green.

**NOT MINE:** hooks, roster, `int hooks`, `canon_commit_check.sh`'s admission, view-skew wiring = **dc**. `Sigil`, issue verbs = **ic**. Contract, WP-close = **vc**.

## The day's finding -- four shapes of one thing

**A ROW CAN PROMISE MORE THAN IT DELIVERS.**

1. **UNCITED COVERAGE** -- already satisfied by a test nothing links to it; five instances across three nodes in one day. No AC id appears in a covering test, so nothing greps it. **PARTIAL uncited coverage is worse: a subject grep that hits the file says nothing about WHICH limbs.**
2. **EXPIRED CITATION** -- the row names a file that cannot cover it. `to-write` is exempt from L2/L3, correctly, so a citation is unvalidated until someone tries to satisfy it.
3. **VACUOUS GREEN** -- true by construction, so the test cannot fail. **The falsifiable arm is over the DECISION, never the OUTCOME: the outcome is quiet precisely because nothing acts; the PLAN is where an act is declared.**
4. **TITLE BROADER THAN BODY** (vc) -- **and this one leaves no trace at all.** AC-10.4 is titled _Hooks continuity_; its body enumerates only `.claude/**`, so `.githooks/**`, the entire commit gate, was asserted by nothing. **The row is internally consistent: lint passes, the citation is right, the test is green, and it reads as covered to anyone who does not open it.**

**PRACTICE:** subject-grep FIRST because it is cheap, **then DRIVE THE VERB when it comes back empty** -- the coverage announces itself and you never guess a filename. A subject grep is still a name grep: vc swept four `fn` spellings for an allocator, got zero, wrote _there is no allocation function of any kind_ into canon; it was `fn next_thread_id`, **a private method invisible to a free-function sweep at ANY name.**

**AND vc's DISCRIMINATOR -- _what does satisfying this row COMPLETELY still leave broken?_ -- IS ASKED AGAINST THE BODY, NEVER THE TITLE.** Against a title it returns _nothing_ every time, for exactly the rows where it matters most, **and that answer looks identical to a correct one.**

## Watch-outs -- evidence

- **THE PROVENANCE RULE, THREE LIMBS** (vc): **neither the INSTANCE, nor the CONTROL, nor the PREDICATE may come from the thing under test.** The predicate limb arrives as the smallest possible edit and looks like a typo fix -- I typed `9`, the scanner said `10`. **Derive the expectation from the fixture's own bytes.** Same limb dressed as a dependency: `uuid::parse_str` validating `uuid::new_v4` passes by construction, including for a library that minted a constant.
- **A GREEN MEANS NOTHING UNTIL EACH TEST HAS DIED FOR ITS OWN REASON** -- distinct kill-sets, not "each can fail". **Assert the mutation APPLIED**; a replace matching nothing reports success and reds nothing. **Table the matrix as a PREDICTION in the file before driving it** -- written after, it is a transcript.
- **"THE TREE DID NOT CHANGE" IS ALSO WHAT A BROKEN VERB PRODUCES.** Every atomicity claim needs the control that the same verb on a passing input DOES change the tree -- **asserted as a change, never `is_ok()`.**
- **A POPULATION IS A CLAIM, AND ITS DEFINING CLAUSE IS WHERE THE ANSWER HIDES.** Two failures and only the first is about emptiness. **EMPTY:** ask an empty directory _was this ever non-empty_, not _is the thing there_ -- answerable from the RECORD, never the listing. **EXHAUSTIVE AND STILL WRONG (vc, 2026-08-20):** _zero of 110 `.bats` files set the override_ was true and complete, and carried a 302-failure finding with a five-day age; the only thing that ever set it was `run_tests.sh`, **which is not a `.bats` file.** A true count over a population you defined is evidence about THE POPULATION. **Read your own qualifier as the hypothesis it is** -- and note this is my `fn next_thread_id` bullet's sibling, not its duplicate: that one hides behind a NAME, this one behind a CATEGORY, and no spelling sweep reaches it.
- **FIVE LIMBS WANT FIVE TESTS, NOT FIVE ASSERTIONS** -- the first failure masks the rest, so an operator fixes one limb and re-runs.
- **WHEN YOU GREEN A ROW, WATCH THE CRITERION COUNT** (vc). If it does not move, the row was not the last one. **A green row and a closed criterion look identical from where the builder stands.**
- **A CRITERION CAN NAME A LIMB NOTHING IMPLEMENTS, AND EVERY SITE THAT KNOWS OF IT CAN BE A READER** -- `project_id` was mandated twice, assumed once, promised once, written by none. **A COMMENT CAN BE NEVER TRUE RATHER THAN STALE: it promised the fix immediately above the call that depended on it, so a reader tracing the defect stops there. DECLINING TO GUESS IS WHAT MADE THE RULING FINDABLE.** **AND THE MIRROR: A COMMENT CAN BE ENTIRELY CORRECT AND UNREAD.** vc's whole diagnosis, count included, sat in the comment above the fixed line for two days before vc re-derived it. **Both failures are the same act -- not reading the prose beside the code -- so the remedy is not _distrust comments_, it is READ THEM AND THEN CHECK.**

## Watch-outs -- instruments

- **A DENOMINATOR CAN BE CORRECT AND STATED AND THE CONCLUSION STILL WIDER THAN IT** (dc, against me). There are TWO hook rosters. **Stating your denominator does not stop you generalising past it.**
- **`cargo test` HALTS AT THE FIRST FAILING TARGET** -- 46/366 against the real 141/985/2. **The stopped run's denominator looks exactly like a denominator.** `--no-fail-fast`.
- **A GITIGNORED SSOT IS INVISIBLE TO `git status`**, so a diff-based blast radius under-reports damage to the one artefact the model calls authoritative, and reads as complete. Walk the filesystem when the claim is _nothing was written_.
- **A GREEN COMMIT GATE IS NOT A GREEN TREE** -- one attachment hash reddened two tests while all four guards passed clean over it, twice.
- **BEFORE CALLING A RED A PEER'S, RE-RUN IT WITH YOUR OWN FILES REMOVED.** `common/mod.rs` compiles into every test in the crate. One command.
- **`grep -c` EXITS 1 ON ZERO** and the pipe form has nowhere to show it. **A ZERO FROM A DATA COMMAND IS SILENT; FROM A MISSING FILE, LOUD; FROM A NAME SEARCH, A FACT ABOUT THE SEARCH.** cwd drifted thirteen times -- `cargo --manifest-path <abs>` beats `cd`. **The Bash tool's shell is zsh: unquoted `--include=*.sh` is a hard error.**
- **A REVISION NAMES SOURCE, NOT THE BINARY THAT ANSWERED** -- `shasum -a 256` and quote the hash WITH the number. **NAME REVISION, CLOCK AND DIRTY COUNT ON EVERY MEASUREMENT**; a verdict reading `+1 dirty` describes no commit and says so.
- **MARK PROVENANCE PER CLAIM: DRIVEN, READ, OR INFERRED** (dc). **The cost lands on the READER, which is why the writer never feels it** -- fencing one claim as inferred told vc exactly which command to run. **VERIFY THE RETRACTION, NOT JUST THE CLAIM.**

## Watch-outs -- four nodes, one checkout

- **PRESENCE IDENTIFIES A FILE AND NEVER ITS AUTHOR**; the working tree is nobody's tree. A peer's red conceals mine and reads as an all-clear.
- **THE WORKTREE AT A NAMED REVISION IS THE ONLY REMEDY FOR THE `--only` CLASS.** `--only` is path-scoped, not hunk-scoped, **and silently skips untracked files inside a named directory** -- `git add -N` first, and read `git status` AFTER the commit rather than trusting its file count. **Mutate inside the worktree, never the shared source.** **Check the checkout SUCCEEDED: `git checkout --detach` ABORTS on an untracked file in the way.**
- **A LIVE CHANNEL DELIVERS AND LEAVES NO RECORD**, and nothing distinguishes _answered_ from _not answered_. On a blocking question the record is for the person waiting. **AND THE GUARDS RUN AT COMMIT, SO THE LIVE CHANNEL IS UNGUARDED.** Measured 2026-08-20: vc's release carried `17:46Z` while `date -u` on this machine read `17:36:30Z` -- ten minutes ahead of UTC and not the BST offset either, so from no clock. **Check A would have REFUSED that stamp at commit; the live channel took it without a word.** **The hazard is TRANSCRIPTION: quote a peer's live stamp into a file and it enters the record laundered through you, past a guard that was watching the wrong door.** Attribute it, never assert it, and flag it back -- a peer's stamp is their file to fix.
- **`prettier` RUNS INSIDE THE COMMIT WINDOW** (`.githooks/pre-commit:11-16` rewrites staged `.md` and RE-STAGES), **after `sync --to-store` hashed the worktree -- so a correct `edit -> sync -> commit` diverges anyway.** Order is **format, THEN sync, THEN commit**; never "sync last", which reads as sync-after-commit and leaves that commit permanently divergent. **A drift guard belongs in the repo-local roster, after the formatters, never the shipped one before them.**
- **Every commit touching an attachment leaves canon divergent until vc syncs, and a later sync repairs the NEXT commit and never that one.** **A table edit is a two-file commit whose second file is one you never edited** -- after editing any SOURCE, ask what renders FROM it before staging.

## Standing rulings

- **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE.** **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE.**
- **HIGHLANDER FORBIDS TWO ANSWERS TO ONE QUESTION; IT DOES NOT REQUIRE ONE ANSWER TO TWO** (vc).
- **TWO ROWS RATHER THAN ONE WIDENED ROW** keep two assertions separately falsifiable; one `file` per row, and the AND gate holds the criterion open until both are green.
- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything found building v3 is work.
- **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON** -- and re-deriving the reason is how you find the reason was wrong.
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING. **`doctor --fix` is WITHDRAWN.**
- **v3 STAYS OFF PATH** until dc repoints `~/.local/bin/intent`. Verified 17:09Z: `-> bin/intent`, 2.19.0.
- **`config.json` DOES NOT MOVE WITH `intent_dir`.**

## Lane and build recipe

`native/**` and the v3 crates are mine. Parity harness = ic. Hooks, roster, `canon_commit_check.sh` = dc. **Canon writes route through vc.**

**`CARGO_TARGET_DIR=<an ABSOLUTE, IN-REPO dir>` FOR ANY VERIFYING BUILD.** Out-of-repo breaks `INTENT_HOME` (`install::home()` walks `current_exe()` ancestors for a marker dir) and manufactures phantom failures; relative under a drifted cwd builds into a nested path nothing reports. Mine is `native/rust/target/cc`. **`rustfmt --edition 2024`, NEVER a bare `cargo fmt`.**
