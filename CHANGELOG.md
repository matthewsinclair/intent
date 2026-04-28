# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.11.2] - 2026-04-28

Second hotfix following v2.11.0/v2.11.1.

### Fixed

- **`intent upgrade` failed with `Error: Unknown version: 2.11.0`** when applied to a project already at v2.11.x. The dispatcher in `bin/intent_upgrade` had cases for every released source version up to v2.10.1 but none for v2.11.0; an in-flight project at v2.11.0 attempting to upgrade to v2.11.1 fell into the `*) error "Unknown version: $VERSION"` arm. Added a `"2.11.*"` case that runs the idempotent v2.11.x migration (no-op on field-already-present and stamp-already-current); a glob match rather than a literal so future patches in the v2.11 line don't need a fresh case each time.

## [2.11.1] - 2026-04-28

CI hotfix following v2.11.0.

### Fixed

- **Pre-commit hook errors on `set -u` + empty `LANGS` array** under some bash versions (CI macOS runner). v2.11.0 introduced the empty-array path (a project with `languages: []` declares zero critics), but the existing `for lang in "${LANGS[@]}"; do` loop expansion erred as "unbound variable" before the loop body ran. Length-guarded the loop with an explicit `if [ "${#LANGS[@]}" -gt 0 ]; then` check. Local installs need to re-run `intent claude upgrade --apply` to pick up the corrected hook template; new installs from v2.11.1 onward get the fix automatically.

## [2.11.0] - 2026-04-28

ST0037 ships: languages-in-use becomes an explicit per-project configuration field, replacing four sites of filesystem-marker probing. The probe-based detection was a regression against design intent (filesystem presence is unreliable evidence; a vendored example or a one-off script can flip the wrong switch). Schema change is automatic via migration; existing fleet projects need no user action beyond `intent upgrade`.

### Added

- **`languages: []`** field in `intent/.config/config.json`. Array of canonical language names (`elixir`, `rust`, `swift`, `lua`, `shell`). Array order is the explicit declaration; the first entry is the primary where a primary is needed. Empty array is a valid state.
- **`intent lang remove <lang> [<lang> ...]`** -- new verb. Reverses `intent lang init`: removes the entry from the agnostic `RULES.md` Language Packs marker block, deletes `intent/llm/RULES-<lang>.md` and `intent/llm/ARCHITECTURE-<lang>.md`, removes the language from `intent/.config/config.json`. Idempotent: a never-installed language emits `noop:` and returns 0.
- **`get_project_languages()`** helper in `bin/intent_helpers`. Reads the field via `jq`, returns one language per line, returns 0 lines when the array is empty or the field is absent. Used by the pre-commit critic gate.
- **`add_project_language()`** / **`remove_project_language()`** helpers in `bin/intent_helpers`. Atomic config-field mutation via tempfile + `mv`.
- **`migrate_v2_10_x_to_v2_11_0`** in `bin/intent_helpers`. Adds the `languages` field with back-fill from existing `intent/llm/RULES-<lang>.md` presence (alphabetical for determinism). When the back-fill set is empty AND a pre-commit hook is installed, falls back to `["shell"]` to preserve current "shell-always" gate behaviour. Idempotent: if the field is already present, only stamps the version.
- **BATS coverage** for the migration (`tests/unit/migrate_v2_10_x_to_v2_11_0.bats`), the new `intent lang init` config writes and the new `intent lang remove` verb (`tests/unit/intent_lang.bats`), the config-driven critic dispatch (`tests/unit/critic_dispatch.bats`), the config-driven pre-commit gate (`tests/unit/pre_commit_hook.bats`), and the regression guards on `/in-session` SKILL.md (no filesystem probes, no phantom skill refs).

### Changed

- **`/in-session` SKILL.md** -- detection table replaced with config-driven flow. The skill reads `(.languages // []) | .[]` from `intent/.config/config.json` and invokes any matching essentials skill. Currently only `/in-elixir-essentials` (and `/in-elixir-testing`) are real per-language essentials skills; rust/swift/lua/shell coding rules ship via the rule library at `intent/plugins/claude/rules/<lang>/` plus the `critic-<lang>` subagent applied on demand.
- **`/in-review` SKILL.md** -- stage-2 dispatch table replaced with config-driven flow. Reads the `languages` array, dispatches one critic per language listed.
- **`/in-tca-audit` SKILL.md** -- critic-selection table replaced with the same config-driven dispatch.
- **`lib/templates/hooks/pre-commit.sh`** -- the `LANGS+=(elixir)` etc. probe block is gone. The hook reads `languages` from config; an empty array means no language critics run, mirroring the explicit-config contract.
- **`bin/intent_init`** -- fresh projects get `"languages": []` in their initial config. The existing `--lang <lang>` flag still seeds the array via `intent lang init`.
- **`intent/docs/working-with-llms.md`** -- "Skills and /in-session auto-load" section rewritten to describe the config-driven flow; the four phantom skill references (`/in-rust-essentials`, `/in-swift-essentials`, `/in-lua-essentials`, `/in-shell-essentials`) are gone.

### Removed

- **Filesystem-probe-based language detection** at four canon sites (`in-session/SKILL.md`, `in-review/SKILL.md`, `in-tca-audit/SKILL.md`, `lib/templates/hooks/pre-commit.sh`). File presence is no longer treated as evidence of language-in-use.
- **Phantom skill references** to `/in-rust-essentials`, `/in-swift-essentials`, `/in-lua-essentials`, `/in-shell-essentials`. Those skills were promised in WP06/WP12 ("ships in WPNN") in the v2.10.x SKILL.md but never authored. The rule-pack + critic-subagent path is the working mechanism for those four languages and the prose now reflects that.

### Fixed

- **`create_v2_directory_structure()` in `bin/intent_helpers`** -- previously created an empty top-level `.intent/` unconditionally during `intent upgrade`, even on projects already on the v2.10 layout (`intent/.config/` present). The next phase (`migrate_v2_9_0_to_v2_10_0`) then refused to proceed because both `.intent/` and `intent/.config/` existed. Skips the `.intent/` creation when `intent/.config/` is in place. Pre-existing latent bug, surfaced when chaining the v2.9.0 → v2.10.0 → v2.11.0 migration sequence.
- **critic-elixir false positives on canonical OTP/Mix idioms (ST0038)**. Three rules misfired in the headless pre-commit gate against correct code (Lamplight ST0163/WP-01 commit attempt):
  - **`IN-EX-TEST-002` (no-process-sleep)** fired on `Process.sleep(:infinity)` in a `Mix.Task.run/1` body in `lib/`. The rule's frontmatter declared `applies_to: ["test/**/*_test.exs"]` but the runner ignored the field. Fixed by adding `applies_to` honoring in `critic_apply_rule` (`intent/plugins/claude/lib/critic_runner.sh`); globs are matched with suffix anchoring so umbrella layouts (`apps/<app>/lib/...`, `apps/<app>/test/...`) resolve correctly.
  - **`IN-EX-CODE-002` (tagged-tuple-returns)** fired on every public `def name(args) do` because the greppable proxy was too coarse to express "fallible function returns bare nil/false" as a per-file regex. Greppable proxy stripped; the rule remains active for the LLM-driven `critic-elixir` subagent via `/in-review`, where the body and call sites can be read.
  - **`IN-EX-CODE-006` (module-highlander)** fired on every public `def name(...)` for the same reason -- and the rule's actual concern (cross-module duplication) is fundamentally not a per-file scan. Greppable proxy stripped; subagent-applied via `/in-review`.
  - New BATS coverage in `tests/unit/critic_runner_applies_to.bats` (15 tests) verifies glob-to-regex translation, umbrella-layout matching, the absence of greppable proxies on the two stripped rules, and the presence of the proxy on `IN-EX-TEST-002`. `tests/unit/pre_commit_hook.bats` updated to stage fixtures under `test/<name>_test.exs` so they match `IN-EX-TEST-001`'s `applies_to`.

