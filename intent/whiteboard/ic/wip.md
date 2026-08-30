---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 14:24Z
status: active
focus: "AC-17.10 IS BUILT: the prose handoff, with the editor as authority on the return AND the model as authority on the departure -- the second half is a defect the criterion does not name. AT-17.10 stays to-write on ONE clause vc must rule. Four more ATs went green because their evidence already existed. NEXT: the list+detail split, then AC-17.8."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1336Z.md`. This file is the COLD-SESSION MINIMUM: state, not story. The keepers here are CLASSES; the incidents are in the archives.**

## DOING

**RE-MEASURE EVERY FIGURE BELOW AT PICKUP. Four nodes write this tree and a number here is spent the moment one does.** Last figures with a HEAD behind them, all at `3a6eda25`: `cargo test -p intent-cli --lib` **124 passed / 0 failed** (101 at `325ca3a6`); `intentsvcs --test the_editor_gets_the_model_bytes` 4 passed; `--test dep_graph_guard` 2 passed. **`ac status` 82/134 satisfied, BLOCKED.** No full `--tests` run has a verdict this session.

**NOTHING OF MINE IS UNCOMMITTED.** Since the fold: `4cb6600f` (AT-17.3/.7/.9/.11 green -- four criteria whose evidence already existed), `354e2136` (`AC-17.10`, the prose handoff), `b003ea0b` (the lockfile `354e2136` should have carried).

**`AC-17.10` IS BUILT AND `AT-17.10` IS CITED, NOT GREEN.** One clause -- _"and the existing realise-then-open path"_ -- reads two ways for a FIELD, because `facade.edit` realises a DOCUMENT and `prose` is declared on `objective`/`context`/`body`/`preamble`, which have no file. **My reading is that clause 2 is satisfied vacuously and design §7 puts artefact editing in its own subsection; the ruling is vc's and it is with them live and on the file.**

**THE TWO DEFECTS BUILDING IT FOUND, both shipped fixed.** (1) **THE CRITERION WARNS ABOUT THE RETURN AND THE DEPARTURE IS BROKEN THE SAME WAY:** `Triple::value` is `one_line`-collapsed, so handing it to `$EDITOR` deletes every paragraph break on the first save -- the round trip is faithful and the bytes were already wrong. `intentsvcs::form::raw` is the shared fix, placed by asking which crate the OTHER consumers live in first. (2) **`NORMAL + Enter` WAS RESOLVED BY TABLE ORDER**, so Enter on a prose row opened an in-place edit of a 673-byte objective inside a one-line value column. `mode::BY_ROW_KIND` + `mode::arm` resolve it by ROW, default arm BY EXCLUSION.

**`0157` HAS NOW HIT FOUR TIMES IN ONE DAY, three of them in this direction.** My `AT-17.10` citation landed in peer commit `121ea719`. **The structural fact, which is new: a register write's disk effect lands in two files the writer never names** (`intent/.canon/st/*.json` and the generated view), so the window between `intent at edit` and your own commit is open BY CONSTRUCTION for every node. `git commit --only` protects the discipline of whoever uses it and nothing else.

## TODO

1. **The list+detail split.** BODY splits where the selected row CARRIES detail -- **triggered by the row, never by a hardcoded list of view kinds**, which is the half that gets forgotten. List and detail share ONE markdown renderer.
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
