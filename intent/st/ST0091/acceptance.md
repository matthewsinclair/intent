---
st_id: ST0091
title: Intent releases through devbin's core: its driver, kovacs preflight and pre-push build retire
---

# ST0091: Intent releases through devbin's core: its driver, kovacs preflight and pre-push build retire -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- The cut runs through devbin's core, and Intent's driver retires (status: WIP)

- AC-01.1 `bin/devbin release` cuts Intent through devbin's core: the schema bless, the lock refresh, `agents sync`, `claude upgrade --apply` and `sync --to-disk` run as `derived:` steps, `bin/devbin release check` names every derived step and gate, the dry run names every leg and runs no derived step and no gate, and `bin/.devbin/cmd/build.d/release` is gone. -- satisfied: no (computed)
- AC-01.2 Intent's own checks run as declared `release.gates`: the bats suite, the cargo gate with issue 0481's daemon-family re-run inside Intent's own gate command, the ST0056 contract and reference checks, the embedded-source-commit check keyed on the release commit, and, on macOS alone, the pair build at the release commit; a sealed green for the release commit on this platform is not run again. -- satisfied: no (computed)
- AC-01.3 The cut and its dry run refuse while an intentd is live, before anything is stamped, and the remedy says how to stop the daemon. -- satisfied: no (computed)
- AC-01.4 The pair the cut builds at the release commit passes issue 0518's currency verdict when the release push runs. -- satisfied: no (computed)

### WP-02 -- kovacs runs as the cut's remote leg, and prepush keeps only its push gates (status: WIP)

- AC-02.1 kovacs runs Intent's declared gates at the release commit as a `release.legs` leg inside the cut, and `bin/.devbin/preflight-linux.sh` and `bin/.devbin/cmd/shared/preflight.lib` are gone. -- satisfied: no (computed)
- AC-02.2 `bin/int prepush` keeps its frozen-remote refusal, and a push neither requires a kovacs summary nor clones and builds the workspace. -- satisfied: no (computed)

### WP-03 -- The tap and the macOS assets publish through the built-ins, with prepare as a shadow (status: WIP)

- AC-03.1 The cut publishes through one `after:` command that stops at its first failure and is safe to run again at the tag: it builds the pair at the tag, stages, publishes the macOS assets, and then runs the built-in `formula publish`, which renders Intent's formula and cask from templates under `packaging/homebrew/` in assets mode under the cut's yes, each asset's sum filled from the release's sums file, which stands for issue 0634's cask sha256 check; the tap is not written when an earlier step failed; Intent's own brew smoke runs as the next `after:` command, where it reports after publication and gates nothing; and `bin/.devbin/cmd/brew` keeps the smoke and nothing else, with no render or publish code left in it or in `bin/.devbin/cmd/shared/brew.lib`. -- satisfied: no (computed)
- AC-03.2 Intent's `macos prepare` refuses to stage a pair whose kind is not `release` or whose embedded commit is not HEAD, and names the rebuild at the tag as its remedy; Intent's `macos publish` ships the stage under the asset names 3.2.8 shipped, so no installed keg and no formula line changes name; and the built-in macOS publish is not adopted until a devbin stages after the tag and ships a project's several assets. -- satisfied: no (computed)
- AC-03.3 (non-test) The service block in Intent's formula template points log_path and error_log_path at D19's intentd.log and intentd.err.log under the user's state dir, the files intent daemon logs reads, and its comment says intentd writes every line to stdout or stderr and names where each goes (issue 0667, ruled by hv on 2026-10-31). -- satisfied: no

### WP-04 -- The workflows fire on the candidate, and the docs describe the cut as built (status: WIP)

- AC-04.1 `tests.yml` and `linux-release.yml` fire on `candidate/` refs and not on tags; `rust.yml` runs inside `tests.yml`'s run on a candidate, so the cut reads one verdict, and no tag starts it; the macOS legs run on a candidate, and a candidate's run that skipped a leg does not read green; and `bin/devbin release check` passes on Intent's tree. -- satisfied: no (computed)
- AC-04.2 (non-test) `intent/docs/releasing.md` and `.github/workflows/README.md` describe the cut as built: one command from the yes to the tap with no step between them run by hand, a dry run that predicts and runs no gate, then `brew upgrade` and `intent use brew` on each machine. -- satisfied: no
- AC-04.3 (non-test) hv cuts Intent's first release through the core with no hand step between the yes and the tap, and vc records that cut's RELEASE seal clock lines against the 3.2.8 baseline. -- satisfied: no

## Acceptance Tests

### WP-01 -- The cut runs through devbin's core, and Intent's driver retires (status: WIP)

