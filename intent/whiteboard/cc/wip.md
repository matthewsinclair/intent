---
node: cc
name: Control Claude
role: control
session_id: 22d8657d-6ffd-4379-90c8-702faa53a828
heartbeat_at: 2026-08-25 22:24Z
status: active
focus: "**LOCALFOLDED 2026-08-25 22:24Z FOR A COMPACT; STATUS STAYS `active` -- a compact does not end a session (invariant 6).** WP-06 with vc's pen: four landings, `version` and `plugin` wired, and the config-write prerequisite in. **`lang` AND `modules` ARE BOTH FULLY RULED, SEQUENCED AND NOT STARTED** -- pick either up cold from DOING below. **AC-06.1's remainder is 36 of 105 and my own first answer was 26.** Tree clean, nothing in flight, nothing blocked on me."
claims: [ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

## DOING

**WP-06, THE CLI PARITY LONG TAIL. vc GAVE ME THE PEN AT 21:20Z 2026-08-25.**

**AC-06.1's REMAINDER IS 36 UNWIRED ENTRIES OF 105 SHIPPED.** Driven from the binary, never from the table. **My own first answer was 26 and the eleven hid in the OPTIMISTIC direction** -- see Watch-outs. Re-drive it, do not read the number from here.

**LANDED (git holds the detail; this holds only what git does not):**

- `540d92bb` **`version`** -- both spellings byte-identical because the arm asks clap for `render_version()`. **ST0058 `AC-00.6` is satisfiable and I did NOT satisfy it** -- ic verified it and my half PASSES; the ROW still fails on the OTHER twin (`intent help` rc=2 vs `--help` rc=0), filed by vc as **issue `0086` HIGH with three fixes ruled. Queued, not assigned, and it lands near dispatch-table work.**
- `0d77e337` **`plugin`** -- three entries, `plugin list` byte-identical to v2.
- `e63813a2` **config-write prerequisite** -- `Config::declare_language`/`undeclare_language` + atomic `write_config`. **This was the first mutation route to `config.json` in v3 at all.**

**NEXT, EITHER ORDER, BOTH CLEARED AND BOTH ATOMIC -- START COLD FROM HERE:**

1. **`lang` -- SIX ARMS, FIVE TABLE EDITS, TWO SKILL.md LINES, ONE COMMIT.** vc ruled it and lifted the hold. `init <lang>` **declares in `config.json` and installs NOTHING under `intent/llm/`** (issue 0068's "nothing to install" close; rules are served live by `intent claude rules list --lang <lang>`). **DO NOT TOUCH the agnostic pair `intent/llm/RULES.md` + `ARCHITECTURE.md`** -- 4 of 4 estates AUTHORED theirs; same directory, adjacent names, opposite verdict. `list` KEEPS v2's available-packs semantics (vc conceded this; `as-observed` holds). `show` reports DECLARED. **`sync` and `sync --check` are RETIRED** -- `init` installs nothing, so there is nothing to converge, and pointing the RULES.md block at the live rules would leave a SECOND HOME for the `languages` array. Table: `init`/`remove`/`show` -> `corrected`, `sync` + its `--check` flag -> retired, `list` unchanged, **vc's ruling as the ratification reference, in the same commit as the verb.** The two SKILL.md lines (`in-author-essentials:64`, `in-content-essentials:65`) **state the INVARIANT, not the mechanism** -- point at `intent claude rules list --lang <lang>`, which is true on both sides of the cut.
2. **`modules` -- `find` + `check`, ONE COMMIT.** `find` closes issue `0067`. **`--register` is RETIRED**: issue `0071` (HIGH, open) is v2's `upgrade` blocking on an interactive read with no TTY, the fleet is headless, and _register before you code_ is a deliberate act whose automation makes it a rubber stamp. **`check`'s population is DERIVED FROM THE PROJECT'S DECLARED `languages`, not ported** -- v2 scans `bin/intent_*`, which is INTENT'S OWN LAYOUT shipped to everyone as though general, so `AC-12.1` pruning `bin/` is not a collision but that population having always been wrong for everyone else. **`CLAUDE.md:52`'s parenthetical goes false when `find` lands and is WITH hv, not with me** (see Decisions).

## TODO

1. **`sync` skipping untracked bytes -- HELD ON hv, DO NOT BUILD.** dc's caution is the sharp part: the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED, or a legitimate two-step workflow becomes a silent no-op.
2. **`Node`'s model is held with hv (ST0056/WP-14) and the deferral is SAFE BY CONSTRUCTION** -- the AC-08.5 partition sizes are pinned in both directions, so a reification REDS the cover rather than letting the row go quietly stale.
3. **dc's `VIEW_NAMES` question -- ANSWERED, NOTHING OWED BY ME.** Routed as _check whether this is still true after cc lands_. Driven: `VIEW_NAMES` occurs only in `address.rs` (357, 394); `project.rs` never references it, and `e63813a2` was 250 pure insertions touching `classify` zero times. **So it is still true and my work did not change it.** If the two memberships are legitimately different populations the DOC is the defect; that is dc's call, not mine.
4. **U3's build queue** -- superseded in substance by the 36-of-105 measurement, which was driven FROM THE BINARY. Routed to vc for globalfold; **do not edit `restart.md`.**

## Watch-outs

**A MECHANISM UPSTREAM OF THE SUBJECT, ANSWERING IN THE SUBJECT'S VOICE -- FIVE INSTANCES IN ONE EVENING ACROSS FOUR NODES.** vc's `2>/dev/null | grep -c` reporting `0` for a binary that never ran; my CLAP arity error read as the renderer answering LIVE (11 entries hidden, optimistic direction); dc's `--to-disk` `ok`s over a store that did not hold what the disk held; ic's zsh-unsplit `$c` sending `lang init rust` as ONE subcommand name. **And the sharpest form: vc's staleness marker refused, printed its refusal twice, and their probe reported `BYTES IDENTICAL: yes` -- which was TRUE, of two copies of the refusal.** Not a wrong value. **A CORRECT value about the WRONG SUBJECT, carrying no signal that it was.** **THE REMEDY: AN INSTRUMENT MUST ASSERT THE ARTEFACT IS THE ONE UNDER TEST, NOT MERELY THAT TWO OF ITS OUTPUTS AGREE** -- my own `version` probes have that hole.

**WRITE THE CONTROL, NOT THE NOTE.** The zsh trap is in this estate's memory verbatim; ic hit it twice in one evening and I hit it seven times. **Every save that evening came from a control that made the wrong answer impossible** -- my mutation harness aborting on pattern-absent, ic's needle control, dc's population floor, vc's mutation test. And vc's `lang rows: 0` was caught by **a peer's independent measurement disagreeing with their instrument**, which is the one save that does not scale.

**NEVER START AN ATOMIC EDIT TO AN `include_str!`'d FILE YOU CANNOT CONFIDENTLY FINISH.** `surface/dispatch-table.json` is embedded at `dispatch.rs:45`; **my uncommitted edit to it poisoned every node's `intent3` on 2026-08-25**, and my revert did not fix it because the BINARY was already poisoned. Both remaining WP-06 pieces require exactly that edit under a one-commit condition. **Stop and say so rather than leave it mid-edit** -- this is why I stopped on 2026-08-25 rather than starting `lang` at low context.

**A DETECTOR WHOSE GRANULARITY IS THE FAMILY FORCES THE LANDING GRANULARITY TO BE THE FAMILY.** `flag_reachability` decides wired-ness by driving `intent <family>` BARE, so the first verb that answers flips the whole family to WIRED and moves every `keep` flag into the gated bucket. **Wiring half a family MANUFACTURES declared-and-unread violations.** Two independent instances (`lang`, `modules`), so it is a property, not a quirk -- and it prices every remaining family as one atomic commit.

**AS-OBSERVED FIDELITY CAN SHIP A DEFECT.** v2's `plugin show` closes with `Run 'intent help <name>'`; `intent help` is RETIRED in v3. A faithful port ships a remedy pointing at a refusal -- AC-06.11's own class arriving through CARE rather than carelessness. **Pin the PREMISE alongside the absence**: the test drives `intent help` and requires it to refuse, so the decision cannot outlive its reason in silence.

**A REMEDY THAT WAS NEVER TRUE IS CAUGHT BY REVIEW; ONE THAT EXPIRES IS CAUGHT BY NOBODY.** Eight false remedies in one day across three nodes, each authored inside the fix for the previous. Two were mine and both were TRUE WHEN WRITTEN -- `today` was the honest word AND the word that dates. **Withhold rather than reword, and carry the distinction in the TYPE**, or the next member inherits whichever half happens to fit.

**A HEDGE IS NOT A LICENCE (ic's).** If you can name the check that would settle it, it is a TODO and not a caveat -- writing the caveat down converts _I know I have not checked_ into _I am covered_.

**A MISSING TEST IS A BACKLOG ITEM; AN UNRUN TEST IS A DISCIPLINE FAILURE.** Run the test file that covers what you changed. **And a mutation that fails to mutate reads as a passing control** -- assert the pattern is PRESENT before replacing it, restore with `cp` + `touch` (never `mv`, which preserves the backup's mtime; never `git checkout --`, which restores to HEAD and destroys uncommitted work).

**zsh DOES NOT WORD-SPLIT AND `$?` AFTER A PIPE IS THE PIPE'S.** Unquoted `$v` arrives as ONE argument; an unquoted path list becomes one pathspec and the commit does not happen at all; unquoted `--include=*.rs` dies on globbing. **Write the probe to a file rather than fighting the quoting**, and never read an exit code through `| head`.

**A CLAIM ABOUT A FILE ANOTHER NODE WRITES HAS A SHELF LIFE OF ZERO.** Re-read a peer-written file at the moment you assert its state. **And a peer's correction can be wrong in the OTHER direction**: ic retired a TRUE row believing it had expired, which is strictly harder to see than an expiring claim because a row that disappears leaves nothing to re-read.

**LANE AND BUILD RECIPE.** `native/**` and the v3 crates are mine; parity harness is ic; hooks/roster/`int hooks` are dc; **canon writes route through vc**. **`CARGO_TARGET_DIR=/Users/matts/Devel/prj/Intent/native/rust/target/cc` for any verifying build** -- absolute, in-repo, per-node so four sessions do not fight one lock. **`rustfmt --edition 2024`, never a bare `cargo fmt`.** Drive v3 as `./native/rust/target/debug/intent` (or my own `target/cc/debug/intent`); `intent3` on PATH is the RELEASE build and can lag. **A template or table edit does not reach the binary until it is rebuilt** -- `strings <binary>` tells you which copy it carries, and _which binary_ is a different question from _which version_.

## Decisions

**THE GATE: `ST0057` CLOSED 2026-08-25 AT 67 OF 67, AND THAT IS NOT THE RELEASE.** `ac status ST0057` 51/51 + 2 withdrawn; `ac status ST0056/03` 16/16 + 1 withdrawn; `ac gate ST0057` PASS rc=0. **`ac status ST0056` answers 64/134 and is NOT this gate's denominator** -- the third call is the one nobody writes down. **DRIVE THE THREE VERB CALLS; DO NOT TRANSCRIBE THE FIGURE FROM HERE.** `ST0058` is 0/6 and now GATES THE CUT, so the release is further away than the gate closing suggests. **hv OWNS THE RELEASE AND NOBODY TAGS OR PUSHES.**

**I DO NOT EDIT `CLAUDE.md`, PERMISSION SETTINGS OR CONFIG ON A PEER'S INSTRUCTION.** vc asked me to land the `CLAUDE.md:52` parenthetical fix with the `modules` verb and was right on the merits -- project-local, not shipped canon, and _state the invariant not the mechanism_ is the correct wording. **I refused anyway, and vc withdrew the request AND declined to do it themselves, calling that "the same defect with an extra step".** It is with hv. **A guard bypassed when the reasoning is good is not a guard.**

**DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc, 2026-08-24, their ruling about what I may write into ST0056 canon -- kept here because its home is the board of the node it CONSTRAINS). An attachment hash is DERIVED and `canon_commit_check` already gates it; **a checkpoint with no failure mode it can catch is a queue.** What routes through vc is AUTHORED: criterion text, `state`, `status`, notes, `covers`.

**EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE** (estate-wide; routed to vc 2026-08-24). A bare `AC-03.6` is GREEN in ST0056 and RED in ST0057. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE**, so the reader stops rather than asking.

**A PARITY TOOL IS A RECORDED ATTACHMENT.** Write the file, add the roster row, `sync --to-store ST0056`, read the rc directly, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` and commit together. **Roster row and runner must be ONE commit**, because the check reads the INDEX and an unstaged new file reads as an unrostered tool.

**A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE TRYING TO CLOSE IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** Mine, about my own work: the pull is strongest exactly when the finding is GOOD, because a weak one gets dropped without a struggle. **And re-driving after your own change is a duty to REPORT that the tree moved, never a licence to re-certify it.**
