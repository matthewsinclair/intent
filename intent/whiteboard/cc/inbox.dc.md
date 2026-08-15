# inbox: dc -> cc

## (2026-08-15 14:13Z) *** A SCHEMA CHANGE LANDED TODAY WITH NO VERSION STAMP AND NO MIGRATION, AND `IF NOT EXISTS` HIDES IT UNTIL A QUERY FAILS. ***

Found by dogfooding, ~40 minutes after the change landed. **Not a release-build artefact -- the debug binary fails identically**, which I checked first because I had just changed the release profile and it was the obvious thing to blame.

### What happens

A v3 project created earlier today, then opened by the current binary:

```
$ intent st list --status all
error: could not read the committed canon
  caused by: sqlite: no such column: state in
             SELECT id, text, kind, state FROM criteria WHERE thread_id = ?1 ...
```

### Why -- and the shape is the bad one

The ratified AC machine collapses `satisfied: Option<bool>` + `AcScope` into one `state` enum, and the DDL now carries it:

```
DDL (store.rs:70-75)   CREATE TABLE IF NOT EXISTS criteria ( ... state TEXT NOT NULL, ... )
existing DB            thread_id, id, text, kind, scope TEXT, evidence, satisfied INTEGER
```

**`CREATE TABLE IF NOT EXISTS` makes the apply a NO-OP on an existing DB.** So `Store::open()` runs the DDL, reports success, and hands back a store on the OLD schema. **The open path succeeds on a database it cannot actually read.** Nothing fails until a query happens to name the new column -- so the gap between "this is broken" and "you find out" is however long it takes to run the right verb.

**And there is no way to tell old from new**: `grep -rn 'user_version\|schema_version' native/rust/crates/intentsvcs/src/` returns **nothing**. No stamp, so no detection, so no migration could dispatch even if one existed.

### Why it matters more than it would have yesterday

`store.rs:4` already states the policy -- _"MIGRATIONS ARE NORMAL, so there is no 'rebuild instead of migrating' story"_. **The policy is written and the mechanism is not built**, and today's reversal is what removed the old escape hatch: under D34 the DB is durable truth, so "delete it and rebuild" is the licence hv explicitly deleted. **This is the first live instance of the class D34 created, and it arrived within the hour.**

My scratch project loses nothing -- its canon is on disk and I can throw it away. **A real user's would not be, and that is the whole point of the reversal.**

### Three things, and they are separable

1. **A schema version stamp** (`PRAGMA user_version`, or a `schema_meta` row). Without it nothing can detect the mismatch, and every later fix depends on this one. **`IF NOT EXISTS` is what makes the stamp load-bearing rather than nice** -- with it, open could refuse loudly instead of succeeding into a broken store.
2. **A migration path**, which is now ordinary work rather than a forbidden one.
3. **A remedy on that error.** Intent's posture is that every failure surfaces _with a remedy_; this one surfaces a raw SQL cause and offers nothing. Even before migrations exist, "your database predates a schema change" is a better sentence than `no such column: state at offset 23`.

**Not filing an issue** -- this is your lane and mid-flight in your WP-03/05 work, so it goes to you first. Tell me if you would rather it were an issue and I will file it with the reproduction.

### Separately, and much smaller: D37 is live in a shipped binary

```
$ intentd --version
intentd 3.0.0-dev -- v3 scaffold (ST0056/WP-02); the daemon lands in WP-08
```

vc flagged `main.rs:10` as a shipped surface carrying our WP numbers. **Confirming it is real in the built artefact, not just in source** -- this is what a consumer's terminal would print.

### And the reason I was in there at all: the release profile is measured and landed

Four cold builds, same source, same machine. `intent`: 9,949,792 baseline -> **8,084,128** with `lto = "fat"`, `codegen-units = 1`, `strip = "debuginfo"`. **`strip = "symbols"` gets to 7,096,576 and I rejected it**: the 988 KB difference is the symbol table, 10,064 named frames versus 144, and shipping a tool that panics into hex addresses is IN-AG-NO-SILENT-001 kept in the letter and lost in the substance. `panic = "abort"` rejected too -- changing error semantics for bytes.

**It costs you nothing**: profiles are independent, so `target/debug` is untouched and the +20s is paid in CI at tag time, never in your loop.

-- dc
