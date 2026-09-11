---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-11 13:46Z
status: active
focus: "LOCALFOLD 2026-09-11 13:14Z ON hv'S PAUSE, THEN THE COMPACT. NOTHING IN FLIGHT, NOTHING UNCOMMITTED. WITH vc: 0154 + 0185 at 1f2f8f6a. ON THE BOUNCE, AFTER vc CLOSES THEM OR SENDS THEM BACK: #50 0139, then 0140, 0146, 0153, 0231 (0066 and 0145 moved to cc) -- the lane column of `intent/wip.md` is the authority. hv: NO NEW WORK; these items ONLY."
claims: [ST0064]
---

# Interface Claude (ic)

## DOING

**LOCALFOLD 2026-09-11 13:14Z ON hv's PAUSE, ON EXECUTION.** The pre-fold board is verbatim at `.history/20260911/wip-prefold-1312Z.md`, committed alone and confirmed TRACKED before this line cited it. **Nothing in flight, nothing uncommitted, no private worktree left, and no daemon running.**

**WITH vc FOR THE RE-DRIVE: `0154` + `0185` at `1f2f8f6a`** (`intent set <address> <field> (<value> | --from <file>)`, one scriptable door over `Facade::set`). The sha, the repro, one control, all five conditions and three found-while-fixing items were sent to vc at 13:10Z. vc closes them, not me.

**Closed by vc since the last fold:** 0194 (`ba338f37`), 0223 (`0b910733`), 0240 (`421c40ee`), 0195 (`8d3b13ae`), 0121. **Moved to cc by vc:** 0083, 0168, 0176.

## TODO -- THE BOUNCE: my lane of the 3.0.1 finish line, in list order

**Source: the lane column of `intent/wip.md`. hv: _THERE IS NO NEW WORK TO BE DONE. We are working on these items and these items ONLY._ hv cuts from the bottom.** Re-drive it on return; it is vc's and it moves.

| #   | Issue  | Sev    | Defect (verbatim from `intent/wip.md`)                                   |
| --- | ------ | ------ | ------------------------------------------------------------------------ |
| 50  | `0139` | medium | `at lint --fix` is advertised and refuses.                               |
| 54  | `0140` | medium | An unsatisfied note is writable only by migration.                       |
| 73  | `0146` | medium | `at edit` is kind-blind and its remedy produces a row lint cannot judge. |
| 74  | `0153` | medium | `intent edit` refuses the address form its own remedy recommends.        |
| 84  | `0231` | medium | TUI repaints can tear on real terminals.                                 |

**THE RULES ON THE BOUNCE (vc, under hv):**

- Claim the id in DOING, **one item at a time**. The next starts only after vc closes the last or sends it back.
- Commit with the id in the subject and tell vc: **the fix sha, the issue's reproduction as it prints now, and ONE control.** vc re-drives, then closes. Never close it myself.
- **Check for a prior fix FIRST** (`git log -S`, the issue id in source comments). Of today's eight, four were delivered and never closed, and one no longer reproduced.
- **A defect found while fixing goes in the commit message, not on the list.**
- **No new tests beyond the ONE that proves the item, seen RED on the old code.** No new instruments, guards, criteria or threads.

## Holds -- work I am NOT doing, each with the condition that releases it

**Cut to the 3.0.1 finish line by vc on 2026-09-11 under hv's instruction; the pre-cull section is verbatim at `.history/20260911/precull-ic-wip-md-0851Z.md`.**

1. **`ST0064` AC-01.7** -- RELEASES WHEN hv signs and notarises the app with their own credentials, or drops the app from 3.0.1. Decision 7 on the finish line (`intent/wip.md`).
2. **The palette `Home`/`End` flip** -- post-cut; product feel; no criterion names it.

Culled with the loop: the estate-wide burn (AC-06.1 and AC-00.1 withdrawn), the `v2:` census (AC-00.16 withdrawn), `0222` (WP-17 is done; the issue stays open as a product defect).

## Watch-outs -- the ones that bear on fixing CLI defects on a shared tree

**Cut to rules that bite THIS work. All 129 entries, with their worked cases, are verbatim in the archive above.**

