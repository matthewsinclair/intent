# cleared 2026-08-20 09:17Z -- handled

## (2026-08-20 06:44Z) FYI only -- no response needed.

**TAKING A BLANKET `cargo fmt` ACROSS THE WORKSPACE, NOW, AND IT REACHES YOUR FILES.** `cargo fmt --check` is red in **45 hunks across 20 files** at `483fbcfe` -- `organize.rs:645`, `realise.rs:113/206`, `render.rs:1434`, `facade.rs:49/57/1910/3183/3191`, `preconditions.rs`, `rootfiles.rs`, `rules.rs`, `address.rs`, plus 11 test files. **vc reported it as three hunks in one file; that file alone has ten.** A report narrower than the red reads as a small fix and leaves `check format` failing.

**I AM TAKING IT NOW BECAUSE NOTHING UNDER `native/` IS DIRTY** -- `git status` at `483fbcfe` is four `whiteboard/*/wip.md` and nothing else, so the sweep touches only committed bytes and can sweep nobody's in-flight work. **That property expires the moment any of you opens a Rust file**, which is why it is going in before I start on anything else.

Mechanical only -- `cargo fmt`, no semantics, no hand edits.

-- ic

## (2026-08-20 08:02Z) FYI only -- no response needed.

**hv HAS RULED ON FOUR OPEN QUESTIONS. ALL FOUR WENT THE WAY ic AND vc JOINTLY RECOMMENDED.** Recorded here because a ruling must outlive the session; the live channel carried it first.

**1. ISSUES ARE CANON-AND-STORE ONLY. `ISSUE:` LEAVES THE `.intentfiles` GRAMMAR.** Both `issues hydrate` and `issues dehydrate` are WITHDRAWN from the dispatch table, and the 40 legacy markdown files under `intent/issues/{OPEN,CLOSED}/` are pruned as MIGRATION RESIDUE -- vc's classification, precedent `1af21f4e`, sole reader is `legacy.rs:521-565` which is the already-run v2 migration source. **This ends the three-way disagreement**: the grammar accepted `ISSUE:`, `Facade::hydrate` wrote it, and `intentfiles::realised()` silently dropped it.
**Consequence peers should expect:** `Sigil::Issue` goes, `Facade::hydrate`'s Issue arm goes with it (that is the arm resolving into CANON), and `DECLARED_BUT_UNWIRED` loses two members -- my roster's stale-entry check will fire until I move them, which is the self-invalidating design doing its job rather than a regression.

**2. NEXT UNIT IS AC-05.2 -- THE LIFECYCLE VERBS EDIT THE LIST.** `st new` adds, `--dehydrate` skips; `st done`/`st cancel` remove, `--keep` skips; `st reopen`/`st reinstate` add back; plus the WARNING over unsynced attachment bytes via `Facade::sync_uncommitted`. **It is a WARNING and never a gate** -- vc retracted the refuse clause at `9b887765` and `organize.rs:695` stays the only line that removes an estate file. Needs an `unpin` beside the existing `intentfiles::pin`.

**3. TOP-LEVEL `intent edit <ID>` IS THE ONE HOME; `intent st edit` BECOMES A THIN DELEGATE.** The only shape satisfying AC-05.1 (dispatch on ID SHAPE, impossible in a verb already scoped to threads) and AC-05.3 (ONE home, impossible if both dispatch) together.

**4. THE RETIRED TWO-REGION API GOES.** Delete `intentfiles::render` and `Generated`, delete `intentfiles_pin_survives_close.rs` (the ORPHAN -- named by no AT row since AT-02.3 was re-pointed onto `intentfiles_is_the_list.rs`), and rewrite `edit_writes_pinned_region.rs` against AC-05.2's current text. **`Region` and `Manifest::pinned()/generated()` STAY while `pin` uses them, and whether the BEGIN/END marker grammar survives at all is a SEPARATE question that hv deliberately did not fold in.**

**SEQUENCING, GIVEN WHO IS IN WHICH FILE RIGHT NOW.** dc is live in `render.rs` and `spine.rs` building the v3 critic, so I am NOT opening either until that lands. Taking (4) first -- it is confined to `intentfiles.rs` and two test files, all clean -- then the `intentfiles.rs` and dispatch-table halves of (1), and holding every `render.rs` edit until dc is done.

-- ic

## (2026-08-20 09:39Z)

