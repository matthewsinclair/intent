# inbox: vc -> hv

## (2026-08-15 09:55Z) *** ANNOUNCE -- D01 IS REVERSED BY HV. THE DB IS THE SSOT. THE FILES ARE RE-CREATABLE. *** Announced at hv's explicit instruction.

**THIS IS THE OPPOSITE OF D01 AS WRITTEN. Read it before you write another line against the old model.**

hv, direct, 2026-08-15, and emphatic that they have said it multiple times already:

> "the db is the SSOT and it's the FILES that are re-creatable... All of intentsvcs MUST be working from the db. There is a sync process, either manual or triggered from the daemon, that enables disk-to-db and db-to-disk updates. But it is definitionally the db and the fact that there is a programmatic, typed API (via the rust intentsvcs) that ensures that the only data that goes into the db conforms by construction to the schema."

**STATED BACK, so the shape is unambiguous:**

1. **The DB is the single source of truth.** Not the committed JSON canon.
2. **The files are the RE-CREATABLE artefact.** That is the direction of the relationship, and it is the reverse of what design.md says today.
3. **All of `intentsvcs` works FROM the db.**
4. **Sync moves data BOTH ways** -- disk-to-db and db-to-disk -- either manually or triggered by the daemon.
5. **The integrity guarantee is STRUCTURAL, not procedural**: the typed Rust API is the only way data enters the DB, so everything in the DB conforms to the schema **by construction**.

**WHAT THIS OVERTURNS.** D01 as written says durable truth is committed schema-validated JSON, the SQLite DB is a rebuildable runtime index, `rm intent.db` is always safe, and there are NO DB migrations ever. **Those consequences do not survive as stated.** Do not reason from them, do not cite them, and do not defend a design decision with them until the canon is rewritten -- I am rewriting D01 now, along with D32's note, D33's second constraint, and AC-14.11.

**THIS IS VC'S ERROR AND I AM NAMING IT AS MINE.** hv said this before, more than once. I recorded the phrasing TWICE -- in D32 ("durable state is in the db") and again in D33 ("db-enforced timestamp") -- and both times wrote it down as **explicitly NOT reversing D01**, on the reasoning that hv's contrast was model-versus-scattered-markdown. I put it on hv's queue as an open question and reported it as open in four separate status reports. **Three of you stopped on this ambiguity independently. That is three signals, and the correct response to the first one was to ask hv a direct yes/no question rather than to record it and route around it.** I kept choosing "recorded, not settled" over "ask", and the cost landed on cc as code written against the wrong truth model.

**The rule I should have followed is one already on this board**: _never settle by inference_ -- which I applied correctly. What I missed is its other half: **refusing to settle by inference is not a resting state. It obliges you to go and get the answer.** An open question parked across three rulings is a decision made by default, and it was made wrong.

**WHAT PROBABLY SURVIVES, and nobody should act on it until it is in the canon**: a timestamp is stamped once at the moment of the event and never re-derived by a later sync **in either direction**. Under the old model I argued that from "the DB is rebuildable"; the argument inverts but the requirement looks unchanged, because a sync that re-stamps rewrites history whichever side is truth. It will be stated properly in D33 rather than reconstructed by each of you.

**WHAT IS NOT AFFECTED**: statements about the MODEL and its state transitions -- entity shape, the AC/AT contract, mutation completeness, Direct/Incidental edges, the schema faces. Those are claims about what is modelled, not about which side is durable. If you are unsure whether something you built is affected, say so and I will rule rather than leave you guessing.

Corrected canon follows shortly. Ask me anything.

-- vc

## (2026-08-15 10:53Z) *** ANNOUNCE -- "no DB migrations, ever" is DELETED. It was never asked for. The intentdb is the durable SSOT, full stop. ***

**hv, verbatim, correcting vc:**

> "no DB migrations, ever -- THIS IS NOT A CONSTRAINT THAT I EVER ASKED FOR. And it's not something that makes _any_ sense. If we have to do a db migration, we have to do a db migration. That is standard fare."

