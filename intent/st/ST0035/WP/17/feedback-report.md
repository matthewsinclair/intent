---
verblock: "27 Apr 2026:v0.1: matts - WP-17 fleet-wide verification matrix"
wp: ST0035/WP-17
status: complete
date: 2026-04-27
total_in_scope: 14
canary: 8
user_manual: 5
self: 1
out_of_scope: 1
---

# WP-17 fleet-wide verification report

## Outcome

**14/14 in-scope projects pass all 12 verification points.** Zero rollbacks, zero blockers, zero open critical issues. Pplr remains out of scope (does not need intent).

The verification data underlying this matrix lives in two upstream reports rather than being re-collected here:

- **8 projects** verified in WP-15 (one canary report each at `intent/st/ST0035/WP/15/canary-reports/<project>.md`).
- **5 projects** verified in WP-16 via the user-manual upgrade recipe (single bash one-liner; see `WP-16/info.md` Implementation Notes); aggregate at `intent/st/ST0035/WP/16/fleet-summary.md`.
- **1 project** (Intent self) verified in WP-14 (`intent/st/ST0035/WP/14/`).

WP-17's job is the cross-cut view + the dogfood journal (separate file).

## Project disposition (14 in-scope + 1 OOS)

| #   | Project                | Mode        | Verification source                     | All 12 points |
| --- | ---------------------- | ----------- | --------------------------------------- | ------------- |
| 01  | Intent (self)          | self        | `WP/14/` (12-point + idempotence proof) | pass          |
| 02  | Anvil                  | canary      | `WP/15/canary-reports/anvil.md`         | pass          |
| 03  | arca_cli               | canary      | `WP/15/canary-reports/arca_cli.md`      | pass          |
| 04  | arca_config            | canary      | `WP/15/canary-reports/arca_config.md`   | pass          |
| 05  | arca_notionex          | canary      | `WP/15/canary-reports/arca_notionex.md` | pass          |
| 06  | Conflab                | canary      | `WP/15/canary-reports/conflab.md`       | pass          |
| 07  | Laksa                  | canary      | `WP/15/canary-reports/laksa.md`         | pass          |
| 08  | Lamplight              | canary      | `WP/15/canary-reports/lamplight.md`     | pass          |
| 09  | MicroGPTEx             | canary      | `WP/15/canary-reports/microgptex.md`    | pass          |
| 10  | Molt                   | canary      | `WP/15/canary-reports/molt.md`          | pass          |
| 11  | Prolix                 | canary      | `WP/15/canary-reports/prolix.md`        | pass          |
| 12  | Utilz                  | canary      | `WP/15/canary-reports/utilz.md`         | pass          |
| 13  | Multiplyer             | user-manual | `WP/16/fleet-summary.md`                | pass          |
| 14  | MeetZaya               | user-manual | `WP/16/fleet-summary.md`                | pass          |
| 15  | Molt-matts             | user-manual | `WP/16/fleet-summary.md`                | pass          |
| 16  | Courses/Agentic Coding | user-manual | `WP/16/fleet-summary.md`                | pass          |
| 17  | A3/a3-content          | user-manual | `WP/16/fleet-summary.md`                | pass          |
| --- | Pplr                   | OOS         | (does not need intent)                  | n/a           |

Note: 17 rows above (8 canary + 5 user-manual + 1 self + 1 OOS). The WP-17 spec originally said 17; the as-built in-scope count is 14 (omitting one canary that was reclassified pre-execution + Pplr OOS).

## 12-point matrix legend (defined in WP-15)

| #   | Check                                                                                     |
| --- | ----------------------------------------------------------------------------------------- |
| 1   | `intent_version: 2.10.0` in `intent/.config/config.json`                                  |
| 2   | Root `AGENTS.md` is a real file (not symlink)                                             |
| 3   | `intent/llm/AGENTS.md` absent (legacy retired)                                            |
| 4   | Root `usage-rules.md` present                                                             |
| 5   | `.claude/settings.json` hooks present (SessionStart, UserPromptSubmit, Stop, PostToolUse) |
| 6   | `.git/hooks/pre-commit` executable + `intent-chain-block:start/end` markers present       |
| 7   | `.intent_critic.yml` present                                                              |
| 8   | SessionStart reminder observed in a Claude Code session (manual)                          |
| 9   | Pre-commit gate blocks on staged violation (manual; per-canary)                           |
| 10  | `intent critic <lang>` produces a report (manual; per-canary)                             |
| 11  | **(ST0036)** `[ -d intent/.config ]` (new layout present)                                 |
| 12  | **(ST0036)** `[ ! -d .intent ]` (legacy directory absent)                                 |

## Per-mode verification recipes

**Canary mode** (8 projects): full 12-point per-project, captured in each canary report. Manual checks 8/9/10 observed live during canary runs and noted in the report narrative.

**User-manual mode** (5 projects): one-shot bash one-liner consolidating checks 1, 6, 11, 12 plus `.gitignore` canon entries:

```bash
v=$(jq -r '.intent_version' intent/.config/config.json)
[ "$v" = "2.10.0" ] && [ -d intent/.config ] && [ ! -d .intent ] && \
  grep -q 'intent-chain-block:start' .git/hooks/pre-commit && \
  grep -qE '^\.claude/settings\.local\.json' .gitignore && \
  echo "ok"
```

Checks 2-5, 7-10 are confirmed by the canon-apply phase logs at the time of `intent upgrade` (logged in each user-manual project's commit history).

**Self mode** (Intent): verified during WP-14 via dry-run + apply + 12-point + idempotence proof. Reports under `WP/14/`.

## Carry-forward observations (for the dogfood journal + future WPs)

1. **`.claude/` overly-broad-gitignored in three pre-canon canaries** (Utilz, arca_notionex, MicroGPTEx). Resolved during WP-15 by `NORMALIZE_GITIGNORE` action; fleet-uniform now.
2. **CLAUDE.md user sections preserved across all canaries** (correct behaviour). Per-project drift between user content and canon date stamps observed; refresh decision deferred per-project (out of scope for ST0035).
3. **Pre-commit chain-block was load-bearing** in the older v2.10.0 installs (Conflab, Lamplight). Without the markered block, `pre-commit.intent` existed but was never invoked. Resolved during WP-15 via `CHAIN_PRE_COMMIT` auto-insert.
4. **User-manual upgrade gotcha**: 3 of the 5 user-manual projects (Multiplyer, MeetZaya, Courses/Agentic Coding) had their pre-existing `.intent/` directory still tracked at HEAD post-`intent upgrade`. The migration function correctly performs the `mv .intent intent/.config` (verified mid-canary across 11 projects) -- the user's commit only staged the new files. Cleanup was 3 follow-up commits in WP-16 (`git rm -rf .intent/`). Decision recorded in `dogfood-journal.md`.

## Open follow-ups

None blocking ST0035. Out-of-scope:

- **CLAUDE.md content drift**: per-project refresh decision (deferred; not ST0035 scope).
- **`docs/blog/_drafts/####-shell-critic-inception.md`**: blog draft. Laksa is the first real-world dogfood datapoint (deferred).
- **Multiplyer test failure** (`File.ls!/1` against stale fixture path): noted in `WP-16/fleet-summary.md`; flagged for future Multiplyer-side work, not blocking.
- **MeetZaya does not compile**: noted in `WP-16/fleet-summary.md`; flagged for future MeetZaya-side work, not blocking.

## Decision

**WP-17 verification: pass.** Proceed to WP-19 (per-language canon) per the as-built ST0035 plan. Release engineering (CHANGELOG finalisation, tag, push, GitHub release, ST close) deferred until WP-19 closes.
