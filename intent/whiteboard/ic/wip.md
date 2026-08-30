---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 14:59Z
status: active
focus: "hv RULED `intent explore [address]` TAKES AN OPTIONAL ARGUMENT, overriding vc's no-address -- design is on the board, FIRST on the bounce. WP-17 IS A WORKING TUI. AC-17.10 and AC-17.11 closed; the list+detail split has geometry, panes and rows; all five thread descents render. ONE RED IS MINE AND WITH vc: explore is in no bucket in the mutator census, and every existing bucket refuses it for a stated reason."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1454Z.md`. This file is the COLD-SESSION MINIMUM: state, not story. The keepers here are CLASSES; the incidents are in the archives.**

## DOING

**RE-MEASURE EVERY FIGURE BELOW AT PICKUP. Five nodes write this tree and a number here is spent the moment one does.** Last figures with a HEAD I read in the same breath, at `4c04bcb5`: `cargo test -p intent-cli --lib` **142 passed / 0 failed**. `intent ac status ST0056` **85/134** at `9d0cf945`. **A full three-crate suite figure is cc's, not mine** -- 218 targets, 1694 tests, and the one red is `explore`, below.

**NOTHING OF MINE IS UNCOMMITTED.** Today: `4cb6600f` (AT-17.3/.7/.9/.11 -- four criteria whose evidence already existed), `354e2136` + `b003ea0b` (AC-17.10 and its lockfile), `e25760af` (the split's geometry), `cbf4e216` (Tab crossing the panes), `4152681a` (all five thread descents), `9d0cf945` (AT-17.10 green + the `help` note names `intent#0086`), plus board commits `8e3a2724` and `a2c59d80`.

**`AC-17.10` AND `AC-17.11` ARE CLOSED. `AT-17.10` went green on vc's ruling** that clause 2 has NO SUBJECT -- `facade.edit` appends `.md` and decides by filename, so no field can enter that path by construction. **It becomes live the day a field realiser appears, and `Files::scratch` is now the one home.**

**THE ONE RED IN THE ESTATE IS MINE AND IT IS WITH vc.** `explore` is in NO bucket in `intentsvcs --test write_moves_only_what_changed`, red since `325ca3a6` and hidden until cc fixed the `dual_path_conformance` hang. **Every existing bucket refuses it for a stated reason** -- nothing proves its writes (`set`/`put` are not in `populations.shipped`, so there is no driver to cite); it is wired; it writes the estate; the measurement does not exist so it is not a measured negative; and `UNPROVEN`'s ratchet is at 22 of 22 with its own message ruling that a new shipped mutator needs a case, a named path or a stated reason. **Minting a fourth bucket is vc's call, recommended and asked.**

## TODO

1. **`intent explore [address]` -- hv's ruling, and it is the FIRST thing on the bounce.** The design, worked out and not yet built:

   - **ONE RESOLVER, THREE SPELLINGS.** `intent://...` through `address::parse`; a `/thread/ST0056` path through `View::parse`; a bare id through `address::promote`, which is ALREADY the estate's one door for ids and refuses ambiguity by naming both forms. **Nothing new resolves anything.**
   - **THE FALLBACK MUST ANNOUNCE ITSELF.** hv said _it just opens at the root_, and the honest form of that is the root PLUS the reason on screen. **A browser that silently opens somewhere other than you asked is the answers-confidently-from-partial-evidence class v3 exists to remove** -- and it is different from `intent edit ST9999`, which must still REFUSE, because `edit` was asked to act on a specific thing and `explore` was asked to open the explorer near one.
   - **THERE ARE THREE OUTCOMES, NOT TWO, AND THIS IS THE FINDING.** The ADDRESS GRAMMAR IS WIDER THAN THE VIEW LADDER: `Entity` carries `Node`, `NodeInbox`, `Event`, `At` and `Attachment`, which are addressable and have NO view. So _named nothing_ and _names something this surface cannot show_ are different facts and must read differently -- the second one is a real address and telling the operator it named nothing would be false.
   - **`Landing` GOES IN `intentsvcs::nav`, NOT IN THE FACE.** The web face needs the same resolution for a URL bar and a jump-to box, and `intentd` cannot reach the CLI. **Radius asked before writing, not after.**
   - **THE ROW:** `args: [{name, type: "address-or-id", arity: "0..1"}]`, and `gen_dispatch_table.sh` will want the census regenerated with its own jq.

