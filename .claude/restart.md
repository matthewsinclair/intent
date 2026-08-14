# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Whiteboard present (`intent/whiteboard/`, hv+cc+vc) -- `/in-session` chains `/in-whiteboard pickup`. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.19.0 BUILT + GREEN, CUT PENDING

**The next action is the release cut** (`bin/release --minor`; interactive; NEVER `--no-confirm`; it stamps all five sidecars and dates the CHANGELOG at the cut -- VERSION/config are deliberately still 2.18.0 until then).

**What v2.19.0 is:** fourteen issues closed (0009-0022). The centrepiece is the AT row grammar (0017, subsuming 0014 + 0015): two anchored arms, `intent at lint` L1-L5, a `--fix` that refuses what it cannot migrate without loss, the close-gate honouring the grammar from day one, and an `at_grammar` ledger step so consumers are swept by upgrading. Alongside: four AC states with descope/rescope + withdraw/reinstate (0013 + hv verb); THE steel-thread enumerator, a voiced `organize`, and a doctor duplicate-id check (0011); the whiteboard header block ruled not-YAML and enforced (0012); `intent claude hook <name>` runtime resolution, settings.json byte-identical everywhere (0016); AGENTS.md prerequisites from declared languages + upgrade convergence (0009); objective-placeholder warning at close (0010); treeindex cache untracked + ignored, consumers converge (0018); the canonical thread index actually indexes all threads (0019); `st list --status all` stops silently discarding what it cannot classify (0020); the dead `credo_checks/` mechanism pruned + a three-state doctor residue report (0021, Laksa); 0020 and 0021 both called in by hv before the cut; `warning()` lowercase voice. Release docs written pre-cut: `intent/history/v2.19.0.md` + `docs/releases/2.19.0/RELEASE_NOTES.md`. `--fix` was hardened three times on real-estate evidence, and the 87 name links its lossy first version destroyed in our own contracts were restored from git (`ee44f63`).

**Suite green at HEAD (hv-run, 2026-08-14, post-0020). Nothing pushed; no tag.** `bin/release` re-runs `intent doctor` + the full suite as its own pre-flight, so the cut is self-certifying -- and it refuses to tag if ANYTHING outside its five sidecars is dirty, another node's whiteboard board included.

## After the cut

1. done.md entry flips to shipped + tag; verify the five sidecars, the CHANGELOG date, the tag on both remotes, and the GitHub release body against the CHANGELOG section. (The narrative and public notes were written pre-cut, so they are already in the tag.)
2. **Consumer sweeps, one `intent upgrade` each** (Lamplight first: 314 AT rows + four bad-status contracts; expect BLOCKED-until-swept -- that is the fix working; residue is named, never guessed). The same pass converges AGENTS.md, portable-hook settings.json, gitignore entries, and PRINTS (never runs) any treeindex `git rm`.
3. Push fleet issue-normalisation commits (Utilz `0171297`, Lamplight `7058fd3a8`).
4. hv-ruling queue: 0004 item 4; `javascript` pack (0009's Node exception); pruning consumers' inert `.claude/scripts/` copies; `intent_claude_prime:212` voice+stream.

## Standing lessons (this cycle)

Grep for a Highlander rule, never read for it (the guard found what two reading passes missed). Mutation-test every guard before believing it (five could not fail as first written). A migrator must not do half of a two-ended migration -- refuse and name everything. Diagnose by running, not reading (five claims fell to three-minute repros, two of them the reviewer's own). Run the real path in a sacrificial copy.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `bin/release --no-confirm`. Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/release` date them at cut time.
