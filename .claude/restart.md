# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the prompt gate, and chains `/in-whiteboard pickup` (the board exists: `hv`, `cc`, `dc`, `ic`, `vc`). Declared languages: elixir, author, content, rust, shell. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md` + `intent/restart.md`.**

## State (as at `58397c5a`, 2026-08-19)

**TWO LIVE THREADS, BOTH IN THE 3.0.0 GATE, AND INTENT IS SELF-HOSTED ON v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2.

**ST0056 (v3.0.0 rewrite)** -- architecture in `design.md` (D01-D36). **The intentdb is the DURABLE SSOT; nothing on disk is truth.** D01 was REVERSED by hv 2026-08-15 -- "committed JSON canon as durable truth, rebuildable SQLite, `rm` always safe, no DB migrations ever" is false in every clause, **do not reason from it**. The committed extract is the INTERCHANGE that travels while the DB never leaves the machine (D34); migrations are normal; `rm intent.db` is not an operation (D36). Contract: **123 criteria / 124 tests**. WPs 01/02/04 Done, 03/05/06/10/11 WIP.

**ST0057 (disk as a sparse projection)** -- D57-1..D57-8 ruled. **Sparseness applies to VIEWS; canon is NEVER sparse**, and D29 (a gitignored path is never canon) is what makes a clone complete. Contract: **46/46**. WP-01 WIP (started BY the close-out commit, not by the pin), 02-08 Not Started.

**THE ONE FACT THAT GOVERNS TOMORROW: cc committed WP-01's CODE at `f41d6760` -- canon RESOLVES at `intent/.canon/` -- but THE 57 + 40 FILES HAVE NOT MOVED AND `intent/.canon/` DOES NOT EXIST.** The live move is next and it happens once.

**Three layers, and confusing them is the recurring error:** canon (`thread.json`, committed, never sparse) / store (`intent/.cache/intent.db`, gitignored, rebuilt) / views (`info.md`, `acceptance.md`, committed, generated). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded at the next `--to-disk`.**

**Roles (hv):** cc builds, ic runs parity/interface, dc owns DevX and distribution, vc stewards (contract, WP-close verification, hv interface; holds ST0056 + ST0057).

## Next

1. **cc**: the live move of 57 + 40 files, then the 88-binary test consolidation with dc.
2. **vc**: ping dc and ic the moment the move lands and the tree is green -- the of-N adjudication trigger (AC-00.11), **gated on the FILE move, not the code**. Then ST0011 (`completed` NULL) and AC-03.16's fix.
3. **dc**: Half B against AC-07.4 -- six declarations, two cost-bearing grep arms, the RED.
4. **ic**: `of_n_labels_its_derivation.sh` (AT-00.12, red); `of_n_closes_over_examined.sh` gated on the move.
5. **v2 carries (default-defer)**: credo_checks fleet issues; fleet pushes Utilz `0171297` + Lamplight `7058fd3a8`.

## Traps that cost real time (this cycle)

- **A LESSON WRITTEN DOWN IS NOT A MECHANISM.** `restart.md` already said a line number in a durable record expires -- and four rotted ones still shipped into a criterion, found by a peer, not by the note.
- **`sync --to-store` REWRITES THE GENERATED VIEWS.** It is not read-only on disk, so a canon edit is a two-file commit and the second file is one you never edited.
- **`at lint` and the read verbs read the STORE, not canon** -- lint straight after a canon edit reads stale. Edit canon, `--to-store`, THEN lint.
- **`intent st list` defaults to in-progress only and returns 2; `--all` is NOT a flag.** Use `st list --status all` (57: 52 Completed / 2 Cancelled / 2 WIP / 1 NotStarted) and print the breakdown, never the bare total.
- **Read the clock, then PASTE -- never read, then type.** A past-dated stamp clears the guard's future-check and its `Z`-check both.
- **A rig assembled by symlinking into the real tree is not isolated** -- `cp` follows the symlink and writes through to production.
- **`grep -c` exits 1 on zero**, so a `||` fallback fires on a true zero.
- **Never `$?` after a pipe.** Redirect to a file and read `$?` from the command itself.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks; **always `git commit --only <paths>`** (a bare commit sweeps a peer's staged index). Whiteboard stamps carry a trailing `Z` read from `date -u`. matts runs the full suite externally and is the acceptance verifier. NEVER `--no-confirm` on the release. **DO NOT PUT v3 ON PATH. DO NOT PUSH TO `upstream`** -- public and frozen. Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/int build release` date them at cut time.
