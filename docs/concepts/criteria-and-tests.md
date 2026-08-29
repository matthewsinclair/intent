# Criteria and tests

**This is where Intent differs from a task tracker, and it is the whole argument.** A criterion's state is computed from evidence rather than asserted by a person.

## A criterion is a condition, not a restatement of the work

"Implement the cache" is a work package. "Evicts oldest entries under memory pressure rather than failing writes" is a criterion — it names a condition that is either true or not, and someone can check.

```
  $ intent ac new ST0001 AC-01.1 --text "Evicts oldest entries under memory pressure rather than failing writes"
```

Ids are caller-assigned. The convention `AC-<wp>.<n>` ties a criterion to the work package that satisfies it. **Intent does not enforce that convention** — it reads well and it is not a rule.

## Two kinds, and the kind decides everything downstream

| Kind       | What backs it                    | Entry state   |
| ---------- | -------------------------------- | ------------- |
| `test`     | A test that runs                 | `computed`    |
| `non-test` | A document, a review, an eyeball | `unsatisfied` |

**A test-backed criterion enters at `computed` and never leaves it by anyone's assertion.** Its state is a function of its covering tests: green tests mean satisfied, anything else means not. There is no verb that marks a computed criterion satisfied, because the whole point is that no such verb should exist.

**A non-test criterion enters at `unsatisfied` and is satisfied by evidence**, because there is no test to compute from:

```
  $ intent ac satisfy ST0001 AC-02.9 --evidence "Reviewed against the upstream rate-limit contract, 2026-08-29"
```

`ac satisfy` refuses a test-backed criterion. That refusal is the model holding: a criterion whose truth is computable must not be assertable.

**Not everything worth requiring is testable, and pretending otherwise is worse than admitting it.** A design that was reviewed, a document that exists, a decision that was recorded — these are real requirements with real evidence, and forcing them into a test file would produce a test that asserts nothing. `non-test` is the honest modelling of that, and the evidence string is what a later reader interrogates.

## Criterion states

Six states. Four carry a payload, and the payload is the point — a state change without its reason is a state change nobody can audit.

| State         | Payload            | Meaning                                       |
| ------------- | ------------------ | --------------------------------------------- |
| `computed`    | —                  | Test-backed; state derived from its tests     |
| `unsatisfied` | note               | Asked for, not yet met                        |
| `satisfied`   | evidence           | Met, and here is what shows it                |
| `descoped`    | target, by, reason | Moved to another thread                       |
| `withdrawn`   | reason, by         | No longer asked for                           |
| `fiat`        | fiat record        | Closed without being met, by a named decision |

```
  authored --> computed        (test-backed)
  authored --> unsatisfied     (non-test)

  unsatisfied <--> satisfied           ac satisfy --evidence / ac unsatisfy
  {computed, unsatisfied, satisfied} --> descoped     ac descope --to <ID>
  {computed, unsatisfied, satisfied} --> withdrawn    ac withdraw --reason
  {descoped, withdrawn, fiat} --> back to entry state ac rescope / ac reinstate
```

**`descoped` and `withdrawn` leave scope; `fiat` does not.** A descoped or withdrawn criterion stopped being asked for. A fiat-closed one is **still asked for and was closed unmet** — so folding it in with the others would shrink the denominator and make a thread that cut and ran look like one that never owed the work. It stays in scope and renders distinctly.

**Rescope and reinstate land on the entry state for the kind**, not on whatever the criterion held before. A test-backed criterion returns to `computed` and is immediately re-derived from its tests; a non-test one returns to `unsatisfied` and has to earn its evidence again. Restoring a stale `satisfied` would restore a claim nothing currently supports.

**No state is terminal.** Every state has an exit, including `fiat`, and that is a structural rule rather than a convenience: a terminal value in a state field is a trap, and the guard that refuses trap states protects every machine in the model.

## Acceptance tests

A criterion with nothing behind it is a promise. An acceptance test is what makes it a computed fact.

```
  $ intent at new ST0001 AT-01.1 --covers AC-01.1 --file tests/cache_eviction.rs
```

| Status     | Meaning                     |
| ---------- | --------------------------- |
| `to-write` | Named, does not exist yet   |
| `red`      | Exists and fails            |
| `green`    | Exists and passes           |
| `n-a`      | Deliberately not applicable |

**Green is reachable only from red.** A test that goes straight from `to-write` to `green` has never been observed failing, so nothing has demonstrated it can fail — which is the difference between a test and a decoration. Intent will not let you skip that step.

A `non-test` acceptance test cites what was read rather than a file:

```
  $ intent at new ST0001 AT-02.9 --covers AC-02.9 --kind non-test --prose "Reviewed the rate-limit design against the upstream contract"
```

`--covers` takes one or more criterion ids. **One test can cover several criteria and one criterion can be covered by several tests**; the relation is many-to-many, and a criterion is satisfied when every test covering it is green.

## The gate

```
  $ intent ac gate ST0001
```

`ac gate` reports `PASS` or `BLOCKED` and exits non-zero when blocked. It is built for a pre-commit hook or CI, not for reading.

**It is also the guard on `st done` and `wp done`**, which is what makes the model bite. Closing something means passing the gate; there is no path around it that does not involve satisfying what was asked.

**What is in scope is the part worth understanding.** `computed`, `unsatisfied`, `satisfied` and `fiat` are in scope. `descoped` and `withdrawn` are not — they stopped being asked for. So withdrawing a criterion genuinely removes it from the denominator, which is why `ac withdraw` requires a reason and records who did it: **it is the one operation that makes a gate easier to pass, and it leaves a trail saying so.**

---

Next: [The store](the-store.md) — why the database is the source of truth and the files are a projection.
