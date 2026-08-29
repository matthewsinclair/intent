# Getting started

This walks one steel thread from nothing to satisfied. It takes about ten minutes and leaves behind a project you can keep.

Everything below runs against a real repository. Intent lives inside your project, not beside it.

## 1. Initialise

```
  $ cd your-project
  $ intent init --lang rust
```

This creates `intent/` in your repository and generates the agent contract at the root.

`--lang` declares which languages the project is in. **It is a declaration, not a detection** — Intent will not guess from the files present, because file presence is unreliable evidence and a wrong guess loads the wrong rules. Pass a comma-separated list, or add languages later with `intent lang init <lang>`.

What you get:

```
  intent/
    .config/config.json     project metadata, the declared languages
    st/                     steel threads
    docs/                   project documentation
    wip.md                  current work in progress
  AGENTS.md                 the tool-agnostic agent contract, generated
```

`AGENTS.md` is **generated from project state** by `intent agents sync`. Do not hand-edit it; the next sync will overwrite you.

## 2. Open a steel thread

A steel thread is one intention. Name the outcome, not the task.

```
  $ intent st new "Rate-limited cache for API protection"
  created: ST0001
```

Then write down why. This is the part that matters and the part everything else hangs off:

```
  $ $EDITOR "$(intent st edit ST0001 info)"
```

Fill in the objective and the context. **Be specific about constraints and about what you ruled out** — that is the information nobody can reconstruct later, and it is what an agent reading this file will act on. "Cache because the upstream API limits us to 100 req/min, and we see 10K req/s at peak" tells a reader something. "Add caching" does not.

Start it when you begin work:

```
  $ intent st start ST0001
```

## 3. Break it into work packages

Work packages are the units that get done. A thread with one work package is fine; a thread with twenty is a thread that should have been two threads.

```
  $ intent wp new ST0001 "Cache layer"
  $ intent wp new ST0001 "Rate limiter"
  $ intent wp start ST0001/01
```

`intent wp list ST0001` shows where they stand. Statuses are `not-started`, `wip`, `done` and `cancelled`, and they move with `wp start`, `wp done`, `wp cancel` and `wp reopen` rather than by editing a field.

## 4. State the acceptance criteria

**This is the step that distinguishes Intent from a task tracker.** A criterion is a condition that decides whether the intention was met — not a restatement of the work.

```
  $ intent ac new ST0001 AC-01.1 --text "Evicts oldest entries under memory pressure rather than failing writes"
  $ intent ac new ST0001 AC-01.2 --text "A warm cache survives a process restart"
  $ intent ac new ST0001 AC-02.1 --text "Requests over quota are refused, not queued"
```

Ids are caller-assigned. The convention `AC-<wp>.<n>` ties a criterion to the work package that satisfies it, and Intent does not enforce it — it is a convention that reads well, not a rule.

## 5. Back each criterion with a test

A criterion with nothing behind it is a promise. An acceptance test is what turns it into a computed fact.

```
  $ intent at new ST0001 AT-01.1 --covers AC-01.1 --file tests/cache_eviction.rs
  $ intent at new ST0001 AT-01.2 --covers AC-01.2 --file tests/cache_persistence.rs
```

A test starts at `to-write`. When it exists and fails it is `red`; when it passes it is `green`.

```
  $ intent at red   ST0001 AT-01.1
  $ intent at green ST0001 AT-01.1 --note "passes at 8k entries under 64MB"
```

**Go through `red` first, even though nothing forces you to.** A test that goes straight from `to-write` to `green` was never observed failing, so nothing has demonstrated it can fail — which is the difference between a test and a decoration. **`at green`'s own help says "reachable only from red"; that describes v2, and v3 does not enforce it.** See [Criteria and tests](concepts/criteria-and-tests.md).

**Not everything is testable by a test, and Intent does not pretend otherwise.** A criterion satisfied by a document, a review or an eyeball is `--kind non-test`, and its acceptance test cites what was read rather than a file:

```
  $ intent at new ST0001 AT-02.9 --covers AC-02.9 --kind non-test --prose "Reviewed the rate-limit design against the upstream contract, 2026-08-29"
```

## 6. Read the state back

```
  $ intent st show ST0001
  $ intent ac list ST0001
```

**A criterion's state is computed, not asserted.** If its covering tests are green, it is satisfied; if they are not, it is not. You do not tick a box, and there is no way to make a thread look done that does not involve making it done.

When you want the gate rather than the listing:

```
  $ intent ac gate ST0001
```

`ac gate` exits non-zero and reports `BLOCKED` if anything in scope is unsatisfied. It is built to be run from a pre-commit hook or CI, not read by a human.

## 7. Close it

```
  $ intent st done ST0001
```

## Where to go next

- **[Concepts](concepts/)** — the model underneath: what a thread is, how criteria reach their state, and why the store rather than the files is the source of truth.
- **[Command reference](reference/)** — the full surface.
- `intent todo` — a flat DOING / TODO / DONE view across every thread and work package, generated from their real status.
- `intent doctor` — findings about **this project**: a stale backup, a thread whose status disagrees with its own gate, a store that has drifted from committed canon. It does not inspect your installation.

**One thing worth doing early.** If you use a coding agent, run `intent agents sync` after any significant change to a thread. It regenerates the agent contract from the project's actual state, which is the entire point — the file the agent reads cannot drift from the project, because it is not maintained by hand.
