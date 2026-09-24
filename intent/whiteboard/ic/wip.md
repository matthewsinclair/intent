---
node: ic
name: Interface Claude
role: interface
session_id: ba4c99b8-b828-4603-9871-50eb86fc3a5b
heartbeat_at: 2026-09-24 18:43Z
status: active
focus: "Holding for hv's fix-set ruling. Banked and accepted by vc today: the explorer docs (member 2, 22548f54) and the skills usage-rules fix (2b, 3bfeb338). Next on hv's go: 0570 (drive, then its body, then a red-first build), then the stacked final-tree run. NO PUSH, NO RELEASE."
claims: []
---

# Interface Claude (ic)

## DOING

- RESUME STATE (ic, 2026-09-24, localfold for hv's /compact). Tree clean at 93f330518; every judged bank landed at its patch-id. NO PUSH, NO RELEASE.
  LANDED by ic today: 860d21ab1 docs/explorer.md, 11 shots, gen_explorer_shots.sh and explorer_shots.py (ST0056's canon with them); 93977ae24 seven canon skills; 1dd9e006f in-tca-init (my own rules-path regression from 93977ae24, caught by bats 621); 72fc48538 in-essentials:59; 93f330518 in-standards:18 (DECISION_TREE, matching _CLAUDE.md); 90684455f THE REGISTER STACK at judged patch-id f7002f91 (76 register fields, the regenerated dispatch-table.md, agent-guide.spec.md, four guide.rs strings, the rules-validate note in render.rs, the wb_register_correct pin, and dc's design-system v2 at 9e615fb7), judged green on 1dd9e006f: cargo 3077/0, bats 747/747. The register is NOT DEPLOYED until the build all.
  BANKED, waiting on hv's ruling; they ride cc's final-tree run if ruled in, and all five stack at 93f330518: refs/bank/ic/fix/0550 (blob d30af124, patch-id bfcf35a1: layout.rs heading rows); fix/0552 (f1da4ec9 / 4ee9e057: run.rs first_read keeps explore's landing reason); fix/0553 (41806303 / c86c5d80: nav.rs view_for takes the bare wp sequence, with a contract test); fix/0560 (d10e8ddf / 4199aa2f: app status uses macapp's own remedy through app_message); fix/0567 (6f5b48d4 / 2168a575: the todo reopen remedy names --reason).
  NEXT AFTER THE BUILD ALL, in order:
  (a) Reference regeneration. Rewrite gen_reference.sh:266's stale stamp, dropping "One of the four such strings in the surface is measured false. See issue 0142". Regenerate both halves at HEAD against `git describe --tags --abbrev=0` (v3.2.0), as intent/docs/releasing.md says. Confirm reference_current_check.sh exits 0. Commit docs/reference, gen_reference.sh and intent/.canon/st/ST0056.json together, because the script is an ST0056 attachment and intentd takes the edit into canon.
  (b) Re-take the shots: gen_explorer_shots.sh --python <this session's scratchpad>/venv/bin/python. It uses a fixed /tmp/intent-demo HOME, which must not exist beforehand. `git status -- docs/images/explorer` shows any change.
  (c) Update docs/explorer.md for each fix ruled in: 0552, "and says why"; 0550, the /help shot; 0553, a work-package address example.
  (d) Send vc the coverage report, every lane file with its disposition. Skills: 32 files, the edited ones in the commits above, the rest clean. trope-catalog is vendored: FMT-03, TN-01, TN-07 and TN-02 have flattened quotes, which go to llm-tropes after the line. Subagents: 18 clean; diogenes needs no metadata.json (payload.rs:183-221). surface: edited in 90684455f; forms.json clean. lib/help: struck. docs/reference: regenerated. New: the explorer page, its shots and the generator.
  FILED through vc: 0550, 0552, 0553, 0560 and 0567. vc files after the line: B16/B17/D4 (read_or_mutate), the arg-note leak guard, and FMT-03 upstream.
  TOOLS: the judge script is <scratchpad>/judge.sh, with HOME isolated and every XDG_* unset. Exporting XDG_* reds the intentd arms. tmp/wt-ic-reg is removed at this fold.
- STATE (ic, 2026-09-24 18:42Z by date -u): holding for hv's ruling on the fix set. NO PUSH, NO RELEASE.
  COMPOSITION (vc's, in landing order): 1 my fix banks 0550, 0552, 0553, 0560, 0567; 2 refs/bank/ic/audit/explorer-docs (blob b53d4238e, patch-id 22548f54, lands only beside 0552 and 0553); 2b refs/bank/ic/audit/skills-usage-rules (blob eae70e2c6, patch-id 3bfeb338: three skills named deps/phoenix_live_view/usage-rules.md, which no estate ships; now deps/phoenix/usage-rules/liveview.md); 3 dc's audit/ci-9-10; 4 dc's 0564; 5 my 0570; 6 cc's 0551; 7 cc's batch 1; 8 cc's team-page docs, with cc's 8b; 9 a vc docs bank only if batch 1 needs one. vc has judged and accepted 2 and 2b. Worktrees kept until landing: tmp/wt-ic-docs (= members 1 and 2, tree d6f0016bf) and tmp/wt-ic-skills (= 2b on HEAD).
  NEXT ON hv's GO: 0570. First the drive (scratchpad drafts/drive-0570.sh, under /tmp/ic70, each gate state with the pointer good and bad, each named remedy driven). Then correct 0570's body where the drive contradicts it: the stale comments are finding.rs:637-648 (not canon.rs), doctor.rs:2085-2095 and canon.rs:831-836; C2's remedy is to reinstall. Then build it red-first on HEAD in its own worktree: the pure text function, the carried errors, the pointer read through gate_resolution, nothing past that. apply --check it on top of members 1-4 before banking.
  THEN the stacked final-tree run (scratchpad drafts/compose-final.sh and judge-final.sh: the CI lines as spelled, XDG set empty, no intent on PATH but the worktree's, a tool table, census and START/END). After the build all, doing 116's (a) to (d).
  COVERAGE MISS for the report: my skills pass marked in-standards, in-elixir-essentials and in-phoenix-liveview clean; it measured paths in this repository only and never in a consumer's deps/.

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
