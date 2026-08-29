---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-29 23:53Z
status: active
focus: "WP-17. Piece 1 (form DSL) is COMMITTED at 771cc847, source only. Piece 2 (edit reshaped + browse twin + INV-09/INV-10 + arity on flags) is AUTHORED AND GREEN IN THE WORKTREE, waiting on a commit order: dc takes render.rs, cc takes lib.rs. Then piece 3 (TUI) and ST0064 (menubar). vc holds the pen while hv is AFK."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260829/wip-fold-2353Z.md`. This file is the COLD-SESSION MINIMUM: state, not story.**

## DOING

**RE-MEASURE EVERYTHING BELOW AT PICKUP. Four nodes write this tree and every figure here is spent the moment one does.**

**COMMIT ORDER IS THE ONLY THING BLOCKING ME, AND IT IS SEQUENCED, NOT STUCK.** `form.rs` is in at `771cc847` — **source only, deliberately without its `pub mod form;` line**, because `lib.rs` also declares cc's `daemon` and neither source was committed: whoever landed `lib.rs` first would put a module declaration into `main` without the module. **An orphan source file compiles**, so `main` is green at every step rather than only at the end. Order from here: **dc takes `render.rs` whole, then cc takes `lib.rs` + `daemon.rs`** and names my one-line declaration. My `tests/form_declares_layout_not_the_field_set.rs` follows `lib.rs` — it references `intentsvcs::form` and would red `main` before then.

**`render.rs` HAS THREE AUTHORS AND I COUNTED TWO.** `fn engine` is cc's, `fn surface`/`fn surface_retired`/`fn browsed` are mine, `fn fc` is **dc's**. I had attributed `fc` to cc from a diff read too fast, and had I committed first I would have signed dc's work under my message. **The measurement that settles authorship is `grep -c 'fn <name>(' ` against HEAD, not a glance at a diff.**

**WHAT IS UNCOMMITTED AND MINE:** `surface/dispatch-table.json` + `.md` (the `edit` reshape, the `browse` row, INV-09, INV-10, the `fc` wiring split, `arity` on flags), `intent-cli/src/dispatch.rs` (`twin_of` + `arity` deserialize), `intent-cli/tests/{twin_spellings_agree,declared_values_are_enforced,unmigrated_surface,self_loop_population,retirement_is_enumerable}.rs`, `intentsvcs/tests/{form_declares_layout_not_the_field_set,write_moves_only_what_changed}.rs`, `surface/forms.json`, `intent/.canon/issues/0149.json`, `spine.rs`. **`spine.rs` is owed to cc.**

**LAST DRIVE:** `cargo test -p intent-cli` rc=0 / 65 targets; `cargo fmt --all --check` rc=0. `intentsvcs` has ONE failure and it is dc's: `a_machine_ratified_in_prose_is_actually_trivial` — `AcceptanceTest.status` now declares two verbs (`at.fc`, `at.set`) so it needs an edge table in `data-model.md`.

**PIECE 2 IS DONE IN THE REGISTER AND HAS ONE GAP LEFT, WHICH IS MINE.** `--editor` declares `arity: "0..1"` and **`spine.rs` does not read it** — `intent edit --help` renders `--editor <program>`, a required value, where the register says optional. Told cc at handover rather than leaving it clean-looking.

## TODO

1. **Close the `arity` gap in `spine.rs`** so the declared optional value is the built one. Then hand `spine.rs` to cc.
2. **WP-17 piece 3 — the TUI realiser.** Adding `ratatui`/`portable-pty`/`tui-term` MOVES `Cargo.lock`, which three nodes build against: **announce before touching it.** Nothing of the sort is in the tree. Mode machine as a DECLARED TABLE with two invariants checked headlessly (every mode leavable, every mode reachable from NORMAL) — `transitions.rs`'s own idiom, and `no_state_can_be_entered_and_not_left` applied to a controller. Editor handoff through the EXISTING `launch_editor` and `$VISUAL` resolver; **the RETURN re-reads the artefact before painting anything derived from it** (`AC-17.10`); terminal restored on every exit path including panic.
3. **ST0064 — the macOS menubar app.** hv named the reference to adopt as-is: `~/Devel/prj/Gtools/native/macos/Geodica`, 2,470 lines of Swift, **structure is the deliverable, not the code**. **Two paths: LIFECYCLE SHELLS OUT** (starting a daemon that is not running cannot be a request TO that daemon; `AC-00.3` ratifies CLI-owned launchd lifecycle) **and STATE COMES OVER THE WIRE.** **THE PORT IS GONE, NOT CHOSEN** — the daemon binds `127.0.0.1:0`, the kernel assigns, and the daemon writes its address into its state directory. **Do not put a port literal in the app.** HTTP carries one auto-generated token; the socket carries none (filesystem permissions are its authz), so `NWEndpoint.unix` needs no token but means writing HTTP framing by hand, against `URLSession` over TCP being Geodica's 127-line service. **vc wants that trade priced from the Geodica code, and the WP size with it** — ST0064 has zero criteria and a scope of `S` that will not survive contact. Control plane + console only in v1. Logo is `docs/design/intent-logo.*`, the turtle. **Swift is NOT a declared language: `intent lang init swift` before the first `.swift` file or the app ships outside the critic gate.**
4. **WP-16 (S)** — `data-model.md` against the schema, both directions. **`lib_staged.sh` ALREADY IS the committed-read mechanism** (it reads the INDEX via `git show :<path>`); `machine_table_check.sh` already parses this document's tables. Source them; do not grow a second.
5. **WP-09 (L)** — `AC-09.4` first. `AC-09.1` needs every row to declare `exposed_on_mcp` and `read_or_mutate`.
6. **`0142`'s structural half** — now much smaller than it looked. vc ruled the register needed **zero new concepts**: relations live in `invariants`, and `arity` already existed on `args`.
7. **ST0065** — out of the 3.0.1 cut. Three hv rulings owed.

## Watch-outs — mechanisms only

1. **THE FIELD IS THE CLAIM AND THE NOTE IS ITS SCOPE.** `Facade::settable_fields` answers _can the NARROW SETTER write this_, not _is this editable_, and using it put `status` — the one field `AC-17.4` builds `select` for — outside the writable set. `facade::unsettable_kind` is the ruled axis: `Elsewhere` means a machine's verb or a child address, which is what a `select` or `button` row IS.
2. **YOUR INSTRUMENT'S OUTPUT IS A CLAIM ABOUT YOUR INSTRUMENT, AND THE BASELINE IS THE ONLY THING THAT SAYS IT RAN.** A mutation run reported three clean kills and had measured nothing: dirty `facade.rs` staged onto a HEAD `model.rs` meant nothing compiled, and every _mutation killed the test_ line was a compile error wearing a kill's clothes. **A mutation worktree carries the WHOLE dirty state.**
3. **A PROBE'S LEAD IS PART OF THE PROBE.** Reshaping `edit` silently repointed `declared_values_are_enforced`'s existing probe at a different refusal — still exit 1, still naming a permitted set, about a question nobody asked — and it stayed GREEN. **Change a row's positionals and every probe keyed on it is measuring something else.**
4. **AN INSTRUMENT'S SHAPE IS A SILENT ASSUMPTION ABOUT ITS CORPUS.** That same harness had `lead` and no `trail` because every declared slot had always been its row's LAST positional. It held until the corpus changed, then measured an arity refusal instead of a vocabulary one.
5. **A DECLARATION KEY IS A PROMISE THAT CODE READS IT.** `key_classes` is blunt: if no type deserializes it, the behaviour is absent or hand-written to match by coincidence, _and the coincidence never surfaces because agreement looks exactly like correctness_. Classifying `twin_of` as a declaration obliged `twin_spellings_agree.rs`; a caution in a comment does not fail.
6. **`INHERITED_UNREAD` IS A RATCHET ON ADDITIONS, NOT AN EXCUSE LIST.** Adding a flag you just wrote is what turns one into the other. The fix for a declared-and-unread flag is the arm.
7. **`gen_dispatch_table.sh` REFUSES ON EVERY DERIVED COUNT WITH A SECOND HOME.** It caught the new-surface count, the `legal_pairs` census, three unclassified keys, an orphan invariant and an invariant's `target` shape — **five refusals in a row, each correct.** `populations` is TABLE ORDER, never sorted. **Let it check you; do not pre-empt it.**
8. **THE MARKDOWN FORMATTER IS A SECOND WRITER AND THE GENERATOR SEES IT.** Author `_em_` in register prose, never `*em*`: the generator emits what you wrote, the formatter normalises it on save, and the next generator run reports a diff. **Also: a BLOCKED commit still runs the formatters, so re-sync before retrying a refused commit** (vc's finding).
9. **A MULTI-WORD SPELLING PASSED AS ONE TOKEN READS AS A COMMAND THAT DOES NOT EXIST, IN EVERY LANGUAGE.** zsh: `${=v}` splits, bare `$v` does not; **`while read -r st path` DESTROYS `$PATH`** because `path` is tied to it; an apostrophe in a single-quoted string is a hard syntax error; an unmatched glob aborts the whole command. **Never `$?` after a pipe** — that one has now fired four times across this board and dc's.
10. **THE WORKING DIRECTORY PERSISTS BETWEEN TOOL CALLS.** A `cd native/rust` three calls ago is why a heredoc wrote nothing and a path resolved to nowhere. Use absolute paths or `cd` explicitly every time.
11. **A GREEN SUITE AND A GREEN CI ARE DIFFERENT GATES.** hv moved fmt and clippy into CI, so **there is no local fmt alarm by design** and a peer's eye is the only one. **And `cargo fmt --all` is the UNGUARDED twin of a guarded door, with the SHORTER spelling** — I ran it and reformatted `daemon.rs` and `facade.rs`, cc's and dc's files, an hour after cc apologised to me for the same thing. `--check` first, always.
12. **AN ATTACHMENT EDIT IS A SUITE-WIDE EVENT**, and the warning reaches the actor, never the node whose bytes moved. **Do NOT `sync --to-store ST0056`.** And a hand-edit to a GENERATED view is destroyed silently by the next sync — vc lost the WP-17 objective that way, in the file whose own footer says so.
13. **A DEBT YOU DECLINE TO PAY GOES IN THE FILE WITH THE FIX WRITTEN OUT, AND YOUR COUNT OF IT WILL BE LOW.** I wrote _three homes_ for the unwired marker; cc measured **12 files, six named constants under FIVE names**. **The five names ARE the finding.** The count only came from someone standing elsewhere.

## Decisions

- **(vc, on my finding) `AC-17.6` STANDS: `intent browse` SHIPS.** `ST0058 AC-00.6` refuses **DISAGREEMENT, not duplication** — driven off `--version` rc=0 against `version` answering the unwired marker. `tui-design.md` §9 had turned _must agree_ into _must not both exist_; the bullet is deleted rather than struck through.
- **(vc) THE REGISTER NEEDED ZERO NEW CONCEPTS — three costumes, one answer each.** Relations between rows are `invariants` (8 already, and none of them is a property a single row can state). The sub-row unit is the same invariant generalised: **every spelling of one capability agrees about whether it exists**, where two argument shapes of one row are two spellings. And **`arity` already existed** — 125 args declare it, 29 of them `0..1` — so it was extended to `flags`, not minted.
- **(cc, adopted) A BOOLEAN THAT IS GENUINELY BOTH IS BEING ASKED OF THE WRONG UNIT.** `probeable` asks about a ROW and `fc`'s answer varies WITHIN it; `partially_wired` would encode the confusion rather than resolve it. **cc's gate arm cannot reach it and they declared that boundary rather than widening a sentence** — the unit a defect lives in can be smaller than the unit an instrument is keyed on.
- **(ic) `rc=2` NOW HAS A MEASURED FALSE INSTANCE.** `fc <thread> AC-nn.n` → rc=1 through the arm; `fc <thread> AT-nn.n` → rc=2 _known command that is not implemented yet_, **for a verb the build provides**. Every other member of the marker set is a duplicated TRUE sentence; this is the true sentence emitted where it is false.
- **(ic) THE FACE IS DERIVED FROM THE ENTITY, NEVER DECLARED BESIDE IT**, and **not every entity is a schema root**: `WorkPackage` is `$defs/WorkPackage`, and mapping it to the FILE handed a wp form the thread's 18 fields — six of nine names overlap, so nothing refused.
- **(ic) A ROW MODELLED AS A FAMILY HEAD WITH A SUBCOMMAND SLOT HAS NO EXACT-PATH FORM**, so its exemption is exact and family-wide at once. **The precondition is a TEST, not a comment.**
- **(ic) One definition, two doors.** `retired_and_unreachable()` serves the exec-path refusal and `surface retired`. **`table.retired()` alone names `organize`, which RUNS.**
- **(ic) Share the FACT, not the prose.** `Replacement` is three arms; `null` (nobody said) and `""` (declared none) stay distinguishable across the wire.
- **(dc/hv) ST AND WP GET NO STATUS VARIANT** — `fiat` sits BESIDE a status that stays `completed`/`done`, so `st.fc`/`wp.fc` are STRUCK, not pending. **`FANS_OUT`'s guard checks the right-hand side only**, so a speculative row would have passed in silence for exactly as long as it was wrong.
- **(hv) 3.0.1 scope is ST0056 + ST0058/0066/0068. No 3.1.0 — all v3 pain lands under one rubric**, which is what licensed `edit`'s shape change.
- **(vc) Register writes are ic's.** `dispatch-table.json` is AUTHORED canon; `dispatch-table.md` is GENERATED — regenerate, never hand-edit.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only.
