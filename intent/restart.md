# Claude Code Session Restart — narrative state

## Current state (2026-04-25, end of session — 12 of 18 WPs Done; WP-11 closed; ST0036 Phase 0 stub waiting elaboration)

**Intent v2.10.0 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) active, WP-11 closed. ST0036 (Directory relocation) Phase 0 stub awaiting forensic WP elaboration before its WP01 can start.**

### ST0035 shape

- **Done (12)**: WP01, WP02, WP03, WP04, WP05, WP06, WP07, WP08, WP09, WP10, WP11, WP12.
- **Not Started (6)**: WP13, WP14, WP15, WP16, WP17, WP18.

Critical path remaining: `WP13 → ST0036/WP01–08 → WP14 → WP15 → WP16 → WP17`. WP13 (S) and ST0036 Phase 0 elaboration can run in parallel — neither blocks the other. WP17 has a second gating input (WP18 — user-doc audit).

### ST0036 shape (new)

Phase 0 stub only at `intent/st/NOT-STARTED/ST0036/`:

- `info.md` — objective, why bundled with ST0035, scope, success criteria, Phase 0 gate.
- `design.md` — provisional canon decisions D1–D5 (new path `intent/.config/`, atomic fail-forward migration, whole-tree preservation, shared rollout with ST0035, CHANGELOG + migration guide), risk register, open questions.
- `tasks.md` — 9 provisional WPs with T-shirt sizing (WP01 migration function, WP02 path probes, WP03 literal sweep, WP04 templates, WP05 BATS, WP06 gitignore, WP07 migration guide, WP08 Intent self-apply, WP09 merge with ST0035 fleet rollout).

Forensic `WP/NN/info.md` elaboration deferred until ST0036 is actively picked up (projected after ST0035/WP13 lands). Phase 0 review gate before any ST0036 WP01 start.

### Progress this session (Sessions 2 + 3 of WP-11)

- **WP-11 Session 2 shipped (commit `1db2b44`)**: edge cases + UX polish.
  - `--force` flag: overwrite user-edited canon files (CLAUDE.md without the Intent footer marker, `.intent_critic.yml`, `usage-rules.md`). Banner warning at top; per-probe `OVERWRITE` marker; `canon_refresh_with_user_section` short-circuits to clean install when set.
  - `--skip-settings` flag: skip `.claude/settings.json` + 3 hook scripts in both Phase 1 probe and Phase 3 execute. Single escape hatch for projects with deeply customised settings.
  - `CHAIN_PRE_COMMIT` execute now prints a paste-ready multi-line snippet (with `git rev-parse --git-path hooks` so the chain works in worktrees + submodules) instead of a one-line hint. Dry-run advertises the snippet.
  - `REFRESH_CLAUDE_MD` pretty-print now shows a unified diff (current vs refreshed-with-user-section-preserved) in dry-run; capped at 60 lines.
  - Worktree / submodule support: replaced `[ -d "$PROJECT_DIR/.git" ]` probe with `git rev-parse --git-path hooks` via new `canon_hooks_dir` helper. Resolves to the shared hooks dir for worktrees, the right module hooks for submodules.
  - Read-only FS handling: writability probe upfront on `--apply`, bails with a clear diagnostic before attempting any work.
  - Helper extraction: `canon_compute_refresh_preview` replaces inline awk in the CLAUDE.md probe; reused by `canon_refresh_with_user_section` and the new dry-run diff.
- **Inline bug fix folded into Session 2**: `canon_compute_refresh_preview` originally used a shell variable to capture preserved user content. Command substitution strips trailing newlines, so a user section ending with a blank line round-tripped one line shorter — bogus DIVERGED on every re-run after fresh install. Fixed by staging preserved content in a temp file (not a variable), reading line-by-line via awk `getline`. Caught during the Session 2 scratch verification.
- **Inline bug fix shipped separately (commit `614980d`)**: `intent init /abs/path` used to crash sed with "bad flag in substitute command: 't'" because the absolute path leaked into PROJECT_NAME, then sed parsed the `/t...` bytes as flags. Fixed by treating path-like arguments (containing `/`, or literal `.` / `..`) as target directories: cd there (creating if needed) and use basename as project name.
- **WP-11 Session 3 shipped + Done (commit `b2a6e5d`)**: verification + close.
  - BATS suite at `tests/unit/intent_claude_upgrade.bats` covers all 5 spec scenarios (fresh install, idempotent re-apply via tree-snapshot diff, user-edit preservation across REFRESH while non-user drift gets reverted, pre-existing non-Intent hook preserved + chained, `--dry-run` produces no filesystem mutation). Tests isolate `HOME` so installed subagents/skills don't bleed into the upgrade probes. 5/5 green first run.
  - MODULES.md row for the upgrade entry rewritten to capture WP-11 scope: canon-install helpers, `--apply`/`--dry-run`/`--force`/`--skip-settings`. The `canon_*` helpers live inside `intent_claude_upgrade` rather than as a separate module.
  - `canon_print` helper unifies the diagnostic format. Status column now lands at col 43 for every canon-artefact line regardless of label length; the previous mix of hand-aligned static lines (col 31) and dynamically-padded script lines (col 42) is gone.
  - `intent wp done ST0035/11` flips status WIP → Done.

Verification: full BATS suite **767/767 green** (5 new from WP-11 file). `intent doctor`: clean on Intent + scratch.

