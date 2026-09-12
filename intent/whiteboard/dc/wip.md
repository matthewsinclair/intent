---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 06:19Z
status: active
focus: "BATCH 1 IS LANDED, all three items. hv named v3.0.2. What is left: the end-to-end --dry-run rehearsal in a clone, with the gates running. NO RELEASE, NO PUSH. NO FIGURE HERE IS EVIDENCE; RUN THE VERBS."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-0607Z.md`.** Everything executed, every narrative and every finding already carried by a commit or by `intent/wip.md` is there, not here.

## DOING -- batch 1, "release and install": LANDED

hv ruled it 2026-09-11, vc directs, code authorised for these items only. Builds and tests in a private worktree or clone under an isolated HOME with its own `CARGO_TARGET_DIR`; read `~/.intent/home` after. Red before green, output read. **No release and no push.** Found-whiles to vc.

1. **Schema faces at release -- LANDED `2f90fd28`.** Pre-flight refuses a tree whose faces disagree with its own VERSION; after the stamp, a `schema faces` step regenerates them through `INTENT_BLESS=1 cargo test -p intentsvcs schema_faces_drift`, reads the files back, and refuses unless every one carries the target; `schema` joins the release commit's paths. Red and green driven in clones, `--dry-run --skip-tests`.
2. **Keg subagents -- LANDED `597a9f26`.** `SUPPORT_PATHS` ships `intent/plugins/claude` whole; the coverage guard reads chains that span lines and refuses a run-time-resolved directory unless the whole tree ships.
3. **Bootstrap after install -- LANDED `9173bbb6`.** The shim's absent-pointer remedy names `intent bootstrap`; the generated formula carries a conditional caveat (post_install was driven and cannot reach the user's HOME). The regenerated tap formula is LOCAL at `9987a93` and unpushed.

**CHANGELOG (mine): `b9a8deb5` and `2f90fd28`.** The entry is `## [3.0.2] - in progress` -- hv named the version; the release dates the heading at cut time and aborts on `## [Unreleased]`. It carries a Fixed line per landed batch-1 item. cc sends a line per batch-2 and batch-3 item as each lands; a fixed defect's entry in `docs/known-defects.md` says fixed in 3.0.2 rather than being deleted.

## TODO

- **The release rehearsal, and it is the last thing owed on batch 1:** an end-to-end `bin/devbin build release --patch --dry-run` in a clone of HEAD with the gates RUNNING (no `--skip-tests`), which is where the bless step executes rather than being logged. Report every gate it printed to vc. If pre-flight's doctor refuses on the view-banner skew (cut item 1), report the exact line and stop -- that one is hv's.
- **Then batch 1's report to vc:** per item, the commits, the proofs, and anything found outside the four.
- **CHANGELOG lines from cc** for batches 2 and 3 as they land, and the matching `docs/known-defects.md` edits.

## Holds

- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **D42: a clock value goes into a board or a message only from a `date -u` read in the same turn's output, pasted.** `git log`, `stat` and `ls -la` print LOCAL; appending `Z` is an assertion.
- **The delivered pair names the last commit touching what it COMPILES IN, not HEAD.** That difference is not staleness: `self_provenance_check.sh`'s currency line decides it.
- **An identity comparand takes the MARKER's scope; only the dirt check gets the wider containment scope.** Fixed at `bc696da6`; the same shape is what to check first in any new build guard.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call.** Never touch a peer's `index.lock`; on a lock refusal re-issue the SAME command.
- **A commit guard that names canon means the file is a thread attachment**: `intent st attach <ST> <thread-relative path> --from <file>` first, then commit the file and its canon together.
- **Every suite and build from a private worktree's own sources under an isolated HOME**, never a test binary built here.
- **`git worktree add` fails when a stale registration for that path exists, and the failure is easy to miss**: `git worktree prune` first, and read the "applied" line before trusting where the tests ran (2026-09-12: a failed `cd` ran a suite in the shared tree).
- **The Bash tool's shell is zsh: unquoted `$var` does NOT word-split** (a path list in a variable reached `git add` as one argument tonight), an unmatched glob aborts the command, and a backtick in a double-quoted commit message runs as a command -- messages go in a file, through `-F`.
- **Homebrew's `post_install` cannot write the user's HOME** (sandbox temp HOME, EPERM; driven). `~/.intent/home` is written by `intent bootstrap` alone, and bootstrap REPOINTS an existing pointer.

## Decisions

- **devbin `0047` (hv, 2026-09-01): option 3, the split.** `fullcycle`'s clean phase forces only the blocked-binaries arm; `_clean_confirm`'s removal prompt stays. Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.
