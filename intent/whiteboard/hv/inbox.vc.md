# inbox: vc -> hv

## (2026-08-16 19:54Z)

**0043 IS CONFIRMED LIVE. There is no inferred step left in it.**

Five arms against Claude Code 2.1.233. Each is a throwaway directory with its own `--settings` wiring `UserPromptSubmit` exactly as `.claude/settings.json` wires it (matcher `""`, timeout 2000), driven headless with `claude -p`:

| arm            | hook exit                   | result                                              |
| -------------- | --------------------------- | --------------------------------------------------- |
| ARM0           | 0                           | `PONG` -- the prompt runs                           |
| **ARM1**       | **1**                       | **`PONG` -- the prompt runs**                       |
| **ARM2**       | **2**                       | **`UserPromptSubmit operation blocked by hook`**    |
| **ARMV3**      | **2** (real v3 build)       | **BLOCKED**, carrying v3's own not-implemented text |
| **ARM2-SLASH** | **2**, prompt `/in-session` | **BLOCKED**                                         |

**ARM1 is the arm that matters, and it was not in the plan.** Confirming that `2` blocks would have left the CAUSAL claim untested: 0043 asserts `d2b8e76d` created this by moving unimplemented commands from `1` to `2`, and that requires `1` not to block. It does not. So the arms establish the cause rather than the symptom.

**ARM2-SLASH settles self-sealing, which was the weakest part of the filing.** `/in-session` is the documented remedy for a stuck gate and it is itself a prompt submission, so the same hook blocks it. The other documented escape -- `touch` the sentinel named in the hook's error output -- is visibly unavailable in ARMV3's output: the text printed is v3's not-implemented message, and **no sentinel path appears, because the script that would print one never ran.**

**One finding the arms added that I did not anticipate: the `claude` process itself exits 0 on a blocked prompt.** The block is in-band, in the output stream. **So any wrapper or automation checking the process exit code sees success while the model never saw the prompt** -- a second silent-failure surface, sitting in exactly the layer you would use to detect the first.

The fixture needs no migrated project and no interactive session, so it is cheap enough to keep as a test. Two notes for whoever lands it: **assert on the OUTPUT, not the exit code** (the blocked run exits 0), and **keep ARM1** -- an assertion that `2` blocks passes equally on a build where every code blocks.

Issue 0043 updated with all of it.

**This is a direct discharge of the caveat I gave you when I filed it.**

I told you I had NOT observed the block in a live session, that it was derived from our own hook's documented contract plus a measured exit code, that the confirmation was cheap, and that **it should be run before anyone acts on `critical`.** It has now been run. **The severity stands at critical, on measurement.**

**What changed my confidence most is ARM1**, which I had not planned. Confirming that `2` blocks would have left the causal claim untested -- and the causal claim is the whole issue, because it says our own fix from this morning created this. `1` does not block. So `d2b8e76d` created it, exactly as filed.

**No decision is being asked of you here.** cc's standing instruction -- this repository does not migrate until 0043 is settled -- is now backed by measurement rather than by my reading, and dc is already building the fix (`install.rs`, one resolver, both consumers reading it). **The thing I would flag for when you are back is the publication hold: dc reads this as upstream of publication, and I agree.**

-- vc

## (2026-08-15 17:12Z) FOR YOUR RETURN -- three parked items, each framed as one decision. Nothing is blocked on you except the tap.

Pens taken, peers told, and the operating rule announced: rule what is rulable between nodes, park only what genuinely needs you, and **hv being away widens nobody's authority.**

**1. CREATE `matthewsinclair/homebrew-intent`.** The only true blocker. D40 names the tap; creating it is an account action and dc asked rather than did, correctly. **AC-11.1 and AC-11.4 are downstream of a publication that cannot exist until it exists**, so WP-11 sits honestly at 2/4. Everything dc can do without it is done.

**2. Do `todo --flush` / `--prune` semantics carry into v3?** One decision, and it decides whether a field exists. **If they retire, the DONE watermark retires with them** and DONE filtering becomes a query over the `completed` dates already in the model. If they carry, the watermark is durable state homed in `config.json` (already ruled) rather than grepped back out of the generated `todo.md` (which is what v2 does, and is why the question surfaced). **Recommendation: retire them** -- the watermark is the only durable state in the system with no record behind it, and retiring is the option that removes a concept rather than relocating one. Not urgent; nothing waits on it.

**3. The whiteboard's hand-authored `## (...)` stamp is the same defect we spent the day lancing, and Intent SHIPS this protocol.** Every consumer inherits it. The protocol already contains its own answer in the same document: _"Use commits when you need ordering you can prove."_ **The clock guard's three checks, my six watch-out bullets and your repeated rulings are all scaffolding around a value nobody should be writing.** dc found it and correctly did not propose the change. **Recommendation: a board entry carries no time and is ordered by its commit** -- but this changes a shipped protocol every consumer runs, so it is a scope call and yours. Cost of waiting: nil.

**FOR INFORMATION, no decision wanted.** Issue **0035** filed high: `ac satisfy` accepts an empty `--evidence` at all three layers, in v3 and in v2 -- declaration structurally invisible (`Flag` deserializes 3 of 8 authored fields), `render.rs:671` `unwrap_or_default()`, facade stores it, and v2 never checks `ref`. **A non-test AC can be satisfied with no citation, which collapses the one distinction the AC/AT machine rests on.** Measured blast radius: **zero** -- all 22 satisfied non-test rows in ST0056 carry evidence. Latent, not realised, no audit owed.

Contract 34/106; gates 02 7/8, 03 9/10, 06 4/10, 11 2/4; lint clean at 106 rows. **The long pole is AC-02.8's unit** -- cc holds it, the rulings it needed are made, and `one_clock.rs` will refuse it mid-way until cc reworks the guard, which they have been told.

-- vc

## (2026-08-16 10:18Z) HV DIRECTIVE, ANNOUNCED TO ALL: NO MORE PUSHES TO `upstream`. The CI/CD budget is spent. `local` is fine.

**From hv, just now, verbatim in substance: _"no more pushes to upstream. The CI/CD budget has been hit. Local pushes are fine."_**

**All four of us have been pushing both remotes on every commit** -- it is in our commit habits and in at least my own board's rules -- so this needs to reach you before your next commit rather than after it.

- **`git push local main`** -- yes, keep doing this. Dropbox remote, no CI.
- **`git push upstream main`** -- **STOP.** Every push there triggers the GitHub Actions matrix, and that is what has run out.

**`int prepush` will not save you**: its clone-check gate is about whether `native/` moved, not about which remote you are pushing to, so it will pass a push to `upstream` exactly as before. **This is a discipline, not a control, until someone builds one** -- and I am not building it in `bin/**` with sessions live.

**Nothing needs rewinding.** Work already on `upstream` stays there; this only changes what we do next. **`main` on `local` and `upstream` are in sync as of `99c66e8b`, so nothing is stranded** -- the divergence starts from here and is expected.

**dc: this may want a devbin guard eventually** -- a `prepush` arm that refuses `upstream` unless explicitly overridden would be the natural home, and it is your lane. **Not asking for it now**; flagging that the place exists so it does not get built somewhere else.

-- vc

## (2026-08-16 11:31Z) FYI only -- no response needed. **hv HAS RULED THE v3 CUTOVER VERSION: 3.0.0.**

**Direct from hv, this session:** _"The v3 cutover version is 3.0.0. We will get all of this done -- including the text search and code parsing -- and then push 3 and then fix forward on patch releases."_

**Three things follow and the third is the one that changes sequencing.**

**1. dc is UNBLOCKED. AC-11.1 and AC-11.4 sat behind a real version and nothing else** -- not the tap, which has existed since 15:19:58Z yesterday. That was the only thing standing between dc and those two rows.

**2. The scope statement is now explicit and it is WIDER than the twelve-WP ladder reads.** Text search and code parsing are named as IN for 3.0.0, not deferred to a patch. WP-13 (`index_scope` / `search_lexical` / `search_structural` / `index_staleness` / `search_degradation` / `background_index` / `mcp_search_tool`) is nine `to-write` rows today and it is not optional.

**3. The release POSTURE is fix-forward on patches.** Ship 3.0.0 when the ladder is done, then correct on 3.0.z. **That is a licence to finish, not a licence to lower a bar** -- the fix-forward half applies after the cut, and the ACs are still the gate before it.

-- vc

## (2026-08-16 11:37Z) `undefined` -- the info you asked for, and it is a smaller question than I made it sound.

**FIRST, A CORRECTION TO MY OWN FRAMING. I told you `undefined` had no place in `parity.md`. It is in `parity.md`, at line 17, and has been since 2026-08-14.** My grep searched lowercase `undefined`; the file capitalises it as **Undefined** in the class list. Same miss I made on `INTENT_VER` this morning -- searching for the literal token instead of the idiom. **The instrument agrees with the file**: `class_vocab_check.sh` reports _"2 states claim a parity class, 6 classes named, 2 grounded -- every claimed parity class is named in parity.md."_ There is no drift and nothing is broken.

**So the actual question is one word: RATIFY OR FOLD.** The class is written, used, and consistent; it is marked _"provisional pending hv"_ and that is the only thing outstanding. It is one of exactly two bullets in that file carrying those words.

**What it is.** `Undefined` = v2 exhibits NO behaviour to be faithful to, so v3 is DESIGNING rather than porting or correcting. **One member, `intent config`**, and the measurement that opened it is the whole argument: v2's `intent config` produces **0 bytes on stdout, 0 bytes on stderr, exit 0**. There is nothing there.

**Why it is not `corrected`.** `corrected` means a v2 behaviour that is WRONG and gets fixed -- it needs an antecedent to correct, and **silence is not an antecedent**. Folding `config` into `corrected` would file a from-scratch design decision inside a bug-fix class, and those want different reviewers: a bug fix is checked against the old behaviour, a design is checked against nothing.

**What ratifying CHANGES, mechanically: nothing.** The row, the instrument and the class list are already consistent. Ratification only removes the "provisional pending hv" marker and closes one of my four open items. **What FOLDING would change: `config`'s row loses the distinction that says nobody has designed this yet, at the moment someone is about to.**

**My recommendation is ratify**, and the reason is the one thing that would actually cost us: **`config` is currently `disposition: pending` -- an honest blank -- and `undefined` is the only marker on that row saying the blank is because v2 was SILENT rather than because we have not looked.** Those two need to stay distinguishable while WP-07 is unbuilt.

---

**SEPARATELY, ON "THE SOONER WE CAN GET THIS PROJECT ONTO v3, THE BETTER" -- here is what actually stands between us and the cut, measured just now rather than estimated.**

**41 of 109 ACs. Four work packages PASS (WP-01 4/4, WP-02 8/8, WP-03 10/10, WP-05 6/6); eight are BLOCKED.**

```
WP-04  4/6     WP-07  0/6     WP-10  1/8
WP-06  6/11    WP-08  0/8     WP-11  2/4
               WP-09  0/5     WP-12  0/4
```

**The shape of the remaining work is one number: 52 of 109 acceptance tests are `to-write`.** Against 32 green, 19 `n/a`, 6 red. **Four whole packages -- 07, 08, 09, 12 -- are at ZERO, and 08 and 09 are the daemon and the MCP surface.** That is not a tail; it is roughly half the ladder, and your 3.0.0 scope statement adds WP-13's nine rows on top.

**Two things I can tell you that the numbers do not.** **WP-11 is 2/4 and both remaining rows were unblocked by your version ruling twenty minutes ago** -- dc has been deliverable-complete and idle waiting on exactly that, so those two should move without new work. **And the six `red` rows are honest reds, not unbuilt ones**: each names the missing arm on its own row, which is why they are red rather than parked at `to-write` where nothing would check them.

**I am not going to tell you it is close. It is not.** But the four passing packages are the foundation ones, and the failure modes we have been finding all day are instrument defects rather than model defects -- **the architecture has not moved under us once today**, which is the thing that would actually cost weeks.

## (2026-08-16 11:55Z) FYI only -- no response needed unless you disagree. TWO FACTS ABOUT THE LADDER THAT CHANGE WHAT "BLOCKED" MEANS, both found by peers today.

**1. AC-09.4 (the agent guide) sits in WP-09 by GROUPING, not by DEPENDENCY** (ic, measured). The work package is titled _MCP server and agent guide_ and the two halves share a title, not a dependency: **the guide needs no daemon, no MCP server and no store -- just the CLI and the dispatch table.** `intent llm guide` is already wired in clap and returns not-implemented. So **WP-09 reading 0/5 overstates what is actually blocked**, and one of its five is reachable now. I have told ic to rewrite their spec under D45 immediately (their file, their charter, no sequencing call needed) but **NOT to start the renderer**, because its one line lands in `render.rs`, which is the hottest file in the tree with cc in it. **That is a coordination fact, not a scope one -- I did not treat your "we will get all of this done" as licence to reorder your ladder.**

**2. AC-11.1/AC-11.4 are NOT unblocked by the version ruling, and I was wrong to tell you they were** (dc corrected me). The version is necessary and not sufficient: **both need a PUBLICATION to exist** -- AT-11.1 is an install transcript from a published tag, AT-11.4 compares the sha of a published artefact. There is no v3 tag or release. **The real gate is two do-not-publish-before issues: 0036** (the unmigrated-project refusal names `intent upgrade`, which v3 lacks) **and 0038**. My board and dc's had each been pointing at a different wrong blocker, and the real one was neither.

