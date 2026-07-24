# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Whiteboard present (`intent/whiteboard/`, hv+cc+vc) -- `/in-session` chains `/in-whiteboard pickup`. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.17.3 SHIPPED

**v2.17.3 SHIPPED (2026-07-24).** Patch closing the last vacuous-pass hole in the acceptance close-gate; fixed + closed as issue **0004** (no ST -- standing hv fix-under-issue ruling). `intent ac gate` reported a pass for a target it never evaluated: an unresolvable ST/WP degraded to an empty AC set, so the gate found nothing unsatisfied and exited 0 in silence -- indistinguishable from a verified contract, which is why it survived three releases. Fix: resolution is a distinct FAILABLE step ahead of evaluation, in one resolver (`resolve_target`) the whole `ac`/`at` family shares; the `/NN` segment is validated (nothing did before); `acc_path` retired; new `resolve_wp_dir` in `intent_helpers` shared by the three resolving `intent_wp` sites; every verdict announced, PASS included; the ST0044 WP-lenient rollup granted only to a WP that EXISTS. Companion: `parse_wp_specifier`'s bare `10#` expansion made `wp show|start|done <st>/abc` die with raw bash noise -- guarded once in the shared helper. Tag `v2.17.3` (`2828f89`), wrap `5793d7d`, both remotes + GitHub release. Detail: `intent/done.md`, CHANGELOG `[2.17.3]`, `intent/issues/CLOSED/0004/`. Prior: v2.17.2 (0002+0003), v2.17.1 + v2.17.0 (ST0055), v2.16.1 (ST0054), v2.16.0 (ST0053).

## Open follow-ups (non-blocking)

- **hv ruling owed -- issue 0004 item 4:** hv asked for uniform non-zero exit on a BLOCKED `ac status`; premise does not reproduce (exits 0 on both BLOCKED shapes; `intent_acceptance_cli.bats:111` asserts it). `status` = reporter (stdout), `gate` = gate (`$?`). Own issue if hv wants the change.
- Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits in their own repos (Conflab pushed).
- Utilz-side todo guard (separate repo): `generator: utilz todo` + symmetric guard.
- AT-name traceability (vc deferral); `bin/release` v2 polish (auto config.json bump); headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Backlog

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007; Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; ST0040/ST0041 deferred.

## Fleet

Members pick up v2.17.3 on next `intent upgrade`. Excludes Pplr, Sites-in-Laksa, llm-tropes. NOTE: 2.17.3 TIGHTENS a gate -- automation calling `intent ac gate` on an id that does not resolve was getting a silent pass and now gets BLOCKED + exit 1. That is the fix working, not a regression, but it is the one way this patch can surface as a new failure on a member. `st done` / `wp done` are unaffected (their own existence checks fire first).

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `bin/release --no-confirm`.
