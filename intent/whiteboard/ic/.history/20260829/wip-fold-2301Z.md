---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-29 23:00Z
status: active
focus: "HOLDING for vc. Cleared a shared-tree red on my own `surface retired` row (dc found it, ic ruled it): exempt on purpose grounds, hazard guarded by a test. `intent-cli` rc=0 61/366/0, fmt rc=0. Earlier: `intent surface retired` built and driven (hv ruled the spelling), the `fc` register row, four false doc homes corrected, issues 0146/0147/0148. NOTHING COMMITTED and the next ST0056 commit is a JOINT act across nodes -- read DOING before touching git."
claims: [ST0065, ST0056/09]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260829/wip-fold-2222Z.md`; handled inbox entries beside it. This file is the COLD-SESSION MINIMUM.**

## DOING

**vc SEQUENCED ME: AC-00.5 -> WP-16 -> WP-09.** Landed since the fold: **dc found `cargo test -p intent-cli` RED at rc=101 in the shared tree on MY `surface retired` row, and it is fixed** -- `surface` ruled exempt from the unmigrated refusal on purpose grounds, dc's family-wide hazard closed with a test rather than a comment. Full crate now rc=0, 61 targets, 366 passed, 0 failed, `cargo fmt --check` rc=0. dc and vc both told.

**THE DELIVERED CLI NOW REFUSES, AND THE REPAIR IS BEHIND THE JOINT COMMIT.** dc's uncommitted `SCHEMA_VERSION` 14->15 (`store.rs:458` worktree, HEAD still 14) migrated the shared per-machine store to 15. Measured on one tree: `~/.local/bin/intent` (`30a2dd81`) speaks 14 and answers **rc=1 on every store-reading verb**; `native/rust/target/debug/intent` (`dirty-180fb4a3`, built after the bump) speaks 15 and works. **The discriminator is WHEN YOUR BINARY WAS BUILT, not who you are** -- cc reported it as estate-wide and it is not. Repair is a release build, which the shared-artefact guard refuses while `native/rust` is dirty. **So `0148` is a live outage, not a tidiness item.** The refusal carries **rc=1**, so it cannot join `retirement_is_enumerable.rs`'s rc==2 unbuilt population -- **and cc was right that filing that as luck was wrong.** The population is defined by a PREDICATE and a predicate admits whatever later matches it, so the test now checks each member carries the not-implemented-yet marker (mutation-proved). **Following it down: `rc=2` carries FOUR meanings and only PROSE separates them** -- `AC-00.5` made only RETIREMENT enumerable, the register has no built-ness field by construction, so the message is the ONLY oracle available rather than a lazy one.

**ST0058 `AC-00.5` IS PROVEN AND BLOCKED ON THE COMMIT, NOT ON THE WORK.** Its falsifier is now `tests/retirement_is_enumerable.rs` rather than a transcript -- retired vs unbuilt separated on exit code plus roster membership alone, no message text, mutation-proved both directions. **It cannot be SATISFIED yet**: this estate's evidence convention names a committed revision (`AC-00.2` cites `31d9de1f`) and my work is uncommitted. Satisfying it now would assert a capability off a dirty worktree -- the `st repair` class, which is what today was spent correcting.

**`at.fc` TAKEN ON THE REGISTER SIDE OF dc's FIAT CASCADE.** `FANS_OUT` gains `("at.fc", &["fc"])` and the ruling generalises from `ac.fc` to any fiat-close edge. `st.fc` / `wp.fc` need one line each WHEN THEIR EDGES LAND -- deliberately not pre-added, because `the_fan_out_mapping_names_only_rows_that_exist` checks the rows, not the verbs, so a speculative entry passes in silence.

**WP-16 GROUNDWORK, NO CODE YET.** The population derives to **11 entities / 67 properties** from the three JSON schema faces (each face's root plus its `$defs` objects) -- matches vc's ruling, `Invoker` and `Subject` included, and the event root is titled `Envelope` rather than guessed from the filename. **The committed-read mechanism ALREADY EXISTS and is `lib_staged.sh`**, which reads the INDEX via `git show :<path>`, not HEAD -- sharper than this board's earlier note, and ratified after the class bit the estate three times. WP-16 sources it; it does NOT grow its own. `machine_table_check.sh` already parses this document's tables for the state machines, so it is the sibling to read before writing a second parser.

**READ THIS BEFORE TOUCHING GIT. THE TREE IS UNCOMMITTED AND THE NEXT ST0056 COMMIT CANNOT BE MADE BY ONE NODE.** `intent sync --to-store ST0056` took TWO nodes' uncommitted attachment bytes into canon -- mine (`parity/tools/gen_reference.sh`) and dc's (`data-model.md`). `canon_commit_check.sh` exits 1 on the divergence (vc verified), so **the commit must carry `intent/.canon/st/ST0056.json`, `data-model.md` AND `gen_reference.sh` together or it is refused.** `git commit --only <path>` cannot assemble that. **hv holds commit authority and has been told twice.** Filed as `0148`. **vc 2026-08-29: it is now FIVE paths** -- `design.md` and `tui-design.md` joined as ST0056 attachments. **Do NOT `sync --to-store ST0056`.**

**`intent surface retired` IS BUILT AND IS NOT IN THE DELIVERED BINARY.** Driven in debug: 8 rows, `organize` correctly absent, JSON parses off stdout, bare `intent surface` refuses rc=1. **It needs a rebuild to ship, and a rebuild is a shared-artefact act nobody has taken.**

## TODO -- vc's order, resume here

0. **AC-00.5 -- DONE bar the satisfy, which is blocked on the commit.** See DOING.
1. **WP-17 DSL (vc, 2026-08-29, assigned to ic) -- OUTRANKS WP-09 because it is on cc's critical path.** Design is RATIFIED and I have NOT read it yet: `intent/st/ST0056/tui-design.md` (318 lines, 13 sections) + `design.md` `D56` for the web stack. Eleven criteria AC-17.1..17.11, all with ATs. Three pieces IN ORDER: **the form DSL** (layout only; field existence and type come from the schema face and are NEVER enumerated -- `AC-17.2`, held both ways), then **the `intent edit` register row** (mine by SSOT; `--path` MUST survive as a documented output contract; `intent browse` must NOT also ship, per `ST0058 AC-00.6`), then **the TUI realiser**. vc names five decisions to READ rather than rebuild. Reference impl is throwaway; the document is the deliverable.
2. **WP-16 (S)** -- the `data-model.md`-against-schema drift check. vc ruled DERIVE 11 entities (not the 9 the criterion listed -- that enumeration was deleted as the list it warned against). First-run RED on 4 missing tables is CORRECT and vc is writing those tables. **Read a COMMITTED revision and name it in the output**, never the worktree.
3. **WP-09 (L)** -- MCP + agent guide, 5 criteria, Not Started. `AC-09.4` first: `intent llm` renders from the dispatch table, no hand-maintained list. `intent mcp` refuses rc=2 today.
4. **0142's structural half (S)** -- refusals have no declared home in the register, so the only place to state one is a help string nothing checks. Why `0147` exists.
5. **ST0065** -- OUT of the 3.0.1 cut. Three hv rulings owed (does AGENTS.md exist at fresh init; what is it a mirror OF; generator-or-copy for the index given `usage-rules.md` cannot join). Catalogues written. **`_AGENTS.md` has already moved under the unruled proposal (`8a997c1e`) -- accepted out loud, not a blocker.**

**OWED TO PEERS:** cc -- coordinate the built-ness marker their gate arm emits so the reference can mark unbuilt rows (`fc` is the first case). dc -- re-read the `fc` row against their probe when it is green.

## Watch-outs -- mechanisms only

1. **In this register the FIELD is the claim and the NOTE is its scope.** Retracted twice in one day by notes sitting beside fields I had read: `populations.shipped` (its `why` says do not union the arrays), and `st help` (its note says v2 bare-word arm).
2. **A partial read reports in the shape of a complete one, and an ABSENCE concluded from one cannot be falsified from inside.** Cost dc two wrong reports in an hour; costs me the same shape via fields-without-notes.
3. **A zero from your own instrument is a claim about the instrument.** My jq keyed `name` where the register says `spellings` and returned 0 flags across ALL families; only the control caught it.
4. **A harness that runs the subject twice cannot measure a verb that refuses duplicates.** Mine printed `ok: AC-01.1 created` at `rc=1`. The impossible pairing is a free control.
5. **`at lint` exempts `to-write` from L2/L3, so `conform` can mean EXEMPT rather than CHECKED.** Drive a row off `to-write` before believing a green.
6. **Snapshot before a mutating verb whose effect you will want to characterise.** A HEAD-vs-now diff attributes to your run every change since HEAD, and in this tree the baseline moves under four people.
7. **A tool in a family does not inherit the family's behaviour.** I inferred `gen_reference.sh` read the git index from `rulings_check.sh` doing so. It reads the worktree (`lib_surface.sh:39-51`); the pinning is elsewhere. Citing a sibling is not reading the tool.
8. **`gen_dispatch_table.sh` REFUSES on every derived count with a second home** -- new-surface count, `legal_pairs` census, `populations` (TABLE ORDER, never sorted), the MCP-withhold derivation, and render idempotency (it caught double spaces the renderer collapses). **Five refusals in one session, every one a real defect. Let it check you; do not pre-empt it.**
9. **An attachment edit is a SUITE-WIDE event, and its only fix adopts other nodes' uncommitted bytes.** `0148`. The warning reaches the actor, never the node whose bytes moved.
10. **A MULTI-WORD SPELLING PASSED AS ONE TOKEN READS AS A COMMAND THAT DOES NOT EXIST, IN EVERY LANGUAGE.** zsh: `${=v}` splits, bare `$v` does not. Rust: `.args(&[path])` with `path = "st repair"` is ONE argv element and clap answers rc=1 _unrecognized subcommand_ -- indistinguishable from an absent verb, and it made my own AC-00.5 test report the retirement refusal as missing. Split the spelling at every boundary you hand it across. **Never `$?` after a pipe.** An apostrophe inside a single-quoted string is a hard syntax error. `bash -n` an edited shell file AND drive it.
11. **A mutation-proof runs in a DETACHED WORKTREE with its own target dir.** Removing a filter still COMPILES, so an in-place mutation hands peers a working binary with wrong behaviour -- worse than a broken build.
12. **Check WHICH assertion fired when a test goes red.** A test can go red for the wrong reason.

13. **A GREEN SUITE AND A GREEN CI ARE DIFFERENT GATES, AND THE SUITE CANNOT SEE THE FORMATTER.** `cargo test -p intent-cli` was rc=0 while `cargo fmt --check -p intent-cli` was rc=1 on my own `surface_retired` arm. Run BOTH before handing anything to a commit; the pre-commit formatter is a second writer and CI is where the consequence lands.

14. **A POPULATION DEFINED BY A PREDICATE ADMITS WHATEVER LATER MATCHES IT.** Mine kept rows refusing at rc==2; a store refusal at rc==2 would have made a LIVE verb read as unbuilt. Give every predicate-derived population a member-level check against a second channel, and mutation-prove that check. (cc, bitten twice this week.)
15. **A DEBT YOU DECLINE TO PAY GOES IN THE FILE WITH THE FIX WRITTEN OUT.** The unwired marker now has three homes; the right fix is one `pub const` in `render.rs`. Not taken mid-outage, because widening into a shared file carrying a peer's live work is the wrong trade -- but named, so the next holder can land it in a minute rather than discover it.

## Decisions

- **(hv) `intent surface retired`** -- spelling ruled 2026-08-29 over bare `retired` and over `llm retired`. **Cost stated and taken: a 14th top-level root holding one leaf.** `surface/` is already the estate's word for the register and gives later surface queries a namespace. **The verb must NEVER encode membership in its own exit code** -- that reinstates the contradiction it was minted to resolve.
- **(hv, 2026-08-25) Retirement is enumerable, not a new exit code.** `spine.rs` keeps its exit-code decision untouched; a caller branches on MEMBERSHIP.
- **(hv via vc) `fc` is 1 AND 3:** wire the arm before the tag, AND cc's built-ness gate as the class fix. Not alternatives.
- **(hv) 3.0.1 scope is ST0056 + ST0058/0066/0068. Docs ship WITH the tag, written against the CUT.**
- **(vc) Register writes are ic's.** `surface/dispatch-table.json` is AUTHORED canon; `dispatch-table.md` is a GENERATED view -- regenerate, never hand-edit.
- **(ic) One definition, two doors.** `retired_and_unreachable()` is read by the exec-path refusal and by `surface retired`. Two doors onto one roster is the design; two answers to _is this retired_ is the divergent-copy shape. **`table.retired()` alone names `organize`, which RUNS.**
- **(ic) Share the FACT, not the prose.** `Replacement` is a three-arm enum because a refusal speaks a remedy and a roster prints a column; a shared string would make one door parse the other's wording. `null` (nobody said) and `""` (declared none) stay distinguishable across the wire.
- **(ic, adopted by dc) `FANS_OUT`: do not split it -- rename it, and each member states its shape.** The members were admitted for one reason (the default derivation is wrong for this op); only the NAME overclaims.
- **(ic) A record may QUOTE what was said, but must POINT AT, never reproduce, what is currently true.** A basis arguing for a value the vocabulary already declares is a second home for a settled fact, and reads as open until someone looks.
- **(ic) A surface claim travels with what makes it checkable -- and the register answers _does this exist_, never _does it still do what the last doc said_.** For a behaviour change the check is the source or the test.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only. **`add + commit --only + reset` is NEW files only.**
- **(ic, on dc's finding 2026-08-29) `surface` is EXEMPT from the unmigrated refusal, and the recorded ground is PURPOSE, not mechanism.** "It never opens a project" is true and is the weaker half. hv ruled retirement enumerable so a caller who typed a retired v2 command could learn what replaced it -- **and that caller is almost always standing in an unmigrated project, so this verb's primary case IS the state the sweep tests.** Third member of the `llm guide` / `claude rules` class, not a new ground.
- **(ic) A ROW MODELLED AS A FAMILY HEAD WITH A SUBCOMMAND SLOT HAS NO EXACT-PATH FORM.** `entry.path` and `entry.path.split(' ').next()` are the same string, so an exemption for it is exact and family-wide at once and a future sibling inherits it silently. `plugin` carries that as prose; I did not add a second comment. **The precondition is a TEST that fails when a second leaf lands** -- positive-controlled, so it cannot pass vacuously. A caution in a comment does not fail.
