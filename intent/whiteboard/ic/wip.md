---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-13 10:24Z
status: active
focus: "LOCALFOLDED 2026-09-13 10:11Z on hv's word, holding for instructions while hv and vc sort next steps. Nothing of mine is in flight and nothing of mine is uncommitted. 0311 landed and closed; AC-14.10 landed whole with its AT row; five wb reviews returned; AC-14.12 is built, driven, banked and HELD as the first commit of vc's cutover sequence."
claims: []
---

# Interface Claude (ic)

## DOING

- **Nothing in flight.** Localfolded 2026-09-13 10:11Z on hv's word; holding for instructions while hv and vc settle next steps. The session's landings are in `.history/20260913/wip-prefold-1011Z.md` and, durably, in the commits and in issue 0311's own record.
- **WHAT I AM PART OF NEXT, when vc signals it: the cutover sequence, where AC-14.12 is the FIRST commit.** The hold below carries its condition and everything that lands with it.

## TODO

- **The batched reference regeneration, after the tag, on vc's signal.** Both halves `--rev v3.0.2 --baseline v3.0.1`, and **`--baseline` must be passed because both generators hardcode `v3.0.0`**. Neither needs a build.
- **AC-14.10's AT row is `AT-14.10` and it is RED on one axis only.** The verb-and-flag axis is green through `no_skill_names_an_unshipped_verb`, which is the instrument that already asks the question rather than a second reader beside it. It goes green when AC-14.12 lands, because AC-14.10 also asks that the `intent claude ws` family be updated and that is the held half.

## Holds

