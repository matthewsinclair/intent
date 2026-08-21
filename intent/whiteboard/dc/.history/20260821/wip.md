# dc -- 2026-08-21 archive (fold 1)

**A short session: pickup, a plan, and hv's stop. No code was written and no row moved.** What follows is the part that would not survive the restart -- the reasoning, the dead ends, and three measurements whose subject and revision only this session knew.

## Session frame

Woken as a node at 09:28Z, revision `706db8ee`, tree clean. All four nodes came up within five minutes (cc 09:29Z, ic 09:29Z, vc 09:31Z, dc 09:33Z). Folded on hv's instruction at ~12:40Z, relayed by vc on the live channel and written to `inbox.vc.md` at 12:44Z. **Nothing in flight.**

## 1. THE AC-11.6 CONTRACT CONFLICT -- the finding of the session, and it exists in no file but this one

**AT-11.6's deliverable `shared_artefact_build_guard.sh` is ABSENT (driven). I did not build it, and the reason is not time.** The row contradicts a ruling that postdates it, and the two halves are both on the record:

- **AC-11.6, as vc minted it:** _the green arm must not be a clean tree only -- a builder dirty in paths they DO own must still be allowed, or the guard is a freeze rather than a control and gets bypassed._
- **cc, 2026-08-20, correcting my own reshape** (`.history/20260820/wip.md:78`): _a deliberate, announced, entirely legitimate publish build carries the union identically ... Only requiring a CLEAN TREE reaches authorship, and the prize is that `dirty-<sha>` becomes `<sha>`._

**The conflict is exact, and it is not a wording problem.** The green arm AC-11.6 mandates -- dirty-in-owned allowed -- is precisely the arm that produces an unattributable union binary. **The criterion's green arm reintroduces the defect the criterion exists to prevent.**

**AND THIS IS WHY THE RED-FIRST ARM COULD NOT BE MADE RED**, which the row records as a fact without ever naming the cause. The discriminator AC-11.6 asks for does not exist, for two independent reasons:

1. **There is no ownership oracle and the protocol forbids building one.** Whiteboard Protocol 3.0, invariant 3, in its own words: _Claims by ST ID (in the `wip.md` header block), **never glob paths**._ Ownership of a dirty path is therefore undeclarable by design.
2. **Even with an oracle it would not discriminate** -- cc's finding above. A shared-tree build carries the union of every node's dirty bytes whoever invokes it, so authorship is not recoverable from the artefact at all.

**PROPOSED RESOLUTION, RECORDED AS A PROPOSAL AND NOT A CHANGE.** Amend the green arm to: _clean tree required for a build into the shared path; dirty builds go to a private `CARGO_TARGET_DIR` and are marked `dirty-<sha>`._ Both arms then become reachable and the guard is buildable. **Routes to vc as contract steward, to hv if vc bounces it.**

**THE TELL, NAMED BECAUSE IT IS THE REASON TO ROUTE RATHER THAN THE REASON TO ACT: this amendment would conveniently unblock my own row.** That is exactly the shape my own board says to stop and route.

**AND IT COUPLES 11.6 TO 11.7.** cc's prize -- `dirty-<sha>` becoming `<sha>` -- is what makes AT-11.7's _source commit_ field mean currency rather than approximately-currency. **11.6 is upstream of 11.7 in meaning, but 11.7 is buildable first** because its spec is sound and its positive controls are already on record.

## 2. A SHARPENING OF MY OWN CITATION-SPLIT RULE, which as written would have mislabelled both rows I touched today

The board carries the split as: _does the cited file carry the row's own literal id? 2 hits means ready to green; 0 hits means the citation is wrong._

**THE RULE HAS AN UNSTATED PRECONDITION: THE CITED FILE MUST EXIST.** Driven today:

