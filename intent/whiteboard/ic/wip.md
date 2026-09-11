---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-11 09:35Z
status: active
focus: "0192 IN DOING, ALREADY DELIVERED AT 4d296181 (2026-09-08) AND RE-DRIVEN AT HEAD; WITH vc TO CLOSE. 0207 STARTS ONLY AFTER vc CLOSES 0192 OR SENDS IT BACK. THE LANE: the CLI-surface lane of the 3.0.1 finish line in `intent/wip.md`, ONE ITEM AT A TIME, in list order -- 0192, 0207, 0209, then 0079, 0149, 0291, 0103, 0137, 0194, 0299. hv: THERE IS NO NEW WORK; these items and these items ONLY."
claims: [ST0064]
---

# Interface Claude (ic)

## DOING

**AGGRESSIVE LOCALFOLD 2026-09-11 09:18Z, ON vc's SIGNAL (hv's instruction), ON EXECUTION.** The whole pre-fold board is verbatim at `.history/20260911/wip-prefold-aggressive-0916Z.md` -- **committed and confirmed TRACKED at `09326bb3` BEFORE this line cited it.** The previous fold's archive is `.history/20260911/wip-prefold-0914Z.md`. **Cut:** every watch-out's worked text (129 entries, all in the archive), the Decisions section (every entry is ruled and executed or superseded by the 2026-09-11 cull), and the fold notes. **Kept:** the two holds, vc's cull text, untouched.

**`0192` -- CLAIMED 2026-09-11 09:35Z on vc's go. NO CODE CHANGE: the fix is already at `4d296181` (2026-09-08, an ancestor of HEAD), and the issue reading `open` is W129 again.** Re-driven on a private build of `36352b21` (the last build-input commit at HEAD `c5113fc3`), in a throwaway `HOME`/project, baseline synced first:

- The issue's own reproduction now refuses: `>>`-appended text after the banner gives `sync --to-store` rc=1, naming "text after the trailing banner (added by hand)". The file keeps the text and the canon does not.
- Control: an `## Objective` edit is carried (rc=0, into the canon and back out on render), so the estate could exhibit the carry.
- Control: whitespace only after the banner is still rc=0.
- Control: the same section BEFORE the banner still refuses (rc=1).
- The proving test, `failure_text_after_the_trailing_banner_refuses`, goes RED with the guard mutated to `if false` and GREEN restored (`info_round_trip`, 21 passed).
- **With vc to rebuild, re-drive and close. `0207` waits for that.**

## TODO -- THE BOUNCE: the CLI-surface lane of the 3.0.1 finish line

**Source: `intent/wip.md`, the numbered list. hv's words: _THERE IS NO NEW WORK TO BE DONE. We are working on these items and these items ONLY._ hv cuts from the bottom.** Re-drive the list on return -- it is vc's and it moves.

| Order | #   | Issue  | Defect                                                                   |
| ----- | --- | ------ | ------------------------------------------------------------------------ |
| 1     | 3   | `0192` | `st edit` hands back a generated view; the next render destroys the edit |
| 2     | 4   | `0207` | `at green/red/na --note` replaces the whole note, destroying history     |
| 3     | 5   | `0209` | `st start` writes a smaller copy of an unhydrated thread over the path   |
| 4     | 23  | `0079` | `st new` says created and writes no files                                |
| 5     | 24  | `0149` | `intent edit` ignores the kind it was given and answers about a thread   |
| 6     | 25  | `0291` | `edit wp ST/NN --path` returns the parent thread's file                  |
| 7     | 26  | `0103` | `wp list` returns zero rows for 71 threads whose WPs are in the store    |
| 8     | 27  | `0137` | `ac list` shows a fiat-closed criterion as satisfied: no                 |
| 9     | 31  | `0194` | `intent search` with a hyphenated term leaks a SQLite error              |
| 10    | 34  | `0299` | `at lint` says a file lacks an id it carries (the cross-thread case)     |

**THE RULES ON THE BOUNCE (vc, under hv):**

- Claim the id in DOING, **one item at a time**.
- Commit with the id in the subject, tell vc; **vc closes the issue after re-driving the fix.** Never close it myself.
- **A defect found while fixing goes in the commit message, not on the list.**
- **No new tests beyond the ONE that proves the item fixed. No new instruments, guards, criteria or threads.**
- `AC-01.7` is hv's decision 7 (credentials), not a list item.

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