**0038 is the one to know about, because it lands squarely on the migration you want fast.** dc measured it end to end: **a migrated project CANNOT COMMIT.** v3 exits 1 for a known-but-unbuilt command, the pre-commit gate reads 1 as "findings" and blocks, so the remedy names a finding that does not exist and the only escape is `--no-verify`. **The gate is not at fault** -- it reserves 2+ for "tooling unavailable" and v2 honours that; v3 collapsed unavailable into your-code-is-bad, so the fail-open branch is correct and never reached. **Combined with 0036, `brew install` shadows the v2 install, so first contact is in a project you were not thinking about and the gate refuses everything in it.** It is now AC-10.9.

**Two criteria added today for defects that lived where NO AC looked** -- 0038 above, and cc's cold-store defect where `ingest::resync` rebuilt seven tables and skipped `event_log`, **the one table derived from nothing, so every fresh clone had no history and nothing reported it.** That is AC-03.11, and **WP-03 goes PASS to BLOCKED at 10/11** as a result. I would rather carry the honest number than hold a green over a question nobody asked. **Both defects were found by a node measuring something else, twice in one day, which is a fact about the contract rather than about either node.**

**Running total: 41 of 111. Four gates were passing this morning; three are now.**

## (2026-08-16 11:58Z) ISSUE 0039 -- THE CANON DECLARES FOUR COMMANDS THAT DO NOT EXIST, AND EVERY CHECK REPORTS AGREEMENT.

**I went looking for the declared-but-not-deserialized class systematically after meeting it a fourth time, and it has a live user-visible instance.**

`dispatch-table.json` declares `aliases` on five entries, four of them `disposition: keep`. **`pub struct Entry` does not have the field** -- not unread, structurally invisible, exactly as `required` was on `Flag` in 0035. Measured against a binary built from the current tree, with a nonexistent thread so nothing could mutate:

```
at green     -> error: this project has not been migrated ...   (wired)
at done      -> error: unrecognized subcommand 'done'          (GONE)
at red       -> error: this project has not been migrated ...   (wired)
at notdone   -> error: unrecognized subcommand 'notdone'       (GONE)
```

**And v2 documents them in its own help: `done|notdone <stid> <atid>   Aliases for green | red`.** These are not obscure spellings -- `green`/`red` describe the row's state and `done`/`notdone` describe what the user did, which is why v2 has both.

**`issues new` and `lang rm` are correct in the table today and will be absent the moment those families are wired**, so the defect count GROWS as the surface is built, and each new instance arrives already reported green.

**THE PART THAT IS WORSE THAN THE BUG: `surface_check.sh` contains ZERO occurrences of `aliases`, and so does `dispatch_ssot.rs`.** The tool whose whole job is checking the binary against the table cannot see this, **because an unknown canon key is not a mismatch -- it is invisible.** Adding a field to the canon silently adds an UNCHECKED field rather than a failing one.

**So the recommendation that matters is not the two commands.** This is the fourth declared-but-not-deserialized field in three files -- `Flag.required`/`accepts`/`default`/`value`, `Entry.exposed_on_mcp`, `Entry.read_or_mutate`, now `Entry.aliases`. **Four fixes have been proposed and none closes the class.** One check comparing the canon's authored key set against the types' deserialized key set, refusing on any key no type reads, would have caught all four before any shipped. **A `keep` row that does not ship is worse than a `retire` row: `retire` is a decision with a ratification, this is an accident with neither.**

-- vc

## (2026-08-16 15:06Z) ANNOUNCE -- ISSUE 0043, CRITICAL: A MIGRATED PROJECT BLOCKS EVERY CLAUDE CODE PROMPT, AND THE LOCKOUT CANNOT BE CLEARED FROM INSIDE THE SESSION. DO NOT MIGRATE THIS REPO UNTIL IT IS SETTLED.

**Two consumers read the same exit code and take opposite decisions from it.**

- The **pre-commit gate** reads `2` as "the critic tooling is unavailable" and **fails open**. Correct -- and it is why 0038 was fixed by moving unimplemented commands from `1` to `2` at `d2b8e76d`.
- **Claude Code's `UserPromptSubmit` hook reads `2` as "BLOCK this prompt".** That is the contract, and our own shipped `require-in-session.sh` uses it deliberately: `:20` documents _"Block (exit 2 + stderr message)"_ and `:71` is a bare `exit 2`.

`.claude/settings.json` wires `UserPromptSubmit` -> `intent claude hook require-in-session`, matcher `""`, ie every prompt. **v3 does not implement `claude hook`. Measured just now: `rc=2`.**

```
$ intent claude hook require-in-session
error: `claude` is a known command that is not implemented yet     rc=2
$ intent claude hook session-context
error: `claude` is a known command that is not implemented yet     rc=2
```

