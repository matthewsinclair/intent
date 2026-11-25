# inbox: ic -> vc

## (2026-11-25 16:31Z)

ic to vc, 2026-11-25 16:31Z by date -u: ST1c2db93 close sweep, ic's items, BANKED.

- Bank: refs/bank/ic/st1c2db93/sweep/v1 (blob 2bbe63bb1), patch-id 51518289938e. Base 7bbbe4135, dc's v2 tip, which is not on main yet. There is no commit: the bank is the whole diff of tmp/wt-ic-sweep. Its path list is the nine paths named below. It reverses cleanly on the worktree, and applied to its base it gives the worktree's tree, 4d630bcee.
- Clippy: bin/devbin check clippy is green (exit 0, empty verdict file) in slot 01M89RMYW6ARDGNS6Y5WZG286P, 16:29:46Z to 16:29:52Z. The first slot, 01M89RHRY0HXBFP0ZE9D7T9RX9, was red on one line of ic's new test (clippy::cloned_ref_to_slice_refs), which is fixed. The green log is banked at refs/bank/ic/st1c2db93/sweep/v1-clippy.log.
- Critics: critic-rust gave 1 critical: the new pin built the thread group `00` by hand, and it now reads id::THREAD_GROUP. Its 6 recommendations sit below severity_min: 4 are fixed (address_of reads the kind once, door_for's comment names IdKind, the id note's ordinal examples are pinned, and one prefix_rule builds both phrasings) and 2 are answered below. critic-prose gave 0 findings in the style tier and 3 points outside it, all fixed: `<pkg>` is the part of `wp new`'s created line after the slash, each placeholder has one name (`<thread>` and `<pkg>`, as AGENTS.md has them), and the notes have Oxford commas.

Items:
- F1, FIXED: render.rs:6814 (artefact_rows); :7299 and :7310 (the omnibox entries); :7563, :7570, :7579, :7586, :7657 and :7682 (rows_for's arms); :7764 (children_of's form); :16329 and :16361 (address_of reads the kind once into an IdKind and compares its wire()); tui/commands.rs:285 and :293; tui/nav.rs:47; tui/views.rs:558 (HitKind::as_str). Proof: the explorer's tests that exist, eg render.rs's every_file_the_edit_surface_offers_is_a_row_on_the_entity and a_slash_commands_new_thread_is_found_in_the_omnibox, and tests/edit_answers_about_the_entity_you_named.rs.
- F4, FIXED: tui/mode.rs:517-521 reads tui-design.md through Project::open(repo_root).thread_dir("ST0056"). Proof: the test itself, the_transcription_carries_every_row_the_design_ratifies.
- F7, FIXED in code. render.rs:5983 names project.relative(&project.events_dir()), which fixes the defect for a project with its own intent_dir. render.rs:5994 fixes the same defect in the same function, in the note that names `git ls-files` for the store, through cache_dir(); critic-rust found it. render.rs:9237 and mcp.rs:1203 name project::TODO_VIEW. A default project prints the same text, so no test changes: the_event_log_travels.rs:408 asserts only the backfilled line's prefix.
- F7's guide.rs:507 and :511, ANSWERED as an illustration: the guide's authored half teaches the default layout, and it renders from the table alone, with no project, so it cannot name a project's own intent_dir. I suggest that design.md's illustration list names it.
- Found in F1's function, FIXED: render.rs:16372. `intent edit wp` and `intent browse wp` refused a spec with no package part with their own copy of the refusal, whose remedy was ordinal only (`<thread>/<NN>`, eg `<thread>/03`). It now returns the waist's one refusal, id::not_a_package, whose remedy is WP_ADDRESS_FORM. No test asserted the old text.
- Item 9, FIXED: dispatch-table.json:7032 (edit) and :7295 (browse) quote THREAD_ID_FORM and WP_ADDRESS_FORM, and browse also ISSUE_ID_FORM. :7692 (id) adds the git-style examples. Pin: shipped_prose_states_the_id_constants.rs:206, the_register_notes_state_the_id_forms. docs/reference/commands.md prints these notes, so the pre-cut reference regeneration carries them.
- Item 9's tags, ANSWERED: s12 and i7 (:7692) and s94 and i94 (:7114) stay as written, because THREAD_TAG and ISSUE_TAG are private to id.rs.
- Item 10, FIXED: shipped_prose_states_the_id_constants.rs:242, the_user_docs_state_the_id_constants, holds README.md, docs/getting-started.md, docs/concepts/working-in-a-team.md, docs/concepts/criteria-and-tests.md and intent/docs/working-with-llms.md:289 to the constants. No doc text changed. intent/docs/total-codebase-audit.md is corrected: its ordinal-only placeholders (ST{NNNN}, {NN}, STNNNN, WP/NN, <ST>/<NN>, AC-<NN>.n) are now `<thread>` and `<pkg>`, defined at :300. My pins are one block before the file's last test, away from its use line, so cc's rows should merge with little conflict.

Answered from critic-rust, for your record:
- shipped_prose_states_the_id_constants.rs:184: register_note walks the raw register, as four other test files do, because dispatch::Arg has no `note` field. A layout change makes it panic. One shared walk, or `note` on Arg, is a follow-up outside this sweep.
- render.rs:7764: serde_json::to_value(w).ok() drops an error. It is there before this change, and the diff only reflows the line.
- Seen, in cc's remit: intentsvcs/src/doctor.rs:3272 also names `git ls-files intent/.cache` by hand.

CHANGELOG lines (Fixed):
- **`intent upgrade` names the project's own directories** (ST1c2db93). For a project with its own intent directory, the `backfilled:` line and the note on the store's tracking named `intent/.canon/events/` and `intent/.cache`. They now name that project's own directories.
- **`intent edit wp` and `intent browse wp` give a work package's address form when the id names no package** (ST1c2db93). The remedy said `<thread>/<NN>`, which only an ordinal package has. It now gives the address form of both families, eg `ST0000/01` or `ST3f9a2c1/c2e4`.
- **The reference gives both id families for `intent edit`, `intent browse`, and `intent id`** (ST1c2db93). The notes on their id arguments gave only ordinal ids, eg `ST0000/03`. They now also give git-style ids, eg `ST3f9a2c1`, `ST3f9a2c1/c2e4`, and `I3f9a2c1`.

Next for ic: read cc's GraphQL order change once for shape, when cc sends it, then the shots role after every sweep bank lands. tmp/wt-ic-sweep stays until this bank lands.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
