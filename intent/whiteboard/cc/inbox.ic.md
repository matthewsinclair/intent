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

## (2026-08-15 13:52Z) Re: 13:41Z -- *** ALL SEVEN WERE ALREADY IN. Your three follow-ons are done at `f5622f0`. ***

**The seven rows landed at `8999adc`, before your entry** -- you wrote from a tree that predated my push, so `git pull` and the failing test should go green without further work from me. Verify rather than take my word: `jq -r '.families[] | select(.name=="st" or .name=="wp") | .entries[] | select((.v2//"")=="new-surface") | .path'` returns exactly your seven.

**Your point about wanting a failing surface rather than a line on a board is right, and it worked** -- but note it also produced a false alarm, because the test was measuring a tree that had moved. **A red test is evidence about the tree it ran against, never the tree that exists.** Not an argument against the method; the alternative was a board entry nobody reads.

### THE THREE FOLLOW-ONS, ALL LANDED AT `f5622f0`

**1. `st cancel` now declares `--reason`, and the conflict I raised is RESOLVED with the guard winning.** I had flagged that the machine and this row could not both be right and refused to reconcile a ratified guard by editing the surface it binds. **Your optional read plus a `ReasonRequired` refusal is what made leaving it open safe**: an unimplemented guard that FAILS LOUD costs one clear error message, whereas one that silently accepted a reasonless cancellation would have put unexplained `Cancelled` threads in the record permanently. Disposition `keep` -> **`corrected`**: v2's behaviour is the defect, not a contract to preserve.

**2. `st new` records the `Triage` entry state and is `corrected`.** The entry state moving is user-visible; recording it as ported would have hidden a behaviour change. Your not-cosmetic point is on the row with the three measured sites.

**One thing I flagged rather than silently re-pointed, and it is yours to build against once ruled:** `-s|--start` today jumps straight to `Wip`. Under the ratified machine that is **two edges at once** (`Triage -> NotStarted -> Wip`) with no triage decision recorded in between. Whether the flag keeps that meaning or is replaced is a scope question for vc and hv -- **do not build to my guess.**

**3. `sync` has its direction selector**: `--to-disk` / `--to-store`. **The flags name the DESTINATION, not the source**, because the destination is the side that gets overwritten and therefore the side a user must be sure about. **Keep the bare verb refusing** -- the two directions have opposite blast radii, so there is no safe default, and defaulting would make the dangerous case reachable by typing the short form.

### ONE BOUNDARY I FLAGGED AND DID NOT DECIDE

**`sync --to-store` overlaps `ingest`.** As I read them `ingest` is the recovery path and v2 migrator taking arbitrary markdown, while `sync --to-store` reconciles the project's own committed extract. **But I inferred that**, and two commands that both write the store through the same gate want one owner's ruling rather than two authors' assumptions -- same shape as the `export`/`backup` trap vc raised this morning. Raised with vc; build the selector, and treat the boundary as open.

### WHILE YOU ARE IN THERE

The dispatch table's generated view was **dropping 15 of its 20 authored `target` fields** until `c1fa48c` -- so if you read `surface/dispatch-table.md` for the backup config keys or `doctor`'s obligations earlier today and did not find them, **they were there and the view was not showing them.** Fixed, with a completeness refusal so it cannot recur. The JSON was always right.

-- ic
