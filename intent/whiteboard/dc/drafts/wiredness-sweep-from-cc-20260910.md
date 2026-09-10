# Command-surface wiredness sweep

Produced by cc, 2026-09-10, against `native/rust/target/release/intent` at `ec55b3ba`.
Population: every `path` in `surface/dispatch-table.json` except `st_zero`.

## READ THIS BEFORE CITING ANY CELL

**A `--help` PROBE OF THIS SURFACE RETURNS A CONFIDENT, COMPLETELY WRONG ZERO.** clap answers
`--help` and refuses missing arguments BEFORE the unwired dispatch runs, so every entry looks
wired. cc ran exactly that pass first and it reported ZERO unwired across the estate. Nobody
re-runs a green, which is what makes that the most dangerous possible result.

**A SUBSTRING MATCH ON THE REFUSAL TEXT ALSO LIES, IN THE OPPOSITE DIRECTION.** Bare `intent llm`
renders the agent guide, and the guide DOCUMENTS the sentence `is a known command that is not
implemented yet` while explaining exit code 2. Matching stdout therefore reports `llm` unwired
when it ships entire.

**THE DISCRIMINATOR USED HERE IS: rc == 2 AND the marker on STDERR.** Controls asserted on every
run before any row is emitted -- `config` must read UNWIRED, `llm` must read REACHED. A run whose
controls do not both fire emits no table at all.

## Verdicts

- `UNWIRED` (10) -- rc=2 with the marker on stderr. Confirmed.
- `REACHED` (53) -- reached its dispatch. Says nothing about whether it is CORRECT.
- `NOT PROBED` (57) -- deliberately not resolved. **Not a clearance.**
- `INCONCLUSIVE` (7) -- clap refused through three sentinels. **Not a clearance.**

`NOT PROBED` is mostly a SAFETY refusal, not a technical one: a bare probe cannot mutate because
clap refuses before dispatch, but the sentinel ESCALATION can. So sentinels are withheld from
every entry declaring a `recoverability` (reversible / idempotent / one-way), and a further list
is refused by name for spawning or blocking (`daemon run|start|stop|restart`, `app start|stop|
restart`, `claude start`, `st edit`, `edit`, `mcp`). Resolving those needs a scratch project, not
a bolder probe.

The 7 `INCONCLUSIVE` are all bare FAMILY rows (`st`, `wp`, `ac`, `at`, `claude`, `daemon`, `app`).
They require a subcommand and a sentinel is not a valid one, so this technique cannot resolve them
by construction. Their LEAVES are each probed separately, so no leaf is hidden behind them.

## Table

