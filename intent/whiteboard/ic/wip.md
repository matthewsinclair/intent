---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 14:48Z
status: active
focus: "AC-17.10 IS BUILT and the list+detail split is geometry, panes and rows. All FIVE thread descents render -- four of them were reachable, entered and BLANK. Two rulings sit with vc: AC-17.10 clause 2, and where AC-17.8 meets a generated view at all."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1336Z.md`. This file is the COLD-SESSION MINIMUM: state, not story. The keepers here are CLASSES; the incidents are in the archives.**

## DOING

**RE-MEASURE EVERY FIGURE BELOW AT PICKUP. Four nodes write this tree and a number here is spent the moment one does.** The run STARTED with my paths at `4152681a` and **I cannot show HEAD was still there when it finished** -- a peer landed `4c04bcb5` around it, which is the second time today I have pinned a figure to a commit I did not read in the same breath. `cargo test -p intent-cli --lib` **142 passed / 0 failed**; the FULL `cargo test -p intent-cli` **494 passed / 1 failed**, and **the one failure is cc's, not mine** -- `prefix_resolution.rs` from `d0cf4ad1` drives verbs off the dispatch table without `.env("HOME", testkit::fixture_home())`, which is a test writing to the operator's REAL home. Reported to cc live; their file, not mine to edit.

**NOTHING OF MINE IS UNCOMMITTED.** Today, since the fold: `4cb6600f` (AT-17.3/.7/.9/.11 -- four criteria whose evidence already existed), `354e2136` (`AC-17.10`), `b003ea0b` (the lockfile that should have travelled with it), `8e3a2724` (board + escalation), `e25760af` (the split's geometry), `cbf4e216` (Tab crossing the panes), `4152681a` (all five thread descents).

**TWO RULINGS SIT WITH vc AND BOTH ARE ON THE FILE AS WELL AS LIVE.** (1) `AC-17.10` clause 2 -- _"and the existing realise-then-open path"_ -- reads two ways for a FIELD, so `AT-17.10` is CITED and stays `to-write`. (2) **`AC-17.8`'s REFUSAL HAS NO DOOR:** measured on ST0056, 301 attachments and not one generated view among them, so _refuses a generated view by naming it_ is a behaviour nothing can reach. **Same class as the `Cancel` edge no key could produce.** Three options sent with a recommendation (A: the documents pane lists them read-only, vocabulary from the table and disposition from `Project::edit_disposition`).

**THE READ HALF OF `AC-17.8` NEEDS NOTHING FROM ME.** `Project::edit_disposition(rel)` IS the authored/generated split the criterion demands -- `Open` / `OpenRoundTrip { round_trips }` / `Refuse { author_with }`, derived from `Project::classify` rather than from a filename list, with hv's 2026-08-29 thread-cover ruling already in it.

## TODO

1. **The list+detail split -- GEOMETRY, PANES AND ROWS ARE DONE; the MARKDOWN RENDERER is not.** §6 wants one renderer for both panes and §2's mockup WRAPS a long detail value with its continuation indented to the value column. **That does not break `AC-17.11`** -- the guarantee is column alignment and an aligned continuation keeps it -- but it is the same criterion-versus-mockup shape vc ruled on for `AC-17.11`, so confirm before building. Styling (bold, code, em) needs `plan` to emit spans rather than `String`s, and every alignment assertion is on `String`s.
2. **`AC-17.8`** -- reach every authored artefact and refuse a generated view BY NAMING IT. The authored/generated split is READ from the model, never hard-coded.
3. **`AC-17.6`'s reword** -- vc is performing it; sentence sent. **The row stays UNSATISFIED**: the verbs shipping is not the criterion, and `AT-17.6` is still to-write.
4. **WP-09 -- start at `AT-09.4`.** `intent llm` already derives from the table, so the WP's first honest act is the AT, not new code. **No longer gated on `organize`.**
5. **ST0064 (L)**, WP-16 (S), `0142`'s structural half, **ST0065** (WIP, two WIP work packages, ZERO acceptance criteria).
6. **Owed and unclaimed:** `rustfmt::skip` is used in ZERO places and that is load-bearing -- belongs in `AGENTS.md` or the Rust pack. **`AC-17.10`'s soft-wrap flags** (design §7) -- the CORRECTNESS half holds because nothing transforms in either direction; the comfort half means widening the ONE shared launcher for one caller, which is a decision about `render.rs`. **EMBED's pty** is its own build; what shipped is full-pane.
7. **Not mine, but somebody's:** `IN-RS-CODE-004`'s mechanical proxy is a grep that cannot tell a doc comment from a signature, so it refused the comment EXPLAINING the fix it had just demanded. Same class as the whiteboard header guard's rule that scanning prose makes reporting the defect an offence.

## Watch-outs -- mechanisms only

**A lesson that now has a guard is not here; the guard is the durable form.**

1. **THE TRANSIT CLASS.** _Failures cluster where the READ and the WRITE are two separate acts, and vanish where they are one call._ **Now measured on git itself:** a private-index discipline holds only while HEAD does not move between `read-tree` and `commit`, and a gate refusal prises those apart. **`git add` + `git commit --only` REMOVES the window** -- `--only` builds the tree from HEAD plus the named paths, never from the index as a whole. Don't make the window small; remove the second act.
2. **A CORRECT PRINCIPLE APPLIED AT THE WRONG RADIUS, AND IT IS ONLY VISIBLE FROM OUTSIDE THE CRATE YOU ARE STANDING IN.** Twice today. I argued against a hand-written value map because it is a second home, then put the derivation in `intent-cli`, where `intentd` cannot reach it -- guaranteeing the second copy. Same again with nav. **Ask which crate the OTHER consumers live in before deciding where a derivation goes.**
3. **A TRUE RESULT FROM AN INSTRUMENT THAT COULD NOT HAVE ANSWERED DIFFERENTLY.** Still the dominant class. **Two more today:** `pad = 0` destroyed alignment and all seven tests stayed green, because the loop's `continue` skipped every row; and my keymap sweep held seventeen characters and no `g`, so `C-g` read as unreachable after I had bound it. **A sweep is only as good as its ALPHABET. Assert the loop EXAMINED something** -- a count of rows looked at is the only thing separating "all correct" from "none inspected".
4. **RIGHT ANSWER, WRONG INSTRUMENT -- FOUND ONLY BY ASKING WHY A GREEN PASSED.** `descents(l, "wp")` was correctly empty while the lookup ignored `#/$defs/WorkPackage` and read THREAD's properties. **Assert on the SUBJECT, never on the CONSEQUENCE.**
5. **THE PROPERTY THAT MATTERS OFTEN LIVES BETWEEN TWO SOUND INSTRUMENTS** (`AC-00.16`). Found deliberately twice: the machine proved Esc walks toward rest and the keymap proved the key arrives, and NEITHER is _holding Esc gets you out_; `focus.rs` proved the tab walk total and reversible and NOTHING proved the arrows drive it. **Ask what neither instrument can see.**
6. **DERIVE THE SECOND VALUE OUT OF EXISTENCE RATHER THAN TESTING THAT TWO AGREE.** Where prose must carry a claim, give it a WITNESS and take the number out. **A declaration key is a promise that code reads it** -- the dispatch table refuses an unclassified key, and my invented `no_address_basis` was exactly that.
7. **`gen_dispatch_table.sh` REFUSES ON EVERY DERIVED COUNT AND HAS NEVER BEEN WRONG. Five refusals in one edit today, every one real.** `populations` is corpus order and must be regenerated with the generator's OWN jq -- a re-implementation gets the order wrong silently. **Its own sentence is the rule: `do NOT adjust the label to make the number come out`.**
8. **A DECLARED EXEMPTION MUST CHECK ITS OWN PREMISE.** `ESC_NOT_OURS` is read rather than retyped, and the exempt mode must ACTUALLY trap -- an exemption that never verifies what it forgives goes on forgiving something that is no longer there.
9. **THE SHARED WORKTREE AND THE SHARED INDEX.** The dispatch table is COMPILED IN, so a malformed one is a binary nobody can start. **`git commit --only` protects your own discipline and NOTHING protects you from a peer's broad commit** -- three instances today, in both directions. Never `--no-verify`; never remove a peer's `index.lock`. **After any commit, `git show --stat HEAD` and count the files against what you staged** -- the only diagnostic that exists.
10. **A RECEIVED ARTEFACT IS EVIDENCE, NOT DATA** (vc, ruled) -- reproduced, never corrected; the correction lives BESIDE it. **Generated-vs-authored governs who may COMMIT; received-vs-originated governs who may EDIT.**
11. **NEVER ASK A PEER TO DO WHAT THIS SESSION WAS DENIED.** A peer's approval is not hv lifting a boundary. **It worked: the block was lifted at the source and store writes now run.**
12. **`cargo fmt --all` IS THE UNGUARDED TWIN OF A GUARDED DOOR.** `rustfmt --edition 2024 <my files>` only. **A peer's untracked file reading as unformatted is NOT a finding** -- untracked cannot reach CI.
13. **SHELL QUOTING EATS CONTENT SILENTLY, AND zsh IS NOT bash.** `$var` does NOT word-split; backticks inside double quotes are command substitution; an apostrophe inside single quotes is a hard syntax error; `$?` after a pipe is the last stage's (`$pipestatus`). **Use a quoted heredoc for anything carrying prose.**
14. **THE WORKING DIRECTORY PERSISTS BETWEEN TOOL CALLS.** Cost me two calls today.

