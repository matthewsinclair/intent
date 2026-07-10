---
node: cc
name: Control Claude
role: control
session_id: 7ba41ae6-fc6b-4845-9c8e-af601bbf64ac
heartbeat_at: 2026-07-10T16:15Z
status: active
focus: "v2.17.0 SHIPPED (2026-07-10): `intent issues` command (ST0055) + pipe-sanitize fix + scripts/release->bin/release move. Tag b7e94e2 + wrap 20c1b5f, both remotes + GitHub release, doctor green, tree clean. REMAINING: WP-05 fleet normalise -- Utilz+Conflab done (uncommitted in their own repos, hv to commit there), Lamplight DEFERRED (needs hv decision). ST0055 stays WIP until Lamplight (AC-05.1)."
claims: [ST0055]
---

# Control Claude (cc)

## DOING

**2026-07-10 -- v2.17.0 SHIPPED. `intent issues` (ST0055) live.** Tag `b7e94e2` (release) + `20c1b5f` (post-tag wrap: config.json + CLAUDE.md -> 2.17.0), both remotes + GitHub release, doctor green, tree clean. hv ran `bin/release --minor` (full suite green in pre-flight). Contents: `bin/intent_issues` (dir-per-issue tracker, 5 verbs + `new` alias, `--severity`, `--json`) + companion pipe-`|`-in-title fix (`sanitize_title` at st/wp/issues input, `slugify` promoted to helpers) + `scripts/release` -> `bin/release` fleet-align. 22 new bats + no regressions; critic-shell clean (2 passes). Dogfood: Intent issue 0001 (the pipe bug) filed + closed via the new command. `docs/releases/2.17.0/RELEASE_NOTES.md` + CHANGELOG `[2.17.0]`.

**Open (post-ship):**

- **WP-05 fleet normalise:** Utilz + Conflab already normalised to canon (dir-per-issue, `_templ` dropped) but the changes sit UNCOMMITTED in their own repos -- hv commits there. Lamplight DEFERRED -- messy (multi-`.md` per issue collides with `issue_file`'s first-match; statuses PENDING/RESOLVED/DONE) -- needs hv decision + likely an `issue_file` primary-file robustness tweak. ST0055 stays WIP until this closes (AC-05.1).

## TODO

- **ST0055 WP-05 (cross-repo, the only thing left to close ST0055):** Lamplight normalisation -- needs hv decision (multi-`.md`-per-issue primary-file convention + PENDING/RESOLVED/DONE status pass) + likely an `issue_file` robustness tweak. Utilz + Conflab already done (uncommitted in their repos, hv to commit).
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard (v2.16.1 C4 follow-up; handoff note delivered).
- DEFERRED (needs hv ruling): AT-name traceability -- machine-check `acceptance.md` AT ids against real bats `@test` names.

## Watch-outs

- New-command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent` (project command, needs project context) -- no dispatch edit unless it is global or needs noun re-injection. Register in MODULES.md FIRST (Highlander), single template source under `lib/templates/`.
- `bin/release` does tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it.
- ST0046 ("add modules properly to the intent cli") was moved to `intent/st/NOT-STARTED/` by hv in commit af9b02e (pre-flight). Possibly related to ST0055 scope -- confirm with hv.

## Decisions

- (2026-07-10) ST0055 claimed by cc; no peer overlap (hv/vc claims empty at pickup).
- (2026-07-09) v2.16.1 SHIPPED: release commit d2ddb96 + post-tag wrap 18bf8cc, both remotes + GitHub release, doctor green. ST0054 (usage-rules v1.x alignment) + 4 companion chores (C1-C4).
- (2026-07-09) localfold vs globalfold (hv, authoritative): localfold = per-workstream tidy before a compact; globalfold = project-wide tidy before EOD when all workstreams close.
- (2026-07-07) hv RATIFIED ST0053 D1-D5 [shipped v2.16.0].