> "The intentdb is the durable SSOT. Everything else is a secondary artefact. We can certainly _recreate_ the db from previously extracted .json from the db, and we can certainly take a properly formatted .md file and ingest that SUCH THAT IT GOES THRU THE HARD GATE OF THE INTENTSVC API to become properly formed db items. But the db is the durable single source of truth. The end."

**FOUR THINGS, and none of them is a hedge:**

1. **The intentdb is the durable SSOT. Everything else is a secondary artefact.**
2. **MIGRATIONS ARE NORMAL.** If we need one, we do one. Delete "no DB migrations, ever" from your reasoning wherever you are carrying it. **Any decision in the estate justified by "we can never migrate" is resting on a constraint that was never asked for.**
3. **Re-creating the DB from a previously extracted `.json` is a CAPABILITY, not a licence to treat the DB as disposable.**
4. **Ingesting a properly formatted `.md` or `.json` produces well-formed DB items ONLY because it passes the HARD GATE of the intentsvcs API.** The gate does the work, not the file format.

**NOTHING ON DISK IS TRUTH.** `thread.json`, the `.md` views and `events.jsonl` are secondary artefacts of the same kind. There is no Highlander contest between them because none of them is a competing claim to truth. If you were holding "which disk artefact wins" as an open question -- I was, and I put it to hv as one -- it dissolves.

**THE EVENT LOG GETS A FILE FORM: `events.jsonl`, append-only** (hv, same ruling). Plus an `intent events` surface for query/extract/ingest/egest, and `intent db sql` for arbitrary queries including `intent db sql < query.sql`. **`intent db sql` is READ-ONLY and that boundary is load-bearing**: write-SQL is a second door into the SSOT, and the typed API being the ONLY door is the entire reason the DB's contents conform by construction. The write case is `intent events ingest`, which replays through the gate.

**THIS WAS MY ERROR AND IT IS THE SECOND OF ITS KIND TODAY.** I carried "no DB migrations, ever" as though it were a requirement to be preserved, and was still arguing hours after the reversal that it "survives" -- optimising to protect an invention. It came from the old disposable-DB model as a CONSEQUENCE and acquired the momentum of a REQUIREMENT because it was written into D01 beside things hv actually did rule. **A consequence recorded next to a decision starts getting defended like one.** Worth checking your own boards for the same shape.

**CANON CORRECTED** at design.md (D01, the DDL row, WP-13's T3 deferral -- which still stands, now for the simple reason that adding vector tables is a migration and migrations are normal), acceptance.md (AC-02.3's rationale, corrected twice today), and data-model.md (the event log is durable truth like everything else in the DB).

**THREE RUST DOC COMMENTS STILL CARRY THE FALSE CLAIM and they are cc's lane, not mine to edit**: `lib.rs:13`, `store.rs:3`, `store.rs:26` all say the DB is rebuildable with no migrations ever; `event.rs:5-7` says DB-only state must be losable and the event log is explicitly NOT durable truth. All four are now false.

-- vc

## (2026-08-15 10:56Z) *** ANNOUNCE -- hv's REAL standing requirement, and it is the one I mistook for "no DB migrations": PLATFORM AND DATA-MODEL OPENNESS. ***

**hv, verbatim:**

> "The constraint/requirement that IS something I want is: platform and data model openness. I want there to be ALWAYS a 1-1 mapping between the db schema entities and an equivalent .json or .md version of those entities SO THAT I can get my data out of the db and use it somewhere else LOSSLESSLY. That is the reason for the disk-to-db and db-to-disk syncing."

**THIS IS WHY BIDIRECTIONAL SYNC EXISTS.** Not backup, not disposability, not migration-avoidance. **Never being locked in.** Every entity in the DB must be extractable to a `.json` or `.md` you can take somewhere else and use without Intent.

