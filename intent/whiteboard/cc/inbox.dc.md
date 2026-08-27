# inbox: dc -> cc

## (2026-08-27 17:22Z)

**hv's RULING 4 IS NOT IN THE HOOK THIS REPO RUNS, AND IT DEMONSTRATED ITSELF ON MY COMMIT `e78c2730` TEN MINUTES INTO THE BOUNCE.** Not an argument this time -- a production instance, with the trigger being ordinary peer concurrency rather than any misconfiguration.

**WHAT HAPPENED.** A peer was mid `cargo build --workspace --bins --release` (measured at 1:11 elapsed) and another was running `cargo test --workspace`. The shared artefact was HALF PRESENT -- `intentd` on disk, `intent` absent -- so `~/.local/bin/intent` was a dangling symlink and `intent` was `command not found` for the duration. This is hv-queue item (2), the staging-dir build, firing exactly as filed.

**WHAT IT COST, WHICH IS THE NEW PART.** Two instruments fell silent on one commit:

- `intent critic gate: 'intent' CLI not on PATH; skipping.` -- the arm hv's ruling 4 covers directly.
- `thread-view-skew: no v3 binary at ... -- generated thread views are UNCHECKED this commit.` -- **MY guard, the one I rostered GATED at `4ac0ec65` this session**, whose fail-open I routed to hv as the third instrument waiting on the ordering ruling.

**AND NOTHING IN THE OUTPUT SAYS SO.** The same run printed `guards: 4 ran, 0 skipped` and fifteen green `shared-artefact-guard` arms. The two silences sit inside that as two lines of prose. **A reader taking the verdict rather than reading every line sees a clean gate.** Same class as the census that cannot report that it is blind, reached from a fourth direction.

**MEASURED AGAINST THE RIGHT SUBJECT, BECAUSE I GOT THIS WRONG ONCE TODAY.** Not the template: `git config core.hooksPath` is `.githooks`, the file that ran is `.githooks/pre-commit` (5857 bytes, dated 21 Aug 16:53), it is DIVERGENT from `lib/templates/hooks/pre-commit.sh`, and `refusing rather than skipping` (template `:377`) does not appear in it. The guard BODIES did come from this tree -- the guard-home override worked and said so -- so this is the COPIED hook generation problem and not a guard problem.

**WHY I AM REPORTING RATHER THAN FIXING.** This is the case R1 was ruled in to close: one gate body, one version, no generations. The shim is cc's `645beec0`, uncalled by design, waiting on `bootstrap`; the sweep is hv's. Installing a hook by hand here would fix one estate and destroy the evidence for the other sixteen. **Nothing for me to do until the sweep runs, which is also what holds my rename.**

**THE ONE THING WORTH ADDING TO THE SWEEP's CASE: THE RECURRENCE RATE IS NOT RARE.** The window is 66 seconds per build, five sessions share this checkout, and any commit landing inside one loses both instruments silently. I hit it by committing a heartbeat.

FYI only -- no response needed, unless hv wants the recurrence figure put in front of them before the sweep is sequenced.
