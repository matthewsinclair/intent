---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 18:44Z
status: active
focus: "FIRST ON THE BOUNCE IS hv's TUI DIVERGENCE -- hv has looked at what I built and it is not what was agreed, so WP-17 needs a scoping conversation before any more of it is built. WP-09 is the live build: AC-09.6 (the facade drive, ~94 arms by hand) is the critical path and every other 09 row sits behind it."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1844Z.md` (five folds before it, same day). This file is the COLD-SESSION MINIMUM: state, not story. The keepers here are CLASSES; the incidents are in the archives.**

## DOING

**RE-MEASURE EVERY FIGURE BELOW AT PICKUP. Five nodes write this tree and a number here is spent the moment one does.** Read in the same breath as HEAD `caa61e9c`: `intent ac status ST0056` **90/135, 25 descoped, 2 withdrawn**. **`cargo test -p intent-cli -p intentsvcs` DOES NOT BUILD `intentd`** -- run `cargo build -p intentd` FIRST or two daemon round-trip tests fail against a stale binary. That is a FALSE RED and it caught me three times in one afternoon.

**NOTHING OF MINE IS UNCOMMITTED.** Every commit today `--only` scoped and file-counted against what I staged.

**FIRST ON THE BOUNCE, AND IT IS NOT A ROW: hv HAS RULED THE TUI DIVERGENT FROM WHAT WAS AGREED** (via vc, 2026-08-30). hv's words: _I just looked at what IC has done and it's quite different to what we agreed, so there's work for IC to do to get towards the desired design._ **Read as SCOPE, not a defect report.** It SUPERSEDES the `tui-design` section-9 plural-path question as the live item -- **do not spend on section 9 until hv scopes this directly.** WP-17 sitting at its 10/12 ceiling is convenient rather than awkward: both remaining rows are blocked on builds that are not mine, so there is nothing to redo before hv says what the design should be.

**RED ON MAIN AND NOT MINE, REPORTED TO dc:** `hook_compat::the_two_binaries_agree_byte_for_byte_on_every_shipped_hook`. `intent/plugins/claude/bin/intent_claude_hook` is GONE and the v2 door finds nothing; the directory mtime puts it in `AT-12.1`'s window. **The prune's own gate went green while this went red**, which is a population that could not see what the change reached.

## TODO

1. **`AC-09.6` -- THE FACADE DRIVE, and it is the critical path.** `AC-09.2`, `.3` and `.5` all need the tier, the tier needs the declared `facade` field, the field needs this drive. **vc minted it as AGREEMENT, not coverage: EXPOSED IMPLIES SERVABLE**, so it closes by NARROWING `exposed_on_mcp` as well as by building methods. **The output is a DECISION for hv, not a backlog of 54 functions.** Read the arms, report the fraction with one door. **`fc` is the first counterexample and is filed as `0171`** -- four facade methods (`ac_fc`/`at_fc`/`st_fc`/`wp_fc`) selected by target shape, which is model reasoning in a coordinator.
2. **`AT-09.4`'s green is vc's call.** Clause 1 has its two arms (`7ecb1e62`); clause 2 has its class guard (`2840924e`) and its one subject guarded (`caa61e9c`). vc has ruled the row does NOT close on one guarded instance.
3. **`claude subagents` IS IN THE CUT** (hv, via vc): _we need functionality parity with v2 and that means plugins and claude subagents._ A subagents family reaching v2 parity, `plugin` checked in the same pass. **Surface change heading at me; lane not settled** -- vc has asked cc whether it is theirs or dc's.
4. **`ST0065` IS hv's DIRECTLY NOW.** The proposal reached hv's inbox 16:55Z (`051cb77e`) and the routing debt is discharged. Not mine to route again.
5. **Blocked on other people's builds, not on me:** `AC-17.1` (needs the web realiser), `AC-17.6` (`intent browse` does not ship -- zero dispatch arms, bucketed in `DECLARED_BUT_UNWIRED`).
6. **Owed, small:** the `no_op` field is declared with a note and used by 30 rows -- I nearly reported it as unread by reading the wrong level. **`AC-17.10`'s soft-wrap flags. EMBED's pty** is its own build; what shipped is full-pane. **`ST0064`** parked (out of the cut), **WP-16**, `0142`'s structural half.

## Watch-outs -- mechanisms only

**A lesson that now has a guard is not here; the guard is the durable form.**

1. **A CORRECT PRINCIPLE APPLIED AT THE WRONG RADIUS, VISIBLE ONLY FROM OUTSIDE THE CRATE YOU ARE STANDING IN.** **vc's discriminator, which needs no re-arguing: MODEL things go down, FACE things stay in the face.** `nav` and `form` moved because they describe what an entity IS and every face shares that; a tool list describes what THIS surface exposes. **Name the falsifier when you decide** -- the day `intentsvcs` or `intentd` needs the tool list, `dispatch.rs` moves with its tests.
2. **A TRUE RESULT FROM AN INSTRUMENT THAT COULD NOT HAVE ANSWERED DIFFERENTLY.** Still the dominant class, and today it was mostly MINE. **Positive-control the instrument on an answer you already know, every run.** I grepped `facade\.` and got 15 call sites for a 135-row surface; `st_list` is spelled three ways and the real reach is 74 of 84. **A blind SEARCH wastes a measurement; a blind VERIFICATION manufactures confidence** (dc's, and it is the worse half).
3. **AN INSTRUMENT THAT FAILS THREE WAYS IN A ROW IS TELLING YOU THE APPROACH IS WRONG, NOT THE CODE.** Three resolver iterations over `render.rs`, three structural causes, **and every intermediate version produced a PLAUSIBLE split.** The controls refused each time; without them the second would have decided a canon change across 94 rows. **Root cause worth keeping: naive brace matching cannot find an arm body, because braces inside `format!` literals are indistinguishable from block braces without a lexer.** `common::string_literals` is the lexer-aware one and it already exists.
4. **AGREEMENT IS NOT DERIVATION, AND THE DIFFERENCE IS INVISIBLE ON EVERY TREE WHERE THE TWO COINCIDE.** Every `guide.rs` test drove the COMMITTED table, so all passed equally against a guide that derives and one that keeps its own list and matches. **Mutate the SOURCE of the answer, not the answer.**
5. **A CHECK WHOSE TWO SIDES ARE THE SAME OBJECT.** The guide's exit-code guard drove exactly the two causes the guide named, so its population was the guide's own sentence. **The cure was deleting the enumeration, not fixing the population** -- with no list there is nothing to take a population from.
6. **GUARDING ONE INSTANCE DOES NOT CLOSE A CLASS CLAIM** (vc's). It is a bar satisfied by one member of a growing set, and **it gets weaker every time the work succeeds**, because the next member is the one nothing catches. The population being one TODAY does not make the claim continuously true.
7. **AN EXCEPTION WHOSE DISCHARGE CONDITION CANNOT BE MET IS WORSE THAN NO CONDITION** (cc's). It reads like the kind that cannot rot and behaves like the kind that does. **Ask whether the condition is REACHABLE before writing it down.**
8. **A STRUCTURED FIELD THAT MERELY LOOKS LIKE THE HOME FOR A CLAIM IS WORSE THAN NO FIELD.** I put `daemon start`/`stop` into `populations.self_loop` on two peers' request, having DRIVEN the behaviour -- and `self_loop` is a population of the STATE MACHINE. **Three nodes read the name rather than the membership rule**, and it went red on main. The test's own words: _the row was added on a belief about the SURFACE rather than about the MODEL._
9. **CHECK WHICH LEVEL A FIELD LIVES AT BEFORE CONCLUDING NOTHING READS IT.** I measured `no_op` across every row, found ZERO users, and was about to report a declared-and-unread field. It is nested under `target` and all 30 self-loop members carry it. **Third instance today of an instrument reading the level that has no such key, and the first with my name on it.**
10. **`gen_dispatch_table.sh` REFUSES ON EVERY DERIVED COUNT AND HAS NEVER BEEN WRONG.** Twice more today: `self_loop` membership requires `target.no_op`; and `*emphasis*` would be rewritten by the repo formatter, so the rendered view was not its fixed point. **Neither would have failed a test.** _Do NOT adjust the label to make the number come out._
11. **`git commit` COMMITS THE INDEX AS IT STANDS -- AND SO DOES `git commit --amend`.** `add` + `commit --only <paths>` is the safe write **and it is not the only one: `git apply --cached` is HUNK-scoped** (cc, demonstrated). **My rule named one safe path and I read it as _the_ safe path**, which is the same hole shape as the `--amend` one. **Verify the staged diff BY CONTENT** -- grep a marker unique to the other author's hunks and assert zero -- and re-verify inside the committing command, not two minutes before it.
12. **A SHARED-TREE MEASUREMENT HAS A SHELF LIFE SHORTER THAN THE WORK IT AUTHORISES** (cc's, sharpened). **Check at the moment of the WRITE, not at the moment of the plan.** `guide.rs` was clean when I planned and dirty when I reached it.
13. **A SHARED FILE'S DIFF CARRIES NO AUTHORSHIP SIGNAL, AND IT FEELS LIKE ONE BECAUSE THE HUNKS ARE ADJACENT.** dc and I ran the same inference on one `render.rs` diff and got two DIFFERENT wrong answers. **Ask, before reverting and before announcing.**
14. **THE DAEMON IS MACHINE-SCOPED, NOT PROJECT-SCOPED** (`~/.local/share/intent/intentd.sock`). Driving `daemon start`/`stop` in a throwaway project does NOT isolate it. Run `stop` first so you know you did not kill a peer's, and verify with `status` afterwards.
15. **NEVER REWRITE THE SHARED DELIVERED ARTEFACT MID-MEASUREMENT.** `~/.local/bin/intent` symlinks straight into `target/release`, so `bin/devbin build all` silently changes what every node's instrument reports -- worse than the `include_str!` blast, which at least failed loudly. Announce-and-hold through dc.
16. **A WHOLE-TREE WRITE REACHED FOR TO ANSWER A QUESTION ABOUT YOUR OWN FILES.** `cargo fmt --all` and `-p <pkg>` are the unguarded twins of a guarded door -- `rustfmt --edition 2024 <the files I edited>` only. **Never `--no-verify`**; when the critic gate is wrong, reword and file it.
17. **A HUNG RUN AND A LONG RUN ARE ONE OBSERVATION** (cc's). Every `cargo test` under `timeout`, and `ps` before saying the word _lock_. **A model that explains an observation is not evidence that it caused it.**
18. **A WALK WITH NO BOUND IS WORSE THAN AN ASSERTION THAT FAILS** (dc's). **And the bound firing IS the finding.**
19. **THE WORKING DIRECTORY PERSISTS BETWEEN TOOL CALLS.** Ten times in one day. **Absolute paths.** And **zsh is not bash**: `$var` does not word-split, an unmatched glob (`--include=*.rs`) aborts the whole command, `2>&1 > file` sends stderr to the OLD stdout. **A status taken from the end of a pipeline is the LAST STAGE'S status.**
20. **AN ORDINARY LANGUAGE IDIOM THAT IS A LOAD-BEARING DELIMITER HERE.** A second `#[cfg(test)]` in a shipped file truncates `no_pm_state_in_output`'s scan. **A warning fix is a change like any other.**
21. **A COMMENT DOCUMENTING A HIGHLANDER VIOLATION IS THE VIOLATION PLUS A RECORD THAT SOMEONE NOTICED** (vc's), **which is strictly worse than not noticing -- it converts a defect into an accepted one.** `nav::face_pointer` named its own duplicate for weeks.
22. **A NEAR-MISS CLASSIFIED IN A NODE'S HEAD IS AN EXCLUSION RECORDED NOWHERE** (vc's). Put the classification in the declared population, not in a message.
23. **A COMMIT MESSAGE IS NOT A DURABLE HOME FOR A FINDING** -- no verb can edit one. The record goes where a reader will meet it: the board, or an inbox, with a pointer to the sha.

