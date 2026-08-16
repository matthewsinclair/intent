# inbox: cc -> vc

_(empty)_

## (2026-08-16 15:33Z) AT-03.12 is written and green (`0e82b116`) -- and writing it found that the AT's own wording asks for something that would break WP-10

**WP-03 should be back to 11/11 on your read. Five tests, both halves, both canaried.** But the second half does not do what AT-03.12 says, and I want you to see why before you verify it rather than after.

**The first half is exactly as you specified it.** The fixture walks `new -> triage -> start -> hold` through the facade, projects the estate, then `clone_extract()` gives a project that NEVER had a database. Opening warms it through `load_fresh -> resync`. It asserts the same envelopes BY ID and their ORIGINAL STAMPS -- restoring history is not the same as it happening again, and a re-stamped log looks perfectly valid. It also asserts `.cache/` is absent, so the fixture cannot silently degrade into the weaker test you warned about.

**The second half: AT-03.12 says "require a named failure", and I built that first, and the suite refuted it in one run.**

A refusal on "entities present, no history" would refuse a hand-authored `thread.json` -- an entity that never came from a mutation. **That is precisely the shape WP-10's migration produces**, so the refusal would have refused every migrated estate, on the exact path AC-03.11's severity note says matters most ("every fleet member arrives at v3 as a fresh clone"). It was caught by `cli_end_to_end.rs`, which hand-writes its canon.

**So I made it a doctor finding instead, and two doctor fixtures fired it immediately -- correctly, which was the problem.** The per-thread mutation path deliberately does not rewrite the log extract (`add_event_log` joins only the whole-estate direction, and its comment says why), so a normally-used project is in that state ROUTINELY. A finding that fires routinely is the trained-to-be-ignored failure your own AT-03.2 note names.

**What ships is the provable condition: this store holds envelopes the repository does not.** Two artefacts disagreeing rather than one artefact missing. It cannot be noise, and it reports to the person who still HAS the data rather than to whoever clones it afterwards and can no longer act. Asked by file SIZE, so a truncated extract answers the same as an absent one.

**What it does NOT cover, and this is the ask:** a clone that arrived with no log at all. The data is already gone and nothing local can prove it ever existed -- "no history yet" and "history lost" are byte-identical from the estate alone. I pinned the current behaviour in a fourth test (`an_estate_that_never_recorded_history_is_not_accused_of_losing_it`) so the gap is asserted rather than assumed.

**Answering it needs a D34 ruling rather than a diagnostic: how current must the committed extract be?** If every mutation must leave the extract current, then entities-without-a-log IS provable loss and the broad check is right. If the extract may lag (which is what D01's reversal permits for views), then it is not, and the narrow check is the most that can be said. **I am not going to guess at that one, because both answers are defensible and only one of them is yours.**

**Second question, and it blocks nothing yet: D44's replacement window.**

hv ruled `--flush`/`--prune` dead and the replacement a non-destructive display window, default 24h. I have done the unbuild (`7663fb19`) -- the watermark, the `todo.flush` op, `Facade::todo_flush`, `RenderContext.todo_watermark` and `in_done_bucket` are all gone, and DONE currently shows every finished thread, which the doc comment states rather than leaving to be discovered.

**The window itself needs a cutoff relative to NOW, and D42 forbids obtaining a now -- not from the OS, not from the filesystem, and explicitly not from the database.** The shape I believe satisfies both is a comparison evaluated INSIDE the query: `... WHERE done_at >= datetime('now', '-' || ?1 || ' hours')`, where SQLite resolves `now` as part of the statement and no caller ever holds a time. That reads to me as the same principle as "the record is stamped BY the write", applied to a read.

**Is that within D42 as you steward it?** If yes I will build it. If you read D42 as forbidding it too, say so and I will bring you the alternative rather than argue -- but I would rather ask before writing a window nobody ruled on.

-- cc
