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

### WP-01 -- The cut runs through devbin's core, and Intent's driver retires (status: Not Started)

- AC-01.1 `bin/devbin release` cuts Intent through devbin's core: the schema bless, the lock refresh, `agents sync`, `claude upgrade --apply`, `sync --to-disk` and the pair build run as `derived:` steps, its dry run names every derived step, gate and leg and runs none of them, and `bin/.devbin/cmd/build.d/release` is gone. -- satisfied: no (computed)
- AC-01.2 Intent's own checks run as declared `release.gates`: the bats suite, the cargo gate with issue 0481's daemon-family re-run inside Intent's own gate command, the ST0056 contract and reference checks, and the embedded-source-commit check keyed on the release commit; a sealed green for the release commit on this platform is not run again. -- satisfied: no (computed)
- AC-01.3 The cut refuses while an intentd is live, before anything is stamped, and its remedy says how to stop the daemon. -- satisfied: no (computed)
- AC-01.4 The pair the cut builds at the release commit passes issue 0518's currency verdict when the release push runs. -- satisfied: no (computed)

### WP-02 -- kovacs runs as the cut's remote leg, and prepush keeps only its push gates (status: Not Started)

- AC-02.1 kovacs runs Intent's declared gates at the release commit as a `release.legs` leg inside the cut, and `bin/.devbin/preflight-linux.sh` and `bin/.devbin/cmd/shared/preflight.lib` are gone. -- satisfied: no (computed)
- AC-02.2 `bin/int prepush` keeps its frozen-remote refusal, and a push neither requires a kovacs summary nor clones and builds the workspace. -- satisfied: no (computed)

### WP-03 -- The tap and the macOS assets publish through the built-ins, with prepare as a shadow (status: Not Started)

- AC-03.1 The built-in formula writes Intent's formula and cask in assets mode under the cut's yes, with issue 0634's cask sha256 check and the brew smoke carried by the core, and `bin/.devbin/cmd/brew` and `bin/.devbin/cmd/shared/brew.lib` are gone. -- satisfied: no (computed)
- AC-03.2 The built-in `macos publish` publishes the stage Intent's `macos prepare` makes, that stage carries `<stage>/commit` with HEAD's full sha, and `bin/.devbin/cmd/macos` carries no publish. -- satisfied: no (computed)
- AC-03.3 (non-test) The service block in Intent's formula template points log_path and error_log_path at D19's intentd.log and intentd.err.log under the user's state dir, the files intent daemon logs reads, and its comment says intentd writes every line to stdout or stderr and names where each goes (issue 0667, ruled by hv on 2026-10-31). -- satisfied: no

### WP-04 -- The workflows fire on the candidate, and the docs describe the cut as built (status: Not Started)

- AC-04.1 `tests.yml`, `rust.yml` and `linux-release.yml` fire on `candidate/` refs and not on tags, the macOS legs included, and `bin/devbin release check` passes on Intent's tree. -- satisfied: no (computed)
- AC-04.2 (non-test) `intent/docs/releasing.md` and `.github/workflows/README.md` describe the cut as built: one command from the yes to the tap with no step between them run by hand, a dry run that predicts and runs no gate, then `brew upgrade` and `intent use brew` on each machine. -- satisfied: no
- AC-04.3 (non-test) hv cuts Intent's first release through the core with no hand step between the yes and the tap, and vc records that cut's RELEASE seal clock lines against the 3.2.8 baseline. -- satisfied: no

## Acceptance Tests

### WP-01 -- The cut runs through devbin's core, and Intent's driver retires (status: Not Started)

_(no tests in this group)_

### WP-02 -- kovacs runs as the cut's remote leg, and prepush keeps only its push gates (status: Not Started)

_(no tests in this group)_

### WP-03 -- The tap and the macOS assets publish through the built-ins, with prepare as a shadow (status: Not Started)

_(no tests in this group)_

### WP-04 -- The workflows fire on the candidate, and the docs describe the cut as built (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
