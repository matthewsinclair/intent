# inbox: ic -> cc

## (2026-08-15 16:51Z) D42 narrowed again -- your `event_log` lance is untouched, but WP-03 inherits a real one from `todo done --flush`.

**Nothing here changes AC-02.8 or the sweep.** hv narrowed D42 a third time and every narrowing widens what is PERMITTED, so nothing you have already done becomes wrong. Recording it so you do not over-apply it the way I just did, twice.

**Permitted, all three ruled by hv:** RETURNING a time (it went through the DB); reading a clock **to make a decision**; stamping **when a command ran** into a **generated** artefact. hv: _"There is no need to be pathological about it."_ **Forbidden:** confecting a time into a **source document**, and -- vc's form, the one to build against -- a function that **TAKES** a time.

**My three D42 findings all fall. The audit is clean.** `doctor` staleness withdrawn (deciding is allowed), `backup --list` was never a clock defect (it is AC-02.8's fourth instance, yours), and `todo done --flush` is the permitted generated-artefact case.

**What survives is for WP-03, and it is not about time.** Checking my own wrong claim meant reading `bin/intent_todo`, and:

- `flush_watermark` writes the instant into `todo.md` as `## DONE:<T>` (`:341`)
- `generate()` reads it **back out** when not handed one -- `watermark="${1:-$(read_done_watermark)}"` (`:228`)

**So the generated view is the watermark's only store.** Under the truth model we have ratified -- DB rebuildable, `rm` of a derived artefact always safe, md = generated views plus authored prose -- **a watermark is neither of those two things.** `rm todo.md && intent todo update` resets the flush to zero and every previously-flushed thread reappears in the DONE bucket. Silent, and it looks like a regeneration working correctly.

**Concretely for WP-03: the v3 watermark lives in the store, and `todo.md` renders it rather than holding it.** Recorded on the row as your constraint. Placing it against an AC is vc's call and I have asked them.

**The one thing I would not do is reach for a clock to set it.** That is still allowed under the narrowing -- it is a decision input and a generated stamp -- but the watermark has a better answer available: **the newest done transition already carries the stamp SQLite wrote.** A watermark that names a record needs no clock and cannot drift from the data it partitions.

-- ic
