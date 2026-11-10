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

### WP-02 -- kovacs runs as the cut's remote leg, and prepush keeps only its push gates (status: Not Started)

- AC-02.1 kovacs runs Intent's declared gates at the release commit as a `release.legs` leg inside the cut, and `bin/.devbin/preflight-linux.sh` and `bin/.devbin/cmd/shared/preflight.lib` are gone. -- satisfied: no (computed)
- AC-02.2 `bin/int prepush` keeps its frozen-remote refusal, and a push neither requires a kovacs summary nor clones and builds the workspace. -- satisfied: no (computed)

### WP-03 -- The tap and the macOS assets publish through the built-ins, with prepare as a shadow (status: Not Started)

- AC-03.1 The built-in formula writes Intent's formula and cask in assets mode under the cut's yes, each asset's sum filled from the release's sums file, which stands for issue 0634's cask sha256 check; Intent's own brew smoke runs as an `after:` command under the same yes, where it reports after publication and gates nothing; and `bin/.devbin/cmd/brew` keeps the smoke and nothing else, with no render or publish code left in it or in `bin/.devbin/cmd/shared/brew.lib`. -- satisfied: no (computed)
- AC-03.2 The built-in `macos publish` publishes the stage Intent's `macos prepare` makes, that stage carries `<stage>/commit` with HEAD's full sha, and `bin/.devbin/cmd/macos` carries no publish. -- satisfied: no (computed)
- AC-03.3 (non-test) The service block in Intent's formula template points log_path and error_log_path at D19's intentd.log and intentd.err.log under the user's state dir, the files intent daemon logs reads, and its comment says intentd writes every line to stdout or stderr and names where each goes (issue 0667, ruled by hv on 2026-10-31). -- satisfied: no

### WP-04 -- The workflows fire on the candidate, and the docs describe the cut as built (status: WIP)

- AC-04.1 `tests.yml`, `rust.yml` and `linux-release.yml` fire on `candidate/` refs and not on tags, the macOS legs included, and `bin/devbin release check` passes on Intent's tree. -- satisfied: no (computed)
- AC-04.2 (non-test) `intent/docs/releasing.md` and `.github/workflows/README.md` describe the cut as built: one command from the yes to the tap with no step between them run by hand, a dry run that predicts and runs no gate, then `brew upgrade` and `intent use brew` on each machine. -- satisfied: no
- AC-04.3 (non-test) hv cuts Intent's first release through the core with no hand step between the yes and the tap, and vc records that cut's RELEASE seal clock lines against the 3.2.8 baseline. -- satisfied: no

## Acceptance Tests

### WP-01 -- The cut runs through devbin's core, and Intent's driver retires (status: WIP)

_(no tests in this group)_

### WP-02 -- kovacs runs as the cut's remote leg, and prepush keeps only its push gates (status: Not Started)

_(no tests in this group)_

### WP-03 -- The tap and the macOS assets publish through the built-ins, with prepare as a shadow (status: Not Started)

_(no tests in this group)_

### WP-04 -- The workflows fire on the candidate, and the docs describe the cut as built (status: WIP)

- AT-04.1 `tests/unit/release_candidate_trigger.bats` -- covers AC-04.1 -- status: to-write -- Minted by dc with the train's first bank, refs/bank/dc/st0091/wp04-trigger-v1 (blob ee3409fcdced, patch-id 2c4e9bc3ab35, base 991e92946). It stays to-write until the train lands: the file is in the bank and not yet on main, and `intent at red` refuses a row whose cited file is absent, since that would be a live absent_at finding. The judge of the train's tip sets its verdict. Its three tests: a push of the cut's candidate starts release.ci.workflow as committed at HEAD, asked of devbin's own reader (`ci-state --fires`); no tag push starts tests.yml, rust.yml or linux-release.yml; and a candidate's run carries the macOS leg and rust.yml's job, each keyed on the candidate prefix config names. Each test reads the candidate template from config, never a literal. Each red was driven, and each is the base's own line restored: test 1 with HEAD at 991e92946, whose committed tests.yml admits no candidate; test 2 with linux-release.yml's base L25, `tags: ["v[0-9]*"]`, back under push; test 3 with tests.yml's base L292, test-macos's tag condition, back. AC-04.1's release-check half is judged at the tip, once WP-01 offers cut.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
