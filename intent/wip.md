---
verblock: "2026-10-16:v3.14: vc - globalfold before hv's move; the 3.2.4 train and 0601 landed on main unbuilt, build all and the push are hv's next acts"
intent_version: 3.2.3
---

# Work In Progress

Measure before believing anything below: `.claude/restart.md` carries the commands, `intent/restart.md` the rules every bank, train and landing runs under. No count appears in this file.

## DOING

- **3.2.4, ONE RELEASE, CUT ONLY ONCE THE WORK RUNS ON MAIN'S HEAD (hv, 2026-10-12).** Main carries, unpushed and unbuilt: dc's train (0591, 0592, 0593, 0594, 0605, 0596, 0597 and the CHANGELOG lines, each landed by its judged patch-id with its close; HEAD dfe4ff8eb at the train's end), 0600, 0601 (b) with its fixup (c0e45eb8f) and (a1) (57d07b6d2; 0601 closed at 197503ced), ST0083's WP-01, 02, 03 and 05, ST0081's WP-06 and WP-03's docs. The installed pair is still 701c4d98b (3.2.3): `bin/devbin build all` then `intent daemon restart` are hv's next acts, with the box free, and the push follows them (the pre-push refuses until build all). Then, vc's hand: Utilz's step 2 all-clear, sinclair-vc's "build all has run" signal for the book, kovacs's o5-at051 and o7-drive-b on vc's GO, devbin's two re-booked runs. 0599 is dc's next, in 3.2.4 after the train; 0590 waits on its design question. The known-defects re-drive runs at the cut (dc, hold 31).
- **ST0083, THE AGENT CONTRACTS (cc holds it; hv decision 62's nine rulings, vc decision 63's eleven).** Landed: WP-01 the `@AGENTS.md` import, WP-02 the `contracts: root | dotclaude` key, WP-03 the doors, WP-05 the deny (`Edit(/AGENTS.md)`, `Edit(/.claude/AGENTS.md)`). Next in cc's lane: WP-07 (Dropbox-ignore `intent/.cache`), WP-04, WP-06, then WP-08 the docs after the train. Jen's book is recreated on v3 as `dotclaude` by sinclair-vc in one session after build all, from the banked steps (refs/bank/vc/contracts/reply-to-sinclair-vc.md, blob c03d21d900d0). kovacs's three repos are `contracts: root`, nothing written.
- **ST0081, INTENT ON WINDOWS THROUGH WSL (cc holds it), in lock-step with devbin's ST0008.** WP-06 landed (11017d7bd); WP-03's two pages landed (8c6e1cd39), WIP until the post-cut run; AC-07.1 satisfied on hv's word (3f037d00f), SessionStart unconfirmed on screen; 0595 fixed on main (c775f61ef, 7ee4fedd5); 0600 fixed and closed from kovacs's o6a run; o6a and o6b measured Homebrew 7's trust (project memory). Still queued on kovacs on vc's GO with a census: o5-at051 and o7-drive-b. WP-02 waits on hv's D55; WP-04 is a real-machine install hv arranges.
- **ISSUE 0605's RESTRICTION ON UTILZ** until the fixed pair is installed: no scoped `sync --to-store <ID>`, no hand edit under `intent/st/`; store-only writes are fine. hv's `intent index rebuild` repaired its store (measured, one row per key); the store file keeps its freed pages until a VACUUM of hv's hand. The fix is on main (915245eb2), unbuilt.

## TODO

- **hv's HAND, in order**: `bin/devbin build all`; `intent daemon restart`; the push, measured at the moment of pushing; optionally a VACUUM of Utilz's store file; optionally the Linux `!` re-measure of the gate on kovacs, whose Claude Code is 2.1.301.
- **vc, AFTER BUILD ALL**: Utilz's step 2 all-clear to utilz-vc; the build-all signal to sinclair-vc; GO kovacs's o5 and o7-b; re-book devbin's dc 0200 run and its cc WP-01 + 0194 run.
- **AWAITING hv**: `intent claude skills sync` and `subagents sync`, which carry 0589's catalogue and critic-prose fix to every session on the machine (vc: now); who fixes llm-tropes, the catalogue's vendor source, and Utilz's `indicators.txt`, which still hold the flattened quote classes (vc: dc for llm-tropes, Utilz's vc for Utilz); the stabilisation line after 3.2.4 (vc: yes); a SHA256SUMS file on each release, in ST0081 WP-02 (vc: yes); naming `local` and `upstream` as the release remotes in `bin/.devbin/config.yaml` (vc: yes); 0590's design question (`--project` on issues alone or global).
- **AWAITING hv, from devbin-vc**: D55 (devbin's release core attaches CI-built assets, or an upload-only workflow in Intent; both vcs lean to the first); where the joint WSL job lives; whether a hosted runner's WSL 1 counts as evidence; and one re-vendor of devbin into Intent once ST0008 is complete, which under D55's first option must land before Intent's 3.2.4 cut.
- **AFTER THE LINE, vc's hand, no push**: hv decision 28's ten Conflab and Baize rulings, which repair records. None is done.
- **AWAITING hv**: a `/refresh` in the explorer (vc: after the line; ic's, size S).
- **WITH hv**: the pushes of the other estates still unpushed, measured at the moment of pushing.
- **ISSUES UNASSIGNED**: 0598 (CLAUDE.md's four-rule index lost its reason; ST0083 WP-08's docs territory); 0602 (skills uninstall leaves the skill's empty directory, low); 0604 (CLAUDE_CONFIG_DIR relocates `~/.claude` and Intent names `$HOME/.claude`, low; measure first that the variable reaches the Bash tool and the hooks).
- **GTOOLS**: the WP-04 slot vc's 2026-09-26 resume named is unscheduled; gtools-vc owns it.
- **COURSES**: courses-vc re-drives its five Intent findings against 3.2.3 and sends vc the ones that still reproduce, to file; its trope catalogue finding is covered by 0589.
- **MACHINE, hv's call**: the resident `SetStoreUpdateService` processes, and the Spotlight-family services that pushed the load high (`ps -axo comm= | grep -c '[S]etStoreUpdateService'` measures the first).
- **ST0060** and **ST0077** stay in Triage as the next line's queue (hv, 2026-09-23).
