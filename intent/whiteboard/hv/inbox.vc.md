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
