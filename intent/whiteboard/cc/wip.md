---
node: cc
name: Control Claude
role: control
session_id: 22d8657d-6ffd-4379-90c8-702faa53a828
heartbeat_at: 2026-08-25 14:36Z
status: active
focus: "**LOCALFOLDED 2026-08-25 14:35Z FOR A COMPACT; STATUS STAYS `active` -- a compact does not end a session (invariant 6).** **TWO hv-ASSIGNED PIECES LANDED TODAY AND BOTH WERE MISDIAGNOSED IN THE FILING:** id normalisation (`58979836`) was not a missing feature but nine verbs answering a malformed id with a NOT-FOUND; the output handler (`3c2f50d6`, `5473a5cc`) was not a missing width check but `fill` being a MINIMUM with no maximum, in v2 and v3 alike. **GATE UNCHANGED AT 66 OF 67 -- I MOVED NONE OF IT, AND AC-08.5 STILL WAITS ON THE DENOMINATOR.** **hv RETRACTED THE ATTRIBUTION SWEEP: the `(C)` line was never required, and four nodes measured an unasked question carefully.** Nothing in flight, tree clean, all four inboxes at the sentinel."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

## THE DENOMINATOR IS IN DOUBT, AND THE FIND IS ic's -- 2026-08-25

**`Issue.body` IS A DECLARED SCHEMA FIELD THAT NEITHER DOOR CAN WRITE.** Verified both halves at source: `0077.json` carries `body: ''` with `body` among its declared keys, and **`settable_fields` matches only `Thread`/`Wp`/`Ac`/`At` -- `Issue` falls to the `other =>` arm and is refused by name.** `issues add` takes `<TITLE>` and `--severity` only.

**AND THE SWEEP CALLS `E::Issue { .. }` `Reachable`, WHICH IS TRUE.** `put` reaches it and refuses BY NAME, so the door exists. **That is an ADDRESS-axis fact. AC-08.5's subject is _every writable FIELD of every entity_.** So this is a field-axis gap on a form the gate's own instrument marks green, **and the instrument cannot see it by construction** -- `declared_reach` answers a question about addresses.

**SAME SHAPE AS THE DOOR-BLINDNESS vc RULED, ONE AXIS OVER: an instrument internally consistent, correct in its own printed output, and scoped to a NARROWER QUESTION than the criterion it serves.** And `Issue` will not be the only one -- **every form the sweep calls `Reachable` on refusal-by-name grounds is unexamined on the field axis.**

**CONSEQUENCE FOR MY OWN BUILD, AND IT IS WHY THIS IS AT THE TOP: closing limb 1's four ADDRESS forms would not close limb 1.** Do not write an arm until the denominator is settled. Going in with vc's dispositions; ic filed it as an exhibit and it is more than one.

## OUTPUT SHAPE -- LANDED `3c2f50d6` + `5473a5cc`, AND THE FILED BUG WAS AGAIN THE SMALL HALF -- 2026-08-25

**hv FILED `issues list` IGNORING THE TERMINAL WIDTH. IT WAS NOT IGNORING IT** -- `terminal_width()` existed and `issues list` already called it. **`fill` WAS A MINIMUM WITH NO MAXIMUM, SO ONE OVERSIZED CELL SET THE WIDTH OF EVERY ROW**: 312 columns into an 80-column terminal from a single 287-character title. **BOTH IMPLEMENTATIONS CARRIED THE RULE IN NEAR-IDENTICAL WORDS** -- v2 `content-fit is the floor, so nothing is ever truncated`, v3 `a narrow terminal never truncates, it just stops padding` -- **and neither comment states the consequence.** Not a regression: a shared design decision nobody had re-read. **Same shape as `%04d` being a minimum width, twice in one day.**

**THE WIDEST TITLE WAS MINE**, 287 chars, filed twenty minutes earlier with the whole finding in the TITLE because `Issue.body` is unwritable. Median is 152 and 0043 was already 269, so I was the worst offender rather than the cause -- **but the reason titles run long at all is AC-08.5's denominator surfacing as a UI defect.**

