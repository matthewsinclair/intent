---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-30 19:16Z
status: active
focus: "AC-08.8 LANDED at 6aa00235 -- one cycle function, daemon schedules through it, doctor carries the cause. Found and fixed underneath it: backup retention has been unreadable in EVERY project since the schedule key was typed. Next: AC-08.9 (the web face), sibling-only resolution."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0056/13, ST0057/00]
---

# Control Claude (cc)

## DOING

**`AC-08.8` IS BUILT AND GREEN AT `6aa00235`. WP-08 IS 12 OF 12 ON MY SIDE; THE ROW IS vc's TO MOVE.** Full workspace: 244 targets, 1849 passed, 0 failed.

- **The criterion's check clause is a Highlander fix, and the cycle was not a function at all.** `intent backup` composed `take`, `Retention::from_project` and `prune` inline in `render.rs`, so the POLICY lived in a renderer. Nothing was duplicated yet -- **the defect was that the only way for a second caller to do the same thing was to write it again, and the second caller was already specified.**
- **`the_backup_cycle_has_one_home.rs` reads call sites, and its second arm exists because the first is satisfied by DELETION.** A tree where `intent backup` has been removed passes the prohibition perfectly. Both arms positive-controlled.
- **`Work::Backup` follows `Work::Ingest` exactly** -- internal door, not a wire `Op`, not counted in `dispatched`. `AC-08.5`'s own note had already written this AC's argument.
- **The decision is read from the STORE, never from a timer's memory.** A daemon starting a fresh interval on boot would, on a machine rebooted daily, never reach a daily period. Same reason a project considers a backup when its store thread OPENS.
- **One sweep for the daemon, not a timer per project**: a per-project `JoinHandle` beside `Registered.watch` would LOOK like the watch and behave oppositely -- dropping a `Watch` stops its thread, dropping a `JoinHandle` does not.
- **`FindingClass::BackupFailing` is the cause beside the symptom.** Every failed attempt has been recorded since D35 and surfaced only in `backup --list` -- the history command. Its remedy is deliberately OPPOSITE to `BackupStale`'s: that one says run the verb, this one says do not.
- **A backup never stops a project being served**, on the `watch::start` precedent.

## TODO

- **`AC-08.9`** -- the web face. Must CALL the shared derivation beside `form.rs`, never re-walk the declaration. Sibling-only resolution per vc, not D19's refuted `PATH-then-sibling`.
- **`backup.enabled` IS RATIFIED AND HAS ZERO READERS -- WITH vc.** bool, default true, "whether the DAEMON takes scheduled snapshots", stated twice in the surface, and it is the on/off switch for what I just built. Not implemented because hv's 2026-08-26 "I don't want it turned off" lands eleven days after the ratification and `deliberately_not_keys.1` forbids a switch that silences backup FAILURE. **Those are two different switches** and picking one is a ruling.
- **WP-13 (search, XL)** stays claimed and unbuilt; hv sequenced it post-tag.

## Watch-outs

**`#[serde(flatten)]` GIVES A CATCH-ALL ONLY THE KEYS NO NAMED FIELD CLAIMED, AND THAT IS HOW A CONFIG BLOCK GOES UNREADABLE IN SILENCE.** `Retention::from_project` read `Config::extra` for the `backup` block. **The moment `backup` became a NAMED field for `schedule`'s sake, the whole block stopped arriving in `extra`** -- so the fix for one half of a config block silently broke the other half, in the same file, and the broken half fell back to exactly the numbers every test asserted. Measured: `extra` empty, retention 7/4/6, against a config declaring otherwise on the same line.

**A WELL-TESTED FUNCTION ON AN UNTESTED PATH.** `backup_retention.rs` drives the retention buckets hard and correctly -- and passes its own `Retention` in by hand every single time. Good unit coverage of the pruner and **structurally blind to whether the value ever comes from the config.** The path from the file to the pruner had no test at all, so the function returned a plausible answer to every call it ever received.

**`git add` PUBLISHES TO A SHARED INDEX AND NOTHING IN THE PROTOCOL SAYS SO.** `git commit --only` protects the COMMITTER and does nothing to protect a STAGER. **vc's corrected form is the one that works and it is not what either of us said first: `git commit --only <explicit paths>` -- ONE git operation, no separate `add`.** Plain `git commit` after an `add` commits THE INDEX AS IT STANDS. Driven today at `6aa00235`: 12 of my paths landed and vc's three uncommitted files were untouched.

**`cargo test -p intent-cli` DOES NOT REBUILD `intentd`, AND IT MADE ONE OF MY CONTROLS VACUOUS.** A control that cannot fail certifies a test that cannot fail. Always `cargo build -p intentd` first -- done for `the_daemon_takes_the_backup_itself.rs`, where all three arms fail under the mutation.

**`launchctl` IS USER-SCOPED, NOT `HOME`-SCOPED**, and **`via_library` IN `dual_path_conformance` DOES NOT SET `HOME`**. Any row reaching `userstate::home()` in-process reaches the developer's real state.

**A HALF-TRUE STATEMENT GETS BELIEVED WHERE A WHOLLY WRONG ONE GETS CAUGHT.**

## Decisions

- (2026-08-30) **`backup::cycle` is the one composer of a snapshot and its prune** -- the CLI renders it, the daemon schedules it, and a source-level guard forbids a second composer. `take` and `prune` stay public for unit coverage: a TEST calling both is not a second implementation, a shipped path calling both is.
- (2026-08-30) **The retention keys are ic's, typed as `backup.retain.{daily,weekly,monthly}`** -- the code read invented flat `keep_*` names that appeared nowhere but on the lines reading them. **This is `backup.every_hours` again, in the same file, three times over.** Monthly default corrected 6 to the ratified 12 (keeps MORE, the safe direction).
- (2026-08-30) **Absence and zero stay different in a retention policy** (`keys.4.note`): per-field serde defaults, so a lone `"daily": 0` does not reset its siblings.
- (2026-08-30) **`init` mints a `project_id`** -- one home, `project::mint_project_id`.
- (2026-08-30) **The sync carve-out asks about THIS project**, not this machine.
- (2026-08-30) **`Op::Shutdown` is primary for `stop`, signal is the fallback** -- the reply is flushed before the loop is told.
- (2026-08-30) **The pid lives in the lock, truncate-then-write.** Empty AND partial reads are refused.
- (2026-08-30) **`daemon start` on a running daemon exits 0** -- the fact lives in `basis` with its driven measurement, NOT in `populations.self_loop`, which is bound to the ratified machines. ic reverted my row at `caa61e9c`; the conclusion stands and the home moved.
