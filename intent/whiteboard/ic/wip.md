---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 20:41Z
status: active
focus: "ALL FIVE OF hv's TUI RULINGS ARE BUILT AND ON MAIN: c5d66741 machine, 4640eab9 colour, 3655ab33 doc, 21b7e8f9 in-place FIELD, 61759934 dropdown+boot+chrome. hv drives it next; AC-09.6's read (with vc) is the live work underneath."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1955Z.md` (seventh fold today). Cold-session minimum: state, not story.**

## DOING

**RE-MEASURE EVERY FIGURE AT PICKUP. Five nodes write this tree.** Read beside HEAD `c5d66741`. **`cargo build -p intentd` FIRST or two daemon round-trip tests are a false red.** The shared tree may not build at pickup: dc's payload work (`skills.rs` -> `payload.rs`) was mid-flight at fold -- **verify in a detached worktree at HEAD if so**, which is this session's standing technique (`git worktree add --detach <scratch> HEAD`, copy your modified files in, test there, commit from the main tree by hunk).

**THE TUI REWRITE IS BUILT -- all five hv rulings, five commits: `c5d66741` machine, `4640eab9` colour-as-roles (mode chip a lamp), `3655ab33` design doc, `21b7e8f9` in-place FIELD through Model::read/write with facade.set adjudicating, `61759934` dropdown + boot-into-threads + 3-line foot.** Known next steps recorded in tui-design.md section 7: the status transitions PICKER, EMBED's pty, the `intent edit` surface wiring (section 9). **Slice 1's record ("LANDED `c5d66741`"): Mode {Omnibox rest, Nav, Menu, Field, Embed}, COMMAND deleted, Esc toggles the home pair and NEVER quits (quit = `:q` / Ctrl-C), Enter's third arm descends through doors DECLARED on rows (nav::descents + collection builders), `tui/omnibox.rs` new and pure (subsequence matcher, ordinal pin: an id's own spelling ranks first, ST0056/0056 collision planted), `tui-design.md` sections 3+4 rewritten in the same commit (the transcription test parses section 3 -- doc and machine move together or not at all). Driven under a pty against the real store: descend reaches the 151-issue list, `:` seeds, `/` menus on empty buffer only. 159/159 lib tests.

## TODO

1. **TUI: hv drives what is built; my known remainder** -- the status transitions picker (select rows currently open the text collector and `facade.set` refuses illegal states in its own words -- never silent, but the picker is the design), EMBED's pty option, `intent edit <kind> <id>` surface wiring per section 9. The pyte rig lives at `scratchpad/drive3.py` / `drive_field.py`; drive a THROWAWAY project for write tests (`scratchpad/fieldproj`), never the shared store.
2. **AC-09.6 (on the return, with vc):** the syn instrument WORKS -- `scratchpad/armscan/`, controls `st list`->st_list [1] / `fc`->[4] / `version`->[0] / `at green`->at_set. Partition at fold: 49/94 one-door, 14 zero-door, 11 multi-door, 20 walker-unresolved. **vc's rulings, all 2026-08-30: parents are NAMESPACES -- narrow them off `exposed_on_mcp` (`claude rules` etc.); no-store rows get NO exemption class -- the per-row test is "does it tell an agent about THE PROJECT" (`version` narrows out, `schema` gets a facade method, `lang list` touches config so it was mis-classed by arm shape); NO AT -- non-test row, closes by `ac satisfy` with authored evidence naming the instrument, its controls, AND its two self-caught defects (as_str scoring on 60 arms; or-patterns 6 of 26).** `claude subagents` = known-pending falsifier: dc's reshape NARROWS it rather than wires it, so the close is a narrowing note on the row. Remaining walker holes: `Some(("list", a)) | None` top-level or-arms (mechanical), parent-granularity rows (a finding, not a hole).
3. **dc's three surface items, mine, accepted 2026-08-30:** (a) the `claude` family narrowing rows in dispatch-table when dc's payload layer lands (skills + subagents -> one lifecycle, `--kind`); (b) plugin.json was a THIRD command-surface home, now stripped -- the table rows are mine to author; (c) **`agents` family-root answers rc=2 while `agents sync`/`validate` answer 0 -- real defect, FILE AS AN ISSUE on the bounce** (not yet filed).
4. **MODULES.md uncommitted**, carrying dc's web.rs row + my omnibox row -- the first committer takes both.
5. **AT-09.4 clause 2** held: class check, must not close on the one guarded instance; a deferral to hv if the common/mod.rs move fights.
6. **`AC-17.1` UNBLOCKED by cc's web.rs (2026-08-30): `POST /op` answers the socket's own dispatch, token at `~/.local/share/intent/intentd.token` 0600, `Op::Shutdown` refused over HTTP.** The browser-side realiser is mine, after the TUI slices. Still blocked on others: `AC-17.6` (`browse` unwired). Owed small: `AC-17.10` soft-wrap flags, EMBED's pty, WP-16, `0142`'s structural half, ST0064 parked.

## Watch-outs -- mechanisms only

1. **A HAND-ROLLED SPAN FINDER FAILS THREE WAYS ON render.rs; syn IS THE FLOOR.** Naive brace matching cannot find an arm body because `format!` braces are indistinguishable from block braces without a lexer. The armscan instrument is the standing proof; positive-control it on known answers EVERY run -- the controls caught both of its defects (sibling-type methods scoring; dropped or-patterns).
2. **THE SHARED TREE BREAKS IN BOTH DIRECTIONS MID-EDIT (vc's class 4).** My mode.rs broke peers' builds while dc's intentsvcs rename broke mine, same hour. The detached worktree at HEAD is the technique; a partial commit to unblock a peer is worse than the worktree.
3. **HUNK-SCOPED STAGING IS REAL AND WAS USED TWICE TODAY** (`git apply --cached <filtered patch>`): render.rs (dc's hunks co-resident) and the canon extract (a peer's dirty script record swept in by MY sync). Path-scoped `--only` sweeps co-residents; verify `git diff --cached --stat` before every commit.
4. **CANON SYNC ORDER: sync FIRST (reads the worktree), then commit file + canon together.** Bare `intent sync` does NOT re-ingest attachments; `--to-store` does, and it announces when it is safe. A commit-then-sync leaves THAT commit divergent forever.
5. **THE CRITIC GATE WAS RIGHT AGAIN** (`Result<_, String>` -> the surface's own `Refused`). Reword-and-fix beats `--no-verify` every time it has been tried.
6. **AN INSTRUMENT'S LABELS ARE PART OF THE INSTRUMENT.** The pty driver's frame labels described keys an edit never installed (a silent no-op replace); the frames were right and the labels lied. Assert the replace count in every scripted edit -- the one unasserted replace today was the one that failed.
7. **A PEER'S MESSAGE STATES THE WORLD AT SEND TIME** (vc's own words, after sizing a severance of work hv had already commissioned whole). The working tree is the more current source; prefer the measurement to the claim.
8. **`Refused` is the display-refusal type of the TUI surface; `FacadeError` + `Remedy` is the model's.** Do not mint a third.
9. **A BLOCKED PRIVATE-INDEX COMMIT LEAVES NO TRACE IN git status, AND AN ABANDONED ONE INVERTS INTO AN INCOHERENT HEAD.** The nine-file dropdown slice was roster-blocked, parked for the queue, and the later FIELD commit swept the SHARED files' accumulated content -- HEAD then failed to compile between `21b7e8f9` and `61759934` while every worktree stayed green (the gate does not build the staged set). Rule: re-land or revert a parked private-index commit THE SAME HOUR, and never land a slice that shares files with a parked one. Companion rules already ruled in: the HEAD-PIN (record HEAD at read-tree, refuse if moved -- vc, on my near-miss), and the ambient reset in the same breath (dc's cost note). Also: `intent st attach <ST> <rel> --from <file>` is the surgical attachment sync -- adopted over bare `sync --to-store` after vc's 20:24Z destroyed-write incident (announce ANY disk->store sync first).
10. **A STALE BINARY MAKES CORRECT CODE READ AS BROKEN, second costume today** -- the field drive "lost" three notices that an eprintln pass proved were always set; the pty had launched a binary built before the code existed. Build IN THE SAME BREATH as the first drive of any new behaviour.
11. **The scratchpad worktree (`scratchpad/wt-tui`) is REGISTERED in the main repo and now STALE (pre-payload HEAD)** -- `git worktree remove`/`prune` when done; peers see it in `git worktree list`.

## Decisions

- **2026-08-30 hv: TUI rewrite authorised at full scope, right away.** The WP-09 pause is hv's accepted cost; AC-00.3/AC-00.4 move with it. vc carries the consequence, not me.
- **2026-08-30 hv (via dc): `claude` is a plugin with payloads; one lifecycle (`intent claude list|install|sync|uninstall --kind`); the subagents rows get NARROWED off the table, not wired.**
- **2026-08-30 vc: AC-09.6 has no AT and needs none** -- eleven test-kind ATs covering non-test criteria estate-wide read as evidence and provide none; do not add a twelfth.
- **2026-08-30 ic (at `c5d66741`): Esc never quits; quit is an act.** Retires the ratified at-the-root-it-QUITS, recorded in tui-design.md section 3 with hv's provenance.
