---
verblock: "27 Apr 2026:v0.1: matts - WP-17 dogfood journal for ST0035 + ST0036"
wp: ST0035/WP-17
status: complete
date: 2026-04-27
---

# WP-17 dogfood journal

Synthesised from session restart files, the WP-15 canary aggregate, the WP-16 fleet summary, the WP-14 self-apply report, and the Intent commit log. Chronological where useful, thematic where the chronology obscures the lesson.

## What the rollout looked like end-to-end

ST0035 + ST0036 shipped together as v2.10.0. Two parallel work streams converged: ST0035 specified the canonical LLM config (three-file canon, hooks, pre-commit critic gate, `.intent_critic.yml`, `bin/intent_critic`, `lib/templates/.claude/`, `intent/docs/working-with-llms.md`) and ST0036 specified the breaking directory move (`.intent/` -> `intent/.config/`) with a single migration function (`migrate_v2_9_0_to_v2_10_0`) doing both phases in one `intent upgrade` invocation.

The dogfood arc ran:

1. **WP-08** (ST0036 self-apply): Intent's own repo went through the migration first. Surfaced that the WP-05 diagnostic `mv .intent intent/.config` had skipped Phase 3 entirely (no canon-apply); fixed by reverting + running `intent upgrade` properly.
2. **WP-14** (ST0035 self-dogfood): full 12-point on Intent + idempotence proof (MD5 sanity). Surfaced four canon-installer rough edges (AGENTS.md footer oscillation, RULES.md count rendering, REVIEW warnings firing on customised content, chain-block detection). All fixed in `9315bb6`.
3. **WP-15** canary: started with 3 named projects (Conflab, Lamplight, Laksa); expanded mid-execution to 11 in-scope as canon-installer matured. Surfaced three new actions (`MIGRATE_LEGACY_PRE_COMMIT`, `CHAIN_PRE_COMMIT` auto-insert, `NORMALIZE_GITIGNORE`). Each action came with new BATS scenarios.
4. **WP-16** fleet: original plan had 13 enumerated projects; as built, 8 absorbed into WP-15 canary mid-execution, 5 user-manually upgraded between sessions, Pplr OOS. Surfaced the leftover-`.intent/` gotcha in 3 of 5 user-manual projects.

## What broke (with specifics)

- **WP-05 manual `mv` skipped Phase 3.** The diagnostic rename Intent's working tree was healthy enough to pass tests, but it created a state HEAD couldn't reach via the migration function. Fix: revert to HEAD, re-run `intent upgrade`. **Lesson**: never apply migrations by hand; the function is also responsible for things that aren't visible in `git status` (canon-apply runs).
- **AGENTS.md footer oscillation.** Generator and prettier disagreed on whether to emit a blank line after `---`; every other commit cycle would re-flip. Fix in `intent_agents:506-509`: emit a blank line after `---`. Permanent fix; oscillation gone fleet-wide.
- **RULES.md count produced `(0\n0 rules/sections)`.** `grep -c '^(### |[0-9]+\.)'` was using BRE not ERE; the alternation didn't match, the `|| echo 0` fallback duplicated the count. Fix in `intent_claude_upgrade:488`: ERE + drop the fallback.
- **REVIEW warnings fired on populated RULES.md / ARCHITECTURE.md.** The check was "file exists" not "file is verbatim \_default". Fix: `cmp -s` against the template; warning now silent for projects that did the work.
- **Chain block detection was structural, not semantic.** Old check was "`pre-commit.intent` exists" -> CHAINED; that lied when `pre-commit` didn't actually invoke `pre-commit.intent`. Fix: marker pair (`intent-chain-block:start/end`) as the source of truth; new state `CHAIN_REQUIRED` for the in-between case; new action `CHAIN_PRE_COMMIT_BLOCK` to auto-insert.
- **Legacy single-file pre-commit installs were unrecognised.** Pre-chain-architecture installs had canon body at `pre-commit` with no `pre-commit.intent`. Surfaced when the Anvil canary tried to add a chain stub on top of the canon body. Fix: `MIGRATE_LEGACY_PRE_COMMIT` action -- detect verbatim canon at `pre-commit`, mv to `pre-commit.intent`, write fresh chain stub.
- **Three projects gitignored entire `.claude/`.** Utilz, arca_notionex, MicroGPTEx had blanket `.claude/` patterns that swallowed `settings.json` + `scripts/`. Fresh clones got nothing of the canon. Fix: `NORMALIZE_GITIGNORE` action -- track `.claude/{settings.json,scripts/*,restart.md}`, ignore only `.claude/settings.local.json` + `/AGENTS.md.bak`.
- **Leftover `.intent/` after user-manual upgrade.** 3 of 5 user-manual projects had `.intent/config.json` still tracked at HEAD post-`intent upgrade` -- the migration function correctly mv'd the directory but the user's commit only staged the new files. Resolved with 3 cleanup commits (`git rm -rf .intent/`).

