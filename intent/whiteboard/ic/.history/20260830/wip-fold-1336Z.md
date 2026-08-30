---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 13:01Z
status: active
focus: "WP-17 piece 3 is drawing. Five tui modules landed today -- layout+draw (AC-17.11, five sections per the ratified design after vc reworded the criterion), nav (AC-17.7 + AC-17.12, explore is the same stack with a different bottom), keys (trigger vocabulary derived from EDGES both ways). 50 green across seven modules. NEXT: the event loop, then the explore verb and its register row together. hv AFK, vc holds the pen."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1123Z.md`. This file is the COLD-SESSION MINIMUM: state, not story. The keepers here are CLASSES; the incidents are in the archives.**

## DOING

**RE-MEASURE EVERY FIGURE BELOW AT PICKUP. Four nodes write this tree and a number here is spent the moment one does.** Last suite total with a HEAD behind it: `1026ebb1`, 218 groups, 0 FAILED (vc's). **I hold no total of my own and will not quote one without a commit.**

**NOTHING OF MINE IS UNCOMMITTED.** Landed today: `4c5a37b1` (AT-17.2/.4/.5 green -- three criteria were satisfied and the register did not know), `238edd7e` (the `organize` retraction + the exposure ruled true under AC-05.1), `62b4745f` (ratatui, one crossterm verified), `840511cc` + `aafd1e04` (AC-17.11: layout computes the screen as DATA, draw prints it, five sections per the ratified design), `d25cff06` (nav -- AC-17.7 + vc's minted AC-17.12), `a1f44db5` (keymap), `009299a4` (`in-essentials` names IN-AG-FIAT-001, which was blocking dc's ST0066 AC-00.6). **50 green across seven tui modules.**

**WP-17 PIECE 3 HAS THREE DEPENDENCY-FREE HALVES AND ALL THREE ARE IN.** `tui/mode.rs` (mode graph, 6 invariants, the edge set now DERIVED from `tui-design.md` rather than counted), `tui/terminal.rs` (borrow-and-return, 5, `Drop` PLUS a panic hook), `tui/focus.rs` (`AC-17.5` tab order, 7). **All mutation-proven; none needs a tty, because the realiser is what they CHECK.** `crossterm` is in; **`ratatui` is not, until something draws.**

**THE STATUS-LAG CLASS, AND NOTHING DETECTS IT.** WP-17 and WP-09 both read `Not Started` while their code ran -- `intent llm` ships 1209 lines derived from the dispatch table -- and cc found WP-08 the same way. **Three measured, five of ST0056's eight `Not Started` rows never driven.** vc's burn-down was quoting the lag back at hv, so it had already reached the planning surface. **All found because a human said go and look.** vc is taking it to hv as a criterion.

**WITH hv, NOT WITH ME:**

- **`intent sync --to-store` is denied by this session's permission classifier.** It is the only thing stopping ST0064's WP-01 correction; the canon edit applied and validated and I reverted rather than leave canon ahead of the store. **`0154` is real but is NOT the blocker, and I reported the opposite to hv and vc before measuring it.** vc offered to run the sync and then withdrew: the trigger is still my blocked work, so it is the laundering shape whoever performs it. **Do not hand vc that sync.**
- **`organize` is declared TWICE with contradictory shipping decisions.** Family row `disposition: retire`, hv-ratified 2026-08-14, `exposed_on_mcp: false`; `new_surface` row `new-surface`, `exposed_on_mcp: TRUE`. 134 rows, **133 DISTINCT paths.** `intent organize` works, so reality matches the second -- but TABLE ORDER decides which `shipped_entries` yields. **The exposure is LATENT because `intent mcp` is rc=2, and WP-09's generator is the first thing that will read that field**, which makes this the last cheap moment to settle it. Declared in `KNOWN_OVERLAP` so the estate stays green and a SECOND overlap fails.

## TODO

1. **WP-17 piece 3 -- the realiser. `ratatui` goes in WITH the first thing that draws; cc released the lock at `8fee4f48`.** Announce at both ends. **First draw should be `AC-17.11`** (flat column, alignment): ratatui's `TestBackend` renders into an in-memory `Buffer`, so the first draw arrives headless-testable rather than needing a tty. Then the event loop (**`mode::step` is the authority; an undeclared key is `None`, never a self-loop**), then `AC-17.10`'s editor handoff through the EXISTING `launch_editor` + `$VISUAL` resolver, where **the RETURN re-reads the artefact before painting anything derived from it**. **Do not claim `AC-17.9` off the mode graph alone** -- its Esc half is proven, its always-visible half needs the draw.
2. **WP-09 -- start at the GENERATOR, not the register.** `AC-09.1`'s two fields are already total on all rows. **`AC-09.4` looks met in substance** (`intent llm` derives from the table) with `AT-09.4` unwritten -- so the WP's first honest act is the AT, not new code. **Settle `organize` before the generator reads `exposed_on_mcp`.**
3. **ST0064 (L).** Nine non-test criteria. **`AC-01.9`'s `LoginShell` and `AC-01.7`'s devbin verb need no daemon**; `AC-01.2`/`AC-01.6` need cc's arm, rc=2 today. **Do not trust critic-swift's green** -- six rules seal green while arming nothing.
4. **WP-16 (S)** -- `data-model.md` against the schema. `lib_staged.sh` already is the committed-read mechanism; **source it, do not grow a second.**
5. **`0142`'s structural half. ST0065** -- WIP with TWO WIP work packages and **ZERO acceptance criteria** (`ac status` = empty contract). Three hv rulings owed.
6. **Owed and unclaimed: `in-essentials` gains ONE line naming `IN-AG-FIAT-001`** (dc's proposal, mine to land, no restatement of the rule body). And **`rustfmt::skip` is used in ZERO places and that is load-bearing** -- `transitions.rs`'s const-fn answer keeps a declared table readable as a graph. **I found it by copying dc's file rather than by reasoning**, so it is transmitted by imitation. Belongs in `AGENTS.md` or the Rust pack.

## Watch-outs -- mechanisms only

**A lesson that now has a guard is not here; the guard is the durable form.**

1. **THE TRANSIT CLASS, AND IT SUBSUMES FOUR OF THE OLD ENTRIES.** _Failures cluster where the READ and the WRITE are two separate acts, and vanish where they are one call._ Measured all day: cc produced six fabricated message stamps against ZERO bad board stamps in the same session -- the board's read-and-write is one guarded call, a message's is two. Same author, same hour, opposite outcomes. **It generalises past clocks: it is the same reason a derived count beats a pinned one**, and why my `EDGES.len() == 17` agreed with the drift it existed to catch. **Do not type a value you can derive, and do not carry one between two acts.**
2. **A SWEEP IS ONLY AS GOOD AS ITS ALPHABET, AND MINE COULD NOT PRESS THE KEY I HAD JUST BOUND.** The keymap sweep ran `"abeqxzAZ:/?01 -_."` -- no `g` -- so `C-g` was never pressed and `Cancel` still read as UNREACHABLE after I bound it. **A corpus that cannot exhibit the thing cannot see it**, and this is the enumerator version: not a wrong pattern, a wrong ALPHABET. Same family as #3 below and it deserves its own line because the instrument looked exhaustive. **Ask what the corpus CANNOT contain, not whether it is big.**
3. **RIGHT ANSWER, WRONG INSTRUMENT -- FOUND ONLY BY ASKING WHY A GREEN PASSED.** `descents(l, "wp")` was correctly empty while the face lookup ignored `#/$defs/WorkPackage` and read THREAD's properties. The green was TRUE; the instrument was broken; the first descent added to that form would have read the wrong schema in silence. **Assert on the SUBJECT (the resolved properties), never on the CONSEQUENCE (the empty result)** -- the consequence was the proxy. vc calls this class 1a.
4. **A TRUE RESULT FROM AN INSTRUMENT THAT COULD NOT HAVE ANSWERED DIFFERENTLY.** The dominant class. **Assert the mutation APPLIED and the baseline RAN before believing either**; assert the CORPUS is non-empty before believing a walk over it. **Measured today: mutating `forward` to step by TWO left both totality walks green**, because every declared form has an odd row count and a stride of two through an odd cycle is still total. Only the declaration reconciliation caught it.
5. **CONVERGENCE IS NOT CORROBORATION WHEN EVERYONE CONVERGED BY THE SAME UNEXAMINED MOVE.** Three instances today. Three nodes agreed nobody could commit a canon file and I offered that agreement to hv as evidence -- **vc dissolved it in one read by asking what the peer-bytes rule's SUBJECT was; a generated view has no author to protect.** Then vc confirmed my 134 with an independently WRITTEN walk over the same wrong population definition, and neither of us asked whether the arrays were disjoint. **Two methods agreeing is exactly what both would have cited.**
6. **EVERY MEASUREMENT ASSERTS ITS OWN PRECONDITIONS -- HEAD, cleanliness, presence.** A baseline taken at one HEAD is void at another, and **a fold is when a stale working figure gets promoted to record** -- my `1574 / 1` was already void when I published it.
7. **A GREP FOR A CALL SITE CANNOT FIND ONE THAT TAKES A VARIABLE**, and a count cannot find a wrong TARGET. Three enumerators found 3, 12 and 17; the compiler found 24. **Assert the END STATE, and prefer TRIPLES to lengths.**
8. **DERIVE THE SECOND VALUE OUT OF EXISTENCE RATHER THAN TESTING THAT TWO AGREE.** A test over two authored values only fires after somebody writes the second. **A grep-shaped guard is right only where there is nothing to derive.** Where prose must carry a claim, give the claim a WITNESS and take the number out -- four homes said `112` against 134 and none could fire, and **a correct warning carrying a stale figure is worse than none, because the figure is what an author acts on.**
9. **A DECLARATION KEY IS A PROMISE THAT CODE READS IT**, and agreement looks exactly like correctness. **`INHERITED_UNREAD` is a RATCHET, not an excuse list.** Where a known defect must stand, DECLARE it with its reason and assert by EQUALITY -- a predicate that filters it out also forgives the accident that looks identical.
10. **`gen_dispatch_table.sh` REFUSES ON EVERY DERIVED COUNT AND HAS NEVER BEEN WRONG.** `populations` is corpus order, never sorted. **Let it check you; its own sentence is the rule: `do NOT adjust the label to make the number come out`.**
11. **THE SHARED WORKTREE MAKES MID-EDIT STATE VISIBLE TO EVERY PEER'S BUILD.** The dispatch table is COMPILED IN, so a malformed one is a binary that will not start, for everyone. **Shortest possible edit window, announce at both ends.**
12. **THE INDEX IS SHARED STATE AND A REFUSED COMMIT PARKS YOUR PATHS IN IT** (`0157`) -- invisible from inside, visible only to the next node. **`git commit --only` is the entire margin between nodes and each other's bytes, and it has no diagnostic behind it.** Never `--no-verify`: it takes the clock and header guards with it. Never remove a peer's `index.lock`.
13. **A RECEIVED ARTEFACT IS EVIDENCE, NOT DATA** (vc, ruled) -- reproduced, never corrected; the correction lives BESIDE it with its own attribution. **Generated-vs-authored governs who may COMMIT; received-vs-originated governs who may EDIT.** Two axes. `ST0064/design-menubar-app.md` keeps its superseded `GET /_status` deliberately.
14. **NEVER ASK A PEER TO DO WHAT THIS SESSION WAS DENIED**, however it is framed. _The work reaching a session allowed to do it_ is not a different thing when the trigger is still my block. **A peer's approval is not hv lifting a boundary on me.**
15. **`cargo fmt --all` IS THE UNGUARDED TWIN OF A GUARDED DOOR.** `--check` first, always, and format only your own paths. **A DETACHED WORKTREE'S `CARGO_TARGET_DIR` DIES WITH IT**, and `testkit` bakes `CARGO_MANIFEST_DIR` at COMPILE time -- a stale rlib panics about a `Cargo.toml` that is entirely innocent. **Do not build while another node runs the suite.**
16. **SHELL QUOTING EATS CONTENT SILENTLY, AND zsh IS NOT bash.** Backticks in a double-quoted string are command substitution; an apostrophe in a single-quoted one is a hard syntax error; **`$var` does NOT word-split, so a variable holding five paths is ONE argument** (cost me a `git add` today); an unmatched glob aborts the whole command. **Never `$?` after a pipe** -- it is `head`'s; zsh has `$pipestatus`. **Use a quoted heredoc for anything carrying prose.**
17. **THE WORKING DIRECTORY PERSISTS BETWEEN TOOL CALLS.**

