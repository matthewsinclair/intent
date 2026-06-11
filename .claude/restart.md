# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the UserPromptSubmit gate, chains `/in-whiteboard pickup` if `intent/whiteboard/` exists.
2. **Verify the working tree.** Expect either: (a) clean tree, v2.11.12 tagged and pushed (release done), or (b) clean tree, nothing pushed, release still pending user's `bash scripts/release --patch`.
3. **Read `intent/restart.md`.** It carries the full arc ledger and the next-arc pointer.

## State (2026-06-11, arc complete -- release pending)

ST0042 (Fable 5 review + all nine WPs) and ST0041 (MFIC exploration + harvest) are **Completed** and relocated to `intent/st/COMPLETED/`. Full suite green (user-verified). Nothing pushed. The user runs `bash scripts/release --patch` (v2.11.11 -> v2.11.12; interactive confirm; NEVER `--no-confirm`). After the release: refresh both restart files + wip.md Current State + memory Active Work.

Done-work bookkeeping moved this session: terse ledger in `intent/done.md`, verbose release narratives in `intent/history/v2.11.*.md`, wip.md slimmed to current state + pruned backlog.

## Next arc (after release): ST0043 -- Rethink `intent upgrade`

WIP, not started, targets **v2.12.0 minor**, own session. Design in `intent/st/ST0043/info.md` (Architecture B: convergent end-state + structural-step ledger). Owns all upgrade-subsystem deletions deliberately excluded from ST0042's WP-05/WP-06, plus the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP operations. NEVER manually wrap markdown. NO Claude attribution in commits. No vanity metrics. Fail-forward. User runs the full test suite externally -- pause and ask; single-file bats runs are fine. Refresh BOTH restart files on every release. Compact at ~200-250k tokens.
