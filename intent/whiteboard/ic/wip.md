---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-12 20:17Z
status: active
focus: "LOCALFOLDED 2026-09-12 20:17Z for hv's compact. 0311 is DIAGNOSED and FIXED and NOT LANDED: the fix is unlanded in scratchpad/wt-doc and banked as scratchpad/0311-daemon-fix.patch on base e7a59a47b. Base control reds 4 of 8; the patched control must be RE-RUN because it never reached the binary. Then the protocol half. Nothing of mine uncommitted."
claims: []
---

# Interface Claude (ic)

## DOING

**0311, THE DAEMON'S FEEDBACK LOOP: diagnosed, fixed, proven in MECHANISM, and NOT PROVEN IN RATE. It is not landed and must not be.**

**The unlanded work is a worktree and a patch, never this prose**: the tree is `scratchpad/wt-doc` on base `e7a59a47b`, and the diff is banked at `scratchpad/0311-daemon-fix.patch` (four files). `scratchpad/wt-base` is the matched control tree at the same base; `scratchpad/ctrl-logs/` holds all sixteen run logs; `scratchpad/trace.py` and `trace3.py` are the socket clients that produced the diagnosis.

- **THE CAUSE, traced rather than reasoned.** A projection writes canon, the generated views AND `.canon/project.json`; the store's index recorded only the canon. So the daemon's watcher met `todo.md` and `steel_threads.md` with no baseline, published them as external edits, and ingested again -- twice per write. `daemon_watch::one_external_edit_costs_a_bounded_number_of_ingests` has been saying so at base in its own words all along: _the ingest count moved with nothing editing the project. The daemon is watching its own writes._
- **THE FIX, two halves at the cause.** `sync::differs_from_recorded` is one home for _has the store already recorded these bytes_, and the watcher's LEAF branch asks it -- that branch published on scope alone, unread, which made the module's own stated invariant true on the directory door and false on the leaf door. `record_landed` records everything a projection wrote, at one home with three callers, unioned with `canon_files` because `commit` skips a path whose bytes already match and that baseline is what `refuse_if_canon_moved_under_the_store` reads.
- **WHAT IS PROVEN: the mechanism.** On the fixed daemon one write yields exactly `file_changed` then `project_changed`, three runs identical, where before it was five events and then five more; and a subscription opened after setup now receives nothing at all.
- **WHAT IS NOT PROVEN: the rate.** Base reds the two arms in **4 of 8** whole-suite runs. **The patched half of that control is VOID** -- the lib arm failed first, cargo stopped, and `daemon_subscriptions` never ran on the patched side at all. Zero was an absence produced by not running.
- **NEXT, and it is the whole of the next session's first move:** re-run the alternation with the runner's own positive control -- both sides must show the subscription arm NAMES in their logs before any verdict is read, and a log whose count is zero is a VOID run reported as one (vc). Patched needs enough runs to show zero against the base's 4 of 8. The renamed arm `a_batch_of_leaf_events_costs_at_most_one_store_round_trip` is green, so both sides now reach the binary.
- **One unexplained patched red stands**, 1 in 19 on the single-file population, message never caught. vc: it lands named in the count with its log if it does not recur, and is a new issue if it recurs after landing.

## TODO

**Review cc's four LIFECYCLE rows at `dd3e3444e`** -- `wb touch`, `wb release`, `wb pickup`, `wb archive <kind> <seq>` -- next time the register is open. The two findings from the items pass are the ones to carry in: a verb's `recoverability` moving must take its FLAGS' exposure with it (four withheld verbs still carried a flag marked exposed, all four in this family), and a note's stated ground has to be true, not just its label.

**Review cc's three corrected message rows when the items commit lands.** vc ruled my finding without waiting for a push-back: `wb ask` and `wb announce` are `one-way`, `wb clear` is `idempotent`, MCP exposure follows the field, `st attach` is the precedent in both fields at once. The review happens in that commit rather than as a separate pass. **The thing to check is the pair, not the label**: a row whose `recoverability` moves must have its `exposed_on_mcp` move with it, and `--node`'s own exposure is the second half of the same question.

**Then the protocol half, on vc's signal, which comes when cc's LIFECYCLE group lands** -- so the skill describes verbs that exist rather than verbs that are coming. AC-14.10's `/in-whiteboard` rewrite onto `intent wb`; AC-14.12's deletion of `cmd_ws_new`/`list`/`archive`/`hygiene` from `intent_claude_cwi`, reworded by vc with `intent ac edit` to name every CALLER rather than four arms. **Scoped, nothing edited: size M.** The four functions sit at `intent_claude_cwi:218,278,298,316` with dispatch arms at 430 to 433, and a fifth caller at 392 offers `cmd_ws_new` from the interactive prompt -- deleting the four and the four arms leaves that one calling a function that is gone, and in a shell script it fails at the call rather than at load. `in-whiteboard/SKILL.md` names the `ws` family in its scaffolding paragraph, its header-format section and its inbox-shape section. **The open question I have asked cc**: whether `ws new`'s scaffolding becomes a `wb` verb or goes away, because it decides whether that paragraph is rewritten or deleted.

**TWO THINGS cc HANDED ME FOR THE SKILL, BOTH OF WHICH CHANGE WHAT I WOULD HAVE WRITTEN.** First, **the `wb` family's MCP exposure is SPLIT and the skill must state the RULE, never the members**: after the recoverability correction, `status`, `show`, `claim`, `unclaim` and `clear` are exposed and `ask`, `announce`, `decide` and `register` are withheld -- and the split follows each row's `recoverability` field, so a sentence naming the five goes stale silently the first time a field moves. Whatever the skill says about what an agent may do unasked has to be that sentence and not that list. Second, **a node joins by being REGISTERED, with the mechanism left to the cutover** -- the roster still reads each node's `wip.md` header today, so a skill claiming the store is the whole answer is wrong in the one case a reader would actually hit.

**The batched reference regeneration, after the tag, on vc's signal.** Unchanged: both halves `--rev v3.0.2 --baseline v3.0.1`, and `--baseline` must be passed because both generators hardcode `v3.0.0`. Neither needs a build.

## Holds -- work I am NOT doing, each with the condition that releases it

0. **THE QUIET WINDOW IS SUPERSEDED IN PRACTICE AND WAS NEVER FORMALLY LIFTED, WHICH IS WHY IT IS STILL WRITTEN DOWN.** vc opened it 2026-09-12 (no `cargo test`, no `cargo build`, no drives, while dc measured the daemon family on an idle host) and then spent the afternoon ordering builds, worktree suites and a rebuilt pair -- so it is spent by conduct rather than by a word. **A hold discharged by conduct is the one that rots**: the next node to read this board would either obey a dead rule or learn to skim the section. Recorded here, not deleted, until vc says it is lifted.

1. **The palette `Home`/`End` flip** -- RELEASES WHEN hv sets post-3.0.1 work and names it.
2. **The unruled ic-lane defects** -- issue 0303 (the register's `as-observed` rows) and `subagents/.manifest/global-agents.json` (three bats tests assert it). RELEASES WHEN hv rules either in or out.

## Watch-outs -- one line each, leaned 2026-09-12 14:14Z to what bears on the work in front of me; the full list and its worked cases are in `.history/20260912/wip-prefold-1414Z.md`

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
