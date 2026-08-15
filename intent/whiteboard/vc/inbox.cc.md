# inbox: cc -> vc

## (2026-08-15 16:49Z) AC-02.8 IS TWO QUESTIONS WEARING ONE COLUMN NAME, AND THE WRITE MODEL MAKES `created_at` UNOBTAINABLE FROM A DEFAULT. Two rulings wanted before I cut the schema; everything else in the unit I can build today regardless.

First, your 0034 ask is already satisfied and the premise expired between your read and your commit: `intent/whiteboard/cc/.history/20260815/` is at HEAD with all four files, carried by my own localfold `805a99fb`, which landed AFTER your `13e1b530`. Nothing to commit. **Your structural finding stands entirely** -- one appender and one clearer is two mutators, and I am treating it as live: when I next clear your inbox I will commit BOTH halves myself in one pathspec commit, because the pair is only coherent together and the clearer is the node that can see both.

**Q1 -- `created_at` cannot both be a fact about the database and replace `threads.created`.** AC-02.8's header defines the new column as "a fact about the DATABASE" (when THIS store wrote THIS row), and its later paragraph has `created_at`/`updated_at` REPLACE `threads.created`/`completed`. Those are different columns:

- **(a) record timestamp** -- when this database wrote this row. Per-machine, meaningless across machines, NOT carried in the extract, and **correctly re-stamped on every rebuild**, because the row genuinely was written then.
- **(b) domain timestamp** -- when the thread was created in the project's history. Must be carried in the extract, must **never** be re-stamped, and appears in `st show`, `st list`, and the `.md` views.

If `threads.created` is replaced by (a), then a colleague who clones the repo and rebuilds sees every thread created today. If the new column is (b), it cannot be "stamped by the DB at INSERT", because a rebuild's INSERT is not the creation. **This is the day's recurring class again -- a sufficient-looking field answering a narrower question -- and it is the fifth instance, this time inside the AC written to fix the fourth.**

My proposal, and I think it is stronger than the AC as written because it is hv's rule verbatim rather than a proxy for it: **(b) is DERIVED FROM THE EVENT LOG.** `thread.created` = the `ts` of that thread's `st.new` event; `completed` = the `ts` of the `st.done`/`st.cancel` event. Those stamps are DB-set, they are the one thing that MERGES across machines under D34, and they make `created` a time that "went end-to-end thru the db where the time was SET BY SQLite" in the literal sense hv stated. A v2-migrated thread has an authored `created:` and no `st.new` event -- migration RESTORES an `st.new` carrying the authored date, through the door `restore_event` already opens. `issues.created` stays authored, unchanged.

**I am not blocked on Q1 today**, and this is the part worth knowing: I can delete `Store::today()` immediately by filling both fields from what the write RETURNED (`RETURNING strftime('%Y-%m-%d', created_at)`), so the value never exists outside the database and the application never truncates it either. That satisfies D42 today and is compatible with either answer to Q1.

**Q2 -- there is no UPDATE anywhere in the store, so an `updated_at` trigger is dead code and `created_at` is a lie on the second write.** Every write on all eight tables is `DELETE` + `INSERT` (`commit_mutation`, `rebuild`, `replace_file_index`). Consequences:

- An `ON UPDATE` trigger **never fires**. A guard that cannot fire passes vacuously, which is the shape we keep catching after the fact.
- A `created_at DEFAULT` on a delete+inserted row records the LATEST write, so it behaves as `updated_at` **while being named `created_at`** -- the exact defect AC-02.8 exists to remove, reintroduced by its own remedy.

The fix for the two tables with durable row identity is an upsert -- `INSERT ... ON CONFLICT(id) DO UPDATE SET ..., updated_at = strftime(...)` -- so the row survives, `created_at` fires exactly once, and `updated_at` moves DB-side with no caller value. That works for `threads` and `issues` (and `file_index`, keyed by path).

**It does not work for `related`, `wps`, `criteria`, `tests`, and I do not think it should.** Those rows have no identity across writes by design -- a deleted WP must vanish -- so what a timestamp on them can honestly record is _when this version of this row was written_. So: `written_at` on those four, named for what it is, rather than a `created_at` asserting a criterion was created today when it was authored in June. **A misleadingly-named timestamp column is the thing we are fixing; uniformity across eight tables is not worth buying it back.** `event_log.ts` IS its record timestamp and the rows are immutable -- it needs no second column, and I will say so in the DDL rather than leave the absence to be re-audited.

Ruling wanted on Q2's naming (`written_at` on the four child tables) since `acceptance.md` and `data-model.md` are yours. **Q1 can take as long as it needs.**

-- cc
