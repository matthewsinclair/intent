---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-15 20:44Z
status: active
focus: "0331 (a) not yet green in wt-0331 (four named fixes); the footer bank unjudged in wt-footer; (b) counts sweep in wt-0331b; then 0321 (c), 0375. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **0375, LAST, AFTER 0338 (ii), 0338 (i), 0377 AND 0331 COMMENTS (vc, ruled at the fold).** At S: a sixth WbItemKind `directive`, legal on hv only (`wb add directive --node hv`; any other node refuses naming hv and the protocol's `## Standing directives` section). Views render it under `## Standing directives` on hv's board only. `wb migrate hv` carries the section's lines as directive items; a non-hv board carrying the section refuses at migrate naming it. A fold never archives a directive. No DDL (kind has no CHECK). Register rows: the kind's value row and `wb add`'s roster row. in-whiteboard SKILL.md hv section names the kind and verb, through the skill's own canon path. Close note names the Laksa follow-up (laksa-vc re-carries hv's seven directives).
- **0331's stale-comments pass also takes four strings ic found (vc routed them, 2026-09-15; confirmed at a0c7300eb).** views.rs passes `thread.json` as the footer's source at :437 and :948, where canon is `intent/.canon/st/<ST>.json`; every thread view renders that footer, so the fix re-renders them all and the landing carries that mechanical diff. export.rs:47 says one `thread.json` per thread and one `issue.json`. contract.rs:71 cites the deleted `bin/intent_acceptance:454`. intent/st/ST0056/parity/tools/coverage_map.sh:23 says tests/ is gone, and it is not. `thread.json` also stands in views.rs doc comments at :1160 and :2311.
- **IN FLIGHT: RESUME HERE (cc, 2026-09-15, compact fold).** 0377 landed at 33990b80b and is closed. **1. wt-0331 (base 55c933916): 0331 (a) is built and NOT YET GREEN; nothing banked.** It carries the dead artefacts with their rows, bats assertions and MODULES.md rows; the `claude rules index` register row settled to as-observed (ic confirms in the train's review) with the legal_pairs census and a regenerated dispatch-table.md; the rules docs; the routed stale comments; todo 13's doc-comment strings; and spine.rs's exclusions paragraph replaced by one scope sentence (vc's ruling: the exit-code roster's scan reaches only the install's `lib/templates`, so `bin/.devbin` is outside it by scope; tell vc that reach when (a) is banked). The judgement (regen/logs/judge-0331a.out) passed intent-cli whole, workspace clippy, the IN-RS-CODE-001 step, rustfmt and four touched bats files, and named four fixes: (i) `init::tests::the_embed_is_not_empty` asserts at least ten embedded templates and eight remain, so tie that floor to the DESTINATIONS table instead of a number; (ii) the coverage_map.sh comment edit changed an ST0056 attachment, so carry it into ST0056's canon through the worktree's own build (an unscoped `sync --to-store`, then `st attach`) and bank the canon with it; (iii) `prettier --write intent/llm/MODULES.md`; (iv) the bats harness defaults INTENT_BIN to target/release/intent, which a worktree lacks (every one of those reds was exit 127), so rerun the bats with INTENT_BIN pointing at the worktree's debug build. Then re-judge with regen/judge-0331a.sh (drop its first step, already done), bank with regen/bank-0331a.sh (refs/bank/cc/0331-a) and send vc the hash. **2. wt-footer (base 78ba5bbf8): the footer bank, NOT JUDGED.** views.rs `info` and `acceptance` pass "the thread canon" (vc's ruling (i)). Run regen/judge-footer.sh (it creates refs/bank/cc/0331-footer only if green); it lands right before a rebuild, then `organize --apply` from the new pair and vc's views commit. **3. wt-0331b (base 33990b80b): 0331 (b), the counts sweep, with a background subagent** writing regen/0331b-counts.patch and regen/0331b-decisions.md. If its notification is lost, the patch is `git -C <scratchpad>/wt-0331b diff --binary`. Review every KEEP first, then bank on refs/bank/cc/0331-b. 0331 closes when (a) and (b) have landed. **4. 0321 (c)**, the daemon log exempt from one_clock with its reason, XS. **5. 0375 last (todo 6).** Every lander runs `intent at lint <ST>` and `intent ac gate <ST>` after its store writes and before its commit (vc).

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
