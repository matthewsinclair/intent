# Claude Code Session Restart -- narrative state

## Current state (2026-08-14)

**v2.19.0 is built, verified, documented and unshipped; the cut is the next action.** Twelve issues (0009-0020) closed in one day-long arc: vc triaged the filed corpus against HEAD with mechanical repros (correcting two filed root causes before anything was built), cc executed seven units plus a bounce of five audit findings, hv ratified four rulings and added two, and the release was then hardened three more times on real-estate evidence -- twice on consumer reports against `at lint --fix` (a glob-class guessing hazard; lossiness that destroyed test-name links), once on hv's suite run (a width test asserting an incidental scope coupling as a contract). The 87 name links the lossy fixer destroyed in this repo's own completed contracts were measured and restored verbatim from git (`ee44f63`). Full suite green at HEAD, hv-run. The substance of the release: the AT row grammar with `at lint` L1-L5 and a refuse-what-you-cannot-migrate `--fix`; four AC states (descope/rescope, withdraw/reinstate); one steel-thread enumerator; the whiteboard header block ruled not-YAML; runtime-resolved hooks; declared-language prerequisites; the objective warning; the treeindex cache untracked; the canonical index actually indexing; and `st list --status all` actually meaning all (0020, called in by hv before the cut rather than after). Release docs written pre-cut so the tag carries them: `intent/history/v2.19.0.md` + `docs/releases/2.19.0/RELEASE_NOTES.md`.

## The release checklist (next session, first thing)

1. Tree clean, suite green (both already true; re-verify trivially if anything moved overnight). **Clean means ALL of it** -- the script aborts on anything dirty outside its five sidecars, including another node's whiteboard board.
2. `bin/release --minor` -- interactive, NEVER `--no-confirm`. Its pre-flight re-runs `intent doctor` + the full suite (not behind the dry-run guard, so a dry-run costs a suite run too), then stamps all five sidecars (VERSION, config.json, CLAUDE.md, AGENTS.md, CHANGELOG date), commits as `release: v2.19.0`, tags, and asks once before pushing both remotes.
3. Post-cut: flip `intent/done.md`'s entry to shipped + tag; verify the sidecars, the CHANGELOG date, the tag on both remotes, and that the GitHub release body matches the CHANGELOG section; globalfold refresh. The narrative and public notes are already written and in the tag.
4. Then the consumer sweeps and the rest of `intent/wip.md` Next Up.

## Standing lessons from this cycle

- **Grep for a Highlander rule; do not read for it.** Reading found three of five enumerator copies across two passes; the mechanical guard found the last two immediately. A guard scoped to what is already clean certifies the status quo.
- **Mutation-test every guard before believing it.** Five guards written this cycle could not fail as first written (an invalid ERE swallowed by `|| true` -- in the test guarding the silent-swallow issue; substring assertions; a decoy the path rule rejected anyway; a vacuous applied-probe). Break the behaviour, watch the right test fail, restore.
- **A migrator must not do half of a two-ended migration.** The lossy `--fix` stripped test names before the id existed in the test, converting an honest finding into a misleading one on a row that read complete and proved nothing. Refuse + name everything beats guessing -- and the SUGGESTION was the worse half, because humans followed it.
- **The reviewer's directives are findings-in-waiting.** The lossy strip was the review's own work-order delta, passed again by the reviewer's audit, caught by a consumer measuring. The two filed-record corrections cut the other way the same day. Diagnose by running, not reading -- five separate claims fell to a three-minute repro this cycle.
- **Run the real path in a sacrificial copy** (three more instances: `--fix` comparing repaired paths to the raw form; the AGENTS.md convergence ordered before canon apply; an upgrade fixture already at the target version proving nothing).

## Where detail lives

- CHANGELOG `[2.19.0]` -- the comprehensive narrative until the history doc exists. `intent/issues/CLOSED/0009..0019` -- per-issue record incl. Resolutions with every judgement call and all six recorded mistakes. `intent/done.md` -- terse ledger. Whiteboard `.history/20260814/` per node -- the coordination record.
- Guards worth knowing: `at_grammar_lint.bats` (grammar + lint + fix + the `@` seam), `ac_offscope_states.bats` (four states), `st_enumeration.bats` (THE enumerator + doctor), `objective_placeholder.bats` (0010 + template drift), `no_absolute_home_paths.bats` (0016/0018), `output_width.bats` (scope-matched width parity), `helpers.bats` (voice, enumerator, stamper), `st_commands.bats` (the index row lands).

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full suite externally (single-file bats fine); matts is the acceptance verifier; never `bin/release --no-confirm`; author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/release` date them at cut time.
