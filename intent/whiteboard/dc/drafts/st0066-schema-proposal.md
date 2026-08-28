# ST0066 open item (1) -- the fiat record's shape, and what a schema bump actually costs

Written by dc for vc, to be put to hv. **This is a PROPOSAL and a COST, not a decision.** Everything here is measured against the tree at `dbbeb08e` unless it says otherwise, and where I could not measure something I say so rather than filling the gap.

## The shape, and why it follows the estate's own grain rather than my taste

### AC -- a sixth `AcState` variant, not a field beside `state`

`AcState` (model.rs:1107) is `Computed | Unsatisfied | Satisfied{evidence} | Descoped{to,by,reason} | Withdrawn{reason,by}`. Two of those five are already exactly the thing a fiat close is, and `permitted_for` says so in its own comment:

> Decisions about the REQUIREMENT rather than about its satisfaction, so both kinds hold them and both must store them -- an AT status cannot recompute a scope decision (vc, 2026-08-15).

A fiat close is a third decision of that family: it applies to test-backed and non-test criteria alike, it must always be stored, and no AT status can recompute it. So the proposal is a sixth variant returning `true` from `permitted_for`, beside `Descoped` and `Withdrawn`:

```rust
Fiat {
  #[schemars(length(min = 1))]
  because: String,
  by: String,
  at: String,                                  // RFC 3339 UTC, as the event envelope's `ts`
  invoker: Invoker,                            // tty-or-not, env fingerprint
  #[serde(default, skip_serializing_if = "Option::is_none")]
  inherited_from: Option<String>,              // AC-00.3's cascade marker: the ancestor FC
}
```

**The variant is what makes AC-00.4's "never renders as an ordinarily satisfied one" structural instead of a rule somebody remembers.** A `fiat: Option<...>` field sitting beside `state` would re-create the shape hv's 2026-08-15 ruling removed -- "three stored values, two meanings, one of them never written" -- and `AcState`'s own doc says the collapse into one enum is what made the asymmetry structural. It is also the shape hv already declined in the package menu, in the ruling's own words: _an FC'd AC is not "satisfied", and a ledger is a second home for state the model should carry._

**`#[schemars(length(min = 1))]` on `because` is not decoration.** `Withdrawn::reason` carries it with a comment explaining that a required `String` delivers a narrower property than the one ruled -- it makes the field mandatory, not the reason present, and `String::new()` builds it. That comment records ic tracing an empty evidence from the CLI all the way to the close gate counting it. **AC-00.1 says an FC without a reason does not execute, which is the same property, so it wants the same three enforcement points the estate already uses:** `minLength` in the generated schema (refuses the FILE), a `Guard` on the API call, and `doctor` reporting an estate that already carries one.

**The enum's own doc anticipates this:** _"Exhaustive on purpose, for the reason `AcState::permitted_for` is: a sixth variant should not compile until someone names it."_

### AT -- a `fiat` variant on `AtStatus`

`AtStatus` is `to-write | red | green | n-a`. All four are statements about the instrument. A fiat close is not an instrument state, and recording it as `green` would make the row lie about the test -- so it is a fifth value carrying the same record. This also keeps AC-00.3's "no cascaded child renders as ordinarily closed" structural on the AT side.

### ST and WP -- THIS IS THE FORK, AND I AM NOT SETTLING IT

`ThreadStatus` (`triage | not-started | wip | hold | completed | cancelled`) and `WpStatus` (`not-started | wip | done | cancelled`) are lifecycle machines with a ratified transition table (hv, 2026-08-15). `AcState` and `AtStatus` are outcome records. That difference produces two defensible answers and they are not equally good in the same way:

- **(i) Fiat lives BESIDE the status** -- `status: completed` plus `fiat: Option<FiatRecord>`. It leaves the ruled thread machine untouched. **Its weakness is exactly AC-00.3's requirement:** anything reading `status` alone renders a fiat-closed thread as ordinarily completed, and "every renderer must remember to read the second field" is the weak form of the property.
- **(ii) Fiat lives IN the status** -- a `fiat-completed` / `fiat-done` member. The render property becomes structural, and the price is forking a machine hv ratified, with every transition table and every match arm in the estate to revisit.

There is a third shape the estate already uses that may dissolve the fork: **`AcRow.state` is `ac list`'s COMPOSED line** (`descoped-to: ST0057`, `satisfied: yes`) -- one home that composes state and payload into what a reader sees. Under (i) plus a single composer that every status render is required to go through, the property is carried by one function rather than by every caller's memory. **I lean (i) plus the composer**, but this is a ruling about a machine hv ratified and it belongs to hv, not to me at the keyboard.

### `status_reason` is NOT the home, and this is worth stating because it looks like one

`Thread::status_reason` and `WorkPackage::status_reason` exist and read like the obvious place. They are not: their doc says the field **belongs to the current status and any transition without a reason clears it**. A fiat record must survive forever (posture point 3). A cleared-on-transition field is the opposite property.

**But `status_reason`'s doc is the precedent that legitimises the whole design**, and it is worth quoting to hv because it answers the "isn't this a second home" objection in the estate's own voice:

