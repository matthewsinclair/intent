# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the prompt gate, and chains `/in-whiteboard pickup` (the board exists: `hv`, `cc`, `dc`, `ic`, `vc`). Declared languages: elixir, author, content, rust, shell. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md` + `intent/restart.md`.**

## State (as at `5716b43a`, 2026-08-19)

**THE DISK MODEL SHIPPED TONIGHT. `intent organize --apply` removed 423 files at `e7f00e65`** and `intent/st/` now holds `ST0046`, `ST0056`, `ST0057` and `steel_threads.md`. Fifty-two completed and two cancelled threads live only in the database. **This is not a bug and not a loss** -- ST0001 put back on the list returned five files byte-identical to git, a fence-heavy pair returned fifteen byte-identical, and all 282 attachments verify against their own `sha256`.

**THE ARCHITECTURE hv RULED, replacing the two-region manifest design:**

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

**Many writers, no recomputation.** `st new` adds an id, `st done` removes it, a human may edit it. **Nothing derives it from status.** **ABSENT IS NOT EMPTY** -- a missing manifest keeps everything, a manifest declaring nothing keeps nothing.

**Intent is SELF-HOSTED on v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2.

**ST0056 (v3.0.0 rewrite)** -- **the intentdb is the DURABLE SSOT; nothing on disk is truth.** D01 was REVERSED by hv 2026-08-15; the old wording is false in every clause, **do not reason from it**. Contract 131/132, 55 satisfied. WPs 01/02/04 Done, 03/05/06/10/11 WIP.

**ST0057 (disk as a sparse projection)** -- **sparseness applies to VIEWS; canon is NEVER sparse.** Contract 45 (1 withdrawn) / 46, 33 satisfied. WPs 01/02/04/05/07/08 WIP, 03/06/09 Not Started.

**THE GATE: 50 OF 64** -- all of ST0057's rows plus all 18 of ST0056 WP-03. It was 33 this morning.

**Three layers, and confusing them is the recurring error:** canon (`intent/.canon/st/<ID>.json`, committed, never sparse) / store (`intent/.cache/intent.db`, gitignored, the durable SSOT) / views (`info.md`, `acceptance.md`, committed, generated). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded.**

**THE PRECONDITION BLOCK IS 14 AND READS 0 UNMET, AND THAT IS NOT A SCOREBOARD.** hv cleared four on one word and AC-00.3 on the git ruling. **Only AC-00.3 is withdrawn; AC-03.6, AC-06.3, AC-06.4 and AC-07.5 are still owed.** dc's distinction: the question was never whether the work is wanted, it is whether a gate should hold on it.

**Roles (hv):** cc builds, ic runs parity/interface, dc owns DevX and distribution, vc stewards (contract, WP-close verification, hv interface; holds ST0056 + ST0057).

## Next

1. **Everyone, hv's own question: 250 files under `intent/` are not in the store at all** -- `docs/`, `llm/`, `history/`, `eng/`, `plugins/`, and the project-level `done.md` / `wip.md` / `restart.md` / `todo.md`. The store holds threads, WPs and issues, nothing else. hv: _not all of that should be in the db, but certainly some of it should._ **cc's angle: which of the 250 could an artefact even OWN**, since the manifest names artefacts and never files.
2. **dc**: AC-06.3, AC-06.4, and the hosting sweep -- 16 of 32 families dispatch, `intent claude` implements 1 of 8, against 230 call sites in this repo's own machinery.
3. **ic**: `st hydrate`'s render arm (two lines now `address::promote` landed); the `st edit` fork, unruled; the `issues dehydrate` bucket ruling that understates by four.
4. **cc**: AC-03.6, and wiring `intent doctor`'s view-skew detection into the gate -- the detection exists, only the wiring is missing.
5. **vc**: ST0057/WP-09 -- the event log records the MODEL and not the DISK. 423 files left this estate and the log recorded nothing.

## Traps that cost real time (this cycle)

- **THE REVISION IS PART OF THE FINDING, NOT CONTEXT FOR IT.** In a four-node checkout a measurement can be true of a tree one rebuild or one mid-write file out of date. Name revision, clock and dirty count on every measurement.
- **A PEER'S REPORT OF A PEER'S FILE IS ONE HOP TOO MANY -- and your own terminal output is zero hops.** vc carried "`Report.pruned` is not rendered" onto a board in the same hour its own `organize --apply` had printed `1 pruned`, a number already read and quoted.
- **COULD THIS MEASUREMENT HAVE COME BACK THE OTHER WAY?** "ST0056's rows agree so the generator is fine" said nothing about bytes; `doctor` showed the file stale by 1813.
- **A COUNT OF CONTAINERS REPORTED AS A COUNT OF CONTENTS.** `1 refused` stood for 423 files; `is in 0 buckets` understated by four. Twice in one night, two instruments.
- **`sync --to-store` REWRITES THE GENERATED VIEWS.** A canon edit is a two-file commit and the second file is one you never edited.
- **`at lint` and the read verbs read the STORE, not canon.** Edit canon, `--to-store`, THEN lint.
- **`intent st list` defaults to in-progress only and returns 2; `--all` is NOT a flag.** Use `st list --status all`.
- **Read the clock, then PASTE -- never read, then type.**
- **Never `$?` after a pipe.** `grep` here is ugrep. **The Bash tool's shell is zsh**, so an unquoted glob in `--include=*.rs` is a hard error, and PATH can collapse inside a `while read` loop -- that one reported three fake sha256 mismatches with `jq: command not found` in the same output.
- **`grep -c` exits 1 on zero**, so a `||` fallback fires on a true zero.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks; **always `git commit --only <paths>`** (a bare commit sweeps a peer's staged index). Whiteboard stamps carry a trailing `Z` read from `date -u`. matts runs the full suite externally and is the acceptance verifier. NEVER `--no-confirm` on the release. **DO NOT PUT v3 ON PATH** -- the pre-commit gate works because it runs v2, whose version guard is scoped to writes; on PATH, `intent critic` answers 2 in all five languages, which is the code the gate fails open on. **DO NOT PUSH TO `upstream`** -- public and frozen. Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/int build release` date them at cut time.
