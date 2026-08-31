# Getting started

This walks one steel thread from nothing to satisfied. It takes about ten minutes and leaves behind a project you can keep.

Everything below runs against a real repository. Intent lives inside your project, not beside it.

**Every command on this page has been run, in this order, from an empty directory, and the sequence ends at a passing gate.** That is worth stating because the previous version of this page had not been: it broke on its first command, told you to create a test covering a criterion it never had you create, and ended at a gate its own steps could not pass.

## 1. Initialise

You need a repository to work in. Intent does not create one:

```
  $ mkdir your-project && cd your-project
  $ git init
  $ intent init
```

`intent init` creates `intent/` in your repository and generates the agent contract at the root.

```
  intent/
    .config/config.json     project metadata, the declared languages
    st/                     steel threads
    docs/                   project documentation
    wip.md                  current work in progress
  AGENTS.md                 the tool-agnostic agent contract, generated
```

Then declare which languages the project is in:

```
  $ intent lang init rust
  declared: rust
```

**It is a declaration, not a detection** — Intent will not guess from the files present, because file presence is unreliable evidence and a wrong guess loads the wrong rules. `lang init` takes more than one language and is idempotent, so you can add to it later.

**`intent init --lang rust` does not work in this build and refuses with a reason that is out of date.** It says `intent lang init` is not implemented; that command is implemented and is the one above. What is missing is `init` calling it. Declare languages as a separate step until that is wired.

`AGENTS.md` is **generated from project state** by `intent agents sync`. Do not hand-edit it; the next sync will overwrite you.

## 2. Open a steel thread

A steel thread is one intention. Name the outcome, not the task.

```
  $ intent st new "Rate-limited cache for API protection"
  created: ST0001
```

Then write down why. This is the part that matters and the part everything else hangs off:

```
  $ $EDITOR intent/.canon/st/ST0001.json
  $ intent sync --to-store ST0001
  $ intent sync --to-disk  ST0001
```

**You edit the canon extract, not `info.md`.** `info.md` is a generated view — `intent st edit ST0001 info` refuses to hand you its path for exactly that reason, and it is right to. Fill in the `objective` and `context` fields in the JSON, sync them into the store, then render the view back out.

**There is no verb for this yet, and the refusal you get if you try says there is.** It names `intent st` as the place to author thread fields; no `intent st` verb writes `objective` or `context`. The canon route above is the working one.

**`intent st edit` does work for the authored files** — `design`, `impl` and `tasks` are yours, not generated, so it prints their paths and `$EDITOR "$(intent st edit ST0001 design)"` behaves as you would expect. **Be specific about constraints and about what you ruled out** — that is the information nobody can reconstruct later, and it is what an agent reading this file will act on. "Cache because the upstream API limits us to 100 req/min, and we see 10K req/s at peak" tells a reader something. "Add caching" does not.

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

`intent wp list ST0001` shows where they stand. Statuses are `not-started`, `wip`, `done` and `cancelled`, and they move with `wp start`, `wp done`, `wp cancel` and `wp reopen` rather than by editing a field. **`wp cancel` requires `--reason`** and refuses without one, because the reason is recorded on the work package as the reason for its current state. **`wp reopen` is legal only from `done`** — the machine has no terminal states, so there is always a route, but it goes through the states rather than around them.

## 4. State the acceptance criteria

**This is the step that distinguishes Intent from a task tracker.** A criterion is a condition that decides whether the intention was met — not a restatement of the work.

A criterion is one of two kinds, and **the kind decides what satisfies it**:

- `--kind test` — satisfied by its covering tests going green. Nothing else can satisfy it.
- `--kind non-test` — satisfied by named evidence you record, for the things a test cannot decide: a review, a document, a read.

```
  $ intent ac new ST0001 AC-01.1 --kind test --text "Evicts oldest entries under memory pressure rather than failing writes"
  $ intent ac new ST0001 AC-01.2 --kind test --text "A warm cache survives a process restart"
  $ intent ac new ST0001 AC-02.1 --text "Requests over quota are refused, not queued"
```

**`--kind` defaults to `non-test`, so pass `--kind test` deliberately.** A criterion you meant to be test-backed and left to the default will sit unsatisfied no matter how green its tests are, because greenness is not what satisfies a non-test criterion. The third line above takes the default on purpose — it is satisfied in §5 by evidence rather than by a test.