- 0. **AC-14.12's landing** -- built, driven on all four paths, `shellcheck` clean, banked at `scratchpad/ac1412-cwi.patch`. **RELEASES ON vc's CUTOVER SIGNAL**, where it is the FIRST commit of that sequence (vc, 2026-09-13, ruling shape 3 on my hold): it lands when this estate's boards are generated and the three checks `ws hygiene` alone performed have no subject here. The hook-template exclusion STANDS, so the header guard is not completed, and for estates that stay hand-authored after that pair those three checks are gone -- accepted, with the reason written into the landing: the header format becomes the renderer's, the header-reading `wb register` is the reader that matters for a board about to migrate, and the two guards that ever caught anything at HEAD stay. In that same landing: the four bats arms that drove `hygiene` are deleted with that reason, the two door arms re-point as measured, and **I author the register row's ratification myself** -- `claude ws` to `target.state: retire`, citing hv's 2026-09-12 ruling that ST0069 completes and WP-14 reifies the whiteboard in 3.0.2, under vc's pen, dated from a `date -u` read, saying what falsified the D14 note.
- 1. **THE QUIET WINDOW IS SUPERSEDED IN PRACTICE AND WAS NEVER FORMALLY LIFTED, WHICH IS WHY IT IS STILL WRITTEN DOWN.** vc opened it 2026-09-12 (no `cargo test`, no `cargo build`, no drives, while dc measured the daemon family on an idle host) and then spent the afternoon ordering builds, worktree suites and a rebuilt pair -- so it is spent by conduct rather than by a word. **A hold discharged by conduct is the one that rots**: the next node to read this board would either obey a dead rule or learn to skim the section. Recorded here, not deleted, until vc says it is lifted.
- 2. **The palette `Home`/`End` flip** -- RELEASES WHEN hv sets post-3.0.1 work and names it.
3. **The unruled ic-lane defects** -- issue 0303 (the register's `as-observed` rows) and `subagents/.manifest/global-agents.json` (three bats tests assert it). RELEASES WHEN hv rules either in or out.

## Watch-outs

- **A CHANGE THAT IS CORRECT AGAINST A RULING CAN STILL BE RED AGAINST AN INVARIANT, AND I LANDED ONE WITHOUT RUNNING THE SUITE.** The skill rewrite named `intent wb register --name` and `--role` -- exactly the spelling vc ruled -- and `no_skill_names_an_unshipped_verb` refused it because cc had not built them yet. cc measured main red before my own run did. **A peer's ruling is not a green**, and the instrument that catches this IS the criterion: for AC-14.10, a document naming an unbuilt flag is the requirement failing rather than a test being early. Run the arms that read what you just wrote, before landing, every time.
- **A SHARED AGGREGATOR GOES STALE UNDER YOU BETWEEN THE READ AND THE JUDGEMENT, AND THE STALE READ LOOKS EXACTLY LIKE A FINDING.** I had `wb archive`'s four-value enum from earlier in the session and a written finding that `wb add` could create a hold nothing could archive -- mechanism named, consequence named, wrong. `c9f40c79e` had added the fifth value in between. Driving it (`intent wb archive hold 1 --node ic` against its `watchout` control) refuted it in one call. **Re-read the row at the moment you judge it, and drive the refusal rather than reasoning about it.**
  THE SHARED TREE, which is where the near-misses were:
- **A canon write is verified PAST the daemon's ingest, never at the moment of it** -- read the attachment's `sha256` and `bytes` back against the file, then read them again after the ingest window, and commit the design and its canon in one call.
- **In `git status`, column ONE is a peer's index and column TWO is yours** -- `M ` is staged by somebody else mid-commit and is not your dirt; `--only` on your own paths is what keeps the two apart.
- `git add <paths>` then `git commit --only <the same paths>` in ONE call; against a peer's `index.lock` wait and re-issue the SAME command, never remove it; judge by `git log -1`.
- **`git commit --only <path>` commits the WORKING TREE version, not the staged one** -- a peer's staged edit in a file you are writing lands in YOUR commit under YOUR message. Check `git status` for `MM` before adding.
- **Never `git stash` here**: the stash list holds other sessions' entries back to v2.3.0 and a pop can apply a stranger's work. Mutate in place and restore.
- **A text edit in a shared aggregator is located by its ROW, never by a pattern** -- a replace on two common field lines put my row's note on `st hydrate`. Verify by parsing the file back and asking which row carried it.
- **Never write the register back through a serialiser**: it reformats lines nobody touched. Insert as TEXT.
- Diff a shared aggregator (`tests/suite.rs`, `CHANGELOG.md`, the register) before `git add`; a `mod` registration lands in the same commit as the file it names.
- THE REGISTER, which refuses by name and is usually right:
- A new ROW moves `populations.{declared,shipped,probeable}`, `legal_pairs`'s `n` and `census_note`, and the new-surface family count in `dispatch::tests`; a FAMILY needs `flags: []` on its own row; a new KEY must be classified in `key_classes`.
- A shipped mutation needs `recoverability`, and one withheld from MCP while recoverable needs `recoverability_anomaly` -- the generator refuses the silence and says do not bend the label.
- A generated view is regenerated by its tool in the same commit and never hand-edited; the rendered markdown must be a formatter fixed point, so no `*emphasis*` in register prose.
- An exact command literal in shipped source must be declared in `command_rosters_are_derived_or_declared`, and a shipped mutator must sit in exactly one bucket of `write_moves_only_what_changed`.
- BUILDING AND VERIFYING, once the quiet window lifts:
- Private detached worktree, its OWN IN-TREE target dir, isolated `HOME`; read `cat ~/.intent/home` afterwards.
- **After editing `intentsvcs`, build `intentd` BEFORE the intent-cli suite** -- the staleness guard reds the whole daemon class in 0.00s and reads as a broken machine.
- **A worktree carries a STAGED copy across `checkout --detach`** (`git checkout -- .` restores from the INDEX): use `git reset --hard <new head>`.
- A test gated on a grammar needs the CLI crate's own pass-through feature, or it silently does not compile and passes by not existing.
- An AT row citing a file is a citation only if the FILE carries the row's literal id.
- Nothing in this workspace may read a clock; bound work in SQLite instructions, not seconds.
- **A RUN THAT NEVER REACHED THE TEST IS NOT A PASS, AND IT WEARS A PASS'S SHAPE.** My control reported _patched: 0 subscription arms red_ and the patched side had never run `daemon_subscriptions` at all -- the lib arm failed first and cargo stopped. `grep -c <arm name>` was 4 on a base log and 0 on every patched log. **The runner's positive control is now that both sides show the arm NAMES before any verdict is read**, and a zero count is a VOID run reported as one (vc). I nearly reported the absence as the result, one hour after putting the same question to two other nodes.
- JUDGEMENT, earned 2026-09-12 evening, all six from things that went wrong:
- **THE SAME FAILURE TWICE IS n=2, NOT A MECHANISM** (cc, on their own run, and worth holding because I read peers' suite results all day). Two daemon arms red twice with the same two names reads exactly like determinism; the control at the base commit was green once and then red with the SAME shape, so the family reds at base too and the difference is not established at that sample size. **A repeated name is a hypothesis with a denominator of two.** The answer is a matched run, not a louder reading of the first one.
- **AN ARGUMENT FROM THE CORPUS IS UNDERDETERMINED UNTIL YOU LOOK FOR THE ROWS THAT CONTRADICT IT.** I refused three `recoverability: reversible` rows by generalising from the pairs that fit (`st done` against `st reopen`) and never checked for rows that would break the rule. cc found three -- `ac new`, `at new`, `issues add` -- all `reversible` with no removal verb, so the same argument reaches the opposite conclusion. **The authority was one grep away**: `guide.rs:271` renders the field as _another command on this surface undoes it_, which settles it without an inference. Right answer, wrong route, and the route was the part I was confident about. (The counter-examples then dissolved -- each of those three has a RETIRING verb, `ac withdraw`, `at na`, `issues close` -- which only shows how much work the unchecked half of an argument is doing.)
- **A TEMPLATE IN THIS TREE IS NOT A DRAFT, AND EDITING ONE IS A DEPLOYMENT.** `~/.intent/home` points at this working copy and the installed shim execs `lib/templates/hooks/pre-commit.sh` from it, so my half-finished gate arm refused every commit on ANOTHER estate, mid-deploy, until I reverted it. Uncommitted is not private. Hook and template work happens in a worktree, always.
- **EVERY REPLACE ASSERTS ITS MATCH COUNT, AND THIS IS NOT A MARKDOWN RULE.** A blind swap of `2 => Err(Failure::Unavailable(` hit `doctor_verdict` AND the critic's verdict match, so a rules-less install went from refusing at 2 to sealing a clean verdict at 0 with no output. The arm directly above it warns about that exact fall-through in its own comment. A test caught it; I proved the cause by reverting my own patch and re-running rather than reasoning about it.
- **A PASS THAT PRINTS NOTHING IS INDISTINGUISHABLE FROM AN ARM THAT NEVER RAN** (vc). The doctor arm was silent on 0, so its silence after landing was evidence of nothing -- the same shape as this estate's `0 of 0` digest, one level up. A gate arm you have only ever seen silent must be driven red before it is believed.
- **VERIFY THE PRODUCER YOURSELF BEFORE LANDING THE CONSUMER.** The gate calls `intent` from PATH, so the arm had to land after the pair was rebuilt at its producer -- and I read the pair's own version and doctor's 0 and 4 in my own shell rather than taking the broadcast. Two commits with a rebuild between them, deliberately, instead of one clean-looking landing.
- **A CODE ALREADY CARRYING FOUR MEANINGS DOES NOT TAKE A FIFTH.** Exit 2 is the unbuilt population's predicate, and `retirement_is_enumerable` refused to let doctor answer there. The ruled shape said 2; the estate's own record said 2 was full. Bringing the name and the evidence beat implementing the ruling as given.
- **`cmd | tail` ANSWERS `tail`'s EXIT CODE, AND IT BIT ME TWICE IN ONE SESSION** -- once reading `intent doctor` as rc=0 when it was 1, once reading a background suite as green when it had a failure. Capture without a pipe before reading `$?`, and read a suite's raw output rather than a tail of it.
- JUDGEMENT, earned 2026-09-12 afternoon:
- **A MEASUREMENT'S POPULATION IS PART OF THE SENTENCE, AND MINE WAS NOT.** I wrote "the six `1..n` rows" into the register off a jq that selected objects carrying `flags` and never looked at `args`. Eight declarations exist; two are positionals. A claim wider than the thing it measured, written into the surface in the commit whose subject was the surface telling the truth. **Say what the query walked, in the sentence, or do not quantify.**
- **A NAME SEARCH REPORTED AS A CONCLUSION.** I told vc four entities were homeless; `Envelope` had a complete table at `event_log` all along. Two artefacts describing one thing in two naming conventions cannot be joined by name, and the failure LOOKS like a finding rather than like an error.
- **A NEW CODE PATH HAS NEVER BEEN DRIVEN, WHICH IS THE WHOLE OF AC-16.4.** The row-shaped table reader was a second reader with its own green; I drove it red both ways plus its refusal before trusting it. A reader that has only ever agreed is indistinguishable from one that always agrees.
- **A REFUSAL MUST NAME WHERE TO GO, NOT WHERE IT LOOKED.** Three entities share one table, so `absent from wb_node` sent a reader to a table holding two others. It names `wb_node#wb_message` now.
- **THE STALE CLAIM IS USUALLY BESIDE THE ONE YOU WERE SENT FOR.** vc sent me to `organize --default`'s help; the `disposition_basis` under it stated the definition-by-exclusion hv had rejected in as many words, on the very date the basis carries. The code was fixed that day and the register was not. **Read the whole row, not the field you were pointed at.**
- JUDGEMENT, earned earlier:
- **READ THE LANDED CODE, NEVER THE ANNOUNCEMENT.** A release-note paragraph said a rebuild was required before a first search; the door's own doc comment said the daemonless query reconciles, and every caller was a daemon or a test. Both readings were defensible and only the call sites settled it.
- **A PEER'S LANDING SILENTLY FALSIFIES DOCUMENTATION, and nothing reports it.** WP-22 made one Upgrading paragraph wrong; the hydrate refusal made a known-defects remedy send the reader to a refusal. When a verb's behaviour moves, the pages that tell a reader to run it are the defect surface -- go and look, they will not tell you.
- **Drive a surface as a USER before calling it done.** Every face was internally consistent and passing, and two still withheld what the reader came for. No test asserting the envelope can see that, because the envelope was right.
- **A fixture that drops an argument it was handed tests a shape its caller cannot produce** -- every green it gives is about a different row from the one the test says it built.
- A claim wider than the thing is not a bug and still has to be narrowed.
- A count in a doc is DEAD (hv): name the thing, or the verb that reports the figure.
- `intent fc` is the human's verb, even in a sandbox.
- Read the clock in the same command as the stamp.

## Decisions

_(none)_

---

_Generated by Intent v3.0.1 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
