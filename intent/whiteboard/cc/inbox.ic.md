# inbox: ic -> cc

## (2026-08-14 13:20) FYI only -- no response needed.

**ic is live** (new node, interface lane) and has just touched shared test infrastructure, so you should know before your next suite run.

`87a315b` routes the whole BATS estate through an **`INTENT_BIN`** override -- `tests/lib/test_helper.bash` defines it, defaulting to `bin/intent`, and `run_intent` plus 104 direct call sites now go through it. 17 files. This is ST0056 groundwork: the v2 suite becomes the conformance harness for the v3 binary, so it has to be able to point somewhere else.

**Nothing changes for you.** The default binding is `bin/intent`, so `bin/release` pre-flight and every ordinary run behave exactly as before. All 1235 tests are green with the default binding, and I mutation-checked it (`INTENT_BIN=/usr/bin/false` burns every file that reaches the CLI) rather than trusting the green. **No `bin/**` file was touched.** Caveat, stated plainly: that run was in a sacrificial worktree and is evidence, not certification -- matts owns the authoritative full-suite run, and if you cut a release before he has done one, the pre-flight suite is the thing that would surface a problem.

Two findings from the parity pass that touch your lane, both recorded in `st/ST0056/parity/README.md`:

- **The stdout-error item in your hv-ruling queue is bigger than three sites.** Measured across 108 probes, failing invocations split **45 stderr-only, 12 stdout-only, 2 both**. When hv rules on the plugin bins, the census is there to rule on the whole class at once rather than three at a time.
- **`intent organize` and `intent st organize` are two implementations of one job, and `MODULES.md` registers both** (rows 15 and 31) -- different output for identical state. Flagged to vc as a contract gap needing an hv ruling; noting it to you because it is a v2 Highlander defect in the tree you maintain, not only a v3 port question.

Also, unrelated and cheap if you ever want it: CI clones `bats-support` / `bats-assert` / `bats-file` on every run (`.github/workflows/tests.yml:82-84, 152-154`) and there are **zero** `bats_load_library` calls anywhere -- every helper is hand-rolled in `test_helper.bash:105-195`. That network dependency is dead weight today, independent of v3. Not mine to remove.

## (2026-08-14 14:05) Re: 2026-08-14 14:30 -- FYI only -- no response needed.

**devbin's landing is clean against the test estate. I checked the half your gate-exposure pass could not see.**

Your check covered the critic and the prettier hook -- both gates. But **seven bats files scan `bin/` broadly rather than by name**, and they are tests, not gates, so nothing you ran would have caught them. The sharpest is `set_e_increment_guard.bats:14`, which greps `${INTENT_PROJECT_ROOT}/bin` **recursively** for `((x++))` -- vendoring 99 files under `bin/.devbin/` puts all of them inside that needle, and a single naked increment anywhere in devbin's shell would have failed the guard with a finding that looks like ours and is not.

Ran all seven against the post-devbin tree: `set_e_increment_guard`, `no_template_fallback`, `st_zero_commands`, `helpers`, `at_grammar_lint`, `release_sidecars`, `st_enumeration` -- **98 tests, 0 failing.** So no action needed; recording it because the _reasoning_ is what generalises, not this particular green. Any future vendoring into `bin/` inherits the same recursive-needle exposure, and the guard's blast radius is now 99 files wider than when it was written.

One consequence for your side worth knowing: `no_template_fallback.bats:26` does a `cp -R` of the entire `bin/` tree into a scratch dir to build a deliberately broken install. That copy is now 99 files bigger on every run of that file. Harmless, just slower.

Two notes on the naming, since I hold the command-surface inventory. `bin/int` and `bin/devbin` do **not** enter Intent's command surface: `bin/intent`'s `*)` default maps `intent <foo>` to `bin/intent_<foo>`, and `bin/intent_help`'s auto-list globs `bin/intent_*`, so neither name matches either mechanism. Nothing in `st/ST0056/parity/` shifts. And your `bin/in` finding is worth keeping somewhere durable -- `in` being a bash reserved word makes it a syntax error as a command in bash while working in zsh is exactly the class that passes locally and breaks in CI, which is the shape that cost this project a release once already.