```
shared_artefact_build_guard.sh   AT-11.6   ABSENT   0 hits
provenance_fields_check.sh       AT-11.7   ABSENT   0 hits
canon_commit_check.sh            AC-03.6   EXISTS   3 hits  -> ready
thread_view_skew_check.sh        --        EXISTS   149 lines
```

**For an ABSENT file, 0 hits is trivially true and means UNBUILT, not wrong-citation.** The two states are opposite -- one is work owed, the other is a defect in the row -- and the rule as written collapses them. `bin/int` scoring 0 for AT-11.6 was a wrong citation because `bin/int` **exists**; that is what made the reading valid there. **Test existence first, then the id.**

## 3. PERISHABLE MEASUREMENT -- an unplanted positive control for `lib_binstale.sh`, at `706db8ee`

```
native/rust/target/release/intent          2026-08-20 15:01:03
native/rust/crates/intentsvcs/src/migrate.rs   2026-08-20 17:57:50
native/rust/crates/intentsvcs/src/facade.rs    2026-08-20 17:57:50
                                           -> release binary 2h56m STALE
```

**`thread_view_skew_check.sh` reads exactly that binary**, so my conditional on its roster admission is not theoretical -- **it is live at this revision**. A stale binary renders with the OLD generator, matches the OLD committed views, and greens on precisely the commit that changed the generator.

**THIS CONTROL IS DESTROYED BY THE NEXT `cargo build --release`.** If it is gone at the next pickup, that is expected and not evidence of anything.

## 4. DEAD END -- do not re-walk the disposition route for the WP-07 hosting sweep

I tried to re-measure the carried claim (_16 of 32 families dispatch; `intent claude` implements 1 of 8_) out of the dispatch table. **It does not answer the question.**

```
surface/dispatch-table.json   27 families / 109 entries
  disposition:  87 keep | 14 new-surface | 7 retire | 1 pending
v3 --help:      34 top-level families, 8 `claude` subcommands
carried claim:  "32 families"
```

**`disposition` is about v2 -> v3 PORTING, not implemented-vs-stub**, so grouping by it measures a different property entirely -- the exact class this board warns about. And **none of 27 / 32 / 34 reconcile**, so the denominator itself is unresolved before any numerator is worth taking. The real signal is the central unbuilt-verb reporter at `render.rs:495` (`is a known command that is not implemented yet`); **the sweep should be driven through the binary, not read off the table.**

## 5. Facts worth carrying, each driven here

- **`ac gate` NAMES the unsatisfied rows; `ac status` gives only N/M.** `ac gate ST0057` -> rc=1, `unsatisfied: AC-07.7 AC-08.5 AC-03.6 AC-01.5`. I did not know the verb existed this morning either.
- **The gate derivation, now driven end to end:** `ac status ST0057` 47/51 + `ac status ST0056/03` 15/16 = **62 of 67**. The third call takes a **WP-scoped STID**, which no instruction in this estate mentioned. I flagged the restart's `63 of 67` as internally inconsistent at pickup (63 + the five named outstanding = 68) and declined to re-assert either number; vc corrected it at `14298e6b` and supplied this derivation at 10:26Z.
- **The parity-guard roster lives at `bin/.devbin/cmd/precommit`** (`:103` provenance, `:106` roster, `:112` self-provenance). `bin/int` is a symlink to `devbin`. **This is the file both held roster admissions edit**, and `bin/` is the one genuine cc/dc collision, open for hv.
- **There is no `target/dc`.** My compliance with hv's new prune-at-fold ruling is **vacuous today** -- I have been driving `target/debug`, not a per-node dir. `target/tmp/` and `target/dist/` exist and match no node moniker, so **the per-node rule does not reach them** and nothing else does either.
- **`intent` on PATH is v2.19.0 and correctly refuses this tree at exit 2.** Everything above was driven through `./native/rust/target/debug/intent`.

## 6. Plan authored, then stopped by hv's fold -- kept because the ordering was reasoned, not arbitrary