2. **The list+detail split's MARKDOWN RENDERER**, the one piece of it not built. §6 wants ONE renderer for both panes; §2's mockup WRAPS a long detail value with its continuation indented to the value column. **That does not break `AC-17.11`** -- the guarantee is column alignment and an aligned continuation keeps it -- but confirm before building, because it is the same criterion-versus-mockup shape vc already ruled once. Styling needs `plan` to emit spans rather than `String`s and every alignment assertion is on `String`s.
3. **`AC-17.8`, and the READ HALF NEEDS NOTHING BUILT.** `Project::edit_disposition` IS the authored/generated split -- `Open` / `OpenRoundTrip` / `Refuse { author_with }`, derived from `classify`, with hv's thread-cover ruling in it. **The REFUSAL HAS NO DOOR:** 301 attachments on ST0056 and not one generated view among them, so _refuses a generated view by naming it_ is unreachable. Three options with vc.
4. **Enter inside the detail pane.** §4 says `⏎` descends row -> detail -> editor, and **WHICH entity a detail row edits is a real question** -- a criterion's text is not the thread's.
5. **`AC-17.6`'s reword** -- vc's. The row stays UNSATISFIED; `AT-17.6` is still to-write.
6. **WP-09 -- start at `AT-09.4`.** `intent llm` already derives from the table, so the WP's first honest act is the AT.
7. **ST0064 (L)**, WP-16 (S), `0142`'s structural half, **ST0065** (two WIP work packages, ZERO acceptance criteria).
8. **Owed:** `rustfmt::skip` is used in ZERO places and that is load-bearing -- `AGENTS.md` or the Rust pack. **`AC-17.10`'s soft-wrap flags** -- the correctness half holds because nothing transforms in either direction; the comfort half means widening the one shared launcher. **EMBED's pty** is its own build; what shipped is full-pane.
9. **Not mine:** `IN-RS-CODE-004`'s proxy is a grep that cannot tell a doc comment from a signature, so it refused the comment EXPLAINING the fix it demanded. vc is filing it. **`AGENTS.md`'s Swift lines are vc's**, downstream of `fb520be3`; I measured it and declined to adopt it.

## Watch-outs -- mechanisms only

**A lesson that now has a guard is not here; the guard is the durable form.**

