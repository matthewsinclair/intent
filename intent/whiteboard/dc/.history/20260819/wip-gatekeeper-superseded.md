# dc -- archived 2026-08-19: the Gatekeeper narrative, REFUTED

**Kept verbatim because it is the specimen, not because it was right.** Every
figure below was published to four nodes and two of them acted on it: ic
withdrew every timing figure they held, and cc revived an 81 -> 3 consolidation.
**The mechanism did not reproduce**: 40 freshly compiled, never-executed test
binaries executed in 364ms total (~9ms each) against a published ~30s each, off
by a factor of ~3,300. The PHENOMENON was real -- 99% overhead, measured twice
-- and the clean slate removed it. A real effect does not make a refuted
mechanism correct.

**This is the fifth wrong explanation for one wall-clock gap.** It is archived
rather than deleted so the next reader sees what a confident, well-sourced,
internally-consistent, wrong finding looks like from the inside.

---

## DOING

Nothing in flight. **THREE FIXES HELD, VERIFIED, UNCOMMITTED** -- waiting on matts's suite to finish rather than firing the pre-commit gate into a live acceptance run: `critic_report_format.bats` (2 json regressions, mine from `3a646965`) and `critic_arming_census.bats` (the retarget-guard catch). 126 assertions / 9 suites / 0 failures on the fixed tree.

## TODO

1. **THE RUST SLOWNESS IS SOLVED AND THE REMEDY IS MOSTLY NOT MINE. IT IS GATEKEEPER.** Measured on matts's live run, twice:
   - 11min20s wall / **8.51s** summed `finished in`; then 22min40s wall / **11.87s**. **99.1% overhead, linear at ~30s per suite.** 88 suites will take ~44 minutes to do ~23 seconds of work.
   - **`rustc` NOT running** (compile already done), test binary state `S` with **RSS 32 KB** -- blocked BEFORE the test code loads -- and `syspolicyd` at **21-22% sustained**.
   - Same binary, first exec vs second: `intent` **20633ms -> 26ms**, `acceptance_surface` **19459ms -> 24ms**, `session_hook_lockout` **10949ms -> 23ms**. `spctl --status` = assessments enabled; binaries are adhoc/linker-signed.
   - **Cargo makes ONE BINARY PER `tests/*.rs`**: 25 intent-cli + 56 intentsvcs = **81 per build**. That is also why `deps/` holds 291 executables and **778,425 files**, and why `target/debug` is 15GB (~25GB across the per-node dirs).
   - **REMEDIES, in impact order:** (a) register the terminal under Privacy & Security -> Developer Tools -- matts's call, it is a real security setting; (b) **consolidate 81 `tests/*.rs` into 2-3 binaries** -- mine, see 2; (c) prune stale generations and `.noindex` the target dirs.
   - **NOT MEASURED AND NOT CLAIMED: the COMPILE phase.** `build.rs` outputs are executables that get RUN and proc-macro dylibs get loaded, so the same tax plausibly applies -- but I have no measurement and two probes today already returned what I expected while being unable to return anything else.
2. **TEST-BINARY CONSOLIDATION IS REVIVED AND I KILLED IT ON A CONFOUNDED CONTROL.** I compared Lamplight (19 binaries, ~10min) against Intent (80 binaries, 1m56s) and concluded we were the faster project. **I was comparing two validation-CACHE states, not two binary counts.** 81 -> 3 binaries cuts validation from ~23min to under a minute even if Developer Tools is never enabled. Changes a spelling four nodes use (`--test X` becomes `--test suite X`, including the `INTENT_BLESS=1` re-pin).
3. **TODO 5, BLOCKED ON matts: the hook's distinct exit code + Half B's refusal + generalising the guard roster's presence test for cc's `canon-ignore-guard.sh`.** One re-cut of one block in `pre-commit.sh`; cc offered to do the roster part to my spec and I took it instead so the file is re-reasoned once. **It changes fail-open semantics every fleet consumer inherits on upgrade.**
4. **`canon_commit_check.sh`'s MISSING ARM -- bytes with no canon record.** vc ruled the eligibility contract already exists and is single-homed at `Project::classify()` (`ATTACHMENT_EXTENSIONS = md, txt, sh`), so a shell restatement would be the fourth list. **Blocked on the surface question: the arm needs the binary to EXPOSE the classification.** ic first for shape, then matts.
5. **`intent sync` HAS NO SCOPE and that is now an empirical finding, not an architectural one.** The correct per-node workflow needs an unscoped whole-estate write; two nodes running it clobber at rc=0. **`canon_commit_check.sh` is rostered `manual` PRECISELY because of it**, so the estate holds a detector it cannot afford to run -- and that gap has already cost one real divergence (`critic-gate.md`, 15,428 bytes of formatter table padding).
6. **BASH-4 SWEEP: DONE, control committed** (`f8e05490`). 0 constructs across the 7 shipped hooks; the finding is that **`bash -n` at 3.2.57 is blind to both classes** and none of the seven sets `-e`, so a stray bash-4 BUILTIN is rc=0 with a plausible number.
7. **`doctor` = option 3, ruled.** XS. Check whether v3's doctor needs the mirror.
8. **WP-11 dist wiring** -- no `dist-workspace.toml`, no formula. Policy-unblocked, sequence-blocked: needs a tag, and `staged_version` is `3.0.0-dev`. AC-11.1/11.4 open.
9. **`output-contracts.md` owes five additions**: suppression as TRANSMISSION; the refuting datum uncollected in the author's own output; ic's _a control that appears to fire and doesn't_; the aggregating formatter; and **the new one -- an instrument that cannot distinguish BLOCKED from BUSY.**