Shipped: `--format={terminal,md,json}` and `--width` through one resolver, replacing four spellings across eleven flags. `--json`/`--markdown` KEPT as aliases -- **v2 parity obligations, which is not the same as a compatibility shim, so fail-forward does not reach them.** A disagreeing pair refuses. `export --format` deliberately NOT collapsed (own projection registry, own refusals); `--quiet`/`--verbose` are volume rather than shape and stayed out; **`todo` correctly has no `--width`** -- it prints a document, so a width would mean wrapping prose and would break it as a persisted artefact.

**FIVE DEFECTS OF MY OWN, EVERY ONE FOUND BY DRIVING AND NONE BY READING.** Three are classes:

- **A DECLARED `default` IS NOT A CHOICE.** Adding `default: terminal` made clap supply `--format` unasked, so every `--markdown` read as two formats and refused. **The refusal was correct about its inputs and wrong about the world.**
- **`value_source` PANICS ON AN UNDECLARED ID WHERE `try_get_one` RETURNS `Err`** -- which is the entire reason `opt` and `flag` exist, stated in their own doc comments. I reached for a sibling API that did not share the property and put a panic on every verb without `--format`.
- **A MUTATION THAT FAILS TO MUTATE READS AS A PASSING CONTROL.** One clip mutation returned GREEN because `sed` silently no-op'd on a pattern containing my own delimiter. **A broken mutation reports the safe-looking answer.**
  And two slips: `--width 0` is the declared contract for _use the terminal_ and my first draft refused it; my clean hand-driven pass was of a binary predating my own next edit.

**THE GAP THAT REACHED hv BEFORE IT REACHED A TEST: `intent issues --width 80` REFUSED WHILE `issues list --width 80` WORKED.** The family row declares `default: list`, so the bare form is documented, shorter, and **the one anybody types first.** A default verb whose flags the default route cannot accept is this change's own inconsistency one level up. `issues` is the only family with a bare table form -- st/wp/ac/at all refuse without a subcommand. Pinned and mutation-proven at `5473a5cc`.

**AND `remedy_coverage` CAUGHT MY NEW ERROR TYPE WITH NO PROOF LINE.** A test that scans source for `thiserror` derives and demands a hand-written roster entry -- **the shape that survives a new module by an author who never read it.**

## THE ATTRIBUTION SWEEP -- RETRACTED BY hv, AND THE METHOD LESSON OUTLIVES ITS SUBJECT -- 2026-08-25

**hv RULED THE `(C)` LINE WAS NEVER REQUIRED**: _this isn't a problem, has never been a problem, and is not something that I suggested we go looking for. The only constraint is that I DO NOT WANT ANY CLAUDE EXHAUST IN MY COMMITS._ **So my census is dropped rather than filed as closed-with-no-action -- a carefully measured fact about nothing is still a fact about nothing.** Only the `^Claude-Session:` half survives, it gates, and it is dc's. Verbatim material in `.history/20260825/wip-fold-1435Z.md`.

**WHAT SURVIVES IS ABOUT METHOD AND IT SURVIVES STRONGER: I APPLIED A CONTROL TO HYPOTHESIS 1 AND NONE TO HYPOTHESIS 2, IN THE SAME MESSAGE -- AND HYPOTHESIS 2 IS THE ONE THAT REACHED A REMEDY.** I nearly sent _a compact drops it_, drove it, and killed it. **Killing the first hypothesis felt like the diligence for that message**, so I reached for a second explanation with the rigour already spent, and nothing in the paragraph distinguished the driven claim from the undriven one. **Running one control was worse than running none, because it vouched for the neighbour it sat beside.** vc falsified the second cleanly: the state my mechanism requires (`(C)` absent, trailer present) has a population of ZERO.

**AND IT LANDED ON THE WORST POSSIBLE OUTPUT.** The undriven half did not stay on my board -- **it went into a fleet message telling vc their remedy was the one thing that would not work.** The unevidenced half was the half carrying the recommendation.

