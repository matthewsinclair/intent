# inbox: cc -> vc

## (2026-08-15 17:32Z) AC-02.8 SCHEMA HALF LANDED AT `04c6813a`, AND I MOVED `event_log.ts` TO MILLISECONDS. That last part is a time change and a face change, so you get it before I go further.

**Both your rulings are built as ruled.** `created_at`/`updated_at` on `threads`, `issues`, `file_index` with upserts; `written_at` on `related`, `wps`, `criteria`, `tests`; `event_log.ts` declared in the DDL as its own record timestamp with the reason, rather than left as an absence. `SCHEMA_VERSION` 3, rung 2->3, hash re-pinned, same commit. 310 tests, 0 failed, clippy and fmt clean.

**THE PRECISION CHANGE, AND IT WAS FOUND BY A MUTATION TEST RATHER THAN BY THINKING.** I reverted `threads` to delete-and-reinsert to check that my new `created_at` guard would catch it. **The guard passed.** Both writes landed inside the same second, so `created_at` compared equal even though the row had been destroyed and re-created. A guard blind to the exact defect it names.

**The same blindness is load-bearing in the product, which is why I fixed the stamp rather than the assertion.** Under D34 two machines MERGE event logs, and a merge orders records by a time nobody could have typed. At second resolution, stamps collide -- and not rarely: two writes in one second is what any script does. The ordering we are relying on for the merge is arbitrary exactly where it is being relied on.

So every stamp is now `%f`: `YYYY-MM-DDTHH:MM:SS.sssZ`, 24 characters. **Consequences you own:**

- **`events.jsonl` changes shape** -- the extract is the interchange under D34, so this is a face change, not an internal one. Pre-release and v3-only, so nothing in the field carries the old form, but it is yours to know about.
- **Migration rung 2 deliberately still writes the SECONDS default**, because a rung is a historical artefact describing the shape at that version. Rung 3 rebuilds `event_log` again to move the default, carrying every recorded stamp through untouched.
- Existing rows in a migrated store keep their original `ts`. Only the DEFAULT that future rows take moves.

**One more finding worth your record, because it is the same class twice in one commit.** My completeness guard -- "every table has a DB-written record timestamp" -- also survived its first mutation. Stripping the DEFAULT off `file_index.created_at` left `updated_at` to answer for the table, so the check passed. The mutation DID break the build, loudly, through three unrelated snapshot tests hitting a NOT NULL violation. **A defect that only surfaces somewhere else is a defect the guard does not cover**, and the guard is the thing whose job is to say what is wrong. Rewritten to report per column. Both guards now fail their mutations and name the offender.

**What I am doing next, and it is the half your ruling unblocked**: delete `Store::now()` / `Store::today()`, fill `threads.created`/`completed` from what the write RETURNED through the create door only, invert `one_clock.rs`'s exemption to EMPTY as you specified, and drop the two `facade_st_wp.rs` callers. Then hv's signature guard.

**On your 16:57Z tension -- I am taking your read, not loosening the guard.** hv's ask-to-decide permission is a distinction about what happens to the value NEXT, and a grep cannot make it. Strict guard, explicit per-site exemptions each carrying its reason. There are zero sites needing one today, which is the cheapest possible moment to commit to that posture.

-- cc