### Migration notes

- `intent upgrade` from any v2.10.x project runs `migrate_v2_10_x_to_v2_11_0` automatically. The migration is atomic, idempotent, and cannot lose user data.
- Polyglot projects: declare languages in primacy order with `intent lang init <primary> <secondary> ...`. The first entry is the primary; later entries follow. To change primacy, `intent lang remove <lang>` and re-init in the desired order.

## [2.10.1] - 2026-04-28

v2.10.x polish line. Two new pieces of maintainer infrastructure (a release script and a `intent doctor` migration-leftover warning), the gate-firing fix that surfaced post-v2.10.0 dogfood, and three pre-existing v2.10.0 dogfood-journal follow-ups that needed closing.

### Added

- **`scripts/release`** -- maintainer release orchestrator. Single-invocation cut: pre-flight (clean tree, doctor, tests, gh auth), version bump (`--patch / --minor / --major / vX.Y.Z`), CHANGELOG date finalisation, sidecar sync (VERSION + AGENTS.md), commit, idempotent tag, push to both remotes (local + upstream), GitHub release publication. Modelled on Conflab's release pattern, pared back to Intent's surface (single repo, two remotes, no native binary, no Homebrew tap). `--dry-run` previews every step with no side effects.
- **`intent doctor` check 4d** -- warning (not error) when a stale top-level `.intent/` directory remains after a v2.9 -> v2.10 migration. Auto-staging is intentionally NOT done: the user runs `git rm -rf .intent/` themselves so the cleanup is visible in the commit.

### Fixed

- **`/in-session` UserPromptSubmit gate-firing loop**. The SKILL.md inlined an awk pipeline whose positional-field expansion was silently emptied by Claude Code's skill renderer, producing a malformed project_key that prevented the per-session sentinel from being written. The waterfall is now in `intent/plugins/claude/skills/in-session/scripts/release-gate.sh`, invoked by the SKILL by path; the renderer never sees the pipeline.
- **`intent claude upgrade --dry-run` UX** -- distinguish three states for the `config.json` pre-flight (canonical, legacy `.intent/`, absent) so a pre-relocation project no longer reports its expected-missing config as a hard problem. The legacy-location case now points the user at `intent upgrade` for the relocation step.
- **`IN-RS-CODE-005` (lifetime-elision-first) carve-out** -- explicit "Does Not Apply" entry for teaching examples in `intent/plugins/claude/rules/**` and `tests/fixtures/critics/rust/**`. Closes the false-positive that fired on a clean fixture during ST0034 WP07 verification.

### Changed

- **Diogenes test-spec handoff** -- the four critic agent.md files (`critic-{elixir,rust,swift,lua}`) now uniformly suppress the Diogenes RECOMMENDATION for targets under `tests/fixtures/critics/`. Critic self-test fixtures are not real test code; the handoff was firing inconsistently across critics post-WP07.

## [2.10.0] - 2026-04-27

Retargeted from v2.9.1 mid-development to bundle ST0036 (directory relocation, breaking change) into the same release. Version bump reflects the semver-breaking directory move; LLM canon work (originally scoped as v2.9.1) ships alongside.

Two steel threads landed in this release:

- **ST0035** -- Canonical LLM Config + Fleet Rollout. Three-file root canon (`AGENTS.md` + `CLAUDE.md` + `usage-rules.md`), session hooks (SessionStart + UserPromptSubmit strict gate + Stop), pre-commit critic gate via `bin/intent_critic`, `.intent_critic.yml` per-project config, the `working-with-llms.md` canon narrative, and per-language canon (`intent lang init`).
- **ST0036** -- Directory relocation `.intent/` -> `intent/.config/`. Breaking change. Intent's per-project metadata directory moves from a separate top-level `.intent/` to a nested `intent/.config/`, eliminating the "two top-level dirs" smell. Migration handled atomically by `migrate_v2_9_0_to_v2_10_0` on `intent upgrade`.

Fleet rollout: 14 in-scope projects (Intent self + 8 canary + 5 user-manual; Pplr OOS) all on v2.10.0 canon. Canary discipline surfaced and resolved three canon-installer rough edges (`MIGRATE_LEGACY_PRE_COMMIT`, `CHAIN_PRE_COMMIT` auto-insert, `NORMALIZE_GITIGNORE`) before fleet sweep. See `intent/st/ST0035/WP/15/canary-summary.md`, `WP/16/fleet-summary.md`, `WP/17/feedback-report.md`, and `WP/17/dogfood-journal.md`.

### Added

- **ST0035** (Canonical LLM Config + Fleet Rollout).
- **ST0036** (Directory relocation: `.intent/` -> `intent/.config/`). Breaking change. Intent's per-project metadata directory moves from top-level `.intent/` to nested `intent/.config/`, eliminating the "two top-level dirs" smell. Migration handled atomically by `migrate_v2_9_0_to_v2_10_0` on `intent upgrade`.
- **`intent lang` command** (ST0035/WP-19) for per-language canon installation. Subcommands: `list`, `show`, `init`. `intent lang init <lang> [<lang> ...]` is idempotent and multi-language; copies `intent/plugins/agents/templates/<lang>/{RULES,ARCHITECTURE}.md` into `intent/llm/{RULES,ARCHITECTURE}-<lang>.md` and appends a marker-bracketed entry to the agnostic `intent/llm/RULES.md` Language Packs section. Replaces the rejected auto-language-detection approach (real projects are polyglot; explicit user choice via `--lang` is more honest). Available canon languages: `elixir`, `rust`, `swift`, `lua`, `shell`. New stub templates ship for the four newer languages.
- **`intent init --lang <list>`** flag invokes `intent lang init` for each named language post-init. Comma- or space-separated list. Equals form (`--lang=elixir`) also accepted.
- **Agnostic `_default` canon now includes RULES.md + ARCHITECTURE.md** in fresh `intent init`. Previously only `MODULES.md` + `DECISION_TREE.md` were laid down; canon-installer's `_default` templates were only seen via `intent claude upgrade --apply`. Now `intent init` produces a v2.10.0-complete baseline including the Language Packs anchor that `intent lang init` writes into.

### Changed

