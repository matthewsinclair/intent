---
node: cc
name: Control Claude
role: control
session_id: b9414b4d-2a1e-42bc-98e9-9fc6b795f865
heartbeat_at: 2026-07-02T21:44Z
status: active
focus: "v2.14.0 BUILT -- ST0050 + ST0051 closed, 23/23; tree being made clean; matts runs scripts/release --minor manually, then the post-release wrap"
claims: [ST0050, ST0051]
---

# Control Claude (cc)

## DOING

**v2.14.0 homerun COMPLETE. Awaiting matts' manual `scripts/release --minor`, then the post-release wrap.** ST0050 (`intent todo`) + ST0051 (width) both closed through their own gates.

- **ST0050 CLOSED at 23/23** (`intent/st/COMPLETED/ST0050/`). Six WPs, this session's commits: `fa43630` WP-01/02 (read path + `--json` + verbs + `parse_wp_specifier` Highlander), `b68a37f` WP-06 (DONE flush/prune + `## DONE:<T>` watermark + ISO `completed:`), `6493215` WP-03/04 (curated help + harness), `a10f279` WP-05 (README/usage-rules/CHANGELOG/release notes), `c145171` close. 18/18 `intent_todo.bats`; `intent_todo` shellcheck-clean. Dogfooded: ST0050 got the first ISO `completed:` stamp.
- **ST0051 CLOSED** (`50e248f`, prior). 5/5 `output_width.bats`.
- **Full suite green (matts, external).** Confirms the `completed:` ISO change + flush logic didn't regress st/wp/gate/organize.
- **Release notes written:** `docs/releases/2.14.0/RELEASE_NOTES.md` + `intent/history/v2.14.0.md`. CHANGELOG `## [2.14.0] - 2026-07-02` in place; `scripts/release --dry-run --minor` parses clean (2.13.1 -> 2.14.0).

## TODO

1. **matts runs `bash scripts/release --minor`** (his call; NEVER `--no-confirm`). It reads `VERSION` (2.13.1 -> 2.14.0), commits `release: v2.14.0` (VERSION+CHANGELOG+AGENTS), tags, pushes both remotes, gh release -- carrying the 10 unpushed commits.
2. **Post-release wrap** (config.json + CLAUDE.md still at 2.13.1 -- `scripts/release` does NOT touch them): self-upgrade `intent upgrade --apply` (2.13.1 -> 2.14.0 bumps config.json + regens canon) OR manual bump; then commit + push the wrap + this board.

## Watch-outs

- **10 commits unpushed** (main ahead of both remotes): 2x v2.13.1 wrap (`c0eeefe`, `bb1d1f4`) + ST0050 capture (`1372035`) + ST0051 (`50e248f`) + the 6 ST0050 build/close commits. All ride the 2.14.0 release push.
- **`scripts/release` needs a CLEAN tree** -- it aborts on modified/untracked files. Board + wip + release-notes committed to satisfy this before matts runs it.
- **WP-06 flagged for matts' acceptance-verify:** the `## DONE:<T>` watermark is STICKY (not auto-daily). Reversible pre-release via `intent st start ST0050` if he wants a different model. Sent to vc's inbox + noted in `intent/wip.md`.
- **config.json / CLAUDE.md lag:** they carry 2.13.1 until the post-release wrap (known `scripts/release` v2 gap -- it bumps `VERSION`, not config.json).

## Decisions (ratified)

- (2026-07-02) hv RATIFIED D1-D4. cc RULED WP-06 verb placement (`done --flush/--prune`) + as-built (sticky watermark, `>=` inclusive membership, UTC, prune stdout/note-stderr) -- recorded in `ST0050/design.md`, flagged to matts for acceptance-verify.
- (2026-07-02) ST0050 + ST0051 INDEPENDENT; both ship in 2.14.0 (a minor -- new command surface).
- (earlier) v2.13.1 SHIPPED. Detail in `intent/wip.md` + git history.
