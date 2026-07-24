# Claude Code Session Restart -- narrative state

## Current state (2026-07-24)

**v2.17.3 SHIPPED.** Patch closing the last vacuous-pass hole in the acceptance close-gate, fixed + closed as an issue (no ST -- standing hv fix-under-issue ruling from 0002/0003):

- **Issue 0004** -- `intent ac gate` reported a pass for a target it never evaluated. Surfaced by hv in Lamplight closing ST0290/WP-04: the gate returned exit 0 with no output, which is also what a genuine pass looks like. An unresolvable target degraded to an EMPTY acceptance-criterion set, and each command in the family reported its own flavour of vacuous success over it -- the gate found nothing unsatisfied and exited 0, `ac status` printed `0/0`, `ac list` printed no rows. "Does not exist" and "has nothing unsatisfied" were the same internal state, and only the second was ever reported. Fix: resolution is now a distinct, FAILABLE step ahead of evaluation, in one resolver (`resolve_target`) the whole `ac`/`at` family shares; it validates the `/NN` segment, which nothing validated anywhere before, and reports a bad target as BLOCKED + exit 1 from the gate, `Error:` + exit 1 from the readers. `acc_path` retired; new `resolve_wp_dir` in `intent_helpers` (WP analogue of `resolve_st_dir`), shared by the three resolving `intent_wp` sites. Every verdict is now announced, PASS included -- silence-on-success was the camouflage that hid this for three releases. The ST0044 WP-lenient rollup survives but is granted only to a WP that EXISTS. Companion: `parse_wp_specifier` fed `/NN` to a bare `10#` expansion, so `wp show|start|done <st>/abc` died with raw bash noise; guarded once in the shared helper.

Tag `v2.17.3` (`2828f89`), wrap `5793d7d`, both remotes + GitHub release. CHANGELOG `[2.17.3]` is the release note (patch precedent). Prior: v2.17.2 (issues 0002 + 0003), v2.17.1 + v2.17.0 (ST0055 `intent issues`), v2.16.1 (ST0054), v2.16.0 (ST0053).

## Open follow-ups (non-blocking)

- **hv ruling owed -- issue 0004 item 4:** hv asked for uniform non-zero exit on a BLOCKED `ac status`; the premise does not reproduce (exits 0 on both BLOCKED shapes; `intent_acceptance_cli.bats:111` asserts it). Design as it stands: `status` = human-facing reporter (verdict on stdout), `gate` = machine-facing gate (verdict in `$?`). Changing it is a deliberate behaviour change plus a test rewrite -- own issue if wanted.
- Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits in their own repos (Conflab pushed).
- Utilz-side todo guard (separate repo): `generator: utilz todo` + symmetric guard.
- AT-name traceability (vc deferral); `bin/release` v2 polish (auto config.json bump); headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Where detail lives

- `.claude/restart.md` -- next-session focus. `intent/wip.md` -- current state + backlog.
- `intent/done.md` -- shipped ledger (July; older months in `intent/history/YYYYMM-done.md`). `intent/st/COMPLETED/ST0055/` -- closed thread. `docs/releases/2.17.0/` + CHANGELOG `[2.17.0]`/`[2.17.1]`/`[2.17.2]`/`[2.17.3]`; issues 0002+0003+0004 in `intent/issues/CLOSED/`.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full suite externally (single-file bats fine); matts is the acceptance verifier; never `bin/release --no-confirm`.