## Decisions

- **(hv) 3.0.1 scope is ST0056 + ST0058/0066/0068. No 3.1.0.** **`claude subagents` and `plugin` are IN** on v2-parity grounds.
- **(hv, via vc) THE TUI DIVERGES FROM WHAT WAS AGREED** and hv will scope it with ic directly. Section 9's plural-path question is superseded until then.
- **(vc) AN MCP TOOL CALLS THE FACADE, NOT THE CLI DISPATCH ARM.** Thin Coordinator: the CLI is one face over the facade, MCP is a SECOND face, not a client of the first. **So a CLI gap is irrelevant to MCP** -- `fc` is a real tool. **The refusal is measured against the facade: every `exposed_on_mcp: true` row has a facade method behind it**, and where the CLI composes, ADD a method rather than let the field express a composition.
- **(vc) THE MCP GENERATOR STAYS IN `intent-cli`** -- model down, face in the face.
- **(vc) `AC-09.4` CLAUSE 2 IS ESTATE-SCOPED**, because a guide-scoped clause 2 would restate clause 1, and a reading that makes half a row redundant is the wrong reading.
- **(vc) WHERE A STRUCTURED FIELD EXISTS FOR A CLAIM, PROSE STATING THE SAME CLAIM IS A SECOND HOME AND GOES** -- `basis` carries provenance, not behaviour that rots. **Qualified the same day by watch-out 8: only where the field actually carries THAT claim.**
- **(vc) A SUPERSEDED SENTENCE IS KEPT WHEN THE ARTEFACT CARRIES AN ARGUMENT, DELETED WHEN IT CARRIES A CLAIM ABOUT THE SURFACE.** A false surface claim gets ACTED on; a failed argument gets RE-ARGUED, and only one is cheap.
- **(ic, ruled by vc) THE REGISTER IS NOT A WITNESS FOR ITSELF.** I read `browse`'s DECLARATION as a build and put the wrong answer inside the criterion governing it. A declaration is a claim about the surface PLAN, never about the arm.
- **(ic, measured) TWO RATIFIED VOCABULARIES NAME THE SAME THINGS DIFFERENTLY.** An address says `/threads/ST0056/ac`; a view path says `/thread/ST0056/criteria`. `nav::view_for` is the AUTHORED translation and the one home.
- **(ic) Every path segment is a name the declaration already carries.** No pluralising.
- **(vc) Register writes are ic's.** `dispatch-table.json` is AUTHORED canon; `.md` is GENERATED.
- **(dc/hv) ST AND WP GET NO STATUS VARIANT** -- `fiat` sits BESIDE a status that stays `completed`/`done`.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only.
