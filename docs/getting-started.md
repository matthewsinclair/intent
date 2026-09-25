# Getting started

This walks one steel thread from nothing to satisfied, and leaves behind a project you can keep.

Everything below runs against a real repository. Intent lives inside your project, not beside it.

**Every command on this page has been run, in this order, from an empty directory, and the sequence ends at a passing gate.**

## 1. Initialise

You need a repository to work in. Intent does not create one:

```
  $ mkdir your-project && cd your-project
  $ git init
  $ intent init
```

`intent init` prints what it wrote, one path per line. That list is the tree, so there is one statement of it rather than two that have to agree:

```
  intent/.config/config.json     the project's metadata and declared languages
  .gitignore                     ignores the store and the backups
  .prettierignore                keeps the formatter off generated views
  AGENTS.md                      the agent contract, generated from project state
  CLAUDE.md                      the Claude-specific overlay
  intent/.canon/events/...json   the record of this act, one file per project act
  intent/.intentfiles            which threads are realised on disk
  intent/llm/ARCHITECTURE.md     this project's architecture, for you to write
  intent/llm/RULES.md            this project's own rules, for you to write
  intent/st/steel_threads.md     the thread register, generated and empty for now
  intent/todo.md                 the flat DOING / TODO / DONE view, generated
  intent/wip.md                  current work in progress
```

**`intent/st/` is there, holding only the empty register.** A thread's own directory, `intent/st/ST0001/`, arrives when you start your first thread at the end of §2, so a tree showing one here would be describing a project further along than the one you have.

**The store is created and not listed.** `intent/.cache/intent.db` is this machine's state and never belongs in history, and the `.gitignore` above keeps it and `intent/.backup/` out of git, so a `git add .` is safe.

`intent init` also tells you if no author is recorded for this machine; `intent bootstrap` records one once, or set it in `intent/.config/config.json`.

Then declare which languages the project is in:

```
  $ intent lang init rust
  declared: rust

  Summary: 1 language(s) declared; 0 error(s).
```

**It is a declaration, not a detection** — Intent will not guess from the files present, because file presence is unreliable evidence and a wrong guess loads the wrong rules. `lang init` takes more than one language and is idempotent, so you can add to it later.

**Declaring a language does not rewrite `AGENTS.md`**, so regenerate it, and the agent contract then names the language's toolchain:

```
  $ intent agents sync
  Syncing AGENTS.md with latest project state...
  ok: AGENTS.md updated at project root.
```

**You can also declare them as you initialise: `intent init --lang rust,shell`** creates the project and then declares both, through the same code `lang init` runs. An undeclarable name refuses before anything is written, and the refusal says nothing was created. `intent lang list` names the languages you can declare. `init --lang` writes `AGENTS.md` before it declares the languages, so run `intent agents sync` after it too.

`AGENTS.md` is **generated from the templates and the project's configuration**: `intent init` writes the first one and `intent agents sync` regenerates it. Do not hand-edit it; the next sync will overwrite you. `intent doctor` says, as an uncounted advisory, when it is behind.

## 2. Open a steel thread

A steel thread is one intention. Name the outcome, not the task.

```
  $ intent st new "Rate-limited cache for API protection"
  created: ST0001
```

A new thread starts in `Triage`, where it waits to be picked up; `intent st new --start` opens one already in progress.

Then write down why. This is the part that matters and the part everything else hangs off:

```
  $ intent set intent:///threads/ST0001 objective "Cache because the upstream API limits us to 100 req/min"
  $ intent set intent:///threads/ST0001 context "We see 10K req/s at peak, and the upstream refuses rather than queues"
```

`intent set <address> <field> <value>` writes one field of one entity. For prose that spans lines, `--from <file>` reads the value from a file instead.

**`info.md` is a generated view of the thread, not its source.** `intent st edit ST0001 info --path` prints its path, which is where to read what you wrote; without `--path`, and run from a terminal, `st edit` opens the file in `$VISUAL` or `$EDITOR` instead. **Write the objective and the context with `intent set`, the direct route.** You can also type them into `info.md`: open it with `intent st edit ST0001`, write the two sections, then run `intent sync --to-store ST0001` to carry them into the store before your next command. Until that runs, a command that would render the file over your edit refuses and names that step; a running `intentd` carries the edit on its own. A hand edit anywhere else in `info.md` is not carried: `intent doctor` reports it as skew, and the next render overwrites it.

**A new thread carries `info.md` and `acceptance.md`, and nothing else.** `design`, `impl` and `tasks` are not created for you and are not made real by creating the file — a thread's file set is a property of the model, not of the directory. They join a thread as attachments: `intent st attach ST0001 design.md --from <your file>` is the writer, and once a thread carries one, `intent st edit ST0001 design` opens it, or prints its path with `--path`. An attachment is yours: an edit to it is kept. Until then the thread has no design of its own to open, and the reasoning goes in `objective` and `context` above. **Be specific about constraints and about what you ruled out** — that is the information nobody can reconstruct later, and it is what an agent reading this file will act on. "Cache because the upstream API limits us to 100 req/min, and we see 10K req/s at peak" tells a reader something. "Add caching" does not.

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

`intent wp list ST0001` shows where they stand. Statuses are `not-started`, `wip`, `done` and `cancelled` (the listing prints them as `Not Started`, `WIP`, `Done` and `Cancelled`), and they move with `wp start`, `wp unstart`, `wp done`, `wp cancel` and `wp reopen` rather than by editing a field. **`wp cancel` requires `--reason`** and refuses without one, because the reason is recorded on the work package as the reason for its current state. **`wp reopen` takes a `done` package back to `wip`**, and refuses one that has not started — the machine has no terminal states, so there is always a route, but it goes through the states rather than around them — and it, `wp cancel` and `wp reinstate` all require `--reason`.