**Contracted as AC-02.6, and it REOPENS WP-02 from PASS 5/5 to 5/6.** Held mechanically rather than by intention:

- **The table list is ENUMERATED FROM THE GENERATED DDL FACE, never a hand-maintained roster** -- so a new table enters the check the day it lands and cannot be forgotten. This is ic's enumerate-do-not-sniff rule and dc's measured-not-designed rule, applied to the thing they were both really about.
- Each table either **has a file form**, or carries an **explicitly DECLARED exemption naming why it is derivable**. **Absence of a file form is never the answer** -- D05's refusal posture applied to coverage.
- **Lossless proved by round-trip in BOTH directions**: db-to-disk then disk-to-db reproduces the DB content, and re-emitting reproduces the files byte-for-byte.
- **The file form must be usable WITHOUT Intent.** "Use it somewhere else" is the entire point, so standard self-describing formats, no Intent-only decoding.

**MEASURED NOW -- 8 tables in the DDL and TWO GAPS:**

```
threads, wps, criteria, tests, related   -> thread.json          ok
issues                                   -> issues/<n>.json      ok
event_log                                -> schema face, NO artefact   GAP (now events.jsonl, hv-ruled)
file_index                               -> no face, no exemption      GAP
```

`file_index` is a working-tree scan cache (path/size/mtime/sha256/state/findings, built from the tree not from canon) and is a **plausible** exemption -- but it must be DECLARED as one, with the reason, not left to be inferred from an absent schema face. That is the same "absence read as an answer" shape as `event_log`'s missing artefact, ic's banner-sniffing backstop, and my own hooksPath grep. Four instances, one class.

**AT-02.6 is `openness.rs`, and its discriminating case is ADDING A TABLE with no file form and no exemption and watching it go red.** A test that only checks the tables which already have file forms passes on the defect -- **which is exactly how `event_log` survived this long.**

**THE CORRECTION I OWE, and it is the useful part.** I have spent this morning defending "no DB migrations, ever" as though it were hv's requirement. It never was. It was a **consequence** of the old disposable-DB model that I mistook for the constraint -- **and the real constraint was sitting right next to it the whole time, doing the actual work.** Bidirectional sync was in the design from day one and I had it filed under the wrong justification. **When you inherit a rule with a rationale attached, the rationale is the part most likely to be wrong**, because it is the part nobody re-derives. The rule survives; go and check what it is actually for.

```
ac:   30/97 satisfied -- BLOCKED
lint: ST0056 ok -- 97 AT row(s) conform
gate: ST0056/02 BLOCKED -- 5/6; AC-02.6
```

-- vc

## (2026-08-15 14:08Z) FOR RATIFICATION -- Machine 3 has grown a FIFTH state in the implementation. It is right, and it is yours rather than mine.

**You ratified the acceptance-criterion machine with four states: `Satisfied | Unsatisfied | Descoped | Withdrawn`. The code has five.** The extra one is `computed`, and I think it is correct -- but extending a machine you ratified is not vc's call, so this is the one thing today I am stopping on rather than ruling.

### THE PROBLEM IT SOLVES, which the ratified table does not answer

A **test-backed** AC's satisfaction is computed from its covering ATs and is never stored -- that is the asymmetry you ratified. But `ac descope` and `ac withdraw` carry **no kind guard**, so a test-backed AC can be descoped. **`ac rescope` then has to land it somewhere.** Landing on `Unsatisfied` would store a satisfaction claim about a criterion whose satisfaction is computed, which is the double truth the four-state collapse exists to remove. **There is no fourth value that fits**, so cc introduced `computed` as the in-scope value for a test-backed criterion.

### WHY I THINK IT IS RIGHT -- it beat my own ruling on my own grounds

cc asked me to choose between two forms and I ruled for one at 14:04Z. Then I read what they had built, found this third form, and **reversed at 14:07Z** because it satisfies my two strongest grounds better than the form I chose:

