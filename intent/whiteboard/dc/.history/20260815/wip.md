# dc -- archived 2026-08-15 10:55Z (localfold, day one)

Node created and picked up 08:57Z. Everything below is DONE and landed on both remotes; the live board keeps only what is still open.

## Charter, as established

hv's words via `whiteboard/README.md`: _dev-x and build environment, so that cc concentrates on functionality for the CLI / daemon_. cc, ic and vc each wrote an intro within two minutes of each other (08:54Z / 08:55Z / 08:55Z) and their three independent readings of the lane agreed, which is worth more than any one of them.

Boundary adopted as **proposed-pending-hv**, in cc's words: `bin/intent`, `bin/intent_*` -> cc; `bin/int`, `bin/devbin`, `bin/.devbin/**` -> dc. The load-bearing argument is not ownership but the freeze -- `bin/intent*` is the measurement baseline the whole parity contract is scored against, so if it moves, ic's burn figures and register rows are measuring a moving target. "cc's" means "cc is the one who has to refuse".

## Landed -- ten commits, both remotes, CI green on all four legs

    f8948cc  guards wired + CI un-swallowed + *.bak class + toolchain pin refused
    7acab9c  bin/int prepush -- clone HEAD, build it, run the binaries
    bfd3e16  whiteboard commits off the test matrix
    49f0676  bin/int hooks -- three-state wiring report
    73e5d64  bin/int cache -- stale-cache detection
    9defbbd  int hooks false-ABSENT fix (worktrees)
    0a2e72b  int cache severity split (cc's discriminator)
    plus board folds

**The two guards (hv approved, a flat "Ok").** `bin/int precommit` runs `provenance_check.sh` and `view_skew_check.sh`, chained LAST in `.git/hooks/pre-commit` so the gate sees the set that actually lands rather than the pre-prettier one. Deliberately not in `lib/templates/hooks/pre-commit.sh`, which ships to every consumer while these guards check `intent/st/ST0056/parity/**`. Not merged, either -- two scripts, two invariants, two messages, and the runner names which refused; vc's separation ruling intact. Mutation-tested with four canaries in a sacrificial worktree, all discriminating.

**CI could not fail on integration tests.** A separate step ran one of the two integration files under `|| echo`, so it always exited 0; `run_tests.sh` already covers `integration/` and propagates status, so the step was redundant as well as toothless. Deleted along with a `bats || true` fallback. Measured before deleting: the suite passes, so the swallow was hiding nothing at the time. Result on `f8948cc`: `rust` success both legs, `Intent Tests` success all four -- and the second-order fact is the one that matters, that the suite is now green through a workflow that CAN fail on integration tests.

**`bin/int prepush`** -- clone HEAD, build it, run the binaries. vc left the pre-commit/pre-push/CI trade to me and measurement settled it: ~16s cold, too slow per-commit, and CI reports only after the bad state is already public. Path-triggered; a whiteboard-only push skips in 0.5s. Canaried against a fixture reproducing `a1a949c`'s exact signature -- root `Cargo.toml` plus `native/rust/Cargo.toml` both at HEAD -- REFUSED, both manifests named.

**The repository at HEAD verified sound**, which nobody had done since the half-move: fresh clone, cold build 14.02s, `intent` and `intentd` both produced and both running `3.0.0-dev`, exactly one workspace manifest, no stray root manifest, no duplicate tree.

**`*.bak` ignored as a class**; `/AGENTS.md.bak` pruned as redundant. `.claude/settings.local.json.bak` was the only unignored untracked file in a repository `gh` reports PUBLIC. vc's finding, verified here rather than taken on report.

**Whiteboard commits off the test matrix**, as an ignore-list rather than an allow-list: `paths-ignore` fails safe, `paths:` fails silent. Only `intent/whiteboard/**` went in, measured inert -- the two suites naming those paths both `create_test_project` first. `intent/st/**` deliberately NOT ignored despite the temptation, because I had not measured whether its suites use fixtures.

**`int hooks`** (vc's ruling, check-first). Three states -- WIRED / UNWIRED / ABSENT -- because the middle one, a hook that is present and executable and invoking something else, looks installed and protects nothing. Guard names read from the runner, never listed, so the roster cannot rot. Four canaries: fresh clone -> ABSENT (the hole reproduced); legacy hook -> UNWIRED; `--install` -> wires both and pre-existing content survives; three installs -> idempotent.

**`int cache`** -- cc's stale-cache class, measured still live: 228 dep files naming `native/target`, a directory that does not exist. Discriminated against a cold clone at the same revision (zero stale, 222 correct) so the difference is the cache, not the code. Warns rather than refusing, because the residue is latent and a gate that refuses over a usually-harmless condition gets switched off. cc then supplied the severity split -- 181 superseded vs 30 with no sibling, all proc-macro host artefacts, i.e. exactly the code-generating half -- and cleaned it: 32s, 246 tests passed, 3.1G -> 1.7G.

## Refusals, both deliberate

**No `rust-toolchain.toml`.** Measured: cargo and rustc are Homebrew's real binaries, rustup is NOT INSTALLED, and a file pinning 1.70.0 is ignored in silence. It would bind CI alone while reading as a project-wide guarantee. `rust.yml` records the toolchain per run instead, which produced a previously-unknown fact: CI and local are both `rustc 1.97.1 (8bab26f4f 2026-07-14)`, commit hash included.

**The auto-mode brief write was refused by the harness classifier** and not routed around, and vc was not asked to do it instead.

## Corrections made and taken

- **vc's** "two defects in that CI line" -- the `$?` half does not hold; `A || echo "$?"` propagates correctly (tested at 42 and 7). vc re-tested and withdrew it.
- **ic's** "unstaging is lossless where worktree == HEAD" -- backwards; that condition is exactly when the index holds the only copy. vc ruled the same.
- **Mine**: the path filter walk-back. A board-only commit still fired the suite because my push carried ic's `20e8c4b`. Mechanism correct; my commit message would have had a reader over-expect.
- **Mine**: I shipped a false-ABSENT defect in `int hooks` that lived forty minutes. In a linked worktree `.git` is a FILE, so `$ROOT/.git/hooks` does not resolve and the check reported ABSENT on a wired worktree -- the exact failure that file's own comment claimed it prevented. Fixed by asking `git rev-parse --git-path hooks` instead of reimplementing git's rule.
- **vc's** withdrawn `core.hooksPath` defect. Published at `high`, refuted by cc, withdrawn after re-running: `intent claude upgrade` resolves through `canon_hooks_dir()` and follows a redirect. vc's error was grepping for the string and concluding the mechanism was absent -- the correct API never needs to name it.
