---
verblock: "27 Apr 2026:v0.1: matts - WP-16 fleet rollout aggregate summary"
wp: ST0035/WP-16
status: complete
date: 2026-04-27
total_projects: 13
absorbed_into_canary: 8
user_manual: 5
out_of_scope: 1
---

# WP-16 fleet rollout summary

## Outcome

**All in-scope fleet projects on v2.10.0 canon.** No formal canary execution this WP; the 13 projects originally enumerated in WP-16 were absorbed into WP-15 canary (8) or upgraded user-manually between sessions (5). Pplr declared out of scope. Three legacy `.intent/` directories cleaned up this session.

## Disposition by project

### 8 projects absorbed into WP-15 canary

| Project       | Canary commit (project repo) | WP-15 report                      |
| ------------- | ---------------------------- | --------------------------------- |
| Anvil         | 39c63bd                      | `canary-reports/anvil.md`         |
| arca_cli      | 2e7c14f                      | `canary-reports/arca_cli.md`      |
| arca_config   | ca85f26                      | `canary-reports/arca_config.md`   |
| arca_notionex | 9de67e9                      | `canary-reports/arca_notionex.md` |
| MicroGPTEx    | b375d1f                      | `canary-reports/microgptex.md`    |
| Molt          | 7abd972                      | `canary-reports/molt.md`          |
| Prolix        | 4508e94                      | `canary-reports/prolix.md`        |
| Utilz         | ed31017                      | `canary-reports/utilz.md`         |

All pass the 12-point verification matrix; see `intent/st/ST0035/WP/15/canary-summary.md` for the aggregate.

### 5 projects upgraded user-manually (verified this WP)

| Project                | Upgrade commit | Cleanup commit | Notes                                                             |
| ---------------------- | -------------- | -------------- | ----------------------------------------------------------------- |
| Multiplyer             | 30d584d3       | e73a84c6       | Legacy `.intent/` (stale 2.8.1) removed this session              |
| MeetZaya               | 4b115c08       | d2c8a2d7       | Legacy `.intent/` (stale 2.8.1) removed this session              |
| Molt-matts             | 418349b        | --             | Clean from upgrade; per-user Molt config repo                     |
| Courses/Agentic Coding | dbeda4d        | 8c8431a        | Legacy `.intent/` (stale 2.1.0) removed this session              |
| A3/a3-content          | b0f87c2        | --             | Clean from upgrade; multi-step chain from 2.1.0 worked end-to-end |

All five pass full verification:

```
v=2.10.0 layout=ok legacy=ok chain=ok gi.local=ok gi.bak=ok AGENTS.md=real file
```

### 1 project out of scope

| Project | Reason                                       |
| ------- | -------------------------------------------- |
| Pplr    | Does not need intent (declared out of scope) |

## Issue surfaced + resolved this WP

**Stale `.intent/` directories tracked at HEAD post-migration**: three projects (Multiplyer, MeetZaya, Courses/Agentic Coding) had their `.intent/config.json` from the pre-2.10.0 layout still tracked at HEAD with stale `intent_version` (2.8.1, 2.8.1, 2.1.0 respectively) even though `intent/.config/config.json` was correctly written by `intent upgrade`.

Root cause: the user-manual `intent upgrade` runs created the new layout but the pre-existing tracked `.intent/config.json` was not `git rm`'d during commit. This is a user-workflow gap, not a canon bug -- the migration function correctly performs the `mv .intent intent/.config` (verified mid-canary across 11 projects). When the user manually committed, only the new files were staged.

Resolution: `git rm -rf .intent/` + commit + push to `local` in each affected project. All three now pass the `[ ! -d .intent ]` check.

## Carry-forward observations

- **Multiplyer**: 1 known test failure -- `File.ls!/1` raises in `apps/multiplyer/lib/multiplyer/ta2/catalog/sources/filesystem.ex:111` against a stale fixture path (`003-missing`). Fix would be `File.ls/1` + `{:error, :enoent} -> []` pattern. User out-of-scope this rollout; flagged for future Multiplyer-side work.
- **MeetZaya**: does not compile. User out-of-scope this rollout; flagged for future MeetZaya-side work.
- **A3/a3-content**: was originally listed as "explicitly excluded" in WP-16 (content-only). Reclassified as in-scope by the user-manual upgrade. The multi-step migration chain (2.1.0 to 2.10.0 in a single pass) worked end-to-end -- a useful datapoint for the migration function's robustness across long version gaps.
- **Symlink replacement** (Arca/\* AGENTS.md): all three Arca projects show real files post-canary. The symlink-to-real-file migration that WP-16 originally called out as a distinct case happened naturally during canon application.
- **Multiplyer AGENTS.md**: present (was originally flagged as "missing" in WP-16's per-project notes). Confirmed real file post-upgrade.

## Decision

**WP-16 done.** All in-scope projects on canon. Three cleanup commits this session removed the legacy `.intent/` pollution. Proceed to WP-17 (verification sweep + dogfood journal) and WP-18 (`intent/usr/*.md` audit) in parallel.
