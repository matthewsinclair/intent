---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-15 18:55Z
status: active
focus: "0377 (a) on refs/bank/cc/0377-wip: lifeline re-run green, cost run next, then bank for vc's train; then 0331 whole, 0321 (c), 0375. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **0375, LAST, AFTER 0338 (ii), 0338 (i), 0377 AND 0331 COMMENTS (vc, ruled at the fold).** At S: a sixth WbItemKind `directive`, legal on hv only (`wb add directive --node hv`; any other node refuses naming hv and the protocol's `## Standing directives` section). Views render it under `## Standing directives` on hv's board only. `wb migrate hv` carries the section's lines as directive items; a non-hv board carrying the section refuses at migrate naming it. A fold never archives a directive. No DDL (kind has no CHECK). Register rows: the kind's value row and `wb add`'s roster row. in-whiteboard SKILL.md hv section names the kind and verb, through the skill's own canon path. Close note names the Laksa follow-up (laksa-vc re-carries hv's seven directives).
- **0331's stale-comments pass also takes four strings ic found (vc routed them, 2026-09-15; confirmed at a0c7300eb).** views.rs passes `thread.json` as the footer's source at :437 and :948, where canon is `intent/.canon/st/<ST>.json`; every thread view renders that footer, so the fix re-renders them all and the landing carries that mechanical diff. export.rs:47 says one `thread.json` per thread and one `issue.json`. contract.rs:71 cites the deleted `bin/intent_acceptance:454`. intent/st/ST0056/parity/tools/coverage_map.sh:23 says tests/ is gone, and it is not. `thread.json` also stands in views.rs doc comments at :1160 and :2311.
- **IN FLIGHT: RESUME HERE (cc, 2026-09-15, evening).** **1. 0377, ruled (a) by vc: NoCache on the canon registration, S.** The change (intentd watch.rs only) is on refs/bank/cc/0377-wip on 23118bc0c. With it and no test touched, intent-cli whole is green and intentd reads 9/0, 5/0 and 36/1, and no event-shape test went red. The one red, a_daemon_outlives_nobody's owner-killed arm, starts a daemon that registers no project, so no watch exists in it; it ran at load 325 to 654 and re-ran green at load 83, with all six of that file's arms. The one cost run on vc's protocol (regen/0377-cost.sh: the worktree's own debug pair, a private intentd under an isolated HOME, a hookless shallow clone as the estate, one small board commit, ps -M per thread before and after, one sample for thread names) is read against the issue's ~8 s of system time on the canon fsevents thread; listed.rs follows as its own XS only if that run convicts it. Then bank on refs/bank/cc/0377 with the base named and send vc the hash; the judge is whole suites in vc's train, and the watcher family re-runs once. **2. 0331 whole** (hv item 3, with todo 13's strings and the issue's stale comments; ic reviews the register row): module_check_hook.json and its exit_code_consumers row; critic-guard.sh with its exit_code_consumers row and guard_dispatch.bats's declared-absent case; the rules-index trio under intent/plugins/claude/rules with the unwired `claude rules index` register row; the dead llm and prj templates with their init.rs DESTINATIONS rows and every test list that names them; global-agents.json with the critic_prose, rule_pack_shell and highlander_audit bats assertions that read it; and the MODULES.md rows for all of them. **3. 0321 (c)**, the daemon log exempt from one_clock with its reason, XS. **4. 0375 last (todo 6).** Every lander runs `intent at lint <ST>` and `intent ac gate <ST>` after its store writes and before its commit (vc).

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, folded 2026-09-15; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. The home pointer is ~/.local/share/intent/home, and restart.md line 25 still names ~/.intent/home (sent to vc).
- **TWO MORE TRAPS (cc, 2026-09-15).** `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch.
- **THREE MORE TRAPS (cc, 2026-09-15).** An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon (ic's 0339 and as-written banks did): judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright, and `at green/red/na --note` never appends either: it refuses with NoteWouldBeLost unless the passed text CONTAINS the existing note, then sets the note to that text. To add a line, pass the old note plus the line.
- **TRAPS FROM THE EVENING (cc, 2026-09-15).** A new AT row's cited file must carry the row's literal id, and `at new` and `at green` never read a citation (issue 0267), so only `at lint` and the gate see a missing one: 0338 (i)'s landing took ST0057's gate from PASS to BLOCKED on exactly that until a header line named the row. A new row starts `to-write`, and `at green` is legal only from `red` (issue 0337); `at green --note` on a row already green is accepted and its text must contain the old note. A daemon cost run needs an estate the size of the real one, so neither a fresh `init` (too small to show a walk) nor a worktree commit (the gate refuses it under an isolated HOME): a hookless clone of the repo is the scratch estate.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
