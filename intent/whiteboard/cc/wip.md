---
node: cc
name: Control Claude
role: control
session_id: 98a46c38-f370-4d67-b2c5-c2536e0ae8f9
commit_session_id: 0167bZhMQsEXFM5JZUZxL5g7 -- ROTATED THIS SESSION (time NOT read off a clock, so NOT stated -- an invented stamp is worse than an admitted gap) and NOT witnessed on a commit of mine yet; the harness announced it. The prior value 01XYetoGJWvBxvL4PE8sGZTu was witnessed on eaef2a04f, 0b5d46c96 and 018016241 and SURVIVED A COMPACT unchanged at 15:55Z. So: a compact does NOT rotate it, and something else does. Read it off your own first commit rather than trusting this line. POINT-IN-TIME.
heartbeat_at: 2026-09-04 17:43Z
status: active
focus: "FOLDED 2026-09-04 17:43Z ON hv INSTRUCTION AHEAD OF A COMPACT -- STATUS STAYS active, a fold is not a session ending and this does NOT release. Pre-fold verbatim at .history/20260904/wip-prefold-1741Z.md; today's narrative is there and is not repeated on this board. RESUMING UNDER vc's DIRECTION ON THE BOUNCE. ONE THING WAITS ON hv AND ONLY ONE: commit 0232, one file, path set agreed with dc. NOTHING OF MINE IS IN FLIGHT ANYWHERE -- and that had to be PROVEN today, not asserted. NO FIGURES FROM MEMORY; RUN THE VERBS."
claims: [ST0056/06, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT. FOLDED FOR A COMPACT; RESUMING UNDER vc's DIRECTION ON THE BOUNCE.** Pre-fold verbatim: `.history/20260904/wip-prefold-1741Z.md`.

**WHAT THE BOUNCE MUST NOT RE-DERIVE, because each cost real time today:**

- **`WP-06` IS RULED OUT, NOT UNSTARTED.** hv ruled 2026-08-31 that `config`, `ext` and `learn` **ship declared-and-unbuilt in 3.0.1**. The ruling lives in two Rust doc comments and one `MODULES.md` cell, **on no board but this one**. **Do not re-size those rows.** hv's later _everything is in 3.0.1_ does not silently reach them -- a general policy stated after a specific ruling does not vacate it.
- **`render.rs` IS QUIET AND NONE OF IT WAS EVER MINE.** dc's A1 landed `8783243ce`, ic's `entity_json` collapse `0f52cd5b`. ic attributed dc's 142-line diff to me because I own the file; had I taken their word I would have committed dc's A1 under my message. **Before touching `render.rs` for the `agents` fix, ANNOUNCE.**
- **`0232` IS THE LAST LIVE INSTANCE of the store-resolvable / git-invisible class.** vc's `0233` landed `0f41e87f4`, ic's `0230`+`0231` landed `2bb0960d5`. dc's sweep at `bin/.devbin/cmd/canon` names `0232` alone.
- **BARE `intent` IS BEHIND HEAD.** `~/.local/bin/intent` symlinks into `target/release/`; **`native/rust/target/debug/intent` is the current one.** Drive that or pass `INTENT_BIN`. Rebuild is blocked on `0196` and is with vc.

## TODO -- startable, mine, smallest first

- **XS** **Commit `0232`** -- awaiting hv. **One file**, `add` + `commit --only intent/.canon/issues/0232.json`. Path set agreed with dc; `0230`/`0231` were ic's and have landed; **`intent/.canon/project.json` is excluded by both of us -- its `todo_watermark` re-dirties on any `todo` drive.**
- **S** **`intent agents` bare CLAIMED (vc routed it to me).** Answers rc=2 _not implemented yet_ while `agents --help` lists `init`/`generate`/`sync`, and `agents sync` is what `in-essentials` rule 2 orders every agent to use. **Same class as `browsed()`, same file, fixed by me at `eaef2a04f`: a dispatch arm making a false claim about the build.** Canon half is dc's.
- **XS** `0095`/`0096` -- CLOSE as never-specified. Driven: empty in title AND body. `0223` debris.
- **XS** File the `implemented_check.sh` false-positive class. One candidate eliminated: bare-versus-armed probe disagreement is NOT it.
- **S** `0063` -- FIELD MIGRATION, title's 187 chars into the body. **NOT a close.**
- **S** `0205` -- vendored fourth block ACCEPTED, reason at `bin/.devbin/lib/builtins:66`.
- **S** Migrator-commit -- `migration.md` Phase B step 7 and `AC-00.8` stop claiming _one commit_. Correct doc and row; do NOT build the commit.
- **M** `0192` RULED IN -- refusal in `info_read_back`, placement decided.
- **S-M** `SERVED_BY_DAEMON` (`render.rs:235`) is ONE entry **and a `const`, so it can only ever hold PAYLOAD-FREE variants.** Project the dispatch table's payload-free arms, and **REFUSE on a daemon-eligible arm carrying arguments rather than skip it** -- silence there reports a clean table while a verb quietly loses daemon coverage.
- ~~`implemented_check.sh` classify on marker AND rc=2~~ **STRUCK**: the file classifies on the MARKER by design, with B1/B2 proving exit 2 alone does not classify. The "fix" would have ADDED dc's W72 blindness over the controls that disprove it.

## Holds -- mine, with the condition that releases each

- **M** `AC-06.1`'s coverage half -- RELEASED WHEN a burn TSV **covering the estate** exists AND `INTENT_BIN` resolves to one binary. **Re-driven 16:00Z, unmet:** a burn TSV EXISTS (`parity/tools/burn-baseline.tsv`) and **existence is not coverage**, which is the distinction this hold keeps. `INTENT_BIN` unset; four `intent` binaries reachable. `coverage_map.sh` refuses to publish and is RIGHT to.
- **L** `0216`/`0226` fix -- RELEASED WHEN a monotonic version the ingest does not own exists. The obvious fix collides with `written_at`, which the ingest rewrites wholesale.

## Decisions owed by hv -- question, options, recommendation

**THREE WERE STRUCK OR WITHDRAWN RATHER THAN ANSWERED, ALL BECAUSE THE PREMISE FAILED, NOT BECAUSE THEY WERE ANSWERED.** `config` bare and `WP-08 endorse/override` -- hv had already ruled. **`agents` bare WITHDRAWN: vc's census found bare noun verbs split FOUR ways** (`st`/`wp`/`ac`/`at` rc=1; `issues`/`lang`/`modules`/`plugin`/`llm` rc=0; `doctor` runs; `config`/`agents` rc=2) **and my three-option menu contained none of them -- there was no convention to rule toward.** Re-pose against the four measured states if one is wanted.

- **Should `at green` run the L3 arm?** (i) warn, do not refuse (ii) refuse (iii) leave. **REC (i)** -- refusing breaks the legitimate write-then-cite order.
- **`INTENT_BIN` flip and re-baseline -- which order?** (i) flip then re-baseline (ii) re-baseline then flip. **REC (i).** _Neither this cut_ is STRUCK: there is no cut to be outside of.
- **`burn.sh` re-run, or accept `AC-06.1`'s coverage half red?** (i) run (ii) accept red on the row (iii) descope. **REC (i), and it is hv's because full-suite runs are.**
- **Flip `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"` to a gate?** (i) clear the class then gate (ii) stay report-only (iii) gate now. **REC (i).** Lint half is dc's.

## Open, no owner

- **`--all`: 16 invocations (9 `install`, 7 `uninstall`), 5/7/2/2, confirmed three ways with a negative control; 14 in `keep`-classified files.** `dispatch-table.json` declares it **zero** times; v3 refuses at **rc=1 clap parse**. **NOT RUN, NOT CLAIMED TO FAIL.** **It is NOT a new hole** -- `register.md:35` already records that `keep` cannot promise a file's SETUP survives v3, crediting **cc, 2026-08-14**, two paragraphs above the table I quoted. **THE LIVE RESIDUAL: three questions, two predicates.** burn asks _does this reach the v2 CLI_; `fixture_probe.sh` asks _does this hardcode a v2 PATH_ (correctly `none`); **neither asks whether the ARGV still PARSES under v3**, where all 16 live -- so `keep` + `none` reads as _nothing known against this file_ and should not. **7 of 16 are `uninstall --all`, which canon never names**, so 44% sits outside a canon-vs-table arm; **suite-vs-table catches both.**
- **Something WALKS the CLI surface.** Seven CLI-token-titled creations in `event_log`, two episodes eight days apart, machine-paced. No generator found.
- **Is my unfiled daemon-lock race a duplicate of `0210`?** Adjacent ground, NOT compared.
- **Does one fix serve both `0216` and `0226`?** Same collision, opposite symptoms. Not driven.
- **The roster guard says _N parity file(s) in this commit_ and means the WORKING TREE.** Wording defect, unfiled.

## Watch-outs

**Rules only. Every worked example is in `.history/20260904/wip-prefold-1741Z.md` and `.history/20260903/wip-prefold-1718Z.md`.**

**A. THE INSTRUMENT ANSWERED A QUESTION ADJACENT TO THE ONE ASKED, AND ANSWERED IT CORRECTLY.** The dominant family; it arrives while I am being careful. **RULE: name the FIELD you read and the INSTRUMENT you read it with.** **REFINED by dc: the old form said flatly that `--help` is about the PLAN, and that is FALSE when help is derived from the parse source** (`spine.rs:624` declares `entry.flags` onto clap from a table compiled in by `include_str!`, so the declaration answering `--help` is the one that rejects the argument). **Not _distrust help_ but _establish what help is derived from_.** A declared row remains a statement about the plan.

**A2. FABRICATION WITH THE CORRECT VALUE PRESENT. TWICE TODAY.** A sha typed wrong with the real one four lines up in my own output; then **NINE DAYS OLD written into this board and two messages in the same call where I had echoed `rel binary mtime: 2026-09-04 15:33Z` myself.** **It is not a fact about clocks -- it is a fact about any opaque token a reader cannot check by looking**: shas, pids, issue numbers, line numbers, dates. **RULE: substitute the command, never the value.** And a **date travelling ATTACHED to a sha inherits the sha's authority and none of its checkability**.

**B. CONTROLS, OR THE READING IS NOT EVIDENCE.** A control that cannot distinguish _safe_ from _never tried_ is not a control, and it must vary the axis the check reads. **An instrument inside its own population is CONFIRMATORY, not merely noisy** (`Op::Registry` is in `wire::UNCOUNTED` for this reason). **RULE: positive-control the instrument before its silence means anything** -- it caught a wrong-corpus grep of mine today, where `grep -c` returning 0 for a string I expected is what said the CORPUS was wrong rather than the pattern.

**B2. A DEFECT THAT IS THE DISAGREEMENT BETWEEN TWO INSTRUMENTS' POPULATIONS IS INVISIBLE TO BOTH, AND NO PREDICATE FIX REACHES IT (dc).** `implemented_check.sh` probes what the table DECLARES; `canon_mandated_verbs_check.sh` probes what canon NAMES; `--all` is in the gap. **Neither instrument is bad and neither population is wrong -- the PAIR has a seam and the defect IS the seam.** **RULE: ask what neither population contains, and remedy with a STATIC comparison of the two populations, never a better predicate in either.** **AND: read an instrument's STATED SCOPE before believing its silence** -- `pertest.md`'s zero rows mean _not pending_ (41 of 113), never _not covered_.

**B3. AN ASSUMED POPULATION -- INFERRING A PEER'S METHOD RATHER THAN READING IT. TWICE TODAY, BOTH DIRECTIONS, BOTH WITH dc.** **RULE: a correction to a peer's FIGURE requires reading their PATTERN, and a claim about what an instrument over-matches is settled by a negative control, never by reasoning about the regex.**

**C. ARITHMETIC AND SAMPLE SIZE.** **n=2 is not a result about a stochastic process.** **A total you did not enumerate is not a total you may publish.** **RULE: state n and state the variance; below n=5 on a process you have watched vary, the honest sentence is _observed twice, not characterised_.**

**D. PREMISES.** **Drive a ruling's premise before building on it** -- of four build rulings in one day, THREE had false premises. **Assert the premise your fix rests on as a test, in the direction that would embarrass you.**

**D2. A MEASUREMENT IS RE-TAKEN AT THE MOMENT IT IS QUOTED, AND THE RELAY IS WHERE IT ROTS.** **The tell is a possessive tense:** _the daemon IS stale_ is a claim about now sourced from a reading about then, and nothing in the sentence marks the gap. **RULE: a figure crossing into a message gets re-driven in the turn that sends it.**

**D3. AN OBJECTION HAS A PREMISE TOO, AND MINE GO UNDRIVEN BECAUSE THEY FEEL LIKE CAUTION.** Twice today I reasoned from D56's PRINCIPLE against a §10a DETAIL I had not read, with ic having read it both times -- and the second time the refuting text was **a doc comment I wrote myself** (`intentd/src/web.rs:28-37`). **The tell is that an objection does not feel like a claim; it feels like diligence, so its premise is never stated and therefore never driven.** **RULE: before an objection leaves this session, name its premise as a sentence and drive THAT.** When the premise is _the design does not permit X_, the drive is opening the design document.

**D4. _I DECLINED TO MEASURE BECAUSE OF SIDE EFFECTS_ IS ONLY HONEST IF I CHECKED WHETHER THEY WERE CONTAINABLE.** Unchecked, restraint produces an unmeasured claim wearing the costume of rigour -- and I then reasoned forward from the gap using **v2 evidence about a v3 binary**. **RULE: name the mechanism by which the side effect reaches the operator and ask whether it can be contained.** A scratch `HOME`, a tempdir root and a detached worktree are the three that work here.

**E. THIS BOX AND THIS SHELL.** **A probe whose EXIT CODE is the finding never goes through a pipe** -- `cmd | head` reports head's status, done twice in one day, the second time after catching the first, and **both times the WRONG number was the reassuring one**. zsh has `pipestatus`, not `PIPESTATUS`, and reaching for either is already the wrong shape. Unquoted `$var` does not word-split; an unmatched glob aborts the whole command. **A heredoc carrying prose is ALWAYS `<< 'EOF'`** -- board prose is dense with backticks and an unquoted heredoc command-substitutes every one (it executed `intentd daemon` against the real HOME once; `intentd`'s refusal of an unrecognised argument is the only reason nothing happened). **A clock value goes in through a `2026-09-04 17:43Z` placeholder plus `sed`.** `cargo check --workspace --all-targets` -- the flag is the half memory drops. **`stat -f %m` does NOT follow a symlink on macOS; `stat -Lf %m` does.**

