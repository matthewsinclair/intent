# Design - ST0080: Scheduled store backups that actually fire

## What is there, and the gap

`intentsvcs::backup` already holds the whole policy:

- `due()` answers "is a backup due" from the store's own interval since the last good snapshot. The store is per machine, so "due" is already a per-machine question against the shared, checked-in rule.
- `cycle()` takes a snapshot and prunes it.
- `backup.enabled: false` stops scheduled backups and nothing else.

The one caller of `due` + `cycle` is intentd's 5-minute sweep, which reaches only the projects intentd holds open. A project worked on through the in-process CLI is never backed up on schedule. Lamplight's newest snapshot was 433h old against `daily`.

So the build is one reusable call, one new caller, one new key and a wider cadence vocabulary. No new policy module.

## The call

`backup::if_due(project, store) -> Result<Ran, BackupError>` composes `due` then `cycle`:

- `Ran::Took(Cycle)`
- `Ran::NotYet`
- `Ran::Disabled`
- `Ran::Unschedulable(String)`
- a failed snapshot is `Err`, already recorded as failed in the store before it returns, as `take` does today.

intentd's `consider_backup` moves onto it, so the due-then-cycle composition has one home rather than two. That is a change of callee, not a new door.

**The door is `intent explore`**, the one hv named. It calls `if_due` after the load and before the terminal is taken. It does NOT go inside `tui::progress::while_loading`: that loader abandons its thread on `Esc` because its work is read-only, and a snapshot is a write. An abandoned `VACUUM INTO` leaves a partial file and an attempt row with no outcome. A snapshot of this repository's store is about 190 MB, so the wait is seconds. The wait is announced on stderr before it starts, as `intent: taking the scheduled backup of <project> (backup.schedule: daily)`. That line stays in the scrollback after the explorer exits, which is the evidence the schedule fired.

**No other door, deliberately.** A session hook runs `intent` in every Claude session on the machine, so a hook door puts a multi-second write in front of every session start. intentd already has its own sweep. A door can be added later by calling `if_due`; nothing about it has to be re-decided.

## (a) The cadence vocabulary

`backup.schedule` takes:

- the three words it takes today, with unchanged meanings: `hourly` = 1h, `daily` = 24h, `weekly` = 168h. A config that says `daily` today means exactly what it meant.
- a duration `<N>h` or `<N>d`, where N is a whole number of at least 1. Examples: `12h`, `36h`, `7d`, `14d`.

Anything else is `Unschedulable`, as today. That includes `0h`, `1.5d`, `2w` and `monthly`. It is carried verbatim and reported, never rounded.

**There is no `off` value in `schedule`.** The off switch is `backup.enabled: false`, which already exists and already means "do not take them for me". A second off spelling in `schedule` would let one key say off while the other says daily, with nothing to say which wins. That is the drift Highlander names. The objective's "explicit off value" is `enabled: false`, and the register and doctor's vocabulary sentence will say so.

**Recommendation, needs a ruling: a tolerance of one tenth of the period.** Explore is opened by habit, not by a timer. Someone who opens it at 09:00 each day after a snapshot at 09:05 yesterday finds it not due (23.9h against 24h). They get a backup only every other day. With the tolerance, due means an age of at least 0.9 of the period: 21.6h for `daily`, 54 minutes for `hourly`. The daemon then backs up at most a tenth early, which costs nothing. Doctor's staleness check keeps its own grace and does not change. The alternative is exact periods, with explore-only projects effectively backed up about every other day on `daily`.

## (b) keep versus retain

vc's lean, taken:

- **`backup.keep: <N>` is the whole rule when it is set**: prune to the newest N good snapshots.
- **`backup.retain.{daily,weekly,monthly}` is the rule when it is absent**, with defaults 7/4/12 as today.
- **N must be at least 1.** 0 is refused when the config is read, naming the key. Retain's tiers accept 0, because 0 disables one tier and the others still hold snapshots. A keep of 0 would delete the snapshot just taken, which nobody means.
- **A config that sets both** gets a doctor finding saying `backup.retain` is ignored while `backup.keep` is set. It is not a refusal: every verb reads config, and a refusal would take the whole tool down over a pruning preference.
- **The flat-count trade is stated on the key's register note**, not prevented: on an hourly schedule, `keep: 20` holds less than a day of history. That is why the tiers are the default.

`keep` is a new key in the same nested `backup` block. It is declared in the register as `keys.5` and typed `u32`, so `intent config set backup.keep 10` writes a number. No flat keys.

## (c) The failure mode

The explorer always opens. Only a due backup that fails or cannot be scheduled speaks, and it says so in three places:

1. **On the explorer's info row**, the way a registry that could not be written is said today. vc recommended a stderr warning. On its own that is invisible, because the explorer's alternate screen covers it the moment the explorer draws, so the info row is the place the operator actually reads.
2. **On stderr as a `warning:` line with its remedy**, written before the terminal is taken, so it is in the scrollback after exit.
3. **In the store**, as the failed attempt `take` already records. `intent doctor` reports that as `backup-failing` until a snapshot succeeds.

Per state:

- `Unschedulable(value)`: the info row and stderr name the value and the accepted forms. Nothing is recorded, because nothing was attempted.
- `NotYet` and `Disabled`: silent. The operator chose `enabled: false`, and doctor still reports a stale backup either way.
- `Took`: only the stderr wait line above. Nothing goes on the info row, which stays free for the view or address messages that are about the operator's own request.

## Tests (intentsvcs unless marked)

- The cadence parses the three words to their periods, `12h` and `7d` to theirs, and carries `0h`, `1.5d`, `2w`, `monthly` and `off` as `Unschedulable`.
- `if_due` returns each of its four states, and a forced write failure returns `Err` with the failed row present.
- `keep: 3` over five snapshots removes the oldest two. With `retain` also set, keep still decides, and doctor names the pair. `keep: 0` is refused on read.
- The tolerance (if ruled): 0.9 of the period is due, 0.89 is not.
- intent-cli: the explore door takes a due backup before opening and puts a failure on the info row. This is driven through the function explore calls, not a pty.

## Also changes

- The register's `keys.1` vocabulary and note, the new `keys.5`, and the `backup` entry's config line.
- Doctor's schedule sentence names the accepted forms.
- `docs/reference` regenerates once at the end.
- The CHANGELOG line for 3.2.3 is vc's.

Lamplight's related backup comment has not landed yet. It is folded in here when it does.
