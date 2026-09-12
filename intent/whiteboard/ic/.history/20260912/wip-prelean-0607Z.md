---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-12 06:02Z
status: active
focus: "HOLD for hv's rulings on the defect list in intent/wip.md -- nothing in ic's lane is ruled, and batch 1 (release and install) is dc's. The doc audit lane is done, verified and pushed. Nothing in flight."
claims: []
---

# Interface Claude (ic)

## DOING

**Empty.**

## TODO

**Empty. Hold until hv rules an ic-lane item in `intent/wip.md`** (vc, 2026-09-11; hv has ruled batch 1, release and install, to dc).

**BANKED -- the ic lane of hv's doc audit, done and pushed 2026-09-11.** The board as it stood is verbatim in `.history/20260912/wip-prefold-0602Z.md`; the audit record is in `.history/20260911/wip-prefold-2126Z.md` and in the commits (`97e12733` through `05bb44ca`, plus issue `0303` at `c0e8907c`).

- **The lane:** `docs/reference/**`, `surface/*.md`, `lib/help/**` (all deleted as DEAD), and the canon skills and subagents. vc verified it; the per-file report covered every file in it.
- **HOW TO REGENERATE MY LANE'S PAGES** -- all three generators live under `intent/st/ST0056/parity/tools/`:
  - `gen_reference.sh --rev <sha> --baseline v3.0.1` writes the family pages, `commands.md` and `index.md`;
  - `gen_cut_surface.sh --rev <sha> --baseline v3.0.0 --out docs/reference/cut-surface.md`, whose baseline stays 3.0.0 because the page exists to show what the cut added;
  - `IN=surface/dispatch-table.json OUT=surface/dispatch-table.md bash .../gen_dispatch_table.sh` renders the register view;
  - then `prettier --write`, which the generators are written to survive.
  - **Each generator is an ST0056 ATTACHMENT**: editing one means `intent st attach ST0056 parity/tools/<file> --from <path>`, which rewrites `intent/.canon/st/ST0056.json`. **Ask vc for the ST0056 window first** -- vc attaches its prose to the same file, and `--only` is path-scoped, so whoever commits that file commits every hunk in it.
- **OPEN, AND NOT MINE TO CLOSE:** issue `0303` (the register's `as-observed` rows claim v3 reproduces v2's exits; five of six sampled are false) waits on hv. `subagents/.manifest/global-agents.json` is held for hv: dead to v3, but three bats tests assert it. The CODE findings are items 11 and 21 of `intent/wip.md`.

## Holds -- work I am NOT doing, each with the condition that releases it

1. **The palette `Home`/`End` flip** -- RELEASES WHEN hv sets post-3.0.1 work and names it. Product feel; no criterion names it.

## Watch-outs -- the ones that bite if an item comes back

**Cut hard for the compact. Every earlier entry, with its worked case, is verbatim in `.history/20260911/` (the `wip-prefold-*` and `precull-*` files).**

- **SHARED TREE: `git add` the explicit paths, then `git commit --only <explicit paths>`.** `--only` does not stage an untracked file. **Never remove a peer's `index.lock`: re-issue the SAME command.** Put the wait IMMEDIATELY before `git add`: a peer took the lock between my check and my add twice on 2026-09-11.
- **BUILD AND TEST ONLY IN A PRIVATE DETACHED WORKTREE, UNDER AN ISOLATED `HOME`. EACH GUARDS A DIFFERENT THING, AND ONE IS NOT THE OTHER.**
  - The isolated `HOME` (`CARGO_HOME`/`RUSTUP_HOME` pointed at the real toolchain) guards `~/.intent/home`: `dual_path_conformance` runs `intent bootstrap` in-process, and `publish_home()` resolves the install from the TEST binary. At 16:48Z on 2026-09-11 `~/.intent/home` named **cc's `wt-cc`**, from a run that isolated the store but not HOME; it was restored the same day and read `/Users/matts/Devel/prj/Intent` at every check after. The repair is `~/.local/bin/intent bootstrap`, no `--force`. **Read `cat ~/.intent/home` before removing ANY worktree.**
  - The worktree guards the live store: `attachment_drift_detected.rs` runs doctor on `repo_root()`, and a run from the shared checkout migrated the live `intent.db` 17 -> 18.
- **bats READS THE CORPUS OF THE TREE ITS BINARY LIVES IN.** v3 takes its install root from `current_exe`, so an `INTENT_BIN` pointed at another tree's binary validates this tree's rule files against that tree's corpus, and every rule under test reads as a duplicate id. Use the tree's own `target/release/intent` (test_helper's default); never point `INTENT_BIN` across trees.
- **A COUNT IN A DOC IS DEAD (hv, 2026-09-11): "what does a hardcoded number like that mean? ... nothing."** Name the things, describe the change, or name the verb that reports the figure; a count survives only as evidence in a commit message or an issue. An operating threshold that instructs the reader ("split a WP over 60 files") is an instruction, not a count. This outranks the rest of the audit protocol.
- **THE DOC AUDIT'S ONE RULE: AS-BUILT IS THE TRUTH, AND THE DOC MOVES TO THE CODE.** Measure every checkable claim -- drive the verb read-only, or cite `file:line` at HEAD -- and anything that writes runs in a scratch project under an isolated HOME. A doc that is right where the code is wrong goes to vc as a CODE finding, unfixed.
- **NAME A DELETION TO vc BEFORE IT LANDS** when vc has not ruled on that file (vc, 2026-09-11, on `installed-agents.json`: the deletion held, but it landed unannounced). A DEAD disposition under the protocol is not a ruling on the particular file.
- **LAND BY PATCH, AND DIFF THE COMMIT AGAINST THE TESTED PATCH.** Before landing, check `git log <base>..HEAD` and `git diff --stat <base> HEAD -- <my paths>`. If code moved underneath, rebase, rebuild `intentd`, and rerun.
- **A NEW DISPATCH-TABLE ROW OR FLAG: THE GENERATOR'S GREEN IS NOT A STARTUP PROOF** (`owner_wp: null` panicked every command). A new ROW moves the status count, the `legal_pairs` census and the three populations; a new FLAG moves none of them. Retiring a row's only writer flips it to `read`: drop `recoverability`, leave the writes-nothing bucket, and supersede `mcp_review` inline (the 0139 / 0181 shape). Insert as TEXT, since `jq` reformats the whole file.
- **`docs/reference/*.md` IS GENERATED FROM A COMMITTED REVISION** (`gen_reference.sh --rev <sha> --out <scratch>`). When a fix changes a verb's surface, regenerate that family's page at the fix sha as its own commit, and take only that page. The rest of the set is behind HEAD, and that is not the item's to fix.
- **A DOC LINE IS A CLAIM: DRIVE EVERY CLAUSE BEFORE COMMITTING IT.** Twice on 2026-09-11 I wrote a clause before measuring it: the 17-page diff claim and the descoped/withdrawn refusal. Both held, and both were luck until driven.
- **TO PROVE A WRITE SURVIVES A DAEMON INGEST, FORCE ONE** (author an unrelated file, wait for `disk.sync_from_disk`). The daemon HOME is a short `mktemp -d`, because the socket is limited to 104 bytes.
- **THE BASH TOOL IS zsh:** an unquoted `$var` does not word-split (use `${(@f)...}` for a path list), `pipestatus` replaces `PIPESTATUS`, and **a bare `====` is an `=cmd` expansion that aborts the whole command** (hit at 16:52Z; quote separators). **Read the clock in the same command as the stamp** (`NOW=$(date -u ...)`).
- **`intent fc` IS THE HUMAN'S VERB, EVEN IN A SANDBOX.**