1. **A CORRECT PRINCIPLE APPLIED AT THE WRONG RADIUS, AND IT IS ONLY VISIBLE FROM OUTSIDE THE CRATE YOU ARE STANDING IN.** Four times now. **Today it was applied BEFORE the fact for the first time:** `form::raw` went into `intentsvcs` because the web textarea needs the same bytes and `intentd` cannot reach the CLI. **Ask which crate the OTHER consumers live in before deciding where a derivation goes.**
2. **A TRUE RESULT FROM AN INSTRUMENT THAT COULD NOT HAVE ANSWERED DIFFERENTLY.** Still the dominant class and it arrives by new routes. **Today's was a PIPE:** `... | sort | uniq -c | head -20` then `grep -c FAILED` returning 0 -- a true count over a corpus my own instrument had truncated, where the `head` was there to shorten output and silently changed what the count was ABOUT. **Ask what your instrument could not have seen, not whether it agrees.**
3. **A MEASUREMENT WHOSE SUBJECT IS NOT THE ONE NAMED.** `Triple::value` runs prose through `one_line`, so handing those bytes to `$EDITOR` destroys every paragraph break -- and **the round trip stays faithful, so no test of the return path can see it.** `AC-17.10` names the RETURN as the dangerous half; **a criterion that names one half by name licenses you to stop looking at the other.**
4. **A REAL ANSWER RESTING ON THE ORDER OF A LIST.** `NORMAL + Enter` was resolved by `.find()` -- table order -- while the table's own test says order is not an answer. **Define the default BY EXCLUSION and assert with the list in both orders**; that is the only control that separates the two.
5. **THE PROPERTY THAT MATTERS OFTEN LIVES BETWEEN TWO SOUND INSTRUMENTS.** And **an AGREEMENT REACHED THE SAME WAY TWICE IS NOT A CHECK** -- two nodes read `replacement` at the level that has no such key and turned one mistake into a consensus. **Ask whether your method could have produced a different answer from theirs.**
6. **A DECLARED THING NOTHING READS IS THE SAME DEFECT AS THE VALUE IT ANNOTATES.** My `help` `spelling_note` was right, dated and unreachable, so two nodes re-diagnosed the row it had already answered. **Give prose a POINTER an instrument can find** (`intent#0086`), or expect it to be re-derived.
7. **REACHABLE AND BLANK IS THE FAILURE THAT READS AS DATA.** Four of five descents rendered empty and `AC-17.7`'s _every level is reachable_ was true of a screen that said nothing. **Distinguish NOT BUILT from EMPTY in the return type**, and hold the built list against the declaration by set equality BOTH ways.
8. **`gen_dispatch_table.sh` REFUSES ON EVERY DERIVED COUNT AND HAS NEVER BEEN WRONG.** Its own sentence is the rule: _do NOT adjust the label to make the number come out._
9. **THE SHARED INDEX MAKES AUTHORSHIP NON-DETERMINISTIC IN BOTH DIRECTIONS, AND ONE MECHANISM NO CARE CLOSES:** a register write goes to the STORE, and the store's disk effect is canon PLUS a generated view -- **two files the writer never names**, so the author cannot stage narrowly and the sweeper cannot exclude them. `git commit --only` protects your own discipline and nothing else. **After any commit, `git show --stat HEAD` and count the files against what you staged.**
10. **NEVER ASK A PEER TO DO WHAT THIS SESSION WAS DENIED**, and never adopt a change you cannot attribute to yourself -- that is the same provenance defect as sweeping one, chosen rather than suffered.
11. **`cargo fmt --all` IS THE UNGUARDED TWIN OF A GUARDED DOOR.** `rustfmt --edition 2024 <my files>` only. **Never `--no-verify`**; when the critic gate is wrong, reword and file it.
12. **SHELL QUOTING EATS CONTENT SILENTLY, AND zsh IS NOT bash.** `$var` does NOT word-split -- it cost a commit today. Backticks in double quotes are command substitution; an apostrophe in single quotes is a hard syntax error. **Use a quoted heredoc for anything carrying prose, and list paths inline rather than through a variable.**
13. **THE WORKING DIRECTORY PERSISTS BETWEEN TOOL CALLS.** Three times today.
14. **A HUNG RUN AND A LONG RUN ARE ONE OBSERVATION** (cc's, adopted). A suite that never returns looks like a busy machine, and the innocent explanation is available before the true one. **A background run with no verdict is not evidence of anything.**

## Decisions

- **(hv) 3.0.1 scope is ST0056 + ST0058/0066/0068. No 3.1.0.**
- **(hv, RULED 2026-08-30, OVERRIDING vc's NO-ADDRESS) `intent explore [address]` TAKES AN OPTIONAL ARGUMENT** -- an `intent://` URI, or an id it tries to match, **and if it cannot match, it opens at the root.** hv's reason: _it does not obviate `intent edit` in any way; it is convenience for the user as they need._ vc's earlier ruling was made while hv was AFK and is superseded at the source. **vc's "three doors onto one surface" objection does not reach it: this is ONE door accepting spellings the estate ALREADY resolves, not a second resolver.**
- **(vc) `AC-17.10` CLAUSE 2 HAS NO SUBJECT** -- not vacuously satisfied. **A population empty BY ACCIDENT when it should have members is the defect; a CATEGORY that does not exist is not.**
- **(vc) DERIVATION IS SHARED, RENDERING IS PER FACE.** `triples`, `raw`, `field`, `View` + `path` + `kinds`/`descents` live in `intentsvcs`. **`Stack` stays in the face: it is STATE.**
- **(vc) `AC-17.11` REWORDED to `tui-design.md` §2.** **A criterion contradicting a ratified design is the criterion being stale.**
- **(ic, AC-05.1) `organize` is EXPOSED on MCP** -- the rule is UNDOABILITY, not blast radius. **(ic) `C-g` cancels the MENU.**
- **(ic) Every path segment is a name the declaration already carries.** No pluralising.
- **(vc) Register writes are ic's.** `dispatch-table.json` is AUTHORED canon; `.md` is GENERATED. **(vc) The `help` `target.spelling` write is WITHDRAWN, not deferred -- do not make it later.**
- **(dc/hv) ST AND WP GET NO STATUS VARIANT** -- `fiat` sits BESIDE a status that stays `completed`/`done`.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only.
