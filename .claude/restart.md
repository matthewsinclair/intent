# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the UserPromptSubmit gate, chains `/in-whiteboard pickup` if `intent/whiteboard/` exists.
2. **Verify the working tree.** Expect: clean tree, `v2.11.12` tagged and pushed to both remotes, post-release wrap committed.
3. **Read `intent/restart.md`.** It carries the narrative state and the next-arc pointer.

## State (2026-06-11, v2.11.12 shipped)

ST0042 (Fable 5 review + all nine WPs) and ST0041 (MFIC exploration + harvest) shipped in v2.11.12 (tag `574b015`, GitHub release published). Both STs Completed in `intent/st/COMPLETED/`. Done-work bookkeeping: terse ledger `intent/done.md`, verbose narratives `intent/history/v2.11.*.md`. Fleet picks up v2.11.12 on next `intent upgrade`.

## Next arc: ST0043 -- Rethink `intent upgrade`

WIP, not started, targets **v2.12.0 minor**, own session. Design in `intent/st/ST0043/info.md` (Architecture B: convergent end-state + structural-step ledger). Owns all upgrade-subsystem deletions deliberately excluded from ST0042's WP-05/WP-06, plus the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP operations. NEVER manually wrap markdown. NO Claude attribution in commits. No vanity metrics. Fail-forward. User runs the full test suite externally -- pause and ask; single-file bats runs are fine. Refresh BOTH restart files on every release. Compact at ~200-250k tokens.