- `state` becomes **required on every criterion**, so a criterion that LOST its state is refused rather than validating cleanly as "a computed one". My form needed absence permitted schema-wide, which was the hole I was arguing against.
- `{state: computed}` says on its face that the value is derived, so an external reader needs no Intent rule to read the file correctly. My form required a conditional -- **I argued against transferring a rule to the reader and then chose the form that transfers one.**

### WHY IT NEEDS YOU, and it is a process point rather than a design one

**It currently exists in `transitions.rs` and in the test that checks `transitions.rs`. Those are not two witnesses.** cc built the conformance test as a _second transcription of your ratified tables_, taken from `data-model.md` rather than from the code -- a genuinely good idea. But one author transcribing one document twice in one session produces two artefacts that **agree with each other and both differ from what you ratified.** The check reports conformance and cannot see the extension.

**cc did not hide it** -- they wrote the divergence into the code comment in plain language, which is the only reason I found it by reading rather than by a mutation run.

It is now recorded in `data-model.md` under "The fifth state" with the reasoning and my reversal. **Nothing is blocked on you**: cc should keep building on it, and if you rule against it the cost is one enum value and two edges. But four-becomes-five in a ratified machine should be a decision you made, not one you discover.

### One accepted cost, so the ratification is informed

Two fields can express nonsense -- `{kind: non-test, state: computed}` and `{kind: test, state: satisfied}`. The API refuses both today; I have asked cc to make the schema face refuse them too, so the extract cannot carry a combination that ingest will reject.

-- vc

## (2026-08-15 14:27Z) ONE INTERACTIVE STEP IS THE ONLY THING BLOCKING AC-11.2'S SECOND HALF -- and it is key material, so it is yours by necessity rather than by convention.

**Your signing ruling is recorded as D38**, conditional removed as you stated it: signing with the Geodica ADC keys is right regardless of whether brew needs it, so the Gatekeeper analysis is interesting and not load-bearing.

**dc has already implemented the first half.** `int macos <doctor|sign|notarize|env|store-creds>` is ported from Lamplight's devbin rather than reinvented, and **both binaries are signed now** -- verifying identically to `conflab`, which has shipped Developer ID signed through the geodica tap since July (same authority chain, same team identifier, same hardened-runtime flag).

### THE ASK

**Run `int macos store-creds` once, interactively.** It needs `APPLE_ID`, `APPLE_APP_PASSWORD` and `APPLE_TEAM_ID`. **No session should handle that material and none of us has asked for it or will** -- it is the same boundary as an interactive login. Until it is stored, notarisation is written and untested.

**Nothing else is blocked on it.** WP-11 continues; this gates only AC-11.2's second half.

### WHY AC-11.2 IS STILL RED, since a glance at the signing result would suggest otherwise

Its evidence is _a decision-log entry **and** a notarised artefact_. The first now exists. **Signing is not notarising, and the AC names both.** dc explicitly declined to mark anything satisfied and I have ratified that rather than merely accepted it -- a green here would be the same vacuous-evidence shape dc caught themselves on this morning, where two correct-looking results did not test the thing in question.

### TWO THINGS TO EXPECT THAT LOOK LIKE FAILURES AND ARE NOT

Both measured against `conflab`, both now in canon so nobody debugs them twice:

- **A bare Mach-O binary cannot have a notarisation ticket stapled to it.** `stapler validate` reporting no ticket is the CORRECT steady state -- the ticket lives on Apple's servers and Gatekeeper checks online.
- **`spctl -a -t exec` reports "rejected" on a correctly signed CLI.** It is refusing to assess a bare executable under a policy meant for app bundles -- a category error with a valid signature attached. **`codesign --verify --strict` is the check that means anything for a bare CLI.**

**This is the second item on your desk from me today**, the other being Machine 3's fifth state (`computed`) at 14:08Z, which is right and needs ratifying rather than fixing. **Neither blocks anyone.**

-- vc

## (2026-08-15 15:10Z) ONE NEW DECISION FOR YOU: which platforms does v3.0.0 ship? It is unruled, dc is blocked on it in spirit, and it costs platform reach

