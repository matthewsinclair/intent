---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-15 13:52Z
status: active
focus: "0338 (ii) BANKED at refs/bank/cc/0338-ii on 018c0adf6. READY is owed to vc after hv's bounce, then the matched control in vc's hold (regen/ctl-run.sh, both arms prebuilt). Then (i) with AC-07.6's arm, 0377, 0331 delete-all, 0321 (c), 0375. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **0375, LAST, AFTER 0338 (ii), 0338 (i), 0377 AND 0331 COMMENTS (vc, ruled at the fold).** At S: a sixth WbItemKind `directive`, legal on hv only (`wb add directive --node hv`; any other node refuses naming hv and the protocol's `## Standing directives` section). Views render it under `## Standing directives` on hv's board only. `wb migrate hv` carries the section's lines as directive items; a non-hv board carrying the section refuses at migrate naming it. A fold never archives a directive. No DDL (kind has no CHECK). Register rows: the kind's value row and `wb add`'s roster row. in-whiteboard SKILL.md hv section names the kind and verb, through the skill's own canon path. Close note names the Laksa follow-up (laksa-vc re-carries hv's seven directives).
- **0338 (ii) IS BANKED, NOT LANDED (2026-09-15, vc's trains rule): refs/bank/cc/0338-ii, patch blob e1d6caee5977de105dce73527ec5abdadae45994, base 018c0adf6.** It holds 1233 lines over 22 files: intentfiles.rs, facade.rs, 18 intentsvcs test files, ST0057 design.md and its canon. Before banking it went red first on the base, then green on the workspace check, the targeted tests (the touched suite modules, the intentfiles and facade unit tests, and intent-cli's no_pm_state_in_output), intentsvcs clippy and rustfmt. Recover it with `git cat-file -p refs/bank/cc/0338-ii | git apply` on the base. **NEXT: send vc READY, then run the matched control inside the hold vc announces:** `bash /private/tmp/claude-501/-Users-matts-Devel-prj-Intent/e3744ab1-9442-4c6e-81f7-fcfee1d1af21/scratchpad/regen/ctl-run.sh` runs the HEAD arm (wt-0338ctl) and then the (ii) arm (wt-0338ii), both prebuilt, each under a fresh HOME. Send vc both readings; vc lifts the hold, stacks the bank and runs the full suites, and the landing is on vc's word. 0338 stays OPEN; the dated (ii) note and the commit text are in regen/kit. If the scratchpad is gone, rebuild both arms from the bank at the base.
- **AFTER (ii), IN vc's ORDER, EACH ONE BANKED AS A PATCH BLOB AND LANDED ON vc's WORD (vc, 2026-09-15, hv's rulings applied).** (i): organize and st hydrate carry bytes, and an unloadable blob refuses naming its path. AC-07.6's reword lands in the same commit, with one new arm in tests/address_empty_authority.rs that sends a slugged address to each door 0338 names and asserts the refusal names it. **The as-built differs from vc's reword (static reads at 018c0adf6, sent to vc):** post, put and set name the URL; hydrate and dehydrate name the slug and the form but not the URL; mcp::resource_read never looks at the authority, so a slugged read answers from THIS project. detach_attachment does not check either, and its only caller today builds a local address. **vc ruled (2026-09-15): (i) makes mcp::resource_read refuse a slugged URI by name before it dispatches on the entity, and makes Facade::detach_attachment refuse one too, although its only caller passes authority None today, so no future caller can act on THIS project with another project's address. Drive the as-built after the bounce; the new test arm then covers every door that takes an address, and the AC-07.6 reword in the same commit says what the doors do after (i) (hv's ruling stands: refusal is the contract).** Then 0377. Then 0331, deleted in full (hv item 3): module_check_hook.json and its exit_code_consumers row, critic-guard.sh, the rules-index trio and the unwired `claude rules index` with its register row in the same commit (ic reviews the row), the dead llm and prj templates and their init.rs DESTINATIONS rows, global-agents.json and its three bats checks, and the stale comments; 0331 closes. Then 0321, ruled (c): the daemon log is exempt from one_clock, with the reason written down (an operational stream that must write when the store cannot), XS. Then 0375, last: dc dropped ST0069/14, so there is no claim conflict, and `wb archive directive` needs no refusal code.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, folded 2026-09-15; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. The home pointer is ~/.local/share/intent/home, and restart.md line 25 still names ~/.intent/home (sent to vc).
- **TWO MORE TRAPS (cc, 2026-09-15).** `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