> The HISTORY is the event log, not this field. Every guarded verb puts its reason in the envelope, so the sequence of decisions is durable and queryable; this carries only the latest one [...] That is a denormalised read of the log rather than a second source of truth: both are written by the same call, and only the log is ever read for history.

So: **the entity carries the fiat record; the event log carries the history, as it does for every guarded verb.** That is not the "separate fiat ledger" hv declined -- the declined thing was a fiat-specific ledger holding state the model should carry. The event log is the estate's existing universal history and already carries `principal`, `ts` at millisecond precision, `op` and an opaque `payload`.

## Minutia 3 -- partial coverage, on vc's measured input

vc's sweep found four ST0056 ATs held `red` while their own note says the instrument passes, purely because the model cannot say "green but covers half the criterion", and two of the four cite another row as precedent -- so it is an institutionalised convention enforced by nothing but note text. Conflab has already closed two by hv's fiat with the half recorded "ACCEPTED UNVERIFIED" in hand-written prose.

**If the fiat record carries `because` alone, hv's "which half was accepted" lands in prose again -- which is the exact mechanism ST0066 exists to replace.** Two options:

- **(a) free text in `because`, by convention.** Cheap, and it reproduces `0116`: a convention enforced by note text.
- **(b) a structured field** on the fiat record naming the accepted-unverified part.

**I lean (b)**, because the motivating case for the whole thread is hand-written provenance and (a) preserves it one layer up. It widens the record, so it is a ruling.

## The migration cost, measured

### The store needs almost nothing, and this is the good news

- **`criteria.state` is ONE `TEXT NOT NULL` column holding the whole `AcState` as serde JSON**, deliberately: _"One column because the state is one value: the trio could hold combinations the model has no meaning for [...] The discriminant stays queryable as `json_extract(state, '$.is')`."_ **So the AC variant needs ZERO DDL change and zero row rewrite.**
- **`tests.status` is unconstrained `TEXT NOT NULL`. So the AT variant needs ZERO DDL change.**
- **No CHECK constraint guards any status column** in `threads`, `wps`, `criteria` or `tests` -- the only CHECK in those tables is `attachments`' text-XOR-blob. Nothing at the database refuses the new values.
- **Only the ST/WP fiat record needs DDL**: one nullable column on `threads` and one on `wps` under shape (i). The migration ladder already exists (the DDL comments name "rung 11").

### The published faces move, and that is cheap and already ruled

`SCHEMA_JSON_VER: 11` and `SCHEMA_SDL_VER: 9` (faces.rs:44-48) both move; `SCHEMA_DDL_VER: 11` moves only if the ST/WP columns land. D41 (hv, 2026-08-15) already rules that these move independently of `INTENT_VER` and that a consumer regenerating a client reads the second alone. These are hand-maintained constants regenerated into five faces by the build.

### The expensive question is `intent/thread@3.0`, and it is a fleet question, not a code question

**`ingest.rs:111` is an EXACT STRING MATCH** -- `if thread.schema != THREAD_SCHEMA` refuses with `schema is {..}; this binary reads {..}`. It cuts both ways: a bumped binary refuses every existing extract, and an unbumped binary refuses every new one.

**Measured here: 169 tracked extracts under `intent/.canon` -- 67 threads, 101 issues, 1 project.** Issues carry their own `intent/issue@3.0` and FC does not touch them, so any bump should be thread-only; bumping issues as well would be churn with no cause.

**Measured: `intent/thread@3.0` has NEVER moved** -- 28 commits touching `model.rs`, every one of them `3.0`. There is no precedent to lean on in either direction.

Three options and their real costs:

- **(a) Bump to `@3.1` and rewrite.** The new binary refuses all 67 thread extracts here until they are rewritten, and every consumer estate the same. **In THIS estate the rollout has to be atomic across five nodes on one shared checkout** -- the first node to sync makes the repo unreadable to every peer still on the old binary.
- **(b) Bump and widen the check to a read-range** (read `3.0` and `3.1`, write `3.1`). No big-bang on read, but the first sync still emits `@3.1` and peers on the old binary still refuse. Same five-node window, later.
- **(c) DO NOT BUMP.** The variant is additive: an old extract parses unchanged under the new binary, and an old binary meets `"is": "fiat"` only in a thread that has actually been fiat-closed -- where refusing is CORRECT rather than a bug. **Churn: zero of 169 extracts, until an FC actually happens.** The price is that the version string stops being a capability marker and becomes a statement about content.

**I lean (c)** and I hold it loosely, because the thing it gives up -- a version that tells a reader what the writer could do -- is precisely what D41 says the face versions are for, and those are moving anyway.

## What I could NOT measure, stated rather than left as a gap

I tried to establish whether `SCHEMA_JSON_VER` had moved historically while `intent/thread@3.0` held, which would have been evidence about how this estate treats additive contract changes. **The loop returned empty for all 19 commits and the empty was the INSTRUMENT, not the fact** -- the same reader returns `11` on `HEAD`. **So I am not reporting "it never moved"; I do not know.** D41's stated semantics are better evidence than a count would have been in any case.

## Housekeeping

**`fc` does not collide.** Zero `fc` tokens in `surface/`, and it is absent from the top-level verb list. Checked on paper against the dispatch surface and `--help`; no writing verb was run to find out.