- `bin/intent_helpers`: `migrate_v2_9_0_to_v2_10_0()` replaces the earlier `migrate_v2_9_0_to_v2_9_1()` stub. Bundles version stamp + ST0036 directory relocation. Canon-apply logic still lands in ST0035/WP11 via `intent claude upgrade --apply` (separate step).
- `bin/intent_upgrade`: chain extended to v2.10.0 (new gate `needs_v2_10_0_upgrade`, new case, new chain tail).
- Root `VERSION` bumped to `2.10.0`.
- **Treeindex ignore canonicalised** (ST0036/WP-06): new `lib/templates/_treeindexignore` template is the single source of truth. `bin/intent_treeindex::ensure_treeindexignore` reads from the template instead of an inline heredoc (Highlander cleanup per CLAUDE.md project rule #6). `intent claude upgrade --apply` installs the file when absent (new `INSTALL_TREEINDEXIGNORE` action; existing files left alone). Granularity flipped from blanket `.intent/` to `intent/.config/cache/` + `intent/.config/backup/` so `config.json` stays indexed.
- **Pre-commit hook template** (ST0036/WP-04): `lib/templates/hooks/pre-commit.sh` now probes `intent/.config/config.json` instead of `.intent/config.json` when deciding whether to skip the critic gate (fail-open in non-Intent repos). Newly-installed gates and any project that re-runs `intent claude upgrade --apply` after v2.10.0 pick up the corrected probe.

### Breaking

- **Per-project metadata directory relocated**: `.intent/config.json` → `intent/.config/config.json`. Same for `.intent/backup/` → `intent/.config/backup/`. Anything scripting against `.intent/` (CI, editor plugins, ad-hoc `jq`) breaks on upgrade; update to `intent/.config/`. Migration is fail-forward: old location is pruned, no backwards-compat symlink. Full migration guide including recovery from interrupted upgrades: [`intent/docs/migration-v2.10.0.md`](./intent/docs/migration-v2.10.0.md).

### Removed

- **`intent/usr/*.md`** retired (ST0035/WP-18). The three hand-authored user docs (`user_guide.md`, `reference_guide.md`, `deployment_guide.md`) were stamped at `intent_version: 2.6.0` (2026-03-05), seven minor versions behind the v2.10.0 canon, and substantially duplicated by `README.md`, `intent/docs/working-with-llms.md`, `intent help <cmd>`, and `AGENTS.md`. Per fail-forward (no preservation, prune actively): all three deleted; the `intent/usr/` directory is gone. Cross-references updated: `README.md` Documentation and Getting Help sections now point at `intent/docs/working-with-llms.md` (canon narrative) + `intent help` (commands) + `intent/docs/migration-v2.10.0.md` (upgrade path); `docs/blog/0005-getting-started-with-intent.md` Intent Documentation section refreshed; `intent/docs/migration-v2.10.0.md` "unchanged subdirectories" list trimmed.
- **ST0010** (Anthropic MCP Integration, v2.0.0-era) cancelled — superseded by v2.9.0 skills / subagents / extensions. Moved to `intent/st/CANCELLED/` with deprecation annotation.
- **ST0015** (Enhanced Steel Thread Templates, v2.0.0-era) cancelled — superseded by v2.9.0 tooling. Moved to `intent/st/CANCELLED/` with deprecation annotation.

## [2.9.0] - 2026-04-23

### Added

- **ST0034: Agentic Software Engineering Suite.** Rules become first-class citizens of Intent. Each rule is an atomic Markdown file with structured frontmatter, a Detection heuristic, and bad/good examples. Skills cite rules by stable `IN-*` IDs; Critic subagents enforce them.
- **Rule library** at `intent/plugins/claude/rules/` with packs for `agnostic`, `elixir`, `rust`, `swift`, `lua`, and `shell`. Schema reference at `intent/plugins/claude/rules/_schema/rule-schema.md`. Schema is intentionally compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, 2026 Manuel Zubieta) so upstream rules drop into Intent's discovery unchanged.
- **`intent claude rules`** command surface: `list`, `show`, `validate`, `index`. The `validate` subcommand is the canonical authoring gate; `index` regenerates a deterministic, sorted `index.json`.
- **Critic subagent family**: `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`. Thin orchestrators that read the rule library at invocation time, apply each rule's Detection heuristic to target source files, and emit a stable severity-grouped report. Modes: `code` and `test` (`critic-shell` is `code` only).
- **`.intent_critic.yml`** per-project config for disabling rules and adjusting severity thresholds. Sample at `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`.
- **User extension system** at `~/.intent/ext/<name>/` with the `intent ext` command surface (`list`, `show`, `validate`, `new`). Extensions contribute subagents, skills, or rule packs without modifying canon. Discovery is layered: canon is the default; user extensions override by name with a visible shadow warning. Manifest schema at `intent/plugins/claude/ext-schema/extension.schema.json`.
- **Reference extension `worker-bee`** at `~/.intent/ext/worker-bee/`. The migration seeds it from `lib/templates/ext-seeds/worker-bee/` on first run; further development happens at the user-local path, not in canon.
- **Authoritative documentation**: `intent/docs/rules.md` (rule library guide), `intent/docs/critics.md` (critic contract and report format), `intent/docs/writing-extensions.md` (extension authoring guide with worker-bee worked example).
- **Migration `migrate_v2_8_2_to_v2_9_0`** in `bin/intent_helpers`: stamps version, bootstraps `~/.intent/ext/`, seeds worker-bee, prunes installed copies of the deleted `elixir` subagent and the relocated `worker-bee` from `~/.claude/agents/` and `~/.intent/agents/installed-agents.json`. Idempotent — running the upgrade twice is safe and never overwrites user state.
- **`/in-session` bootstrap skill** for post-`/compact` skill loading.
- **`tests/unit/docs_completeness.bats`** verifies the new docs are present, cross-referenced, and that `intent agents sync` is idempotent.

### Removed

- **`elixir` subagent** (replaced by `critic-elixir` plus the Elixir rule pack). The migration aggressively prunes installed copies on upgrade.
- **`worker-bee` from Intent canon** (relocated to the reference extension at `~/.intent/ext/worker-bee/`). Re-install via `intent claude subagents install worker-bee` after the v2.9.0 upgrade.

### Changed

- **`in-standards` skill** loads agnostic rules by ID (no longer a "re-read CLAUDE.md" reminder).
- **`in-review` skill** stage-2 dispatches to `critic-<lang>` based on project language detection.
- **`in-elixir-essentials` and `in-elixir-testing` skills** declare machine-readable `rules:` frontmatter listing the IN-\* IDs they cite. Bodies remain rule-reference tables — content lives in the rule files.
- **TCA suite refactored for the rule library**: `in-tca-init` selects rule packs by ecosystem instead of inventing per-audit R-numbering; `in-tca-audit` dispatches `critic-<lang>` per WP and captures the verbatim critic report; `in-tca-synthesize` consumes the stable critic schema (CRITICAL/WARNING/RECOMMENDATION/STYLE + IN-_ IDs); `in-tca-remediate` and `in-tca-finish` cite IN-_ IDs throughout. The 1195-line `intent/docs/total-codebase-audit.md` is updated for v2.9.0; pre-v2.9.0 lessons-learned appendices are preserved with a historical-context note.
- **CLAUDE.md, MODULES.md, DECISION_TREE.md** updated for the v2.9.0 surfaces. DECISION_TREE.md gains three new branches: rule placement, skill placement, and rule-vs-skill-vs-subagent.
- **Help files** updated: `lib/help/ext.help.md`, `lib/help/rules.help.md`, `lib/help/claude.help.md` (now lists the `rules` subcommand and the `critic-*` family).
- **`creating-custom-agents.md`** distinguishes canon subagents from extension subagents; cross-links `writing-extensions.md`.

### Attribution

