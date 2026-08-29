# Concepts

Intent is a small model. Four ideas carry almost all of it, and the rest of the tool is machinery for keeping them honest.

|                                                         |                                                                        |
| ------------------------------------------------------- | ---------------------------------------------------------------------- |
| **[Steel threads and work packages](steel-threads.md)** | The unit of intention, and the units of work inside it                 |
| **[Criteria and tests](criteria-and-tests.md)**         | How a thread's state is computed rather than asserted                  |
| **[The store](the-store.md)**                           | Why the database is the source of truth and the files are a projection |

## The one idea underneath all of them

**Nothing that matters is maintained in two places.**

Every recurring failure Intent is built against has the same shape. A comment and the code it describes are two homes for one fact, and they drift. A design doc and the implementation are two homes, and they drift. A status field somebody types and the work that was actually done are two homes, and they drift. In every case both copies look fine, nothing reports the divergence, and the first person to trust the wrong one pays.

So Intent is arranged so the second home does not exist:

- **A criterion's state is computed from its tests**, not typed by a person, so it cannot disagree with them.
- **`AGENTS.md` is generated from project state**, not written, so the file your agent reads cannot drift from the project.
- **The views on disk are generated from the store**, not edited, so there is one authority and everything else is a projection of it.
- **Rules live in one library** that critics, gates and agents all read, so "what the project requires" has one answer.

Where a second copy is genuinely unavoidable, Intent's answer is to put a check between them rather than to hope. That is what the commit-time gates are for.

## What Intent deliberately does not do

**It is not a task tracker and it does not want to be one.** There is no assignee, no sprint, no burndown, no estimate in hours. Work packages have four states because that is what is needed to know whether something is in flight; anything more is a different tool's job and Intent does not compete with it.

**It does not manage your tests.** Intent records which test backs which criterion and what state that test is in. It does not run your suite, does not parse its output, and does not pretend to know whether your assertions are any good.

**It does not stop you doing the wrong thing.** The gates refuse specific, mechanical contradictions — a criterion claimed satisfied with nothing behind it, canon that names bytes the commit does not carry. They are backstops on narrow failures, not a review.