## What was rough

- **`intent claude upgrade --dry-run` said "No `intent/.config/config.json` found".** True but unhelpful in the dry-run flow (which runs before the relocation phase). Could be reworded to say "expected; relocation happens during apply". Not a blocker; minor UX polish for v2.10.x.
- **WP-17 spec said "10-point matrix".** Was overtaken when ST0036 added 2 more checks. WP-15/WP-16 specs were quietly revised; WP-17 wasn't (caught at WP-17 closure, hence the closure-pattern tidy applied here). **Lesson**: when a verification matrix grows, sweep all WPs that reference it, not just the new WP.
- **WP-15 `--dry-run`/`--apply` references in spec.** `intent upgrade` doesn't accept `--dry-run` or `--apply`; the flags were copied from `intent claude upgrade`. Spec drifted from CLI surface; caught and tidied in WP-15 closure.
- **WP-15 named "3 canaries" but executed 11.** Spec was right at the time; scope expanded organically as canaries kept passing. Closure pattern (tidy spec to as-built + write summary) absorbed the drift cleanly.
- **WP-16 originally had a 13-project enumeration.** Most of those got absorbed into WP-15 canary mid-execution; a few were user-manually upgraded between sessions. WP-16's scope-as-built note + `fleet-summary.md` reconciled.
- **`/in-start` was getting forgotten.** SessionStart hook nudged `/in-session` only; `/in-start` (orientation-only) needed to be remembered separately. Folded the orientation step into `/in-session` so a single command handles orient + skills + gate. `/in-start` retained as the rare orientation-only standalone case.
- **`~/.claude` repo had 1.6GB of session transcripts as untracked.** No `.gitignore` covered `projects/`. Caused git operations to be slow + noisy. Fix: expand `~/.claude/.gitignore` to cover runtime caches, IDE state, backups; commit + push the curated agents/skills/config.

## What surprised

- **Long migration chains "just worked".** A3/a3-content went 2.1.0 -> 2.10.0 in a single `intent upgrade` (multi-step chain through the migration function table). Robust across long version gaps -- a useful datapoint for the migration function design.
- **`mix usage_rules.sync` and Intent's root `usage-rules.md` coexist cleanly.** Lamplight runs both; they don't collide. The hand-authored Intent root file lives at `usage-rules.md`; the synced deps content goes elsewhere by default.
- **CLAUDE.md user sections preserved across every canary.** The marker-pair-based "user content" preservation was load-bearing and just worked -- not a single canary lost user content despite the canon refreshing the surrounding template prose.
- **Symlink-to-real-file migration happened naturally during canon application.** Three Arca projects had `AGENTS.md` as a symlink pre-canon; canon application replaced with real files. WP-16 originally called this out as a distinct case; turned out to be free.
- **Multi-Claude-Code-session contention surfaced the per-project sentinel design.** The `UserPromptSubmit` strict gate originally used a single shared sentinel file; running two Claude Code sessions in different Intent projects stomped each other's session_id. Per-project-keyed sentinel (`/tmp/intent-claude-session-current-id-${project_key}`) solved it; legacy fallback retained for sessions that pre-date the per-project hook.

