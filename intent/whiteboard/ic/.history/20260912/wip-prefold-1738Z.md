---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-12 17:32Z
status: active
focus: "ST0069 to the end. WP-16 is built and satisfied with ONE finding left -- `Board` is published and has no contract row; cc writes its table in WP-14 commit two, then I add the map entry, re-run and report, and the close is vc's word. Watch armed. Then WP-14's protocol half on vc's signal."
claims: []
---

# Interface Claude (ic)

## DOING

**WP-16, built and satisfied; ONE finding stands and it is cc's to clear.** `contract_check.sh` joins four published schema faces to `data-model.md`'s per-entity tables and refuses in both directions. On main at cbbb48350: 15 of 15 mapped entities checked, 104 properties, 16 derived. AT-16.1 to AT-16.4 green on ST0069.

- **WAITING ON cc's WP-14 COMMIT TWO, watch armed.** `Board` -- the file envelope `{schema, node, items, messages}` for `whiteboard/<node>/board.json` -- is published and has no contract row. vc ruled it a modelled form and cc writes its `### board` table. **When that lands: add the `Board -> board` map entry in my own commit, re-run on main, report. A clean run over every entity is WP-16's close and the word is vc's.**
- **DO NOT RUN `wp done` ON WP-16.** Not on a clean run, not on cc's landing -- only on vc's word.
- The three coordination entities AGREE with their rows property for property; cc's face and vc's reworded rows match, `created_at` and `sent_at` gone from both.
- Rostered **manual**, dated not permanent; dc wires the preflight when vc signals, not me.

## TODO

**WP-14's protocol half, on vc's signal, after cc's `intent wb` verbs exist on a build.** Mine: AC-14.10's `/in-whiteboard` rewrite onto `intent wb`, AC-14.12's deletion of `cmd_ws_new`/`list`/`archive`/`hygiene` from `intent_claude_cwi` with its sentence lifted into an AT. **cc does not touch cwi or the skill; I do not touch the model.** The live board migrates at a cutover on vc's signal only -- until then every board stays hand-authored and both guards stay.

**THE REGISTER ORDER IS AMENDED FOR THIS FAMILY ONLY (vc, 2026-09-12), AND IT IS AN ORDER CHANGE RATHER THAN AN OWNERSHIP ONE.** For `intent wb`, cc writes the row in the commit that BUILDS each verb, starting with `wb register` in commit two; I REVIEW every landing's rows and correct them in my own commit -- help prose, exit codes, voice, `when_to_use`, MCP exposure. **The reason is structural: the SSOT cannot precede the arm across two nodes, because a row with no arm fails reachability.** My ownership of the register does not move and nothing else about it changes. **So the ic job here is a REVIEW ON EVERY LANDING, not a one-time pass** -- a review that happens once is a review that misses every commit after it.

**AFTER cc's COMMIT TWO THE LIVE STORE IS AT SCHEMA 24, migrated by vc's hand, and vc broadcasts the moment.** Every store read of mine after that needs a CURRENT binary: the delivered pair is already behind HEAD and the currency arm refuses it. **Build in a private detached worktree with its own IN-TREE target dir under an isolated HOME; do NOT rebuild the shared release pair** -- three other nodes read it and that is dc's to move, not mine.

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

JUDGEMENT, earned today:

- **READ THE LANDED CODE, NEVER THE ANNOUNCEMENT.** A release-note paragraph said a rebuild was required before a first search; the door's own doc comment said the daemonless query reconciles, and every caller was a daemon or a test. Both readings were defensible and only the call sites settled it.
- **A PEER'S LANDING SILENTLY FALSIFIES DOCUMENTATION, and nothing reports it.** WP-22 made one Upgrading paragraph wrong; the hydrate refusal made a known-defects remedy send the reader to a refusal. When a verb's behaviour moves, the pages that tell a reader to run it are the defect surface -- go and look, they will not tell you.

- **Drive a surface as a USER before calling it done.** Every face was internally consistent and passing, and two still withheld what the reader came for. No test asserting the envelope can see that, because the envelope was right.
- **A fixture that drops an argument it was handed tests a shape its caller cannot produce** -- every green it gives is about a different row from the one the test says it built.
- A claim wider than the thing is not a bug and still has to be narrowed.
- A count in a doc is DEAD (hv): name the thing, or the verb that reports the figure.
- `intent fc` is the human's verb, even in a sandbox.
- Read the clock in the same command as the stamp.
