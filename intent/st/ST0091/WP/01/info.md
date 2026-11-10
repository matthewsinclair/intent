---
wp_id: WP-01
title: The cut runs through devbin's core, and Intent's driver retires
scope: M
status: WIP
---

# WP-01: The cut runs through devbin's core, and Intent's driver retires

## Objective

`bin/devbin release` cuts Intent with devbin's core, and `bin/.devbin/cmd/build.d/release` is gone. What the driver does that the core does becomes the core's, and what the core does not do is declared in Intent's config: its own gates in `release.gates` and its derived steps in `derived:`.

## Builder and judge

dc builds this package and claims `ST0091/01` in the same chain as `intent wp start ST0091/01`, once the vendor sweep has brought the new core into Intent's tree. vc judges it and claims nothing.

## What moves

dc's inventory, measured at 7ff46b458; line references are to that commit. dc read it again on 2026-11-10 at 9ff4b3e9c, against the vendored devbin 0.1.20 core (`refs/bank/dc/st0091/wp01-inventory-v2.md`, which holds the first inventory whole and dc's re-read at file and line), and vc's rulings on that read are the section below.

- `bin/.devbin/cmd/build.d/release` (1524 lines) goes. Its pre-flight (:534) goes to the core's pre-flight and its version stamp (sidecar sync, :942) to the core's stamp. Its confirmation (:245), tag (:1418), push to both remotes (:1459, :1461) and GitHub release (:1468-1505) go to the core's one yes before anything runs, the tag on green, the push and the release object.
- `release.verbs` gains `cut`.
- `derived:` carries the schema-face bless (:1066), the lock refresh, the stamp CLI (:1188), `agents sync` (:1200), `claude upgrade --apply` (:1250) and `sync --to-disk` (:1289). Each is a project command that prints on stdout the paths it wrote and nothing else, because the core takes every stdout line of a derived command into the set of paths the release owns.
- `release.gates` declares Intent's own checks, which stay Intent's: the bats suite (:725); the cargo gate (:767-847), with issue 0481's one re-run of intentd's load-sensitive family (:827) moving into Intent's own gate command, because the core re-runs nothing and a red local leg stops the cut; the ST0056 contract and reference checks (:638, :659), which ST0084 later moves; the embedded-source-commit check `macos publish` makes today (`cmd/macos` :1405, against the string at `native/rust/crates/intentd/src/main.rs:75`), keyed on the release commit; and the pair build at the release commit (`pair_at_tag`, :290), held to macOS, which keeps issue 0518's currency verdict green on the release push. A sealed green for a commit and platform is not run again (Devbin D57).
- The refusal of a live intentd (:704) runs at the core's pre-flight, before the confirmation and before anything is stamped, and the dry run reaches it. The 0.1.20 core has no seam there for a project's own check, so vc asked devbin-vc for one on 2026-11-10, and the check's wiring waits for it.
- `tests/unit/release_script.bats` and `release_sidecars.bats` follow what they test.

## Rulings on dc's re-inventory

vc's, on 2026-11-10, under hv's pen, each checked in the vendored source and each open to hv's reversal. The inventory is `refs/bank/dc/st0091/wp01-inventory-v2.md`: section C holds the findings, and section E dc's re-read at file and line.

- **The live-intentd refusal (C1).** It belongs at the core's pre-flight, through a declared list of `bin/devbin <words>` lines that a dry run and a cut both run. The 0.1.20 core has no such list, and vc asked devbin-vc for it. dc builds the check now as one project verb that writes nothing, and wires it when the seam exists. Three ways that need no devbin change are refused. A project `release` handler shadows every release verb, `release gates` on a remote leg included. A refusal in the first derived step comes after the confirmation and after `version set`, and the dry run passes where the cut refuses. A notes provider's `state` is also read by `release status`, `release check` and the stamp's read-back, so it would refuse those while intentd runs. If devbin-vc declines, which cost to pay is hv's to say.
- **What the dry run names (C2).** The core's dry run names every leg and gives a count of derived commands, and `bin/devbin release check` names every derived command and gate. AC-01.1 is reworded to that.
- **The candidate's trigger (C4).** `tests.yml` fires on a push to main and on tags, so the core's pre-flight refuses a cut and a dry run until the workflows fire on `candidate/` refs. `ci.query: -` is not used to get past it. WP-04's trigger change is built first and banked as the train's first unit.
- **The pair build (C8).** It is a gate and not a derived step, because a derived step runs before the release commit exists and the pair must embed that commit. It is held to macOS by `platforms:` on an option of a gate that runs its command's `all`, which the core honours and whose pre-flight refuses an option active on no leg. That is vc's source read, and dc drives it.
- **Size (C14).** WP-01 is M.
- **Where a check lives.** A check that judges the tree as it stands before the stamp, and writes nothing, belongs at the pre-flight seam: the live intentd, and the schema faces carrying the current version. A check whose truth depends on the release commit is a gate: reference currency, the embedded source commit, the suites and the pair. Each check has one home, and dc's design shows the split.
- **Built this way until hv rules.** `release.remotes` is declared as `local` and `upstream`, so the third remote takes nothing (C3). The core's release commit message, `release: <tag>` alone, stands (C10).
- **Forced by the core (C9).** Its formula step refuses a lightweight tag, so Intent's release tags are annotated from 3.3.0. Intent's own readers of a tag peel it already, and dc's bank says which it checked.

## The judge

One heavy run on the bank's tip, booked through devbin-vc's desk: the whole bats tree and the whole cargo suites, because hook and scanner arms read shell and config files; `intent critic shell --files` on each changed shell file; `prettier --check` on each changed `.md`; `bin/int precommit` over the staged tree; and the core's dry run of Intent's cut, read for every derived step, gate and leg. dc mints this package's AT rows with its bank, and each arm carries its AT id from the start. dc may bank WP-01 to WP-04 as one train, judged in one run and landed in order.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0091/acceptance.md`, under the `WP-01` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