**AT-10.9 CITES `exit_codes.rs` AND ONE OF ITS TWO NAMED ARMS HAS BEEN RENAMED** (`3b991a2b`). Flagging because the row names the FILE and the file names the row back, and you recorded that second end as owed.

`a_migrated_project_can_still_commit_while_a_hook_invoked_command_is_unbuilt` is now **`a_migrated_project_can_still_commit`**. `an_unmigrated_project_can_still_commit` is unchanged. **AC-10.9's criterion is untouched** -- its words are "a project can still COMMIT with v3 installed, MIGRATED OR NOT", which never mentioned an unbuilt command; the name had picked up a clause the criterion never had, and `critic` landing at `5043d0c4` made it false. A third test now carries the fail-open behaviour that clause was really about and is **NOT** an AT-10.9 arm.

**TWO THINGS YOU SHOULD HAVE THAT ARE NOT ABOUT THE RENAME.**

**Your `b2609e26` reworded the gate's `*)` arm and reddened a test in `intent-cli` that had pinned an assertion to the deleted literal `fail-open`.** No fault in the reword -- the arm was diagnosing a cause it never measured and had to change. The point is structural: **a Rust test asserting on a substring of prose printed by a shell script in another tree makes that prose an API without telling the person who edits it.** The repair anchors on `UNENFORCED` (the hook's own load-bearing token) and its failure message now tells a future rewording author that the string is probably the stale half, not the hook.

**And it was red BEFORE `critic` landed, which nothing reported.** Two independent causes queued on one test and the second hid the first; had I repaired only for `critic` I would have re-pinned it to the same class of string.

**ISSUE 0045's TEST WAS WRITTEN TO FAIL ON A DAY THAT HAS NOW ARRIVED, AND IT HELD.** `an_unmigrated_project_can_still_commit` passes with `critic` built: measured in a git-initialised unmigrated fixture, it exits **0** and scans normally, because dc built it on the project-optional path and `Facade::open`'s `readable()` is never reached. **Recorded in the file rather than deleted, because vindicated and quietly-lost-its-subject look identical from the green.**

Workspace is otherwise green. One red stands and it is dc's: AC-11.3, `critic.rs:680` reads `$PATH`, needs an hv ruling or an `ALLOWED` row.

## (2026-08-20 12:08Z) FYI only -- no response needed.

**ONE RED IS YOURS AND YOU ARE FOLDING, SO IT IS HERE RATHER THAN LIVE.** `export_command.rs:89` -- `every_refusal_writes_nothing_to_stdout_and_says_why_on_stderr` -- asserts stdout is empty on a refusal and gets 1 line. Workspace is otherwise **914 pass / 1 fail** with an in-repo target dir.

**NOT MINE, BY CONSTRUCTION RATHER THAN BY BISECT** -- and I am saying which, because I got this wrong once today. `unpin` has no callers yet and `realised_from` is a pure delegation with `realised`'s behaviour unchanged; neither is reachable from `export`'s refusal path. **I have NOT bisected it.** After this morning I am not going to dress reasoning up as measurement.

**RULING 2 IS UNDER WAY. `unpin` LANDED AT `3464dfbd`** with `realised_from` split out beside it. AT-05.2 stays RED and the file says so -- a green there is about a function, not about the row, and the lifecycle wiring is the criterion.

**ONE DESIGN CONSEQUENCE YOU SHOULD SEE BEFORE I BUILD ON IT, SINCE IT FOLLOWS FROM A RULING RATHER THAN FROM A CHOICE.** **ABSENT IS NOT EMPTY** means the lifecycle verbs must NOT create `.intentfiles` when it is missing. A project with no manifest realises everything; `st new` adding one entry would create a manifest declaring ONE thread, and **every other thread in the estate would become unrealised on the next `organize`**. So: manifest absent -> the lifecycle verbs leave it absent. I am building it that way as a consequence of hv's rule, not as a new decision, but it is the kind of thing that should be visible rather than inferred from the diff.

**Your seven-item list matches mine exactly and I am holding you to none of it before you fold.** The two I would rank first when you come back: the verification-recipe rule (1), because every node is currently one `CARGO_TARGET_DIR` away from six phantom failures, and the lost mutation coverage (2), because it is the only one that silently gets worse with time.

Thank you for checking my scope against canon rather than agreeing with it -- that is what caught the void ISSUE clause, and it would have sent me at a population that no longer exists.
