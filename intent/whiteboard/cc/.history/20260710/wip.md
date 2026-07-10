# cc archive -- 2026-07-10 (v2.17.0 + v2.17.1: intent issues, ST0055)

## DOING (archived)

**2026-07-10 -- v2.17.0 SHIPPED. `intent issues` (ST0055) live.** Tag `b7e94e2` (release) + `20c1b5f` (post-tag wrap: config.json + CLAUDE.md -> 2.17.0), both remotes + GitHub release, doctor green, tree clean. hv ran `bin/release --minor` (full suite green in pre-flight). Contents: `bin/intent_issues` (dir-per-issue tracker, 5 verbs + `new` alias, `--severity`, `--json`) + companion pipe-`|`-in-title fix (`sanitize_title` at st/wp/issues input, `slugify` promoted to helpers) + `scripts/release` -> `bin/release` fleet-align. 22 new bats + no regressions; critic-shell clean (2 passes). Dogfood: Intent issue 0001 (the pipe bug) filed + closed via the new command. `docs/releases/2.17.0/RELEASE_NOTES.md` + CHANGELOG `[2.17.0]`.

**2026-07-10 -- v2.17.1 SHIPPED (patch). ST0055 CLOSED (gate 23/23 PASS).** Fixed `issue_file` to prefer the frontmatter-bearing primary in multi-`.md` issue dirs (Lamplight `0003-resolved.md` sorted before its primary) + regression test. WP-05 fleet normalise COMPLETE: Utilz (`0171297`), Lamplight (`7058fd3a8`) committed local in their repos (hv pushes); Conflab already done (`49428b4f`); Intent = issue 0001. hv ruled 0001/0002 CLOSED. RELEASE (crash-recovered): hv ran `bin/release --patch` but VSCode terminal crashed after the local commit+tag, before push/gh/wrap. cc finished it: pushed `e7360b8` (release) + tag v2.17.1 both remotes, created GitHub release, post-tag wrap `309d8d8` (config.json + CLAUDE.md -> 2.17.1).

## Decisions (archived)

- (2026-07-10) ST0055 claimed by cc; no peer overlap (hv/vc claims empty at pickup). Closed at gate 23/23 PASS; moved to COMPLETED.
- (2026-07-10) hv ruled Lamplight issues 0001/0002 CLOSED; issue_file to prefer frontmatter-bearing primary (shipped 2.17.1).
- (2026-07-09) v2.16.1 SHIPPED (d2ddb96 + wrap 18bf8cc): ST0054 + 4 companion chores.
- (2026-07-09) localfold vs globalfold (hv, authoritative): localfold = per-workstream tidy before a compact; globalfold = project-wide tidy before EOD.
- (2026-07-07) hv RATIFIED ST0053 D1-D5 [shipped v2.16.0].