**hv's SCOPE POINT IS THE OUTER LAYER AND IT LANDS ON ALL FOUR OF US: THE FIRST QUESTION TO ASK OF A MEASUREMENT IS WHO ASKED FOR IT.** Nobody asked for any of this. **Measuring an unasked question carefully is more expensive than measuring it badly**, and my scope error and my control error compound rather than sit side by side: if the question had been asked, my unrun hypothesis would still have been wrong; because it was not, being wrong cost four nodes an afternoon.

## The rule the routing left behind -- mine, and it generalises

**vc TOOK BOTH ROUTED FINDINGS AT `b5641e16` AND MY COPIES ARE CUT, WHICH IS THE POINT OF ROUTING THEM.** Verified against their board with a positive control rather than from their commit message -- **a commit message is testimony about a document.** They own the rulings; a copy here would go stale the moment either ruling moved, and the node that routed a finding away is the one that stops maintaining it. Verbatim text in `.history/20260824/wip-routed-verbatim-2120Z.md`.

**WHAT STAYS IS THE JUDGEMENT, BECAUSE IT WAS ABOUT MY OWN WORK: A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE TRYING TO CLOSE IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** My round-trip finding was true, driven, and mine -- and every one of those is a reason to want it counted. **The pull is strongest exactly when the finding is GOOD**, because a weak one gets dropped without a struggle. DC-1 was what settled it, and I would not have reached it alone: I was arguing about which limb it satisfied when the answer was neither.

## TODO

1. **`AC-08.5` -- THE DENOMINATOR SECTION ABOVE IS EVERYTHING OPERATIVE.** Gate driven 2026-08-25 14:35Z: `ac status ST0057` 50/51 + 2 withdrawn BLOCKED, `ac status ST0056/03` 16/16 + 1 withdrawn PASS, `ac gate ST0057` unsatisfied AC-08.5. **66 of 67, and I moved none of it today.** Held on hv: limb-1 dispositions, the limb-2 design call, the `Node` population citation, the biconditional cover.
2. **U3's BUILD queue** -- the family-and-subcommand delta driven **FROM THE TWO BINARIES**, never from `dispatch-table.json`. **`restart.md`'s U3 paragraph is STALE ON PRESENCE and I drove it**: `claude skills`, `lang`, `plugin`, `ext` and `version` ALL answer `--help` in v3. Presence is not parity -- `intent init --lang` refuses with _`intent lang init` is not implemented yet_, which is the real shape of the gap. **Routed to vc (globalfold); do not edit `restart.md`.** **Confirm `treeindex` absent BY POPULATION, not by one grep.**
3. **HELD ON hv -- DO NOT BUILD.** `sync` skipping untracked bytes; the relay wording is close to what AC-03.6 names as insufficient. **dc's caution is the sharp part: the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED, or a legitimate two-step workflow becomes a silent no-op.**

**CLOSED TODAY, NOT CARRIED:** id normalisation landed `58979836` with vc minting AC-06.12 / AT-06.12 / EXP-10 at `028c3697`; the output handler landed `3c2f50d6` + `5473a5cc`; **ic's routed `FacadeError` question is ANSWERED BY A FINDING RATHER THAN A RULING** -- the refuse-by-name shape already ships on the issue door, so it was never a new error type to design.

## BLOCKED

**NOTHING OF MINE IS BLOCKED, AND NOTHING IS BLOCKED ON ME.** This section claimed otherwise for a full day after it stopped being true, while the contradiction sat twelve lines above it. **A board can hold its own contradiction and read fine, because nobody reads two sections against each other** -- and the entry that goes stale is never the one being edited.

## Mine -- what `intent/restart.md` does not carry

**THE FIVE I ROUTED TO vc LANDED AT `16d58112` AND ARE CUT FROM HERE, WHICH IS THE POINT OF ROUTING THEM.** Keeping a copy after the destination took them would be the two-homes defect committed one step after fixing it. **Verified before cutting, with a positive control**: quotation-is-testimony (carrying the-envelope-beats-the-byline), `FIXED`-is-not-a-state, the recorded-reason join, a-correct-refusal-is-not-a-save, and format-then-sync-then-commit are all in `intent/restart.md`'s traps.

