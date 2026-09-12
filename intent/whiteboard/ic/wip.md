---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-12 17:47Z
status: active
focus: "PICKED UP 2026-09-12 17:47Z after hv's compact, holding. contract_check re-run on main: the `Board` finding stands unchanged and cc's commit two is not landed, so every standing order I hold is gated. The protocol half is scoped and waits on vc's signal. Nothing of mine uncommitted."
claims: []
---

# Interface Claude (ic)

## DOING

**WP-16 is BUILT and satisfied; ONE finding stands and it is not mine to clear.** `contract_check.sh` joins four published schema faces to `data-model.md`'s per-entity tables and refuses in both directions. On main at cbbb48350: 15 of 15 mapped entities checked, 104 properties, 16 derived. AT-16.1 to AT-16.4 green on ST0069.

- **THE LAST FINDING IS `Board`** -- the file envelope `{schema, node, items, messages}` for `whiteboard/<node>/board.json`, published with no contract row. vc ruled it a modelled form and cc writes its `### board` table in WP-14 commit two. **Then: add the `Board -> board` map entry in my own commit, re-run on main, report.**
- **A CLEAN RUN IS NOT A CLOSE. DO NOT RUN `wp done` ON WP-16** -- not on a clean run, not on cc's landing, only on vc's word. The two conditions are deliberately separate; the gap between them is where a node closes its own package on its own evidence.
- Rostered **manual**, dated not permanent. dc wires the preflight when vc signals; the path and exit contract are already with dc.

## TODO

**Review every `intent wb` verb landing, one commit per landing, on vc's per-verb signal.** The register order is amended FOR THIS FAMILY ONLY (vc, 2026-09-12): cc writes the row in the commit that BUILDS each verb, starting `wb register` in commit two; I review and correct -- help prose, exit codes, voice, `when_to_use`, MCP exposure. **The order of writing moved; my ownership did not.** The reason is structural: the SSOT cannot precede the arm across two nodes, and a row with no arm fails reachability. **It is a review ON EVERY LANDING, never a pass** -- a review that happens once misses every commit after it. vc signals per verb, so this is not a watch I keep.

**WP-14's protocol half, on vc's signal.** AC-14.10's `/in-whiteboard` rewrite onto `intent wb`; AC-14.12's deletion of `cmd_ws_new`/`list`/`archive`/`hygiene` from `intent_claude_cwi`, its sentence lifted into an AT. **cc does not touch cwi or the skill; I do not touch the model.** The live board migrates at a cutover on vc's signal only -- until then every board stays hand-authored and both guards stay. **Scoped 2026-09-12 17:47Z, nothing edited: size M.** The four functions sit at `intent_claude_cwi:218,278,298,316` with their dispatch arms at 430 to 433, and **a fifth call site at 392 offers `cmd_ws_new` from the interactive prompt** -- deleting the four functions and the four arms alone leaves that one calling a function that is gone. `in-whiteboard/SKILL.md` names the `ws` family in its scaffolding paragraph, its header-format section and its inbox-shape section, so the rewrite is not one line.

**The batched reference regeneration, after the tag, on vc's signal.** Unchanged and still pinned as two literal commands below the fold at `.history/20260912/wip-prefold-1738Z.md`: both halves `--rev v3.0.2 --baseline v3.0.1`, and **`--baseline` must be passed** because both generators hardcode `v3.0.0`. Neither needs a build.

**AFTER cc's COMMIT TWO THE LIVE STORE IS AT SCHEMA 24, by vc's hand, and vc rebuilds the shared pair at that commit and broadcasts.** Between the landing and the broadcast: NO store reads through anything. From the broadcast on, PATH `intent` is current and reads and writes through it are fine. **I do not rebuild the shared pair** -- vc does, at every landing that matters.

## Holds -- work I am NOT doing, each with the condition that releases it

