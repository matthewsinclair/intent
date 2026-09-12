---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 06:07Z
status: active
focus: "BATCH 1, release and install (hv ruled, vc directing). Items 2 and 3 landed; item 1 (schema faces at release) is not started. NO RELEASE, NO PUSH. NO FIGURE HERE IS EVIDENCE; RUN THE VERBS."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-0607Z.md`.** Everything executed, every narrative and every finding already carried by a commit or by `intent/wip.md` is there, not here.

## DOING -- batch 1, "release and install"

hv ruled it 2026-09-11, vc directs, code authorised for these items and nothing else. Every build and test in a private worktree under an isolated HOME with its own `CARGO_TARGET_DIR`; read `~/.intent/home` afterwards. Red before green on each item, output read. **No release and no push.** Found-whiles go to vc, never into the fix.

1. **Schema faces at release -- NOT STARTED.** The v3.0.1 tag carried `schema/*` stamped `INTENT_VER: 3.0.0`, so five schema tests fail at the tag. The release must make that impossible: the stamp step re-blesses the faces, or preflight refuses when a face's version disagrees with the version being cut. Design call mine; vc prefers the refuse-plus-regenerate shape the pipeline already uses. Proof owed: red on a fixture where VERSION is bumped and the faces are not, green on the fix, and `build release --patch --dry-run` in a clone showing the step.
2. **Keg subagents -- LANDED `597a9f26`.** `SUPPORT_PATHS` ships `intent/plugins/claude` whole, and the coverage guard reads chains that span lines and refuses a run-time-resolved directory unless the whole tree ships. Proofs in the commit message.
3. **Bootstrap after install -- LANDED `9173bbb6`.** The shim's absent-pointer remedy names `intent bootstrap`, and the generated formula carries a conditional caveat. The regenerated tap formula is committed LOCALLY at `9987a93` in `/opt/homebrew/Library/Taps/matthewsinclair/homebrew-intent` and is NOT pushed. Proofs in the commit message; `post_install` was driven and cannot reach the user's HOME.

## TODO

- Item 1 above, then report batch 1 to vc: per item, the commits, the proofs, and anything found outside the four.

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
