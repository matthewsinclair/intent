---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-13 08:30Z
status: active
focus: "AC-14.10 LANDED at 124aa0d6f -- /in-whiteboard rewritten onto intent wb, guard sections kept, stamps from the store. AC-14.12 is BUILT, DRIVEN and HELD: deleting ws hygiene removes three checks nothing else performs, and the only good home for them is a hook template vc ruled off WP-14 path, so the ruling is vc's. Next: the AT rows owed for both, and cc's wb pickup change."
claims: []
---

# Interface Claude (ic)

## DOING

**0311 IS LANDED WHOLE AT `b0f773046` AND CLOSED IN THAT LANDING.** Seven files: the four source files, the issue's canon, its dehydrated view and `.intentfiles`. The re-run was green on all three `intentd` binaries with all three arm NAMES present. The result lives in the issue's own record -- cause, both halves of the fix, the mechanism proof, the rate measurement with its positive control, and the residual family reds with their counts and messages. The pair rebuilds at that sha by vc's hand.

**DOING NEXT, in vc's order:**

1. **Review cc's `wb pickup` change when it lands**, against Finding 1 as vc ruled it: pickup writes `status: active`, the heartbeat, `session_id` when `--session` is named and `focus` when `--focus` is named; `touch` stays heartbeat-only; `release` writes paused. Then `wb release`'s `when_to_use` reads true as written and needs no edit, and Finding 3's note gets the surface spellings (`wb show`, `wb status`) rather than the facade names.
2. **The `wb add` row is REVIEWED and sound**; the finding is on its neighbour, `wb archive`, whose enum gained `hold` at `c9f40c79e` while its `when_to_use` still enumerates four kinds -- and that sentence is the published MCP tool description. Archiving a hold is a THIRD reading (its condition was met), not the DONE or the retirement the prose offers. cc's three corrected message rows are correct as ruled.
3. **The protocol half: AC-14.10 is LANDED at `124aa0d6f`, AC-14.12 is BUILT, DRIVEN and HELD** (the hold and its condition are below). vc's ruling narrows it: **the two whiteboard guards STAY in the pre-commit roster through this cut**, retired only after the fleet's last hand-authored whiteboard migrates -- so AC-14.10's `/in-whiteboard` rewrite KEEPS its guard sections and says a generated board's stamps come from the store, and **no hook-template work is on WP-14's path**. AC-14.10 and AC-14.12 are both non-test rows still uncovered on the record; each gets an AT row when its landing is in, 14.12's driven by an AT proving the four `cmd_ws_*` functions AND every caller are gone -- the fifth caller at `intent_claude_cwi:392` is the one a four-arm deletion leaves calling a function that no longer exists.

## TODO

**THE FOUR LIFECYCLE ROWS ARE REVIEWED (`dd3e3444e`) AND THE FINDING IS A BUILD DEFECT, not a wording one; it is with vc.** Full text at `scratchpad/review-lifecycle.md`, reported in the inbox. **`active` is a status nothing on this surface can write**: `wb_node` takes three in-session UPDATEs (`store.rs:4291`, `:4300`, `:4373`), the status one has a single caller (`facade.rs:5082`, `wb_release`) and it only ever writes `Paused`, and `register_nodes` inserts `'paused'` with a NULL session and an empty focus and touches an existing row not at all. `wb_pickup` moves the heartbeat and nothing else, while SKILL.md:219 has pickup writing `session_id`, `heartbeat_at` AND `status: active` -- so `wb pickup` serves one third of that step and AC-14.7 is not met. It reaches the shipped face: `wb status` prints the status word and the focus, and nothing can set either. **`wb release`'s `when_to_use` says _Registering or picking up again is what makes a node active_, which is false in both halves, and `a_tool_description_comes_from_its_row.rs:70` publishes it as the MCP tool description.** Minor third: `wb pickup`'s note names `board` and `boards`, which are facades rather than verbs a reader can run.

**Both findings I carried in are discharged** -- flag exposure now tracks its verb's on every row, and `wb archive` at `idempotent` is right (checked against the rows that would contradict it: `claim`/`unclaim` undo each other and are idempotent too, so `one-way` marks a verb that ACCUMULATES).

**`wb add`'s row is still queued**, and cc's three corrected message rows with it.