0. **THE QUIET WINDOW IS OPEN NOW (vc, 2026-09-12, correcting the trigger from WP-22's landing to immediately).** Until vc says it is lifted: **no `cargo test`, no `cargo build`, no drives on this box** -- dc measures the daemon family on an idle host, then runs the final rehearsal. RELEASES WHEN vc says the window is lifted. The post-tag reference regeneration is after the tag and is unaffected by it.

1. **The palette `Home`/`End` flip** -- RELEASES WHEN hv sets post-3.0.1 work and names it.
2. **The unruled ic-lane defects** -- issue 0303 (the register's `as-observed` rows) and `subagents/.manifest/global-agents.json` (three bats tests assert it). RELEASES WHEN hv rules either in or out.

## Watch-outs -- one line each, leaned 2026-09-12 14:14Z to what bears on the work in front of me; the full list and its worked cases are in `.history/20260912/wip-prefold-1414Z.md`

THE SHARED TREE, which is where the near-misses were:

- **A canon write is verified PAST the daemon's ingest, never at the moment of it** -- read the attachment's `sha256` and `bytes` back against the file, then read them again after the ingest window, and commit the design and its canon in one call.
- **In `git status`, column ONE is a peer's index and column TWO is yours** -- `M ` is staged by somebody else mid-commit and is not your dirt; `--only` on your own paths is what keeps the two apart.

- `git add <paths>` then `git commit --only <the same paths>` in ONE call; against a peer's `index.lock` wait and re-issue the SAME command, never remove it; judge by `git log -1`.
- **`git commit --only <path>` commits the WORKING TREE version, not the staged one** -- a peer's staged edit in a file you are writing lands in YOUR commit under YOUR message. Check `git status` for `MM` before adding.
- **Never `git stash` here**: the stash list holds other sessions' entries back to v2.3.0 and a pop can apply a stranger's work. Mutate in place and restore.
- **A text edit in a shared aggregator is located by its ROW, never by a pattern** -- a replace on two common field lines put my row's note on `st hydrate`. Verify by parsing the file back and asking which row carried it.
- **Never write the register back through a serialiser**: it reformats lines nobody touched. Insert as TEXT.
- Diff a shared aggregator (`tests/suite.rs`, `CHANGELOG.md`, the register) before `git add`; a `mod` registration lands in the same commit as the file it names.

THE REGISTER, which refuses by name and is usually right:

- A new ROW moves `populations.{declared,shipped,probeable}`, `legal_pairs`'s `n` and `census_note`, and the new-surface family count in `dispatch::tests`; a FAMILY needs `flags: []` on its own row; a new KEY must be classified in `key_classes`.
- A shipped mutation needs `recoverability`, and one withheld from MCP while recoverable needs `recoverability_anomaly` -- the generator refuses the silence and says do not bend the label.
- A generated view is regenerated by its tool in the same commit and never hand-edited; the rendered markdown must be a formatter fixed point, so no `*emphasis*` in register prose.
- An exact command literal in shipped source must be declared in `command_rosters_are_derived_or_declared`, and a shipped mutator must sit in exactly one bucket of `write_moves_only_what_changed`.

BUILDING AND VERIFYING, once the quiet window lifts:

- Private detached worktree, its OWN IN-TREE target dir, isolated `HOME`; read `cat ~/.intent/home` afterwards.
- **After editing `intentsvcs`, build `intentd` BEFORE the intent-cli suite** -- the staleness guard reds the whole daemon class in 0.00s and reads as a broken machine.
- **A worktree carries a STAGED copy across `checkout --detach`** (`git checkout -- .` restores from the INDEX): use `git reset --hard <new head>`.
- A test gated on a grammar needs the CLI crate's own pass-through feature, or it silently does not compile and passes by not existing.
- An AT row citing a file is a citation only if the FILE carries the row's literal id.
- Nothing in this workspace may read a clock; bound work in SQLite instructions, not seconds.

JUDGEMENT, earned 2026-09-12 afternoon:

- **A MEASUREMENT'S POPULATION IS PART OF THE SENTENCE, AND MINE WAS NOT.** I wrote "the six `1..n` rows" into the register off a jq that selected objects carrying `flags` and never looked at `args`. Eight declarations exist; two are positionals. A claim wider than the thing it measured, written into the surface in the commit whose subject was the surface telling the truth. **Say what the query walked, in the sentence, or do not quantify.**
- **A NAME SEARCH REPORTED AS A CONCLUSION.** I told vc four entities were homeless; `Envelope` had a complete table at `event_log` all along. Two artefacts describing one thing in two naming conventions cannot be joined by name, and the failure LOOKS like a finding rather than like an error.
- **A NEW CODE PATH HAS NEVER BEEN DRIVEN, WHICH IS THE WHOLE OF AC-16.4.** The row-shaped table reader was a second reader with its own green; I drove it red both ways plus its refusal before trusting it. A reader that has only ever agreed is indistinguishable from one that always agrees.
- **A REFUSAL MUST NAME WHERE TO GO, NOT WHERE IT LOOKED.** Three entities share one table, so `absent from wb_node` sent a reader to a table holding two others. It names `wb_node#wb_message` now.
- **THE STALE CLAIM IS USUALLY BESIDE THE ONE YOU WERE SENT FOR.** vc sent me to `organize --default`'s help; the `disposition_basis` under it stated the definition-by-exclusion hv had rejected in as many words, on the very date the basis carries. The code was fixed that day and the register was not. **Read the whole row, not the field you were pointed at.**

JUDGEMENT, earned earlier:

- **READ THE LANDED CODE, NEVER THE ANNOUNCEMENT.** A release-note paragraph said a rebuild was required before a first search; the door's own doc comment said the daemonless query reconciles, and every caller was a daemon or a test. Both readings were defensible and only the call sites settled it.
- **A PEER'S LANDING SILENTLY FALSIFIES DOCUMENTATION, and nothing reports it.** WP-22 made one Upgrading paragraph wrong; the hydrate refusal made a known-defects remedy send the reader to a refusal. When a verb's behaviour moves, the pages that tell a reader to run it are the defect surface -- go and look, they will not tell you.

- **Drive a surface as a USER before calling it done.** Every face was internally consistent and passing, and two still withheld what the reader came for. No test asserting the envelope can see that, because the envelope was right.
- **A fixture that drops an argument it was handed tests a shape its caller cannot produce** -- every green it gives is about a different row from the one the test says it built.
- A claim wider than the thing is not a bug and still has to be narrowed.
- A count in a doc is DEAD (hv): name the thing, or the verb that reports the figure.
- `intent fc` is the human's verb, even in a sandbox.
- Read the clock in the same command as the stamp.
