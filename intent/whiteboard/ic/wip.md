---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 02:09Z
status: active
focus: "Main is 1574 passed / 1 failed and the one is dc's schema-version bump. Everything of mine is IN. WP-17 pieces 1+2 done, piece 3 has the mode machine and the terminal guard; the daemon FAMILY landed with `daemon status --format`, so ST0064's address resolution has a row. Next: ratatui with the first thing that DRAWS, and ST0064's WP-01 body which is BLOCKED -- 0154."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260829/wip-fold-2353Z.md`. This file is the COLD-SESSION MINIMUM: state, not story.**

## DOING

**RE-MEASURE EVERYTHING BELOW AT PICKUP. Four nodes write this tree and every figure here is spent the moment one does.**

**MAIN IS 1574 PASSED / 1 FAILED.** The one is dc's `a_face_whose_contract_moves_must_bump_that_faces_version` -- the schema versions need bumping after `fiat`. Nothing of mine is uncommitted.

**WP-17 PIECES 1 AND 2 ARE DONE. PIECE 3 HAS ITS TWO DEPENDENCY-FREE HALVES**: `tui/mode.rs` (`1331fcdf`, six invariants) and `tui/terminal.rs` (`ebedff7f`, five), all eleven mutation-proven, `crossterm` in and `ratatui` deliberately NOT until something draws.

**THE `daemon` FAMILY LANDED** (`ad046a45`) -- four verb rows plus a head, `daemon status --format terminal|json`, and `recoverability_anomaly` on start/stop/run because they are REVERSIBLE and withheld for process control. **ST0064's address resolution now has a row**; the arm is cc's.

**`flag()` IS GONE** (`0fcf471a`), 24 sites, `IN-AG-NO-SILENT-001`. **The ellipsis is DERIVED** (`84d51e0b`) -- the register authors no `...` anywhere. **`surface` takes `--format`** (`be76955b`), my own deviation, ruled and fixed under my name.

**FILED: `0153`** (`edit`'s remedy names an address form it cannot parse) and **`0154`** (no entity's authored prose is editable after creation -- the CLASS, with `0090` and `0151` as its instances).

**BLOCKED AND IT IS NOT MINE TO UNBLOCK: ST0064's WP-01 body** still specifies the superseded `GET /_status`. vc assigned me the correction; `0154` is why I cannot make it.

## TODO

1. **WP-17 piece 3 -- the realiser. TWO HALVES IN, BOTH DEPENDENCY-FREE BY DESIGN.** `tui/mode.rs` (`1331fcdf`) is the declared mode graph, six invariants; `tui/terminal.rs` (`ebedff7f`) is the borrow-and-return, five invariants, `Drop` PLUS a panic hook because the hook runs BEFORE the message prints and `Drop` cannot. **Both are provable with no tty**, which is the point: the realiser is what they check. **`crossterm` is in (+18 crates, 245 -> 263), announced to all three before the lockfile moved and held until the tree built.** `ratatui` goes in with the first thing that DRAWS -- a lockfile move that delivers nothing justifies itself later or not at all. **Next: the event loop** (`mode::step` is the authority; an undeclared key is `None`, never a self-loop), **then the editor handoff -- the EXISTING `launch_editor` + `$VISUAL` resolver, and the RETURN re-reads the artefact before painting anything derived from it (`AC-17.10`).**

2. **ST0064 -- the macOS menubar app.** Reference to adopt as-is: `~/Devel/prj/Gtools/native/macos/Geodica`, 2,470 lines of Swift; **structure is the deliverable, not the code**. **LIFECYCLE SHELLS OUT** (starting a daemon that is not running cannot be a request TO that daemon; `AC-00.3`) and **STATE COMES OVER THE WIRE**. **THE PORT IS GONE, NOT CHOSEN** -- the daemon binds `127.0.0.1:0` and writes its address into its state dir. **No port literal in the app.** `URLSession`-over-TCP is Geodica's 127-line service against `NWEndpoint.unix` needing no token but hand-rolled HTTP framing; vc wants that priced from the Geodica code with a WP size. ST0064 has zero criteria and a scope of `S` that will not survive contact. Control plane + console only in v1. Logo is `docs/design/intent-logo.*`. **`intent lang init swift` before the first `.swift` file or the app ships outside the critic gate.**
3. **WP-16 (S)** -- `data-model.md` against the schema, both directions. **`lib_staged.sh` ALREADY IS the committed-read mechanism** (`git show :<path>`); `machine_table_check.sh` already parses this document's tables. Source them; do not grow a second. **dc is editing both right now** -- coordinate before touching.
4. **WP-09 (L)** -- `AC-09.4` first. `AC-09.1` needs every row to declare `exposed_on_mcp` and `read_or_mutate`.
5. **`0142`'s structural half** -- much smaller after vc's zero-new-concepts ruling.
6. **ST0065** -- out of the 3.0.1 cut. Three hv rulings owed.

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
14. **A HELPER THAT READS A DECLARED THING THROUGH A TYPE ASSUMPTION RETURNS A PLAUSIBLE FALSE WHEN THE ASSUMPTION BREAKS, AND `.ok()` IS WHERE IT HAPPENS.** `fn flag` reads every match as `try_get_one::<bool>().ok()`; clap stores a `bool` for `SetTrue` and ONLY that, so a flag declared WITH A VALUE is a type mismatch, the `.ok()` discards it, and the flag reads as ABSENT -- given, parsed, invisible. `intent edit ... --editor` printed the path while `intent st edit ... --editor` opened an editor. **The fallback was a reasonable thing for the command to do, so it read as a design decision.** Agreement and correctness look identical from outside.
15. **THE COMMIT GATE READS THE INDEX, SO A STAGED INTERMEDIATE BLOCKS EVERY NODE.** dc's half-moved `data-model.md` against an unmoved `transitions.rs` refused MY unrelated commit, correctly. **Measure whether it is yours before asking anyone to move**: the same guard against a clean worktree carrying only my files returned rc=0. And **never reach for `--no-verify`** -- the guard was right.
16. **A WORKTREE IS A MUTABLE SHARED RESOURCE AND ITS STATE IS NOT WHAT YOU LAST SET IT TO.** `git checkout --detach` ABORTS on a dirty file and prints `Aborting` -- I read the compile error below it and not the refusal above it, and measured the wrong commit. A baseline taken at one HEAD is void for a comparison at another. **Every measurement now asserts HEAD, cleanliness, and the presence of what it needs BEFORE it measures**, and a binary is asserted to carry the change before it is driven.

17. **`sync --to-store` HAS NO UNIT NARROWER THAN A THREAD, SO A CANON SYNC IS A SUITE-WIDE WRITE OF WHATEVER IS DIRTY.** Editing one tracked attachment (`gen_dispatch_table.sh`) put my commit behind a canon sync that would have ingested vc's uncommitted `acceptance.md`. **Nothing would be lost and that is not the point** -- the bytes move under someone else's name. Ask; do not sweep. And **never `--no-verify`**: it takes the clock and header guards with it.
18. **THE DISPATCH TABLE IS COMPILED IN, WHICH MAKES IT THE WORST FILE IN THE REPO TO BE MID-EDIT IN.** A malformed one is not a bug a user could hit, it is a binary that will not start, for every node building from the shared worktree. `values: null` -> `invalid type: null, expected a sequence` at `dispatch.rs:680`, before any work. **I wrote that null because `jq` prints `null` for an ABSENT key and I read the print as the value.** Absent and null are different declarations and one of them panics.
19. **A GREP FOR A CALL SITE CANNOT FIND ONE THAT TAKES A VARIABLE.** Deleting `flag()`: vc's grep found 3, a second found 12, mine found 17, **the compiler found 24** -- `flag(m, other)` matches no literal pattern and never could. Four enumerators, four answers, each short of the next. **Assert the END STATE, never a count**, and let the build enumerate when it can.
20. **REMOVING A SWALLOW MEANS FINDING OUT WHAT IT WAS SWALLOWING, AND THERE IS NO WAY TO FIND OUT EXCEPT TO REMOVE IT.** `flag()`'s `.ok()` was hiding a type mismatch AND a legitimate case -- `value_source` PANICS on an id a subcommand does not declare, and `--force` is declared on some `claude skills` verbs and not others. My first `given()` crashed 47 tests. **The warning was three thousand lines up in the same file and I QUOTED IT into the new exemption while shipping its violation.** Reading a warning and applying it are different acts.

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