**Review cc's three corrected message rows when the items commit lands.** vc ruled my finding without waiting for a push-back: `wb ask` and `wb announce` are `one-way`, `wb clear` is `idempotent`, MCP exposure follows the field, `st attach` is the precedent in both fields at once. The review happens in that commit rather than as a separate pass. **The thing to check is the pair, not the label**: a row whose `recoverability` moves must have its `exposed_on_mcp` move with it, and `--node`'s own exposure is the second half of the same question.

**Then the protocol half, on vc's signal, which comes when cc's LIFECYCLE group lands** -- so the skill describes verbs that exist rather than verbs that are coming. AC-14.10's `/in-whiteboard` rewrite onto `intent wb`; AC-14.12's deletion of `cmd_ws_new`/`list`/`archive`/`hygiene` from `intent_claude_cwi`, reworded by vc with `intent ac edit` to name every CALLER rather than four arms. **Scoped, nothing edited: size M.** The four functions sit at `intent_claude_cwi:218,278,298,316` with dispatch arms at 430 to 433, and a fifth caller at 392 offers `cmd_ws_new` from the interactive prompt -- deleting the four and the four arms leaves that one calling a function that is gone, and in a shell script it fails at the call rather than at load. `in-whiteboard/SKILL.md` names the `ws` family in its scaffolding paragraph, its header-format section and its inbox-shape section. **The open question I have asked cc**: whether `ws new`'s scaffolding becomes a `wb` verb or goes away, because it decides whether that paragraph is rewritten or deleted.

**TWO THINGS cc HANDED ME FOR THE SKILL, BOTH OF WHICH CHANGE WHAT I WOULD HAVE WRITTEN.** First, **the `wb` family's MCP exposure is SPLIT and the skill must state the RULE, never the members**: after the recoverability correction, `status`, `show`, `claim`, `unclaim` and `clear` are exposed and `ask`, `announce`, `decide` and `register` are withheld -- and the split follows each row's `recoverability` field, so a sentence naming the five goes stale silently the first time a field moves. Whatever the skill says about what an agent may do unasked has to be that sentence and not that list. Second, **a node joins by being REGISTERED, with the mechanism left to the cutover** -- the roster still reads each node's `wip.md` header today, so a skill claiming the store is the whole answer is wrong in the one case a reader would actually hit.

**The batched reference regeneration, after the tag, on vc's signal.** Unchanged: both halves `--rev v3.0.2 --baseline v3.0.1`, and `--baseline` must be passed because both generators hardcode `v3.0.0`. Neither needs a build.

## Holds -- work I am NOT doing, each with the condition that releases it

0. **AC-14.12's landing** -- built, driven on all four paths, `shellcheck` clean, banked at `scratchpad/ac1412-cwi.patch`. **RELEASES ON vc's CUTOVER SIGNAL**, where it is the FIRST commit of that sequence (vc, 2026-09-13, ruling shape 3 on my hold): it lands when this estate's boards are generated and the three checks `ws hygiene` alone performed have no subject here. The hook-template exclusion STANDS, so the header guard is not completed, and for estates that stay hand-authored after that pair those three checks are gone -- accepted, with the reason written into the landing: the header format becomes the renderer's, the header-reading `wb register` is the reader that matters for a board about to migrate, and the two guards that ever caught anything at HEAD stay. In that same landing: the four bats arms that drove `hygiene` are deleted with that reason, the two door arms re-point as measured, and **I author the register row's ratification myself** -- `claude ws` to `target.state: retire`, citing hv's 2026-09-12 ruling that ST0069 completes and WP-14 reifies the whiteboard in 3.0.2, under vc's pen, dated from a `date -u` read, saying what falsified the D14 note.

1. **THE QUIET WINDOW IS SUPERSEDED IN PRACTICE AND WAS NEVER FORMALLY LIFTED, WHICH IS WHY IT IS STILL WRITTEN DOWN.** vc opened it 2026-09-12 (no `cargo test`, no `cargo build`, no drives, while dc measured the daemon family on an idle host) and then spent the afternoon ordering builds, worktree suites and a rebuilt pair -- so it is spent by conduct rather than by a word. **A hold discharged by conduct is the one that rots**: the next node to read this board would either obey a dead rule or learn to skim the section. Recorded here, not deleted, until vc says it is lifted.

