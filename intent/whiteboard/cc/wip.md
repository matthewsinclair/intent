---
node: cc
name: Control Claude
role: control
session_id: 22d8657d-6ffd-4379-90c8-702faa53a828
heartbeat_at: 2026-08-25 20:53Z
status: active
focus: "**LOCALFOLDED 2026-08-25 20:52Z FOR A COMPACT; STATUS STAYS `active` -- a compact does not end a session (invariant 6).** **ST0057's GATE IS CLOSED AT 67 OF 67 AND AC-08.5 IS GREEN** -- vc greened it at `7652f49a` and re-drove it on the shared pair after my `8957261a`. **BOTH LIMBS WERE MINE AND I REFUSED MY OWN GREEN FOUR TIMES.** **GATE CLOSED IS NOT RELEASED; hv OWNS THAT.** ST0056 is 63/133 and untouched, which is NOT this gate's denominator. Nothing in flight, tree clean, all four inboxes at the sentinel, shared pair coherent at `8957261a`."
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

## TODO

1. **U3's BUILD queue** -- the family-and-subcommand delta driven **FROM THE TWO BINARIES**, never from `dispatch-table.json`. **`restart.md`'s U3 paragraph is STALE ON PRESENCE and I drove it**: `claude skills`, `lang`, `plugin`, `ext` and `version` ALL answer `--help` in v3. Presence is not parity -- `intent init --lang` refuses with _`intent lang init` is not implemented yet_, which is the real shape of the gap. **Routed to vc (globalfold); do not edit `restart.md`.** **Confirm `treeindex` absent BY POPULATION, not by one grep.**
2. **HELD ON hv -- DO NOT BUILD.** `sync` skipping untracked bytes; the relay wording is close to what AC-03.6 names as insufficient. **dc's caution is the sharp part: the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED, or a legitimate two-step workflow becomes a silent no-op.**
3. **`Node`'s MODEL IS HELD WITH hv AND THE DEFERRAL IS SAFE BY CONSTRUCTION.** Whether the model should carry a `Node` type at all is ST0056/WP-14's scope. **The AC-08.5 partition sizes are pinned in both directions, so if WP-14 ever reifies it the cover REDS and announces itself** rather than the row going quietly stale.

**CLOSED TODAY, NOT CARRIED:** AC-08.5 both limbs and the gate; id normalisation (`58979836`, row minted by vc at `028c3697`); the unified output handler (`3c2f50d6`, `5473a5cc`); `st attach` text and bytes (`41acbe38`, `c4709d3f`); the `created` stamp refusal (`3b3f5bd3`); the Highlander fold of my own second instrument home (`12cced26`). **ic's routed `FacadeError` question was ANSWERED BY A FINDING RATHER THAN A RULING** -- the refuse-by-name shape already shipped on the issue door.

## BLOCKED

**NOTHING OF MINE IS BLOCKED, AND NOTHING IS BLOCKED ON ME.** This section claimed otherwise for a full day after it stopped being true, while the contradiction sat twelve lines above it. **A board can hold its own contradiction and read fine, because nobody reads two sections against each other** -- and the entry that goes stale is never the one being edited.

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