## Decisions

- **(hv) 3.0.1 scope is ST0056 + ST0058/0066/0068. No 3.1.0.**
- **(hv) `intent explore` -- the TUI at the top level. (vc) NO ADDRESS: deep linking is `edit`'s job.**
- **(vc) `AC-17.11` REWORDED to `tui-design.md` §2** -- five sections, two rules. **A criterion contradicting a ratified design is the criterion being stale.**
- **(vc) DERIVATION IS SHARED, RENDERING IS PER FACE.** Entity+declaration -> triples, and `View` + path + `kinds`/`descents`, live in `intentsvcs`. **`Stack` stays in the face: it is STATE, and the web's equivalent is browser history.**
- **(vc) `AC-17.12` minted** -- the TUI's stack and the web's URL are the same sequence from the same declarations.
- **(ic, AC-05.1) `organize` is EXPOSED on MCP** -- the rule is UNDOABILITY, not blast radius. **(ic) `C-g` cancels the MENU** -- the design names `Cancel` and resolves the default keymap to emacs.
- **(ic) Every path segment is a name the declaration already carries.** No pluralising; §9's prose needs hv's hand.
- **(vc) Register writes are ic's.** `dispatch-table.json` is AUTHORED canon; `.md` is GENERATED.
- **(dc/hv) ST AND WP GET NO STATUS VARIANT** -- `fiat` sits BESIDE a status that stays `completed`/`done`.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only.
