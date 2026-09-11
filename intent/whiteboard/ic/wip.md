---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-11 11:43Z
status: active
focus: "ON THE BOUNCE. #35 0223 FIXED at 6df192b7 under vc's ruling (A), tightened, and it is WITH vc for the re-drive. Next: #36 0240, only after vc closes 0223 or sends it back. The lane column of `intent/wip.md` is the authority. hv: NO NEW WORK; these items ONLY."
claims: [ST0064]
---

# Interface Claude (ic)

## DOING

**`0223` FIXED AT `6df192b7` AND WITH vc FOR THE RE-DRIVE. vc closes it, not me.** vc ruled option (A), tightened: the door opens only when a literal `--` sits BEFORE the title's last occurrence in the argv clap was handed. The reason is in clap_builder 4.6.6: `--` reaches `ArgMatches` nowhere, because `PendingArg.trailing_idx` is `pub(crate)`.

- The population is read from the built command tree, and the guarded verbs are whichever leaf declares a `title` positional (st new, issues add, wp new). The refusal runs in `spine::parse` before any verb.
- The one test, `a_bare_command_word_is_refused_as_a_title_and_passes_after_the_door` (`cli_end_to_end.rs`), is red on the unfixed tree. The loose door (a `--` anywhere) reds its trailing arm.
- intent-cli is green (543 suite, 277 lib) on the landed base. The commit is byte-identical to the tested patch, and every run was under an isolated HOME.

## TODO -- THE BOUNCE: my lane of the 3.0.1 finish line, in list order

**Source: the lane column of `intent/wip.md`. hv: _THERE IS NO NEW WORK TO BE DONE. We are working on these items and these items ONLY._ hv cuts from the bottom.** Re-drive it on return; it is vc's and it moves.

| #   | Issue  | Sev    | Defect (verbatim from `intent/wip.md`)                                           |
| --- | ------ | ------ | -------------------------------------------------------------------------------- |
| 36  | `0240` | medium | `edit` returns a path at rc=0 for an AC, AT or attachment that does not exist.   |
| 42  | `0195` | medium | `intent search` reports every hit at line 0.                                     |
| 43  | `0083` | low    | `st hydrate` reports exists for a file it just created.                          |
| 47  | `0154` | high   | No door to edit a WP body after creation (issue bodies and titles now have one). |
| 48  | `0185` | medium | No verb writes a thread's title, objective, context or body.                     |
| 49  | `0168` | medium | `ac list` never shows criterion text, and there is no `ac show`.                 |
| 50  | `0139` | medium | `at lint --fix` is advertised and refuses.                                       |
| 54  | `0140` | medium | An unsatisfied note is writable only by migration.                               |
| 66  | `0066` | medium | `_inbox/` is invisible to st show/list, ac gate, at lint, todo.                  |
| 72  | `0145` | medium | `st edit` writes on its refusal, and the remedy names an empty list.             |
| 73  | `0146` | medium | `at edit` is kind-blind and its remedy produces a row lint cannot judge.         |
| 74  | `0153` | medium | `intent edit` refuses the address form its own remedy recommends.                |
| 77  | `0176` | high   | `todo notdone`/`toggle` mutate around the Facade.                                |
| 84  | `0231` | medium | TUI repaints can tear on real terminals.                                         |

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
- **READ THE CLOCK BEFORE WRITING THE STAMP, IN THE SAME COMMAND.** On 2026-09-11 I typed `10:47Z` into a script whose own `date -u` read `10:35Z`, and caught it before the commit. Pass the read value in (`NOW=$(date -u ...)`), never a literal.
