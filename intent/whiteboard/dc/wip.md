---
node: dc
name: DevX Claude
role: worker
session_id: 4e187b62-bbfa-4424-8bf9-a7e03ae421e0
heartbeat_at: 2026-09-24 08:27Z
status: active
focus: "LOCALFOLDED at EOD 2026-09-24. dc's 3.2.1 work is done; 3.2.1 is ready to push at a3ed12629, then CI, the cut's hold and the cut. Next: hold 29 (the guards pass rides the fleet sweep after the tag). The resume todo carries the state. NO PUSH, NO RELEASE."
claims: []
---

# DevX Claude (dc)

## DOING

- **RESUME (dc, localfold for hv's /compact, 2026-09-24): THE 3.2.1 DOC AUDIT, dc's LANE. NO PUSH, NO RELEASE.** (edited)
  - LANDED in main: d1e96ed74 (DEPRECATIONS, record banners), 638aa1d6a (README, install, TCA page), cc34b16e4 (releasing, rust-on-macos, skills-triage, tn001, tests/README), 8271466f0 (pre-commit-hook, banking, both migration pages, workflows README), 3eba23354 (install.md's jq requirement). design-system.md is inside ic's register landing 90684455f, read back at its judged patch-id 9e615fb74. Every judged bank is landed, and the tree was clean at 93f330518.
  - BANKED, NOT LANDED:
    - refs/bank/dc/audit/ci-9-10 = blob ffa1e816b515f251aa99675ae044a09b2612fdbf, patch-id 3fa48ac5abc3b031fab7ae22b13389a0442ded29. pr-checks.yml takes the PR body through env:; check-documentation and test-coverage check out with fetch-depth 0 and take CHANGED first; rust.yml's prettier step gets pipefail. The red/green reproduction is beside it (ci-9-10-repro.sh 8ab0be3b5, ci-9-10-repro.log ea3dc1283). It lands only on hv's word.
    - refs/bank/dc/audit/tap-readme = blob 81eb855bd: the tap README commit 5f71f19 in tmp/tap-intent, on origin 5892b41. No push; hv pushes it before `publish`.
    - refs/bank/dc/kd/drive-v2.sh = blob 331d7bdb7: the known-defects drive, plus e_init_pointer (0558), e_preserved_seed (0565) and e_mcp_stale (0549).
    - refs/bank/dc/audit/ledger.md (778811be8) and refs/bank/dc/audit/lane-report-draft.md (1bec79574).
  - WAITS ON hv's RULING on the fix set:
    - The CI bank.
    - 0564: if it is ruled in and cc's fix lands, remove pre-commit-hook.md's paragraph "The seed as written is not valid YAML to a strict reader" in dc's next batch.
    - Each issue ruled OUT that a reader meets on a documented route becomes a known-defects entry. On vc's scope, these are already STATED: 0555, 0556, 0557, 0559, 0561, 0562, 0564, 0568, 0569, 0571, 0572. 0558 is an ENTRY while it is open; 0565 and 0549 are driven and reachable; ic takes any explorer entries; 0566, 0573 and 0574 are not reachable by a reader.
  - NEXT AFTER THE BUILD ALL:
    - Re-drive known-defects on the final pair, once `intent --version` names the final HEAD. Run drive-v2.sh with its S path set to the session's scratchpad, over every entry plus the ruled-out new ones, and replace each transcript whole.
    - Reword the preamble's "every issue this page cites is closed" for entries that cite open issues. The pin names the final pair's commit: it is provenance, on vc's ruling.
    - Then the 0564 paragraph, if ruled in, and the CI README lines for the steel-thread check, check-documentation and test-coverage, if the CI bank lands.
    - Each commit goes by literal path under START and END. They go after cc's final-tree run, as vc ruled, and the release driver's pre-flight covers the final HEAD.
    - Then the lane report to vc, from the draft.
  - AFTER THE LINE: the int/devbin help edits (one bank, one macOS bats run) and the yml comment fixes; the vendored help findings go to Devbin through vc. Hold 29 stands for the fleet sweep.

## TODO

_(none)_

## Holds

- **THE GUARDS ADOPTION PASS FOR THE REMAINING ESTATES IS HELD UNTIL THE 3.2.1 CUT'S FLEET SWEEP (hv's ruling via vc, 2026-09-23).** The recipe runs intent claude upgrade --apply, so each estate's hooks wiring rides that sweep, one commit per estate for both. The pilot, MicroGPTEx 80d4c13, passed vc's check. (edited)
  THE RECIPE: git config core.hooksPath .githooks; then intent claude upgrade --apply, which writes the four chain blocks (tracked) and the four .intent carriers; add .githooks/*.intent to .gitignore; declare formatters wherever the old hook checked any; red-control with staged probes; commit by path; run doctor.
  THE ESTATES: Molt and Prolix take the recipe unchanged, because their hook is byte-identical to MicroGPTEx's. Anvil, Baize, Cdtempl, Courses and Riffle share a chain-block-only hook and need no formatter declaration. Prodinfra, Conflab and Lamplight each get their own diff first. vc sequences Devbin, Laksa and Gtools.
  KNOWN LIMIT: core.hooksPath is per clone, so a fresh clone runs no hook until it is set, and only doctor --verbose's uncounted advisory says so.
  RIDER (ic's note (c), 2026-09-23, restated here on 2026-09-24 from archived doing 110 so it is not buried): prettier 3.9.8 rewrites the generated board and inbox views in both shapes, so wherever the recipe declares a markdown formatter in an estate that has a whiteboard, the same commit adds the prettier-ignore lines for `intent/whiteboard/*/wip.md` and `intent/whiteboard/*/inbox.*.md`, as Intent's own .prettierignore does.
  SCOPE: the seven estates this hold does not name (Molt-matts, Molt-flynn, arca_cli, arca_config, arca_notionex, Courses/002, ficton-content) are ruled by vc on a measurement of which carry Intent (intent/wip.md, AFTER THE CUT).

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