**So in a migrated project every prompt is refused -- and the refusal is self-sealing.** The documented escapes are to run `/in-session` (which needs a prompt) or to `touch` the sentinel path the gate prints (which it no longer prints, because it prints v3's not-implemented message instead). **Neither is reachable from inside the session.**

**This is not a mistake in `d2b8e76d` and I want that said plainly, because cc measured the right thing and reasoned it correctly.** The fix was made against the pre-commit gate, is right about it, and its comment is accurate. **The defect is that an exit code was treated as a property of the TOOL when it is a property of the CALLER's contract, and nothing enumerated the callers.** There are exactly two shipped consumers of `intent`'s exit codes -- `pre-commit.sh` and `.claude/settings.json` -- they disagree about what `2` means, and only one was in view. **Whichever number is chosen globally, one consumer is wrong: `1` breaks the commit gate, which is 0038; `2` breaks the prompt gate, which is this.**

**The detail that decides how to think about it: `require-in-session.sh:26` says _"an unexpected abort would block every prompt."_ The script's author foresaw exactly this failure and defended the only half they could reach** -- the script aborting. Nothing there can defend against the command that INVOKES the script not existing and returning the same code by another route.

**Why this is worse than 0038, which is the reason for an announce rather than an inbox note.** 0038 blocked commits **and left the tool you would use to fix it working.** This blocks the tool. And it lands exactly on hv's plan: the point of migrating Intent quickly is to dogfood v3, **the dogfood is conducted through Claude Code sessions, and this closes them at the moment of migration.**

**It also breaks 0016's hooks-continuity invariant in the most direct way available.** `.claude/settings.json` and `.claude/scripts/**` are byte-identical after migration, exactly as AC-10.4 requires -- **and the sessions are dead anyway.** Same finding as AC-10.9's: byte-identity cannot see a semantic break.

**WHAT I AM ASKING FOR, and none of it is mine to build:**

- **cc**: `claude hook` implemented, and the wider point -- **the unimplemented-command exit cannot be one constant answering to two contracts.** Worth a comment beside `EXIT_UNAVAILABLE` naming BOTH consumers, since a comment naming only the pre-commit gate is how this happened.
- **dc**: this is upstream of publication. A user who installs v3 over a v2 project, per 0036's shadowing, meets this without having asked to migrate.
- **ic**: no action, but the surface consequence is yours if `claude hook` needs a row.
- **hv**: **a scheduling call.** AC-10.9 already carried the note that "WP-10 landing before WP-07 puts every migrated project in this state" -- written about the commit gate, and now true of the prompt gate too.

**ONE THING NOT CLAIMED, and please do not let this issue's severity outrun its evidence:** I have NOT observed the block in a live Claude Code session. It is derived from our own shipped hook's documented contract plus a measured exit code. **The confirmation is cheap -- a throwaway migrated project opened in Claude Code -- and it should be run before anyone acts on `critical`.** I am filing at that severity because the cost of being right and slow exceeds the cost of being wrong and fast, not because the observation is complete.

**Also filed today and related: 0042** -- `intent info` is unimplemented, so the pre-commit hook resolves an empty `INTENT_HOME` and **the whiteboard clock and header guards silently stop enforcing.** Same cause, **opposite failure direction**, which is the argument for enumerating the consumers rather than fixing them one command at a time.

-- vc

## (2026-08-16 20:01Z) Re: 2026-08-16 19:54Z

**FOLLOW-UP TO THE 0043 CONFIRMATION: the rig then answered the question the proposed fix asks.** Item 3 of that fix says enumerate the consumers of `intent`'s exit codes and write them down. **Intent ships THREE Claude Code hooks and only the fatal one had ever been measured.** All three now, same rig:

| Intent hook        | wired command                           | under v3   | effect                                                         |
| ------------------ | --------------------------------------- | ---------- | -------------------------------------------------------------- |
| `SessionStart`     | `intent claude hook session-context`    | **rc=2**   | **does NOT block** -- the session starts, silently contextless |
| `UserPromptSubmit` | `intent claude hook require-in-session` | **rc=2**   | **BLOCKS every prompt**                                        |
| `Stop`             | bare `echo '...wrap-up reminder...'`    | unaffected | **does not invoke `intent` at all**                            |

**Two of the three break, in OPPOSITE directions, and the third was never at risk.**

**`SessionStart` failing open is a finding, not a relief.** Measured with a stub and with the real v3 binary: the prompt runs, the session is usable, `session-context.sh` never executes -- so **the project context it injects, and the `/in-session` reminder that is the documented entry to the whole gate mechanism, silently do not arrive.** The migrated-project experience is exactly: **the session opens with its context quietly missing, and then the first prompt is refused.**

**And `Stop` is clean only by accident of how it is wired.** I measured `Stop` at exit 2 as well: **3s and `PONG` at exit 0, versus 24s and ZERO output at exit 2** -- Claude Code reads `2` from `Stop` as _"do not stop"_. Intent's `Stop` is a bare `echo`, so nothing reaches it. **But routing `Stop` through `intent claude hook`, which is the obvious tidying move, arms a third distinct failure from the same constant.**

**So `2` has FOUR meanings across four contracts, measured, not reasoned: fail-open (pre-commit), block (UserPromptSubmit), advisory (SessionStart), refuse-to-stop (Stop).** Root Cause table in 0043 updated to all four.

**Increment on the 19:54Z confirmation, no decision asked.**

Having built the rig I used it to enumerate all three of Intent's Claude Code hooks rather than the one in the issue. **Two of three break under v3, in opposite directions.** The one nobody had looked at, `SessionStart`, does NOT block -- **it fails silently**, dropping the project-context injection and the `/in-session` reminder. So a migrated project opens a session that looks normal, is missing its context, and then refuses the first prompt.

**This does not change the severity or the hold; it makes the fix's shape clearer.** dc is building one resolver both consumers read, and there are more consumers than either of us had counted.

-- vc

## (2026-08-16 20:26Z)

**ANNOUNCE -- OPERATIONAL, APPLIES TO EVERY NODE THAT MOVES AN AT ROW, AND THERE IS A NUMBER ON IT NOW: 14,253 CHARACTERS.**

**`intent at red|green|na` DESTROYS THE ROW'S NOTE.** That is issue 0033, filed 2026-08-15, and **I walked into it today with the issue in working memory.** Moving AT-03.12 to green took the row **from 1,560 bytes to 106 -- 1,447 characters of authored contract**, including the three arms the criterion required and its explicit refusal of AT-02.8/AT-04.5 as coverage. Recovered from `git show`. **`intent at lint` reported `ok -- 112 rows conform` immediately afterwards**, so the contract's own linter cannot see it leave.

**THE TRANSITION GRAPH MULTIPLIES IT, and this is the part nobody had joined up.** `to-write -> green` is refused; green is reachable only from `red`. **So recording a passing test costs TWO rewrites, not one. The status machine is correct, and its correctness doubles this defect's damage.**

**THE FORWARD MEASUREMENT, which is new and is why this is an announce rather than a note on my board.** Every instance so far was found retrospectively. The same count runs forwards: **112 AT rows, 59 not yet green or n/a, 34 of those carrying a note. 14,253 characters standing directly in front of a status change.**

| row        | status     | chars at risk |
| ---------- | ---------- | ------------- |
| `AT-10.9`  | `to-write` | **3,993**     |
| `AT-06.11` | `to-write` | 1,704         |
| `AT-00.1`  | `red`      | 1,288         |
| `AT-04.6`  | `red`      | 1,172         |
| `AT-06.1`  | `red`      | 990           |
| `AT-00.5`  | `red`      | 938           |

**It does not arrive as one loss. It arrives one row at a time, across WP-04 to WP-14, every time somebody does exactly the right thing** -- writes the test, runs it, moves the row. **The `red` rows are nearer, because a red row is one passing test away from a transition.**

**INTERIM DISCIPLINE, a mitigation and explicitly not a fix: COMMIT BEFORE ANY `at` STATUS CHANGE.** A committed note is recoverable with `git show`. **A note written and moved in the same session is gone.**

**I am deliberately NOT offering a wrapper that preserves the note**, and the reason matters more than the offer: it would be a second way to move an AT status, one safe and one not, **and the one people reach for under pressure is the one the tool ships.** The fix belongs in `intent at`.

FYI only -- no response needed, except where noted below.

**No decision asked, and nothing is blocked on you.** Recorded because it is the kind of thing that is invisible until someone counts it, and because the cost is spent on the build's schedule rather than the fix's.

**0033 has been `high` and correct since 2026-08-15. What is new is that the remaining exposure is measurable and it is 14,253 characters across 34 rows** -- authored reasoning that will be destroyed one row at a time, by the documented command, as WP-04 through WP-14 land. **It is an argument for priority, not for severity.**

**And the instance that convinced me is my own**: I destroyed 1,447 characters today with the issue in front of me, and the tool said `ok` twice and then `112 rows conform`.

-- vc

## (2026-08-17 03:00Z)

**ONE QUESTION, AND IT IS ABOUT YOUR OWN WORDS BEING READ AGAINST A PRINCIPLE. No rush -- it changes one parameter, not a design.**

**D44 as you wrote it:** _"a param that trims the done to (by default) the last 24 hours ... if the user wants a longer done list **in the todo file**"_.

**I ruled the window applies to the TERMINAL RENDER, and the committed `intent/todo.md` carries everything.** My reasoning: a committed file is a record, and a clock-dependent window makes its content depend on when it was generated rather than on what happened -- so a row dropped by the window and a row deleted from the model become indistinguishable in a diff. cc has built it exactly that way (`def9ca44`).

**cc then flagged the thing I had not weighed, and they are right to.** You said _"in the todo file"_. **The practical consequence of my ruling is that an agent reading `intent/todo.md` -- which is the artefact agents actually read -- still sees the full DONE list.** So your stated want is delivered to the terminal, and not to the surface you named.

**I am not settling this by silence, which is why it is here rather than on my board.** Three ways it can go:

1. **Keep my ruling** -- terminal windows, file carries everything. Principle over literal words.
2. **Take you literally** -- the file windows too, and it must then also record WHICH window generated it, or a dropped row and a deleted row look identical in a diff.
3. **Both surfaces window, and `todo.md` stops being committed** -- which is a bigger change and I am naming it only for completeness.

**My recommendation is still 1**, and I would rather you overruled me than that the file quietly did something you did not ask for. **Nothing is blocked on this** -- it is built, tested and shipped as ruled.

**Two smaller things, no response needed.** **0043 is CLOSED** -- cc implemented `claude hook` and `info`, dc verified, I verified independently, and dc lifted the publication hold, so **0036 is now the only hold.** And the D44 unit: `completed` is date-resolution, so a 6-hour window silently means a whole day -- **I ruled the config REFUSES a value finer than the data can honour** rather than rounding silently, keeping the `hours` unit you named.

-- vc

## (2026-08-17 03:28Z)

**While you were AFK: the reverse sweep is run, 0038 is closed, and it turned up one new high that is worth your attention precisely because it is NOT broken today.**

**0045 -- the two gates block on opposite exit codes.** The git pre-commit gate blocks on **1** and fails open on everything else. Claude Code's `UserPromptSubmit` blocks on **2** and passes everything else. Both measured, both working as designed. **The consequence nobody had stated: every non-zero code blocks exactly one of the two consumers, so a command that genuinely cannot answer has no safe code to return.** The only code that blocks neither is `0`, which is the silent-success shape this whole thread exists to prevent.

**And the migration refusal returns 1 -- the one that blocks commits.** `Facade::open` gates on migration before anything else, so every facade-opening command in an unmigrated project returns 1. The shipped hook then refuses the commit with _"blocked by findings at severity >= warning ... review the findings above, fix them"_ over a project with **no findings and nothing to fix.** The real remedy is printed three lines above and framed as the thing to be reviewed.

**It does not reproduce today, and that is the argument for filing it rather than against.** The only thing holding it off is that `critic` is unbuilt and exits 2 into the fail-open branch 0038's own fix created -- **a reprieve nobody chose.** Whoever builds `critic` will reach for `Facade::open` because that is how every other command is written, and nothing in the code, the tests or the specs will warn them. **One line and a comment now; an outage plus a fleet-wide re-install of an untracked hook file later.** Routed to cc.

**No action needed from you on it** -- the fix is a code call in cc's lane and I have recommended the cheap route. Flagging it because it lands in the migration window, which is a scheduling fact and yours.

**0038 CLOSED** on a fresh end-to-end re-measurement rather than a re-read. **0042 reproduced independently and deliberately NOT closed** -- cc's and dc's boards both said CLOSED, the file says OPEN with two outstanding items, and the file is right.

**Under the standing grant, two contract calls, both reversible and both recorded with reasoning.** AC-10.9 widened from _"a migrated project can still commit"_ to _"migrated or NOT"_ -- the old wording inherited 0038's fixture state, so the unmigrated project, which is where every project in the estate sits until WP-10 runs on it, was covered by nothing. AT-10.9 gains a fourth arm and stays `to-write`.

**AND THE ONE QUESTION STILL ON YOUR DESK IS UNCHANGED, because it is about your own words rather than about code.** D44: you said _"a longer done list **in the todo file**"_. I ruled the window applies to the terminal render only, on the principle that a committed file is a record and must not depend on when it was generated. cc built it that way, then flagged the consequence I had not weighed -- **an agent reads `intent/todo.md`, so your want is delivered to the surface you did not name.** My recommendation is still terminal-only. **Nothing is blocked either way** -- it is built, tested and shipped as ruled at `def9ca44`. I am not settling it by silence because it is a principle read over your literal words.

Suite green (rust + bats) noted, thank you -- it means ic's two held table edits can go.

## (2026-08-17 09:39Z)

**ONE RULING NEEDED, and it is a general rule rather than a feature: are SELF-LOOPS legal transitions?**

cc stopped building `intent issues` mutations because `data-model.md` ratifies Machines 1-3 and **no issue machine**, and AC-04.6 requires the implemented graph to match the ratified machines exactly. Wiring `close`/`open` means declaring edges on their own authority. **They were right to stop and to ask** -- the point of a ratified machine is that whoever implements it does not get to add to it.

**The mechanical half is easy and I would rule it under the standing grant: declare Machine 4 from v2's measured behaviour** -- `issues add` enters at `open`, `close` is `open -> closed`, `open` is `closed -> open`, no guards.

**The half I am NOT ruling, because it crosses all four machines, is what cc found underneath it.** v2's `issues close` on an already-closed issue **returns 0 with `already CLOSED`**. v3's Machines 1-3 **refuse** a self-loop -- `intent st done` on a completed thread is an `IllegalTransition`. **Those cannot both be right, and the choice is not an issues detail.**

**Why it is worth your minute rather than mine.** My own v2 transition matrix counted self-loops separately -- **seven undeclared movements, twelve counting self-loops** -- precisely because a self-loop is not a movement. Refusing them makes every idempotent script a special case; accepting them means `st done` on a completed thread succeeds, which is the shape 0046 is about. **The right answer is probably "accept and report, at exit 0, without re-running the guard", but that changes ratified Machines 1-3, so it is yours.**

**Nothing is blocked.** The three mutations report themselves unbuilt at exit 2, which is what they already did, and cc has a test guarding against building them by reflex.

**And the other question is still open, unchanged, because it is about your words rather than code.** D44: you said _"a longer done list **in the todo file**"_; I ruled terminal-only on the principle that a committed file is a record. cc built it that way and then measured something that makes the refusal easier to justify rather than harder: **the cutoff is `date('now','-Nh')` truncated to a date, so at 02:00 a 6-hour window reaches into yesterday and at 12:00 it does not** -- the same configuration produces a different DONE bucket depending on the hour it is read at. Recommendation unchanged: terminal-only.

## (2026-08-17 10:10Z) Re: 2026-08-17 09:39Z

**THE SELF-LOOP QUESTION HAS GROWN A SECOND HALF, AND THE TWO SHOULD BE RULED TOGETHER: WHICH FIELDS GET MACHINES AT ALL?**

cc measured AC-04.6's second condition today and found four fields an entity can enter and never leave -- `Thread.acceptance`, `Criterion.kind`, `AcceptanceTest.kind`, `Issue.status`. All four are values authored canon puts there with no verb to move them. **Measuring that debt is not paying it, and paying it runs straight into the criterion's FIRST condition**: giving any of them an exit means declaring a state machine, and `data-model.md` ratifies three, of which none is these. So cc stopped -- correctly, and for the same reason they stopped on `intent issues`.

**cc's framing, which I agree with: this is ONE ruling covering four rows, not four calls.** The `intent issues` block was never an issues problem; it is the fourth instance of one question.

**MY RECOMMENDATION, AND IT IS SMALLER THAN "DECLARE FOUR MACHINES".**

Three of the four do not look like state variables at all, and cc's own finding is the argument. `Criterion.kind` **cannot move independently**: the kind/state pairing is enforced in the JSON Schema face (`model.rs:414-432`, held by `tests/ac_kind_state_invariant.rs`), so a kind conversion has to move `kind` AND `state` in one act. **A field that cannot move alone is a component of a state, not a state.** `Thread.acceptance` is `Option<AcceptanceMode>` -- `exempt` or absent -- which is an attribute. `AcceptanceTest.kind` has `Criterion.kind`'s shape.

So:

- **`Issue.status` -> Machine 4**, exactly as I proposed at 09:39Z. This is the one real lifecycle, and it is the one with a real verb family behind it.
- **the two `kind` rows -> WIDEN Machine 3** over the (kind, state) pair rather than adding machines. ic independently hit this from the register side: they have no notation for a multi-field atomic move and are recording it as a constraint on the verb. **That gap is diagnostic, not clerical** -- a verb that must move two fields atomically is a verb over a compound state.
- **`Thread.acceptance` -> immutable after creation.** No machine, no edge; changing it is authoring, not a transition.

**That is one new machine and three rulings, rather than four machines. If it is right, most of what currently reads as blocked was never owed.**

**WHY I AM NOT RULING IT.** Machines 1-3 are yours; declaring more on my own authority is the same class as the Machine 4 call I escalated at 09:39Z, and ruling three of four while escalating the first would be incoherent. The self-loop question is the same rule seen from the other end -- one asks what edges a machine carries, this asks which things are machines -- so ruling them separately risks two answers that do not compose.

**NOTHING IS BLOCKED AND NOBODY IS WAITING IDLE.** The mutations report themselves unbuilt at exit 2, which is what they already did; cc has a test that panics if a fifth such field appears without a measuring arm; AT-04.6 stays red with its reason corrected to say four owed mutations rather than the missing measurement it was still claiming (`9e189ee7`). **What a ruling changes is what gets BUILT** -- four machines is materially more surface than one -- so it is worth your minute before anyone starts.

**Unchanged and still on your desk: self-loops (09:39Z) and D44's window (recommendation still terminal-only).**

**FYI, no response needed:** issues 0040 and 0041 are closed on measurement (`4ff5a829`). Your `st_prefix` retirement ruling is discharged in all three parts, and the migrator does more than you ruled -- it BLOCKS a non-`ST` prefix rather than merely naming it, which is what actually removes the success-over-an-unseen-estate shape. Closing 0041 turned up a new one, filed as **0047**: renaming a status display arm reds neither surface, because a machine ratification moved `st new` to `Triage` and silently defanged the assertion that still reads as though it pins the vocabulary.

## (2026-08-17 10:38Z) FYI only -- no response needed.

**ANNOUNCE: THE Bash TOOL RUNS zsh 5.9, NOT bash, AND IT HAS BITTEN TWO NODES TODAY IN OPPOSITE DIRECTIONS.** Both produced a confident, plausible, wrong measurement from an instrument that was silently broken.

- **No word-splitting on unquoted expansion (vc).** `c="st list"; set -- $c` gives `$# = 1`, not 2. A probe loop written as `$BIN $cmd` passes the whole string as ONE argv element, so every multi-word row answers `unrecognized subcommand 'st list'` -- **which is exactly what a surface where nothing is implemented looks like.**
- **`path` is a special variable tied to `PATH` (dc).** `while read -r want path` destroys the search path on the first iteration, `shasum` then cannot be found, and every comparison fails -- **a broken instrument reporting maximum alarm.** One step from filing an issue saying the whole vendored tree had been modified.

**THE EXPOSURE IS INLINE ONLY.** Every parity tool carries a bash shebang and is executed, so it word-splits correctly and its `path` is local. **The hazard is the interactive prompt -- which is where we all take our first measurement of anything, and where a result is most likely to be believed and least likely to have a control beside it.**

**The pair covers both failure directions, which is why it is worth one message rather than two.** dc's rule: a wrong zero certifies absence, a wrong maximum certifies catastrophe, **and the second is far more persuasive because it looks like diligence rewarded** -- nobody re-checks an instrument that has just found something big. vc's produces the plausible zero; dc's produces the alarm. **A control that fires in the known-good direction is the only thing that separates either from a real finding.**

Practical: quote or use arrays for multi-word command paths; never name a loop variable `path`; and prefer a script with a bash shebang over an inline loop for anything whose result you intend to write down.

## (2026-08-17 10:57Z) Re: your "Some test regressions" report

**THE RED IS NOT A REGRESSION. Nothing that has landed is broken, and you can stop worrying about it.** You ruled the legs to cc/ic and I have not touched them -- this is the diagnosis only, so they are not chasing a ghost.

**Measured, not inferred:**

| what                                                        | result                                                     |
| ----------------------------------------------------------- | ---------------------------------------------------------- |
| `78a12dce` (`git archive` extract, own target dir)          | **green, 21 passed 0 failed** -- the failing test included |
| `b2173b1b` (dc, detached worktree, independent method)      | **green**, same two legs                                   |
| `crates/intentsvcs/src/facade.rs` vs the rust leg finishing | modified **+41s AFTER**                                    |
| `crates/intentsvcs/tests/close_gate_parity.rs`, same anchor | modified **+48s AFTER**                                    |

So the leg compiled a tree that stopped existing 41 seconds later. The assertion that panicked -- `"the same gate refuses once the coverage goes red"` -- is **the one your own self-loop ruling retires**, and cc's tree already rewrites it to `Outcome::AlreadyThere` citing that ruling. The format leg is the same class nine minutes earlier: it caught cc's `Outcome` refactor before it was formatted.

**What I did NOT verify, so you do not read more into this than it carries: whether cc's CURRENT tree passes.** That is one command and it is theirs.

**FILED 0049 (medium, `304cf05b`), because the real defect is not that the tree moved -- it always will on a five-node estate.** It is that the run pair records **neither which tree it measured nor whether that tree survived**, so a stale verdict is indistinguishable from a live one by inspection. **The four stale GREENs from that same run are the worse half** -- a red gets read carefully, a green certifying a tree nobody has never gets questioned. That is dc's asymmetry pointed back at the instrument that produced it.

**Filing it caught me in the same defect, in writing, inside the hour.** I told cc "HEAD (78a12dce) is GREEN" -- true when sent, false forty minutes later when cc landed `b2173b1b`. dc hit the identical thing in the same window against their own sha and flagged it unprompted. **`HEAD` is a pointer, so a claim about it is a claim about whatever it points at WHEN READ -- and `rust` is a pointer at a tree in exactly the same way.** That is the whole of 0049, and pinning the sha protects the measurement while only naming the commit protects the report. The issue itself was refuting itself in three places on first writing; fixed in the same commit, with the wrong sentence left visible as its own evidence.

**Proposed fix is two separable parts, annotate-never-suppress:** stamp `git rev-parse HEAD` + the dirty list at run start so a verdict names its referent, and `find <scope> -newer <run-start anchor>` at seal so it also says whether the tree moved. Part 2 needs no clock and cannot false-positive on a quiescent tree; the mtime idiom is already in-house at `runlog:665`. **It narrows the window and does not close the class**, which the issue states rather than papers over.

**It lands in `bin/.devbin/**`, so it is dc's call (0048) and I filed rather than built.** Nothing here needs you before you are back.

## (2026-08-17 12:39Z) FYI only -- no response needed.

**CORRECTING THE SELF-HOSTING ANSWER I GAVE YOU, because it was measured on one half and stated of the round trip.**

I said the read half is done and the write half does not exist. **The write half exists and it corrupts our own estate on first use.** Issue 0056 (ic's, confirmed independently by me): `legacy.rs:608` ingests `"n/a" | "n-a" | "na"` liberally; `views.rs:461` emits only `n-a`. So **`sync --to-disk` rewrites 23 rows of this repository into a token v2's linter refuses at L1 -- and L1 BLOCKS the close gate, while v2 is still the only tool that can close a thread.** Twenty rows are in ST0056 and three are in `COMPLETED/ST0054`, so it reaches threads nobody is editing.

**So the answer is not "near on feasibility, far on volume". It is that the first migration of our own estate silently corrupts it**, in a direction `intent at lint --fix` cannot repair, because the generator puts the token back.

**The `0 blocking / 9 carried / exit 0` ingest figure was CORRECT and stays correct** -- and the liberal reader is exactly why it could not see this. A tolerant parser removes the only signal on the way in, so the damage is created on the way out and found by a third tool. My error was the layer, not the measurement: **I measured one half and reported the system.** Second time on this thread.

**Nothing needs deciding from you on it** -- the fix is one method (`AtStatus::display()`) at two call sites, it is cc's, and it is unblocked. This is a correction to the record you read, not an escalation.

**Still pending you, and nothing else is: the five provisional-vc items** -- migration policy, D43, D46, the `new-surface` out-of-scope ruling, and the withheld-13 field ruling. The last of those clears six `surface:` lines from `intent doctor`'s own output.

## (2026-08-17 16:09Z)

**Three decisions with no holder, and one of them is new since you left. Nothing here needs a reply to unblock me -- I am building. It needs a reply before the hoist lands.**

**1. THE CONTRACT ROLE IS GENUINELY UNOWNED, AND IT IS A HOLE RATHER THAN A DEFERRAL.** You put me on the tools; I am on AC-10.5 and the split with cc meets your not-checking-my-own-homework condition structurally. But the contract work I was doing did not move to anyone. I offered it to ic and they declined correctly -- you had already put them on the migrator, and a peer cannot re-task them against that. So the five provisional-vc items (migration policy, D43, D46, the `new-surface` out-of-scope ruling, the withheld-13 field ruling) plus `ac gate`'s ratification have no holder while all four nodes build. They are enumerated with their homes in `intent/st/ST0056/deferred.md`. **The decision is not "who does the work" -- it is whether these wait for the hoist. I think they can. I want that said rather than assumed, because five provisional rulings quietly ageing into settled canon is how a contract stops meaning anything.**

**2. NEW, FROM dc, AND IT BITES ON HOIST DAY IN THE DIRECTION NOBODY GUARDED.** Measured, not reasoned: a project declaring `intent_version: 3.0.0`, driven with **v2** on PATH.

```
intent upgrade   exit 1   "refusing downgrade: project is at v3.0.0, target is v2.19.0"
intent st new    exit 0   and writes SEVEN files
intent todo      exit 0   and writes
intent doctor    exit 0   "doctor: intent v2.19.0" -- notices nothing
git commit                LANDED through the shipped gate
```

**v2 knows the project is from the future -- it says so in those words -- and applies that knowledge at the one command that was already safe.** The asymmetry: v3-on-a-v2-project REFUSES, v2-on-a-v3-project WRITES. The guarded direction is the harmless one. And the unguarded direction is the state every session on this estate enters the moment the hoist lands: `which -a intent` returns three v2 entries and v3 is deliberately off PATH. In v3 the markdown is a generated view, so those writes are discarded at the next regeneration without anything reporting a loss.

dc's framing, which I am passing on in their words rather than mine: **the migrator converts the PROJECT end and nothing converts the TOOL end** -- a two-ended migration done at one end. **AC-10.9's subject is "a project can still COMMIT with v3 installed", so the v2-installed case is OUTSIDE that row, not inside it** -- dc's sharpening of my first wording, and it is the better one. The row appears to answer "can a project still commit" and answers it for only one of the two binaries, which is exactly how a reader six weeks from now reads it as covered. The gap is a gap, not a wording quibble. **Neither dc nor I are proposing a criterion -- the moratorium is yours and it holds. Whether this needs a row, or is only a sequencing fact for the cutover, is your call.**

**3. THE ONE POLICY QUESTION cc AND ic BOTH REFUSED TO EXTEND BY ANALOGY.** Your carry ruling covers a CLOSED **thread**: an unreadable row blocks in live work and carries in closed. Does it reach a CLOSED **issue** with an unreadable field? cc has told ic to BLOCK rather than carry as the conservative default and put it to you to widen. Both of them stopped at the boundary rather than reasoning across it, which I think is right and worth saying out loud.

**Where I am, so this is not just a list of asks.** AC-10.5's corpus and denominator landed at `07b6e08a`. The canary is pinned at `42fb5269` -- chosen by a stated criterion, not by being newest: it is the last revision where `intent/issues/` populates BOTH arms (23 OPEN + 38 CLOSED). At HEAD the OPEN arm is empty, so a corpus captured there would ship the migrator's BLOCK path unexercised while every count reconciled against zero. dc found that.

The split with cc had its first cross-check and it agrees: my census reads 56 threads / 140 WPs / 281 criteria / 228 tests / 1077 files, ic's independent Phase A read 56 / 140 / 283 / 230, and the +2/+2 is entirely two AC rows landed after the pin. Two parsers, two languages, two authors, agreeing exactly once the revision is held fixed.

**Three conservation findings are with cc, all measured on that corpus.** The `body` catch-all that carries unnamed sections is on work packages and not on threads, so 178 thread sections and 106,678 bytes have no field to land in -- including `## Related Steel Threads` in 55 of 56 threads while `related` is hardcoded empty. Work-package section ORDER is lost in 140 of 140, because the reassembly is alphabetical. And ic's issue hole now has a size: 61 issues, 503 sections, 641,893 bytes, read by nothing.

**FYI only unless you want to rule -- I am not blocked on any of the three.**

## (2026-08-17 16:16Z)

**A fourth decision, and it is the one I would rank first. ic found it, ic asked me to carry it so you get one voice on one question, and the framing below is mine.**

**THE WARRANT FOR RETIRING `intent organize` HAS AN UNBUILT PRECONDITION, AND THE PRECONDITION IS WP-10.**

You retired it on 2026-08-14 with this reason, verbatim from the dispatch table: _"organize (both faces) is planned vestigial by construction; a strictly structured model cannot hold data in the wrong spot or the wrong format, so the disorder this repairs cannot arise."_

**That sentence is true of a v3 estate. The migration is what turns a v2 estate into one. As built, it does not.**

v2 expresses a thread's status as a DIRECTORY -- 55 of 56 threads live at `intent/st/<BUCKET>/<ID>/` while v3's canonical path is `st/<ID>/`. Phase A walks the buckets correctly. But it discards WHERE it found each thread: `model::Thread` has no location field, so Phase B cannot move a thread even in principle. The migration writes fresh canon and regenerated views at the flat path and leaves every bucketed file exactly where it was.

**MEASURED, AND THE TWO NUMBERS ARE THE WHOLE FINDING:**

```
plan() on the canary   311 planned files (56 thread.json, 1 event log, 254 views)   and reports NO BLOCK
the buckets             386 files stay exactly where they were
```

**Both figures are correct. The migration is complete by every measure it takes of itself.** That is what makes this a ruling rather than a bug report -- there is no failure to point at. I verified the bucket split independently at my own pin and got 387, decomposing as 194 regenerated at the flat path and 193 authored files nothing regenerates; ic read 386 = 194 + 192 at HEAD. Neither of us adopted the other's number: both are right at their own anchor and the one-file delta is the estate moving between them.

**THE HALF THAT WORRIES ME MOST IS NOT THE LOST PROSE.** The 193 authored files (`design.md`, `impl.md`, `tasks.md` and the one-offs) end up as markdown in the repository at a path the model does not point at -- bad, and visible once someone looks. The other 194 are worse and quieter: **the migration MANUFACTURES the 0011 class.** Two `info.md` per thread, one generated from the model and one v2 artefact that nothing regenerates and everything still reads. Two artefacts claiming one natural role at two paths, on 55 of 56 threads -- and `legacy.rs` is built to REFUSE exactly that when it finds it in an estate. The migration would create what the parser exists to reject.

**WHY IT NEEDS YOU AND NOT US.** Flattening means MOVING authored prose, and `WriteSet` has only `add` -- no remove. So the mechanism is a decision (does `WriteSet` gain a move? does the facade delete after the batch commits, with rollback restoring?), not something to be quietly patched into the join. **ic recorded it in `migrate.rs` as a named hole and deliberately did not build around it, which I think is exactly right.** Nobody is blocked: ic is on `plan()`, cc is on the scanner, I am on the conservation checks, and the checks now REPORT this rather than pass it. But the hoist should not land with the disorder carried in wholesale and the tool that repaired it already gone.

**Not proposing a criterion. AC-10.5's prose conservation is already the right row; this changes the shape of the check, not the contract -- and it changed it before I built the check, which is the cheapest moment it could have happened.**

## (2026-08-17 16:39Z)

**Item 2 of my earlier batch is DISCHARGED and I am striking it myself rather than leaving it to read as pending.** You ruled _"Ok, just fix it"_ and the fix landed at `53f88757` -- v2 now refuses a project from the future at the commands that WRITE, not only at `intent upgrade`. Nothing is owed on it. The other three (the unowned contract role, the closed-ISSUE carry question, and `intent organize`'s unbuilt precondition) are still open.

**AND ONE NEW DECISION, MEASURED, WHICH cc IS ALSO PUTTING TO YOU FROM THEIR SIDE. One voice would be better than two, so this is the measurement and cc holds the ask.**

**`WpStatus` HAS THREE VARIANTS -- `NotStarted`, `Wip`, `Done` -- AND NO `Cancelled`. `ThreadStatus` HAS SIX, INCLUDING IT.** Ten work packages across the captured fleet are cancelled and **cannot be represented in v3 by any spelling**. No match arm in the parser can fix it; it is a model question and a ratified machine, so cc declined to add the variant on their own reading and I am not ruling it either.

The measurement it came out of, for context on how big the rest is: **2140 work packages across the four fleet estates, 131 of which the parser cannot read -- 122 in closed threads (CARRY) and 9 in live ones (BLOCK).** The whole fleet's migration blocks on nine work packages, eight of them `proposed` in live Lamplight threads. That is a small and fixable number. cc has taken three of the four findings as fixes; this is the one that is not theirs.

**Why the fleet was captured at all, since it was not asked for.** AC-10.5 names four members and only the Intent canary existed. All four are now captured and verified byte-exact -- 7172 files, 436 threads, 2140 WPs, 20106 authored prose sections, **67 live threads against the canary's 2**. **Intent is the least representative member of its own fleet**: it is the one estate maintained by the people writing the parser, so its data is the data the parser was shaped around. **Run only against the canary, the migration reports clean.** Every finding above came from an estate we did not write.

## (2026-08-17 18:36Z)

**One new contract question, and a state summary. The question is canon HYGIENE, not design -- nothing in the model is in doubt.**

**A VOID PREMISE SITTING NEXT TO A LIVE CONCLUSION HAS NOW CAUGHT THREE OF FIVE NODES IN THREE SEPARATE PLACES, WITHIN ONE HOUR OF EACH OTHER.**

When you reversed D01 on 2026-08-15 -- _"the db is the SSOT and it's the FILES that are re-creatable"_ -- D29's derivation died with it. `design.md:243` says so correctly and explicitly: _"The conclusion stands; its derivation is VOID and is replaced here rather than reworded."_ The struck text is still on the page, immediately under a conclusion that still stands, in a file that is otherwise authoritative.

Today: **ic cited the void derivation to dc. I cited the same void derivation to dc an hour later, plus `"rm intent.db is always safe"`, which is on D01's do-not-cite list word for word. cc separately found the same dead reasoning still LIVE in code at `sync.rs:132`** and declined to edit canon in another node's lane.

**None of the three was careless, and that is the whole point.** All three of us went and found a canon citation before proposing a change -- the behaviour the project wants. **The citation we found was the dead one, because it reads as a sentence.** A strikethrough survives careful reading and does not survive fast reading, and both of us proposing a change were reading fast because we were proposing a small one. ic's diagnosis, which I am adopting: **a void premise adjacent to a live conclusion is close to the worst possible shape for something nobody is supposed to cite.**

**Both of us, independently, proposed removing the SSOT from the cutover gate's subject.** dc refused both. That is how far a dead premise carries.

**MY OWN RECORD ON THIS IS THE WORST OF THE THREE AND YOU SHOULD WEIGH IT.** `design.md` already records that the reversal was _"hv's FOURTH statement of it and vc's third failure to take it"_. **Today was my fourth.** I have found and fixed the mechanism rather than resolving to do better: my cross-session memory file still carried the PRE-REVERSAL truth model verbatim, so a stale premise was being correctly re-supplied into every session I start. That is fixed. **But three nodes hitting the same page is not a memory problem, it is a document problem, and the document is mine.**

**I am NOT proposing a change and I am not touching it without you.** The options are real and they trade off differently -- delete struck derivations outright and lose the audit trail of what was believed; move them to a graveyard section; mark them mechanically so a grep can refuse them; leave them and accept the cost. **`design.md` is the ratified record of how decisions were made, so deleting reasoning has a real price and it is not mine to pay.** Your call on whether this is worth a fold at all.

---

**STATE, briefly. The migration now runs end to end through the command an operator types.**

`intent upgrade` landed (`8770cea3`) and the whole fleet has been through it. **Canary, Utilz and Baize convert; Lamplight REFUSES at exit 1 with the per-line classed report, and re-verifies 5613/5613 against its pin afterwards -- so AC-10.2 is measured satisfied on all three arms, with atomicity proven by recomputing every hash against git's own record.** ic separately ran the cutover gate on a commit: a real SIGKILL at 293 of 295 writes, and the re-run reached every canon file.

**The fleet earned its keep twice more today, both times on estates we did not write.** Lamplight's 54 live threads fired the BLOCK arm for the first time ever -- until today every measurement any of us had taken was of the success path. And **Utilz declares 2.18.0 and was MIGRATED rather than refused**: the version floor is enforced on the door that READS and not on the door that WRITES. cc has built and mutation-tested the fix.

**Four open decisions of yours, unchanged**: the unowned contract role, the closed-ISSUE carry question, `intent organize`'s unbuilt precondition, and `WpStatus` having no `Cancelled` (cc holds that ask). This canon-hygiene question is a fifth, and it is the least urgent of them.

## (2026-08-17 18:39Z) Re: 2026-08-17 18:36Z

**Sharpening the canon-hygiene item I sent you twenty minutes ago, because two peers have improved it and one of the improvements shows my framing was the weaker half. The count is now FOUR instances, not three, and the fourth is the one that decides it.**

**I framed it as a DOCUMENT-SHAPE question** -- how should canon record a replaced derivation, given a strikethrough reads as a sentence. **dc reframed it as a PROPAGATION question and that is the one that would actually have prevented today: _"a strikethrough in `design.md` does not reach a node that never re-reads `design.md`, and the current mechanism is that three people remember."_** Mine is about the page. dc's is about the distance between the page and the reader. **I did not misread the strikethrough -- I never re-read the page at all, and neither did ic.** Credit for that half is dc's; I am relaying it rather than absorbing it.

**THE FOURTH INSTANCE IS cc's AND IT REMOVES THE LAST COMFORTABLE READING.** `design.md:243` names cc in D29's own correction -- _"flagged by cc, who found the same dead reasoning at `sync.rs:132`"_. **cc found that void derivation, correctly declined to edit canon in another node's lane, and then four days later rested a committed doc comment on the adjacent dead ruling**: `upgrade_command.rs` excluded the store citing "D34/D36" for it being "per-machine and **rebuilt**", when **D36 is your ruling that the DB is NOT disposable** (_"`rm intent.db` -- Why would anything in Intent EVER do this?"_). I verified both citations at source rather than taking cc's report. cc's own words: **finding a struck premise once buys no immunity to the next one.** So this is not three nodes being careless in one hour; it is four instances across four days including the person who found the problem.

**AND cc SUPPLIED THE EVIDENCE THAT MAKES IT AN ARTEFACT PROBLEM RATHER THAN A DISCIPLINE ONE: `design.md:243` is roughly 1400 words in which the word VOID appears ONCE, mid-paragraph, with the replacement grounds immediately after it. Nobody skimmed.** Every one of the four went looking for a canon citation before proposing a change -- the behaviour you want -- and found the dead one, because at reading speed a struck derivation is a sentence in an authoritative file underneath a conclusion that still stands.

**THE ARGUMENT I WOULD PUT WEIGHT ON, AND IT IS NOT A NEW PRINCIPLE -- IT IS D33 APPLIED.** "Read the whole 1400-word entry before citing any part of it" is the same SHAPE of instruction as "remember to stamp from the clock", and **you have already ruled on that shape**: D33, _"we should never ever get into this clock nonsense again... leave all timestamping up to the db"_ -- a hand-held rule replaced by a mechanism, and the hand-authored rules DELETED once it exists. **The precedent is yours, it is recent, and it is about exactly this class.** I am not claiming the remedy is the same, only that the project has already decided this class does not get solved by asking people to be more careful. **My own case is the proof: I have now failed on D01 four times, and the thing that finally fixed it was a file, not a resolution.**

**STILL NOT PROPOSING A CHANGE, AND I HAVE TOUCHED NOTHING.** The options trade off against a real cost -- `design.md` is the ratified record of HOW decisions were made, so deleting reasoning loses the audit trail of what was believed and why it failed, which is a large part of what makes it worth having. What I would want your view on is narrower than the whole question: **whether a struck derivation should carry a marker a grep can find.** That keeps the record complete, costs one token per instance, and is the only version of this that a tool could ever enforce. Everything beyond that -- graveyard sections, deletion, restructuring -- is a bigger call and I have no recommendation.

## (2026-08-17 18:47Z) FYI only -- no response needed.

**Two findings today have the same shape and I am naming it once rather than twice, because if you rule on it, one decision covers both.**

**THE SHAPE: the migrator is silent about something the contract says it reports.**

- **Artefacts (AC-10.5).** The canary's migrator report is 9 carried findings, while the estate loses 61 unread issues, 192 files that are the only copy of authored prose, and 178 thread sections with no modelled field. **My check named those; the migration did not.** AC-10.5 asks the MIGRATION to name its residue, so the row stays `to-write` until the migrator's own report reconciles against the census -- not until my findings reach zero, which is a different and wrong test.
- **Provenance (AC-02.8b).** `threads.created` is populated on 56 of 56 threads and the `event_log` holds **zero rows**. The value is carried straight from v2 frontmatter. The ratified form of that row says `created` is the `ts` of a RESTORED `st.new` event -- adopted over the plain-column form precisely because it is your rule literally rather than a proxy. **ic found it; I verified it from the store rather than from their report.**

**WHY THEY BELONG TOGETHER: in both cases the cheap check PASSES.** Every count reconciles; every value is correct. **What is missing is the migrator saying what it did**, and no predicate that reads the end state can see the difference. One ask covers both: **the migrator's outputs must reconcile against what the contract claims it emits.** That is one decision, not two, and I would rather you saw the shape than two rows.

**Both are annotated on their rows with the measurement, and neither is blocked on you** -- cc has the queue for the first and the second is an implementation cc and ic have between them. Raising it only because the two together are a pattern and either alone reads as a gap.

**One correction to my own canon-hygiene item from earlier, and it comes from ic.** I framed the failure as _believing a premise we found_. ic supplied the inverse from the same day: they went to `migration.md`, found no mention of `st.new`, and were a step from telling dc the contract said nothing about events. **Absence in the first file you look in is not absence.** Same root -- treating one file as the boundary of what canon says -- from the opposite direction. It strengthens the case for a mechanism rather than adding a second item, so I have folded it in rather than filing separately.

## (2026-08-17 19:06Z)

**Your six rulings are recorded as D47-D52 and the `[VOID]` mechanism you authorised is built. One new thing needs you, and it is a cutover blocker.**

**v3 migrates a project successfully and then refuses to read or write it. It affects every v2 project that has ever had a single steel thread.**

**You can check this yourself in a scratch directory in four commands** -- no corpus, no capture, no build. I re-ran it rather than taking it from ic:

    intent init X ; <v3> upgrade ; <v3> st list          -> exit 0, table renders   POSITIVE CONTROL
    intent init X ; intent st new "a thread"
                    <v3> upgrade   -> exit 0, "migrated: 1 thread(s), 6 file(s) written"
                    <v3> st list   -> exit 1, REFUSES

**The population is wider than "mature projects", and this is ic's correction to both of us**: v2's `intent st new` writes to `intent/st/NOT-STARTED/ST0001/` **from creation**, so a project four commands old breaks on its first thread. The only estate that survives is one with zero threads -- which is why neither of us could find a bucket-free control to test against. **It is not scarce; it is unconstructible with any content.**

**On the real fleet, every read verb and `st new` refuse on all three migrated members.** Clean `git archive` extract at `b79e06de`, commit recorded beside the binary, never on PATH. Only `info` answers, and it reads no project state.

    member         migrated  bucketed  BOTH  bucketed-only
    Utilz                 9         9     9              0
    Baize                25        21    21              0
    Intent's estate      56        55    55              0

**`bucketed-only = 0` everywhere. Not one thread the tool names as "carrying v2 canon this binary cannot read" is actually unmigrated** -- every one has a `thread.json` written by the migration now being refused. **The refusal is 100% false positives.** The detector asks a per-DIRECTORY question ("is there a `thread.json` beside this `info.md`") where the real question is per-THREAD ("is this id in migrated canon anywhere"), and the stale v2 copy sits one directory below the migrated one. **And the remedy the error prints is a no-op that claims success**: a second `intent upgrade` exits 0, reports `311 file(s) written` and `ok: this project is now Intent v3.0.0-dev`, and the estate still refuses.

**YOUR D49 RULING IS THE FIX, NOT THE PROBLEM, AND I AM NOT RE-OPENING THE BUCKET QUESTION.** You said threads and WPs _"just are"_ and that sync puts them in the right spot by definition. **The migration already does that half correctly** -- every thread lands at the canonical flat path. What it does not do is stop the stale v2 copies being read back as unmigrated canon. This is a defect in a detector, and your ruling describes the world in which that detector is trivially right.

**Why you are hearing it rather than just being told it is fixed: you ruled the adjacent question an hour ago without this in front of you, and it changes the priority, not the answer.** You called the bucket question not a priority -- correct about conservation and about `organize`, and not correct about this. **Nothing is needed from you unless you want to reorder something.** cc owns the `project.rs` fix and has it top of list; ic will re-drive the gate on two estates once it lands; I will not verify it only on the estate that found it.

**One thing on my own harness, which is the honest half.** My fleet run recorded canary, Utilz and Baize as CONVERTING, and every conservation number still reproduces -- ALTERED 0, ADDED 0. **But nothing in that run ever asked whether the tool could still open the estate afterwards.** ic's framing is the right one and I have taken it: **a conservation green sitting on top of a liveness failure, where the two instruments cannot see each other.** Mine asks only whether the bytes survived. That is a gap in my harness rather than an error in its numbers, and I am closing it with a read verb after the convert **asserted on OUTPUT, not on exit code** -- the exit code is exactly what a lockout can fake.

## (2026-08-17 20:15Z)

**Asking you to re-rule D50 (`WpStatus` gains `Cancelled`), because the question I put to you was wrong and it produced the right answer to it.** cc found this going to write the match arm; I have verified every claim at source myself rather than relaying it.

**MY FRAMING IS WHAT CAUSED IT.** I told you _"ten work packages across the fleet have no representable status"_. **That is true, and it is a FLOOR, and it invited exactly the answer you gave** -- _"Then add it. Why is that so hard? It's another value on the status enum?"_ **Which is the correct answer to "we need one more value" and does not reach the actual situation.**

**MEASURED, and I counted it independently of cc by a different method and got the same total.** Lamplight's work-package rows that cannot be expressed as not-started / wip / done even after normalising case and separators:

    cancelled 10   todo 5   pending 4   superseded 3   relocated 2   proposed 2
    planned 1  phase-2a-complete 1  partial 1  moved 1  future 1  dropped 1
    closed 1   absorbed 1
    ------------------------------------------------------------
    TOTAL 34, of which cancelled is 10

**So `Cancelled` fixes 10 and leaves 24.** And `Superseded`, `Absorbed`, `Relocated`, `Dropped`, `Partial` are **not spelling variants of anything in the enum** -- they are distinct ideas with as good a claim to being a state as `Cancelled` has. **The problem is not a missing variant. v2's WP status was free text, and an enum cannot grow to 29 spellings.**

**AND THERE IS A RATIFIED DECISION POINTING THE OTHER WAY THAT YOU WERE NOT SHOWN.** `data-model.md:454`, in the set you ratified on 2026-08-15: _"No `Hold` or `Cancelled` at WP level is proposed -- a WP that stops mattering is a scope change on the thread, not a state on the package."_ `transitions.rs:379` transcribes it. **Neither was in front of you when you ruled**, and I did not put it there because I did not know it was there -- cc found it only by going to write the code.

**It is also not one variant even for the 10.** My own ratified rule (`data-model.md:383`) says **no terminal states: every state has at least one declared exit.** So `Cancelled` costs a verb to ENTER it, **a verb to LEAVE it**, a row in the ratified transition table, surface rows in the dispatch table, and three transcribing tests -- for 29% of the problem.

**WHAT cc PROPOSES AND I AGREE WITH, offered as a recommendation and not a decision: a `status_legacy` companion, exactly like the `scope_legacy` that already exists.** Verified: `model.rs:314` carries `scope_legacy: Option<Legacy>` for precisely _"v2 recorded a value the v3 vocabulary cannot express"_, and **`status_legacy` does not exist at all.** That asymmetry is the real gap. It **covers all 34 rather than 10, needs no machine change, reverses no ratified decision, and puts the tail where a tail belongs: visible, marked, and not silently canonicalised into a lie.** It is also the same shape as a finding I sent cc earlier -- two structurally identical fields, one guarded and one not, and the unguarded one is not less likely to break, only less likely to be looked at.

**THE CAUTION, AND YOU SHOULD HAVE IT BEFORE THE NUMBERS PERSUADE YOU: THE ENTIRE TAIL IS ONE ESTATE.** cc measured Lamplight only; Baize and Intent are clean (three spellings each, all mappable) and Utilz has no work-package directories at all. **So _"the fleet needs 15 concepts"_ and _"Lamplight is unusually messy"_ produce exactly the same table.** That does not change the arithmetic against the 10 -- those are Lamplight's too -- but the case rests on one member and I am not going to let my numbers imply four.

**Nothing is blocked and nobody is overriding you.** cc has stopped on `WpStatus` rather than building either option, is doing the `covers` fix meanwhile, and has said plainly that if you still say add it, they will add it and build both verbs. **I am bringing you in because you asked to be brought in when needed, and a ruling made without a ratified decision that contradicts it is the case.**

## (2026-08-17 20:26Z) Re: 20:15Z FYI only -- no response needed.

**One precision on the D50 measurement, in the direction that WEAKENS my own case, because you are deciding on it.**

I told you I counted Lamplight _"independently of cc by a different method"_. **The EXTRACTION was genuinely different -- cc parsed work-package rows, I grepped `status:` out of `info.md` files -- but we normalised IDENTICALLY (case, `-`, `_`), which cc has just pointed out.** So the two counts agree on extraction and share a normalisation. **Two instruments sharing a rule are not two independent checks of that rule**, and if the folding were wrong we would both be wrong the same way.

**I have looked and I do not think it is wrong** -- the folding maps `not started` to not-started and `complete` to done, which are the only two judgement calls in it and neither is close. **The 34 and the 10 stand.** But "two nodes measured it" is worth slightly less than I implied, and you should have that before you weigh it.

**Also correcting a smaller thing I said to cc and not to you**: I attributed our differing spelling counts to cc keeping case variants apart. They folded case too. The raw-spelling counts differ (29 raw against my folded set) because we counted different things; **the inexpressible totals match exactly rather than approximately.**

## (2026-08-17 21:08Z)

**THE HOIST WORKS, AND HERE IS THE NUMBER YOU ASKED ABOUT WITH THE CONTROL THAT MAKES IT HONEST.**

    engine   verdict                          time
    v2       46/114, 68 unsatisfied          95.38s
    v3       46/114, 68 unsatisfied (SAME)    0.02s

**IDENTICAL SCORE AND IDENTICAL SET -- not just the same number, the same 68 criteria named.** That control is the whole point: **a run that stops early is faster than a run that finishes**, and my first attempt at this measurement WAS that mistake -- v3 came back in 0.015s having refused before scoring, and reporting it would have been a 5,500x speedup for doing no work.

**And Intent's own estate migrated in 0.50 seconds** -- 56 threads, 311 files written, exit 0, on a clean clone at `bcbd02cd` with cc's `Thread.body` in it.

**THE DB REALLY IS THE SSOT AND I STOPPED TAKING THAT ON TRUST.** I copied the migrated tree, **deleted every `.md` file under `intent/st`**, and asked again: `46/114`, same set. D01-reversed is now measured rather than declared.

**WHY IT IS THIS FAST, AND IT IS NOT MAINLY RUST.** v2's 95 seconds is 31s user and 59s SYSTEM -- it is forking subshells per row, re-reading and re-parsing the markdown for each of 114 criteria. v3 asks the store once. **The shape of the win is the one you predicted: parsing per row became a query.**

**TWO PARITY DEFECTS THE COMPARISON FOUND, both cc's, both reported to them.** `intent ac status` prints the GATE's line (`gate: ... BLOCKED`) and exits 0 -- a blocked-looking line beside a passing exit code, which is what a consumer misreads. And `intent at lint` prints **nothing at all** and exits 0, where v2 prints `lint: ok -- 114 AT row(s) conform`; **a check with no output cannot be told from a check that did not run.**

**WP-15 CREATED for the skills triage, as you asked.** 26 skills in `intent/plugins/claude/skills/`, and the denominator is written into the WP on purpose -- a triage reporting "the obviously dead ones were removed" leaves the unexamined ones as the unmeasured arm, which is the condition it exists to end. Sequenced after the hoist and marked a PRECONDITION of WP-12 closing rather than a follow-on. **No ACs written: your moratorium holds until the hoist lands, and the surface these skills must be measured against has not stopped moving.**

**ONE THING FOR YOU, NOT URGENT.** dc found that a ruling I gave over the live agent channel reached their board but never reached the contract, so **their board went AHEAD of the artefact and neither of us can attest the wording.** I have ruled the operational half -- **a ruling given over the channel is not landed until it is in the artefact it governs, and the node who GIVES it lands it, never the node with the interest** -- but the general form binds all five of us and is yours to ratify. It is the cost of the medium we are told to prefer: it writes nothing to disk.

**And a contract defect of mine, fixed at `c6f37188`: AC-11.5 was UNCLOSABLE** -- marked `(non-test)`, and that branch returns before the gate ever looks at a covering AT, so its test could have gone green forever with no effect. **Then the note I wrote warning that a hand-written flag would close the row SPELLED the flag, and closed the row.** The document is its own data format. Caught in the same minute by the scan; nothing else in the contract does it.

## (2026-08-17 21:27Z)

**A HOLE IN A GATE YOU SET, found by ic, and the part of it that is mine to rule I have ruled. Nothing needs you tonight.**

**YOU GATED THE CUTOVER ON: a second migration over an interrupted estate must reach the same end state as a clean one. NOT ONE OF THE 114 ACs COVERS THAT PROPERTY.** ic measured it rather than asserting it: 0 ACs mention it, the single lexical match is AC-09.3 (bridge mode surviving a daemon restart -- different subsystem, different question), and **0 AT rows cite either of the two instruments that measure it.** The nearest, AC-10.2, is atomicity on a REFUSAL -- nothing written when Phase A blocks -- not a crash mid-write followed by a re-run.

**So both instruments could be deleted tomorrow and not one AC would go red. WP-10 could close green with the property you gated the cutover on entirely unmeasured.** Same shape as a vacuous green, except the omission is in the CONTRACT rather than in an instrument.

**WHAT I HAVE RULED, because it is a contract question and the contract is mine: WP-10 DOES NOT CLOSE until this property is in the contract.** That converts the gap into a block, which is the only thing that makes a gap safe while it is open. **What I have NOT done is mint the AC -- your moratorium holds, and ic explicitly declined to propose one because they hold WP-10 and an author writing his own criterion is the arrangement none of us should want.** When you lift the moratorium, this is the first row to write.

**MEANWHILE THE PROPERTY HAS BEEN MEASURED ANYWAY, on all four fleet members, each predicted in writing before the run and scored after:** canary PASS, baize PASS (a shape the gate was NOT built around -- 540 files identical, store byte-equal, interrupted at 158/160), utilz REFUSED on the version floor, lamplight REFUSED on residue. **Two passes, two refusals for two different and correct reasons, zero false greens.**

**AND A NUMBER YOU SHOULD HAVE, because it changes what Lamplight is.** All of us -- my board, the durable project memory -- have carried _"~1158 permanent legacy rows"_ for Lamplight since the sweep programme was ruled dead. **What actually BLOCKS its migration is TEN findings across TWO threads.** Both numbers are real and they answer different questions, and the big one has been standing in for the small one. **The member that reads as hopeless is ten fixes in two files from migrating.** ic gave it as measured and explicitly not as explained; the follow-ups are mine. **[ANNOTATED 2026-08-17 22:47Z -- THE CLAIM ABOVE IS WITHDRAWN. It is left standing because a record corrected in place stops being a record; this note is appended so the withdrawal is findable FROM the claim rather than 42 lines below it (ic's method, and their hazard: a future reader greps, finds the claim, finds no withdrawal, and gives a retired number a second life).** **What stands: 10 findings, two threads, both live. What is withdrawn: that fixing them would let the estate migrate.** Full withdrawal entry later in this file at `(2026-08-17 22:44Z)`.]**

**AT-11.5 CLOSED GREEN.** dc sent the close claim rather than taking it, declared they were not neutral, and listed the evidence against themselves before I asked -- including a canary arm that does not reproduce, recorded as proving nothing and not counted. I verified the load-bearing halves at source rather than accepting them. **The one thing worth your time: `sign` announced the hardened runtime as fact without reading it back, and with the option dropped `codesign` STILL prints `valid on disk` and `satisfies its Designated Requirement` over `flags=0x0(none)`.** A green from the tool everyone trusts, over a binary without the protection.

## (2026-08-17 21:41Z)

**ONE THING FOR YOU IN `bin/**`, WHICH IS YOURS AND NOT OURS, AND IT IS SMALL.**

`ac_is_nontest` (`bin/intent_acceptance:90`) matches the doc-check marker **ANYWHERE in an AC's line**, so a row whose PROSE discusses the marker is classified as carrying it. **The fix is to anchor the match to the kind position -- immediately after the AC id -- rather than anywhere in the line.**

**Measured radius (dc): 22 AC rows match, 21 carry it in the kind position and are classified correctly, ONE was misread.** So it is one row today. **But it is a class by construction: any AC whose note discusses the marker joins it, which is exactly what happens to a row the moment somebody documents the defect.** Both of us did it to the same row within an hour, in opposite directions -- one note spelled the satisfaction field and CLOSED a criterion that must not close; the note announcing that repair spelled the doc-check marker and made the row unclosable again.

**THE GENERAL FORM, AND IT IS THE DAY'S CLASS AT THE SMALLEST SCALE: `acceptance.md` is its own data format, so writing ABOUT a marker writes the marker.** A note explaining a repair is itself a repair, in whichever direction the token points. Three separate times today, including once in `MEMORY.md` where my note that _"12-WP ladder"_ is stale is now the only match for `12-WP ladder`.

**AND THE PART THAT SHOULD WORRY YOU MORE THAN THE BUG: my own checker had agreed with the gate on all 114 rows all session, and the agreement was a coincidence of inputs.** My scan read the marker positionally; the gate reads it anywhere. **On every row where the token sits only in its proper slot the two readings coincide** -- so the checker was reproducing the gate's answer rather than confirming it, and would have gone on doing so. It is corrected to the gate's reading and now reports any row where the two split. **Two readers of one format agree until the first row that separates them, and nothing tells you which row that is.**

**STATE OF THE CONTRACT: gate 47/114, lint clean at 114 rows, and my parser and the gate agree on the unsatisfied set by a reading that now matches the authority's rather than by luck.**

## (2026-08-17 22:41Z) Correcting a number I gave you two hours ago

**I ran it myself on the pinned corpus at `fcfa0ffd` (5613/5613 verified) and reproduced your result exactly: 10 findings, `unparseable-row: 9, broken-reference: 1`, ST0276 and ST0345.**

**FOLLOW-UP 1 -- ARE THOSE TWO THREADS LIVE? YES. Both `status: WIP`, both unbucketed.** So the block is hv's carry policy working exactly as designed rather than a migrator defect.

**FOLLOW-UP 2 -- DOES THE CARRY POLICY ACCOUNT FOR THE REST? NOT ESTABLISHED, AND I NEARLY SAID IT WAS.** I reasoned: the migrator blocks on live residue, it found 10, therefore everything else is carried. **That inference is wrong, and it is wrong in the way you warned about.** **Phase A REFUSES before the carry stage runs -- zero carry lines in the whole output -- so the run says nothing whatever about the other 1633 AT rows. The absence is the refusal's shape, not evidence.** You held it as hypothesis; I had it as a conclusion for about two minutes.

**AND I HAVE TO CORRECT MY OWN RELAY, WHICH MATTERS MORE THAN EITHER ANSWER.** I published _"ten fixes in two files from migrating"_ and put it in the shared `MEMORY.md`. **It is not established and there is positive reason to doubt it: three OTHER live (`WIP`) threads carry 49 rows of the same legacy form -- ST0201 (6), ST0264 (22), ST0276 (21) -- and the migrator names 8 of ST0276's 21 and NOTHING for the other two.** So the 10 is a numerator with no stated denominator. **The memory is corrected.**

**WHAT THE BLOCKERS ACTUALLY ARE: one class in three fields -- a qualifier fused to a value where the grammar expects a bare token.**

    covers AC-00.1 (see the correction on the AC)
    status: green **(cc, `cc2c4faf2`); mutation-proved.** Restoring the...
    status: green for Stage A

Accepted rows terminate the value with `--`; rejected ones run prose straight on. **Same class as ST0056's own `covers`-qualifier ruling at `959b0190`.**

**AND cc's 10 IS NOT YOUR 10.** cc reported `NOT_STARTED` at 13 fleet-wide (Lamplight 10, Laksa 3) and read it as reconciling with yours. **It does not: theirs are status values in COMPLETED threads and cc's own sentence says they change nothing about what blocks; yours ARE what blocks and are all malformed AT rows.** Two unrelated measurements both landing on 10, on one estate, on one day. **A reconciliation fitted over that would have been comfortable and wrong.**

**A CORRECTION AGAINST MY OWN METHOD, which is the part I would keep: I used `::` -- the retired `path::name` citation -- as a proxy for "unparseable". It is wrong. I collapsed 21 of ST0276's citations and the finding count did not move at all, 9 before and 9 after.** A proxy is not the parser, and mine was counting a different population than the one that blocks.

**Nothing owed back. Your run stands exactly as you reported it; everything corrected here is downstream of my reading of it, not of your measurement.**

## (2026-08-17 22:44Z) WITHDRAWAL -- "ten fixes in two files from migrating"

**I sent you that phrase twice tonight and it is withdrawn. The earlier entries are left standing rather than edited, so it is visible that the claim moved and when.**

**WHAT STANDS:** Lamplight's Phase A refuses on **10 findings across two threads (ST0276, ST0345)**, both live (`status: WIP`, unbucketed) -- so the block is your carry policy working as designed. Reproduced by vc on the pinned corpus at `fcfa0ffd`, 5613/5613 verified.

**WHAT IS WITHDRAWN: that fixing those ten would let the estate migrate.** Not established, with positive reason to doubt it. **Phase A REFUSES before the carry stage runs** -- zero carry lines in the whole output -- so the run says nothing about the other 1633 AT rows, and **the absence is the refusal's shape rather than evidence.** Three other LIVE threads carry 49 rows of the same legacy form (ST0201 6, ST0264 22, ST0276 21) while the migrator names **8 of ST0276's 21 and nothing for the other two.** A numerator with no denominator.

**ATTRIBUTION, because ic declined the pass I offered them and they were right to: ic coined the phrase, I amplified it into THIS file and into the shared `MEMORY.md`.** Theirs is the coinage; mine is the durable record, which is the half that would have outlived the conversation. **Both are withdrawn; the memory is already corrected.**

**THE LESSON IS ic's AND IT GENERALISES: "blocks on ten" is a measurement, "ten fixes FROM MIGRATING" is a claim about a DENOMINATOR, and no caveat written beside a phrase disarms a conclusion baked into its wording.** Their board carried "stated as measured and NOT as explained" one line below the claim, and the caveat could not reach it.

## (2026-08-18 06:01Z)

**I MADE A RULING ON YOUR MORATORIUM THAT ENLARGES WHAT IT PERMITS, AND YOU SHOULD SEE IT RATHER THAN INHERIT IT.** cc asked whether the moratorium covers writing an AT for a row **already in the contract at `to-write`**. **I ruled it does not.** Your words were no new defect classes, no new acceptance criteria, no new instruments; an existing `to-write` row is already a criterion, and the five files it names are already cited BY the contract, so writing them discharges an obligation rather than extending one. It shrinks the open set. **Two conditions: the cited path and covered AC stay as written, and if writing the test makes cc want to move either it comes back to me first.** cc accepted both and has already named where it will bite (AT-10.2 cites `intentsvcs/tests/migrate_refusal.rs` while the faithful drive is now through the shipped command in `intent-cli`).

**A ruling that ENLARGES a restriction should be visible to whoever set it. Reverse me and cc stops at the measurement.**

**THE FINDING THAT MADE IT URGENT, and it is the most consequential thing either of us has measured this week.** cc found it in their own lane and I verified it independently rather than take it: **`Facade::upgrade` -- the migrator -- is called by exactly ONE test in the entire workspace, and that call is AC-10.7's unmigrated DETECTION.** Convert and refuse have no coverage in the cargo suite. All five `to-write` AT rows for WP-10 name files that do not exist on disk.

**And my own half, which is the correction to the sentence at the top of my board.** "The hoist rehearsal is green -- 46/114 identically" **is pinned at `bcbd02cd`, and every component of it has since moved**: `legacy.rs` +280/-17 over 3 commits, `contract.rs` +187/-42 over 3, `store.rs` +42/-7, `model.rs` +40, `facade.rs` +32/-5 -- **and `acceptance.md` itself, so the subject moved too. The v2 number is 48 now, not 46.**

**PUT TOGETHER: at this moment the migrator's convert path is held by one stale hand run and nothing else.** Not "untested" flatly -- ic's cutover gate drives convert twice and refuse twice, and that is real evidence. **But it is hand-driven, outside cargo, and pinned against the same moved migrator, so it is the same class as my rehearsal rather than an answer to it. The thing that re-runs on every commit covers neither behaviour.** When the hoist lands, this is the most consequential code in the repo.

**WHAT I AM DOING ABOUT IT, RUNNING NOW: the rehearsal re-run at `6248236e`, both halves on one tree** -- clean clone, dirty=0, its own target dir, binary built from that clone. Measuring v3 alone would be uninterpretable, because a 46 would be ambiguous between "v3 regressed" and "the contract moved" and **halves measured at different times cannot separate those.** **Predicted in writing before it started: 48/114 both sides, identical satisfied set, all 56 threads migrate, and the deleted-`.md` control still 48.** cc's sharpening is in the control: the surviving `.md` count is asserted **at the moment the store answers**, not at the moment the files were deleted, because a store quietly falling back to files still on disk returns the right number for the wrong reason.

**Nothing here needs you except the ruling.** The re-run is mine and the result goes to you either way.

## (2026-08-18 06:16Z)

**THE HOIST, MEASURED AT `6248236e` RATHER THAN AT THE PIN I HAD BEEN QUOTING -- AND A RECOMMENDATION AT THE END, WHICH IS YOURS TO TAKE OR REFUSE.**

**PARITY.** v2 over FILES, v3 over STORE, and a control with every `.md` under `intent/st` deleted **all print `48/114, 66 unsatisfied` -- and the three outputs are IDENTICAL BYTE FOR BYTE**, including the traversal quirk that emits `AC-08.8` before `AC-08.7`. **That ordering artefact is a fingerprint: v3 reproduces v2's traversal, not merely v2's answer**, which is not something a reimplementation reaches by agreeing. Migration: **56 threads, 40 issues, 352 files, exit 0.**

**CONSERVATION, first ever run over Intent's OWN estate: ALTERED 0, ADDED 0, DOUBLED-SECTION 0, DEFERRAL-REFUTED 0, dispositions 115 of 115 parsed, liveness names all 56 threads.** Residue: **STRANDED 192, UNACCOUNTED 354, LOST-PROSE 384** -- the bucketed-path class, unchanged in kind. **Nothing is deleted or corrupted; those files sit at `intent/st/<BUCKET>/<ID>/` while canon is flat, so they are reachable from git and from a human and not from the model.**

**AND A HAZARD THAT TURNS OUT NOT TO BE ONE, which I think is the single most useful line here: the migration does not touch `intent/whiteboard/` at all -- zero files.** It writes `intent/st` (74), `intent/issues` (40), `todo.md`, `events.jsonl`, `.config`. **So "four nodes writing their boards while the repo migrates" is not a conflict. That was the thing I understood to be blocking scheduling, and it does not exist.**

**FOUR INSTRUMENT DEFECTS FOUND TODAY, ONE PER NODE, AND THEY ARE ONE CLASS.**

- **ic:** `interrupt_rig.sh` killed a SUBSHELL, not the migrator. `wait` returned 137, the rig read that as proof the interruption landed, and the migration ran to completion -- it printed "interrupted at 36/40" over a tree holding 40 of 40. Fixed `77ddeb07`. **Their canary and baize convert PASSes are withdrawn pending re-run.**
- **me:** `conservation_check.sh` printed `SUBJECT unpinned @ /Users/matts` for an estate elsewhere -- field 3 is a revision when pinned and a PATH when not, and I truncated both to 12 chars. Fixed `31ff8204`.
- **dc:** the **`Intent Tests` (bats) CI leg has failed on all of the last 20 runs**, back to 2026-08-15, while Rust passed -- so **"597 passed / 0 failed" is one language of five.** `prepush_push_range.bats:97` calls bare `git init`, so the branch name comes from **your personal `~/.gitconfig`**; the runners have no `init.defaultBranch` and get `master`.
- **cc:** `Facade::upgrade` -- the migrator -- **has exactly ONE test call in the whole workspace, and it is the unmigrated DETECTION.** Convert and refuse have no cargo coverage. I verified it.

**THE THROUGH-LINE, and it is why I am reporting them together: in three of the four the defective arm had NEVER EXECUTED against the subject that mattered.** ic's kill path only runs when something is interrupted; my unpinned branch only runs on an estate without a `.CAPTURE` marker, which Intent's own is and no fleet member is; dc's fixture only diverges on a machine without your gitconfig line. **Each was green for as long as the inputs allowed and not one minute longer.**

**THREE DECISIONS ARE YOURS.**

1. **The moratorium ruling I reported last message** -- writing an AT for a row already at `to-write`. Ratify or reverse; cc is holding for you rather than taking it.
2. **`rig_selftest.sh` is a NEW instrument built under a moratorium that bars new instruments.** ic's, not mine to bless, and they have made the withdrawal clean: delete it and the rig fixes stand. **They also declined to let it in through my ratification, which was the right call and not one I would have made for them.**
3. **The interruption-property AC has changed shape.** A row minted from the DESIGN would have been satisfied by ic's broken rig -- it interrupted, re-ran, compared, and said so. **The standard I have ratified: an AT earns its green from an instrument demonstrated RED, and from a criterion naming the SUBJECT it was handed, not only the verdict.** ic's sharpening: a comparator that correctly says IDENTICAL over a never-interrupted tree is behaving properly, so the verdict alone discriminates nothing.

**MY RECOMMENDATION, AND IT IS A CHANGE OF POSITION.** I have been treating the hoist as gated on instrument confidence. **I now think that is the wrong gate, because the hoist does not destroy anything.** Migration WRITES canon alongside the v2 files; the 192 stranded files stay in git; the store is per-project at `intent/.cache/intent.db`; and `git revert` undoes the commit. **The irreversible step is deleting the v2 files afterwards, and nobody is proposing that.**

**So: hoist when you want to, and treat the residue questions as post-hoist work with the store in place.** What genuinely should NOT wait is cc's coverage gap -- **the migrator is about to become the most consequential code in the repo and the suite says nothing about it** -- but that is a reason to release cc to write those tests, not a reason to hold the migration. **You know things I do not; this is a recommendation and not a finding.**

## (2026-08-18 06:20Z)

**REFINING MY OWN RECOMMENDATION FROM 25 MINUTES AGO, because I tested the thing I had asserted and got it partly wrong.** I told you the hoist destroys nothing. **That is true with one exception I had not found, and the exception has a one-line remedy.**

**WHAT I TESTED: the real hoist migrates a DIRTY tree** -- four nodes hold files in flight -- **and every rehearsal so far, mine included, has run at `dirty=0`.** Two ways that could go wrong in opposite directions: the migrator refuses on a dirty tree (so the hoist can never run), or it reads git HEAD rather than the working tree (so uncommitted prose is silently not carried).

**RESULT, three cells varying one thing each:**

- **A -- tracked `info.md`, edited and NOT committed: CARRIED**, into canon and into the store. **So the migrator reads the WORKING TREE, not HEAD, and uncommitted work is not lost. It does not refuse on a dirty tree either; exit 0.** That is the answer to the question and it is the good one.
- **B -- an UNTRACKED `design.md`: not carried.**
- **C -- a tracked but non-canonical `notes.md`: not carried.**
- **The migrator names none of the three in its output.**

**I PREDICTED B WOULD BE CARRIED AND IT WAS NOT, AND THE REASON MAKES IT A SMALLER FINDING THAN IT LOOKED.** A **committed** `design.md` is not carried verbatim either -- I checked ST0055's, 0 of 3 verbatim probes in the store. **So B is not about tracking at all: it is the bucketed-path residue my own conservation check already counts as STRANDED 192.** My experiment rediscovered a number I had already measured, by a different route. **Corroboration, not a new finding, and I would rather say that than let it read as a discovery.**

**THE ONE PART THAT IS GENUINELY NEW, and it is the exception to "nothing is destroyed": for a TRACKED stranded file the content survives in git even though the model cannot reach it. For an UNTRACKED one it survives NOWHERE -- not in the store, not in git, and the migration does not name it.**

**SO THE PRECONDITION, which is cheap and which I should have stated before recommending anything: commit everything under `intent/` before the hoist.** **I checked the live repo: untracked count under `intent/` is ZERO right now**, and the only dirt is two of ic's rig tools, modified and in flight. **So the hazard is not live -- it is a thing to check at the moment you go, not a thing to fix.**

**MY RECOMMENDATION STANDS, with that precondition attached: hoist when you want to; commit first; treat the residue as post-hoist work.** The one thing I would still not defer is cc's coverage gap on the migrator.

**And a caveat on my own probe, since I have been holding everyone else to this today: the verbatim-phrase test has a false-negative mode.** `acceptance.md` also scored 0 of 3, and that is NOT loss -- the migrator parses AC rows into `.criteria[].text` and `.tests[].note`, so a 70-character probe spanning a whole row cannot match by construction. **The probe answers "is this prose in the store verbatim", not "is this content carried", and those differ for anything structured.**

## (2026-08-18 06:26Z)

**A PROCEDURAL CHANGE TO THE HOIST, MEASURED RATHER THAN REASONED, AND IT IS THE LAST THING I WOULD WANT US TO DISCOVER AFTERWARDS.**

**The parity comparison becomes UNRUNNABLE the moment the hoist lands.** The migration writes `"intent_version": "3.0.0-dev"` into `intent/.config/config.json`; `bin/intent` grew a forward-compatibility guard yesterday (`53f88757`, yours) that refuses a project from the future. **I ran v2's gate on the already-migrated tree: exit 2, `error: this project declares Intent v3.0.0-dev, and this is Intent v2.19.0`.**

**That guard is correct and I am not asking for it to change.** The consequence is procedural, not technical: **after cutover there is no v2 half, so v2-versus-v3 parity can never be measured again on this project.**

**SO: the parity run must happen IMMEDIATELY BEFORE the hoist, on the actual tree being hoisted.** Today's run at `6248236e` is fresh, byte-identical, and **is not a substitute** -- it is a rehearsal of a measurement that gets exactly one real execution. **Ten minutes, and it is the only chance.**

**AND dc's RESULT, which strengthens rather than weakens the number.** All five bats files covering the v2 acceptance gate were red **locally** -- 62 of 299 failures -- for the cause dc traced: `test_helper.bash:93` builds every fixture declaring `intent_version: 3.0.0`, your forward-compat guard refuses it, **so every fixture was refused at exit 2 before any command ran.** The helper's own comment named the remedy (`INTENT_FIXTURE_VERSION`) and nothing in the tree ever set it. Now wired; suite 959 ok / 2 not-ok.

**The exact scenario I described to dc two hours ago -- a new failure landing invisibly behind a permanently red leg -- had already happened, and neither of us knew.** **So the 48/114 everyone has been quoting was produced by a tool whose own test coverage was refusing to execute.** The number is unchanged and the byte-identical agreement is unaffected -- **agreement was always the claim and agreement is not correctness** -- but there is now evidence those guards can run, which there was not this morning.

**Nothing needed from you here. The three decisions from my earlier messages still stand, and the parity-before-cutover point is a ten-minute addition to whatever procedure you choose.**

## (2026-08-18 06:31Z)

**ONE SAFETY ITEM BEFORE THE HOIST, SHORT, AND IT IS THE ONLY THING TODAY WHOSE FAILURE MODE IS NOT RECOVERABLE BY RE-RUNNING.**

ic enumerated `interrupt_rig.sh`'s refusal sites: **24 of them, 11 driven**, now 17. **Among the seven still undriven is the guard that stops the rig migrating THIS repository while the four of us are working in it.**

**Three of three refusal paths ic drove this morning were defective** -- the subshell kill, a `set -u` abort from a deleted variable, and `--rev` silently discarded under an override. **The live-repo guard has the same provenance: same file, same author, same never-executed status. That is not a claim it is broken; it is that nothing distinguishes it from the three that were.**

**The asymmetry is the point. Every other defect found today produced a wrong NUMBER. This one would produce a wrong REPOSITORY.** I have asked ic to drive it next, ahead of the five that need a clone and a build.

**Two smaller things, both FYI.**

**The one-shot parity procedure is now written down** -- `parity.md`, under Measurement rules, at `6280e281`. Seven steps, each control present because its absence produced a wrong green at least once. **It had been living as a shell script in a scratch directory that is deleted with the job: an instrument for a measurement with exactly ONE execution, in ephemeral storage.** Same class as everything else today, in my own lane.

**And ic's finding upgrades the method argument in a way worth having when you rule on the moratorium.** Their first two defects came from driving a case that FAILED. The third came from a case that **PASSED**, and was a finding only because a prediction had been written down first. **So the method is not "drive it" -- it is predict, drive, score against the prediction, and the third step is what turns a green into a defect.** ic is making that argument to you themselves, with the denominator attached (17 of 24, not 17 of 17). **I am not making it for them; I am confirming the three-part form is what the evidence actually supports.**

## (2026-08-18 06:51Z)

**INTENT IS HOISTED. `0ec2ac79`, on `local`, verified by reading the remote's own ref. The estate lives in the v3 store and the committed canon is its extract.**

```
v2 gate over FILES (one-shot, taken FIRST)   48/114, 66 unsatisfied, 143s
migrate                                      56 threads, 40 issues, 352 files, exit 0
sync --to-store                              store and extract agree
v3 gate over STORE                           48/114, 66 unsatisfied
COMPARISON                                   IDENTICAL BYTE FOR BYTE
commit                                       353 files, 25277 insertions
search whiteboard                            exit 0
```

**THE MIGRATION REFUSED ON ITS FIRST ATTEMPT, and this is the part worth your time.** A pre-versioning store from 2026-08-15 was sitting at `intent/.cache/intent.db`, and the migrator would not guess at a schema nothing had recorded. **`intent/.cache/` is GITIGNORED -- so a clone cannot contain that file by construction, and NO rehearsal any of us could have built would ever have found it.** Not an untested branch: **an input the test environment is structurally incapable of holding.**

**cc then closed the class rather than the instance, in about a minute, and it is the cheapest check anyone ran today:** a clone plus all fourteen OTHER gitignored artefacts in this repo -- twelve `.DS_Store`, two of them inside the migrator's own walks, plus `.treeindex/` -- converts to the identical 56/40/352. **The database was not the first of a class. It was the only one.** The general remedy is not a better rehearsal; it is knowing which inputs your rehearsal structurally cannot hold, and going to look at those directly.

**The old store held ZERO canonical rows** -- 775 `file_index` entries, derived from disk, empty WAL -- so nothing existed only there. **Moved, never deleted; it is at `~/.intent-prehoist-store-20260818T064735Z` and restores in one command.**

**FOUR THINGS I GOT WRONG TODAY, since you will read the peer traffic.** My first move put the old store at `intent/.cache.pre-hoist-<stamp>`, which the ignore rule does not match, so I turned a hidden object into a visible artefact and `sync` correctly refused it. My subject-stability guard compared a full sha256 against its own 16-char DISPLAY truncation and refused while printing two identical-looking values. My out-of-band re-check disagreed with the script because `shasum` embeds the filename and one form ran relative and one absolute. **And I told three nodes the parity comparison could "never be measured again", which is too strong: a clone at a pre-hoist commit still has a v2 config, so the committed state is re-derivable. Taking it first was still right; the irreversibility was overstated and dc had already elevated it to critical path on my word.**

**PEER WORK WAS NEVER AT RISK: the migration writes nothing under `intent/whiteboard/`, and the commit excludes every file that was in another node's hands.**

**WHAT IS NOW OPEN, and none of it blocks anything today.**

- **dc's shim is the last mechanical step of (a) and it is YOURS.** 21 v2 projects sit behind global symlinks at `~/.local/bin/intent` and `~/bin/intent`; flip them and those break, leave them and Intent is unusable from tomorrow. **Both guards are correct -- the entry point is what must become project-aware.** dc built and drove it three ways and has NOT installed it. **v3 is still not on PATH.**
- **`doctor` exits 1 on a freshly migrated estate (`backup-stale`)** -- the first thing every node runs will report a residue. With cc for after.
- **`intent claude ws` is unimplemented**, and 6 of 31 v3 leaves besides. Not blocking: boards are plain files.
- **The interruption AC is still 0 of 114 and still blocks WP-10's CLOSE, not the hoist.** ic's gate PASSED on the real subject before I committed.
- **The residue question you were asked in one line by ic is still open** and I am not treating "crack on" as its answer.

## (2026-08-18 06:57Z)

**COORDINATING AS YOU ASKED. THREE NODES TASKED, AND TWO THINGS NEED YOU.**

**YOUR QUESTION FIRST: NO FORCING NEEDED, IT IS ONE CLAUSE, AND THE MODEL IS WHAT IS WRONG.**

`doctor.rs` asserts that an AC's group names a work package. **ST0043, ST0044, ST0045, ST0046, ST0050 and ST0051 have ZERO work-package directories and 73 AC rows between them.** `AC-01.1` on ST0044 never referenced a WP-01 -- there isn't one and there never was. **In a thread with no WPs the group is a numbering device; grouping BY work package is a convention used where WPs exist.** The fix is `&& !thread.wps.is_empty()` and it kills **73 of the 78 findings**.

**It hides nothing: 37 threads DO carry work packages and not one of them is in the finding set** -- I checked that specifically, because a suppression that also hides real defects is the thing we have been catching all day. **And the fault is MINE: `data-model.md:193` said "group = WP seq or `00` for ST-level", which I wrote from ST0056's shape.** Corrected in this commit. **So we are not forcing acceptance on old threads and not editing six closed ST's contracts -- we are fixing a rule that was over-specified.**

**YOUR OTHER POINT, THE 11 SECONDS: it is real and it is the only outlier.** Measured on the hoisted repo: `st list` 0s, `ac gate` 0s, `at lint` 0s, `todo` 0s, **`doctor` 23s** (debug build; your release run was 11.05s user / 19.1s wall). **The gate went 95s to 0.02s by moving off file parsing; doctor looks like it stayed on it** -- 255 views have to be rendered and compared to disk. **Hypothesis, not measurement; cc is tasked to test it rather than take it.**

**TASKED:** cc -- the one-clause fix, the doctor performance, and two after-items. ic -- the second-upgrade experiment (what happens when `upgrade` runs twice on an already-migrated project: **ids are minted not derived, so a re-run could union into duplicates**; nobody has asked and every fleet member will do it). dc -- `backup-stale`, and a judgement call on the gate below.

**NOW THE TWO THAT ARE YOURS.**

**1. THE PRE-COMMIT GATE NOW EXITS 0 AND ENFORCES NOTHING, ACROSS ALL FIVE LANGUAGES, SILENTLY. I CAUSED THIS BY HOISTING.** dc measured it on the real repo at `0ec2ac79`: `intent` on PATH is v2, v2 correctly refuses the migrated tree at exit 2, **and 2 is the code the gate fails open on.** Three correct behaviours composing into a gate that passes while measuring nothing. **It is a DEGRADATION, not a blocker -- commits and pushes work. What is gone is rule enforcement, and it went quietly.**

**dc's shim is the fix and it is your call, not ours.** It resolves the project, reads the version the project declares, and execs the matching binary; driven three arms (v2 project, v3 project, outside any project -- the third matters because `init` and `bootstrap` are not built in v3). **It is not installed. `~/.local/bin/intent` and `~/bin/intent` are machine-GLOBAL and 21 other projects on this machine are v2** -- flip them naively and those break. dc has refused to install it twice on the grounds that it is outward-facing and yours. **They were right both times.**

**2. THE MORATORIUM WAS "UNTIL THE HOIST LANDS" AND THE HOIST HAS LANDED, so by its own terms it is spent -- but I am asking rather than reading.** ic made the point this morning that a general directive read as answering a specific question is how a decision nobody made ends up cited by two independent sources, and they were right about "crack on". **So: is the moratorium lifted?** Nothing in flight depends on it -- cc's two items are fixes to existing code -- but WP-15's ACs and the interruption-property AC both wait on the answer.

**Your residue question from ic is still open too, and I am still not treating "crack on" as its answer.**

## (2026-08-18 18:08Z) FYI only -- no response needed.

ANNOUNCE (vc, to every node) -- hv RULED a change to the 3.0.0 gate.

Verbatim: "Definitely BEFORE the release. We're getting this whole thing feature complete before we release 3.0.0."

The subject is ST0057, disk as a sparse projection of the store. It is now INSIDE the 3.0.0 gate, not after it.

State at `6accab7e`, measured, not recalled:

|                                               |                                                |
| --------------------------------------------- | ---------------------------------------------- |
| ST0057 WPs built                              | 0 of 8 (three are L)                           |
| ST0057 objective / context                    | empty / empty                                  |
| ST0057 ACs / ATs                              | 0 / 0                                          |
| `.intentfiles`                                | does not exist                                 |
| `intent/.canon/`                              | does not exist                                 |
| `intent/st/`                                  | 57 dirs, 797 files                             |
| ... belonging to threads nobody is working on | 468 (52 completed, 2 cancelled, 1 not-started) |

How it surfaced: hv looked at their own file tree, saw 50-odd hydrated ST directories, and asked why -- immediately after I reported that nothing of mine was outstanding. It was outstanding. My report scoped "outstanding" to my inbox and stated it in the grammar of a claim about the estate, which is the defect my own board warns about: a criterion must name its subject.

What it changes, per node:

- **cc** -- the pre-release build queue grows by eight WPs. ST0057 WP-01 (canon relocation) and WP-02 (`.intentfiles`) unblock the rest.
- **dc** -- WP-01 changes what a released artefact contains and what a fresh clone looks like, so any distribution work assuming today's `intent/st/` layout now has an expiry date inside the gate.
- **ic** -- parity scope grows with it; WP-01 moves the files a parity run reads.

What is mine, starting now: ST0057 has no acceptance contract at all. I am writing the objective, the context, and the AC/AT set so the thread reaches cc as a ratified boundary rather than as my prose.

FYI only -- no response needed. Reply only if the WP-01/WP-02 ordering is wrong against your own queue.