**F. THE SHARED CHECKOUT.** Canon cannot be split, so every canon commit is silently multi-node. **`add` + `commit --only` is the only safe write; a live `index.lock` is a WAIT, never a removal, and the retry is the SAME command re-issued, never recomposed.** A release build DELETES the shared pair before building (`0196`) while the dirty-tree guard REDIRECTS -- two correct rules that can take the machine down together.

**F2. FILE OWNERSHIP DOES NOT IMPLY DIFF AUTHORSHIP, AND `git status` CARRIES NO AUTHOR AT ALL.** `--only` protects a FILE and not a HUNK (W39); this is one step further -- **the natural way to decide whose a dirty hunk is, _who owns this file_, is not evidence.** ic attributed dc's A1 to me on exactly that inference and nearly had me commit it. **RULE: read the diff for its SUBJECT before accepting or asserting that a hunk is anyone's.** Four independent lines beat one ownership inference.

**F3. CURRENCY IS A PROPERTY, NEVER A VALUE -- AND SO IS EVERY COUNT.** A pin trailing HEAD is the correct steady state. **I handed vc `214/214`; they adopted the wording and were carrying it to hv; it is `216/216` because the ESTATE moved.** The property (store and disk agree exactly, both directions) is true and re-confirmed; the number was stale before it was read. **dc's LIMB, and it is theirs: A VALUE YOU ARE PASSING ON IS ONE YOU HAVE NOT MEASURED** -- relaying launders a claim, which arrives as a peer's measurement and leaves as an estate fact with no author, and the courier has no stake in being right. **AND THE LOOP: a wrong figure came back to its own author through two hops** (vc measured, dc relayed, I published it back to both) -- **a claim returning via a third party reads as independent confirmation and is your own error wearing a different name.** **RULE: hand the PROPERTY and the verb that reproduces it, never the number.** **DISCRIMINATOR for a stale binary: a STORE read survives it; a claim about BINARY BEHAVIOUR does not.**

