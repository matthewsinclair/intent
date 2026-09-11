---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-11 17:30Z
status: active
focus: "BOUNCE. AC-00.6 is N/N in wt006 and REHEARSED clean on top of dc's real prune (no shared path, R3 byte-identical); NOT landed. It lands when vc says dc's prune is on main. WITH vc: 0140."
claims: [ST0064]
---

# Interface Claude (ic)

## DOING

**AC-00.6, MY EIGHT MIXED bats FILES: DONE IN THE WORKTREE, BANKED, NOT LANDED.** (vc routed it 2026-09-11 17:01Z under hv's cut rulings: `hv/wip.md` decision 6 and the AC-00.6 row. vc ruled every item by 17:17Z.) **HOLD UNTIL vc SAYS dc's PRUNE IS ON MAIN.** dc's prune waits on vc's canon commit. Then rebase, re-drive and land ONE commit with "AC-00.6" in the subject. Send vc the sha, the per-file counts and the R3 dry-run diff (the message carries all three).

- **Banked, all under the scratchpad** (`/private/tmp/claude-501/-Users-matts-Devel-prj-Intent/b148e605-2046-46b1-9830-53a81fc2d54f/scratchpad/`):
  - The worktree is `wt006`, at base `e70b667f2`. Population A is deleted locally and `test_helper` points at v3; both are dc's and stay out of my commit. It has its own release build.
  - My patch is `ac006.patch` (sha256 `232f9a43b7e5d2ef`, 9 paths, +74 -297): `bin/.devbin/cmd/build.d/release` plus 8 files under `tests/unit/`, including the `rule_index.bats` deletion.
  - The commit message is drafted IN FULL in `msg006.txt`, with every deleted and edited test named with its reason.
  - The TAPs and the R3 dry-run outputs are in `b006/`.
- **Per file, ok/not ok before -> after, against v3 with v2 absent:** pr_language_code_guard 3/1 -> 4/4; release_script 15/1 -> 16/16; release_sidecars 21/3 -> 23/23; rule_pack_rust 8/1 -> 9/9; rule_validator 4/5 -> 7/7; rule_index 0/8 -> deleted; test_autopsy 16/7 -> 18/18; test_diogenes 10/9 -> 12/12.
- **R3 (the release script):** the native pair is folded into the one `SIDECAR_FILES` declaration. The dry run, with `--dry-run --patch --skip-tests` in the clone, is byte-identical before and after (`diff` rc 0). The control, with Cargo.lock dropped, changes line 23.
- **REHEARSED ON dc's REAL PATCH, 2026-09-11 after the compact:** `d6-prune.patch` (86 paths, in dc's scratchpad) and mine share NO path. Both apply clean on `5268f5e3`, 95 paths. The seven files are N/N there (same counts as above). R3 was re-run with dc's prune committed underneath: byte-identical before and after (rc 0), identical to the first run, and the Cargo.lock control changes line 23 again. The rehearsal worktree and clone are removed.
- **THE BINARY MUST BE THE LANDING TREE'S OWN BUILD.** v3 takes its install root from `current_exe`, so a binary from another tree validates against THAT tree's corpus. Every rule file under test then reads as a duplicate id: rule_pack_rust went 8/9 and rule_validator 6/7 in the rehearsal until the binary sat inside the tree. Never point `INTENT_BIN` across trees.
- **TO LAND after vc's word:**
  - Take a fresh detached worktree at the new main, which has dc's prune in it. `git apply` the patch there and confirm it applies clean. If dc's prune touched any of my 8 files, stop and reconcile.
  - Rebuild the release under an isolated HOME and re-run the eight files (seven, since rule_index is deleted).
  - Land by patch into the shared tree: explicit `git add` of the 9 paths (`git rm` for rule_index), then `git commit --only` of them, with `-F msg006.txt`. Diff the commit against the patch.

**WITH vc FOR THE RE-DRIVE: `0140`** -- `ac edit --note <text>`. The fix is `e396bf90`, the reference page `3da61a0a`, and the 0211 note `323a9785`. vc closes it, not me.

## TODO

**Empty.** Every open row is landed and with vc, marked not workable, or dc's (hv decision 3). vc will say if a drive sends something back. The lane column of `intent/wip.md` is the authority.

**THE RULES ON THE BOUNCE (vc, under hv), for anything that comes back:** claim the id in DOING, one at a time. Check for a prior fix first. Put a design call to vc, with options, before writing code. Write ONE proving test, seen red on the unfixed tree. Commit with the id in the subject, and send vc the sha, the repro as it prints now, and ONE control. A defect found while fixing goes in the commit message, not on the list. Never close it myself.

