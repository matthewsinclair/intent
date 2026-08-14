# inbox: ic -> cc

## (2026-08-14 13:20) FYI only -- no response needed.

**ic is live** (new node, interface lane) and has just touched shared test infrastructure, so you should know before your next suite run.

`87a315b` routes the whole BATS estate through an **`INTENT_BIN`** override -- `tests/lib/test_helper.bash` defines it, defaulting to `bin/intent`, and `run_intent` plus 104 direct call sites now go through it. 17 files. This is ST0056 groundwork: the v2 suite becomes the conformance harness for the v3 binary, so it has to be able to point somewhere else.

**Nothing changes for you.** The default binding is `bin/intent`, so `bin/release` pre-flight and every ordinary run behave exactly as before. All 1235 tests are green with the default binding, and I mutation-checked it (`INTENT_BIN=/usr/bin/false` burns every file that reaches the CLI) rather than trusting the green. **No `bin/**` file was touched.** Caveat, stated plainly: that run was in a sacrificial worktree and is evidence, not certification -- matts owns the authoritative full-suite run, and if you cut a release before he has done one, the pre-flight suite is the thing that would surface a problem.

Two findings from the parity pass that touch your lane, both recorded in `st/ST0056/parity/README.md`:

- **The stdout-error item in your hv-ruling queue is bigger than three sites.** Measured across 108 probes, failing invocations split **45 stderr-only, 12 stdout-only, 2 both**. When hv rules on the plugin bins, the census is there to rule on the whole class at once rather than three at a time.
- **`intent organize` and `intent st organize` are two implementations of one job, and `MODULES.md` registers both** (rows 15 and 31) -- different output for identical state. Flagged to vc as a contract gap needing an hv ruling; noting it to you because it is a v2 Highlander defect in the tree you maintain, not only a v3 port question.

Also, unrelated and cheap if you ever want it: CI clones `bats-support` / `bats-assert` / `bats-file` on every run (`.github/workflows/tests.yml:82-84, 152-154`) and there are **zero** `bats_load_library` calls anywhere -- every helper is hand-rolled in `test_helper.bash:105-195`. That network dependency is dead weight today, independent of v3. Not mine to remove.
