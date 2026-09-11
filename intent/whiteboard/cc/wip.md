---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: NONE ON THIS SESSION'S COMMITS -- read off my own 981a55049 with the grep below and it came back EMPTY, while vc's b13d58d2c four commits earlier carries session_01QdJZysgcMJ1SEeyo7wAUpE. So the marker is written by SOME commit paths and not mine, and the previous value on this line (0167bZhMQsEXFM5JZUZxL5g7) is a different session's and has been deleted rather than carried. UNEXPLAINED, not investigated -- it is a lead for whoever owns the stamper. READ IT WITH grep, NEVER WITH THE TRAILER PARSER: git's %(trailers:key=Claude-Session,valueonly) and git interpret-trailers --parse return EMPTY on EVERY commit here, because the mandated (C) line is a non-trailer line in the final paragraph and git rejects the whole paragraph. THE WORKING READ: git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*'. POINT-IN-TIME -- read it off your own last commit, never off this line.
heartbeat_at: 2026-09-11 12:07Z
status: active
focus: "ON THE BOUNCE 2026-09-11: #71 0259 IN FLIGHT. Held on vc rulings: 0100 (shape) and 0084 (a sidecar writer first). With vc: 0069, 0080, 0226, 0097. Then my P3/P5 items in list order. hv: no new work, these items ONLY; intent/wip.md is the authority. One id at a time: claim, commit with the id, tell vc, vc closes. The 16 clippy lints are hv's decision 4."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

**IN FLIGHT (2026-09-11 12:07Z): #71 `0259`.** With vc: `0069` (`c0b688a1`), `0080` (`4f718b3c`), `0226` (`7ac7de0c`; drive the lib test, not the fenced harness) and `0097` (`a15870e8`). vc closed `0268`.

**THE RULES ON THE BOUNCE (vc):** one item at a time, claimed here; the id in the commit subject; tell vc, who re-drives and closes it. The only new test allowed is the proving one, seen red. No new instruments, guards, criteria or threads. A defect found while fixing goes in the commit message, not on the list. **hv: _THERE IS NO NEW WORK TO BE DONE._** The list in `intent/wip.md` is the authority, and `intent issues list` is the live state. **Read the lane column there, never a copy here.**

**RUN THE GATES RATHER THAN READING A FIGURE HERE.**

## TODO -- startable, mine, in the list's order

- **My P3/P5 items in the list's order:** #78 `0136`, #79 `0141`, #80 `0114`, #82 `0152`, #83 `0210`.

## Holds -- mine, with the CONDITION that releases each

**A hold whose condition still stands is never archived by a fold. None below is released.**

- **#64 `0100` -- RELEASED WHEN vc rules the shape** (asked 2026-09-11 12:02Z): (a) `status: Option<WpStatus>` plus `status_legacy`, about 40 reads across 23 files, L; or (b) `status_legacy: Option<Legacy>` carrying the raw v2 spelling beside the substituted status, M. Both need a store rung 17->18, so tell dc before touching `store.rs`. REC (b).
- **#65 `0084` -- RELEASED WHEN vc rules** (asked 12:07Z). The obvious fix (`sync::inspect` stands down for a non-UTF-8 attachment) is banked at scratchpad `0084-banked.patch` (sha256 `74679d6cee74fe2a`) and NOT committed. Lifting the refusal makes canon name an opaque attachment whose sidecar no door writes: `export::canon_blobs` has no non-test caller and `WriteSet` is text-only, so the second `--to-store` refuses `broken-reference`. REC (a): add a sidecar writer (WriteSet bytes + projection + record_landed) along with the inspect change, proved by a double restore.
- **A hold's condition names the artefact it waits on** (eg `git log --since=... --grep <id> -- native/rust`), not a string that any mention of the id matches. `git log --grep 0216` matched `69ebc932`, a 2026-09-03 issue-body edit, and I told vc the `0226` hold was released a minute before it was (2026-09-11 11:37Z).
- **POST-CUT (culled from the 3.0.1 loop 2026-09-11):** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). Their conditions stand and none is 3.0.1 work.
- **The 16 `collapsible_if` in intentsvcs -- RELEASED WHEN hv says go on hv's decision 4 (NOT a list item) AND `facade.rs`/`store.rs`/`daemon.rs` carry no peer's uncommitted work.** They are our own code at a fixed compiler (10 -> 10 across 1.98.0 -> 1.98.1), so this is mechanical, not a policy call. They are the SOLE blocker on the `rust` workflow, and they gate the five prettier arms that have never measured in CI. My REC is to collapse them.

## Decisions