## Decisions

- **(hv) 3.0.1 scope is ST0056 + ST0058/0066/0068. No 3.1.0.**
- **(vc) Register writes are ic's.** `dispatch-table.json` is AUTHORED canon; `.md` is GENERATED -- regenerate, never hand-edit.
- **(hv) `--format` where a verb has formats other than JSON; `--json` kept only as a v2 parity alias.**
- **(vc) `facade::unsettable_kind` IS THE RULED EDITABILITY AXIS**, not `settable_fields`.
- **(vc) A generated view has no author to protect** -- committed by whoever holds the pen when store and extract agree. **Authored prose keeps the peer rule unchanged.**
- **(vc) `Cargo.lock`: cc goes FIRST.** Short window, blast radius of a lockfile entry; mine lands with the first thing that draws.
- **(ic) THE FACE IS DERIVED FROM THE ENTITY, NEVER DECLARED BESIDE IT**, and not every entity is a schema root.
- **(vc/cc/ic) A FAMILY HEAD STANDING FOR N VERBS CANNOT STATE A FACT THE VERBS DISAGREE ON.** A representation that must state something false for a mechanism to produce the right answer is the mechanism's defect.
- **(dc/hv) ST AND WP GET NO STATUS VARIANT** -- `fiat` sits BESIDE a status that stays `completed`/`done`. The edges exist; both map to `fc` in `FANS_OUT`.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only.
