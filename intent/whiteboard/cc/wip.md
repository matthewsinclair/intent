---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-15 16:21Z
status: active
focus: "0338 (ii) LANDED at d4a3a0684. 0338 (i) BANKED at refs/bank/cc/0338-i on 6509c2ce5, green; re-take on 465d508cc after the bounce (regen/retake-0338i.sh), then land in the next train with vc's landing rulings. Then thread_spec's XS, 0377, 0331 whole, 0321 (c), 0375. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **0375, LAST, AFTER 0338 (ii), 0338 (i), 0377 AND 0331 COMMENTS (vc, ruled at the fold).** At S: a sixth WbItemKind `directive`, legal on hv only (`wb add directive --node hv`; any other node refuses naming hv and the protocol's `## Standing directives` section). Views render it under `## Standing directives` on hv's board only. `wb migrate hv` carries the section's lines as directive items; a non-hv board carrying the section refuses at migrate naming it. A fold never archives a directive. No DDL (kind has no CHECK). Register rows: the kind's value row and `wb add`'s roster row. in-whiteboard SKILL.md hv section names the kind and verb, through the skill's own canon path. Close note names the Laksa follow-up (laksa-vc re-carries hv's seven directives).
- **0331's stale-comments pass also takes four strings ic found (vc routed them, 2026-09-15; confirmed at a0c7300eb).** views.rs passes `thread.json` as the footer's source at :437 and :948, where canon is `intent/.canon/st/<ST>.json`; every thread view renders that footer, so the fix re-renders them all and the landing carries that mechanical diff. export.rs:47 says one `thread.json` per thread and one `issue.json`. contract.rs:71 cites the deleted `bin/intent_acceptance:454`. intent/st/ST0056/parity/tools/coverage_map.sh:23 says tests/ is gone, and it is not. `thread.json` also stands in views.rs doc comments at :1160 and :2311.
- **IN FLIGHT: RESUME HERE AFTER THE BOUNCE (cc, 2026-09-15).** 0338 (ii) landed at d4a3a0684. **0338 (i) is BANKED at refs/bank/cc/0338-i (blob bb30b4cb2d6c4665eb4e075f90320ca3b67182f4) on 6509c2ce5, and green there**: intentsvcs lib 261 and suite 1388/0, intent-cli targeted 58 and mcp lib 25, workspace clippy and rustfmt clean, and all 9 new arms red first on the stack tree. It carries FacadeError::CrossProjectAddress with require_local run first in all eight facade doors that take an address, resource_read and browser_url calling it, Step.content as bytes with OrganizeError::NothingToWrite and the byte-wise gate, and the corrected browsed and hydrate docs. **1. Re-take it on 465d508cc (vc: tui-fix and aswritten-proposal on main, pair built there, intentd restarted; the bank applied there in a dry run before the fold) and send vc the blob:** `bash /private/tmp/claude-501/-Users-matts-Devel-prj-Intent/e3744ab1-9442-4c6e-81f7-fcfee1d1af21/scratchpad/regen/retake-0338i.sh 465d508cc`. The ref moves only if green, and a dry run already showed tui-fix and (i) apply in sequence on 6509c2ce5. **2. Land it in the next train with dc's rscode, on vc's word, with the store writes vc ruled:** (a) `intent ac edit ST0057 AC-07.6 --text` "**Empty authority means THIS project.** A cross-project reference carries the slug, parses and round-trips, and every door that takes an address refuses it by name before resolving anything against this project, with a remedy that spells this project's own address; no door resolves another project's address. A reference that hard-codes the project name breaks on rename or fork, so the empty form is the one intra-project prose must use." (b) `intent at new ST0057 AT-07.8 --covers AC-07.6 --file native/rust/crates/intent-cli/tests/another_projects_address_is_refused_by_name.rs`, then `at green` once the file is in the tree. (c) AT-07.6's note keeps its 2026-08-19 record and gains "2026-09-15 (cc, 0338 (i)): the file now also drives every facade door with a slugged address; AT-07.8 covers the intent-cli doors.", through `intent at green ST0057 AT-07.6 --note` (vc confirmed: it extends the note, where `at edit --note` would replace it). (d) WP-07's body line "**Empty authority means THIS project.** Cross-project references carry the slug and resolve against intentd's registry." becomes the AC text's first two sentences, through `intent set intent:///threads/ST0057/wp/07 body --from <file>`. (e) 0338 closes with `organize --apply`. The ready files are in the scratchpad's regen/kit (ac-07.6.txt, at-07.6-note-line.txt, wp07-body-new.md, commit-0338i.txt). **3. AC-03.1 is covered, and computed stands (vc):** the estate arm prints `AT-03.1: 0 opaque attachment(s) of 544 carried across 75 thread(s)`, and the constructed fixtures fail without (i). **4. Then, each banked and landed on vc's word:** thread_spec's URL refusal as its own XS commit with no issue (a URL handed to a verb that takes an id must name the whole argument, say the verb takes a steel thread id and give an example: CLI st hydrate and dehydrate, MCP st hydrate, dehydrate and edit); 0377; 0331 whole (hv item 3, with todo 13's four stale strings, and ic reviews the register row); 0321 (c), the daemon log exempt from one_clock with its reason, XS; 0375 last (todo 6). The bank ref survives a reboot; the scratchpad does not.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, folded 2026-09-15; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. The home pointer is ~/.local/share/intent/home, and restart.md line 25 still names ~/.intent/home (sent to vc).
- **TWO MORE TRAPS (cc, 2026-09-15).** `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch.
- **THREE MORE TRAPS (cc, 2026-09-15).** An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon (ic's 0339 and as-written banks did): judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright; `at green/red/na --note` extends it.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