### Lessons worth keeping (this session)

- **Command substitution silently strips trailing newlines.** Bash's `$()` chops trailing `\n`, so `var="$(extract_lines_from_file)"` loses any blank lines at the end. For exact round-trip preservation, stage in a temp file and read line-by-line. This bug only manifested as bogus DIVERGED reports after fresh install; idempotence tests in BATS would have caught it but Session 1 was timeboxed before the BATS suite landed. Rule: when bytes need to round-trip exactly, files beat variables.
- **Don't paper over upstream bugs to keep the test green.** Hit a sed crash in `intent init /abs/path` while setting up the scratch project. Initial reflex: "work around by `cd` first, then `intent init .`". User pushback ("we always leave the place better than we find it") was correct — the bug was ~15 LOC away from a real fix, and absorbing it would mean every future test author trips the same trap. Fix-first beats workaround-first when the cost is bounded.
- **Scratch + BATS catch different bugs.** Scratch testing exposed both bugs above (intent init crash + blank-line stripping); BATS would not have, because the `make_scratch_project` helper would `cd` first by convention, hiding the path issue. Conversely, BATS catches behaviour drift (e.g. "snippet output regression") that scratch testing only catches if you happen to look. Run both, in that order: scratch first to fix mechanics cheaply, BATS second to lock in the contract.
- **Helper extraction is cheaper than copy-paste even at N=2.** `canon_compute_refresh_preview` was inlined in two places (Phase 1 drift probe, `canon_refresh_with_user_section`). Extracting it not only fixed both call sites at once when the blank-line bug surfaced, but made the new dry-run diff a one-liner. Highlander Rule applies inside files too, not just across them.

## Resume target (WP13 + ST0036 Phase 0)

WP-11 closed. WP13 and ST0036 Phase 0 are the next two parallelisable threads — neither blocks the other.

**WP13** (S — Update Intent's own `CLAUDE.md` to reference the canon):

- Spec: `intent/st/ST0035/WP/13/info.md`.
- Depends on WP09 ✓ (template) and WP03 ✓ (`intent/docs/working-with-llms.md`).
- Smallest unit; ships first to unblock WP14 (self-dogfood).

**ST0036 Phase 0 elaboration** (the gate before any ST0036 implementation WP starts):

- Stub at `intent/st/NOT-STARTED/ST0036/`. `info.md`, `design.md` (D1–D5 + risk register), `tasks.md` (9 provisional WPs) all populated.
- Forensic `WP/NN/info.md` elaboration deferred — needs to land before ST0036/WP01.
- Phase 0 review gate (per Intent convention): user reviews + approves all 9 WPs before any code lands.

After both: WP14 (Intent self-dogfood) runs `intent claude upgrade --apply` on Intent itself, carrying both ST0035 canon AND ST0036 directory relocation in one pass. Then canary (WP15: Conflab → Lamplight → Laksa), fleet (WP16: 12 Intent + Pplr), verification sweep (WP17), and WP18 (`intent/usr/*.md` audit, in parallel with WP15/WP16).

## Rollout universe (17 projects, unchanged)

- 15 downstream Intent projects: Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab, Courses/Agentic Coding, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz.
- Intent (self) — dogfooded in WP14.
- Pplr — non-Intent today; `intent init` first, then canon apply.

Excluded: Sites (inside Laksa), llm-tropes (content-only), A3/\* (content-only).

Canary order (WP15): Conflab → Lamplight → Laksa. Fleet (WP16) highest-delta first (Multiplyer, Arca trio), ends with Pplr.

## Resolved decisions (all 5, retargeted #1)

1. Version: **2.10.0** (retargeted from 2.9.1 mid-ST to bundle ST0036).
2. Hook enforcement: strict `UserPromptSubmit` gate blocking first prompt until `/in-session` runs. Reassess post-rollout.
3. Pre-commit critic threshold: CRITICAL + WARNING blocks (default; tunable per-project via `.intent_critic.yml severity_min`).
4. PostToolUse advisory critic: off by default. Opt-in via `.intent_critic.yml post_tool_use_advisory: true` + manual stanza add to `.claude/settings.local.json`.
5. Cancelled STs go to `intent/st/CANCELLED/`; deprecation annotation inline.

## Session conventions (unchanged)

- T-shirt sizing only.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (gotcha: `ST0035` or `35`, not `0035` — leading zero is parsed as octal).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- Fail-forward: no backwards-compat shims.
- Document first, code next, hard review gate.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.

## Recent commits (chronological)

- `b2a6e5d` — WP-11 Session 3 + Done: BATS suite, MODULES audit, alignment polish.
- `1db2b44` — WP-11 Session 2: flags, paste-ready chain snippet, dry-run diff, edge cases.
- `614980d` — fix: `intent init` accepts absolute path argument (sed crash on `/abs/path`).
- `989451a` — session wrap for previous compact (WP-11 Session 1 shipped).
- `e999f82` — WP-11 Session 1: canon-install helpers + extended upgrade.
- `052ba9d` — session wrap for prior compact (11 of 18).
- `2e99857` — mark ST0035/WP-10 Done.
- `1ae5f61` — WP-10: delete deprecated artefacts + flip residual code paths.
- `f4c68b9` — ST0036 Phase 0 stub.
- `b760b39` — retarget ST0035 v2.9.1 → v2.10.0.
