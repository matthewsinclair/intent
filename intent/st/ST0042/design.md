# Design - ST0042: Fable 5 Review of Intent codebase

## Approach

The review is structured as an MFIC exercise (ST0041). Each axis is made operational:

- **Mechanically** -- coverage is enumerated up front (see Coverage map), not hand-picked. Mechanical sweeps (placeholder drift, hardcoded versions, shellcheck) run over whole surfaces; reviewer agents are assigned per dimension so no subsystem is skipped silently. Anything deliberately excluded is named in Scope exclusions.
- **Falsifiable** -- every finding is a claim with file:line evidence and an explicit refutation condition ("this finding is wrong if ..."). Findings without a refutation condition do not graduate to WPs.
- **Independent** -- the reviewing session is causally independent of the sessions that produced the code (fresh context; accepted bar per ST0041). Findings are verified against the code itself, never against docs' or commit messages' account of it. Load-bearing findings get a second-pass refutation attempt before being presented.
- **Control** -- the human review gate. Findings become WPs only after user adjudication; nothing executes from this review without it.

## Coverage map

| #   | Dimension                | Surface                                                                        | Method                            |
| --- | ------------------------ | ------------------------------------------------------------------------------ | --------------------------------- |
| 1   | Shell rule compliance    | `bin/` core scripts                                                            | critic-shell subagent             |
| 2   | Architecture             | dispatch (`bin/intent`), helpers monolith, plugin boundaries, config loading   | reviewer agent                    |
| 3   | Templates and generation | `lib/templates/`, generators, placeholder substitution                         | mechanical sweep + reviewer agent |
| 4   | Test suite quality       | `tests/` (bats unit + integration, runner)                                     | reviewer agent                    |
| 5   | Docs canon consistency   | AGENTS.md / CLAUDE.md / usage-rules.md / working-with-llms.md / MODULES.md     | reviewer agent                    |
| 6   | Upgrade/migration chain  | `bin/intent_upgrade`, `migrate_*` in `intent_helpers`, dispatcher case ladders | reviewer agent                    |
| 7   | Plugin surface           | `intent/plugins/claude/` + `intent/plugins/agents/` (skills, subagents, rules) | reviewer agent                    |
| 8   | Robustness               | quoting, sed escaping, temp files, bash-3.2 compat, concurrency                | mechanical sweep + critic-shell   |

### Scope exclusions

- `examples/` and historical ST docs (`intent/st/COMPLETED/`) -- archival, not load-bearing.
- `intent_main` legacy wrapper -- declared legacy in MODULES.md; reviewed only for deletion candidacy, not quality.
- Generated artefacts (root `AGENTS.md`, `.treeindex` files) -- generators are in scope, outputs are not.

## Finding format

```
ID:          F-<dimension>-<n>
Claim:       <one falsifiable sentence>
Evidence:    <file:line references>
Refuted if:  <what observation would kill this finding>
Severity:    blocker | high | medium | low
Confidence:  confirmed (I-verified) | reported (agent only)
```

## Design Decisions

- Findings are deduplicated across dimensions before verification; the surviving set is verified finding-by-finding (read the cited code, attempt refutation) before anything is presented.
- Severity reflects consequence for Intent's users, not engineering effort; effort is carried separately as t-shirt size on the proposed WP.
- No vanity metrics: counts of findings/tests/files reviewed are not reported as outcomes.

## Audit framing (read first)

The purpose of this audit is **architectural integrity, Highlander violations, and poor design/implementation** -- not security. Security findings are incidental: if one surfaces it is recorded (see T1), but it is not the lens and execution sessions should not over-index on it. Prioritise the design-quality themes (Highlander duplication, dead code, drift, thin-coordinator violations, convergent-vs-ceremony architecture).

## Findings

