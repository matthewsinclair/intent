# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Whiteboard present (`intent/whiteboard/`, hv+cc+vc) -- `/in-session` chains `/in-whiteboard pickup`. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.17.2 SHIPPED

**v2.17.2 SHIPPED (2026-07-13).** Patch fixing two dogfooded CLI bugs, closed as issues (no ST -- hv ruled fix-under-issue). **0002**: `intent todo` rendered `[?]` for a non-canonical status -- `canonical_status` relocated `bin/intent_st` -> `bin/intent_helpers` (both `intent st` + `intent todo` source it); `status_box` canonicalises first. **0003**: the pre-commit critic gate errored + fail-opened on declared `author`/`content` -- one language registry in `critic_runner.sh`; `intent critic` no-ops prose at exit 0 + `--languages`; the gate defers to the exit code (a gate-side skip was built + reverted). Tag `v2.17.2` (`22c409e`), wrap `e525f04`, both remotes + GitHub release. Detail: `intent/done.md`, CHANGELOG `[2.17.2]`, `intent/issues/CLOSED/{0002,0003}/`. Prior: v2.17.1 + v2.17.0 (ST0055), v2.16.1 (ST0054), v2.16.0 (ST0053).

## Open follow-ups (non-blocking)

- Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits in their own repos (Conflab pushed).
- Utilz-side todo guard (separate repo): `generator: utilz todo` + symmetric guard.
- AT-name traceability (vc deferral); `bin/release` v2 polish (auto config.json bump); headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Backlog

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007; Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; ST0040/ST0041 deferred.

## Fleet

Members pick up v2.17.2 on next `intent upgrade` (issues 0002 + 0003 fixes: intent todo canonical-status + critic prose-language handling, additive). Excludes Pplr, Sites-in-Laksa, llm-tropes.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `bin/release --no-confirm`.