Block 1 `lib_binstale.sh` (ungated; unblocks the held `thread_view_skew_check.sh` admission **without taking hv's decision** -- that was the point of putting it first). Block 2 escalate the AC-11.6 conflict. Block 3 AT-11.7. Block 4 hosting sweep. **Nothing started.**

## 7. A LIVE, UNPLANTED INSTANCE OF AC-11.6's SUBJECT -- surfaced by the gate on my OWN fold commit, and stronger than the episode the row was written from

The self-provenance arm printed this while committing this fold (`5f8d5b7d`), and I confirmed it independently rather than taking the guard's word:

```
intent    sha256 957aa2b2e9029f5b   dirty-483e65e49190d6134d31ae312ccb0319b3da68b2
intentd   sha256 b672a608d56e984d   dirty-5819417bcc0e7d31e1d052e79d6d6896c4a25849
```

**THE TWO MARKERS DIFFER.** The shared `target/release/` currently holds **two binaries built from two DIFFERENT dirty trees**, and they are invoked as a matched pair.

**THIS IS A SHARPER STATEMENT OF AC-11.6 THAN AC-11.6's OWN.** The row's founding episode was one unattributable binary. This is worse and is a different shape: **the shared path holds an INCOHERENT PAIR** -- not merely two artefacts that cannot be attributed, but two that do not agree with each other about what tree they came from. **Anyone running `intent` and `intentd` together right now is running two different trees**, and nothing anywhere says so. The existing guard REPORTS this and never fails -- correctly, since enforcement is ruled to sit at `int macos publish` -- so the estate has been told and no gate acts.

**It also refutes any residual case for the ownership discriminator.** Neither marker names an owner, and there is no owner to name: each is the union of whatever was dirty when that binary was built. Two builds minutes apart from one shared tree produce two different unions. **Ownership is not merely undeclarable, it is not a property the artefact has.**

**PERISHABLE: the next `cargo build --release` destroys this pair.** If the markers agree at the next pickup, that is a rebuild and not a refutation. **Capture the sha256 pair above, not the markers** -- the markers are provenance, never identity, which the guard says in its own words.

## 8. I REPORTED "dc HOLDS NONE OF THE GATE" AT PICKUP AND IT WAS WRONG -- how, because the mechanism is the interesting part

**cc corrected it at 12:59Z**, after hv's hold had landed, and drove it rather than asserting it: `grep -cE 'canon-ignore|pre-commit\.intent|AC-01\.5' dc/wip.md` -> **0**.

**HOW I GOT IT WRONG.** I read the five outstanding rows and their owners off `intent/restart.md`, which assigns ST0057 AC-01.5 to **cc**. That is **true of the ROW and false of the REMEDY** -- vc ruled the remedy dc's in `AT-01.5`'s note on 2026-08-20, and no edit cc can make reaches the row. **A row's owner and a remedy's owner are two fields and the file carries one of them.**

**AND I DID CHECK -- WITH THE WRONG INSTRUMENT.** I ran `ac gate` on both threads, which is the right verb and gave the right answer to the question it asks: **which rows are unsatisfied.** It does not carry remedy ownership and cannot. **A true measurement of a different property, offered as proof** -- my own board's line, committed this morning, in the same session in which I did it. The instrument was correct, driven, and about something else.

**THE CHECK THAT WOULD HAVE CAUGHT IT is the one cc ran, and it takes seconds: grep my OWN BOARD for each outstanding row id.** A row ruled mine that appears nowhere on my board is either unrecorded or not mine, and both are worth knowing. **Neither `ac gate` nor `restart.md` can answer it; only the board can.** Add it to pickup.

**cc's framing is worth keeping verbatim:** _I am not reporting that you missed it -- I found it in my own pickup, it was step 1 of my plan, and hv's hold landed before I sent it, so the gap was mine to close and I closed it late._

## 9. DELTA AFTER THE 13:01Z FOLD -- and it opens with me committing, in one message, the exact error I was naming in that message

**ZERO OF FOUR NODES BOUNCED. NOT THREE.** All four -- cc, dc, ic, vc -- reported `resume`, unchanged `session_id`, this morning's conversation intact. **And all four independently told the others _three of four bounced, but not me_, each citing `ListAgents` showing the other three as started ~5 minutes ago.**

**`ListAgents`' `started` IS SOCKET AGE, NOT SESSION AGE.** When the topology changed every peer re-registered, so **everyone looked fresh to everyone else**. Four correct self-reports; one unanimous wrong inference about the population.

**I DID IT INSIDE THE MESSAGE WHERE I NAMED THE CLASS.** I wrote to vc that `command -v intent` answers about `intent` and was offered as an answer about the tree -- population-vs-instance -- and in the same message offered `ListAgents` start times as evidence about SESSION age. **A true measurement of a different property, offered as proof: third instance today, and the only one where the author had just written the rule down.** Knowing the class does not confer immunity; the check does. **The instrument was correct and about something else, which is what makes this class invisible from the inside every single time.**

**AND THE UNANIMITY IS THE PART THAT SHOULD FRIGHTEN US, NOT THE ERROR.** Four nodes cross-checking each other produced agreement, not correction, because **all four ran the same wrong instrument.** Consensus across nodes is worth nothing when the nodes share a method -- it is the reconciles-because-both-sides-share-an-error case already on my board, at four-way scale. **vc caught it by comparing self-reports against inferences, not by adding a fifth voice.**

**One precision on vc's instruction, offered because it matters for their globalfold: my board never carried the wrong figure.** Driven -- the only `three of four` on it is `intent claude upgrade --apply`'s three-of-four-actions, unrelated. **The bad inference existed only in the SendMessage and in chat.** So there was nothing to fix, which is itself worth knowing: **the thing that would have poisoned a cold session was never written down, and the thing vc asked me to correct was the one place it was safe.**

## 10. SHELL STATE AND CONTEXT STATE ARE INDEPENDENT -- with the consequence, which is the half worth keeping

My Bash shell **re-initialises from the user's profile on every invocation**, so it picked up all three of hv's bindings the moment they landed. My **context only reloads on restart**, so it did not. Both were true at once, all afternoon.

**THE CONSEQUENCE: THE ENVIRONMENT HALF OF A CHECK CANNOT DETECT AN UNBOUNCED SESSION.** A fresh shell makes an unbounced node **indistinguishable** from a bounced one -- my raw `INTENT_HOME` / `type intent` / sibling output was clean and comparable to a genuinely restarted node's, and it had to be. **Only session identity separates them**, and `session_id` is the only field that carries it. **`ListAgents` start time is NOT that field** (section 9).

Corollary for any future roll-call: **ask for `session_id`, not for environment.** Environment answers whether the SHELL reloaded; identity answers whether the SESSION did. A roll-call that asks only the first gets four green answers from four unbounced nodes.

## 11. THE AT-11.6 PERISHABLE PAIR, VALUES ON THE BOARD RATHER THAN A POINTER TO THEM

vc captured these before they aged out. **My own instruction was _capture the sha256 pair, never the markers_ -- and I then wrote the instruction onto the board WITHOUT the pair**, which is an instruction with its data missing and would not have survived the bounce.

```
intent    sha256 957aa2b2e9029f5b   marker dirty-483e65e4...
intentd   sha256 b672a608d56e984d   marker dirty-5819417b...
```

**Two different markers on a pair invoked as matched.** The markers are provenance and are not identity; the sha256s are what distinguish one build from another.

---

# AFTERNOON SESSION -- post-bounce, fresh session `d2fad1a7` (fold at 2026-08-21 15:31Z)

**THE BOUNCE TOOK AND IT WAS DRIVEN.** My board carried `80fa1787`; this session is `d2fad1a7`. Later, all four: cc `ef9e17d5`->`87048274`, ic `6e1c92e1`->`d5a0bd62`, vc `575f9585`->`3049725b`. **Read off each node's OWN board -- four independent self-reports, not one shared instrument.** That is the difference from yesterday, when all four read `ListAgents` socket age and all four were wrong.

## 1. THREE FALSE FINDINGS I NEARLY ESCALATED, ALL FROM ONE METHOD ERROR

**`int hooks` resolves its target from the BINARY's location, not the cwd** -- `bin/devbin:29-30` derives `PROJECT_ROOT` from the launcher's own path, and `cmd/hooks:75` uses it. The PATH `int` is `Intent/bin/int`, so standing anywhere else it answers **about Intent**.

Withdrawn, all three mine: that it reports a wired clone with `core.hooksPath` unset; that `--install` fails to install; that it asserts a wired gate over a missing dispatcher. **All three were Intent's true state read through a mis-aimed instrument.** `Intentv2/bin/int hooks` reports `gate ABSENT` correctly.

**AND THE TELL WAS IN THE OUTPUT THE WHOLE TIME.** Line 1 is `hooks in /path/...`. **I grepped for `gate|WIRED` and cut off the line naming the subject.** The instrument was not silent; my filter made it silent. **A filter that drops the subject line converts a correct report into a confident wrong one.**

## 2. `sync --to-disk <ID>` IS A WHOLE-FILE SECOND WRITER (the session's biggest hazard)

It rewrites **every attachment file of that thread** from the store. cc was live in `runner_roster_check.sh`; my sync pulled ~130 lines of their unreviewed work into my index. **HEAD 266 / my staged copy 397 / worktree 411 -- a line-count check was the only thing that stopped me committing a peer's work under my name.**

`--to-store` first is disk-authoritative so it round-trips, but that only holds for files on disk at that instant. **Same class as the markdown formatter, far more reach: four nodes, one checkout.**

## 3. A BARE `git reset` IS A SHARED-INDEX OPERATION AND NOTHING NAMED IT

I ran `git reset --quiet` twice (14:42:57Z, 14:44:51Z) with **no pathspec** -- it resets the WHOLE index, unstaging every peer's staged work. Content survives (mixed); staging does not.

**vc FALSELY CONFESSED TO IT** in commit `af91be8b` -- _"--only protects the commit, not the index -- I cleared a peer stage proving it"_. vc then drove `--only` in a throwaway repo both directions and **it preserves a peer stage in both**. Corrected at `7d28d882`.

**vc's CLASS, AND IT IS THE SHARPEST THING TODAY: A FALSE CONFESSION IS CAUGHT BY NOTHING WE OWN.** Every rule we have is tuned for claims that FAVOUR the claimant -- _a change that would conveniently green your own work is the one to stop and route_. **A claim that inconveniently blames you passes every check precisely because it costs the claimant. Owning a fault reads as rigour, so nobody audits it.** The only control is a peer who checks a self-accusation as hard as a self-serving one.

## 4. TRAPS THAT COST TIME TODAY

- **THE ROSTER TABLE IS A SINGLE-QUOTED SHELL STRING. ONE APOSTROPHE BREAKS THE PARSE.** I broke it twice -- **the second time inside the sentence warning about apostrophes.** Verify the count is 0 BEFORE writing.
- **`direnv status` prints `Found RC allowed 1` and `1` MEANS NOT-ALLOWED.** I read the word `allowed` and reported the file as approved. **A status field whose name is the opposite of its value.**
- **`zsh -i -c` DOES NOT FIRE direnv** -- the hook is on `precmd`, which `-c` never runs. My "it does not work interactively" was a false negative. `direnv exec .` is the honest test.
- **AN EXISTENCE TEST BEFORE A PATH TEST:** `canon_commit_check.sh` is under **ST0056**'s tools dir, not ST0057's, though it covers an ST0057 row. My board had the wrong path.

## 5. CROSS-OWNER ATOMICITY IS IMPOSSIBLE WITH `--only`, AND IT BIT

Tracking a new parity tool **requires a roster row**; the row lives in a file a peer owns; `--only` is path-scoped, so **whoever commits that file commits both parties' work.** No split exists.

Result: between `d8dd6dc6` (cc) and `5d2b1f0d` (me) **HEAD was internally inconsistent** -- committed roster said `gated`, committed runner did not dispatch. It read GREEN because the roster check reads the runner from the **WORKTREE**. **A fresh clone got DISAGREE and nothing local could see it.**

**cc BELIEVED THEIR WIDENING HAD LANDED WHEN IT WAS UNCOMMITTED** ("the widening landed as: 51 files rostered"). Driven: `git show HEAD:... | grep -c not-an-instrument` -> 0 while the worktree had 30. **`fixed is not a state`, in a peer's mouth, six messages after they agreed with the class.**

## 6. AC-03.6 -- WHY cc's REVERT WAS RIGHT AND WHAT ACTUALLY CLEARED IT

cc wired this in full on 2026-08-19 and **reverted it**: `REV="HEAD"` at pre-commit time is the **PARENT**, so a planted index divergence printed `EXAMINED 0 of 280` at exit 0. Their prescription: _a `--staged` MODE, NOT A CALL SITE -- a real change to a 425-line instrument._

**That mode landed; the file is 464 lines. I wired `--staged`, never the default.** Two-sided control on a REAL index, unplanted: `ADDS 1 of 88` rc=1 (my own commit), `ADDS 0` rc=0 after sync-then-commit-together. cc hit it independently at `ADDS 1 of 89`.

**The row's closing line -- _whoever admits it wants the path trigger_ -- IS SUPERSEDED. hv declined the path trigger explicitly**, with cc's 3.6-4.9s figure in front of them, choosing unconditional at ~4.6s -> ~7.3s gate total.

**THREE COST FIGURES, NONE COMPARABLE:** cc 3.6-4.9s at `61b93440`; mine 2710-2760ms narrowed at `ecea0eeb`; recorded 2.49-2.55s at `4ba598f1`. Different revisions, machines, days.

## 7. THE `.envrc` -- INERT, AND THE CLAIM ON IT WAS THE REAL DEFECT

vc landed `.envrc` (`9b883bd1`) putting v3 first inside this checkout. **It is BLOCKED and does nothing for anyone** -- `direnv exec .` -> _.envrc is blocked. Run `direnv allow`_. **Needs hv to run `direnv allow`; nobody else should.**

Its claimed side effect -- _that is the drift exposure closed for Intent_ -- **is false in the hook path.** Driven non-interactive: PATH has no v3, `INTENT_HOME` = `Intentv2`, `intent` -> 2.19.0, guards still resolve from the frozen v2 tree. **Exactly hv's stated grounds for declining direnv: green where you look, absent where it matters.** Worse for us: **all four nodes commit through non-interactive tool calls, so it is inert for every commit we make.**

**vc's MECHANISM FOR HOW IT GOT PAST THE RULING, AND IT GENERALISES: they framed the benefit as a SIDE EFFECT.** hv's decline was filed under "the mechanism", so **disclaiming it as incidental moved the same claim outside the ruling's scope. A benefit reclassified as incidental escapes the ruling that governs it.**

## 8. THE ORDERING GAP (vc's, and it is unclosed)

cc and I reported hv's hold-lift ruling **differently**, both stamped 14:30Z, both correctly carrying **no time hv could be quoted on**. **Date-only granularity cannot sequence two same-day rulings in opposite directions on one item.** The transcription rule solves ATTRIBUTION and is silent on ORDERING. Resolved this once by directness -- my question named all three items with a single-item option that hv declined -- but the gap stands.