**Labelled, since you asked last time which of these were FYI.**

### 1. DECISION NEEDED -- WHICH TARGETS DOES v3.0.0 SHIP? (new, dc raised it, I confirmed it)

**There is no shipped-target statement anywhere in `design.md`.** I checked rather than took dc's word. The only platform sentence in the whole contract is AC-02.1, and that is a **CI build gate**, not a distribution commitment -- "builds with fmt + clippy on macOS and Linux" says nothing about what a user can install. No D-number covers it.

**Why it needs you and not me: it is a REGRESSION IN PLATFORM REACH, knowingly taken.** Intent v2 installs by clone-and-symlink and runs anywhere bash runs, and v2.11.14 fixed a Linux-only `set -e` break, so **Linux is a supported v2 platform in fact, not just in principle.** A macOS-only v3 takes that away from anyone on Linux at the cutover. That is a product call.

**dc's recommendation, which I think is right:** first cut ships **macOS arm64 only**, hand-rolled on the Conflab pattern (build -> sign -> notarise -> verify -> checksum -> publish, local), with **cargo-dist DEFERRED** until Linux artefacts are in scope -- where its matrix earns its keep and there is no signing question to fight.

**dc's counter-argument, quoted rather than summarised because it is the part that makes this a decision:** _"Intent v2 installs by clone-and-symlink and runs anywhere bash runs, so a macOS-only v3 is a REDUCTION IN PLATFORM REACH. That is a real regression for hv to accept knowingly, not something to discover after the cutover."_

**Nothing is blocked meanwhile** -- the macOS leg is needed under every option, so dc is building it now. The matrix changes how much gets built, never how signing works. **But this is the answer dc most wants before building anything, and an open question parked across three rulings becomes a decision made by default.**

Two supporting measurements, both dc's, both independently checkable: **cargo-dist 0.32.0 does not notarise at all** (zero hits for `notarytool`, `notariz`, `stapler`, `altool`, `xcrun`), so the tool AC-11.1 named covers at most half of D38 next door. And **Conflab -- same Apple team, same CLI+daemon shape, same tap -- has both paths and chose local**: `MACOS_RELEASE_CI` has been `off` since 2026-04-16, with four months of releases behind it. That is a revealed preference, not an opinion.

### 2. STILL ON YOUR DESK, UNCHANGED -- Machine 3's fifth state (`computed`), from 14:08Z

One word, yes or no. It is right and needs ratifying rather than fixing. Nothing blocked; cc's cost of reversal keeps rising.

### 3. FYI ONLY, NO ACTION -- the gate was over-reporting, and it was my defect

`ac gate` was scoring **AC-06.4 and AC-06.7 satisfied** while the prose of each said it did not close. Corrected: both texts now credit what actually shipped (search works; the WP view landed), both are held open on one uncovered arm each, and the thread count moved from 32/101 to 31/102.

The cause worth your half-minute: **an AT covers a FILE, an AC states a CRITERION, and the coverage claim joining them is hand-made by me and never re-tested.** Both ACs grew arms over three days as cc reported progress; the AT rows never moved. Underneath it, `ac gate` satisfies an AC on the **first green AT** covering it (`bin/intent_acceptance:454` ORs, it does not AND), so the natural repair -- a second row at `to-write` naming the gap -- has no effect on the verdict. **Filed as issue 0032**, not fixed: it is v2 tooling that WP-04 replaces, `bin/**` is not mutated with sessions live, and the interim control is in place. The note that matters is in the issue: **whoever builds the v3 close-gate must not port the early-return.**

-- vc

## (2026-08-15 15:15Z) CORRECTION to my 14:26Z: the codesign check I told you was the one that means anything RETURNS 0 ON AN UNSIGNED-BY-US BINARY

**FYI only in the sense that nothing is blocked -- but read it, because I put a bad check in front of you three hours ago and you may have written it down.**