## Decision: should `intent upgrade` warn / auto-stage leftover `.intent/`?

**Question** (from WP-17 carry-forward): the user-manual upgrade gotcha caught 3 of 5 projects this rollout. Should `intent upgrade` detect a leftover `.intent/` post-migration and either warn or auto-stage the `git rm`?

**Decision: warn on next `intent doctor` invocation, do not auto-stage.**

Rationale:

- **Auto-stage is too aggressive.** `git rm` modifies the user's index. The user might want to verify or hand-stage; auto-staging during `intent upgrade` (or worse, during a passive `intent doctor`) violates the principle that Intent commands operate on Intent state, not the user's git index.
- **Warning at `intent doctor` matches the existing posture.** `intent doctor` already checks for "interrupted migration" (sentinel file). Adding "leftover `.intent/` post-migration" as another check is structurally identical -- the user runs `intent doctor` voluntarily and gets actionable output.
- **`intent upgrade` already prints "relocated: .intent/ -> intent/.config/"** during the migration. The leftover only appears in the user's working tree if the pre-migration state had `.intent/config.json` tracked at HEAD. A doctor check that says "I see a tracked `.intent/` directory but you're at v2.10.0 layout. Run `git rm -rf .intent/ && git commit` to clean up" is the right granularity.
- **Filed as v2.10.x follow-up**: add a check to `intent doctor` (`bin/intent_doctor`) that fails with a hint when both `intent/.config/config.json` and `.intent/config.json` exist. Estimated S; not in v2.10.0 scope.

## Lessons for future STs

- **WP closure pattern** (tidy spec to as-built + write summary + `wp done`) applied cleanly to WP-15, WP-16, WP-17, WP-18. Reinforce in template guidance somewhere; the pattern is recurring across STs and worth codifying.
- **Three-phase migration functions** (relocate + stamp + canon-apply) are the right shape for breaking changes. Each phase is independently testable; the chain is transparent in `intent upgrade` output.
- **Marker-pair-based content preservation** (chain block; CLAUDE.md user section) outperforms structural detection by a wide margin. Use markers anywhere a canon-installer needs to identify "the bit it owns".
- **Canary discipline (one-at-a-time with verification before fleet sweep)** caught three rough edges that would have been expensive to surface fleet-wide. Cheap because each canary is small and isolated; expensive if surfaced after fleet rollout.
- **Per-project state files in `/tmp`** outperform shared state files when multiple Claude Code sessions can run concurrently. Hash the project root path into the file name; legacy fallback for sessions that pre-date the per-project file.
- **When two skills are usually invoked together, fold the common case** (kept the rare-case standalone separate). Reduces "I forgot to run X" failures.
- **Personal config repos need `.gitignore` discipline early.** Runtime droppings (session transcripts, caches, IDE state) accumulate fast; fix before they swamp the repo.

## Suggestions for v2.10.x / v2.11.0 follow-ups

- `intent doctor` warning for leftover `.intent/` post-migration (decision above).
- `intent claude upgrade --dry-run` reword the "expected during dry-run" cases (`intent/.config/config.json` not found pre-relocation).
- Per-language canon (`intent lang init` + `intent init --lang`) -- in scope as ST0035/WP-19; closes ST0035.
- Optional: `intent upgrade --auto-cleanup` flag that does `git rm -rf .intent/` + stages the deletion if and only if `.intent/` is tracked at HEAD AND `intent/.config/config.json` exists. Off by default.
- Optional: Per-project CLAUDE.md drift report (canon date stamp vs file mtime) -- helps users decide when to refresh.
