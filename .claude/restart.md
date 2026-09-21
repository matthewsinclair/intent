# Claude Code Session Restart

This file is the entry point and holds no state: current work is `intent/wip.md`, and the rules every node works under are `intent/restart.md`. Never write a state or a count here: a number printed beside the command that regenerates it gets read instead of the command.

## Three measurements before any work

1. **Ask the other nodes** (`ListAgents`, then `SendMessage`). A board header says what its node wrote at its last fold, not what it is doing now.
2. **Bound how far the installed pair is behind the tree.** `intent --version` gives the marker; `git diff --name-only <marker>..HEAD -- native/rust surface docs/design lib/templates/llm lib/templates/prj` (`DIRT_SCOPE` in `native/rust/build-support/source_commit.rs`) lists what the pair does not carry. Empty is current, a `dirty-` marker names no commit, and an unscoped `rev-list --count` overstates. It answers for the compiled half only: scripts under `intent/plugins/` are read live from the tree, so ask the behaviour, not the version.
3. **Run the gate.** `intent st list` names the open threads and `intent ac gate <ST>` judges each one. A row held on a condition carries that condition in `intent/wip.md`'s TODO.

What waits on hv lives in `intent/whiteboard/hv/inbox.vc.md` and in the hv lines of `intent/wip.md`'s TODO. A ruling's artefact (eg `intent wp list <ST>`) outranks any document that says the ruling is unanswered.

## First actions after `/compact` or a new session

0. **In a fresh clone only, run `bin/int hooks` first.** Hooks are tracked in `.githooks/` but reached through `core.hooksPath`, which a clone does not inherit, so a fresh clone runs none of them. `bin/int hooks` reports; `bin/int hooks --install` is the only form that writes.
1. **Invoke `/in-session`.** It loads the skills, releases the prompt gate and runs whiteboard pickup. A session is solo unless it was launched as a node through `intent claude start <node>`.
2. **Read `intent/wip.md`, then `intent/restart.md`.**