1. **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE** _(estate-wide; ROUTED TO vc 2026-08-24, not yet landed -- checked, absent from `intent/restart.md`)_. A bare `AC-03.6` is GREEN in ST0056 and RED in ST0057. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE**, so the reader stops rather than asking.
2. **DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc, 2026-08-24 16:57Z -- their stamp, attributed). **Deliberately NOT routed: it is vc's ruling about what I may write into ST0056 canon, so its home is the board of the node it CONSTRAINS** -- vc agreed and called that the sharper line. An attachment hash is DERIVED from a file I committed and `canon_commit_check` already gates it; **a checkpoint with no failure mode it can catch is a queue.** What routes through vc is AUTHORED: criterion text, `state`, `status`, notes, `covers`.
3. **A PARITY TOOL IS A RECORDED ATTACHMENT.** Write the file, add the roster row, `sync --to-store ST0056`, **read the rc directly**, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` and commit together. **Roster row and runner must be ONE commit; either alone disagrees**, and the check reads the INDEX, so an unstaged new file reads as an unrostered tool.

## Today's lessons, and every one was paid for -- 2026-08-25

1. **A PROSE EDIT TO A GENERATED ARTEFACT HAS READERS. ENUMERATE WHAT READS THE _VALUE_, NOT WHAT NAMES THE IDENTIFIER.** I reworded `_CLAUDE.md`'s footer and split `CANON_INTENT_FOOTER_MARK`, a contiguous substring of that prose -- **every project upgrading afterwards would have been permanently declassified as user-authored, and the tool's documented remedy `--force` destroys the `user:start` block.** vc's grep for the two IDENTIFIERS found nothing because the live reader asserts the string's VALUE; **its silence was a fact about the pattern.** vc has adopted the same rule.
   **AND THE AFTERMATH IS ITS OWN INSTANCE, WHICH IS WHY THIS ENTRY IS LONGER THAN THE LESSON: ic DROVE THE MARKER _AS I HAD REPAIRED IT_, GOT THREE GREENS, AND CONCLUDED THE HAZARD NEVER EXISTED** -- _had that marker keyed on the WORDING rather than the path, your `:54` edit would have de-recognised every consumer at once ... it did not, because the marker is path-based._ **It DID key on the wording, it DID de-recognise every consumer, and path-based is what I changed it TO.** A true measurement of the post-fix state, offered as evidence about the pre-fix state. **Second time in one day a peer read my repair as proof there was nothing to repair** -- and ic's own closing hedge (_the safety may be someone's earlier work rather than luck_) was the right instinct published one trace too early. **A fix erases its own evidence, so the state you can measure is never the state that failed.**
   **AND ic's REPLY IS THE BETTER RULE AND IT IS THEIRS: A HEDGE IS NOT A LICENCE. IF YOU CAN NAME THE CHECK THAT WOULD SETTLE IT, IT IS A TODO AND NOT A CAVEAT.** They wrote _the safety may be someone's earlier work rather than luck_, **named the exact check, did not run it, and published the conclusion the check refutes** -- and their own diagnosis is the sharp half: **writing the caveat down converted _I know I have not checked_ into _I am covered_, which is worse than not noticing, because a caveat reads as diligence to everyone including its author.** One `git log -1 -- <file>` would have closed it. **Recorded here because it arrived AFTER my fold, which is the case that passes every must-survive list.**
2. **A MISSING TEST IS A BACKLOG ITEM; AN UNRUN TEST IS A DISCIPLINE FAILURE, AND ONLY ONE IS FIXED BY WRITING MORE TESTS.** All three of us first said the property was unasserted. **`intent_claude_upgrade.bats:86` asserted the old wording and went red immediately** -- it had been there all along, in the file the new test was going into. **Run the test file that covers what you changed.**
3. **`init` READS THE EMBED; `render` READS DISK.** `embed_templates.rs` emits a per-file `include_str!`, so a template edit reaches `init` only after a rebuild and reaches `render` at once. **I generalised from a correct observation about `init` to a false rule about rendering; ic narrowed it and I drove their alternative confound dead.** `[[INTENT_VERSION]]` is `ctx.version` at `rootfiles.rs:343` -- **the CALLER's choice**, and all four shipped originating sites pass `env!` by convention with nothing enforcing it.
4. **zsh COST ME FOUR PROBES IN ONE DAY AND EVERY ONE ANSWERED PLAUSIBLY.** Unquoted `$v` (no word-split, `claude skills` arrived as one arg and would have CONFIRMED a stale claim I was refuting); `head -5` read as an absence; unquoted `--include=*.rs` dying on the glob and printing nothing; nested quotes in a probe. **Write the probe to a file instead of fighting the quoting**, and **gate a commit on its check with `&&`** -- ic's remedy, and my board blocks had exactly that defect. **A FOURTH ARRIVED AFTER THIS WAS WRITTEN AND IN THE ACT OF AVOIDING A DIFFERENT DEFECT:** I built an explicit path list to avoid vc's `git add -A`, passed it as unquoted `$P`, and **seven paths arrived as ONE pathspec, so the commit did not happen at all.** **The safe form and the unsafe form fail in opposite directions -- `-A` commits more than you meant, an unquoted list commits nothing -- and only one of those tells you.**

5. **A CLAIM ABOUT A FILE ANOTHER NODE WRITES HAS A SHELF LIFE OF ZERO, AND MY FOLD SHIPPED ONE.** I wrote _all four inboxes at the sentinel_ into the focus line of `40a0d191`. It was true when I cleared them at `cb142147` and false by the time I committed: **dc appended their guards announce 32 seconds earlier at `851ae74a`.** Nothing was wrong with the observation; it was simply about a file whose single writer is somebody else, **so the interval in which it stayed true was bounded by dc's typing and not by anything I control.** The board's own discipline is _verify in the COMMIT, not the file_, and I applied it to the sections I cut and not to the sentence describing my inboxes. **Re-read a peer-written file at the moment you assert its state, not at the moment you last set it.**

6. **`git checkout -- <file>` RESTORES TO HEAD, NOT TO BEFORE YOUR MUTATION, AND I USED IT AS AN UNDO ON A FILE I HAD UNCOMMITTED WORK IN.** Mutation-proving the bare-form arm, I reverted `surface/dispatch-table.json` that way and **destroyed my own uncommitted flag declaration rather than the mutation** -- the restore was faithful to HEAD and HEAD had never seen my work. Cheap to redo and it was not cheap to notice: the test went red on the RESTORE, which reads as a broken test rather than a lost edit. **Second time today a restore step lied to me about which state I was in**, after `mv` preserving a backup's mtime served a binary built from the mutation. **A restore is a claim about a baseline, and both times I had the wrong baseline in mind rather than the wrong command.**

## Lane and build recipe -- cc-specific

`native/**` and the v3 crates are mine. Parity harness = ic. Hooks, roster, `int hooks` = dc. **Canon writes route through vc.**

- **`CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc` FOR ANY VERIFYING BUILD** -- absolute and in-repo, and per-node so four sessions do not fight one lock. Out-of-repo breaks `INTENT_HOME` resolution (`install::home()` walks `current_exe()` ancestors for a marker); relative under a drifted cwd once built **1.2G** where gitignore hid it.
- **`rustfmt --edition 2024`, NEVER a bare `cargo fmt`.**
- **Drive v3 as `./native/rust/target/debug/intent`.** `intent3` on PATH is the RELEASE build and can lag it.
- **THE CLOCK AND HEADER GUARDS GOT STRICTER ON 2026-08-25 AND A REFUSAL IS NOW PROBABLY REAL** (dc, `e28c215b`/`db15e857`). Their `did THIS COMMIT add it` filter was `printf | grep -q`; under `pipefail` a SIGPIPE'd `printf` made the test read FALSE, so **each guard was sound whenever there was nothing to catch and unsound exactly when there was.** If one of my commits is refused with a clock or header finding, fix the finding -- do not assume dc broke the guard.
- **A TEMPLATE EDIT DOES NOT REACH `init` UNTIL THE BINARY IS REBUILT** (`include_str!` embed). `strings <binary>` tells you which copy it carries -- **"which binary" is a different question from "which version".**
