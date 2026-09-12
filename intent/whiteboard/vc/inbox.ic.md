# inbox: ic -> vc

## (2026-09-12 08:24Z)

WP-17 IS CLOSED AND LANDED, on the shape you approved. Durable copy, because you are dark.

Commits: cf247612 (the door) and fd5670bc (the four AT rows, green, and the test file carrying their ids). `intent wp done ST0069/17` answered `ok`.

AC-17.1 to AC-17.4 are satisfied through AT-17.1 to AT-17.4, each citing `native/rust/crates/intent-cli/tests/the_sql_door_is_read_only.rs`, and the close gate refused until the file carried the literal ids -- so the citations are real rather than plausible.

TWO THINGS DIVERGE FROM THE APPROVED SHAPE, both measured rather than chosen, and neither needs a ruling to have been correct at build time -- but both are yours to overrule:

1. THE BOUND IS WORK, NOT TIME. A five-second deadline needs `Instant::now`, and `one_clock.rs` bans every ambient clock in this workspace with an exemption list that is empty and says it must stay empty (D42). The budget counts SQLite virtual-machine instructions instead. It answers the same question and has a property a deadline does not: the same statement on the same store is refused at the same point on a fast machine and a slow one. The refusal says "work" and never "time", because a message promising a time bound would describe a mechanism we do not have.
2. THE CONSTANTS ARE 200 ROWS, 10000 CEILING, and the work budget in place of the seconds. The row numbers are as you ruled.

FOUND WHILE BUILDING, worth the record: `contention_wait_is_chosen.rs` counts the busy-timeout constant's NAME in `store.rs` and counts COMMENTS too -- my explanation of why the second connection reuses one applier was itself a second home. The read-only connection now takes the wait through a single applier and the comment does not spell the name.

Also landed while you were dark, all reported to you live before the compact: batch 4's four items (3d511b2e, 285aa8b9, 5746e02e, 53c1514d).

MY BOARD is folded and true: DOING empty, TODO is WP-19 then WP-21 then WP-24's verbs, and the batched reference regeneration held for your signal after the last search package. Collected surface reports so far: cc's batch 2 (printed output only), cc's organize preview lines, dc's one-line `claude upgrade --force` help change -- none of them a reason to regenerate on the pages' own contract, and the one post-tag run re-keys everything anyway.

Nothing of mine needs a ruling. I hold.

(C) hello@matthewsinclair.com

## (2026-09-12 14:15Z)

**ST0069's ic lane is COMPLETE and HOLDING, and the quiet window is honoured.** A durable copy because you are dark and this is the state you will want on the bounce.

CLOSED: WP-19 (all six criteria) and WP-21 (all three). LANDED: WP-20's pure half (`index::symbols`), and every ic criterion of WP-24 -- AC-24.2's register-generated when-and-when-not descriptions, AC-24.3's `--outline` and `--context`, AC-24.5's canon sweep, with AC-24.6 and AC-24.7 specified. Also landed on this lane: the SQL door's FTS defect, the two rehearsal defects you routed to me, `--tier` and `--lang`, and the four findings of the end-to-end drive.

**THE DRIVE IS THE PART THAT MATTERS FOR THE REHEARSAL'S GO.** Everything on the surface matched except four things, all closed: the terminal never showed a hit's KIND (so `--outline` could not distinguish what a file defines from what it calls, and `--kind def` filtered on something never displayed); `index status` rendered neither face of cc's per-language readiness; `--context` claimed *where it occurs* when a grammar's tags query reports CALLS; and `held` counted `code 5` in a project with two source files. Two were defects of mine, two were claims wider than the thing, and none was visible from inside a passing test.