Ids are caller-assigned. The convention `AC-<wp>.<n>` ties a criterion to the work package that satisfies it, and Intent does not enforce it — it is a convention that reads well, not a rule.

## 5. Back each criterion with a test

A criterion with nothing behind it is a promise. An acceptance test is what turns it into a computed fact.

**A test row cites a file, and the gate checks two things about that file: that it exists, and that it contains the test's own id.** So write the files before you cite them:

```
  $ mkdir -p tests
  $ echo '// AT-01.1 -- evicts oldest entries under memory pressure' > tests/cache_eviction.rs
  $ echo '// AT-01.2 -- a warm cache survives a process restart'     > tests/cache_persistence.rs

  $ intent at new ST0001 AT-01.1 --covers AC-01.1 --file tests/cache_eviction.rs
  $ intent at new ST0001 AT-01.2 --covers AC-01.2 --file tests/cache_persistence.rs
```

The id in the file is what ties a row to the thing that runs; without it the gate reports `does not carry the literal id` and the criterion stays unsatisfied even with the row green.

A test starts at `to-write`. When it exists and fails it is `red`; when it passes it is `green`.

```
  $ intent at red   ST0001 AT-01.1
  $ intent at green ST0001 AT-01.1 --note "passes at 8k entries under 64MB"
  $ intent at red   ST0001 AT-01.2
  $ intent at green ST0001 AT-01.2 --note "passes across a restart"
```

**Go through `red` first, even though nothing forces you to.** A test that goes straight from `to-write` to `green` was never observed failing, so nothing has demonstrated it can fail — which is the difference between a test and a decoration. **`at green`'s own help says "reachable only from red"; that describes v2, and v3 does not enforce it.** See [Criteria and tests](concepts/criteria-and-tests.md).

**Not everything is testable by a test, and Intent does not pretend otherwise.** `AC-02.1` was created `non-test` in §4. Its acceptance test cites what was read rather than a file, and **the criterion is then satisfied by naming the evidence**:

```
  $ intent at new ST0001 AT-02.1 --covers AC-02.1 --kind non-test --prose "Reviewed the rate-limit design against the upstream contract"
  $ intent at na  ST0001 AT-02.1 --note "Reviewed 2026-08-31; refusal path confirmed against the upstream contract"
  $ intent ac satisfy ST0001 AC-02.1 --evidence "AT-02.1: reviewed against the upstream contract"
```

**`at new` refuses a `--covers` naming a criterion that does not exist**, which is what you want — a test covering nothing is a row that can never move its criterion.

## 6. Read the state back

```
  $ intent st show ST0001
  $ intent ac list ST0001
  ac: AC-01.1  covered-by: AT-01.1  satisfied: yes
  ac: AC-01.2  covered-by: AT-01.2  satisfied: yes
  ac: AC-02.1  covered-by: AT-02.1  satisfied: yes
```

**A test-backed criterion's state is computed, not asserted.** If its covering tests are green, it is satisfied; if they are not, it is not. You do not tick a box, and there is no way to make a test-backed thread look done that does not involve making it done. **A non-test criterion is the deliberate exception**: it is satisfied by evidence you record, which is why the evidence is named on the record and `ac unsatisfy` clears the satisfaction and the evidence together.

When you want the gate rather than the listing:

```
  $ intent ac gate ST0001
```

`ac gate` exits non-zero and reports `BLOCKED` if anything in scope is unsatisfied. It is built to be run from a pre-commit hook or CI, not read by a human.

## 7. Close it

```
  $ intent st done ST0001
```

`st done` runs the gate first and refuses a thread that would not pass it, so a closed thread is a thread that was actually finished.

## Where to go next

- **[Concepts](concepts/)** — the model underneath: what a thread is, how criteria reach their state, and why the store rather than the files is the source of truth.
- **[Command reference](reference/)** — the full surface.
- `intent todo` — a flat DOING / TODO / DONE view across every thread and work package, generated from their real status.
- `intent doctor` — findings about **this project**: a stale backup, a thread whose status disagrees with its own gate, a store that has drifted from committed canon. It does not inspect your installation.

**One thing worth doing early.** If you use a coding agent, run `intent agents sync` after any significant change to a thread. It regenerates the agent contract from the project's actual state, which is the entire point — the file the agent reads cannot drift from the project, because it is not maintained by hand.