Eight reviewer dimensions ran (the seven planned plus an upgrade-rethink design dimension added at the user's request). Findings below are grouped into themes. Severity is consequence-for-users; `confirmed` means independently re-verified against the code this session, `reported` means single-reviewer with cited evidence not yet re-verified. The MFIC leak write-up is in impl.md.

### T1 -- Config eval is arbitrary code execution (confirmed; incidental to the audit's purpose)

> Out of the audit's declared scope (architectural integrity, not security), recorded because it surfaced. Treat as a quality/robustness defect -- the `eval` is also a fragile design -- rather than a security workstream.

`load_intent_config` evals the output of `parse_json`, which interpolates raw JSON values into `key="value"` strings via jq. A value containing `$(...)`, backticks, or `$VAR` expands at eval time. A checked-in `intent/.config/config.json` (or `~/.config/intent/config.json`) therefore runs arbitrary shell on any contributor's machine on the next project-scoped `intent` command.

- Evidence: `bin/intent_config:35` (jq `to_entries` interpolation), `:84` + `:96` (`eval`); reached from `bin/intent:181-183` for every non-global command.
- PoC (run + reverted this session): a config with `"author": "$(touch /tmp/intent_eval_pwned)"` created the marker file on `intent st list`.
- Refuted if: a sanitisation pass between parse and eval is found (none exists).
- Fix direction: stop evaling. Read individual fields with `jq -r '.field'` (the `get_config_field` approach already in `intent_helpers`), eliminating the eval entirely.

### T2 -- Rules/docs-path drift, the v2.11.11 class, still live (confirmed HIGH + reported MEDIUM/LOW)

The v2.11.11 release fixed generated guidance and the critics to reference rules via the CLI rather than the repo-local `intent/plugins/claude/rules/` path. The fix did not reach the skills or several other surfaces.

- F-PLG-1 (HIGH, confirmed): nine canon skills still cite the dead local path -- `in-session`, `in-standards`, `in-review`, `in-ash-ecto-essentials`, `in-elixir-essentials`, `in-elixir-testing`, `in-tca-init`, `in-tca-synthesize`, `in-tca-finish`. `in-session` auto-loads every session in every fleet project, so every non-Elixir project is steered to a nonexistent path. (`in-session/SKILL.md:42,47-50` et al.)
- F-TPL-1 (HIGH, confirmed): `[[LANG]]` in `lib/templates/llm/_usage-rules.md:25,66` is substituted by no generator; `canon_substitute_placeholders` (`intent_claude_upgrade:228-238`) does not cover it, so it ships verbatim into consumers' `usage-rules.md`.
- F-TPL-12 (MEDIUM, reported): generated `AGENTS.md`, `_CLAUDE.md`, and `templates/elixir/AGENTS.md` reference `intent/docs/working-with-llms.md`, `critics.md` etc. as project-local, but those exist only in the Intent install; `_usage-rules.md` correctly qualifies "at the Intent install", so the canon disagrees with itself.
- F-PLG-12 (LOW, reported): the installed `.intent_critic.yml` points at `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`, absent in consumers (`lib/templates/_intent_critic.yml:9`).
- Fix direction: finish the swap to CLI/qualified references, and add a mechanical guard (test or critic) that greps every propagated/generated artefact for the dead local path so this class cannot regress a fourth time.

### T3 -- Commands that report success while doing nothing (confirmed/reported, No-Silent-Errors)

- F-TPL-4 (MEDIUM): `intent st new` legacy (single-file) path references `lib/templates/prj/st/_ST####.md`, which does not exist; the guarded `if [ -f ]` creates no file yet prints `created:` (`intent_st:491,502,505-506`).
- F-TPL-9 (MEDIUM): `intent agents init --template {rust,lua,shell,swift}` prints "Created AGENTS.md at project root." while the guarded copy created nothing (those template dirs have no AGENTS.md) (`intent_agents:176-179,204-205`).
- F-TPL-11 (MEDIUM): `intent_init` interactive agent install invokes `"$SCRIPT_DIR/intent_agents"`, a path that does not exist (the script is at `intent/plugins/agents/bin/intent_agents`), so the "Y" branch can never succeed (`intent_init:283`).
- F-shell (CRITICAL): upgrade backup uses `cp -r ... 2>/dev/null || true` then prints "Backup created successfully" unconditionally, immediately before destructive migration (`intent_upgrade:140-144`).

### T4 -- AGENTS.md generation wrong and self-misreporting (confirmed/reported MEDIUM)

- F-PLG-3 (MEDIUM, confirmed): the generator reads `$PROJECT_ROOT/.claude/{skills,agents}` while installers write to `$HOME/.claude/...`, so the "Installed Skills" section reads "No skills installed" on a fully provisioned machine (verified against live `AGENTS.md:66`; generator `intent_agents:74,102`).
- F-TPL-10 (MEDIUM, reported): `templates/_default/AGENTS.md` lacks three of the four sections `intent agents validate` requires, so a fresh `--template _default` install fails the tool's own validator; it also self-describes as "auto-generated by intent agents sync" though sync replaces it with divergent heredoc content (a Highlander split: template vs heredoc, `intent_agents:275-512,545,578-593`).

### T5 -- Highlander violations / duplication (confirmed/reported, one live bug)

- Status normalisation diverged -- LIVE BUG (HIGH): `normalise_status` maps `wip` -> `WIP`, but the inline copies in `repair` and `organize` map `wip` -> `In Progress` (`intent_st:107-140` vs `:1410-1426` vs `:1588-1604`).
- `get_intent_version` fallback literal repeated ~14x with drift (`2.2.1`/`2.6.0`/`2.3.x`/...), so a broken install reports a different stale version depending on which script is asked (`bin/intent:27`, `intent_config:17`, `intent_helpers:35,607,...`, `intent_upgrade:14`). Single biggest cleanup; also resolves F-UPG-8.
- Config parsing implemented three divergent ways (`intent_config:23-37` eval+jq, `intent_helpers:67-80` grep/cut, `intent_claude_upgrade:576` grep|sed) (F-ARCH-2).
- `find_project_root` reimplemented three times (`intent_config:40-67`, `critic_runner.sh:51-61`, `pre-commit.sh:58-65`) (F-ARCH-6).
- `update_config_version` exists but the stamp block is inlined in ~12 migrate functions (critic IN-SH-CODE-006).
- ST-dir resolver in three places (`intent_wp:62`, `intent_st:160-165`, `:1500`); ext-dir walk in five (F-PLG-6).
- Fake-HOME test isolation copy-pasted in six test files, absent in the seventh -- the direct cause of T10/F-TEST-1 (F-TEST-9).

### T6 -- Dead / legacy code shipping (reported, fail-forward violations)

- `bin/intent_main` -- dead second dispatcher, diverged (relative-path `chmod` bug at `:98`), zero runtime callers (F-ARCH-3/9).
- `bin/intent_minimal` -- alpha-versioned (`VERSION="2.0.0-alpha"`) Phase-1 stub, no caller (F-ARCH-4).
- `bin/stp` -- symlink preserving the retired STP command a year post-rebrand (F-UPG-10).
- `intent audit` -- Credo checks (R2/R6/R11) overlap rule-library critics with no cross-reference; two parallel Elixir engines (F-ARCH-5).
- Dead migration scaffolding: `needs_migration`, `show_migration_summary`, `count_migration_files`, `update_version_in_frontmatter`, second backup `create_project_backup` (F-UPG-9).
- Orphan templates: entire `lib/templates/eng/tpd/` set (heredoc used instead), `lib/templates/usr/_user_guide.md` (F-TPL-6/7).

### T7 -- MODULES.md registry drift (reported, Highlander-registry integrity)

- Three live subagents on disk (`intent`, `diogenes`, `socrates`) unregistered in MODULES.md, defeating the check-before-you-create gate (F-ARCH-8).
- `needs_v2_9_0_upgrade` registry row describes config-file-reading behaviour the function (which takes a version arg) does not have, with a stale `.intent/` path (F-DOCS-13).

### T8 -- `intent upgrade` needs an architectural rethink (design assessment)

The upgrade-rethink dimension recommends Architecture B: converge on a declared end-state ("make it so", probing on-disk state) with an ordered structural-step ledger reserved for genuine one-way transforms. Rationale, grounded in the code:

- Of 18 v2.x migration steps, ~11 are pure version stamps; only the v2.10 relocation is a genuine structural project mutation. The chain is ~80% ceremony.
- The code already mistrusts its own stamps: three recent predicates were retrofitted to probe observable state "regardless of stamp" (`intent_upgrade:100-110`, `intent_helpers:1033-1037,1070-1073`). `intent_claude_upgrade` already is a convergent diagnose/plan/apply installer.
- F-UPG-3 (HIGH, confirmed): mid-chain steps stamp the live target version, not their step version (`intent_helpers:1442-1450` et al). An interrupted chain claims success and a re-run skips the rest, including the relocation -- permanent silent half-migration.
- F-UPG-1/2 (HIGH/MEDIUM): the dispatcher hard-fails on unknown/future versions (eg 2.12.0) after backup + directory mutation has begun; `needs_*` globs enumerate `2.12.*..2.19.*` literally.
- Canon files have three writers (`create_claude_md`, `canon_refresh_with_user_section`, ad-hoc sed) and the stamp has two (jq vs sed) -- Highlander violation at the centre of the subsystem.
- Recommendation: `intent_upgrade` = orchestrator + sole stamper; `intent_claude_upgrade` = sole canon engine (strip its `VERSION_BUMP`); migration code moves out of `intent_helpers` (parsed by all ~25 commands, used only by upgrade) into upgrade-only scope; numeric semver comparison replaces glob enumeration; downgrade attempts hard-error before mutation; all checks before any mutation; verified backup. Sized L. Candidate for its own steel thread given blast radius.

### T9 -- Canon docs drift (confirmed/reported)

- F-DOCS-2 (HIGH, confirmed): `usage-rules.md:55,338` mandates `intent st cancel` and forbids manual `status:` edits, but `bin/intent_st` has no `cancel` dispatch case -- no compliant way to cancel a thread. Needs a decision: add a thin `cancel` verb or correct the docs.
- F-DOCS-1 (HIGH): `usage-rules.md:22-23,339` still documents the pre-v2.10 `.intent/config.json` path.
- F-DOCS-3 (HIGH): the "Session hook architecture" section of `working-with-llms.md:199-237,254,466,483` documents hook scripts (`lib/hooks/session_start.sh` etc.) and a `matchers` shape that have never existed; the real template uses `matcher` + `.claude/scripts/{session-context,require-in-session}.sh`. The documented strict->soft escape hatch is unexecutable.
- F-DOCS-6 (HIGH): `critics.md:152-162` describes filesystem-probe language dispatch that ST0037 removed; the real `in-review` reads the `languages` array.
- F-DOCS-5 (MEDIUM): `working-with-llms.md:529` references `intent claude skills status`, which does not exist.
- Lower: README claims v2.6.0 and undercounts the skill/test surface by ~half (F-DOCS-9); `in-whiteboard` missing from the `usage-rules.md` skills table (F-DOCS-14); CLAUDE.md says v2.11.0 (F-DOCS-10); `working-with-llms.md` stamp says v2.9.1 (F-DOCS-11); `rules.md` says nine required sections, validator enforces seven (F-DOCS-16); stale ST paths now under `COMPLETED/` (F-DOCS-15); `writing-extensions.md` promises unshipped v2.10 features (F-DOCS-12); README lists nonexistent `intent/usr/` and misplaces AGENTS.md (F-DOCS-8).

### T10 -- Test-suite quality (confirmed/reported; primary MFIC evidence)

- F-TEST-1 (HIGH, confirmed): two `intent_upgrade_dispatcher.bats` tests (stamp test `:48`, unknown-version test `:86`) run `intent upgrade` with no fake HOME, so the upgrade tail-call (`intent_upgrade:427-441`) overwrites the developer's real `~/.claude` skills/agents on every suite run. Fix folds with F-TEST-9 (promote the fake-HOME helper into `test_helper.bash`).
- F-TEST-4/5/6 (MEDIUM/LOW): whole files assert on heredoc constants defined inside themselves (`critic_report_format.bats`), test a test-local reimplementation of dispatch logic (`critic_dispatch.bats`), or only verify the host YAML parser works (`critic_config.bats`) -- green regardless of product behaviour.
- F-TEST-3 (MEDIUM): six modules exercised by no test (`intent_llm`, `intent_organise`, `intent_minimal`, `intent_main`, `stp`, `intent_claude_prime`).
- F-TEST-11/12 (LOW): a permanently-`skip`ped cross-FS test; a dead `intent init` invocation with no assertion.

### T11 -- BSD-only `sed -i ''` breaks Linux upgrades (reported MEDIUM)

The canon installer's three in-place seds use `sed -i ''` (BSD-only); on GNU sed every canon file install/refresh errors. The rest of the codebase uses the portable `sed -i.bak`. Linux is a supported target (`intent_doctor:419-420` ships Linux install instructions). (`intent_claude_upgrade:231,1051,1056`; F-UPG-6/F-TPL-2.)

### T12 -- CLAUDE.md historical-date corruption (reported MEDIUM, known backlog item)

The `VERSION_BUMP` step runs an unanchored global sed over CLAUDE.md (`intent_claude_upgrade:1056`), rewriting every `Intent vX.Y.Z` including historical prose ("migrated to Intent v2.0.0 on 2025-07-16"). Dies as a side effect of the T8 rethink (VERSION_BUMP removed), or fixable standalone by anchoring to the footer line.

## Proposed work packages

Sequenced; sizes are t-shirt. The review gate (user) selects and orders before any `intent wp new`.

| #   | WP                                             | Themes       | Size | Notes                                                                 |
| --- | ---------------------------------------------- | ------------ | ---- | --------------------------------------------------------------------- |
| 1   | Eliminate config eval (RCE)                    | T1           | S    | Highest priority; security. Replace eval with field-wise jq reads.    |
| 2   | Finish rules/docs-path drift + add mech. guard | T2           | M    | Nine skills + `[[LANG]]` + doc refs; guard test so it can't regress.  |
| 3   | Kill "reports success while doing nothing"     | T3           | S    | st new legacy, agents init, intent_init agent path, backup message.   |
| 4   | Fix AGENTS.md generation (paths + sections)    | T4           | S    | PROJECT_ROOT->HOME, validator sections, template/heredoc Highlander.  |
| 5   | Highlander consolidation pass                  | T5           | M    | Status-normalisation live bug first; version fallback; resolvers.     |
| 6   | Prune dead/legacy code                         | T6           | S    | intent_main, intent_minimal, stp, dead scaffolding, orphan templates. |
| 7   | Reconcile MODULES.md registry                  | T7           | XS   | Register 3 subagents; fix stale rows.                                 |
| 8   | Rethink `intent upgrade` (Architecture B)      | T8, T11, T12 | L    | SPLIT OUT to ST0043 (2026-06-11). Subsumes BSD-sed + date-rewrite.    |
| 9   | Canon docs reconciliation                      | T9           | M    | Includes adding `intent st cancel <ID>` (decided 2026-06-11).         |
| 10  | Test-suite hardening                           | T10          | M    | Real-HOME fix + shared helper; replace vacuous tests; cover gaps.     |

Gate decisions taken (2026-06-11):

- WP8 `intent upgrade` rethink -> spun out to **ST0043** (design-level, dwarfs the rest). Its full design lives in ST0043; T8/T11/T12 evidence here is the input.
- WP9 `st cancel` -> **add the command.** `intent st cancel <ID>` does what it says: moves the thread to CANCELLED (status + relocation), consistent with the existing Cancelled-status discipline. The docs already promise it; the WP makes the docs true rather than the reverse.

Gate decisions taken (2026-06-11, execution session):

- `intent audit` (T6) -- **retire.** Two parallel Elixir rule engines is the Highlander violation the audit was hunting; the rule-library critics are the canonical engine. Removal executes in WP-06.
- WP6 scope -- excludes upgrade-subsystem dead scaffolding (F-UPG-9); ST0043 owns all upgrade deletions so that code is touched once.
- Release cadence -- patch after the ST0042 WPs complete (ships `f359917` + fixes); ST0043 then targets a minor (v2.12.0).

WP directory mapping: slate WP1-7 = `WP/01`-`07`; slate WP8 = ST0043 (no dir); slate WP9 = `WP/08`; slate WP10 = `WP/09`. Execution order: WP-09 part A (test isolation) first, then 01, 05 (live bug first), 03, 04, 05 remainder, 02, 07, 08, 06, 09 part B.

## Alternatives Considered

- Multi-agent Workflow orchestration (adversarial verify panels per finding) -- declined for this pass; user opted for the default fan-out with reviewer-then-verify in main context. Revisit if finding volume overwhelms single-context verification.
