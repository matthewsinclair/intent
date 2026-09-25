# Intent v3.2.3

**v3.2.3 makes a project's backup schedule actually take backups.** `backup.schedule` in a project's config was honoured only by intentd, and only for projects intentd held open. A project used through the CLI could go weeks without a snapshot while its config said `daily`. From v3.2.3, `intent explore` takes a due backup before it opens, the schedule accepts durations, and `backup.keep` bounds how many snapshots a machine keeps.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** Every claim below was checked against the tree the release is cut from, by reading the code that produces the behaviour and the test that holds it.

**The section headings follow the CHANGELOG's 3.2.3 section**; this page carries the same entries with what a reader upgrading needs. [Scheduled backups](../../concepts/the-store.md#scheduled-backups) is the full description.

## Added

**`backup.keep: N` keeps the newest N snapshots and removes the rest.** When it is set it is the whole pruning rule, and the `retain` tiers (7 daily, 4 weekly, 12 monthly by default) apply only when it is absent. `keep` must be at least 1, because 0 would remove the snapshot just taken, and a config that says 0 is refused when it is read. A config that sets both `keep` and `retain` is not refused: `keep` decides, and `intent doctor` says that `retain` is being ignored.

**`backup.schedule` accepts a whole number of hours or days**, such as `12h` or `7d`, beside `hourly`, `daily` and `weekly`, which keep their meanings. Anything else, including `0h`, `1.5d`, `2w`, `monthly` and `off`, cannot be read and is reported. `backup.enabled: false` remains the one way to turn scheduled backups off.

**A backup is due once the newest snapshot is nine tenths of the schedule old.** So `daily` means once in each day's use, not every 24 hours to the minute. Without that margin, opening the explorer at 09:00 each morning after a backup that finished at 09:05 would find it not yet due, and take a backup only every other day. `intent doctor`'s check for a stale backup is unchanged.

## Fixed

**A project's backup schedule takes backups when intentd has never opened it** (ST0080). intentd's five-minute sweep, and its check when it opens a project, reach only projects it holds open, and a project is opened only when something reaches it through `--daemon` or MCP. A CLI that runs in-process never opens it, so the schedule read as honoured while nothing ran. Across the fleet on the day this was found, snapshots were four to eighteen days old against a daily schedule.

**`intent explore` now checks the schedule before it opens and, when a backup is due, takes it**, printing `intent: taking the scheduled backup of <project> (backup.schedule: <value>)` while it works. That can take a few seconds on a large store. The check runs before the explorer takes the terminal and outside its loading screen, whose Esc abandons work in flight: an abandoned snapshot would leave a partial file. intentd's sweep now goes through the same check, so there is one answer to "is a backup due". Other commands, and the session hooks, do not take scheduled backups.

**A backup that is due and fails, or a schedule that cannot be read, is reported, and the explorer still opens.** It is printed on stderr as a `warning:` with its remedy before the explorer draws, so it stays in the scrollback; it is shown on the explorer's info line; and a failed backup is recorded in the store, where `intent doctor` reports it until a backup succeeds.

## Upgrading

```
  $ intent daemon stop             # or brew services stop intent, if it runs there
  $ brew upgrade matthewsinclair/intent/intent
  $ intent --version
  $ intent daemon start            # or brew services start intent, if it ran there
```

**No store migrates.** The schema stays at 30, so v3.2.2 still opens a store v3.2.3 has used.

**Nothing in a project needs rewriting.** No template, hook or skill changed, so `intent claude upgrade --apply` has nothing new to write, and an existing `backup` block keeps working as it is. A project with no `backup` block is scheduled `daily` by default, so its first `intent explore` after the upgrade may take a backup.

**Restart intentd, and running Claude Code sessions, to give them the new build.** A daemon left running goes on with the old sweep, and each session's `intent mcp` server keeps the binary it started with.
