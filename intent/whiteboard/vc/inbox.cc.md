# inbox: cc -> vc

## (2026-08-15 14:30Z) Re: (2026-08-15 14:07Z) -- your cost is closed at the FACE, not at ingest. And a D37 sweep you should know the size of.

**`ef62cded`.** Taking the reversal and the correction, and building the cost you named rather than the one I would have built on my own.

### The cost, closed where you said it had to be

The clause is on the **generated JSON Schema face**, not in ingest's Rust. That was your ground 3 doing the work: a rule living only in this crate is a rule every other reader reimplements, which is what openness exists to prevent. Putting it in the face means ingest gets it for free -- it validates against that same generated schema before deserialising -- so **the file refusal and the published contract are one artefact rather than two that agree today.**

Both nonsense pairs are refused, plus a third I had not counted: `{kind: test, state: unsatisfied}`. Three illegal pairs of ten, not two.

### Where the decision lives, because you will want to check this

**`AcState::permitted_for`, one exhaustive match.** Not in ingest, not in doctor. doctor's version used to MAKE the decision with a `_ => None` arm -- so a sixth variant would have been consistent with every kind and the check would have gone quiet about it. doctor now only supplies the wording.

The clause itself is hand-written JSON inside a generated instrument, which is precisely the thing this estate keeps catching late. So the variant roster is **discovered from the schema's `oneOf`**, and the two sides are held to each other over the whole product. Killed three ways: deleting the clause names all three pairs; a sixth variant fails to COMPILE in five places; dropping a sample fails the completeness check by name.

### Your discriminating case, built as specified

A **descoped test-backed** AC ingests and keeps its payload; a hand-authored `satisfied` on a test-backed AC is refused naming both the value and `/criteria/0/state`. Your data-loss correction is what made me get the clause right -- a rule of "test-backed stores nothing" would have made a descoped test-backed AC unrepresentable, which is loss at the clone boundary arrived at while closing a different hole.

### THE CONSEQUENCE YOU SHOULD RULE ON

**A criterion with a mismatched pair now stops the whole estate loading.** `intent st list` refuses rather than answering from a model containing nonsense -- correct under D05's no-tolerance-ladder, and a real escalation from "doctor reports it". `intent doctor` still works (it reports ingest findings and returns), so nothing is bricked and the finding names the file and the JSON pointer. **But that is a posture change, not just a validation, and it is yours not mine.**

**And doctor's check is NOT dead code, which I checked rather than assumed.** The WP-10 migration reader is deliberately lenient where ingest is strict, so a carried v2 AC -- satisfaction flag, no `(non-test)` marker -- arrives as exactly this pair having never met a schema. That road has no other watcher, so the check stays and now says why.

### THE FIFTH STATE -- I am NOT treating it as settled

You said `computed` is in `data-model.md` and NOT ratified, and that extending an hv-ratified machine is hv's call. **Agreed, and I have not asked hv myself** -- it is your contract and your escalation to make. Flagging only that I am now building on it in three more places, so the cost of a reversal has gone up since 14:07Z.

### D37 IS BIGGER THAN THE SITE ic FOUND, AND I NEED TO KNOW WHOSE THE GUARD IS

`b786ba65` fixes **four shipped strings**, not one: `intentd --version`, the `st sync` remedy, the unwired-verb error, and the GraphQL refusal -- all citing ST0056 and a WP. Found by grepping string LITERALS rather than comments.

One of them was **asserted by a test**, which is the part worth your attention: `an_unbuilt_command_names_the_work_package_that_owes_it` pinned the leak in place, having been written as the fix to a worse version of it. Inverted, and now sweeping every family rather than sampling two.

**The part I have NOT done, and am asking about before I do:** the published schema faces carry **~30 more** -- ST ids, WP numbers, D-ids, and `bin/intent_*` paths -- lifted out of Rust doc comments by schemars and async-graphql into `thread.schema.json` and `schema.graphql`, both of which `intent schema` prints to a consumer's terminal.

**Not all of them are violations and I am not going to sweep them by count.** "Natural id, eg `ST0056`" is a value-format example describing the reader's own ids; "carried under the WP-10 policy" is our backlog. That distinction needs reading every hit, not grepping.

**Two questions, both yours:** (a) is AT-00.8's guard yours, ic's, or mine? I do not want us both writing it. (b) Does D37 reach the schema faces at all, or only command output? I read it as reaching them -- `intent schema` prints them -- but the ruling's examples are all output, and if you read it narrower the sweep is much smaller.

### One I made myself, and it belongs in the record

**My first cut of the clause was a `///` doc comment**, which schemars lifted into the JSON face and async-graphql into the SDL. It put `AC-02.6`, a node name, a date and a test path into two published faces -- **a D37 violation authored while closing a different hole**, in the one file that already carries the warning about exactly this, three fields down. Caught by reading the face diff, not by any test. The block is a plain `//` comment now and says so.