- Rule schema and selected rule principles inspired by [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, copyright 2026 Manuel Zubieta), pinned at commit `1d9aa40700dab7370b4abd338ce11b922e914b14`. See `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`.

## [2.8.2] - 2026-04-15

### Fixed

- **ST0033: cwd-resilient dispatch.** `intent` subcommands now work from any directory inside an Intent project, not only from the project root. The dispatcher (`bin/intent`) exports `INTENT_ORIG_CWD` and `cd`s to `$PROJECT_ROOT` before `exec`'ing the subcommand, so every subcommand runs with a known-correct cwd. Outside any project, commands fail cleanly with "not in an Intent project" and no longer create stray `.intent/` or `intent/` directories at the invoker's cwd. `intent treeindex` and `intent fileindex` consult `INTENT_ORIG_CWD` when resolving relative path arguments.
- **Upgrade chain completed through 2.8.2.** `bin/intent_upgrade`'s case statement previously halted at 2.6.0 for any starting version <= 2.5.0 and had no entry for 2.6.0/2.7.0 at all, leaving projects stuck mid-chain. Every starting-version case now chains through `migrate_v2_6_0_to_v2_8_0` (new, pure version stamp), `migrate_v2_8_0_to_v2_8_1`, and `migrate_v2_8_1_to_v2_8_2`. The pre-v2 fallback chain is extended to match. `needs_v2_8_2_upgrade` accepts 2.6.0 and 2.7.0 as starting points.
- **ST0032: Credo custom checks wired into `.credo.exs`.** `intent st zero` (D5a) and `intent audit` now use a standalone `lib/scripts/configure_credo.exs` to programmatically patch `.credo.exs`, replacing the earlier wrong hint about `elixirc_paths` in `mix.exs` and the `intent audit --checks-dir` workaround. Removed 2 broken check templates (`boolean_operators`, `dependency_graph`), fixed 4 buggy ones (`map_get_on_struct`, `missing_impl_annotation`, `debug_artifacts`, `thick_coordinator`), and added `bracket_access_on_struct`. Existing projects that went through `st zero` can re-run D5a to pick up the wiring.

## [2.8.1] - 2026-04-09

### Added

- **TCA pre-flight guard** (`tca-report.sh --check-only`) with 4 checks: shape (WP/ dir, design.md with rule set), feedback-report.md exists, no unfilled `[Fill in:` placeholders, zero unchecked `- [ ]` acceptance criteria in info.md
- **Provisioning Invariants** (§ 0.0 in `intent/docs/total-codebase-audit.md`): four load-bearing rules -- TCA is its own dedicated steel thread, WPs are flat, last WP is synthesis, rank components by later-pain impact not raw violation count
- **`tca-init.sh` provisioning guards**: refuse to provision inside an existing `intent/st/ST*/WP/*` path, refuse to overwrite an audit with populated `socrates.md` files
- **False Positive Guidance as REQUIRED** in the `in-tca-init` design.md template, with an R8/R9 example. Lamplight benchmark: R8 false-positive rate dropped from ~82% to 0% with pre-classification.
- **Audit metadata line** in `in-tca-audit` Post-WP section: `**Agent**: {type}; **Turns**: N; **Raw hits**: N; **FPs**: N` at the top of each component audit's `socrates.md`
- **`chains_to:` frontmatter** on all 5 TCA skills: `in-tca-init` -> `in-tca-audit` -> `in-tca-synthesize` -> `in-tca-remediate` -> `in-tca-finish` -> `in-finish`

### Changed

- **BREAKING (internal TCA scripts)**: `--st-dir` renamed to `--tca-dir` across `tca-init.sh`, `tca-progress.sh`, `tca-report.sh`, and their SKILL.md invocations. 33 occurrences across 6 files. Shell variable `ST_DIR` renamed to `TCA_DIR`. Direct callers of these scripts must update their invocations.
- **`in-tca-finish` skill restructured**: feedback report is now a top-level artifact at `$TCA_DIR/feedback-report.md` rather than a "Feedback WP" `socrates.md`. The `/in-finish` wrap-up is gated on the pre-flight guard passing.
- **Dedup-rate KPI framing** in the TCA reference doc: low dedup rate on newly-authored code is now framed as a positive signal about rule-aware authorship.

### Fixed

- **Premature TCA close-out failure mode**: prevents the "lying session docs" window that occurred during Lamplight ST0121 (commits 75706c18 -> 98616a0c, 2026-04-08). The pre-flight guard makes this mechanically impossible.
- **Silent guard failures** in `tca-report.sh`: `grep -c` and `grep | wc -l` pipelines interacted badly with `set -euo pipefail` (grep returning 1 on zero matches killed the script silently on assignment). Replaced with pure-shell while-loop counters.

### Motivation

Integrates feedback from the Lamplight ST0121 TCA run (2026-04-08/09). The audit worked -- 17 raw violations found, 10 fixed -- but exposed 8 corrections in provisioning and close-out discipline. Documentation was not enough: an eager operator skipped past written guidance. This release replaces guidance with mechanical guards wherever the failure modes allow it. See ST0031 (5 commits, `58143ae..5b4435f`) for implementation detail.

## [2.8.0] - 2026-03-28

### Added