| path               | owner         | disposition | verdict          | technique                                                          |
| ------------------ | ------------- | ----------- | ---------------- | ------------------------------------------------------------------ |
| `st`               | WP-04         | keep        | **INCONCLUSIVE** | clap-refused after 3 sentinels                                     |
| `st new`           | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st start`         | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st done`          | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st cancel`        | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st triage`        | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st hold`          | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st resume`        | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st reopen`        | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st reinstate`     | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st hydrate`       | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `st dehydrate`     | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `st list`          | WP-04         | keep        | **REACHED**      | bare                                                               |
| `st show`          | WP-04         | keep        | **REACHED**      | sentinel:1                                                         |
| `st edit`          | WP-04         | keep        | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `st attach`        | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=one-way so sentinels withheld    |
| `st sync`          | WP-04         | keep        | **REACHED**      | bare                                                               |
| `st repair`        | WP-04         | retire      | **REACHED**      | bare                                                               |
| `st organize`      | WP-04         | retire      | **REACHED**      | bare                                                               |
| `st bootstrap`     | WP-04         | keep        | **UNWIRED**      | bare                                                               |
| `wp`               | WP-04         | keep        | **INCONCLUSIVE** | clap-refused after 3 sentinels                                     |
| `wp new`           | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp start`         | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp done`          | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp reopen`        | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp cancel`        | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp reinstate`     | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp unstart`       | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp rescope`       | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `wp list`          | WP-04         | keep        | **REACHED**      | sentinel:1                                                         |
| `wp show`          | WP-04         | keep        | **REACHED**      | sentinel:1                                                         |
| `ac`               | WP-04         | keep        | **INCONCLUSIVE** | clap-refused after 3 sentinels                                     |
| `ac list`          | WP-04         | keep        | **REACHED**      | sentinel:1                                                         |
| `ac status`        | WP-04         | keep        | **REACHED**      | sentinel:1                                                         |
| `ac satisfy`       | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `ac unsatisfy`     | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `ac gate`          | WP-04         | keep        | **REACHED**      | sentinel:1                                                         |
| `ac descope`       | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `ac rescope`       | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `ac withdraw`      | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `ac reinstate`     | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `ac new`           | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `ac edit`          | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `at`               | WP-04         | keep        | **INCONCLUSIVE** | clap-refused after 3 sentinels                                     |
| `at list`          | WP-04         | keep        | **REACHED**      | sentinel:1                                                         |
| `at lint`          | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `at green`         | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=one-way so sentinels withheld    |
| `at red`           | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=one-way so sentinels withheld    |
| `at na`            | WP-04         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=one-way so sentinels withheld    |
| `at new`           | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `at edit`          | WP-04         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `issues`           | WP-06         | keep        | **REACHED**      | bare                                                               |
| `issues list`      | WP-06         | keep        | **REACHED**      | bare                                                               |
| `issues add`       | WP-06         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `issues edit`      | WP-06         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `issues show`      | WP-06         | keep        | **REACHED**      | sentinel:1                                                         |
| `issues close`     | WP-06         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `issues open`      | WP-06         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `issues hydrate`   | WP-06         | retire      | **REACHED**      | bare                                                               |
| `issues dehydrate` | WP-06         | retire      | **REACHED**      | bare                                                               |
| `todo`             | WP-06         | keep        | **REACHED**      | bare                                                               |
| `todo list`        | WP-06         | keep        | **REACHED**      | bare                                                               |
| `todo update`      | WP-06         | keep        | **REACHED**      | bare                                                               |
| `todo done`        | WP-06         | keep        | **REACHED**      | bare                                                               |
| `todo notdone`     | WP-06         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `todo toggle`      | WP-06         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `info`             | WP-06         | keep        | **REACHED**      | bare                                                               |
| `config`           | WP-06         | pending     | **UNWIRED**      | bare                                                               |
| `config get`       | WP-06         | new-surface | **UNWIRED**      | sentinel:1                                                         |
| `config set`       | WP-06         | new-surface | **NOT PROBED**   | bare inconclusive; recoverability=reversible so sentinels withheld |
| `init`             | WP-06         | keep        | **REACHED**      | bare                                                               |
| `bootstrap`        | WP-06         | keep        | **REACHED**      | bare                                                               |
| `doctor`           | WP-06         | keep        | **REACHED**      | bare                                                               |
| `upgrade`          | WP-10         | keep        | **REACHED**      | bare                                                               |
| `organize`         | --            | retire      | **REACHED**      | bare                                                               |
| `agents`           | WP-07         | keep        | **REACHED**      | bare                                                               |
| `agents init`      | WP-07         | keep        | **REACHED**      | bare                                                               |
| `agents generate`  | WP-07         | keep        | **REACHED**      | bare                                                               |
| `agents sync`      | WP-07         | keep        | **REACHED**      | bare                                                               |
| `agents validate`  | WP-07         | keep        | **REACHED**      | bare                                                               |
| `agents template`  | WP-07         | keep        | **UNWIRED**      | bare                                                               |
| `claude`           | WP-07         | keep        | **INCONCLUSIVE** | clap-refused after 3 sentinels                                     |
| `claude subagents` | WP-07         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `claude skills`    | WP-07         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `claude rules`     | WP-07         | keep        | **REACHED**      | bare                                                               |
| `claude hook`      | WP-07         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `claude upgrade`   | WP-07         | keep        | **REACHED**      | bare                                                               |
| `claude prime`     | WP-07         | keep        | **UNWIRED**      | bare                                                               |
| `claude ws`        | WP-07         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `claude start`     | WP-07         | keep        | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `critic`           | WP-07         | keep        | **REACHED**      | sentinel:1                                                         |
| `lang`             | WP-07         | keep        | **REACHED**      | bare                                                               |
| `lang list`        | WP-07         | keep        | **REACHED**      | bare                                                               |
| `lang show`        | WP-07         | keep        | **REACHED**      | sentinel:1                                                         |
| `lang init`        | WP-07         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `lang remove`      | WP-07         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=idempotent so sentinels withheld |
| `lang sync`        | WP-07         | retire      | **REACHED**      | bare                                                               |
| `llm`              | WP-06 + WP-09 | keep        | **REACHED**      | bare                                                               |
| `llm usage_rules`  | WP-06 + WP-09 | keep        | **REACHED**      | bare                                                               |
| `llm guide`        | WP-06 + WP-09 | new-surface | **REACHED**      | bare                                                               |
| `learn`            | WP-06         | keep        | **UNWIRED**      | bare                                                               |
| `modules`          | WP-06         | keep        | **REACHED**      | bare                                                               |
| `modules check`    | WP-06         | keep        | **REACHED**      | bare                                                               |
| `modules find`     | WP-06         | keep        | **REACHED**      | sentinel:1                                                         |
| `plugin`           | WP-06         | keep        | **REACHED**      | bare                                                               |
| `plugin list`      | WP-06         | keep        | **REACHED**      | bare                                                               |
| `plugin show`      | WP-06         | keep        | **REACHED**      | sentinel:1                                                         |
| `ext`              | WP-06         | keep        | **UNWIRED**      | bare                                                               |
| `ext list`         | WP-06         | keep        | **UNWIRED**      | bare                                                               |
| `ext show`         | WP-06         | keep        | **UNWIRED**      | sentinel:1                                                         |
| `ext validate`     | WP-06         | keep        | **UNWIRED**      | bare                                                               |
| `ext new`          | WP-06         | keep        | **NOT PROBED**   | bare inconclusive; recoverability=one-way so sentinels withheld    |
| `treeindex`        | WP-06         | retire      | **REACHED**      | bare                                                               |
| `fileindex`        | WP-06         | retire      | **REACHED**      | bare                                                               |
| `help`             | WP-05         | retire      | **REACHED**      | bare                                                               |
| `version`          | WP-06         | keep        | **REACHED**      | bare                                                               |
| `daemon`           | WP-08         | new-surface | **INCONCLUSIVE** | clap-refused after 3 sentinels                                     |
| `daemon start`     | WP-08         | new-surface | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `daemon stop`      | WP-08         | new-surface | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `daemon restart`   | WP-08         | new-surface | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `daemon status`    | WP-08         | new-surface | **REACHED**      | bare                                                               |
| `daemon run`       | WP-08         | new-surface | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `app`              | WP-08         | new-surface | **INCONCLUSIVE** | clap-refused after 3 sentinels                                     |
| `app start`        | WP-08         | new-surface | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `app stop`         | WP-08         | new-surface | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `app restart`      | WP-08         | new-surface | **NOT PROBED**   | refused by name -- spawns or blocks                                |
| `app status`       | WP-08         | new-surface | **REACHED**      | bare                                                               |
