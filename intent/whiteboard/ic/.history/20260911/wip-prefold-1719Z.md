---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-11 17:11Z
status: active
focus: "THE CUT, 2026-09-11 17:01Z: AC-00.6 mixed bats files, 8 of 16, routed by vc. Private worktree, own v3 release build, isolated HOME. Lands after dc's prune commit. WITH vc: 0140."
claims: [ST0064]
---

# Interface Claude (ic)

## DOING

**THE CUT: AC-00.6, THE MIXED bats FILES (vc, 2026-09-11 17:01Z, under hv's cut rulings on `hv/wip.md`, decision 6 and the AC-00.6 row).** Mine: `pr_language_code_guard`, `release_script`, `release_sidecars`, `rule_pack_rust`, `rule_validator`, `rule_index`, `test_autopsy`, `test_diogenes`. Delete only a test whose subject is a v2 door or a v2 output format, and name each one in the commit. A real v3 difference goes to vc with expected and actual, and is neither deleted nor edited. A file that is all v2 doors is deleted whole. Each file ends N/N against v3. Work in a private worktree at HEAD with population A deleted locally, its own v3 release build, and an isolated HOME. Land one commit after dc's prune is on main, with "AC-00.6" in the subject. Send vc the sha and per-file before and after counts.

**STATE, 2026-09-11 17:11Z: SORTED AND WITH vc; WAITING ON vc's RULINGS AND ON dc's PRUNE.** The worktree is `scratchpad/wt006` at `e70b667f`, with its own release build. Its before-counts match dc's exactly. Deleted there: `rule_index.bats` whole, plus 8 v2-format tests. Proposed to vc and not yet applied: 5 tests that fail only on v2's `installing:` line. With vc to rule: R1 to R12, 14 tests, none deleted or edited. The TAPs are in `scratchpad/b006/`.

**WITH vc FOR THE RE-DRIVE: `0140`** -- `ac edit --note <text>` (vc's option A, the `at edit --note` shape). The fix is `e396bf90`, the reference page `3da61a0a`, and the known-defects 0211 note `323a9785`. vc closes it, not me. If a drive sends it back, rebuild a private worktree at HEAD.

**hv's shell run at 16:50Z (`tmp/test/20260911-1650.SHELL.*`), measured and reported to hv, not fixed.** 51 failures, all one cause: `bin/intent:123` answers only `claude hook|start|ws` since `d8a8c070` (AC-12.1, 2026-09-10 09:49Z), and `tests/unit/agent_commands.bats` (50) plus `ambient_project_root_guard.bats` test 86 still drive `intent claude subagents` through v2. Neither file has been touched since 2026-08-14. The run reported 130 of 1525 (it ended in `at_lint_wp_scope.bats`), and the Rust log is its header only.

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
