---
description: "Verification gate: a completion claim cites the one run that decided it"
---

# Verification Gate

A completion claim says what ran and what it showed. One run decides a change; it is not run again.

## Rules

### 1. Cite the run that decided it

A claim that something works names the command that proved it and quotes the line of its output that says so. "Tests pass" with nothing behind it is not a claim.

### 2. One run, then stop

The run that proved the change is the evidence. Do not run it again to show it again, after a compact or anywhere else: quote its result. Run it again only when the code it covered has changed since.

### 3. Prove the change, not the proof

Run the test or check that covers the change. Nothing tests a test, and no positive control is run of a measurement. Before any further check, name what it would catch that the first did not; if you cannot, do not run it.

### 4. What "done" requires

| Change type      | The one run                          |
| ---------------- | ------------------------------------ |
| Code change      | The tests that cover it              |
| Config change    | The config loads without error       |
| Build change     | The build                            |
| Doc change       | Nothing beyond the commit's own gate |
| Skill/agent file | It installs and lists                |
| Bug fix          | The test that was red is now green   |

### 5. Acceptance criteria, when the thread has one

If the steel thread carries an `acceptance.md`, "done" is bound to it:

- An Acceptance Test reaches green only from red, and the red is the code as it was, never a planted fault. `intent at green` refuses a row that is not `red`.
- A test-backed AC is satisfied by a green covering AT; a non-test AC by `intent ac satisfy`. The close-gate computes the verdict: `intent ac status <id>` or `intent ac gate`.
- Cite the test **file**, repo-relative, with `--file`, and put the AT's own id inside the test it cites.

See the AC/AT five-step in `working-with-llms.md` (D11).

## Red Flags

| Rationalization                        | Reality                                                        |
| -------------------------------------- | -------------------------------------------------------------- |
| "Context was compacted, so run again." | Quote the result you recorded. The code did not change.        |
| "Another seat proved it; I'll re-run." | Its measured green is the evidence. Read it and land.          |
| "Tests pass." (nothing shown)          | Name the run and quote its line.                               |
| "One more check to be safe."           | Name what it catches that the first did not, or do not run it. |
