# Steel threads and work packages

## A steel thread is one intention, followed end to end

Not a ticket, not an epic, not a task. **A steel thread names something you are trying to achieve and records why it matters**, and it stays in the repository while the work happens so the reasoning and the code age together.

The name comes from the engineering sense: the thinnest complete path through a system that actually works end to end. A steel thread in Intent is the same idea applied to intention — one thing you meant to achieve, followed from the reason it mattered through to the evidence it was done.

Each thread lives at `intent/st/<ID>/` and carries:

|                         |                                                                |
| ----------------------- | -------------------------------------------------------------- |
| **Objective**           | What you are trying to achieve, in outcome terms               |
| **Context**             | Why it matters, what constrains it, what you ruled out and why |
| **Work packages**       | The units of work inside it                                    |
| **Acceptance criteria** | The conditions that decide whether the intention was met       |
| **Acceptance tests**    | What backs each criterion                                      |

**The context field is the one that pays for the whole system.** Objective and work packages are recoverable by reading the code afterwards, slowly and imperfectly. The reasoning is not. "Cache because upstream limits us to 100 req/min and we see 10K req/s at peak" cannot be reconstructed from a cache implementation; it can only be written down at the time.

### How big is a thread

**One intention.** A thread whose objective needs the word "and" is usually two threads. A thread with twenty work packages is one that stopped being an intention and became a project.

The practical test: if you cannot state the acceptance criteria without them contradicting each other in emphasis, the thread is carrying more than one intention.

## Thread states

Six states, and the transitions between them are the only way to move.

| State         | Meaning                                     |
| ------------- | ------------------------------------------- |
| `triage`      | Proposed. Entry state for every new thread  |
| `not-started` | Accepted, not begun                         |
| `wip`         | In flight                                   |
| `hold`        | Paused, with a reason recorded              |
| `completed`   | Done, and the gate passed                   |
| `cancelled`   | Not going to happen, with a reason recorded |

```
  (none) --> triage --> not-started --> wip --> completed
                |            |           |
                |            +--> hold <-+
                |            |           |
                +------------+-> cancelled
```

**Every transition out of the happy path records a reason.** `st hold`, `st cancel`, `st reopen` and `st reinstate` all require one — because a thread that stopped, restarted, or came back from cancelled is exactly the case where a future reader most needs to know why, and it is exactly the case where nobody remembers.

**`st done` is guarded by `ac gate`.** A thread cannot be completed while a criterion in scope is unsatisfied. This is the single most important constraint in the model: **there is no way to make a thread look done that does not involve making it done.**

A cancelled thread reinstates to `not-started`, not to whatever it was before. Cancelling is a decision; undoing it means deciding again from the beginning rather than restoring a state that a cancellation already invalidated.

## Work packages

**Work packages are the units that get done.** They exist so a thread in flight can report where it actually is, and they carry no reasoning of their own — the reasoning is the thread's.

| State         | Meaning                         |
| ------------- | ------------------------------- |
| `not-started` | Created, not begun              |
| `wip`         | In flight                       |
| `done`        | Delivered, and the gate passed  |
| `cancelled`   | Dropped, with a reason recorded |

```
  (none) --> not-started --> wip --> done
                  ^           |       |
                  +-- unstart-+       |
                  |           |       |
                  +-- reinstate       +--> (reopen) --> wip
                  |
             cancelled <-- (from any of not-started, wip, done)
```

**`wp done` is guarded by `ac gate` in the same way `st done` is.** A work package closes when what it owed is satisfied, not when someone decides it feels finished.

`wp unstart` exists because starting a work package by mistake is common and should not require cancelling it. It is not a state change with meaning; it is an undo.

### Work packages can be cancelled, and that was not the original design

The first design had no `cancelled` at work-package level, on the reasoning that a work package which stops mattering is a scope change on the thread rather than a state on the package. **That was wrong, and how it went wrong is worth knowing if you are modelling something similar.**

A live project removed a work package's scope, withdrew every criterion under it, and then found `wp done` refused forever — correctly, because the gate declines to infer an exemption from an emptied contract. **An exemption has to be announced; it is never inferred from emptiness.** The only announced exemption available was thread-scoped, so using it would have discarded the standing of every other criterion on that thread.

The interim workaround was to mark the package done and write a note, which put the distinction in prose because there was no field for it — **and a `done` work package that delivered nothing is then indistinguishable, by query, from one that delivered.** That is the same defect one layer along, which is what made the case for the state.

## Reading the state back

```
  $ intent st list                 # in-progress threads
  $ intent st list --status wip
  $ intent st show ST0001
  $ intent wp list ST0001
  $ intent todo                    # flat DOING / TODO / DONE across everything
```

`intent todo` is **generated from real ST and WP status**, not hand-maintained. It is a projection, and regenerating it with `intent todo update` cannot disagree with the threads because there is nothing in it that is not derived from them.

---

Next: [Criteria and tests](criteria-and-tests.md) — how a thread's satisfaction is computed rather than asserted.
