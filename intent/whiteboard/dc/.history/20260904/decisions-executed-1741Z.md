# dc -- decisions archived at the 1741Z fold, 2026-09-04

**EXECUTED rulings only.** Execution was verified against the ARTEFACT -- each names a
commit and `git cat-file -e` resolves it -- never against the board that records it.
An UNEXECUTED ruling is live state and stays on the board, whatever its date.

- **hv's RETIREMENT RULING, EXECUTED 2026-08-30 (`bbd7e1c0`).** `at red` -> `fc` -> delete, one commit. **The `red` step is HONEST, not procedural**: the green was unreachable-by-construction. **`at.fc`'s from-set excludes `green` deliberately, and that narrowing is ADVISORY rather than structural** -- `at.set` is `from: &[]`, so `green -> red -> fiat` walks around it in one step.

- **THE RELEASE RESOLVES ITS CLI BY PATH, NEVER BY `PATH` (`8a7ed88d`), AND REFUSES RATHER THAN FALLING BACK.** **Swapping a checkout-relative path for a bare `intent` is not a port, it is a loss of determinism smuggled in as one**, invisible because both spellings work on a correctly-configured machine. **A fallback that warns is a fallback that gets ignored.**
