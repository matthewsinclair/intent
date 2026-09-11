---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-11 19:49Z
status: active
focus: "THE DOC AUDIT, ic LANE (hv set it 2026-09-11; vc directs; orders in intent/wip.md at 9a1455a0f). docs/reference, surface, lib/help, canon skills + subagents. As-built is the truth; no code; nothing pushed."
claims: []
---

# Interface Claude (ic)

## DOING

**THE DOC AUDIT, ic's LANE** (hv, 2026-09-11; vc directs; the full protocol is in `intent/wip.md` at `9a1455a0f`). Size XL overall. The lane:

- `docs/reference/**`, measured against the dispatch table and each verb's `--help`.
- `surface/*.md`, including `agent-guide.spec.md`.
- `lib/help/*.help.md`: which ones `guide.rs`/`render.rs` still serve. Orphans are DEAD.
- `intent/plugins/claude/skills/**` and `intent/plugins/claude/subagents/**`: every verb, flag, path and output they cite. cc owns the tree; the job is mine (vc).

**The rules that bite:** the doc moves to the code, and a CODE-WRONG goes to vc with `file:line`, unfixed. Measure every claim, read-only or under an isolated HOME in a scratch project. Generated files are fixed at their source. Commit my lane's paths only. **No `skills sync`: vc resyncs at the end.** Nothing is pushed. The report to vc lists EVERY file with its disposition; the inventory is in the scratchpad at `audit/`.

## TODO

**Empty. No new work until hv sets it** (vc relaying hv, 2026-09-11, after 3.0.1 shipped). The bounce's working rules expired with the release and are verbatim in `.history/20260911/wip-prefold-1938Z.md`.

## Holds -- work I am NOT doing, each with the condition that releases it

1. **The palette `Home`/`End` flip** -- RELEASES WHEN hv sets post-3.0.1 work and names it. Product feel; no criterion names it.

## Watch-outs -- the ones that bite if an item comes back

**Cut hard for the compact. Every earlier entry, with its worked case, is verbatim in `.history/20260911/` (the `wip-prefold-*` and `precull-*` files).**

- **SHARED TREE: `git add` the explicit paths, then `git commit --only <explicit paths>`.** `--only` does not stage an untracked file. **Never remove a peer's `index.lock`: re-issue the SAME command.** Put the wait IMMEDIATELY before `git add`: a peer took the lock between my check and my add twice on 2026-09-11.
- **BUILD AND TEST ONLY IN A PRIVATE DETACHED WORKTREE, UNDER AN ISOLATED `HOME`. EACH GUARDS A DIFFERENT THING, AND ONE IS NOT THE OTHER.**
  - The isolated `HOME` (`CARGO_HOME`/`RUSTUP_HOME` pointed at the real toolchain) guards `~/.intent/home`: `dual_path_conformance` runs `intent bootstrap` in-process, and `publish_home()` resolves the install from the TEST binary. At 16:48Z on 2026-09-11 `~/.intent/home` named **cc's `wt-cc`**, from a run that isolated the store but not HOME; it was restored the same day and read `/Users/matts/Devel/prj/Intent` at every check after. The repair is `~/.local/bin/intent bootstrap`, no `--force`. **Read `cat ~/.intent/home` before removing ANY worktree.**
  - The worktree guards the live store: `attachment_drift_detected.rs` runs doctor on `repo_root()`, and a run from the shared checkout migrated the live `intent.db` 17 -> 18.
- **bats READS THE CORPUS OF THE TREE ITS BINARY LIVES IN.** v3 takes its install root from `current_exe`, so an `INTENT_BIN` pointed at another tree's binary validates this tree's rule files against that tree's corpus, and every rule under test reads as a duplicate id. Use the tree's own `target/release/intent` (test_helper's default); never point `INTENT_BIN` across trees.
- **LAND BY PATCH, AND DIFF THE COMMIT AGAINST THE TESTED PATCH.** Before landing, check `git log <base>..HEAD` and `git diff --stat <base> HEAD -- <my paths>`. If code moved underneath, rebase, rebuild `intentd`, and rerun.
- **A NEW DISPATCH-TABLE ROW OR FLAG: THE GENERATOR'S GREEN IS NOT A STARTUP PROOF** (`owner_wp: null` panicked every command). A new ROW moves the status count, the `legal_pairs` census and the three populations; a new FLAG moves none of them. Retiring a row's only writer flips it to `read`: drop `recoverability`, leave the writes-nothing bucket, and supersede `mcp_review` inline (the 0139 / 0181 shape). Insert as TEXT, since `jq` reformats the whole file.
- **`docs/reference/*.md` IS GENERATED FROM A COMMITTED REVISION** (`gen_reference.sh --rev <sha> --out <scratch>`). When a fix changes a verb's surface, regenerate that family's page at the fix sha as its own commit, and take only that page. The rest of the set is behind HEAD, and that is not the item's to fix.
- **A DOC LINE IS A CLAIM: DRIVE EVERY CLAUSE BEFORE COMMITTING IT.** Twice on 2026-09-11 I wrote a clause before measuring it: the 17-page diff claim and the descoped/withdrawn refusal. Both held, and both were luck until driven.
- **TO PROVE A WRITE SURVIVES A DAEMON INGEST, FORCE ONE** (author an unrelated file, wait for `disk.sync_from_disk`). The daemon HOME is a short `mktemp -d`, because the socket is limited to 104 bytes.
- **THE BASH TOOL IS zsh:** an unquoted `$var` does not word-split (use `${(@f)...}` for a path list), `pipestatus` replaces `PIPESTATUS`, and **a bare `====` is an `=cmd` expansion that aborts the whole command** (hit at 16:52Z; quote separators). **Read the clock in the same command as the stamp** (`NOW=$(date -u ...)`).
- **`intent fc` IS THE HUMAN'S VERB, EVEN IN A SANDBOX.**
