---
node: cc
name: Control Claude
role: control
session_id: 22d8657d-6ffd-4379-90c8-702faa53a828
heartbeat_at: 2026-08-25 22:03Z
status: active
focus: "**WP-06 WITH vc's PEN. TWO LANDINGS: `540d92bb` `version` (both spellings byte-identical, one capability), `0d77e337` `plugin` (byte-identical to v2, and the line I did NOT port is the finding -- as-observed fidelity would have shipped a remedy naming a RETIRED verb).** **AC-06.1's remainder is 36 of 105, and my own first answer was 26** -- 11 hid in the optimistic direction behind a CLAP arity trap. **BOTH REMAINING PATHS ARE BLOCKED ON RULINGS AND NEITHER IS BLOCKED ON ME**: `lang` held by vc, `modules` awaiting two answers, and `CLAUDE.md:52` surfaced to hv because I do not edit that file on a peer's instruction. Tree carries none of my work uncommitted."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

## WHERE THE GATE STANDS -- DRIVE IT, DO NOT READ IT FROM HERE

**ST0057 CLOSED 2026-08-25 AT 67 OF 67.** `ac status ST0057` 51/51 + 2 withdrawn PASS; `ac status ST0056/03` 16/16 + 1 withdrawn PASS; `ac gate ST0057` PASS rc=0. **`ac status ST0056` answers 63/133 and is NOT this number's denominator** -- the third call is the one nobody writes down, and omitting it is how the wrong figure gets produced.

**GATE CLOSED IS NOT RELEASED. hv OWNS THAT AND NOBODY TAGS OR PUSHES.**

**THREE CAVEATS SIT ON THE ROW RATHER THAN IN A MESSAGE**, which ic and I both asked for: issue 0082 (a canon-first attachment the working tree does not hold -- `st attach --from` outside the thread dir reaches it, reporting `ok`), the refusing round trip that makes 0082 safe rather than lossy, and 0084 (the ingest/collector split) untouched. **A green whose caveat lives only in a message is unfinished.**

## LESSONS THAT OUTLIVE THE DAY THEY WERE PAID FOR -- 2026-08-25

### The two that are new today

**EIGHT FALSE REMEDIES ACROSS THREE NODES, EACH AUTHORED INSIDE THE FIX FOR THE PREVIOUS.** `settable_fields` calling an Issue a collection; `WholeBody` naming `intent put`, which is not a command; `Issue.created` naming a verb that creates issues and cannot move the field on one that exists; my `blob` remedy claiming `sync --to-store` picks up a dropped file; `finding.rs:267` telling an operator to move away an artefact Intent was built to keep.

**TWO WERE MINE AND BOTH WERE THE SUBCLASS THAT MATTERS: TRUE WHEN WRITTEN.** `blob`'s _there is no route on this surface today_ was honest, and `st attach` falsified it within the hour -- **`today` was the honest word AND the word that dates.** Then `text` and `blob` shared one sentence whose PUT clause fitted one member and named, for the other, **a route that cannot carry the form at all** (`put` takes a `&str`). **Withheld rather than reworded, with the distinction in the type, because rewording leaves the next member to inherit whichever half happens to fit.** **A REMEDY THAT WAS NEVER TRUE IS CAUGHT BY REVIEW; ONE THAT EXPIRES IS CAUGHT BY NOBODY** -- both of mine were found by driving them again later, and the second AFTER the row went green, which is the only moment nobody is looking.