**G. `0216` AND WRITING CANON.** **The read-verify-retry loop is a REQUIREMENT, not good practice** -- it replaces _one verb at a time_, because **the debouncer sees WRITES, NOT AUTHORS, so spacing protects only against your own burst.** Verify on an OBSERVABLE, never on a duration, never on the tool's `ok`. **After a revert the DISK WINS, so store and disk agreeing AT THE OLD VALUE is the signature, not the exclusion** (vc). `intent st attach` writes store AND canon and NEVER the disk file (`0082`).

**H. FOLDING THIS BOARD.** **Write the pre-fold copy BEFORE folding** -- it is what made this morning's corruption a restore rather than a rewrite from memory, and **nothing compares a folded board to what it was folded from**, so a reconstruction would have passed every check we have. **Anchor on exact strings, never a prefix**: `index("## Decisions")` matched `"## Decisions owed by hv"`, inverted the slice and duplicated the tail. **`hygiene: ok` PASSES ON A CORRUPTED BODY** -- its contract is the header block, deliberately -- so **verify the section and family counts by hand.** vc hit the same prefix bug an hour after I described it to them and survived only because their data lacked a prefixed sibling.

## Decisions

- (2026-09-04, hv via cc) **`config`, `ext` AND `learn` SHIP DECLARED-AND-UNBUILT IN 3.0.1** (2026-08-31). **A general ruling stated later does not vacate a specific one.**
- (2026-09-04, vc) **THE ES RENDERER IS `WP-17`'s BY WP-17's OWN TITLE.** `D56` forbids server-rendered HTML and DESIGNS the client-rendered browser face.
- (2026-09-04, ic via §10a) **`widget` IS A SEMANTIC CLASSIFICATION, NOT A VIEW.** `prose` means the field is long; TUI to `$EDITOR`, browser to `<textarea>`, SwiftUI to `TextEditor`. **Renderers legitimately differ in the WIDGET and never in the model.**
- (2026-09-04, ic REFUTING cc) **`SERVED_BY_DAEMON`'s AXIS IS _REACHABLE BY A CLI VERB PATH_ VERSUS _WIRE-ONLY_, NOT DUAL-PATH.** The table is a `const`, so only payload-free variants fit. **`Op::Set` HAS the dual path and is absent** -- which refutes the axis on evidence I already held. **A one-row table cannot distinguish two axes.**
- (2026-09-04, ic) **ENTITY-TO-JSON HAD TWO HOMES IN `render.rs`**; moved to `intentsvcs` beside `triples`, with the `wp` arm RESOLVING rather than inheriting `_ => None` so `nav.rs:412`'s blank-form prediction is fixed rather than relocated.
- (2026-09-04, cc+dc) **A HOLD DISCHARGED BY THE CONDITION IT NAMED TEACHES THE OPPOSITE OF ONE THAT ERODES.** File the protection BEFORE discharging the hold.
- (2026-09-04, cc+dc) **A DIRECTORY `add` PROTECTS A PATH AND A DIRECTORY PATH IS NOT A FILE.** And **a path that moves on its own is not a path either node can own** (`project.json`'s `todo_watermark`).
- (2026-09-04, cc) **THE CANON STORE AND DISK DO NOT DISAGREE.** The divergence is git-only. Filing writes store and disk atomically; **git is a separate human-gated act and NOTHING REPORTS THE GAP.**
- (2026-09-04, ic) **`git update-index --add` READS THE WORKTREE**, so a synthesised index contains the fresh bytes BY CONSTRUCTION -- it answers _would this path set pass_ and **cannot diagnose any defect whose subject is the index/worktree difference.**
- (2026-09-04, cc) **A SET PASSING TOGETHER IS NOT EVIDENCE ABOUT ITS SUBSETS.**
- (2026-09-03, ic->cc) **THE CLASS IS THE UNDRIVEN NUMBER, NOT THE WRONG ARTEFACT.**
- (2026-09-03, cc+vc) **CONTENTION IS `0216`'s VARIABLE** -- not spacing, not corpus size. **Refusals (`0226`) and silent losses (`0216`) TRADE OFF**, so a single counter prints _fewer losses under load_ and reads as improvement.
- (2026-09-02, cc+vc) **`0216`'s FIX IS DAEMON-SIDE, NOT USAGE DISCIPLINE.** A hazard reachable by an ordinary shell loop cannot be mitigated by how carefully nodes write.
- (2026-09-02, vc) **A RIDER CANNOT BE VIOLATED BY A CASE ITS OWN HAZARD CANNOT REACH.**
- (2026-09-02, vc) **`WP-14` AND ALL 12 OF ITS ACs WERE DESCOPED WHOLE TO ST0069** by hv on 2026-08-30. `AC-09.5`'s wip/boards half goes with it.
- (2026-09-02) **TWO MACHINE PROJECTIONS OF ONE VALUE MUST NOT DRIFT; A HUMAN RENDERING OF IT IS NOT A COPY AT ALL.**
- (2026-09-02) **CONTENT COMPARISON DOMINATES A VERSION COUNTER FOR A COMPARE-AND-SWAP.**
- (2026-09-02, vc) **AN EXCLUSION MUST BE VISIBLE WHEREVER THE BEHAVIOUR IS CLAIMED, NOT ONLY WHERE THE CHECK LIVES.**
- (2026-09-01, hv) **v3.0.1 IS FEATURE COMPLETE, THERE IS NO TAG WINDOW AND NO EXTERNAL CONSUMER, AND COST IS NOT A CONSTRAINT.** The scarcity register is retired as a class.
- (2026-09-01) **A REFUSAL THAT CANNOT SAY WHAT IT FOUND MAKES ITS OWN DEFECT UNDIAGNOSABLE.**
- (2026-09-01) **A REMEDY INHERITS ITS BRANCH'S ASYMMETRY.** Confirm-before-refuse is SAFE on the lock and WRONG on the probe.
- (2026-08-31, vc) **A CRITERION IS NOT REWORDED TO WHAT THE CURRENT STATE SATISFIES.** The ruled form must be HARDER.
- (2026-08-31, vc) **A CLASS CHANGE WITHOUT ITS REASON IS A DELETION WEARING A NEW LABEL.**
- (2026-08-31, ic correcting me) **AN OWNERSHIP SPLIT IS A PURPOSE, NOT A BOUNDARY.** Two hands in one file IS `0206` in miniature.
- (2026-08-31, CORRECTED) **`close --note` STAYS RULED OUT ON A CONTINGENT FOOTING.** The keg has no `edit`; the ruling survives because v3.0.1 ships from the tree.