- **Detrope skill** -- `/in-detrope` for LLM trope detection and stylometric analysis
  - Trope catalog vendored from [llm-tropes](https://github.com/matthewsinclair/llm-tropes) (44 tropes, 8 categories)
  - Context-aware severity assessment (reads project CLAUDE.md for audience/purpose)
  - Two modes: `quick` (diagnosis) and `full` (diagnosis + concrete rewrites)
  - Stylometric profile with AI signal strength rating
  - Integrates with Utilz `cleanz --detrope` for automated pre-scanning

### Changed

- **Blog series detroped** -- all 8 blog posts revised to remove LLM writing tropes
  - Removed magic adverbs, landscape metaphors, negative parallelism, stakes inflation
  - Rewrote to sound human: varied rhythm, concrete detail, reduced AI cadence

## [2.7.0] - 2026-03-19

### Added

- **TCA v3.0** -- Total Codebase Audit process document updated from v2.0 to v3.0 (ST0028)
  - Validated Rust and Swift rules replacing hypothetical ones (from real polyglot audit)
  - Ash Framework supplemental rules (A1-A5) as first-class audit rules
  - Rule precision boundaries (R5 matchable-values-only, R7 defstruct-only)
  - Effective file count model for WP sizing (weight table: Ash DSL 0.25x, Rust 1.5x, etc.)
  - Phase 0.5 pre-filtering of mechanical rules via grep
  - Confidence field (HIGH/MEDIUM/LOW) on audit findings
  - 5-tier priority scheme (P0/P1/P2a/P2b/P3) replacing 4-tier
  - Deduplication by root cause, not rule number
  - Main conversation remediation model (not sub-agents)
  - Test optimization with `mix test --failed`
  - Example C (polyglot: 256 files, 59% dedup rate)
  - New lessons: anti-hallucination, R5 over-reporting, remediation agent failures, R7 false positives
- **TCA skill suite** -- 5 operational skills with 3 automation scripts
  - `/in-tca-init` -- provisioning (SKILL.md + tca-init.sh)
  - `/in-tca-audit` -- component audit execution (SKILL.md + tca-progress.sh)
  - `/in-tca-synthesize` -- cross-component synthesis
  - `/in-tca-remediate` -- batched remediation in main conversation
  - `/in-tca-finish` -- wrap-up and feedback report (SKILL.md + tca-report.sh)

## [2.6.0] - 2026-03-05

### Added

- **Plugin discovery** -- `intent plugin` command for discovering plugins and their commands
  - `intent plugin` / `intent plugin list` -- lists all plugins with command syntax
  - `intent plugin show <name>` -- detailed view of a single plugin
  - `plugin.json` metadata files in each plugin directory for structured discovery
- `intent help claude` -- help file for the claude command namespace
- `intent help plugin` -- help file for the plugin command
- **ST0026 Phase 1** -- Steel Thread Zero code quality enforcement
  - Skills renamed from `intent-*` to `in-*` prefix
  - `intent claude prime` command for memory injection
  - LLM templates: `_CLAUDE.md`, `_MODULES.md`, `_DECISION_TREE.md`, `_ARCHETYPES.md`
  - 9 Elixir archetype templates in `lib/templates/archetypes/elixir/`
  - 5 workflow skills: `in-start`, `in-plan`, `in-next`, `in-standards`, `in-finish`
  - TN004 total codebase audit tech note
- **ST0026 Phase 2** -- Automated enforcement and guardrails
  - `intent audit quick` command with 7 custom Credo check templates (R2, R6, R7, R8, R11, R15, D11)
  - `intent audit health` command with 4 health checks, `--report` and `--diff` flags
  - `intent learn` command for capturing project learnings (footgun/worked/failed)
  - `intent modules check` command for module registry guardrails
  - `intent modules find` command for searching the registry
  - Dependency graph Credo check template (`dependency_graph.ex`, rule D11)
  - Dependency graph template (`_DEPENDENCY_GRAPH.md`) for umbrella apps
  - Claude Code advisory hook template for unregistered module warnings
  - `intent st zero install` command for brownfield project retrofit (D12)
    - 4-phase process: Audit, Gap Analysis, Proposals, Apply
    - 9 ST0000 deliverables checked (D2-D11): CLAUDE.md, MODULES.md, ARCHETYPES.md, Credo checks, DECISION_TREE.md, MEMORY.md, module hook, learnings.md, DEPENDENCY_GRAPH.md
    - Auto-discovers modules from `.ex` files in `lib/` (or `apps/*/lib/` for umbrellas)
    - Flags: `--audit-only`, `--dry-run`, `--deliverable <ID>`
    - Elixir-specific deliverables (D4, D5a, D11) only installed when `mix.exs` present
  - `intent init --with-st0000` flag for greenfield projects (D1)
    - Runs full ST0000 bootstrap after standard project initialization

### Changed

- `intent audit health` now umbrella-aware -- scans `apps/*/lib/` in umbrella projects
- `intent audit health` Highlander suspects reformatted to multi-line output (function name + indented files)
- `intent audit quick --checks-only` now force-copies templates (ensures updates applied on re-run)
- Rationalized CLI output across all commands to Rust-style conventions
  - Lowercase status prefixes: `ok:`, `error:`, `warning:`, `hint:`
  - Action prefixes: `created:`, `updated:`, `removed:`, `started:`, `done:`
  - No separator bars, banners, or unicode decorations
- `intent st` now supports `--help`/`-h` flags

### Fixed

- Credo template `thick_coordinator.ex`: `@default_params` interpolation before definition
- Credo template `highlander_suspect.ex`: unused variable warning on `_arity`
- Credo template `debug_artifacts.ex`: unused `@debug_calls` module attribute removed
- `intent help` now shows `claude` and `plugin` commands in Core section
- `intent help` agents description corrected from "Manage Claude Code sub-agents" to "Manage AGENTS.md for projects"
- `lib/help/agents.help.md` rewritten to document actual AGENTS.md commands (was documenting subagent operations)
- `intent help` now shows Plugins section pointing to `intent plugin`

## [2.5.0] - 2026-02-24

### Added

- **Work package management** -- `intent wp` as a top-level command (ST0024)
  - `intent wp new <STID> "Title"` -- create next WP in STID/WP/NN/info.md
  - `intent wp done <STID/NN>` -- mark WP as Done, hint when all WPs complete
  - `intent wp start <STID/NN>` -- mark WP as WIP
  - `intent wp list <STID>` -- table with WP, Title, Scope, Status columns
  - `intent wp show <STID/NN>` -- display WP info.md
  - `intent wp help` -- show usage
  - Specifier syntax: `ST0011/01` or shorthand `11/01`
  - WP info.md template at `lib/templates/prj/st/WP/info.md`
  - 29 new BATS tests in `tests/unit/wp_commands.bats`
- Shared helpers extracted to `bin/intent_helpers`:
  - `normalise_st_id()` -- normalizes bare numbers and partial IDs to ST#### format
  - `escape_sed_replacement()` -- escapes special characters for sed substitutions
- `intent-essentials` skill Rule 8: "Use `intent wp` commands for work package management"
- **Shared plugin helper library** -- Highlander audit refactoring
  - `intent/plugins/claude/lib/claude_plugin_helpers.sh` -- shared install/sync/uninstall via callbacks
  - `intent_claude_skills` reduced from 654 to 299 lines
  - `intent_claude_subagents` reduced from 1015 to 613 lines
  - `get_config_field()` in `bin/intent_helpers` replaces inline `grep -oE` config extraction

### Removed

- **Backlog.md integration** -- all backlog commands and configuration removed (ST0023)
  - Removed `intent bl` / `intent backlog` wrapper command
  - Removed `intent task` command (create, list, sync)
  - Removed `intent status` command (show, sync, report)
  - Removed `intent migrate` command (embedded task migration)
  - Removed `backlog_dir` and `backlog_list_status` configuration keys
  - Removed backlog directory creation from `intent init`
  - Removed backlog optional tool check from `intent doctor`
  - Removed backlog references from all subagent definitions
  - Removed Node.js setup from CI pipeline (was only needed for Backlog.md)
  - Deleted 3 test files (bl_commands.bats, task_commands.bats, migration.bats)
  - Test suite reduced from 17 to 14 files

### Changed

- Documentation updated with WP bare number syntax, special character support, and directory structure
- CI pipeline simplified: no longer requires Node.js installation
- `intent help` no longer lists backlog-related commands
- `intent info` no longer shows Backlog section
- Consolidated duplicate `version`/`intent_version` config fields to just `intent_version`
- TPD files annotated with "[Removed in v2.5.0]" for historical backlog sections
- Blog posts annotated with editor's notes about removal

### Fixed

- Test side-effect: `agent_commands.bats` no longer modifies real source files during test runs
  - Added `create_source_sandbox()` for tests that simulate source changes
  - Removed `git checkout` from teardown that was reverting uncommitted edits

## [2.4.0] - 2026-02-17

### Added

- **Skills system** -- new always-on enforcement layer for Claude Code (ST0020)
  - `intent claude skills list` -- show available and installed skills
  - `intent claude skills install <name>` -- install skill(s) to `.claude/skills/`
  - `intent claude skills sync` -- update installed skills with latest versions
  - `intent claude skills uninstall <name>` -- remove Intent-managed skills
  - `intent claude skills show <name>` -- display skill content and status
  - SHA256 checksum-based manifest tracking at `~/.intent/skills/installed-skills.json`
- Six skills for proactive code enforcement:
  - `intent-essentials` -- 7 Intent workflow rules (CLI usage, treeindex, steel thread conventions)
  - `intent-elixir-essentials` -- 8 core rules (pattern matching, tagged tuples, pipes, naming)
  - `intent-ash-ecto-essentials` -- 7 Ash/Ecto rules (code interfaces, migrations, actor placement)
  - `intent-phoenix-liveview` -- 7 LiveView rules (two-phase mount, streams, components)
  - `intent-elixir-testing` -- 8 mandatory test quality rules (no control flow in tests, strong assertions, spec-driven)
  - `intent-autopsy` -- session forensics and memory meta-learning (ST0021)
- **Diogenes subagent** -- Elixir Test Architect using Socratic dialog (ST0020 WP-11)
  - Two personas: Aristotle (Empiricist) and Diogenes (Skeptic)
  - Specify mode: 5-phase dialog producing `*.spec.md` test specifications
  - Validate mode: gap analysis comparing specs to test files
- **intent-autopsy skill** -- session forensics and memory meta-learning (ST0021)
  - Inspired by [@chickensintrees](https://github.com/chickensintrees) and adapted from his work with STEF
  - Elixir script (`autopsy.exs`) pre-processes JSONL session files
  - Detects correction pairs, frustration signals, capability regressions, banned patterns
  - Memory-aware analysis: compares findings against MEMORY.md and CLAUDE.md rules
  - Identifies memory gaps, enforcement failures, undocumented conventions, stale memory
  - Ships default `banned-words.txt` with common AI-isms (delve, unfortunately, etc.)
- `intent claude upgrade` command for diagnosing and upgrading LLM guidance layer
  - Dry-run by default (use `--apply` to execute)
  - `--project-dir DIR` to target external projects
  - Diagnoses files, subagents, and skills; generates upgrade plan; applies changes
- Elixir subagent reference documents:
  - `ash-ecto.md` -- Ash/Ecto database patterns (Ash-first, never raw Ecto)
  - `liveview.md` -- LiveView operational patterns (two-phase rendering, streams, uploads)
  - `testing.md` -- Testing reference (DataCase, ConnCase, LiveView, Mox, Ash testing)
  - `project-structure.md` -- Standard Phoenix/Ash project layout
- Elixir project templates for `intent agents init --template elixir`:
  - `AGENTS.md` template with Elixir project overview and commands
  - `RULES.md` template with 9 core rules + framework rules + NEVER DO list
  - `ARCHITECTURE.md` template with domain map and directory structure skeleton
- `usage-rules.md` -- Intent's own LLM-optimized usage reference (~310 lines)
- `docs/upgrade-guide-2.4.0.md` -- human-readable upgrade guide for Intent projects
- **Special character handling** in `st new` -- titles with `/`, `&`, `\` no longer break creation (ST0022)
- **Slug generation** -- `st new` auto-generates a URL-safe `slug:` field in frontmatter, max 50 chars (ST0022)
- **`-s|--start` flag** for `st new` -- create and immediately start a steel thread in one command (ST0022)
- `intent doctor` now checks for Elixir installation (optional, needed for autopsy)
- BATS tests across 17 test files

### Changed

- Refactored Elixir subagent rules from 23 overlapping to 12 non-overlapping rules
  - Organized into 5 categories: Data Access, Control Flow, Composition, Error Handling, Code Hygiene
  - Each rule is distinct with no overlap between categories
- `intent claude skills install` now copies entire skill directory (not just SKILL.md)
  - Scripts and supporting files installed alongside SKILL.md
  - `intent claude skills sync` also copies full directory on update
- Updated `intent agents init` to support `--template <name>` flag
  - Template copies AGENTS.md, RULES.md, ARCHITECTURE.md from template directory
  - RULES.md and ARCHITECTURE.md are human-curated (not overwritten without `--force`)
- Added NEVER DO rule: never put `require` inside a function body (module level only)
- Added YAML frontmatter with `description` field to all SKILL.md files for Claude Code discovery
- `st list` and `st sync` now show "Slug" column instead of "Title" (falls back to title for older threads)
- Updated copyright to 2026 across all source files

## [2.3.4] - 2026-02-04

### Added

- `intent treeindex DIR` command for LLM-optimized directory summaries (ST0019 WP01)
  - Bottom-up directory indexing with Claude Haiku 4.5 for summarization
  - Centralized shadow directory at `intent/.treeindex/` keeps source tree clean
  - `.treeindexignore` configuration for excluding files/dirs from indexing
  - Auto-generated `README.md` in `.treeindex/` shadow directory for LLM orientation
  - Fingerprint-based staleness detection (filenames + sizes, no mtime dependency)
  - `--check` mode for CI/reporting without regeneration
  - `--dry-run` mode to preview without writing
  - `--force` to regenerate regardless of staleness
  - `--depth N` to control directory traversal depth (default 2)
  - Platform-compatible (macOS/Linux stat differences handled)
  - Bash 3.2 compatible (works with macOS default `/bin/bash`)
- 41 bats tests for treeindex command in `tests/unit/treeindex_commands.bats`
- CLAUDE.md convention: check `intent/.treeindex/<dir>/.treeindex` before exploring unfamiliar directories
- Release notes documentation in `docs/releases/2.3.4/RELEASE_NOTES.md`

### Fixed

- `intent init` now displays correct version from VERSION file instead of hardcoded 2.0.0
- `--sync` flag bug in steel thread management

### Changed

- Expanded Elixir subagent with architectural principles, Ash/Phoenix patterns, and testing guidance
- Replaced 'eg' abbreviation throughout documentation (was 'e.g.,')
- Updated all documentation to match as-built codebase (was frozen at v2.1.0)
  - `.github/workflows/README.md`: Full rewrite from STP to Intent
  - `tests/README.md`: Updated to v2.3.4 with all 14 test files
  - `README.md`: Fixed project structure, added treeindex/AGENTS.md/subagent commands
  - `intent/usr/user_guide.md`: Added treeindex, AGENTS.md, Claude subagent sections
  - `intent/usr/reference_guide.md`: Added treeindex, fileindex, agents, subagent command references
  - `intent/usr/deployment_guide.md`: Added plugin/subagent deployment and treeindex integration
  - `examples/hello-world/README.md`: Updated to v2.3.4 with current structure

### Migration

- Added `migrate_v2_3_3_to_v2_3_4()` function in `bin/intent_helpers`
- Added `needs_v2_3_4_upgrade()` function in `bin/intent_helpers`
- Updated `bin/intent_upgrade` to handle v2.3.3 -> v2.3.4 upgrade path
- All version upgrade paths updated to include v2.3.4 migration

### Technical Improvements

- Treeindex uses headless `claude -p` with `--tools ""` for text-in/text-out summarization
- Shadow directory design avoids polluting source tree with index files
- Fingerprint design is git-clone-stable (no mtime dependency)
- Full test suite now at 265 tests

## [2.3.3] - 2025-10-02

### Added

- Comprehensive Elixir style guide for the Elixir Claude subagent
  - Module organization (imports, aliases, whitespace)
  - Function definitions and multiline preferences
  - Testing patterns and fixture design
  - Code composition and pipeline usage
  - Naming conventions and ubiquitous language
  - Documentation standards
  - Type specifications
  - Dependency management
  - Database design precision
  - Version control conventions
- Full style documentation in `intent/plugins/claude/subagents/elixir/style.md`
- Release notes documentation in `docs/releases/2.3.3/RELEASE_NOTES.md`

### Changed

- Updated `intent/plugins/claude/subagents/elixir/agent.md` to reference style guide alongside antipatterns
- Enhanced Elixir subagent now provides both antipattern detection (v2.3.2) and style guidance (v2.3.3)

### Migration

- Added `migrate_v2_3_2_to_v2_3_3()` function in `bin/intent_helpers`
- Added `needs_v2_3_3_upgrade()` function in `bin/intent_helpers`
- Updated `bin/intent_upgrade` to handle v2.3.2 → v2.3.3 upgrade path
- All version upgrade paths updated to include v2.3.3 migration

### Technical Improvements

- Elixir subagent now provides holistic code quality guidance combining antipatterns and style
- Style guide complements antipattern detection for comprehensive code reviews
- Improved upgrade mechanism with full test coverage (212 tests passing)

## [2.3.2] - 2025-09-04

### Added

- Comprehensive antipattern detection to Elixir subagent
  - Detects and remediates 24 common Elixir antipatterns
  - Antipatterns categorized into Code (9), Design (6), Process (4), and Meta-programming (5)
  - Full documentation in `intent/plugins/claude/subagents/elixir/antipatterns.md`
  - Antipatterns sourced from official Elixir documentation
- Antipattern review workflow integrated into Elixir Doctor
- Example usage commands and report formats for antipattern detection
- Key principles for antipattern prevention

### Changed

- Enhanced Elixir subagent with antipattern detection capabilities
- Updated systematic review template to include antipattern analysis
- Elixir Doctor now automatically checks for antipatterns during code reviews

### Technical Improvements

- Better code quality guidance through antipattern detection
- More comprehensive code review process
- Proactive detection of common Elixir mistakes

## [2.3.1] - 2025-08-29

### Added

- Worker-bee agent for Worker-Bee Driven Design (WDD) in Elixir applications
- Resources directory structure for agents with templates and Mix tasks
- Worker-bee agent includes comprehensive WDD validation and scaffolding tools

### Changed

- Enhanced agent system to support resource directories
- Improved subagent installation and management

## [2.3.0] - 2025-08-20

### Added

- Plugin architecture for Intent
- Claude subagents system (renamed from agents)
- AGENTS.md universal AI agent instructions
- Support for multiple AI platforms through AGENTS.md
- New `intent agents` commands for AGENTS.md management
- New `intent claude subagents` commands (replacing old `intent agents`)

### Changed

- Renamed `intent agents` commands to `intent claude subagents`
- Moved subagents to `intent/plugins/claude/subagents/`
- Updated project structure to support plugins

### Technical Improvements

- More flexible agent system architecture
- Better separation of concerns with plugin system
- Universal agent instructions format

## [2.2.1] - 2025-08-11

### Added

- Centralized version management through VERSION file
- `get_intent_version()` function in intent_helpers for consistent version retrieval
- Comprehensive tool dependency checking in `intent doctor`
- Platform-specific installation instructions for all required tools
- Better error handling for missing jq dependency across all commands

### Changed

- Steel threads now start with 'WIP' status instead of 'In Progress' when using `intent st start`
- Tool dependencies categorized as required, core, and optional in doctor command
- Enhanced jq error messages with clear installation instructions
- All scripts now read version from centralized VERSION file

### Fixed

- `intent upgrade` now preserves existing CLAUDE.md files instead of overwriting them
- Silent failures when jq is missing during agent operations
- Missing error messages for required tool dependencies
- Inadequate installation guidance for different platforms
- Version number inconsistencies across different scripts

### Technical Improvements

- Single source of truth for version management
- Reduced maintenance overhead for version updates
- Improved fallback behavior when tools are missing
- Better user experience with actionable error messages

## [2.2.0] - 2025-08-05

### Added

- `intent fileindex` command for systematic file tracking and progress management
- Check functionality (`-C` flag) to explicitly mark files as checked [x] in the index
- Uncheck functionality (`-U` flag) to explicitly mark files as unchecked [ ] in the index
- Toggle functionality (`-X` flag) to switch files between checked/unchecked states
- Flexible operation modes - works both within Intent projects and standalone
- Enhanced Elixir agent with systematic code review workflow using fileindex
- Support for both Elixir module names and filesystem paths in the Elixir agent
- Comprehensive test suite for fileindex command (47 tests including check/uncheck)
- Demo mode (`--demo`) to showcase fileindex functionality

### Changed

- Updated all version references from 2.1.0 to 2.2.0
- Enhanced `intent upgrade` to support 2.1.0 → 2.2.0 migrations
- Improved upgrade path handling for incremental version upgrades
- Updated Elixir agent documentation with systematic review workflow
- Added fileindex to global commands list

### Fixed

- Bash compatibility issues on macOS (associative arrays, readarray command)
- Local variable declarations at global scope in shell scripts
- Missing `assert_output` function in test framework
- Test expectations for error messages

### Technical Improvements

- Replaced bash associative arrays with parallel arrays for macOS compatibility
- Replaced `readarray` with portable while loops
- Added proper error handling for edge cases in file operations
- Enhanced test helper with assert_output function

## [2.1.0] - 2025-07-27

### Added

- `intent agents init` command to initialize agent configuration
- Support for upgrading from Intent v2.0.0 to v2.1.0
- Enhanced agent manifest management with proper initialization
- Improved agent setup workflow with explicit initialization step

### Changed

- Updated all version references from 2.0.0 to 2.1.0
- Enhanced `intent upgrade` to support 2.0.0 → 2.1.0 migrations
- Improved agent installation workflow to require initialization first
- Updated documentation to reflect v2.1.0 features

### Fixed

- Agent directories not being properly created during upgrade
- Missing agent initialization when upgrading from older versions
- Agent manifest not being created in fresh installations
- Incorrect creation of `agents/` directory at project root instead of `intent/agents/`
- Upgrade process incorrectly preserving root-level agent directories

## [2.0.0] - 2025-07-17

### Added

- New `intent` command as the primary CLI (replacing `stp`)
- `intent bootstrap` command for easy global setup
- `intent doctor` command for comprehensive diagnostics
- `intent st repair` command to fix malformed steel thread metadata
- JSON-based configuration system (local and global)
- Full backwards compatibility with STP v1.x projects
- Comprehensive test suite with GitHub Actions CI/CD
- Example projects demonstrating migration paths
- Support for `jq` dependency in workflows
- **Claude Code Sub-Agent Integration**: Complete agent management system
  - `intent agents` command suite (list, install, sync, uninstall, show, status)
  - Intent agent with steel thread methodology knowledge
  - Elixir agent with Usage Rules and Ash/Phoenix patterns
  - Global and project-specific agent support
  - Manifest-based tracking with checksum integrity
  - Seamless integration with intent init, doctor, and upgrade commands

### Changed

- **BREAKING**: Renamed from STP to Intent
- **BREAKING**: Flattened directory structure (intent/ instead of stp/prj/)
- **BREAKING**: Executables moved to top-level bin/ directory
- **BREAKING**: Configuration format changed from YAML to JSON
- Improved error messages and user feedback
- Enhanced migration tools with fail-forward approach
- Streamlined command structure and naming
- Updated all documentation to reflect Intent branding

### Fixed

- GitHub Actions workflow issues with bats libraries
- Symlink issues with stp compatibility command
- Test suite reliability and coverage
- Configuration loading hierarchy
- Path resolution in various environments
- Malformed YAML frontmatter in steel threads after migration
- Legacy field names (stp_version) in steel thread metadata
- Conflicting status values between frontmatter and body content

### Deprecated

- `stp` command (now aliases to `intent` for compatibility)
- Old directory structure (stp/prj/st/ → intent/st/)
- YAML configuration format
- Nested project directory structure

### Migration Guide

#### From STP v1.x to Intent v2.0.0

1. **Automatic Migration**: Run `intent upgrade` to automatically migrate your project
2. **Manual Installation**:

   ```bash
   # Clone Intent repository
   git clone https://github.com/matthewsinclair/intent.git
   cd intent

   # Add to PATH
   export PATH="$PATH:$(pwd)/bin"

   # Bootstrap global configuration
   intent bootstrap
   ```

3. **Project Structure Changes**:
   - `stp/prj/st/` → `intent/st/`
   - `stp/prj/wip.md` → `intent/wip.md`
   - `stp/eng/` → `intent/eng/`
   - `stp/usr/` → `intent/usr/`

4. **Command Changes**:
   - All `stp` commands now use `intent`
   - Same subcommands and options supported
   - `stp` symlink provided for compatibility

See [Release Notes](./docs/releases/2.0.0/RELEASE_NOTES.md) for complete details.

## [1.2.1] - 2025-07-09

### Added

- Directory-based structure for steel threads (replacing single files)
- New steel thread file types: `info.md`, `design.md`, `impl.md`, `tasks.md`
- Migration script `migrate_st_to_dirs` for upgrading from v1.2.0 to v1.2.1
- Support for editing/viewing specific steel thread files with `stp st show/edit <id> <file>`
- `stp st show <id> all` command to view all steel thread files at once
- Automatic file creation when editing non-existent steel thread files
- Version tracking in `stp/.config/version` file

### Changed

- **BREAKING**: Steel threads are now directories containing multiple files instead of single `.md` files
- Updated `stp_st` script to handle both legacy (file) and new (directory) structures
- Enhanced `stp st new` to create directory structure with all template files
- Modified `stp st done` to move entire directories when completing steel threads
- Updated `stp st list` to read from `info.md` files in directories
- Enhanced `stp st organize` to handle directory-based steel threads
- Improved `stp upgrade` to automatically detect and migrate steel threads to directory structure
- Updated all documentation to reflect new steel thread structure

### Fixed

- Version detection in `stp_st` now properly checks for directory vs file structure
- Steel thread organization now correctly moves directories instead of files

### Migration Guide

#### Upgrading from v1.2.0 to v1.2.1

1. Run `stp upgrade` - it will detect old-format steel threads and offer to migrate them
2. The migration will:
   - Create a backup in `.backup/1.2.1/`
   - Create directories for each steel thread (eg `ST0001/`)
   - Split content into separate files based on sections
   - Preserve all existing content and metadata
3. After migration, use `stp st organize --write` to organize by status if desired

#### New Steel Thread Commands

- `stp st show ST0001 design` - Show only the design.md file
- `stp st edit ST0001 impl` - Edit the implementation file
- `stp st show ST0001 all` - View all files for a steel thread

## [1.2.0] - 2025-07-09

### Added

- New `stp llm usage_rules` command for displaying STP usage patterns to LLMs
- `--symlink` option for `stp llm usage_rules` to create usage-rules.md symlinks in projects
- Comprehensive test suite for the llm command (`stp/tests/llm/llm_test.bats`)
- DEPRECATIONS.md file to track deprecated features
- Help documentation for llm command (`stp/bin/.help/llm.help.md`)
- Archive directory structure for deprecated content (`stp/prj/archive/`)

### Changed

- Renamed `usage_rules.md` to `usage-rules.md` to follow Elixir Hex package conventions
- Updated `stp_upgrade` to handle file renaming during upgrades
- Updated all documentation to reference Backlog for historical tracking instead of journal.md
- Simplified `stp_init` to only create `wip.md` and `steel_threads.md` in the prj directory

### Fixed

- Fixed `stp upgrade` version mismatch (was using 1.0.0 instead of 1.2.0)
- Made file organization in `stp upgrade` optional with new `--organize` flag to prevent unexpected file moves

### Deprecated

- `journal.md` file - users should migrate to Backlog task tracking for historical project narrative

### Removed

- `journal.md` creation from `stp_init` script
- Journal template from `stp/_templ/prj/_journal.md`
- All references to `journal.md` from documentation (18 files updated across user guides, reference guides, blog posts, and templates)

### Migration Guide

#### For users with existing journal.md files

1. Your existing `journal.md` has been automatically moved to `stp/prj/archive/journal-deprecated.md`
2. Use `stp bl list` to view task history moving forward
3. Track detailed progress in Backlog task descriptions
4. Use steel thread documents for high-level context and decisions

#### For LLM integration

1. Use `stp llm usage_rules` to display usage patterns
2. Create symlinks with `stp llm usage_rules --symlink` for projects expecting usage-rules.md
3. Reference the usage rules documentation at `intent/llm/usage-rules.md`

## [1.0.0] - 2025-06-03

### Added

- Initial release of Steel Thread Process (STP)
- Core script framework for managing steel threads
- Template system for project documentation
- Integration with Backlog.md for task management
- Comprehensive test suite using BATS
- User and reference documentation
- Blog series explaining STP concepts and methodology

### Features

- `stp init` - Initialize STP in a project
- `stp st` - Manage steel threads (new, list, show, edit, done, sync)
- `stp bl` - Backlog.md wrapper for task management
- `stp task` - Create and list tasks linked to steel threads
- `stp status` - Synchronize steel thread status with task completion
- `stp migrate` - Migrate embedded tasks to Backlog
- `stp upgrade` - Upgrade STP files to latest format
- `stp help` - Comprehensive help system

[2.6.0]: https://github.com/matthewsinclair/intent/compare/v2.5.0...v2.6.0
[2.5.0]: https://github.com/matthewsinclair/intent/compare/v2.4.0...v2.5.0
[2.4.0]: https://github.com/matthewsinclair/intent/compare/v2.3.4...v2.4.0
[2.3.4]: https://github.com/matthewsinclair/intent/compare/v2.3.3...v2.3.4
[2.3.3]: https://github.com/matthewsinclair/intent/compare/v2.3.2...v2.3.3
[2.3.2]: https://github.com/matthewsinclair/intent/compare/v2.3.1...v2.3.2
[2.3.1]: https://github.com/matthewsinclair/intent/compare/v2.3.0...v2.3.1
[2.3.0]: https://github.com/matthewsinclair/intent/compare/v2.2.1...v2.3.0
[2.2.1]: https://github.com/matthewsinclair/intent/compare/v2.2.0...v2.2.1
[2.2.0]: https://github.com/matthewsinclair/intent/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/matthewsinclair/intent/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/matthewsinclair/intent/compare/v1.2.1...v2.0.0
[1.2.1]: https://github.com/matthewsinclair/intent/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/matthewsinclair/intent/compare/v1.0.0...v1.2.0
[1.0.0]: https://github.com/matthewsinclair/intent/releases/tag/v1.0.0
