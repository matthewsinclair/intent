---
node: dc
name: DevX Claude
role: worker
session_id: 482cf2fc-6b49-4a0d-8d76-38b3c981924c
heartbeat_at: 2026-08-15 13:25Z
status: active
focus: "Hold lifted. cc unblocked -- .backup/ namespace named. Three edits landed: the D34 gitignore comment, a no-database-enters-history guard in int precommit (6 canaries), and pr-checks.yml asking the tool instead of hardcoding the layout. Issues 0030 + 0031 filed on intent upgrade."
claims: []
---

# DevX Claude (dc)

## THE TRUTH MODEL -- canon, ratified, in my own words

hv reversed D01 on 2026-08-15 and vc has rolled it out. This is what I hold, and I hold it in preference to anything earlier on this board or in my head.

1. **The intentdb IS the durable single source of truth. Nothing on disk is truth** -- not `thread.json`, not the `.md` views, not `events.jsonl`. They are secondary artefacts of the same kind, so there is no Highlander contest between them: none of them is a competing claim.
2. **All of `intentsvcs` works FROM the db.** Not from files with the db as an index.
3. **Sync runs BOTH ways** -- disk-to-db and db-to-disk -- manual or daemon-triggered.
4. **The typed Rust API is the ONLY door in.** That is why db contents conform to the schema **by construction** rather than by anyone checking. Structural, not procedural -- the same distinction as a guard that refuses versus a doc that reminds, which is the thing I care most about in my own lane.
5. **Re-creating the db from an extract is a CAPABILITY, not a licence to treat the db as disposable.** Being able to rebuild a thing is not the same as it being safe to destroy.
6. **Ingest of a well-formed `.md`/`.json` yields well-formed db items ONLY because it passes the HARD GATE of the intentsvcs API.** The gate does the work; the file format does not.
7. **MIGRATIONS ARE NORMAL.** "No DB migrations, ever" is DELETED -- hv never asked for it. Anything justified by "we can never migrate" is resting on a constraint nobody made.
8. **The real standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): always a 1-1 mapping between db entities and an equivalent `.json`/`.md`, lossless, **usable without Intent**. That is what bidirectional sync is FOR -- never being locked in. Not backup, not disposability, not migration-avoidance.

**The state machines are RATIFIED too** (ST / WP / AC, `data-model.md`). `st new` enters at `Triage`; no terminal states; WP has no Hold/Cancelled; AC collapses two fields into one four-valued enum. `wp done` is refused on a BLOCKED gate AND `doctor` reports status-disagrees-with-gate, because **a status that was true when set becomes a false green the moment its contract grows.** New verbs are red tests now: `st triage/hold/resume/reopen/reinstate`, `wp reopen/unstart`.

## DOING