**A THIRD WAY A GREEN IS ABOUT THE WRONG SUBJECT, AND IT LEAVES NO EVIDENCE.** The familiar two: **an instrument that cannot reach its subject** (a `.dat` probe against `ATTACHMENT_EXTENSIONS`; a wait condition matching `Finished` on a COMPILE line and reading 591 of 1080; `pgrep -f` matching peer argv, which caught vc and me hours apart), and **a binary carrying a stale copy of one** (vc's build baking in my uncommitted table; my own `recoverability` fix read as failed against a stale artefact). **THE THIRD IS: CORRECT INSTRUMENT, CORRECT BINARY, CORRECT ANSWER -- MEASURED BY THE PARTY WHOSE WORK IT CERTIFIES.** Nothing mechanical distinguishes it from a peer's run; only who ran it, and the other two leave evidence while this one leaves none. **Re-driving after your own change is a duty to REPORT that the tree moved, never a licence to re-certify it.** vc's sentence: _a green is a claim about a tree, and the tree moved._

### The three from the WP-06 evening

**A MECHANISM UPSTREAM OF THE SUBJECT ANSWERING IN THE SUBJECT'S VOICE -- AND THE SHARP FORM IS WHEN IT ANSWERS CORRECTLY.** Four instances in one evening across four tools: vc's `2>/dev/null | grep -c` reporting `0` for a binary that never ran; my CLAP arity error read as the renderer answering LIVE (11 entries, optimistic direction); dc's `--to-disk` `ok`s over a store that did not hold what the disk held; and ic's zsh-unsplit `$c` sending `lang init rust` as ONE subcommand name. **Then the sharpest: vc's staleness marker refused, printed its refusal twice, and their probe reported `BYTES IDENTICAL: yes` -- which was TRUE, of two copies of the refusal.** Not a wrong value. A correct value about the wrong subject, carrying no signal that it was. **THE REMEDY IS THAT AN INSTRUMENT MUST ASSERT THE ARTEFACT IS THE ONE UNDER TEST, NOT MERELY THAT TWO OF ITS OUTPUTS AGREE** -- my own `version` probes have that hole and would have agreed with themselves perfectly against a stale binary.

**AND THE NOTE DID NOT PREVENT IT.** The zsh trap is written down in the estate's memory, verbatim, and ic hit it twice in one evening and I hit it seven times. **Every save tonight came from a control that made the wrong answer impossible, not from having read the warning** -- my mutation harness aborting on pattern-absent, ic's needle control, dc's population floor, vc's mutation test. **Write the control, not the note.**

**A DETECTOR WHOSE GRANULARITY IS THE FAMILY FORCES THE LANDING GRANULARITY TO BE THE FAMILY.** `flag_reachability` decides wired-ness by driving `intent <family>` BARE, so the first verb that answers flips the whole family to WIRED and moves every `keep` flag out of the not-gated bucket into the gated one. **Wiring half a family therefore MANUFACTURES declared-and-unread violations.** Hit on `lang` and again on `modules`, which makes it a property rather than a quirk -- and it is why the long tail cannot be sliced the way a porting queue could. **The check refusing to let me half-ship is the guard working, not an obstacle.**

**AS-OBSERVED FIDELITY CAN SHIP A DEFECT, AND THAT IS THE DIRECTION NOBODY WATCHES.** v2's `plugin show` closes with `Run 'intent help <name>'`; `intent help` is retired in v3. **A faithful port ships a remedy pointing at a refusal** -- AC-06.11's own class, arriving through care rather than through carelessness. The pin is to assert the PREMISE alongside the absence: the test drives `intent help` and requires it to refuse, so if that changes the test fails loudly instead of the decision outliving its reason.

### The ones from earlier that still bind

1. **A PROSE EDIT TO A GENERATED ARTEFACT HAS READERS. ENUMERATE WHAT READS THE _VALUE_, NOT WHAT NAMES THE IDENTIFIER.** I reworded `_CLAUDE.md`'s footer and split `CANON_INTENT_FOOTER_MARK`, a contiguous substring of that prose -- every project upgrading afterwards would have been permanently declassified as user-authored. **A grep for the two IDENTIFIERS found nothing because the live reader asserts the string's VALUE.**
2. **A HEDGE IS NOT A LICENCE (ic's).** If you can name the check that would settle it, it is a TODO and not a caveat -- **writing the caveat down converts _I know I have not checked_ into _I am covered_.**
3. **A MISSING TEST IS A BACKLOG ITEM; AN UNRUN TEST IS A DISCIPLINE FAILURE**, and only one is fixed by writing more tests. **Run the test file that covers what you changed.**
4. **A MUTATION THAT FAILS TO MUTATE READS AS A PASSING CONTROL.** Assert the pattern is present before replacing it; a no-op `sed` reports the safe-looking answer. Restore with `cp` + `touch`, never `mv` (it preserves the backup's mtime) and **never `git checkout --`, which restores to HEAD and destroys uncommitted work rather than the mutation.**
5. **zsh DOES NOT WORD-SPLIT AND IT COST ME FIVE PROBES.** Unquoted `$v` arrives as ONE argument; an unquoted path list becomes one pathspec and the commit does not happen at all. **Write the probe to a file rather than fighting the quoting.**
6. **A CLAIM ABOUT A FILE ANOTHER NODE WRITES HAS A SHELF LIFE OF ZERO.** Re-read a peer-written file at the moment you assert its state, not at the moment you last set it.
7. **A DECLARED `default` IS NOT A CHOICE**, and `value_source` panics on an undeclared id where `try_get_one` returns `Err` -- which is the whole reason `opt` and `flag` exist.
8. **DROPPING A TEST BECAUSE A SIMILARLY-NAMED ONE EXISTS IS THE DAY'S CLASS.** ic's file had no positive control for a thread `put` and drove none of the CLI lifecycle tails; **the seven-verb finding would have been folded away with the file that found it.**

## The rule the routing left behind -- mine, and it generalises

**vc TOOK BOTH ROUTED FINDINGS AT `b5641e16` AND MY COPIES ARE CUT, WHICH IS THE POINT OF ROUTING THEM.** Verified against their board with a positive control rather than from their commit message -- **a commit message is testimony about a document.** They own the rulings; a copy here would go stale the moment either ruling moved, and the node that routed a finding away is the one that stops maintaining it. Verbatim text in `.history/20260824/wip-routed-verbatim-2120Z.md`.

**WHAT STAYS IS THE JUDGEMENT, BECAUSE IT WAS ABOUT MY OWN WORK: A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE TRYING TO CLOSE IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** My round-trip finding was true, driven, and mine -- and every one of those is a reason to want it counted. **The pull is strongest exactly when the finding is GOOD**, because a weak one gets dropped without a struggle. DC-1 was what settled it, and I would not have reached it alone: I was arguing about which limb it satisfied when the answer was neither.

## WP-06 -- WHAT LANDED TONIGHT, AND THE NUMBER THAT REPLACED MINE

**vc GAVE ME WP-06 WITH THE PEN AT 21:20Z. TWO LANDINGS, BOTH `--only`, BOTH MUTATION-PROVED.**

- **`540d92bb` -- `version`.** `--version` and `version` now print BYTE-IDENTICAL stdout, both rc=0, because the arm asks clap for `render_version()` rather than composing a line. **Wiring the subcommand alone would have closed the rc gap and left the harder defect: two spellings that both answer, differently, with nothing saying so.** Version string gains the baked commit per the `corrected` ratification. `version --zzz` exits 1 (INV-02) where v2 took it at 0 (INV-08). **ST0058 AC-00.6 is satisfiable and I did NOT satisfy it** -- ic verifies, vc authored, I built; three hands.
- **`0d77e337` -- `plugin`.** Three entries, `plugin list` byte-identical to v2, bare == list. **The finding is the line I did NOT port**: v2's `plugin show` ends `Run 'intent help <name>'` and **`intent help` is RETIRED in v3, refusing at rc=2**. As-observed fidelity would have shipped a remedy pointing at a refusal -- AC-06.11's class arriving through FAITHFULNESS, which is the direction nobody watches. The test pins BOTH halves, so if `help` is ever un-retired it fails and sends someone back to the decision instead of outliving its premise.

**AC-06.1's REMAINDER IS 36 UNWIRED ENTRIES OF 105 SHIPPED, AND MY FIRST ANSWER WAS 26.** Driven from the binary, not the table. **The 11 hid in the OPTIMISTIC direction**: my probe drove every entry with NO ARGUMENTS, so any verb with a required positional died inside CLAP before a renderer arm ran, and I read clap's error as the renderer answering LIVE. `modules find`, `lang init`, `lang show`, `lang remove`, `config get`, `config set`, `ext new`, `ext show`, `plugin show`, `claude start`, `st dehydrate`. **I found it because I went to check whether a CLAUDE.md claim had expired and it had NOT.**

**THE COST TABLE IS THE USEFUL ARTEFACT AND IT IS IN vc's HANDS**: per family, unwired entries x `keep` flags x blocking rows. **`plugin` was the ONLY unwired family with no design question attached.** That is the finding vc took to hv: **the long tail is not a porting queue, it is a queue of RULINGS with porting attached** -- so the bottleneck is vc and hv, not me.

## TODO

1. **`lang` -- HELD BY vc 22:0xZ, DO NOT BUILD.** Ruled (init declares in `config.json` and installs nothing; `list` keeps v2's available-packs semantics; `sync` + `sync --check` RETIRED; three rows to `corrected`), then held pending whether a retirement landed on disk anywhere -- **a table row saying RETIRED when the truth is UNBUILT is exactly what AC-00.5 exists to make impossible, and I would build against it within the hour.** Nothing of it is on disk; the table is clean. **When it lifts: land the config-write prerequisite FIRST as its own commit.**
2. **`modules` -- TWO QUESTIONS OUT WITH vc.** `find` is clean and closes issue 0067. `check` is not: **its scan population is `bin/intent_*` and `AC-12.1` PRUNES `bin/` at the cut**, so a faithful port builds a scanner for files v3 deletes; and `--register` is a `keep` flag declared _interactive_, so AC-06.8 forces either an interactive prompt or a retirement. **`find` cannot ship without `check`** (see the family-granularity finding below).
3. **`CLAUDE.md:52`'s `modules find` PARENTHETICAL IS SURFACED TO hv, NOT EDITED.** It goes false the moment the verb is wired. vc asked me to land the edit with the verb and is right on the merits; **I do not edit `CLAUDE.md` on a peer's instruction** -- that file is named in my operating instructions alongside permission settings, and a guard bypassed when the reasoning is good is not a guard. hv's call.
4. **U3's BUILD queue** -- superseded in substance by the 36-entry measurement above, which was driven FROM THE BINARY. **Routed to vc for globalfold; do not edit `restart.md`.**
5. **`sync` skipping untracked bytes -- STILL HELD ON hv, DO NOT BUILD.**
6. **`Node`'s model is held with hv (WP-14) and the deferral is safe by construction** -- the AC-08.5 partition sizes are pinned in both directions, so a reification REDS the cover rather than going quietly stale.

## BLOCKED

**BOTH BUILD PATHS ARE BLOCKED ON RULINGS, AND NEITHER IS BLOCKED ON ME.** `lang` held by vc; `modules` awaiting two answers; the `CLAUDE.md` line awaiting hv. **Nothing of mine is in flight and the tree carries none of my work uncommitted.** This section claimed the opposite for a full day in August while the contradiction sat twelve lines above it -- **a board can hold its own contradiction and read fine, because nobody reads two sections against each other.**

## Mine -- what `intent/restart.md` does not carry

**THE FIVE I ROUTED TO vc LANDED AT `16d58112` AND ARE CUT FROM HERE, WHICH IS THE POINT OF ROUTING THEM.** Keeping a copy after the destination took them would be the two-homes defect committed one step after fixing it. **Verified before cutting, with a positive control**: quotation-is-testimony (carrying the-envelope-beats-the-byline), `FIXED`-is-not-a-state, the recorded-reason join, a-correct-refusal-is-not-a-save, and format-then-sync-then-commit are all in `intent/restart.md`'s traps.

1. **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE** _(estate-wide; ROUTED TO vc 2026-08-24, not yet landed -- checked, absent from `intent/restart.md`)_. A bare `AC-03.6` is GREEN in ST0056 and RED in ST0057. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE**, so the reader stops rather than asking.
2. **DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc, 2026-08-24 16:57Z -- their stamp, attributed). **Deliberately NOT routed: it is vc's ruling about what I may write into ST0056 canon, so its home is the board of the node it CONSTRAINS** -- vc agreed and called that the sharper line. An attachment hash is DERIVED from a file I committed and `canon_commit_check` already gates it; **a checkpoint with no failure mode it can catch is a queue.** What routes through vc is AUTHORED: criterion text, `state`, `status`, notes, `covers`.
3. **A PARITY TOOL IS A RECORDED ATTACHMENT.** Write the file, add the roster row, `sync --to-store ST0056`, **read the rc directly**, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` and commit together. **Roster row and runner must be ONE commit; either alone disagrees**, and the check reads the INDEX, so an unstaged new file reads as an unrostered tool.

## Lane and build recipe -- cc-specific

`native/**` and the v3 crates are mine. Parity harness = ic. Hooks, roster, `int hooks` = dc. **Canon writes route through vc.**

- **`CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc` FOR ANY VERIFYING BUILD** -- absolute and in-repo, and per-node so four sessions do not fight one lock. Out-of-repo breaks `INTENT_HOME` resolution (`install::home()` walks `current_exe()` ancestors for a marker); relative under a drifted cwd once built **1.2G** where gitignore hid it.
- **`rustfmt --edition 2024`, NEVER a bare `cargo fmt`.**
- **Drive v3 as `./native/rust/target/debug/intent`.** `intent3` on PATH is the RELEASE build and can lag it.
- **THE CLOCK AND HEADER GUARDS GOT STRICTER ON 2026-08-25 AND A REFUSAL IS NOW PROBABLY REAL** (dc, `e28c215b`/`db15e857`). Their `did THIS COMMIT add it` filter was `printf | grep -q`; under `pipefail` a SIGPIPE'd `printf` made the test read FALSE, so **each guard was sound whenever there was nothing to catch and unsound exactly when there was.** If one of my commits is refused with a clock or header finding, fix the finding -- do not assume dc broke the guard.
- **A TEMPLATE EDIT DOES NOT REACH `init` UNTIL THE BINARY IS REBUILT** (`include_str!` embed). `strings <binary>` tells you which copy it carries -- **"which binary" is a different question from "which version".**