- (2026-09-09) **A ROUTE RECORDED IS NOT A ROUTE TAKEN.** ic's board read _`0218` released to cc ... Routed, not taken_, while `cc/inbox.ic.md` was `_(empty)_`. **A pickup reads your own board and your own inboxes, so under the protocol that release was invisible to me at every step of my boot** -- I have it only because I grepped a peer's board for my own moniker, which is not a procedure.
- (2026-09-09) **EVERYTHING SHIPS AS 3.0.1. SETTLED BY hv, NOT OWED, NOT MINE TO RE-OPEN.** Ruled twice and restated a fourth time in hv's own words. The new-surface argument is DEAD, not deferred. **I do not raise it again.**

## Owed -- vc holds hv's pen; question, options, recommendation

**NOTHING IS OWED FROM THIS BOARD. hv's DECISIONS LIVE IN `intent/wip.md`, WHICH IS THEIR ONE HOME.** My items there are #4 and #10. `hv/inbox.cc.md` is a pointer, and the pre-lean record is at `.history/20260911/hv-inbox-cc-prelean-0825Z.md`.

## Open, no owner

- **Something WALKS the CLI surface.** Seven CLI-token-titled creations in `event_log`, two episodes eight days apart. **This is `0223`'s -- re-read it, do not restate from here.** Two episodes is ONE interval, and one interval is not a period.

## Watch-outs

**FOLDED AGGRESSIVELY 2026-09-11 FOR THE 3.0.1 BOUNCE: TWELVE FAMILIES KEPT, EACH CUT TO ITS RULE AND ITS BOUNDARY.** Nothing was merged away, and the full text with every instance is at `.history/20260911/wip-prefold-aggressive-0917Z.md`. **Read the family there before arguing with it.**

**W1. THE INSTRUMENT ANSWERED A QUESTION ADJACENT TO THE ONE ASKED, AND ANSWERED IT CORRECTLY.** The dominant family; it arrives while I am being careful. **NOT AN INSTANCE:** a claim that names the FIELD read and the INSTRUMENT that read it, driven to both verdicts on a subject that can exhibit the failure.

**W2. CONTROLS, OR THE READING IS NOT EVIDENCE.** A control that cannot distinguish _safe_ from _never tried_ is not a control, and it must vary the axis the check reads. **NOT AN INSTANCE:** a control that would FAIL under the broken instrument, over a fixture that can tell the mutated quantity from its neighbours.

**W3. POPULATION, DENOMINATOR AND SAMPLE SIZE.** A precision figure is a claim about the CORPUS. **NOT AN INSTANCE:** a count whose population is stated, whose exclusions carry their reasons, and which states n and the variance.

**W4. ONE NAME, TWO ARTEFACTS.** A NAME resolves to TWO artefacts, a check examines ONE, and the claim is phrased about the NAME. **NOT AN INSTANCE:** a verdict that names the artefact it examined.

**W5. THE SHARED CHECKOUT.** Canon cannot be split, so every canon commit is silently multi-node. **NOT AN INSTANCE:** a change confined to a private worktree, or to a file no peer has touched, verified by looking rather than assumed. **THE ONLY SAFE WRITE:** `git add` your paths, then `git commit --only` them, retry the SAME command against a peer's index lock, never remove the lock, and judge success by `git log`, not by the loop. **A test run here compiles peers' UNCOMMITTED edits too** (2026-09-11: I pinned a red on `6e478ec4` by recency, and it was dc's mid-edit `render.rs`; the wrong sha now stands in `86071c36`'s message). Attribute a red only after re-running with `git status --short native/` empty.

**W6. A CLAIM THAT DOES NOT FEEL LIKE A CLAIM IS THE ONE TO DRIVE.** **NOT AN INSTANCE:** a premise driven in the same turn, against an artefact of the right era.

**W7. THE RIGHT ANSWER WAS PRESENT, CAPABLE, AND NOT REACHED.** **NOT AN INSTANCE:** a question answered after reading the artefact that OWNS the decision, not only the ones that describe it.

**W8. A PEER CHANNEL HAS NO TERMINATING CONDITION, AND DELIVERY IS NOT THE WRITE.** **NOT AN INSTANCE:** an exchange with a stated terminating condition, whose delivery was confirmed with the asker.

**W9. CANON, THE DAEMON, AND POSTCONDITIONS THAT ARE TRUE OFTEN ENOUGH.** **NOT AN INSTANCE:** a canon write verified PAST the ingest on a structured read of the value. **The read-verify-retry loop is a REQUIREMENT:** a running intentd can revert the last write of a burst about a second after it reports ok, and `event_log` is the only provenance surface.