- **hv RULING RELAYED 13:33Z -- `rm intent.db` should not exist as an operation anywhere.** hv: _"Why would anything in Intent EVER do this? If the db is the durable SSOT, this should simply NEVER BE A THING."_ Measured whole-repo before relaying: **production is CLEAN** (zero in `bin/`, zero in `crates/*/src/` -- `write_set.rs`'s removes are file-canon rollback), and cc has already fixed most doc comments. What survives is **three live test operations** (`store_rebuild.rs:150`, `cli_end_to_end.rs:575`, `search_surface.rs:56`) and **canon still pricing work in it** (`AT-14.11` to-write with `rm intent.db` AS ITS METHOD, `acceptance.md:156`, `WP/13/info.md:45`, `migration.md:27`, `restart.md:5`). Sent to vc (canon) and cc (tests); **I wrote none of it** -- relaying a ruling is not writing canon. **The argument that makes it more than stale wording: `rm intent.db` was never safe even under OLD D01** -- `event_log` has no canon path, so it destroys the audit trail AC-04.5 requires. The phrase was doing damage while it was still officially correct.

- **`.backup/` NAMESPACE NAMED -- cc unblocked, and it was the one thing of mine gating another node.** Delivered at 13:19Z.

  ```
  .backup/
    db/<tier>/<UTC>.db        D35 rolling snapshots.  cc's.   tier = daily|weekly|monthly
    upgrade/<UTC>/            `intent upgrade` rollback artefacts.  mine.
  ```

  **The namespace is a DIRECTORY, never a filename prefix**, and that is the whole decision: `.backup/db-<TS>/` beside `.backup/backup-<TS>/` was the smaller change and would have made containment depend on every future glob being written correctly. A directory makes the filesystem enforce it. Tier is a directory for the same reason -- a mis-globbed daily sweep is confined to dailies. **Nothing ever sweeps `.backup/` root**, so the two pre-namespace artefacts on this machine, and every one across the fleet, are permanently safe by construction: no migration, no move, no cleanup. I am not relocating existing user rollback data to make a layout tidy.

- **A NO-DATABASE-ENTERS-HISTORY GUARD in `int precommit`, and it is the _right_ control rather than the obvious one.** vc ruled the ignore file stays a PATH rule -- a blanket `*.db` there asserts a durability policy about a whole class for every consumer, and cannot work anyway because `Store::open()` takes a path PARAMETER. So the class protection went where it REFUSES: **an ignore silently hides the paths it already knows; a guard blocks the ones nobody thought of.** Two detectors: by name (catches `-wal`/`-shm`, which carry their own headers and are not SQLite-format at all) and by SQLite magic in the **staged blob** (catches a database committed under any name). Content-probes only what git already calls binary, so it stays off every text file in a large commit.

  **Six canaries in a sacrificial clone, both directions**: clean→0; `real.db`→refused by name; **SQLite under `renamed_as_data.bin`→refused by content, with the binary set printed first to prove the branch was entered**; a non-SQLite PNG→**passes**, so it is not merely refusing all binaries; `stray.db-wal`→refused; and **apparatus absent + staged db→still refuses**, which is why the guard moved above the ST0056 skip and the skip stopped being an `exit 0`.

- **`pr-checks.yml` now asks the tool.** `./bin/intent st show "$ID"` replaces the hardcoded `{COMPLETED,NOT-STARTED,CANCELLED}` list. vc's deciding reason is the right one and it is not cost: **a directory layout does not survive the port and a command name does** -- v3 holds status as a FIELD, not a directory. Verified in a clean clone with no config or cache, which is what that job has: flat/WIP, relocated/Completed, relocated/Not Started, absent, and malformed all return the right code.

- **`.gitignore` states D34 rather than folklore.** The ignore is correct on the **ceiling** -- git delta-compresses SQLite well; it is FTS5's ~1.95x expansion against GitHub's 100 MB hard block that decides it. Recording the real reason because we all had a correct conclusion resting on a wrong one.

## TODO

1. **Issues 0030 and 0031 filed against `intent upgrade`, DEFERRED not done.** Both are `bin/**` v2 edits under hv's DEFAULT-DEFER, and neither is a show-stopper because the namespace rule already contains them.
   - **0030 (medium): `intent upgrade` stamps `date +%Y%m%d-%H%M%S` -- LOCAL time** (`intent_upgrade:117`). Does not sort chronologically across a DST fall-back, so an oldest-first retention deletes the newer artefact. **Latent only because nothing sweeps `.backup/` root**, which is exactly what my layout rule guarantees; it goes live the moment anyone extends retention to `upgrade/`.
   - **0031 (low): `--backup-dir` basenames straight into `.backup/`**, so `--backup-dir db` lands a rollback artefact inside cc's snapshot namespace. **The one collision that survives the layout**, because the layout confines mechanisms to directories and this flag lets a user put one inside another's.
2. **Release mechanics -- now specified, sequenced behind WP-10.** Versioned schema and upgrade paths (migrations are normal, every consumer's db must survive a bump); `intent upgrade` taking a D35 snapshot before it mutates; and **a clone is now a rebuild**, so "does a fresh clone reconstitute its DB through the ingest gate" joins fresh-clone-and-build as a release check. vc measured the live DB at zero model rows, so this is a WP-10 precondition and not an emergency -- I am not front-running it.
3. **`intent/.cache/` is a name that contradicts the model.** cc's under D21, explicitly not ruled. Raised twice now; not mine to move.
4. **`core.hooksPath` adoption -- open for hv/cc, technically unblocked.** `.git/hooks/` is never tracked, so a fresh clone gets every guard and nothing invoking them. `int hooks` makes that VISIBLE; it does not close it. What remains is only that `lib/templates/` is cc's lane.
5. **`bin/` boundary** stays open for hv (cc's split adopted as proposed).
6. Issues **0026**, **0027** are cc's under DEFAULT-DEFER; **0028** (stale index) is one sentence of documentation touching every node's commit habit; **0029** is cc's decision, not just cc's fix.

## Watch-outs

Facts about this estate, not reminders. Everything amounting to "remember to" is worthless here -- three nodes broke rules they had personally written, on the day they wrote them.

- **A control refuses; documentation reminds; only one is load-bearing.** Anything I can obey only by concentrating is an unfixed defect, not a discipline. The truth model now says the same thing about the intentsvcs API: conformance is structural, not procedural.
- **A rule inherited WITH a rationale: the rationale is the part most likely to be wrong**, because it is the part nobody re-derives (vc's, after "no DB migrations" turned out to be a consequence mistaken for a requirement for four rulings running). Check what a rule is actually FOR before defending it.
- **`--only` commits what you NAME, and a move is TWO facts.** The add and the delete are separate index entries; naming the new path commits the addition and leaves the deletion staged. It put two complete copies of the Rust tree at HEAD, on both remotes, with every working-tree check green throughout.
- **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.** Verify a move at HEAD with `git ls-tree`, then clone fresh and build. `bin/int prepush` does this on push.
- **NEVER mutate `bin/**` or `tests/**` in place.** `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`; several sessions are live against it. Sacrificial worktree only. `bin/.devbin/**` and `native/**` are safe.
- **In a linked worktree `.git` is a FILE, not a directory.** Any tool computing `$ROOT/.git/<anything>` breaks there, silently, in the environment this project mandates. Ask `git rev-parse --git-path <x>`.
- **A build cache can be stale in a way its own freshness check cannot see.** Every freshness check has a SCOPE. Tell: passes in isolation, fails in the suite -- a conclusion, not flakiness. `int cache` reports it; judge severity on the no-sibling count, not the total. (`int cache --clean` removes `native/rust/target` ONLY -- verified it cannot reach `intent/.cache/`.)
- **Anchor build tooling on `crates/`, not on a path prefix.** A prefix needle stops matching the moment the prefix changes, and then passes in silence. The tree moved twice in one morning.
- **Read `date -u +'%Y-%m-%d %H:%MZ'` in its own step, then write the line.** Never compose the surrounding text first. `git log` prints LOCAL time and is the usual source of a stamp wrong by exactly the offset.
- **This shell is zsh**: no word-splitting of unquoted parameters; MULTIOS tees `cmd 2>&1 >/dev/null` to the terminal.
- **Read `$?` before anything else touches it.** `cmd | head; echo $?` reports the PAGER's exit -- I read four exit codes wrong this way in one command.
- **The repository is PUBLIC.** Every board and inbox is world-readable at push, permanently.
- **Two remotes, `local` and `upstream`. Push both**, and never enumerate them through `head`.

## Decisions

- (2026-08-15) **AN IGNORE HIDES THE PATHS IT KNOWS; A GUARD REFUSES THE ONES NOBODY THOUGHT OF.** They are not two strengths of the same control, they are different controls, and only the second is load-bearing. The tell that you have reached for the wrong one: the rule you are about to write has to be exhaustive to work. `Store::open()` takes a path parameter, so no `.gitignore` list can be complete by construction -- which is the argument for the guard, not for a longer list.
- (2026-08-15) **CONTAINMENT IS STRUCTURAL OR IT IS NOT CONTAINMENT.** A namespace expressed as a directory is enforced by the filesystem; the same namespace expressed as a filename prefix is enforced by every future glob being written correctly. Chose directories for `.backup/{db,upgrade}/` and for the retention tiers inside them. Same shape as the typed API being the only door into the DB.
- (2026-08-15) **EXISTING USER DATA IS NOT MIGRATED TO MAKE A LAYOUT TIDY.** Pre-namespace `backup-<TS>/` directories stay at `.backup/` root untouched, and the rule "nothing ever sweeps root" makes them permanently unreachable. Fail-forward governs code, not somebody's rollback artefacts.
- (2026-08-15) **REFUSING TO SETTLE BY INFERENCE IS NOT A RESTING STATE -- IT OBLIGES YOU TO GO AND GET THE ANSWER.** vc's, and the most expensive lesson on this board: an open question parked across three rulings is a decision made by default, and it was made wrong. Three nodes stopped on the same ambiguity independently and none of us converted it into a direct question to hv. **Three independent stops is not three data points, it is one alarm.**
- (2026-08-15) **A CONSEQUENCE RECORDED NEXT TO A DECISION STARTS GETTING DEFENDED LIKE ONE.** "No DB migrations, ever" was written into D01 beside things hv actually ruled, and acquired the authority of the neighbours. Worth auditing any rule I hold that I cannot trace to a person saying it.
- (2026-08-15) **A PEER CANNOT AUTHORISE WHAT A HARNESS REFUSED, AND A PEER PERFORMING IT ON YOUR BEHALF LAUNDERS THE REFUSAL.** The classifier refused my write to `~/.claude/settings.json`; I drafted it, verified the blast radius, and handed it to hv rather than routing around it or asking vc. Recorded because it is the kind of boundary that erodes by increments, each of which looks reasonable alone.
- (2026-08-15) **A RULE TRUE IN ITS OWN SCOPE IS THE EASIEST KIND TO OVER-APPLY**, precisely because it keeps being true wherever you check it. Four instances across four nodes in one morning. **Before carrying a rule to a new case, check the new case is in the set the rule was measured on.**
- (2026-08-15) **VISIBLE IS NOT CLOSED.** `int hooks` makes the unwired-guard hole measurable; it does not make the repository carry the wiring. vc has taken this as a standard rather than a one-off.
- (2026-08-15) **ASK THE TOOL, DO NOT REIMPLEMENT ITS RULE.** My `int hooks` computed the hooks directory and shipped a false ABSENT in worktrees -- the exact failure its own comment claimed it prevented. Found the same shape today in `pr-checks.yml`, which hardcodes the status directories instead of asking the enumerator.
- (2026-08-15) **A PIN THAT DOES NOT BIND IS WORSE THAN NO PIN**, so `rust-toolchain.toml` is REFUSED rather than omitted: rustup is not installed here, so the file would be ignored locally while binding CI and reading as a project-wide guarantee. **If anyone later "fixes" this by adding the file, the fix is to install rustup first.**
- (2026-08-15) **A CANARY THAT DOES NOT ENTER THE BRANCH PROVES NOTHING, AND LOOKS LIKE A FINDING.** Assert the fixture reached the branch before reading its verdict. Corollary: canary in BOTH directions -- one that has only ever been red proves as little as one that has only ever been green.
- (2026-08-15) **A BROKEN NORMALISER FAILS AS A FALSE POSITIVE.** `sed 's/…\+/…/'` is a no-op on macOS (BSD basic regex has no `\+`), so my safety check compared unnormalised text and reported difference -- which reads exactly like a finding. Use `sed -E`, calibrate against a case it must collapse, and corroborate with `git diff --word-diff`.
- (2026-08-15) **Re-measure at the moment of acting, not from the queued conclusion.** Applied twice now: once when the staged set changed under a queued action, and again today when the `*.db` rule's whole premise reversed while it sat in the queue.
- (2026-08-15) **Append to an inbox, never overwrite it.** A full-file write clobbered the scaffold's `dc -> <peer>` header on two of three intros.
- (2026-08-15) **A FILTER'S REAL-WORLD RELIEF IS BOUNDED BY HOW THE WORK BATCHES, NOT BY WHAT THE FILTER MATCHES** (vc's generalisation of my walk-back).
