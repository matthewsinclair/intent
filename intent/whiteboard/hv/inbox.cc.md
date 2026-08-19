# inbox: cc -> hv

## (2026-08-19 15:53Z) FYI only -- no response needed.

**TAKING WP-03 (attachment canon) AND IT TOUCHES SHARED PLATFORM FILES: `model.rs`, `project.rs`, `store.rs`, `export.rs`, `ingest.rs`, and READS `organize.rs`.** ic you are on ST0056 WP-03 (sync) which is `ingest.rs`/`facade.rs`; dc you own `organize.rs`. **I will not touch `facade.rs`, `sync.rs` or `organize.rs` in this pass** -- my ingest edit is `collect_attachments` in `project.rs` plus the sidecar read in `ingest::read`, nowhere near `resync`/`Scope`. Shout if that collides and I will hold.

**THE DEFINITION WAS UNWRITTEN AND I HAVE MEASURED IT INSTEAD OF PICKING ONE.** `opaque` carries six criteria and is defined nowhere in ST0056, ST0057, canon or any board -- the closest is vc's own D57-7 note, _"my BINARIES never dehydrate was wrong"_, so opaque = binary = not representable as text. **`collect_attachments` (`project.rs:699-708`) already names the exact population**: a file eligible by `ATTACHMENT_EXTENSIONS` whose bytes are not valid UTF-8 is REFUSED today with _"not valid UTF-8, so it cannot be carried as text"_. That refusal list IS the opaque set, so AC-03.1 needs no widening of the carry list and no new policy -- the refused become carried, as bytes, in a sibling file.

**MEASURED, AND IT CHANGES WHAT AC-03.1's DENOMINATOR CAN EVER BE: of 745 files under `intent/st/`, exactly ONE is not valid UTF-8 and it is `intent/st/.DS_Store`, which D29 puts outside the corpus. THE ESTATE HAS ZERO OPAQUE ATTACHMENTS.** So AC-03.1's _denominator printed over every opaque attachment in the estate_ is 0 of 0 by construction, and a green over the estate alone would be vacuous -- right verb, right depth, a population that cannot contain the failure. AT-03.1 will be driven by a constructed fixture with real non-UTF-8 bytes AND print the estate zero out loud as a zero.

## (2026-08-16 11:41Z)

**On "the sooner we can get this project onto v3, the better" -- the gate is the MIGRATOR, and it is one thing rather than a lot of things. I am taking it now (WP-10).**

Your `doctor` run is the whole answer and it is working correctly. The first finding is the real one: this repo declares 2.19.0 and **53 steel threads carry v2 canon this binary cannot read**. Nothing else can be exercised on real data until that converts, so every other number below is downstream of it. `intent upgrade` is declared and unwired; `intent ingest` is wired and refuses, because the parser is WP-10 and WP-10 has not been built.

**Measured just now, so the scale is not a guess: 55 of 106 shipped command rows are wired, 51 are dark.** Still-dark families: agents(6), claude(5), ext(5), issues(6), lang(6), llm(3), modules(3), plugin(3), st(3: bootstrap/edit/repair), config(2), and one each of bootstrap, critic, fileindex, info, init, learn, mcp, upgrade, version. **`todo`'s six landed a few minutes ago** (`a7aa0b9e`), and `export` and `ingest` earlier today.

**The honest sequencing, and it is not "wire all 51 first".** Migrating this repo needs a much smaller set than the full surface: the migrator, then the handful of commands you would actually use on a migrated estate -- `st`, `wp`, `ac`, `at`, `todo`, `search`, `doctor`, `sync` -- and **all of those are already wired.** So the path to dogfooding v3 here is the migrator plus verification, not the long tail. `issues` (6 rows) is the one gap in that daily set and it is small; I will take it straight after.

**THE SIX `surface:` LINES ARE A DECISION YOU OWE, NOT WORK ANYONE IS DOING.** They are AC-06.8's mitigation reporting flags that ic's table declares and the build deliberately withholds, pending a ruling on whether they ship. They cost nothing to clear and they are cluttering the one diagnostic you will be reading all week:

- `bootstrap --quiet/-q`, `doctor --quiet/-q`, `doctor --verbose/-v`, `fileindex -v` -- ordinary verbosity flags.
- `upgrade --backup-dir`, `upgrade --no-backup` -- these two are NOT ordinary. `--no-backup` disables the safety net on the one command that rewrites an entire estate, and the v3 migrator is exactly where a backup matters most.

**One line from you per group is enough** ("verbosity flags ship; `--no-backup` withdrawn" or whatever you rule), and the six findings go away. If you would rather they stayed quiet until the surface settles, say so and I will ask ic to mark them and stop reporting them -- but I would rather you saw them than not.

**What I am NOT doing**: wiring `agents`, `lang`, `claude`, `ext`, `plugin`, `llm`, `modules`, `init`, `bootstrap`, `upgrade`'s canon half. Those are the installer and canon-management machinery, they are the largest block of the 51, and none of them is on the path to running v3 against this repo's data. Tell me if you want any of them pulled forward.

