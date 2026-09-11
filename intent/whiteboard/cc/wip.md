---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: NONE ON THIS SESSION'S COMMITS -- read off my own 981a55049 with the grep below and it came back EMPTY, while vc's b13d58d2c four commits earlier carries session_01QdJZysgcMJ1SEeyo7wAUpE. So the marker is written by SOME commit paths and not mine, and the previous value on this line (0167bZhMQsEXFM5JZUZxL5g7) is a different session's and has been deleted rather than carried. UNEXPLAINED, not investigated -- it is a lead for whoever owns the stamper. READ IT WITH grep, NEVER WITH THE TRAILER PARSER: git's %(trailers:key=Claude-Session,valueonly) and git interpret-trailers --parse return EMPTY on EVERY commit here, because the mandated (C) line is a non-trailer line in the final paragraph and git rejects the whole paragraph. THE WORKING READ: git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*'. POINT-IN-TIME -- read it off your own last commit, never off this line.
heartbeat_at: 2026-09-11 14:02Z
status: active
focus: "BOUNCE, 2026-09-11 14:02Z. #74 0153 FIXED at ca2dd018 on vc's shape (a), with vc for the re-drive. 0145 with vc at ff78b917; 0066 at f74fbf31. Held on hv: 0100 (decision 2), 0084 (decision 11). hv: no new work, these items ONLY."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

**#74 `0153` -- FIXED AT `ca2dd018` ON vc'S SHAPE (a), WITH vc FOR THE RE-DRIVE.** Mostly delivered at HEAD. The residue was the shared `NotAddressable` remedy naming an issue id, which `edit` refuses in every form. It now names only a thread id and a full thread address. The proving test reads the forms off the rendered remedy and drives each through `edit --path`, and was seen red on the issue form. One limb of `a_typo_is_a_usage_error_naming_both_forms`, which asserted `0042`, is amended to the ruled forms. Found-whiles in the message: the ambiguous-id remedy offers `intent:///issues/0001` to `edit`, and `edit`'s `kind` enum advertises `issue`. Nothing of mine is dirty.

**#72 `0145` -- FIXED AT `ff78b917` ON vc'S RULING, WITH vc FOR THE RE-DRIVE.** At HEAD it was half delivered: the refused `st edit ST0001 impl` still grew the tracked `.intentfiles` 1652 -> 1671, and the remedy was non-empty only because the refused call realised the thread first. `Facade::carried` (a pure read, and the one home that `hydration` also calls) lets `edit` decide membership before `hydrate`. The proving test was seen red. `a_refusal_does_not_roll_back_a_hydrate_that_already_happened` was deleted on vc's ruling, citing 2026-08-22. Nothing of mine is dirty.

**#81 `0172` -- RULED NOT WORKABLE IN 3.0.1 by vc on hv's 2026-08-30 ruling; lane `--`, out of my column.**

**#66 `0066` -- ROUTED BY vc AND CLAIMED (no clock was read at the claim, so it carries no time), FIXED AT `f74fbf31`, WITH vc FOR THE RE-DRIVE.** On v3 the defect is sharper than the issue's v2 evidence. The migrator walks the top level plus the three status buckets, and `Project::migration` descends into any directory. So a v2 estate with `intent/st/_inbox/ST0002/` upgraded at rc=0 with the thread skipped, and then every verb refused under a remedy (`intent upgrade`) that looped. Phase A now blocks, as `unknown-file-shape`, naming the path and both remedies. The proving test is in `migrate_refusal.rs`, seen red first (`Upgraded { threads: 1 }`). A census of 24 estates found no holding today.

**#77 `0176` -- RULED BY vc: close as not a defect at HEAD, no code.** vc carries the false remedy (a positional reason; `st reopen` printed for a WP) in the close message. The board before the fold is verbatim at `.history/20260911/wip-prefold-1311Z.md`.

**THE RULES ON THE BOUNCE (vc):** one item at a time, claimed here; the id in the commit subject; tell vc, who re-drives and closes it. The only new test allowed is the proving one, seen red. No new instruments, guards, criteria or threads. A defect found while fixing goes in the commit message, not on the list. **hv: _THERE IS NO NEW WORK TO BE DONE._** The list in `intent/wip.md` is the authority, and `intent issues list` is the live state. **Read the lane column there, never a copy here.**

**RUN THE GATES RATHER THAN READING A FIGURE HERE.**

## TODO -- startable, mine, in the list's order

- **Empty.** Every cc row in `intent/wip.md` is either with vc or held on an hv decision. Read the lane column there, never a copy here.

## Holds -- mine, with the CONDITION that releases each

**A hold whose condition still stands is never archived by a fold. None below is released.**

- **#64 `0100` -- RELEASED WHEN hv answers decision 2's added question** (`intent/wip.md`): does 3.0.1 take a store rung 17 -> 18? The shape is ruled (option (b): `status_legacy: Option<Legacy>` carrying the raw v2 spelling beside the substituted status). Tell dc before touching `store.rs`.
- **#65 `0084` -- RELEASED WHEN hv answers decision 11**: does 3.0.1 build the byte write for opaque attachments? The fix is banked at scratchpad `0084-banked.patch` (sha256 `74679d6cee74fe2a`) and NOT committed. Lifting the refusal alone makes canon name a sidecar no door writes (`export::canon_blobs` has no non-test caller, and `WriteSet` is text-only), so the second `--to-store` refuses `broken-reference`. With a go: WriteSet byte writes + projection emits `canon_blobs` + `record_landed`, then the inspect change, proved by a double restore.
- **POST-CUT (culled from the 3.0.1 loop 2026-09-11):** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). Their conditions stand and none is 3.0.1 work.
- **The 16 `collapsible_if` in intentsvcs -- RELEASED WHEN hv says go on hv's decision 4 (NOT a list item) AND `facade.rs`/`store.rs`/`daemon.rs` carry no peer's uncommitted work.** They are our own code at a fixed compiler (10 -> 10 across 1.98.0 -> 1.98.1), so this is mechanical, not a policy call. They are the SOLE blocker on the `rust` workflow, and they gate the five prettier arms that have never measured in CI. My REC is to collapse them.

## Decisions

- (2026-09-11, cc) **A HOLD'S CONDITION NAMES THE ARTEFACT IT WAITS ON, NOT A STRING.** `git log --grep 0216` matched `69ebc932`, an issue-body edit from 2026-09-03, and I told vc the `0226` hold was released a minute before `0216`'s code landed. The working form is `git log --since=<when> --grep <id> -- <path>`.
- (2026-09-11, cc) **A FIX THAT REMOVES A REFUSAL MUST FIRST ASK WHAT THE REFUSAL WAS PROTECTING.** `0084`'s UTF-8 refusal looked like a survivor to delete, and it was the only thing stopping canon from naming a sidecar that no door writes. Driving the fix through the verbs, not just the unit, is what showed it.
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