## Holds -- work I am NOT doing, each with the condition that releases it

1. **`ST0064` AC-01.7** -- RELEASES WHEN hv signs and notarises the menubar app with their own credentials. hv ruled 2026-09-11 that the app ships in 3.0.1 (decision 7), and cc landed `app-sign` / `app-notarize` at `56322937`. **cc's finding, not an ask:** `native/macos/Intent/Intent/Info.plist` has no `CFBundleExecutable`. If notarisation refuses it, the fix is `CFBundleExecutable` = `$(EXECUTABLE_NAME)`.
2. **The palette `Home`/`End` flip** -- post-cut; product feel; no criterion names it.

## Watch-outs -- the ones that bite if an item comes back

**Cut hard for the compact. Every earlier entry, with its worked case, is verbatim in `.history/20260911/` (the `wip-prefold-*` and `precull-*` files).**

- **SHARED TREE: `git add` the explicit paths, then `git commit --only <explicit paths>`.** `--only` does not stage an untracked file. **Never remove a peer's `index.lock`: re-issue the SAME command.** Put the wait IMMEDIATELY before `git add`: a peer took the lock between my check and my add twice on 2026-09-11.
- **BUILD AND TEST ONLY IN A PRIVATE DETACHED WORKTREE, UNDER AN ISOLATED `HOME`. EACH GUARDS A DIFFERENT THING, AND ONE IS NOT THE OTHER.**
  - The isolated `HOME` (`CARGO_HOME`/`RUSTUP_HOME` pointed at the real toolchain) guards `~/.intent/home`: `dual_path_conformance` runs `intent bootstrap` in-process, and `publish_home()` resolves the install from the TEST binary. At 16:48Z on 2026-09-11 `~/.intent/home` named **cc's `wt-cc`**, from a run that isolated the store but not HOME. cc keeps `wt-cc` until hv restores the pointer (`~/.local/bin/intent bootstrap`, no `--force`). **Read `cat ~/.intent/home` before removing ANY worktree.**
  - The worktree guards the live store: `attachment_drift_detected.rs` runs doctor on `repo_root()`, and a run from the shared checkout migrated the live `intent.db` 17 -> 18.
- **LAND BY PATCH, AND DIFF THE COMMIT AGAINST THE TESTED PATCH.** Before landing, check `git log <base>..HEAD` and `git diff --stat <base> HEAD -- <my paths>`. If code moved underneath, rebase, rebuild `intentd`, and rerun.
- **A NEW DISPATCH-TABLE ROW OR FLAG: THE GENERATOR'S GREEN IS NOT A STARTUP PROOF** (`owner_wp: null` panicked every command). A new ROW moves the status count, the `legal_pairs` census and the three populations; a new FLAG moves none of them. Retiring a row's only writer flips it to `read`: drop `recoverability`, leave the writes-nothing bucket, and supersede `mcp_review` inline (the 0139 / 0181 shape). Insert as TEXT, since `jq` reformats the whole file.
- **`docs/reference/*.md` IS GENERATED FROM A COMMITTED REVISION** (`gen_reference.sh --rev <sha> --out <scratch>`). When a fix changes a verb's surface, regenerate that family's page at the fix sha as its own commit, and take only that page. The rest of the set is behind HEAD, and that is not the item's to fix.
- **A DOC LINE IS A CLAIM: DRIVE EVERY CLAUSE BEFORE COMMITTING IT.** Twice on 2026-09-11 I wrote a clause before measuring it: the 17-page diff claim and the descoped/withdrawn refusal. Both held, and both were luck until driven.
- **TO PROVE A WRITE SURVIVES A DAEMON INGEST, FORCE ONE** (author an unrelated file, wait for `disk.sync_from_disk`). The daemon HOME is a short `mktemp -d`, because the socket is limited to 104 bytes.
- **THE BASH TOOL IS zsh:** an unquoted `$var` does not word-split (use `${(@f)...}` for a path list), `pipestatus` replaces `PIPESTATUS`, and **a bare `====` is an `=cmd` expansion that aborts the whole command** (hit at 16:52Z; quote separators). **Read the clock in the same command as the stamp** (`NOW=$(date -u ...)`).
- **`intent fc` IS THE HUMAN'S VERB, EVEN IN A SANDBOX.**
