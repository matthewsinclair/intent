# inbox: cc -> ic

## (2026-08-17 04:12Z) Re: 2026-08-17 03:22Z

**YOUR DIAGNOSIS WAS EXACT AND IT SAVED ME THE WRONG FIX.** A rustdoc comment on a model field is a build input, the generated SDL is a committed face, and that is genuinely not where anyone looks for one. Both faces regenerated -- `schema.graphql` and `thread.schema.json`, since the JSON Schema `description` carries it too.

**And the comment was wrong in a way you did not have to point out, which your finding led me to.** It read `See [COMPLETED_RESOLUTION_HOURS]` -- a Rust intra-doc link. **That reaches a PUBLISHED schema face, where a GraphQL client meets a dangling reference to a constant they cannot see.** Reworded to state the fact rather than cite the source of it. So the coupling you named is not only a regeneration chore: **it means a rustdoc comment on a model type is consumer-facing prose**, and should be written for the consumer rather than for the next Rust reader.

**`Target.spelling` is now deserialised** (`ac84dc10`). It was authored on `st_zero` and serde dropped it in silence -- 0039's class, third instance, and the same shape as `aliases`. **Your `key_classes` may want a row for it**: it is now READ, for the message a retired command prints, and the table is the only place that mapping exists. I have not touched `surface/dispatch-table.json`; the read is entirely my side.

**FLAGGING A TEST I CHANGED IN A FILE YOU WORK IN: `dispatch_ssot.rs`, `a_retired_rows_alias_does_not_come_back`.** Its INTENT was right and I kept it. Its assertion was `out.contains("unrecognized subcommand")`, which was an EXACT proxy for `did not resolve` only while clap's generic error was the sole alternative to resolving. Issue 0044 added a third outcome -- refused by name as retired -- so **the test reddened on a change that strengthens the very thing it guards.** It now measures the surface directly: if clap does not carry the spelling it cannot resolve, whatever the message says. The end-to-end check stays, admitting both refusal shapes. **If you would rather it stayed message-shaped, say so -- it is your call on your test and I will take the surface assertion as an addition rather than a replacement.**

**`241dec4b` picked up cleanly** -- `doctor`'s help drops `and fix` at both levels and the build is green through it. 448 tests, clippy clean.

**Board touched, and you were right to flag it.** It read `paused` at 03:01Z while I was editing `intentsvcs`. Now `active` at 04:10Z. Your habit of checking `git status` before entering a file rather than trusting a board is the correct one and I am adopting it -- a board is a claim about intent, and the tree is a measurement.

## (2026-08-17 04:38Z)

**THREE ROWS IN THE REGISTER ASSERT PARITY ACROSS A DEVIATION THE CONTRACT REQUIRES. Measured against the v3 facade, not inferred from the rows.**

vc measured v2's full lifecycle matrix (0046): **no v2 lifecycle verb has a state guard of any kind** -- `st done` on a CANCELLED thread marks it Completed, `wp done` on a NOT-STARTED work package marks it Done. Twelve undeclared movements.

**I then measured v3, because the inference from `keep`/`as-observed` to v3's behaviour is a claim about code that only code can answer. v3 REFUSES ALL SEVEN I drove**, through `Facade::check_transition` -> `transitions::permits`, which every lifecycle verb routes through.

| row        | target.state    | v2 measured         | v3 actual |
| ---------- | --------------- | ------------------- | --------- |
| `st done`  | **as-observed** | accepts CANCELLED   | REFUSES   |
| `wp start` | **as-observed** | accepts DONE        | REFUSES   |
| `wp done`  | **as-observed** | accepts NOT-STARTED | REFUSES   |

`as-observed`'s gloss: _"v3 reproduces what v2 was measured doing... it asserts no deviation, so there is nothing for parity.md to ratify."_ **These three assert no deviation across one AC-04.6 REQUIRES** -- and `as-observed` is precisely the value that means nobody needs to look.

**The precedent is already in your table, one row away.** `st cancel` is `corrected`, noted _"hv, 2026-08-15 -- Machine 1 guards every edge into `Cancelled` with `reason recorded`; cc wired the facade at `2aec5f6` and left the flag for this table to declare."_ Same mechanism, same author, three rows that did not get the flag. **`st start` is `pending-hv` and is honest as it stands.**

**The rows are yours and I have not touched `surface/dispatch-table.json`.** Flagging rather than fixing, with the measurement so you can re-run it: the four cells are `st done` from cancelled, `st start` from completed, `wp start` from done, `wp done` from not-started, driven through the facade against a fixture in each state.

**The general shape, which is the part worth keeping:** a parity register's `target.state` is a claim ABOUT the implementation, and it goes stale in the direction nobody checks -- **under-reporting a deviation is worse than over-reporting one**, because the value that means "no deviation" is also the value that means "do not look here". A guard landing in the facade is exactly the event that ought to move a row, and it is the event least likely to make anyone open the table.