At 14:26Z I told you: _"`codesign --verify --strict` is the check that means anything for a bare CLI."_ That was wrong in the dangerous direction.

**`codesign --verify --strict` returns 0 on an ad-hoc signed binary.** An ad-hoc signature is a valid signature -- the check answers "is this signature intact", not "is this OUR signature". A binary the linker signed with no identity at all passes it cleanly. **The check that actually discriminates is verification from a QUARANTINED copy**, which is what dc built into `int macos verify` and `int macos stage`.

**dc proved it by accident, on live bytes, within the hour.** `target/release/` is shared mutable state: cc rebuilt at 14:59Z, and the linker's ad-hoc signature silently replaced the Developer ID one on `intent` (`Signature=adhoc`, `TeamIdentifier=not set`), while `intentd`, untouched since 14:23Z, kept `76BQL8L47U`. **One of two shipped binaries stopped being notarised with no signal anywhere, and the obvious check still said 0.**

**The class is the one I have been holding everyone else to, and I supplied an instance: a sufficient-looking check that answers a NARROWER question than the one being asked, and fails green.** It is the same shape as cc's schema stamp this morning (a store that opens cleanly and cannot be read) and dc's own doctor blank-field bug. Mine had the extra property of being handed to you as a recommendation.

Two things that follow, neither needing you:

- **AC-11.2 stays satisfied and its evidence is updated.** The criterion is that the decision is recorded and the mechanism implemented -- both true, and now proven twice (Apple Accepted `cc52d5da` and `5eddb54a`). What changed is that the evidence line now says the **artefact is transient and is not the evidence**; a binary on disk in a shared build directory cannot carry a claim for longer than the next `cargo build`.
- **The two facts I gave you at 14:26Z about stapling and `spctl` still hold** -- a bare Mach-O cannot have a ticket stapled, and `spctl -a -t exec` rejecting a valid CLI is a category error. Those were right. It is only the "use `codesign --verify --strict` instead" half that was wrong.

-- vc

## (2026-08-15 15:17Z) A SECOND DECISION, and it is a one-liner with a measurement behind it: the five schema faces we PUBLISH carry no version at all

Raised by ic's contract question, measured by me before ruling. **Short, and it is a genuine D-number candidate rather than a bug.**

```
schema/ddl.sql             0 occurrences of "version"
schema/schema.graphql      0
schema/thread.schema.json  0
schema/issue.schema.json   0
schema/event.schema.json   0
```

**cc gave the internal, per-machine, never-committed store a `SCHEMA_VERSION` and a hard refusal this morning. The five artefacts we actually publish to people outside this repo have nothing.** A consumer compiles against the DDL, the SDL and the JSON Schemas; a silent change breaks them at their next upgrade with nothing to diagnose it by.

**AC-06.5 does not cover this and it looks like it does** -- it asserts the printed face is byte-identical to the committed file. Both move together, both stay identical, the gate stays green, and the consumer still breaks. **It is a consistency check, not an anchor.**

**The internal artefact is versioned and the external contract is not, which is exactly backwards:** the store can be rebuilt from the extract, and a consumer's code cannot be rebuilt from anything we hold.

**What I would rule if it were mine:** the published faces carry a version, and moving it is a deliberate act -- the same instrument cc built for the DDL, pointed outward. Help text gets nothing; changing a help string breaks nobody's code and pinning it buys churn. **The distinction is whether a consumer COMPILES against it.**

**The pattern this is the third instance of, named so we stop meeting it fresh: a generated artefact with a hand-kept companion that must move with it.** cc's `SCHEMA_VERSION` beside the DDL. dc's sha256 beside the tap formula. The published faces, beside nothing. The first two got tripwires today; the third is the one with a consumer we cannot see.

**Nothing is blocked and nobody is waiting on it** -- ic has been told it is yours and has moved on to the inventory re-probe. It wants a D-number before WP-06 closes, not before the bounce.

-- vc
