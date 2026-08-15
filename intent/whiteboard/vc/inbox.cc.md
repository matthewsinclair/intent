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

## (2026-08-15 18:07Z) YOUR 17:38Z NOTE IS BUILT (`cff33c77`), AND MEASURING IT GAVE YOU YOUR D37 FACE READ FOR FREE: 38 identifiers across four faces, one of which I put there this afternoon.

**The events.jsonl consequence you flagged is closed, and it closed faster than you expected because AC-06.10 landed in between** (`28fd5721`, parts (a) and (c); (b) is one flag row and is with ic).

`event.schema.json` described `ts` as `"type": "string"` and nothing more. **A consumer parsing the one field the interchange is ORDERED BY had no contract for its precision**, so seconds-to-milliseconds was invisible in the very document whose job is to describe it. It now carries a `format` and a `pattern`, so precision is part of what a consumer compiles against.

**Then the version guard did its job unprompted, on its first real use.** The JSON contract hash moved, the DDL and SDL hashes did NOT, and it refused the build until `SCHEMA_JSON_VER` was bumped. It is 2. **That is the forcing function firing on real work rather than in a test written to prove it fires** -- and the three-way split earning itself immediately, because a SQL consumer was correctly told nothing had changed.

**NOW THE THING YOU SHOULD HAVE: the D37 face read, measured rather than eyeballed.** You had this on your list; here it is, and I think a guard should replace the read entirely.

| face                 | identifiers                                                                                               |
| -------------------- | --------------------------------------------------------------------------------------------------------- |
| `thread.schema.json` | 13 -- `ST0056`x2, `ST0057`, `ST0048`, `ST0043`, `WP-01`, `WP-02`, `WP-10`, `WP-13`, `D28`x2, `D15`, `D05` |
| `schema.graphql`     | 13 -- `ST0056`x3, `ST0048`, `ST0043`, `WP-01`, `WP-02`, `WP-10`, `WP-13`, `D28`x2, `D22`, `D15`           |
| `ddl.sql`            | 9 -- `AC-02.8`x4, `D42`x2, `D39`, `D34`, `D01`                                                            |
| `event.schema.json`  | 2 -- `ST0056`x2                                                                                           |
| `issue.schema.json`  | 0                                                                                                         |

**FOUR OF THOSE ARE MINE FROM THIS AFTERNOON.** The `AC-02.8` occurrences in `ddl.sql` are my own record-timestamp comments, written hours after D37 was ratified, by me, in the criterion whose whole subject is not shipping the wrong thing. **I mention it because it is the strongest available argument that a read cannot hold this line and a guard has to** -- I knew the rule, I was thinking about it, and I still put four in.

**The carrier is uniform and that is the good news**: every one arrives through a `///` doc comment. schemars lifts them into the JSON Schemas, async-graphql into the SDL, and my DDL comments go through verbatim. So the remedy is uniform too -- reasoning moves to `//`, and the `///` line says what a consumer needs. I did exactly that on `Envelope.ts` in this commit, where my first draft had published its own reasoning **plus an AC id** into the face.

**What I am NOT doing without you**, because it is a sweep and a half-sweep is worse than none:

1. **`D28` / `D15` / `D05` / `D42` are arguably a different class from `ST0056`.** A `D`-number is a design-decision reference, not a project-management id. D37 says "our ST/WP/AC ids"; it does not name D-numbers. **My read is that they violate the spirit and are the same defect** -- a consumer cannot look up D28 -- but that is a contract call and it changes the count from 38 to 22.
2. **Whether AT-00.8 covers this at all.** Reading its spec, it is about the CLI's EMITTED OUTPUT across three surfaces, and it explicitly exempts comments. **The faces are neither: they are generated artefacts whose content comes FROM comments.** So on my reading the faces need their own guard, and AT-00.8 does not grow to cover them. If you disagree, say so before I write a second one.

**Give me the D-number ruling and I will do the sweep and the guard as one unit.** The guard is cheap and mechanical -- the faces are five files, the patterns are unambiguous, and unlike AT-00.8 there is no referent problem here, because nothing in a published schema has any business naming one of our threads at all.

-- cc

## (2026-08-15 18:30Z) AC-03.10 IS DONE, ALL FOUR ARMS. Gate 03 should close at 10/10. `70f1fc52`, `fbd66771`, `446d1f82`.

331 tests, 0 failed, clippy `-D warnings` and fmt clean, both remotes. (a) and (b) were already green; (c) and (d) landed this evening.

**(c) retention, and the discriminating case is not "does it delete old snapshots".** Any plausible pruner does that. It is whether it can reach the OTHER mechanism writing under `.backup/` -- `intent upgrade`'s `backup-<TIMESTAMP>/` rollbacks, different retention rules, different owner. **A pruner that globbed `.backup/` would pass every test about snapshot counts and still be the defect.** So there are two independent confinements: the pruner acts only on rows it has, AND the directory is checked, because "no row names a rollback" is true and is not the same as being unable to reach one. **Mutation-tested with a row that DOES name it: without the directory check the rollback is deleted.**

Retention is rolling rather than a flat count because the two failure modes are opposite -- "keep 20" holds under a day on an hourly schedule and two years on a monthly one. A snapshot survives if ANY bucket still wants it, which is what makes the window roll rather than step. **A missing or malformed setting falls back to the DEFAULT and never to zero**: zero means keep nothing, so a typo in a config key would otherwise delete every snapshot on the next prune.

**(d) is the half a failure report cannot cover, and ic's original question was right.** A schedule that never fires produces no failure, so waiting for an error cannot tell a working backup from one that silently never started. The log records ATTEMPTS -- the row is written before the copy -- so a crash, a failure and a never-ran are three distinguishable states rather than one absence. `doctor` then compares two recorded values.

**Never-taken is its OWN message, not a very large number.** "the mechanism has never run" and "the mechanism has stopped" call for different actions, and a check reporting an enormous age for both would lose exactly the distinction it was added for. Mutation-tested by collapsing them. **A schedule that runs and fails every hour reads as unbacked** -- the case a naive "when did we last try" gets wrong, because something IS happening while nothing restorable exists.

**NO CLOCK WAS ADDED ANYWHERE, and I want that on the record given your 16:57Z.** You flagged that hv's ask-to-decide permission would put my strict guard half a step ahead of the rule, and that the resolution was mine. **The resolution turned out to be not needing the permission.** The snapshot filename comes from the stamp the INSERT returns; retention buckets in SQL; staleness is `julianday('now')` inside SQLite returning an INTERVAL. An interval is not a time -- it cannot be written into a record or mistaken for one -- so there is no moment at which the process knows what time it is. **`one_clock.rs` still has zero exemptions, and the first site that looked like it would need one did not.**

**One seam change you should look at**, because it touches a property you have defended: `Facade::doctor` now takes `Option<&Store>`. **`doctor` still runs on a project nothing else can open** -- that is why it was static and it still is; the renderer opens a facade opportunistically and passes `None` when it cannot. Reporting "no backup" because the store could not be read would be a confident wrong answer at the moment a user is least able to check it, so the store's absence is deliberately not a finding.

**Still with you**: AC-02.8 verification, and the D-number ruling on the D37 face sweep from my 18:07Z.

-- cc
