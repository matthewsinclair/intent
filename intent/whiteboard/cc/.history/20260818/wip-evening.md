## THE FLAT VIEW -- `d8412be`, and it was not a glyph bug

**`intent/todo.md` rendered SIX states as ONE.** `views.rs::items()` emitted a literal `- [ ]` for every row of every bucket. Because the bucketing is three-valued and `ThreadStatus` is six-valued, **`Completed` and `Cancelled` share the DONE bucket -- so the glyph was the ONLY thing distinguishing them**, and 2 of the 54 rows under `## DONE` were cancelled work presented as completed (ST0010, ST0015).

**THE CUT WAS AT `TodoItem`, NOT AT THE RENDERER** (ic's widening, which I found the second half of). It carried `{id, kind, title, label}` and no status, so `items()` had nothing to compute from **and `--json` had nothing to report** -- a machine consumer could not tell cancelled from completed either. **Both faces lost the same fact, so both AGREED**, which is precisely what `TodoItem`'s doc comment promises they cannot do. The promise was kept and the fact was gone. **Sweep rule that follows: "the value survives to the point of rendering", not "the site reads it from the model"** -- a read-site sweep passes `items()` clean while a struct two hops upstream is still dropping something.

**MEASURED:** 82 rows / 1 glyph -> 82 rows / 4 glyphs. `[x]` 52, `[~]` 2, `[ ]` 17, `[-]` 11; 52 + 2 = 54 is the DONE count. 65 rows changed = 82 less the 17 legitimately blank. Nothing else in the file moved. **Triage and Hold render NOWHERE in the live estate**, so those arms exist only under test -- the ones that rot.

**NO DATA REPAIRED** (ic). ST0010/ST0015 carry `status: Cancelled` correctly; v2 held cancellation TWICE, as a `CANCELLED/` directory AND the field, and the hoist correctly flattened it -- **which left the field as the sole carrier, so a view discarding it was TOTAL loss, not cosmetic.**

**NO TEST EXISTED ON THAT VIEW AT ALL** -- no `#[cfg(test)]` in `views.rs`. Not a weak test; nothing looked. Six added, all driving the real renderer over fixtures carrying both states, because a unit test of `glyph()` alone passes while the renderer emits a constant. **`glyph()` is exhaustive with NO wildcard**: v2's `status_box` has `*) printf '?'`, so a status v2 did not know rendered as a shrug and nobody learned. A seventh state must fail to compile.

## AC-04.4 -- IN FLIGHT, red demonstrated then guarded

**`views::write_all` called `fs::write` UNCONDITIONALLY for every view**, so a byte-identical re-emission moved mtime on all 266 (57 info + 57 acceptance + 150 WP + steel_threads + todo) every sync -- and `file_index` derives clean/changed from mtime, so a no-op sync marked the whole estate changed.

**The idempotence test beside it PASSES while this is live**, because it compares bytes. **Idempotent bytes is not idempotent writing.** Test written first and demonstrated RED (10 of 10 views moved), then the guard. **It does not sleep or trust timestamp resolution** -- every view is aged an hour between runs, so a skipped file keeps the aged stamp; a clock-racing test would pass vacuously on a coarse filesystem, which is the exact failure it exists to detect.

## `status_reason` -- a field four verbs DEMAND and no human face showed

**Found by ic sweeping `views.rs` under the widened rule I gave them**, which is the rule that found it: _the value survives to the point of rendering_, not _the site reads it from the model_. There was no view site to score -- `status_reason` survives into the model, into canon, into the DB and into the committed SDL, and dies at the render boundary.

**`st cancel` / `st hold` / `st reopen` / `wp reopen` REFUSE without a reason, and the refusal argued for the field while hiding it**: _"...and in the event log as part of the decision, which is what lets anyone reconstruct why later."_ I went to check the event log expecting the promise to rest there -- **`intent --help` declares 34 verbs and not one reads it.** `search` does not reach it either (`ingest.rs` never mentions the field). **A promise with no reader on EITHER carrier.**

**ic's narrowing is the honest claim and I use it everywhere: MACHINE-VISIBLE, HUMAN-INVISIBLE.** `schema.graphql:292` (Thread) and `:413` (WorkPackage) expose `statusReason`. The wider "the value is lost" dies on "it is in the schema"; the narrow one survives.

**FIXED on all four faces** -- `views.rs` thread + WP frontmatter, `render.rs` `st show` + `wp show` -- **and the frontmatter key is emitted only when there IS a reason**, so nothing in the live estate churns today. vc RULED the false clause struck without waiting for hv; **a reader for the event log is hv's**, and ic's point makes it cheap: `event.schema.json` is committed, so it is **a built carrier with no door**, not an unbuilt feature.

**This does NOT close AC-03.12 and I am not claiming it** (vc). The field carries only the CURRENT status's reason -- any transition without one clears it -- so rendering it answers _why is it on hold now_ and never _reconstruct why later_. **Fixing the visible half would make the promise look kept.**

**ic's CONTROL ORDERING is now my habit, and it is in both e2e tests: prove the value reached canon, THEN ask the face.** Their first WP drive hit an unfired fixture -- the gate refused the `wp done`, so `wp reopen` returned `ok: already WIP` writing nothing, and every face came back empty. **Emptiness from a face that does not render and emptiness from a verb that never recorded are indistinguishable when you only ask the face.**

## D42 CAUGHT ME, IN A TEST ABOUT MEASUREMENT DISCIPLINE

My AC-04.4 test aged the views with `SystemTime::now() - 3600s`. **`one_clock.rs` failed the suite naming my file** -- there is no clock in this workspace at all. Replaced with a FIXED synthetic stamp (`UNIX_EPOCH + 1_000_000_000s`), which is **stronger for obeying the rule**: the assertion became "still exactly this constant" rather than "still roughly where I put it". **Then re-ran the mutation test, because changing the mechanism invalidates the earlier RED.** 10 of 10 red with the guard off; green with it on. The guard caught it, not the author -- which is the case for structural guards over careful authors.

## OPEN -- four things queued, none started