-- cc

## (2026-08-15 14:48Z) AC-02.6 IS BUILT -- WP-02's last blocker. Claiming the close, and three things you need to rule on first.

**`b8405e2e`.** 283 tests, clippy and fmt clean, pushed to both remotes and fresh-clone-verified on each push. `AT-02.6` = `native/rust/crates/intentsvcs/tests/openness.rs`, 8 tests.

### `events.jsonl` IS BUILT -- it was the actual blocker

The AC was never going to go green without it: **nothing derives history**, so until today the DB was the only copy there was, and the extract could not carry the one table that cannot be rebuilt.

- **JSONL, not a JSON array**, because the log is append-only: an array makes every append a whole-file rewrite and makes a truncated write indistinguishable from a corrupt one.
- Written through the same `WriteSet` as everything else, so a partial history cannot land (AC-04.1).
- **MERGED on the way in, keyed on the ULID.** `sync_from_disk` is destructive for every other entity and must not be for this one -- a restore from yesterday's clone would silently delete today. Idempotent, and two machines' logs are a union rather than a conflict.
- Its own `FacadeError` variant: "your history file is damaged" needs an action of its own and must not be reported as though a thread were malformed. The remedy does **not** name a delete.

### THE ENUMERATION IMMEDIATELY CAUGHT THE CONTRACT

`acceptance.md`'s AC-02.6 ends: _"8 tables in the DDL: threads, wps, criteria, tests, related, issues, event_log, file_index"_.

**The DDL has NINE.** `doc_sections` is not in that sentence. A roster copied from the contract would have missed it -- which is precisely why the AC says to enumerate from the face, so the AC's own instrument caught the AC's own prose. **Your file, so I have not touched it**; flagging rather than editing.

### Where the declarations live, and why it is not a Rust table

**In the DDL face itself**: `-- openness: carried by <path>` or `-- openness: DERIVED -- <why>`. So the published artefact says which data can leave and which is recomputed, which is the openness property rather than a test about it -- and a declaration cannot drift from its table when they are consecutive lines of one string.

**The discriminating case is an ASSERTION, not a mutation somebody ran once.** The checker is a function of a DDL string, run over the real one and over a synthetic one carrying an undeclared table. Three shapes go red: silence, `DERIVED` with no reason, and a form of words a reader cannot act on.

Killed by mutation three ways: not writing the log fails 4 tests; restoring by wipe instead of merge fails 2; dropping a field from the extract fails the round trip **naming the field**.

### TWO THINGS FOUND WHILE BUILDING IT

**1. `events.jsonl` escaped the file scan's JSON parse through SUFFIX LENGTH.** `"events.jsonl".ends_with(".json")` is false, so `inspect()` skipped it by accident rather than by decision. **That is the same passing-by-luck D29 named for the database file, arrived at a second time** -- and a later `contains(".json")` or an extension normaliser would have started calling the one file that carries all history malformed, which blocks every ingest of the project. It is read as JSONL now, so a damaged line is located rather than the document condemned.

**2. D36 residue in my own tests**, which dc measured and correctly did not touch: a store DELETED to construct a fixture, which is the test-fixture idiom the ruling names by name -- one of them sitting a line below a comment written to kill the same idea in prose. Both gone, and **not by renaming**: a clone is now a SECOND PATH holding only the extract. That is also the stronger fixture, because the old form could only reach "a store that is not there" by destroying one.

### THREE THINGS FOR YOU

**(a) The AC's table count is wrong** (eight named, nine present). Yours to correct.

**(b) `AT-02.6` status is to-write in `acceptance.md`.** It exists and is green. Yours to set -- I do not edit the contract.

**(c) TWO D36 SITES REMAIN AND I DELIBERATELY LEFT THEM**, `crates/intent-cli/tests/search_surface.rs:56` and `cli_end_to_end.rs:591`. Both delete the store to force a cold re-ingest, and **both carry a comment saying why: the CLI cannot yet spell the disk -> db direction (AC-03.9, ic's dispatch row).** They are a workaround for a missing command, not a modelled operation. I could hide them behind the clone fixture in one edit -- **and that is the argument for not doing it**: the scaffolding would remove the only pressure to ship the selector, and a D36 sweep would then find nothing while the gap persisted. Left findable on purpose. Overrule me and I will change them.

### Still open from 14:30Z

The ingest posture (a kind/state mismatch now stops the whole estate loading), whether D37 reaches the published schema faces (~30 hits, needs reading not counting), whose AT-00.8 guard it is, and **the fifth state, which you have not ratified and I am now building on in four places.**

-- cc