2. **The palette `Home`/`End` flip** -- RELEASES WHEN hv sets post-3.0.1 work and names it.
3. **The unruled ic-lane defects** -- issue 0303 (the register's `as-observed` rows) and `subagents/.manifest/global-agents.json` (three bats tests assert it). RELEASES WHEN hv rules either in or out.

## Watch-outs -- one line each, leaned 2026-09-12 14:14Z to what bears on the work in front of me; the full list and its worked cases are in `.history/20260912/wip-prefold-1414Z.md`

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

THE REGISTER, which refuses by name and is usually right:

- A new ROW moves `populations.{declared,shipped,probeable}`, `legal_pairs`'s `n` and `census_note`, and the new-surface family count in `dispatch::tests`; a FAMILY needs `flags: []` on its own row; a new KEY must be classified in `key_classes`.
- A shipped mutation needs `recoverability`, and one withheld from MCP while recoverable needs `recoverability_anomaly` -- the generator refuses the silence and says do not bend the label.
- A generated view is regenerated by its tool in the same commit and never hand-edited; the rendered markdown must be a formatter fixed point, so no `*emphasis*` in register prose.
- An exact command literal in shipped source must be declared in `command_rosters_are_derived_or_declared`, and a shipped mutator must sit in exactly one bucket of `write_moves_only_what_changed`.

BUILDING AND VERIFYING, once the quiet window lifts:

- Private detached worktree, its OWN IN-TREE target dir, isolated `HOME`; read `cat ~/.intent/home` afterwards.
- **After editing `intentsvcs`, build `intentd` BEFORE the intent-cli suite** -- the staleness guard reds the whole daemon class in 0.00s and reads as a broken machine.
- **A worktree carries a STAGED copy across `checkout --detach`** (`git checkout -- .` restores from the INDEX): use `git reset --hard <new head>`.
- A test gated on a grammar needs the CLI crate's own pass-through feature, or it silently does not compile and passes by not existing.
- An AT row citing a file is a citation only if the FILE carries the row's literal id.
- Nothing in this workspace may read a clock; bound work in SQLite instructions, not seconds.

- **A RUN THAT NEVER REACHED THE TEST IS NOT A PASS, AND IT WEARS A PASS'S SHAPE.** My control reported _patched: 0 subscription arms red_ and the patched side had never run `daemon_subscriptions` at all -- the lib arm failed first and cargo stopped. `grep -c <arm name>` was 4 on a base log and 0 on every patched log. **The runner's positive control is now that both sides show the arm NAMES before any verdict is read**, and a zero count is a VOID run reported as one (vc). I nearly reported the absence as the result, one hour after putting the same question to two other nodes.

JUDGEMENT, earned 2026-09-12 evening, all six from things that went wrong:

- **THE SAME FAILURE TWICE IS n=2, NOT A MECHANISM** (cc, on their own run, and worth holding because I read peers' suite results all day). Two daemon arms red twice with the same two names reads exactly like determinism; the control at the base commit was green once and then red with the SAME shape, so the family reds at base too and the difference is not established at that sample size. **A repeated name is a hypothesis with a denominator of two.** The answer is a matched run, not a louder reading of the first one.

- **AN ARGUMENT FROM THE CORPUS IS UNDERDETERMINED UNTIL YOU LOOK FOR THE ROWS THAT CONTRADICT IT.** I refused three `recoverability: reversible` rows by generalising from the pairs that fit (`st done` against `st reopen`) and never checked for rows that would break the rule. cc found three -- `ac new`, `at new`, `issues add` -- all `reversible` with no removal verb, so the same argument reaches the opposite conclusion. **The authority was one grep away**: `guide.rs:271` renders the field as _another command on this surface undoes it_, which settles it without an inference. Right answer, wrong route, and the route was the part I was confident about. (The counter-examples then dissolved -- each of those three has a RETIRING verb, `ac withdraw`, `at na`, `issues close` -- which only shows how much work the unchecked half of an argument is doing.)

- **A TEMPLATE IN THIS TREE IS NOT A DRAFT, AND EDITING ONE IS A DEPLOYMENT.** `~/.intent/home` points at this working copy and the installed shim execs `lib/templates/hooks/pre-commit.sh` from it, so my half-finished gate arm refused every commit on ANOTHER estate, mid-deploy, until I reverted it. Uncommitted is not private. Hook and template work happens in a worktree, always.
- **EVERY REPLACE ASSERTS ITS MATCH COUNT, AND THIS IS NOT A MARKDOWN RULE.** A blind swap of `2 => Err(Failure::Unavailable(` hit `doctor_verdict` AND the critic's verdict match, so a rules-less install went from refusing at 2 to sealing a clean verdict at 0 with no output. The arm directly above it warns about that exact fall-through in its own comment. A test caught it; I proved the cause by reverting my own patch and re-running rather than reasoning about it.
- **A PASS THAT PRINTS NOTHING IS INDISTINGUISHABLE FROM AN ARM THAT NEVER RAN** (vc). The doctor arm was silent on 0, so its silence after landing was evidence of nothing -- the same shape as this estate's `0 of 0` digest, one level up. A gate arm you have only ever seen silent must be driven red before it is believed.
- **VERIFY THE PRODUCER YOURSELF BEFORE LANDING THE CONSUMER.** The gate calls `intent` from PATH, so the arm had to land after the pair was rebuilt at its producer -- and I read the pair's own version and doctor's 0 and 4 in my own shell rather than taking the broadcast. Two commits with a rebuild between them, deliberately, instead of one clean-looking landing.
- **A CODE ALREADY CARRYING FOUR MEANINGS DOES NOT TAKE A FIFTH.** Exit 2 is the unbuilt population's predicate, and `retirement_is_enumerable` refused to let doctor answer there. The ruled shape said 2; the estate's own record said 2 was full. Bringing the name and the evidence beat implementing the ruling as given.
- **`cmd | tail` ANSWERS `tail`'s EXIT CODE, AND IT BIT ME TWICE IN ONE SESSION** -- once reading `intent doctor` as rc=0 when it was 1, once reading a background suite as green when it had a failure. Capture without a pipe before reading `$?`, and read a suite's raw output rather than a tail of it.

JUDGEMENT, earned 2026-09-12 afternoon:

- **A MEASUREMENT'S POPULATION IS PART OF THE SENTENCE, AND MINE WAS NOT.** I wrote "the six `1..n` rows" into the register off a jq that selected objects carrying `flags` and never looked at `args`. Eight declarations exist; two are positionals. A claim wider than the thing it measured, written into the surface in the commit whose subject was the surface telling the truth. **Say what the query walked, in the sentence, or do not quantify.**
- **A NAME SEARCH REPORTED AS A CONCLUSION.** I told vc four entities were homeless; `Envelope` had a complete table at `event_log` all along. Two artefacts describing one thing in two naming conventions cannot be joined by name, and the failure LOOKS like a finding rather than like an error.
- **A NEW CODE PATH HAS NEVER BEEN DRIVEN, WHICH IS THE WHOLE OF AC-16.4.** The row-shaped table reader was a second reader with its own green; I drove it red both ways plus its refusal before trusting it. A reader that has only ever agreed is indistinguishable from one that always agrees.
- **A REFUSAL MUST NAME WHERE TO GO, NOT WHERE IT LOOKED.** Three entities share one table, so `absent from wb_node` sent a reader to a table holding two others. It names `wb_node#wb_message` now.
- **THE STALE CLAIM IS USUALLY BESIDE THE ONE YOU WERE SENT FOR.** vc sent me to `organize --default`'s help; the `disposition_basis` under it stated the definition-by-exclusion hv had rejected in as many words, on the very date the basis carries. The code was fixed that day and the register was not. **Read the whole row, not the field you were pointed at.**

JUDGEMENT, earned earlier:

- **READ THE LANDED CODE, NEVER THE ANNOUNCEMENT.** A release-note paragraph said a rebuild was required before a first search; the door's own doc comment said the daemonless query reconciles, and every caller was a daemon or a test. Both readings were defensible and only the call sites settled it.
- **A PEER'S LANDING SILENTLY FALSIFIES DOCUMENTATION, and nothing reports it.** WP-22 made one Upgrading paragraph wrong; the hydrate refusal made a known-defects remedy send the reader to a refusal. When a verb's behaviour moves, the pages that tell a reader to run it are the defect surface -- go and look, they will not tell you.

- **Drive a surface as a USER before calling it done.** Every face was internally consistent and passing, and two still withheld what the reader came for. No test asserting the envelope can see that, because the envelope was right.
- **A fixture that drops an argument it was handed tests a shape its caller cannot produce** -- every green it gives is about a different row from the one the test says it built.
- A claim wider than the thing is not a bug and still has to be narrowed.
- A count in a doc is DEAD (hv): name the thing, or the verb that reports the figure.
- `intent fc` is the human's verb, even in a sandbox.
- Read the clock in the same command as the stamp.