-- cc

## (2026-08-16 12:16Z)

**THIS REPOSITORY'S ESTATE PARSES WITH ZERO BLOCKING RESIDUE. It is migratable.** WP-10 Phase A is built and pushed (`6f6e80c6`). Run it yourself:

```
intent ingest
read: 56 thread(s), 140 work package(s), 280 criteria, 227 acceptance test(s)
residue: 0 blocking, 9 carried
ok: this estate parses -- nothing was read into a store and nothing was written
```

**Phase A is the read-only check half, and it writes nothing at all** -- not a file, not a database. That is migration.md's split and it is the honest thing to ship first: you can now see the migration state of any v2 estate before anything touches it.

**The 9 carried are all in closed threads and all convert as they are**, so there is no work list for you: 8 work packages in ST0023 that predate the frontmatter convention, and one `scope: Medium-Large` in ST0020. Nothing to fix under v2 first, which was the outcome I could not promise before measuring.

**WHAT REMAINS BEFORE THIS REPO IS ON v3 -- three things, and the first is the only one that is mine alone.**

1. **Phase B, the conversion**: emit `thread.json` per thread, split the issues, regenerate views, stamp `project_id`, build the DB, one commit. Straightforward now that Phase A reads the estate correctly.
2. **A model gap that Phase B hits and I will not paper over** (with vc, whose call it is). `scope: Medium-Large` maps to no T-shirt size and `WorkPackage.scope` is a bare enum with no marked-legacy form -- vc ruled that form in on 2026-08-15 and the model half has not landed. **Exactly one row in your estate needs it.** Options are all bad without the model change: normalising is a guess, blocking violates your own carry policy, dropping is loss.
3. **`intent upgrade` itself** -- the verb is still unwired; `ingest` is the door the migrator plugs into.

**TWO DEFECTS OF MINE WORTH YOU KNOWING ABOUT, because both were the parser blaming your estate for my limits.** The first run reported **246 findings**; 227 were "unparseable AT row" because v2 spells `-- covers AC-01.2` with no colon beside `-- status: green` which has one, and one accessor assumed both were keyed alike. The second run reported 20; **19 were fields that had never been authored, reported as values "not in the v2 vocabulary"**. Absent is not invalid, and telling you otherwise would have sent you to repair 19 files v2 was perfectly happy with. The real number is 9. **I found both by RUNNING it against your estate, not by reading the spec** -- and I would not trust a migrator whose author had not.

**Still open with you, unchanged and cheap: the six `surface:` lines in `doctor`.** One ruling per group clears them.

-- cc

## (2026-08-17 18:34Z)

**A PROTOCOL GUARD I WANT TO ADD BESIDE THE CLOCK GUARD, and I am bringing it with a measurement rather than as a proposal, because I broke it myself today and it took a full session to notice.**

**The rule: an archive only ever grows, so a commit that REMOVES lines from `intent/whiteboard/<node>/.history/**` is refused.**

**What happened.** `clear` and `archive` MOVE handled inbox entries into `.history/<date>/inbox.<sender>.md`. That file accumulates across a day. My fold before a compact WROTE it instead of appending, so the one entry being archived replaced the twenty already sitting there: **492 lines destroyed, spanning 2026-08-16 19:54Z to 2026-08-17 14:45Z**, in a commit whose subject said the board had been folded. Recovered from git, re-merged in timestamp order, both halves diffed byte-identical rather than eyeballed, committed at `6560be27`. Nothing is lost.

**Why it survived a full session, which is the actual argument.** The fold looked correct from every angle anyone would check: the live inbox correctly read `_(empty)_`, the archived entry was correctly present in `.history/`, the board read as folded, and `intent claude ws hygiene` was clean. **The only signal anywhere was `514 deletions against 75 insertions` in `git show --stat`** -- on a commit I had no reason to expect deletions in at all, and which I only read because a number looked wrong.

**Why a guard rather than care.** Writing where you meant to append is a one-character difference with no visible symptom, and every human-facing check reads correct afterwards. It is the same shape as the clock class: a protocol invariant that fails silently and leaves a plausible-looking artefact. **The check needs no knowledge of what the fold intended** -- it is arithmetic on a diff, it cannot false-positive on legitimate work because archiving only ever adds, and it is cheap enough to sit in the existing pre-commit gate that already runs the clock guard.

**ic has checked their own board the way I suggested -- `git log --stat -- ic/.history/`, insertions only, zero deletions -- and backs the guard.** Their point, which is better than my diagnosis: the rule is checkable by a machine and needs no knowledge of intent, **which is precisely why it catches a case that survived a full session of things looking right.**

I have not built it. It is a whiteboard-protocol change and it touches `lib/templates/hooks/`, so it is a shipped-consumer surface and yours to sequence -- and we are under your moratorium on new instruments. **Say the word and it goes on the list; say no and the finding stands on my board as a watch-out either way.**

-- cc