**W10. THIS BOX AND THIS SHELL.** **NOT AN INSTANCE:** a command whose quoting and exit code were driven rather than assumed. **An exit code that IS the finding never goes through a pipe** (`if cmd | tail` tests tail's status). **The Bash tool's shell is zsh:** unquoted `$var` does not word-split and an unmatched glob aborts the command, and both return a plausible silence rather than an error.

**W11. FOLDING THIS BOARD.** **NOT AN INSTANCE:** a cut keyed on EXECUTION, with the pre-fold banked first and the asker confirmed.

**W12. A PRECONDITION THAT IS CORRECT CAN STILL BE A MIGRATION, AND THE ONLY WAY TO TELL IS TO COUNT.** **NOT AN INSTANCE:** a fix whose blast radius was counted across the real population BEFORE it was proposed.

## Decisions -- durable principles, carried across folds

**CUT 2026-09-11 TO WHAT GOVERNS THE INGEST, MIGRATION AND STORE-WRITE LANE, OR STANDS AS A RULING IN FORCE.** The rows about the instrument and criteria loop that hv ended are archived verbatim at `.history/20260911/wip-prefold-aggressive-0917Z.md`.

- (2026-09-09, cc, after vc) **A PEER CONTRADICTING MY BOARD IS A PROMPT TO DRIVE THE SUBJECT, NOT TO ESCALATE A CONFLICT.** I routed _vc says placing `WP-14` is theirs, my board says hv descoped it whole_ to vc as an unresolvable state conflict. **My board was right and had been since 2026-09-02**, and `intent ac list ST0056 | grep AC-14` settles it in one second: all twelve rows read `descoped-to: ST0069`. **I escalated a question I already held the answer to** -- which spends a peer's turn and puts a false open item on hv's desk. vc drove it, found the same thing, and struck their own entry. **This is A27 in the other direction: there I trusted my board over a verb, here I distrusted it over a peer, and the cure is the same verb both times.** Residue vc found and routed to hv, NOT mine to touch: `intent wp show ST0056/14` still reads `Not Started`, scope L, with zero in-scope criteria -- a work-package shell is what made both of us read it as live.
- (2026-09-04, cc) **A CITATION INHERITS THE CITED CLAIM'S TRUTH AT CITATION TIME AND NEVER UPDATES.** `0223` rests on `0090`+`0151` to conclude a junk row _can only be closed_; all three are open, the underlying verb shipped, and **none of the three noticed.** Currency is a property of a CLAIM (F3) and equally of every REFERENCE to it -- and a reference is worse, because it reads as sourced.
- (2026-09-04, cc) **A PUBLISHED FIGURE IS A CLAIM ABOUT AN ARTEFACT, AND THE ARTEFACT IS THE COMMITTED ONE.**
- (2026-09-04, hv via cc) **`config`, `ext` AND `learn` SHIP DECLARED-AND-UNBUILT IN 3.0.1** (2026-08-31). **A general ruling stated later does not vacate a specific one.**
- (2026-09-04, cc+dc) **A DIRECTORY `add` PROTECTS A PATH AND A DIRECTORY PATH IS NOT A FILE.** And **a path that moves on its own is not a path either node can own** (`project.json`'s `todo_watermark`).
- (2026-09-04, cc) **THE CANON STORE AND DISK DO NOT DISAGREE.** The divergence is git-only. Filing writes store and disk atomically; **git is a separate human-gated act and NOTHING REPORTS THE GAP.**
- (2026-09-04, cc) **A SET PASSING TOGETHER IS NOT EVIDENCE ABOUT ITS SUBSETS.**
- (2026-09-03, cc+vc) **CONTENTION IS `0216`'s VARIABLE** -- not spacing, not corpus size. **Refusals (`0226`) and silent losses (`0216`) TRADE OFF**, so a single counter prints _fewer losses under load_ and reads as improvement.
- (2026-09-02, cc+vc) **`0216`'s FIX IS DAEMON-SIDE, NOT USAGE DISCIPLINE.** A hazard reachable by an ordinary shell loop cannot be mitigated by how carefully nodes write.
- (2026-09-02) **TWO MACHINE PROJECTIONS OF ONE VALUE MUST NOT DRIFT; A HUMAN RENDERING OF IT IS NOT A COPY AT ALL.**
- (2026-09-02) **CONTENT COMPARISON DOMINATES A VERSION COUNTER FOR A COMPARE-AND-SWAP.**
- (2026-09-01, hv) **v3.0.1 IS FEATURE COMPLETE, THERE IS NO TAG WINDOW AND NO EXTERNAL CONSUMER, AND COST IS NOT A CONSTRAINT.** The scarcity register is retired as a class.
- (2026-09-01) **A REFUSAL THAT CANNOT SAY WHAT IT FOUND MAKES ITS OWN DEFECT UNDIAGNOSABLE.**
- (2026-09-01) **A REMEDY INHERITS ITS BRANCH'S ASYMMETRY.** Confirm-before-refuse is SAFE on the lock and WRONG on the probe.
- (2026-08-31, ic correcting me) **AN OWNERSHIP SPLIT IS A PURPOSE, NOT A BOUNDARY.** Two hands in one file IS `0206` in miniature.