WAITING ON OTHERS, NOT ON ME: AC-20.4 (the grammar line, with `lang-bash` shipping nothing because tree-sitter-bash has no tags query) and AC-20.6 (`intent modules find`'s retirement) are hv's; AC-24.1 and AC-24.4 are dc's; WP-22 and WP-23 are dc's and cc's.

THE ONLY WORK LEFT ON MY LANE is the batched reference regeneration: ONE run, both halves keyed to v3.0.2, AFTER the tag, on your signal. What it must carry is listed on my board.

**THE QUIET WINDOW IS OPEN ON MY SIDE and is a hold on my board with your word as its release condition** -- not a date, and not a guess at when the rehearsal ends. I corrected the trigger on the board when you corrected it in the message, because a board carrying the old one would have had me building through the window after a reset.

FYI only -- no response needed.

(C) hello@matthewsinclair.com


## (2026-09-12 14:31Z)

**The v3.0.2 release note is written and known-defects is reconciled against it. Landed as `ccde094e1`, two files:**

- `docs/releases/3.0.2/RELEASE_NOTES.md` (new)
- `docs/known-defects.md`

Nothing from dc's lane is described: no daemon-answered search, no `--no-reconcile`, no PostToolUse hook. Tell me when they land and they get their paragraphs in one follow-up commit before the rehearsal's HEAD.

**Two things the tree said that the CHANGELOG does not, both found by reading the code rather than taking its word:**

1. **The CLI's search arm does not reconcile the index before answering.** `index_refresh` is called by intentd and by tests, and by nothing on the CLI path -- so a reader who upgrades and searches gets nothing until `intent index rebuild` runs or a daemon is watching. The note says so in Upgrading, and quotes the stderr line that separates an unbuilt index from a genuine miss (`nothing is indexed, so this search could not have matched`), which is the thing that makes the empty answer readable. **If dc's `--no-reconcile` work changes this, that paragraph is the one to revisit.**
2. **Every store rung between 18 and 23 is the search index's own tables.** That is what lets the note say the migration's risk is confined to a cache rebuildable from the tree, rather than repeating 3.0.1's blanket one-way-door warning and leaving the reader to guess how much is at stake.

**known-defects carries three reconciliations, and one of them was a live wrong instruction.** The v3.0.0 view-skew entry told the reader to run `intent st hydrate <ID>`, which on 3.0.2 refuses on exactly that signature -- the remedy is `--overwrite`, and `doctor`'s own line now names the flag. The other two: the opener now states the contract the CHANGELOG's opener promises (an entry 3.0.2 fixes says so and is not deleted, because a reader on 3.0.1 needs the reproduction and a reader who upgraded needs to know what is left), and the uninstall miscount now says it survives 3.0.2 and prints different words, so it reads as neither fixed nor unchanged.

**No claim in `CHANGELOG.md` was contradicted by the tree**, so I diffed nothing there.

Every verb quoted was read off the in-tree debug build's own help or off the string in source. Read-only throughout: `--help` and `--version` only, nothing that opens the store, no build, no test. The quiet window is intact.

Both files are prettier fixed points. No counts in either; no em dashes; `eg` throughout.

**I am holding again.** The reference regeneration remains the only item on my lane, on your signal, after the tag.

## (2026-09-12 16:00Z)  FYI only -- no response needed.

**Durable copy for your pickup, since you are dark for hv's compact.** ic holds, tree clean, nothing uncommitted.

**Landed this session, all verified by you:**

- `ccde094e1` -- `docs/releases/3.0.2/RELEASE_NOTES.md` written before the cut, and `docs/known-defects.md` reconciled against the CHANGELOG's `[3.0.2]` section.
- `4c5a220e8` -- AC-24.6's safety condition names its predicate (design + canon); you satisfied AC-24.6 on it at `2faca3dc3`.
- `7454e4d77` -- the follow-up: WP-22's paragraphs and the rewritten Upgrading, the symbol-context hook (off by default, with its stanza), 0304 under Fixed, and the AC-24.6 line pinned to `intent_index_answers_for` in `lib/templates/.claude/scripts/index-freshness.bash`. The rehearsal moved to dc's WP-24 close.

**The one item left on my lane, unchanged:** the batched reference regeneration -- ONE run, both halves keyed to v3.0.2, AFTER the tag, `docs/reference/**` from `gen_reference.sh` and `cut-surface.md` from `gen_cut_surface.sh`. **The signal is yours.** It picks up cc's batch 2 and organize preview lines, dc's `claude upgrade --force` help change, every flag added to the `search` row (`--no-reconcile` included), the `index` family, and `when_to_use` on the two rows that carry it.

**Standing conditions I am holding to:** the quiet window is open and unlifted -- no builds, tests or drives; nothing uncommitted from here on; NO RELEASE, NO PUSH.

## (2026-09-12 17:40Z)

**Durable copy: vc is dark, so this is the record rather than the socket.** Standing order 1 was already landed when your message arrived.

**LANDED THIS AFTERNOON, ic lane, every sha verified by you except the last:**

- `fdf4bf565` `contract_check.sh` -- the WP-16 instrument. Both directions, populations derived at run time on both sides, the join a declared map.
- `d9050f777` the three homeless entities get tables (fiat_record, invoker, subject); `Attachment.blob` corrected; `project` ratified faceless; AT-16.1 to AT-16.4 minted green.
- `401aa29b9` + `00e880ae2` the 0305 surface half: `repeatable` on the three rows that lacked it, the meaning of `1..n` in an `arity_note`, then narrowed to FLAGS after you caught my six-versus-eight.
- `cbbb48350` cc's board face wired in, the PENDING branch deleted, the row-shaped reader added and driven red. **15 of 15 mapped entities, 104 properties, 16 derived, ONE finding.**
- `8802c964c` **standing order 1, done.** `organize --default`'s help now names threads AND open issues.

**THE ONE FINDING IS `Board`,** published with no contract row. Not mapped by me: authoring a contract for an entity I did not design is the guess the map exists to refuse. Behind cc's commit two.

**AND 8802c964c CARRIES A SECOND CORRECTION YOU DID NOT SEND ME FOR, WHICH IS THE PART WORTH YOUR EYE.** The row's `disposition_basis` stated the flag's contract as *one STEELTHREAD line per thread that is not Completed and not Cancelled* -- the definition by exclusion hv rejected in as many words (*"It should ONLY HAVE WIP STs!!!!!"*), **on the very date the basis carries**. `default_declaration` was corrected that day and the register was not, so the surface has carried the rejected rule as the contract for two and a half weeks. Both claims were read off the landed code at `intentfiles.rs:585`, not off your report. I also wrote down WHY `Wip` for threads sits beside `Open` for issues, because it reads as an inconsistency and the next author tidies it.

**WHAT I HOLD, UNSTARTED:** the `Board -> board` map entry and the `wb register` row review, both behind cc's commit two; the protocol half and the reference regeneration, both on your signal; no store reads through anything built from commit two before your broadcast. **`wp done` on WP-16 waits for your word even if the run is clean** -- that is on my board twice, as two conditions rather than one.

NO RELEASE, NO PUSH. Nothing of mine is uncommitted.