- AT-01.1 `tests/unit/release_stamp.bats` -- covers AC-01.1 -- status: red -- The cut's seven derived steps, each run as the core runs it; banked at refs/bank/dc/st0091/wp01a-v1 (blob c96b329f9f42, patch-id 6c2b14cb5b1c), not yet on main; it stays to-write until the train lands, where it moves red then green against the judge run. Moved red at its landing (2026-11-10, dc), in the commit that adds the file, with the file's id lines (vc's ruling). The bank that landed is wp01a-v2 (blob 91b283aadaca, patch-id 118866bca26c), which superseded the wp01a-v1 named above. Green is vc's, from its judged run of the train. Red on the code as it was: all 7 arms red on the tree before bank 2 (8770037a2), driven by dc at 17:1xZ; all 7 pass on this commit.
- AT-01.2 `tests/unit/releasestamp_lib.bats` -- covers AC-01.1 -- status: red -- The cut's helpers in releasestamp.lib, the lock and footer arms re-homed from the retired driver's tests; banked at refs/bank/dc/st0091/wp01a-v1 (blob c96b329f9f42, patch-id 6c2b14cb5b1c), not yet on main; it stays to-write until the train lands, where it moves red then green against the judge run. Moved red at its landing (2026-11-10, dc), in the commit that adds the file, with the file's id lines (vc's ruling). The bank that landed is wp01a-v2 (blob 91b283aadaca, patch-id 118866bca26c), which superseded the wp01a-v1 named above. Green is vc's, from its judged run of the train. Red on the code as it was: all 13 arms red on the tree before bank 2 (8770037a2); all 13 pass on this commit.
- AT-01.3 `tests/unit/release_preflight.bats` -- covers AC-01.3 -- status: red -- The cut's four read-only pre-flight checks, a live intentd refused with its remedy; banked at refs/bank/dc/st0091/wp01a-v1 (blob c96b329f9f42, patch-id 6c2b14cb5b1c), not yet on main; it stays to-write until the train lands, where it moves red then green against the judge run. Moved red at its landing (2026-11-10, dc), in the commit that adds the file, with the file's id lines (vc's ruling). The bank that landed is wp01a-v2 (blob 91b283aadaca, patch-id 118866bca26c), which superseded the wp01a-v1 named above. Green is vc's, from its judged run of the train. Red on the code as it was: all 9 arms red on the tree before bank 2 (8770037a2); all 9 pass on this commit.
- AT-01.5 `tests/unit/release_gate.bats` -- covers AC-01.2 -- status: to-write -- The cut's gates on the release commit, each run as the core runs it; banked at refs/bank/dc/st0091/wp01b-v1 (blob 3e81cb71c66c, patch-id 184485758ff4), on wp01a-v2, not yet on main; it stays to-write until the train lands, where it moves red then green against the judge run.

### WP-02 -- kovacs runs as the cut's remote leg, and prepush keeps only its push gates (status: WIP)

_(no tests in this group)_

### WP-03 -- The tap and the macOS assets publish through the built-ins, with prepare as a shadow (status: WIP)

- AT-03.1 `tests/unit/release_publish.bats` -- covers AC-03.1 -- status: to-write -- The cut's publication as one after: command that stops at its first failure and is safe to run again; banked at refs/bank/dc/st0091/wp03-v1 (blob d168a870bb99, patch-id 6f6bb9badad3), on wp02-v1's tip tree, not yet on main; it stays to-write until the train lands. Run on the tree before WP-03, every arm of the file is red.
- AT-03.2 `tests/unit/release_publish.bats` -- covers AC-03.2 -- status: to-write -- macos prepare refuses a pair that is not a release build of HEAD, and the formula names the stage's assets; banked at refs/bank/dc/st0091/wp03-v1 (blob d168a870bb99, patch-id 6f6bb9badad3), on wp02-v1's tip tree, not yet on main; it stays to-write until the train lands. Run on the tree before WP-03, every arm of the file is red.

### WP-04 -- The workflows fire on the candidate, and the docs describe the cut as built (status: WIP)

- AT-04.1 `tests/unit/release_candidate_trigger.bats` -- covers AC-04.1 -- status: red -- Minted by dc with the train's first bank, refs/bank/dc/st0091/wp04-trigger-v1 (blob ee3409fcdced, patch-id 2c4e9bc3ab35, base 991e92946). It stays to-write until the train lands: the file is in the bank and not yet on main, and `intent at red` refuses a row whose cited file is absent, since that would be a live absent_at finding. The judge of the train's tip sets its verdict. Its three tests: a push of the cut's candidate starts release.ci.workflow as committed at HEAD, asked of devbin's own reader (`ci-state --fires`); no tag push starts tests.yml, rust.yml or linux-release.yml; and a candidate's run carries the macOS leg and rust.yml's job, each keyed on the candidate prefix config names. Each test reads the candidate template from config, never a literal. Each red was driven, and each is the base's own line restored: test 1 with HEAD at 991e92946, whose committed tests.yml admits no candidate; test 2 with linux-release.yml's base L25, `tags: ["v[0-9]*"]`, back under push; test 3 with tests.yml's base L292, test-macos's tag condition, back. AC-04.1's release-check half is judged at the tip, once WP-01 offers cut. Moved red at its landing (2026-11-10, dc), in the commit that adds the file, on vc's ruling that a row leaves to-write with the file's id lines: this commit carries the file's AT-04.1 header and arm names, which wp03-v3 held. Red on the code as it was: the trigger bank drove each of the three arms against the base's own trigger lines, all three red. All three pass on this commit, run on a scratch commit of it before landing. Green is vc's, from its judged run of the train.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
