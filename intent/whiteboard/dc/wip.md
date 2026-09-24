---
node: dc
name: DevX Claude
role: worker
session_id: 4e187b62-bbfa-4424-8bf9-a7e03ae421e0
heartbeat_at: 2026-09-24 19:19Z
status: active
focus: "LOCALFOLDED for hv's compact. The fix set waits on hv's ruling in words; if hv rules it in, dc builds 0564 on vc's accepted plan. The known-defects re-drive follows the build all. Resume is doing 116. NO PUSH, NO RELEASE."
claims: []
---

# DevX Claude (dc)

## DOING

- **RESUME (dc, second localfold for hv's /compact, 2026-09-24): THE 3.2.1 DOC AUDIT, dc's LANE, AND 0564 IF hv RULES THE FIX SET IN. NO PUSH, NO RELEASE.** (edited)
  - LANDED in main:
    - d1e96ed74: DEPRECATIONS and the record banners.
    - 638aa1d6a: README, install and the TCA page.
    - cc34b16e4: releasing, rust-on-macos, skills-triage, tn001 and tests/README.
    - 8271466f0: pre-commit-hook, banking, both migration pages and the workflows README.
    - 3eba23354: install.md's jq requirement.
    - design-system.md is inside ic's register landing 90684455f, at its judged patch-id 9e615fb74.
  - BANKED, NOT LANDED:
    - refs/bank/dc/audit/ci-9-10 = blob ffa1e816b515f251aa99675ae044a09b2612fdbf, patch-id 3fa48ac5abc3b031fab7ae22b13389a0442ded29, 2 files, +25/-10. It applies at 12fd44738, and vc measured it stacking with ic's five fix banks in both orders. It rides ic's stacked final-tree run, and its security issue is filed when it lands, not before. The red/green repro is beside it: ci-9-10-repro.sh 8ab0be3b5, ci-9-10-repro.log ea3dc1283.
    - refs/bank/dc/audit/tap-readme = blob 81eb855bd: the tap commit 5f71f19 in tmp/tap-intent, one ahead of origin 5892b41. No push; hv pushes it before `publish`.
    - refs/bank/dc/kd/drive-v2.sh = blob 331d7bdb7, byte-identical to the scratchpad's kd/drive.sh: the known-defects drive plus e_init_pointer (0558), e_preserved_seed (0565) and e_mcp_stale (0549).
    - refs/bank/dc/audit/ledger.md (778811be8) and refs/bank/dc/audit/lane-report-draft.md (1bec79574).
  - WAITS ON hv: a ruling on the fix set, in words. hv has not given one, and vc has asked. vc recommends IN. If hv rules OUT, 0564 stays stated where it is and both of its paragraphs stay.
  - NEXT IF hv RULES IN: 0564, on the plan vc accepted as written (vc, 2026-09-24):
    - RED FIRST: one arm, intentsvcs/tests/canon_seeds_the_critic_config_as_the_template_holds_it.rs, registered in tests/suite.rs beside canon_seeds_usage_rules_with_the_project_name. It takes a fresh Fixture and runs canon::apply with defaults, and the seeded .intent_critic.yml must equal lib/templates/_intent_critic.yml byte for byte. It is red on HEAD.
    - THE FIX: rootfiles gains `expand` (blocks and tokens, still the one expander), `substitute` becomes `expand` plus fill_empty_sections, and seed_if_absent (canon.rs:568) adds the filler only where `rel` ends in `.md`.
    - THE SAME BANK: the 0564 paragraphs at pre-commit-hook.md:66 and working-with-llms.md:394 both come out whole. The second page is vc's lane, and vc granted the exception by name. Neither page has a live line citer.
    - RUN BY NAME ON THE BANK:
      - the new arm, red on HEAD and green on the fix;
      - canon_seeds_usage_rules_with_the_project_name;
      - root_files_generated, remedy_coverage and canon_seeds_the_mcp_declaration_once;
      - intent-cli's agents_sync_parity;
      - canon_holds_a_settings_file_it_did_not_write, doctor_reports_a_root_file_behind_its_template and carrier_is_installed_beside_the_block.
      The whole suites and the shell half are paid once, in ic's stacked final-tree run.
    - THE DRIVE: a fresh init under a short HOME, then `claude upgrade --apply --skip-settings` with the worktree's build. Ruby's YAML (/usr/bin/ruby 2.6.10) must load the seed as {"severity_min"=>"warning", "disabled"=>[], "post_tool_use_advisory"=>false}, as it loads the template. The same drive on the installed pair is the red.
    - TO vc: the blob, patch-id and logs, and the Fixed line: "`intent claude upgrade --apply` seeds `.intent_critic.yml` exactly as the template holds it; it had written a Markdown filler line after the template's comment lines, so a strict YAML reader rejected the file (0564)." vc writes the whole CHANGELOG.
    - The worktree build is heavy: census the machine, announce START and END, and never overlap ic's stacked run.
  - NEXT AFTER THE BUILD ALL:
    - Re-drive known-defects on the final pair, once `intent --version` names the final HEAD. Run drive-v2.sh with its S path set to the session's scratchpad, over every entry plus each ruled-out issue a reader meets, and replace each transcript whole. On vc's scope:
      - STATED already: 0555, 0556, 0557, 0559, 0561, 0562, 0564, 0568, 0569, 0571 and 0572.
      - 0558 is an ENTRY while it is open, and 0565 and 0549 are driven and reachable.
      - ic takes any explorer entries.
      - 0566, 0573 and 0574 are not reachable by a reader.
    - Reword the preamble's "every issue this page cites is closed" for entries that cite open issues. The pin names the final pair's commit; it is provenance, on vc's ruling.
    - If the CI bank lands, give the .github/workflows/README.md lines for the steel-thread check, check-documentation and test-coverage their as-built text.
    - Each commit goes by literal path under START and END, after the stacked final-tree run. The release driver's pre-flight covers the final HEAD.
    - Then send vc the lane report, from the draft.
  - AFTER THE LINE: the int/devbin help edits (one bank, one macOS bats run) and the yml comment fixes. The vendored help findings go to Devbin through vc. Hold 29 stands for the fleet sweep.

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
