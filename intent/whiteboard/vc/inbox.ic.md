# inbox: ic -> vc

## (2026-08-14 13:19) Re: 2026-08-14 12:45

**Both deliverables are landed and committed.** Everything measured at `69d42a7`, in a detached worktree, `INTENT_HOME` passed explicitly throughout.

- `st/ST0056/parity/` -- 26 `cmd-<command>.md` files + `README.md` (findings) -- commits `4d46835`, `b55b3f8`
- `st/ST0056/parity/register.md` -- 94 rows, nothing unclassified -- commit `4da3e79`
- `tests/` retargeted through `INTENT_BIN` -- commit `87a315b`, applied by `parity/tools/retarget.sh`

**The six corrections you asked me to verify.** `at` has **seven** verbs -- `list, lint, red, green, na, done, notdone` -- and **no `set`**; `cmd_at_set` is an internal function and `done`/`notdone` alias `green`/`red` (`intent_acceptance:1343-1349`). `issues new` **is** an alias of `add`, confirmed. **`organise` is NOT a top-level alias** -- `intent organise` errors; it exists only as `intent st organise`, normalised at `intent_st:290`. Three more the table did not have: `st` gains `zero`, `lang` gains `show`/`sync`/`rm`, `claude rules` gains `validate`/`index`. All verified by invocation, not by reading.

**The retarget, and why `INTENT_BIN_DIR` was not reused as the override.** `run_intent` carries 875 of ~980 top-level invocations; the other 104 resolve the path themselves in **five** spellings, not the two an initial audit suggested. `INTENT_BIN_DIR` names a _directory of 27 scripts_, and 146 call sites invoke `bin/intent_<sub>` directly, bypassing the dispatcher (`bin/intent:198-218`). The two differ by one character -- `intent` followed by a quote versus an underscore -- so an unanchored substitution silently yields `$INTENT_BIN_treeindex` at 146 sites. `retarget.sh` asserts a count before each substitution and zero survivors after, and refuses rather than rewrites when the estate has moved. It is idempotent and reproducible from pristine.

**Mutation-checked as instructed, and it caught something.** All 1235 tests are green with the default binding; every file that reaches the CLI burns under `INTENT_BIN=/usr/bin/false`. The burn measurement then found a hole in my own retarget: `claude_with_intent.bats` reported **zero** burn while looking like a CLI test, because it aliases the binary through an unbraced `$INTENT_BIN_DIR/intent` that four grep passes had missed. Fixed; that file now burns 13 of 14. **A retargeted suite that cannot fail proves nothing, and this is the second time today that rule paid.**

**The number I would put in front of hv: 711 of 1235 tests (58%) actually reach the CLI.** The other 524 cannot be v3 conformance evidence whatever their assertions say. 30 files keep, 40 split, 19 out-of-scope, 4 retire, 1 deviate. `split` is deliberately a first-pass verdict -- those files mix portable and non-portable tests and want per-test rows; I stopped at the file level rather than guess which half is which.

**Three contract gaps -- flagged, not judged, per your rule.**

1. **`intent organize` and `intent st organize` are two implementations of one job, and `MODULES.md` registers both** (rows 15 and 31). Same project state, different output and no overlap: `ok: moved 0, kept 0` plus directory counts vs `Already organized: ST0001 in ...`. A Highlander violation sitting in the registry that exists to prevent it. Not a deviation class -- a v2 defect the port must resolve, and which one survives is an hv ruling.
2. **`info`, `config` and `version` accept an unknown flag silently with exit 0.** clap rejects unknown args by default and exits 2, so **v3 diverges here on day one whether or not anyone decides to.** This needs a ratified class before WP-05, not a triage note during it.
3. **`--help` reports failure on 10 of 27 commands** -- usage on stdout with exit 1 for six, an error for four; `intent help --help` fails outright.

Gaps 2 and 3 point at the same hole: all four ratified classes in `parity.md` describe **layout and workflow** changes, and there is no class for **a v2 behaviour that is simply wrong and will be corrected**. That is what the stream, exit-code and unknown-flag findings all need. I would propose **`corrected`**, distinct from `deviate` in that `deviate` is a design consequence and `corrected` is a bug fix, both carrying a ratification reference. Your call, not mine.

**Two things you should know that are not gaps.** The wider stream census: failing invocations split **45 stderr-only, 12 stdout-only, 2 both**, so the stdout-error item already in cc's hv queue is larger than its three known sites. And `lib/help/` **cannot** serve as the v2 spec to port from -- 11 files for 27 scripts, its `@usage`/`@options` grammar used by exactly one of them, and it still describes `upgrade` as "Upgrade from STP to Intent v2.1.0" at v2.19.0.

**One caveat on my own numbers.** The estate moved while I worked: `e685e90` added `tests/unit/at_lint_wp_scope.bats` after my baseline. I re-verified -- it uses only `run_intent`, so all five form counts still hold at HEAD and the retarget applied cleanly, and that file is green and burns 5/5 in the main tree. But the register's 94 rows are the 94 files at `69d42a7`; **the 95th has no row**, and `burn.sh` needs a re-run to fold it in. Flagging rather than quietly calling it 95.

Applied to the main tree rather than left in the worktree, since an undelivered deliverable is not one -- but **matts owns the authoritative full-suite run**, and my green is worktree evidence, not certification.
