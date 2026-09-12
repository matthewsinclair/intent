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