A work package has an objective of its own, and `wp new` leaves it unwritten: `intent set intent:///threads/ST0001/wp/01 objective "..."` writes it, and `wp done` warns when it closes a package whose objective still says nothing.

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

Ids are caller-assigned, and **the id's shape is how a criterion belongs to a work package**: `AC-01.1` is WP 01's, and `wp done ST0001/01` is gated on the `AC-01.*` criteria alone. `AC-00.*` belong to the thread as a whole. `st done` is gated on every criterion, whatever its id. A work package with no criteria of its own closes without a contract check, as long as the thread has criteria; a thread with none at all refuses both `wp done` and `st done` as an empty contract.

## 5. Back each criterion with a test

A criterion with nothing behind it is a promise. An acceptance test is what turns it into a computed fact.

**A test row cites a file, and two things are checked about that file: that it exists, and that it contains the test's own id.** `at new` records the row either way, but `at red` and `at green` refuse to move a row whose file is missing, and the gate refuses a thread that cites one. So write the files before you cite them:

```
  $ mkdir -p tests
  $ echo '// AT-01.1 -- evicts oldest entries under memory pressure' > tests/cache_eviction.rs
  $ echo '// AT-01.2 -- a warm cache survives a process restart'     > tests/cache_persistence.rs

  $ intent at new ST0001 AT-01.1 --covers AC-01.1 --file tests/cache_eviction.rs
  $ intent at new ST0001 AT-01.2 --covers AC-01.2 --file tests/cache_persistence.rs
```

The id in the file is what ties a row to the thing that runs; without it the gate reports `does not carry the literal id` and refuses the thread, however green the row is.

A test starts at `to-write`. When it exists and fails it is `red`; when it passes it is `green`.

```
  $ intent at red   ST0001 AT-01.1
  $ intent at green ST0001 AT-01.1 --note "passes at 8k entries under 64MB"
  $ intent at red   ST0001 AT-01.2
  $ intent at green ST0001 AT-01.2 --note "passes across a restart"
```

**Go through `red` first, and the tool holds you to it: `at green` is refused from `to-write`.** A test that went straight from `to-write` to `green` would never have been observed failing, so nothing would have demonstrated it can fail — which is the difference between a test and a decoration. See [Criteria and tests](concepts/criteria-and-tests.md).

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

**A test-backed criterion's state is computed, not asserted.** If its covering tests are green, it is satisfied; if they are not, it is not. You do not tick a box, and a test-backed criterion reads satisfied only when its tests are green. The one way round that is a fiat close, `intent fc`, which a person makes on the record with a reason and which the gate counts separately from satisfaction. **A non-test criterion is the deliberate exception**: it is satisfied by evidence you record, which is why the evidence is named on the record and `ac unsatisfy` clears the satisfaction and the evidence together.

When you want the gate rather than the listing:

```
  $ intent ac gate ST0001
```

```
  gate: ST0001 PASS -- 3/3 satisfied
```

`ac gate` exits non-zero and reports `BLOCKED` if anything in scope is unsatisfied, naming the unsatisfied criteria, or naming the test-contract finding that stops it first, such as a cited file without its id. It is built to be run from a pre-commit hook or CI.

## 7. Close it

Finish or cancel the work packages, then close the thread:

```
  $ intent wp done ST0001/01
  $ intent wp cancel ST0001/02 --reason "rate limiting moved upstream"
  $ intent st done ST0001
```

`st done` runs the gate first and refuses a thread that would not pass it, and it refuses while a work package is still `not-started` or `wip`, naming each one: finish them with `wp done` or drop them with `wp cancel --reason`, then close the thread. `st done` is legal only from `WIP`, which is why §2 started the thread.

Closing a thread unlists it from `intent/.intentfiles`, and `st done` says so: its realised files leave the disk at the next `intent organize --apply`, `intent st hydrate ST0001` writes them back, and `st done --keep` closes without unlisting. The thread itself is in the store and the canon either way.

## Where to go next

- **[Concepts](concepts/)** — the model underneath: what a thread is, how criteria reach their state, and why the store rather than the files is the source of truth.
- **[Command reference](reference/)** — the full surface.
- **[Known defects](known-defects.md)** — what is broken in the current release that you can reach by following these pages correctly, each entry driven against the build the release is cut from.
- `intent todo` — a flat DOING / TODO / DONE view across every thread and work package, generated from their real status.
- `intent doctor` — findings about **this project**: a stale backup, a thread whose status disagrees with its own gate, a store that has drifted from committed canon. Its verdict counts only the project; it also prints advisories it does not count, such as a root file that differs from what your installed Intent would write. A class of finding your project has decided to keep can be acknowledged in `intent/.config/config.json` as `"doctor": {"acknowledged": {"<class>": "<reason>"}}`, using the class name `doctor` prints: it still runs and prints `acknowledged: <class> -- <reason> (N finding(s))`, and its findings leave the count and the exit code.

**One thing worth doing early.** If you use a coding agent, run `intent agents sync` whenever you declare or remove a language, and after you upgrade Intent. It regenerates the agent contract from the templates and the project's configuration, which is the entire point: the file the agent reads is not maintained by hand, so it cannot quietly drift from the project.