- **SHARED TREE: `git add` the explicit paths, then `git commit --only <explicit paths>`** -- `--only` does NOT stage an untracked file, and it is path-scoped rather than hunk-scoped. **Never remove a peer's `index.lock`: wait, then re-issue the SAME command.** Put the wait BEFORE `git add`, since `add` needs the lock too, and a silent `add` in a retry loop makes every iteration a no-op.
- **`ok:` IS THE TOOL'S REPORT, NOT EVIDENCE THE WRITE SURVIVED (`0216`).** A canon write can land and then be reverted by the daemon's disk ingest about a second later. **Verify AFTER the ingest.** `sync --to-store` REPLACES the store from the extract, so hand-edit canon FIRST and drive the verbs AFTER.
- **A GENERATED VIEW HEALS ITSELF ON THE NEXT RENDER; A HAND-AUTHORED FILE CARRYING THE SAME VALUE DOES NOT,** and the tree cannot tell the two apart. That is `0192`'s whole subject: know which one you are editing.
- **DRIVE A DESTRUCTIVE QUESTION IN A SANDBOX OR DO NOT DRIVE IT.** Three of the first three items are data loss, so reproduce them in a throwaway `HOME`/project and never on this tree's store.
- **THE ONE TEST MUST BE SEEN RED ON THE UNFIXED TREE BEFORE IT COUNTS.** A green that would also pass against the bug is decoration. The load-bearing assertion is the place to look first.
- **BOTH RUST CRATES HAVE ONE `suite` TEST TARGET:** `cargo test -p <crate> --test suite <filter>`. **`one_clock` guards the whole workspace and its EXEMPT list is empty** (hv), so a new test must not read `Instant::now`/`SystemTime`. A filtered `--workspace` run prints mostly `0 passed … filtered out`, so read the ONE line with a count.
- **`intent --version` names the commit the binary was built from.** A behaviour claim needs a current binary; a store claim survives a stale one.
- **THE BASH TOOL'S SHELL IS zsh:** no word-splitting on an unquoted `$var`, an unmatched glob aborts the command, and it's `pipestatus` rather than `PIPESTATUS`. **Backticks inside a double-quoted argument are command substitution**, so the command that runs is the one you were writing about.
- **A STATUS FIELD IS A CLAIM BY WHOEVER LAST DROVE A VERB; A COMPUTED RESULT IS DERIVED (W129).** When the two disagree, believe the derived one. That includes an issue reading `open` against a commit that delivered it (`0218`, 2026-09-11).
- **MEASURE BEFORE SPEAKING. The ordering is the whole rule:** three of four wrong claims on 2026-09-10 went out before the evidence existed. Name the observable and stop, because a mechanism nobody drove is a story.
- **EVERY `cargo test` AND BUILD RUNS UNDER AN ISOLATED `HOME`** (`CARGO_HOME`/`RUSTUP_HOME` pointed at the real toolchain). `dual_path_conformance` runs `intent bootstrap` IN-PROCESS under the real `HOME`, and `publish_home()` resolves the install from the TEST BINARY's location. From a scratch worktree it repointed `~/.intent/home` at 09:58:55Z, and deleting the worktree then made the pre-commit shim refuse EVERY commit in the repo. Restored with `~/.local/bin/intent bootstrap` (no `--force`).
- **DEVELOP IN A PRIVATE DETACHED WORKTREE; LAND BY PATCH.** cc keeps `facade.rs` dirty, and `commit --only` is whole-file, so wait for the peer's commit, rebase the worktree onto `HEAD`, rerun, `git apply --check`, apply, commit exactly my paths, and `diff` the commit against the tested patch.
- **AN EXISTING FIXTURE CAN ENCODE THE DEFECT** (three for `0079`, one for `0291`). Repair the fixture so it reaches its state honestly; never touch the assertion.
- **`intent fc` IS THE HUMAN'S VERB EVEN IN A SANDBOX.** To drive a fiat state, hand-set `{"is":"fiat",...}` in canon and `sync --to-store`.
- **WHEN THE FIX IS A DESIGN CALL, PUT THE OPTIONS TO vc BEFORE WRITING CODE,** with one recommendation and what each costs. 0223, 0195 and 0154+0185 each came back ruled with conditions that reshaped the build, and vc's condition 2 on 0223 (stop if clap cannot tell `x -- help` from `x help`) fired: clap_builder 4.6.6 keeps `--` in `pub(crate)` state only.
- **A NEW DISPATCH-TABLE ROW: THE GENERATOR'S GREEN IS NOT A STARTUP PROOF.** `gen_dispatch_table.sh` accepted `owner_wp: null` and the binary then panicked EVERY command at rc=101. A new row also moves the status sentence's count, the `legal_pairs` census, `populations.declared/shipped/probeable`, and a mutating verb needs a bucket in `write_moves_only_what_changed`. Insert the row as TEXT: `jq` rewrites the whole file's formatting.
- **TO PROVE A WRITE SURVIVES A DAEMON INGEST, FORCE ONE.** A daemon that has opened a project records nothing until something on disk moves, so "it held for 4s" alone proves nothing. Author an unrelated file, wait for `disk.sync_from_disk` in `intent events`, THEN read back the store and canon. The daemon socket must fit in 104 bytes, so its HOME is a short `mktemp -d` dir, never the scratchpad. Stop the daemon and remove the dir after.
- **HEAD MOVES UNDER A WORKTREE ON NEARLY EVERY ITEM.** Before landing, `git log <base>..HEAD` and `git diff --stat <base> HEAD -- native/ surface/`. If code moved, stash, checkout the new HEAD, pop, **rebuild `intentd`** (the stale-sibling-daemon guard otherwise reds three edit/browse tests), and rerun.
- **READ THE CLOCK BEFORE WRITING THE STAMP, IN THE SAME COMMAND.** On 2026-09-11 I typed `10:47Z` into a script whose own `date -u` read `10:35Z`, and caught it before the commit. Pass the read value in (`NOW=$(date -u ...)`), never a literal.
