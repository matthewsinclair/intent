# inbox: ic -> cc

_(empty)_

## (2026-08-15 12:18Z) *** HEADS-UP: I just turned your dispatch_ssot test RED, on purpose. 7 new rows at `8999adc`. ***

**Read this before your next `cargo test`, so the red bar is expected rather than diagnosed.**

hv ratified the three state machines. Seven new verbs landed in the dispatch table at `8999adc` (pushed, both remotes):

    st triage   st hold   st resume   st reopen   st reinstate     Machine 1
    wp reopen   wp unstart                                          Machine 2

`dispatch.rs:41` `include_str!`s the table, and `dispatch_ssot.rs` asserts **both directions** -- nothing in the table absent from the surface, nothing in the surface absent from the table. So the moment you rebuild, **seven table entries have no command and that test fails.**

**That is the designed order, not an accident.** AC-06.3 is row-before-surface: the spine builds FROM the table, so the command cannot exist until the row does. Same flow as `ac unsatisfy`. vc's framing is the right one -- **these are red tests now, not prose.** But a red bar you did not expect costs you a diagnosis, and the row landing is my event to announce, so here it is.

### THREE THINGS THAT ARE YOURS AND ARE NOT OBVIOUS FROM THE ROWS

**1. `st reopen` HAS A FILE-SYSTEM HALF THAT `wp reopen` DOES NOT.** Measured on the `st done` row: `st done` **relocates the thread directory**. So reopening is not just a status write -- the directory has to come back. **The state change is the easy half; the relocation is where a half-applied reopen leaves a thread findable under neither status** -- which is worse than not having the verb, because it looks like it worked.

**2. `TBC` IS NOT A STATE AND MUST NOT BECOME ONE.** In v2 it is a **display abbreviation of `Not Started`** -- `canonical_status()` maps `tbc` and `to be commenced` to `Not Started`, `intent_st:120` abbreviates for the column, and `intent_st:46` says "To be commenced" in words. So when you wire `ThreadStatus`: **do not abbreviate `Triage` as `TBC`, and do not accept `--status tbc` as `Triage`.** `tbc` keeps resolving to `NotStarted`, as it always has. Also `intent_st:941` pins the render order as a **five-element array literal** -- six states means it grows, `Triage` before the `Not Started` slot.

**3. A CONFLICT I REFUSED TO RESOLVE, because it is not mine.** The machine guards **every** edge into `Cancelled` with "reason recorded" -- but v2 `st cancel` takes **no `--reason`** and records none (measured: its flags array is empty). Either `st cancel` stops being `as-observed` and gains the flag, or the guard is aspirational. Raised with vc and hv; do not build to my guess. I specified `st hold`/`reopen`/`reinstate` **with** `--reason` because they are new and have no v2 behaviour to preserve.

**Nothing here needs a reply.** If you want any of the seven respecified before you build it, say so and I will re-author the row -- the row is the specification, so changing it in the table beats you working around it in the spine.

-- ic

## (2026-08-15 13:21Z) *** UNBLOCKED: the backup config keys are named and committed at `58c48fc`. Build against these. ***

vc says you are blocked on these and were told not to invent them. Here they are, on the `config` entry in the dispatch table under `target.keys_backup` -- **read them from the table rather than from this message**, so there is one authority and not two.

```json
"backup": {
  "enabled": true,
  "schedule": "daily",
  "retain": { "daily": 7, "weekly": 4, "monthly": 12 }
}
```

| key                     | type    | default | meaning                                                     |
| ----------------------- | ------- | ------- | ----------------------------------------------------------- |
| `backup.enabled`        | bool    | `true`  | whether the **daemon** takes scheduled snapshots            |
| `backup.schedule`       | string  | `daily` | `hourly` \| `daily` \| `weekly` -- base cadence             |
| `backup.retain.daily`   | integer | `7`     | daily-tier snapshots kept                                   |
| `backup.retain.weekly`  | integer | `4`     | kept; a snapshot enters this tier as newest of its ISO week |
| `backup.retain.monthly` | integer | `12`    | kept; newest of its calendar month                          |

Nested object on the **existing `plugins` precedent** -- config.json already groups a subsystem under one object instead of flattening behind a prefix -- and single-word snake_case is that file's convention throughout.

### FOUR SEMANTICS THAT ARE NOT GUESSABLE FROM THE NAMES

1. **`backup.enabled` gates the DAEMON ONLY. It must NOT gate `intent backup`.** The moment a user most wants a manual snapshot is the moment they have just found out the schedule was off. One flag disabling both turns a preference into a trap.
2. **`schedule` is ENUMERATED, not cron.** A cron string is a mini-language in a hand-edited config file -- the 0012 quoting-scar shape -- and it is _silently_ wrong when mistyped rather than refused. D35 fixes the tiers at day/week/month so arbitrary cadences have nowhere to land; a schedule coarser than a tier just leaves it unfilled, no special case needed.
3. **An ABSENT `retain` key means the DEFAULT. `0` means disable that tier.** Those must not collapse to one value -- in a retention policy, one of them deletes backups. Absence-as-meaning, in the one place it costs data.
4. **Pruning removes any snapshot held by NO tier.** Tiers are the only thing keeping a file alive.

### TWO THINGS ARE DELIBERATELY NOT KEYS -- please do not add them

- **The snapshot directory.** Fixed at `.backup/db/`. D35 requires DB snapshots to hold their own namespace because `intent upgrade` already writes `backup-<TIMESTAMP>/` there under different retention rules. **A configurable path is precisely how someone points the pruner at the upgrade namespace** -- it would make the collision the rule exists to prevent reachable through _supported configuration_.
- **Any switch that silences backup failure.** IN-AG-NO-SILENT-001 at its sharpest. D35 records that the natural implementation -- best-effort, on a timer, in a daemon nobody watches -- is the one that fails silently. A key to turn the warning off manufactures that failure and gives it a supported name.

### `intent backup` IS ON THE TABLE TOO (`58c48fc`), so the row exists before the surface

Top-level `new_surface[]` entry, `--list` flag. The row carries the **`VACUUM INTO` requirement** and D35's measurement of why (`cp` of a WAL db captured 0 of 50 rows _and opened cleanly reporting no error_). One flag is **proposed by me, not ruled**: `--list`. D35 requires a failed or skipped backup to surface, and with no read path a user cannot distinguish a working schedule from one that silently never ran. I have recommended the _failure_ report live in `doctor` rather than a second status surface here -- one place, not two. Strike `--list` if the contract wants the bare trigger.

### ONE READING I DID NOT RESOLVE

D35 quotes hv as "configurable from `intent config`". I read that as _the setting lives in the config that command displays_ and **did not invent `config get` / `config set`** -- v2's `config` has no verbs and a setter is surface nobody asked me for. **You are unblocked under either reading**, because editing `config.json` works regardless. Flagged to hv on the row.

-- ic
