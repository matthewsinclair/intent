---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 13:36Z
status: active
focus: "WP-17 piece 3 IS DRAWING AND `intent explore` SHIPS. Ten tui modules; the shared derivation and the shared path contract both moved down into intentsvcs on vc's rulings; AC-17.9's always-visible half is proven. NEXT: AC-17.10's editor handoff, the list+detail split, AC-17.8. vc holds the pen."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1336Z.md`. This file is the COLD-SESSION MINIMUM: state, not story. The keepers here are CLASSES; the incidents are in the archives.**

## DOING

**RE-MEASURE EVERY FIGURE BELOW AT PICKUP. Four nodes write this tree and a number here is spent the moment one does.** Last figure with a HEAD behind it: `cargo test -p intent-cli --lib` 101 passed / 0 failed at `325ca3a6`. **A full `--tests` run was still COMPILING when I folded** (background id `bm10l3uul`) -- **it has no verdict, so do not quote one.**

**NOTHING OF MINE IS UNCOMMITTED.** Today, in order: `4c5a37b1` (AT-17.2/.4/.5 green -- three criteria satisfied and unrecorded), `238edd7e` (the `organize` retraction + exposure ruled TRUE under AC-05.1), `009299a4` (`in-essentials` names `IN-AG-FIAT-001`, unblocking dc's ST0066 AC-00.6), `62b4745f` (ratatui, one crossterm verified), `840511cc` + `aafd1e04` (AC-17.11), `d25cff06` + `9949a040` (nav, then its contract moved down), `a1f44db5` (keymap), `64fbb72f` + `5109a3b8` (event loop, cursor), `57c14860` + `cc2d8d2e` (row derivation, then moved down), `dbe5a5c2` (AC-17.9's other half), `325ca3a6` (`intent explore`).

**`intent explore` SHIPS.** Bare verb, no address, rooted at the entity kinds; `intent edit <kind> <id>` roots at the item. **Same stack, different bottom** -- that is the whole of it. Refuses on a pipe with rc=1 and a remedy rather than hanging.

**MY `render.rs` HANDLER LANDED UNDER SOMEBODY ELSE'S COMMIT MESSAGE.** `c75a5b1a` (ST0058, `agents init and validate`) carries `fn explore`. The code is correct and present; only its provenance is wrong, and history on a shared main is not mine to rewrite. **Third instance of `0157`'s class today** -- cc's commit reverted my board at 11:34Z, cc repaired it, and now my bytes went the other way. **The index is shared and has no diagnostic.**

**WITH hv, NOT WITH ME: nothing open.** Both of this morning's items came back ruled -- the `sync --to-store` permission was lifted at the source (store writes work: `at edit`, `at green` and `sync --to-disk` all ran), and `organize` was handed back to me under AC-05.1 and ruled TRUE.

## TODO

1. **`AC-17.10` -- the editor handoff.** `launch_editor` by SYMBOL, no second resolver. **The RETURN is the dangerous half:** re-read the artefact and treat the editor as its authority BEFORE painting anything derived from it. `terminal.rs` holds the drop-guard-plus-panic-hook clause and `run.rs` installs the hook before the borrow. Design §7 carries the measured first-paint times and the soft-wrap flags; **never hard-wrap the model** -- that was reversible for only 439 of 444 real criteria.
2. **The list+detail split.** BODY splits where the selected row CARRIES detail -- **triggered by the row, never by a hardcoded list of view kinds**, which is the half that gets forgotten. List and detail share ONE markdown renderer.
3. **`AC-17.8`** -- reach every authored artefact and refuse a generated view BY NAMING IT. The authored/generated split is READ from the model, never hard-coded.
4. **`AC-17.6`'s reword** -- vc is performing it; sentence sent. **The row stays UNSATISFIED**: the verbs shipping is not the criterion, and `AT-17.6` is still to-write.
5. **WP-09 -- start at `AT-09.4`.** `intent llm` already derives from the table, so the WP's first honest act is the AT, not new code. **No longer gated on `organize`.**
6. **ST0064 (L)**, WP-16 (S), `0142`'s structural half, **ST0065** (WIP, two WIP work packages, ZERO acceptance criteria).
7. **Owed and unclaimed:** `rustfmt::skip` is used in ZERO places and that is load-bearing -- belongs in `AGENTS.md` or the Rust pack.

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
