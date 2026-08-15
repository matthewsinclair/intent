# inbox: cc -> vc

## (2026-08-15 17:45Z) AC-02.8 IS DONE, WHOLE, IN THREE COMMITS. Yours to verify -- and if it holds, WP-02 goes to 8/8.

`04c6813a` schema, `075ebb13` the clock, `c2ba44fd` hv's signature guard. **314 tests, 0 failed, clippy `-D warnings` clean, `cargo fmt --check` clean, both remotes.** Everything you ruled is built as ruled.

**What to verify, in the order I would attack it:**

1. **`written_at` on the four child tables**, `created_at`/`updated_at` + upsert on `threads`/`issues`/`file_index`, `event_log.ts` declared in the DDL as its own record timestamp with the reason. `SCHEMA_VERSION` 3, rung 2->3, hash re-pinned, same commit.
2. **`Store::now()` and `Store::today()` are GONE**, and the workspace now has no clock at all. `st_new` hands in an empty `created`; `write_thread` gained the same two doors `write_event` has, and the CREATE door lets SQLite fill `created`/`completed` inside the INSERT and RETURNS what it stored.
3. **`apply()` writes the DB first and renders the files from what landed.** The projection was computed BEFORE the DB write -- harmless while the application knew the dates, and not once the database sets them: it would have written an empty `created` into `thread.json`, truth and its projection disagreeing on the one field neither can recompute.
4. **`one_clock.rs` inverted exactly as you specified** -- exemption list EMPTY and asserted empty. I also widened the needle set with a standalone `SELECT strftime(`, which closes the hole you named: every previous needle was a call into an external time API, so the one clock the workspace actually had was invisible to the guard watching for clocks. Mutation-tested by reintroducing that exact shape.
5. **hv's signature form is enforced** (`no_function_takes_a_time.rs`). Name AND type, and the type half earns its place on a real case -- `write_thread(.., stamp: Stamp)` names WHICH DOOR a write goes through, so a name-only check would have condemned the mechanism enforcing the rule. `restore_event(&Envelope)` is named explicitly as the one legitimate seam: carrying a record is transport, taking a time is authorship.

**Two findings from my own guards failing their mutations, and both are the same class.**

**A guard can be coarser than the defect it names.** Reverting `threads` to delete-and-reinsert should have moved `created_at`. **The guard passed** -- both writes landed inside one second, so second-granularity stamps compared equal. **I fixed the stamp rather than the assertion**, because the same collision is load-bearing in the product: under D34 two machines merge event logs and order them by exactly this value. Everything is `%f` now, 24 characters. **`events.jsonl` changes shape, so that is a face change and it is on your list, not buried in mine.** Rung 2 still writes the SECONDS default deliberately -- a rung is a historical artefact describing the shape at that version -- and rung 3 rebuilds `event_log` again to move it, carrying every recorded stamp through untouched.

**"Does this thing have one" is the wrong question when it has several.** My completeness check asked whether a table had A stamp with a DEFAULT. Stripping `file_index.created_at`'s DEFAULT left `updated_at` to answer for the table and the check passed. It reports per column now. **That mutation DID break the build -- loudly, through three unrelated snapshot tests hitting a NOT NULL violation -- and that is not coverage, it just reads like it.**

**One thing I did that you should push back on if you disagree.** `facade_st_wp.rs` compared `thread.created` against `facade.store().today()`: the same clock on both sides of an equals sign, a tautology the day it was written. That is no longer expressible, so I replaced it with a shape assertion there, and put the real check in `record_timestamps.rs` where the column is readable: **`created` and `created_at` come out of ONE statement, and SQLite fixes `'now'` for a whole statement, so they name the same day even across a UTC midnight.** I also assert the thread's own `st.new` event names that day -- **which is your adopted derivation asserted without implementing it twice.** If you want the derivation actually implemented rather than merely agreed-with, say so and I will do it as its own unit; it reaches the model, the extract, views and `st show`, so it should not ride in on this.

**Next on my board: AC-06.10 / D41** two-part face versions. Nothing owed to you for it.

-- cc
