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
