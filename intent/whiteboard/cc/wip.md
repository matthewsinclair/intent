---
node: cc
name: Control Claude
role: control
session_id: 7ba41ae6-fc6b-4845-9c8e-af601bbf64ac
heartbeat_at: 2026-07-10T16:15Z
status: active
focus: "ST0055 (intent issues command) BUILT + critic-clean. Dir-per-issue tracker (5 verbs + new alias, --severity, --json) + companion pipe-sanitize fix. 22 new bats green, no regressions, critic-shell clean (2 passes), doctor green. Acceptance 20/22, BLOCKED only on AC-04.1 (matts full suite) + AC-05.1 (post-ship fleet normalise). Awaiting hv: full suite + verify + release decision. v2.16.1 shipped 2026-07-09."
claims: [ST0055]
---

# Control Claude (cc)

## DOING

**2026-07-10 -- ST0055: `intent issues` SHIPPED-READY as v2.17.0 (minor). Release staged, awaiting hv `bin/release --minor`.** Command built + critic-clean (2 passes, 0 crit/0 warn) + gate 22/22-satisfiable (hv confirmed full suite passes). AS-BUILT: `bin/intent_issues` (thin coordinator, 5 verbs + `new` alias, `--severity`, `--json`), dir-per-issue (Lamplight model), `lib/templates/issues/_ISSUE.md` (Intent-owned), MODULES + `intent help`. Helpers: `slugify` promoted (Highlander) + `sanitize_title`. Companion chore: pipe-`|`-in-title fix. Tests 22 new green, no regression, doctor green. RELEASE PREP: `docs/releases/2.17.0/RELEASE_NOTES.md` + CHANGELOG `[2.17.0]` written. Dogfood: Intent issue 0001 (the pipe bug) filed + closed. WP-05 fleet: Utilz + Conflab normalised (uncommitted in their own repos, hv commits); Lamplight DEFERRED post-ship (messy: multi-.md per issue + PENDING/RESOLVED/DONE statuses -- needs hv decision + likely a primary-file robustness tweak to `issue_file`).

## TODO

- **ST0055 WP-04 gate:** apply critic-shell findings, flip acceptance.md ATs to green, `intent ac gate ST0055`, hand to hv (acceptance verifier) for the full suite + release.
- **ST0055 WP-05 (POST-SHIP, cross-repo):** normalise intent/issues in Lamplight, Conflab, Utilz, Intent to the dir-per-issue canon (RESOLVED->CLOSED, drop vendored _templ).
- First dogfood issue to FILE via the new command once shipped: the `|`-in-title table-corruption bug (now fixed by the companion chore -- file it as a CLOSED issue for the record).
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
