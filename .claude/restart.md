# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Whiteboard present (`intent/whiteboard/`, hv+cc+vc) -- `/in-session` chains `/in-whiteboard pickup`. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.19.0 SHIPPED (tag `071c612`)

**Shipped 2026-08-14.** `bin/release --minor` cut cleanly at tag `071c612`: pre-flight green (doctor + full suite at HEAD), five sidecars stamped to 2.19.0, CHANGELOG dated, pushed to `local` + `upstream`, GitHub release published with the CHANGELOG `[2.19.0]` section as its body. vc verified the cut. **The next action is the consumer sweeps** (see below).

**What v2.19.0 is:** fifteen issues closed (0009-0023). The centrepiece is the AT row grammar (0017, subsuming 0014 + 0015): two anchored arms, `intent at lint` L1-L5, a `--fix` that refuses what it cannot migrate without loss, the close-gate honouring the grammar from day one, and an `at_grammar` ledger step so consumers are swept by upgrading. Alongside: four AC states with descope/rescope + withdraw/reinstate (0013 + hv verb); THE steel-thread enumerator, a voiced `organize`, and a doctor duplicate-id check (0011); the whiteboard header block ruled not-YAML and enforced (0012); `intent claude hook <name>` runtime resolution, settings.json byte-identical everywhere (0016); AGENTS.md prerequisites from declared languages + upgrade convergence (0009); objective-placeholder warning at close (0010); treeindex cache untracked + ignored, consumers converge (0018); the canonical thread index actually indexes all threads (0019); `st list --status all` stops silently discarding what it cannot classify (0020); the dead `credo_checks/` mechanism pruned + a three-state doctor residue report (0021, Laksa); both no-template fallback heredocs deleted rather than corrected (0022); `error()` and its 25 imitators moved to the lowercase voice, finishing `8aba5ab` (0023). **0020, 0021, 0022 and 0023 were all called in by hv BEFORE the cut** -- the batching principle, four times, and why the release grew from eleven issues to fifteen between build and tag. Release docs written pre-cut: `intent/history/v2.19.0.md` + `docs/releases/2.19.0/RELEASE_NOTES.md`. `--fix` was hardened three times on real-estate evidence, and the 87 name links its lossy first version destroyed in our own contracts were restored from git (`ee44f63`).

**New practice adopted here: write the release docs BEFORE the cut so the tag carries them.** The previous write-them-after habit is why v2.17.x and v2.18.0 have neither a history narrative nor public notes; resumed at 2.19.0, deliberately not backfilled.

## Next

1. **Consumer sweeps, one `intent upgrade` each** (Lamplight first: 314 AT rows + four bad-status contracts; expect BLOCKED-until-swept -- that is the fix working; residue is named, never guessed). The same pass converges AGENTS.md, portable-hook settings.json, gitignore entries, and PRINTS (never runs) any treeindex `git rm`. **Claimed by cc**; vc's part is measuring the delta against git before the sweep is trusted.
2. **`credo_checks/` cleanup, issues filed 2026-08-14** in the affected projects: Baize 0001 + Lamplight 0003 (both wired and RUNNING on Credo 1.7.19 with the crash-candidate `missing_impl_annotation.ex` present -- run `mix credo --strict` first; Baize also carries `struct_vars.ex`, which is theirs, so no blanket delete), Conflab 0008 (loaded but registered nowhere, never run, and a genuine two-ended removal via `elixirc_paths`). Laksa + Prolix measured clean.
3. Push fleet issue-normalisation commits (Utilz `0171297`, Lamplight `7058fd3a8`).
4. hv-ruling queue: 0004 item 4; `javascript` pack (0009's Node exception); pruning consumers' inert `.claude/scripts/` copies; the plugin bins writing errors to STDOUT (named in 0023 and left -- it changes what callers capture) alongside `intent_claude_prime:212`.
5. Whiteboard: inboxes are read at pickup only, so a node asking mid-session cannot see an answer appended to its inbox. Cost two round-trips on 2026-08-14. Protocol change, wants an hv ruling on shape.

## Standing lessons (this cycle)

Grep for a Highlander rule, never read for it (the guard found what two reading passes missed). Mutation-test every guard before believing it (five could not fail as first written). A migrator must not do half of a two-ended migration -- refuse and name everything. Diagnose by running, not reading (five claims fell to three-minute repros, two of them the reviewer's own). Run the real path in a sacrificial copy.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `--no-confirm` on the release. Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `intent build release` date them at cut time.
