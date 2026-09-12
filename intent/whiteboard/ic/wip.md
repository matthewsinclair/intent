---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-12 16:41Z
status: active
focus: "ST0069 to the end on hv's word. WP-16 is BUILT and its four criteria are satisfied (fdf4bf565, d9050f777): contract_check.sh compares the published schema faces to data-model.md in both directions, twelve of twelve entities, and it found real drift on its first run. WP-16 CLOSES ON vc's WORD after cc lands the whiteboard faces, not before. Next: the 0305 register edit behind cc's parser fix, then WP-14's protocol half on vc's signal."
claims: []
---

# Interface Claude (ic)

## DOING

**WP-16, built and satisfied; it does not CLOSE yet.** `contract_check.sh` (`intent/st/ST0056/parity/tools/`) joins the three published schema faces to `data-model.md`'s per-entity tables and refuses in both directions. Populations derived at run time on both sides; the join is a declared map, because the two sides share no naming convention and a name-match reports `Envelope` homeless while its table sits at `event_log` complete -- the false finding I filed before writing the tool. AT-16.1 to AT-16.4 green on ST0069.

- **It found drift on its first run:** `Attachment.blob`, a contract row describing a property nothing publishes. Corrected on vc's ruling, the fact kept in prose because what was wrong was the claim and not the knowledge. Writing the `subject` table found a second: the envelope's row said `{kind, id}` and the face has always carried `type`.
- **ONE FINDING STANDS AND IT IS RULED:** `wb_node` / `wb_item` / `wb_message` are pending by construction -- specified, not built, no face published. Not an exemption. **DO NOT RUN `wp done` ON WP-16** until vc says cc's faces have landed.
- Rostered **manual**, dated not permanent; dc wires the preflight line, not me. Path and exit contract are with dc.

## TODO

**The 0305 register edit, behind cc's parser fix** (waiting on it now). Ruled by vc: `arity: "1..n"` means REPEATABLE, one value per occurrence, rendered with no ellipsis -- the ellipsis was clap's multi-value greed swallowing the query. Two edits in one commit: the word `repeatable` onto the three rows whose help lacks it (`at new --covers`, `at edit --covers`, `critic --files`; the three `search` flags already carry it), and the meaning of `1..n` written where the arity vocabulary is defined. `dispatch-table.md` regenerates in the same commit under the skew guard; **the register never meets prettier**; my commit does not touch the spine.

**WP-14's protocol half, on vc's signal, after cc's `intent wb` verbs exist on a build.** Mine: the register rows for `intent wb` (the surface is my SSOT), AC-14.10's `/in-whiteboard` rewrite onto `intent wb`, AC-14.12's deletion of `cmd_ws_new`/`list`/`archive`/`hygiene` from `intent_claude_cwi` with its sentence lifted into an AT. **cc does not touch cwi or the skill; I do not touch the model.** The live board migrates at a cutover on vc's signal only -- until then every board stays hand-authored and both guards stay.

**The batched reference regeneration.** ONE run, both halves keyed to v3.0.2, AFTER the tag exists. `docs/reference/**` from `gen_reference.sh` and `cut-surface.md` from `gen_cut_surface.sh`, which is keyed to the TAG. **The signal is vc's.** Collected so far: cc's batch 2 (printed output only), cc's organize preview lines, dc's single-line `claude upgrade --force` help change, and the search surface's own changes -- every flag added to the `search` row (dc's `--no-reconcile` among them, its markdown already regenerated), the `index` family, `when_to_use` on the two rows that carry it. Name them, do not count them: the count was wrong within the hour. On the pages' own contract (per-verb help and arguments, no output lines, no per-verb exit tables) none of it forces a regeneration; the one run re-keys everything anyway.

**The run itself is pinned, so the signal is an execution and not a decision** (read off the two generators, 2026-09-12 16:07Z, no build involved -- both read the register out of git at a revision with `git show <rev>:surface/dispatch-table.json`, so the quiet window does not touch them and neither needs the delivered binary):

```
intent/st/ST0056/parity/tools/gen_reference.sh   --rev v3.0.2 --baseline v3.0.1 --out docs/reference
intent/st/ST0056/parity/tools/gen_cut_surface.sh --rev v3.0.2 --baseline v3.0.1 --out docs/reference/cut-surface.md
```

**`--baseline` MUST be passed and MUST be v3.0.1.** Its default is hardcoded `v3.0.0` in `gen_reference.sh` and in its sibling, so a bare `--rev v3.0.2` emits a presence-and-delta section reporting two releases of change as one -- wrong, and wrong in a way the output states confidently with a revision beside it. The default was right for exactly one cut and nothing updates it.

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
