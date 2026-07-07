# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.16.0] - 2026-07-08

Minor release adding the **`content` (web-content) project-type pack** (ST0053) and the **`IN-PR-*` shared prose base** it is built on. `content` is the second prose discipline after `author`; both now stand on one shared mechanical prose surface rather than duplicating it. It is a minor, not a patch, because it adds a new project-type surface (and a shared base pack); it is strictly opt-in, with zero behaviour change for projects that do not declare `content`.

### Added

- **The `IN-PR-*` prose base (ST0053).** The mechanical prose-hygiene rules that every prose discipline shares -- banned filler and `eg`-not-`e.g.`, no vanity metrics, heading hygiene, and the mechanical trope pass -- now live once in a `prose` base pack (`intent claude rules list --lang prose`), not copied per discipline. Introduces the `PR` language code.
- **The `content` rule pack (ST0053).** Six web-distinct `IN-CO-*` rules in two tiers: `style` (mechanical) -- page meta present (title / description / canonical), image alt-text, descriptive link text; and `craft` (judgment) -- scannability and web voice, one clear primary call to action, reading level matched to the audience. Enumerate with `intent claude rules list --lang content`. Introduces the `CO` language code.
- **`intent lang init content` (ST0053).** Installs `intent/llm/RULES-content.md` + `ARCHITECTURE-content.md` (web-content information architecture -- pages / posts layout, page front-matter, the content review pipeline), appends the Language Packs entry, and adds `content` to config `languages`. `intent lang list` now enumerates `content`.
- **`/in-content-essentials` skill (ST0053).** The content pipeline -- draft, mechanical detrope, revise for craft, structural check, CTA / reading-level pass -- loaded by `/in-session` when `content` is declared. `/in-review` dispatches `content -> critic-prose`.

### Changed

- **`critic-author` is renamed to `critic-prose` (ST0053).** One prose critic now serves every prose discipline, parameterised by the declared language: it loads the `IN-PR-*` base plus whichever of `author` / `content` the project declares. The two-form detrope is preserved, re-anchored to the base's `IN-PR-STYLE-004` mechanical pass. Projects on 2.15.0 that dispatched `critic-author` should use `critic-prose`.
- **The `author` pack refactored onto the prose base (ST0053).** The four shared mechanical rules moved from `IN-AU-STYLE-*` into `IN-PR-STYLE-*` (with migration aliases); the `author` pack now owns only its discipline-specific rules -- front-matter / objectives plus the four craft rules. No behaviour change for an author project: the same rules apply, sourced from the base plus the discipline pack.
- **`IN-PR-STYLE-001` no longer bans `overall` (ST0053).** The `docs/blog` dogfood confirmed that `overall` hits are the legitimate adjective sense ("overall progress"), matching a 2026-04 autopsy of Intent's own corpus. The banned filler is now `absolutely` only; `overall` is explicitly documented as not-a-tell. Rule content version 1 -> 2.

## [2.15.1] - 2026-07-07

### Fixed

- **`intent st list`, `intent st sync`, and `intent wp list` now share one table renderer.** Each sized its table by a rule of its own: `wp list` hard-capped `Title` at 30 columns (truncating every title, eg `Interpretation schema: verb...`), and `st sync` re-invoked `list` at a fixed `dft_width` while `st list` used the terminal, so the two rendered differently for identical data. They now render through a single `render_table` (`bin/intent_helpers`) that fills the terminal width (or an explicit `--width`), with content-fit as the floor so nothing is ever truncated. `st sync` composes `st list`, so their output is byte-identical.

## [2.15.0] - 2026-07-03

Minor release adding the **`author` project-type pack** (ST0052) -- the first non-code discipline on Intent's `languages` axis. A project declaring `languages: [author]` gets an authoring rule pack, a prose critic, canon templates, and an essentials skill, all activated the same way a code language is. It is a minor, not a patch, because it adds a new project-type surface; it is strictly opt-in, with zero behaviour change for projects that do not declare `author`.

### Added

- **The `author` rule pack (ST0052).** Nine `IN-AU-*` rules in two tiers: `style` (mechanical, greppable -- banned filler and `eg`-not-`e.g.`, no vanity metrics, front-matter + learning objectives, heading hygiene, and a mechanical trope pass) and `craft` (judgment / critic-as-reader -- voice and register consistency, cross-chapter continuity, full `/in-detrope` diagnosis, citation and attribution). Enumerate with `intent claude rules list --lang author`. The pack introduces the `AU` language code across the rule-id validator and enumeration.
- **`critic-author` subagent (ST0052).** The first non-code rule-library critic (prose + courseware). Read-only; two modes -- `review` (the mechanical `style` tier, default) and `craft-check` (the judgment `craft` tier, on instruction). It wires detrope in two forms without forking the trope catalogue: the mechanical trope pass runs by default off `in-detrope`'s `detection: automated` regexes, and the full `/in-detrope` diagnosis is emitted as a handoff recommendation, never invoked by the critic.
- **`intent lang init author` (ST0052).** Installs `intent/llm/RULES-author.md` + `ARCHITECTURE-author.md` (book / course information architecture -- parts, chapters or modules, learning objectives, the authoring pipeline), appends the Language Packs entry, and adds `author` to config `languages` -- exactly like a code language. `intent lang list` now enumerates `author`.
- **`/in-author-essentials` skill (ST0052).** The authoring pipeline -- outline, draft, mechanical detrope, revise, structural check -- loaded by `/in-session` when `author` is declared. `/in-review` dispatches `author -> critic-author`; in an author-only project no code critic runs, and a mixed project (eg `[elixir, author]`) runs each critic on its own subtree.

### Notes

- The headless pre-commit prose gate is deliberately deferred: `.md` extension alone cannot route a file to the author pack, and Intent's own `--` house style trips the trope catalogue's dash-overuse regex, so a headless gate needs a confirmation / suppression layer first. `critic-author` is on-demand (`Task`) only in this release.

## [2.14.0] - 2026-07-02

Minor release adding **`intent todo`** — a flat DOING / TODO / DONE view of every steel thread and work package, projected from real `status:` so it cannot drift (ST0050) — plus a generated-file width fix (ST0051). It is a minor, not a patch, because it adds a new command surface.

### Added

- **`intent todo` — a projected DOING / TODO / DONE board (ST0050).** `intent/todo.md` is a nested GFM checklist bucketed by real status: DOING (`WIP` threads + their work packages), TODO (`Not Started`), DONE (recent completions). Every checkbox is derived from the unit's `status:` and its status-directory placement — there is no separately-stored state, so the file cannot drift from `intent/st/**`. `intent todo` / `todo list` prints it (generating on first use); `todo update` regenerates it. Mutation verbs change _real_ status by wrapping `intent st` / `intent wp` and regenerating — `todo done` / `notdone` / `toggle` — so `todo done` inherits the ST0048 acceptance close-gate (a BLOCKED contract is refused, never bypassed). `intent todo --json` emits the board as keyed-by-bucket JSON (each thread carrying its work packages) for export to other systems.
- **DONE flush / prune + ISO completion timestamps (ST0050).** The DONE bucket is watermarked — `## DONE:<T>`, where `<T>` is the last-flush instant — and lists completions at or after it. `intent todo done --flush` advances `<T>` (clearing the view without touching the record in `COMPLETED/`); `intent todo done --prune` emits the pruned items to stdout (for archiving, eg `>> intent/done.md`) and then flushes. `intent st done` now stamps `completed:` as an ISO 8601 UTC timestamp for exact flush ordering; a legacy `%Y%m%d` stamp is still tolerated everywhere `completed:` is read.
- **`dft_width` config field (ST0051).** A new `intent/.config/config.json` field (default `120`) sets the width for generated files; `intent init` seeds it.

### Fixed

- **Generated `steel_threads.md` no longer truncates at 80 columns (ST0051).** `intent st sync --write` hard-coded an 80-column width, clipping the slug column of the generated index. Generated files now size to `dft_width` (config, default 120); interactive stdout stays at the terminal width; an explicit `--width N` overrides both.

## [2.13.1] - 2026-06-29

Patch release hardening the acceptance close-gate (ST0048). The gate behind `intent st done` / `intent wp done` previously treated a unit with **zero acceptance criteria** -- or no `acceptance.md` at all -- as vacuously done, so work closed with nothing to verify it against. That is now a hard failure: an empty or missing contract is refused, with an explicit `acceptance: exempt` marker as the sole escape. No-Silent-Errors applied to the acceptance layer.

**Behaviour change (read before upgrading):** any in-flight ST/WP that never authored acceptance criteria, or that has no `acceptance.md`, will stop closing until it authors criteria or is marked `acceptance: exempt`. Migration: `docs/releases/2.13.1/RELEASE_NOTES.md`.

### Fixed

- **The close-gate no longer passes an empty or missing contract (ST0048).** `intent ac gate` -- the authority behind `st done` / `wp done` -- now exits non-zero with a BLOCKED report when a present `acceptance.md` has zero in-scope ACs, or when `acceptance.md` is absent. This closes the vacuous-green hole where "every in-scope AC satisfied" was trivially true of zero ACs. Unsatisfied-AC and malformed-line blocking are unchanged.

### Added

- **`acceptance: exempt` frontmatter marker (ST0048).** The sole, explicit, visible escape from the hardened gate: a deliberately AC-free unit (eg a pure content / authorial task) declares `acceptance: exempt` in its `acceptance.md` frontmatter and the gate passes, announcing the exemption (never inferred from emptiness). The default -- no marker -- is enforced. WP scope is WP-lenient: a WP with no own ACs rolls up to the ST boundary as long as the thread carries a contract.

### Changed

- **Canon and consumer comments describe fail-by-default.** `intent/docs/working-with-llms.md` D11, the `bin/intent_st` / `bin/intent_wp` close-gate comments, and the stamped `acceptance.md` template now state that the gate fails an empty or missing contract; the retired "opt-in / legacy-safe / closes exactly as before" framing is removed and pinned out by a grep guard in `tests/unit/acceptance_close_gate.bats`.

## [2.13.0] - 2026-06-25

Minor release adding **`intent claude start` and `intent claude ws`** -- the MAAC (multi-agent agentic coding) whiteboard launcher and workstream lifecycle (ST0047). One command provisions whiteboard workstreams (the Protocol 3.0 nodes), launches a Claude Code session bound to one with the verified effort / permission / context, and manages the node lifecycle. The capability was pioneered by convention in Lamplight (the operational reference), productised as the MVP in Baize, and is now first-class in Intent, served centrally to every project from `$INTENT_HOME`. It is a minor, not a patch, because it adds a new command surface across the fleet.

### Added

- **`intent claude start <ws>` (ST0047).** Launches an interactive Claude Code session bound to a whiteboard workstream: composes the node identity + the project `.claude/restart.md` + a standing "show a daily plan, then wait" instruction, seeds `/in-session` (admitted by the in-session gate's slash-exemption, which chains `/in-whiteboard pickup`), and execs `claude --effort max --permission-mode auto --append-system-prompt ...`. Provisions the workstream first if absent (prompted). `CWI_DRY_RUN` prints the assembled argv instead of launching.
- **`intent claude ws new|list|archive|hygiene` (ST0047).** The deterministic workstream lifecycle that complements the Claude-driven `/in-whiteboard` skill: `ws new` scaffolds a Protocol 3.0 node (`wip.md` frontmatter, `.history/.gitkeep`, bidirectional `_(empty)_` inboxes with every existing peer; `hv` is Workstream Zero, working nodes are made to order); `ws list` reads the roster from frontmatter (read-only); `ws archive` retires a node into `.archived/` keeping its `.history/`; `ws hygiene` runs a mechanical structural lint (warns on oversized boards + stale heartbeats; never archives DOING content -- the semantic archive stays the Claude-driven `/in-whiteboard archive`). `CWI_WB` overrides the whiteboard root.
- **`intent/plugins/claude/bin/intent_claude_cwi`** -- the command's home in the `intent claude` plugin family. It resolves the current project via `find_project_root` (not the tool home), so it is served centrally and available in every project with no per-project install. Behavioural ATs in `tests/unit/claude_with_intent.bats` drive it through the real dispatch via the `CWI_WB` / `CWI_DRY_RUN` seams.
- **A live `intent/whiteboard/` for Intent itself** -- `hv` + `cc` + `vc` (no interface node: Intent is CLI plus data, not UX) plus a roster README, so Intent now dogfoods MAAC on its own development.

### Changed

- **The `/in-whiteboard` skill defers to the script for scaffolding (Highlander SSOT).** Its "Scaffolding a node" prose now points at `intent claude ws new`, and the skill's lazy-inbox wording ("never pre-seeded") was reconciled to the script's eager bidirectional pre-seed (ratified by the WP-01 acceptance + the Baize golden board); `ask` / `announce` keep on-demand inbox creation as a self-healing fallback for hand-added nodes. The Baize `bin/claude_with_intent` prototype is retired in favour of the central command (Highlander -- no divergent second copy).

## [2.12.0] - 2026-06-15

Minor release landing two steel threads. **ST0043** rewrites `intent upgrade` from a 524-line version-case ladder into a ~150-line convergent orchestrator and removes every migration path below the v2.9.0 fleet floor (fail-forward). **ST0045** formalises the Whiteboard Protocol 3.0 rewrite (per-node directories + single-writer inboxes + the `hv` hypervisor node) with an AC/AT contract and closes the reference-vs-skill drift. It is a minor, not a patch, because ST0043 changes upgrade behaviour for every project. Two close-gate hardening fixes ride along.

### Changed

- **`intent upgrade` is now a convergent orchestrator (ST0043).** `bin/intent_upgrade` rewritten to: detect state -> semver sanity before any mutation (error on missing/unparseable version; refuse a downgrade; refuse below the v2.9.0 floor; a future/unknown version no longer hard-fails with "Unknown version") -> verified backup -> walk a state-probed ledger (`LEDGER="relocate_config languages_field"`, dispatched by the `step_<id>_needs/_run/_verify` naming convention) -> one delegation to `intent claude upgrade --apply` -> stamp the target once, last. Applicability is decided by each step probing on-disk state, so an interrupted upgrade re-run does exactly the remaining work.
- **Single version stamper.** `intent_upgrade` is the sole writer of `intent_version` (jq, once, last). `intent/plugins/claude/bin/intent_claude_upgrade` is the sole canon engine; its `VERSION_BUMP` action and both version `sed` stamps are removed, and `canon_substitute_placeholders` is rewritten off BSD `sed -i ''` to a portable `sed > tmp && mv` so canon substitution works on Linux.
- **Whiteboard Protocol 3.0 is the documented model (ST0045).** The `in-whiteboard` skill, the `in-session` / `in-finish` chaining steps, and the "Multi-session coordination" section of `intent/docs/working-with-llms.md` now describe per-node directories with single-writer `wip.md` + per-sender `inbox.<sender>.md`, the `hv` hypervisor node, and `announce`-based shared-platform coordination, replacing the retired 2.0 flat-file model (shared `asks.md`, per-stream files, a shared platform file).

### Added

- **`bin/intent_migrations` (ST0043)** -- a new upgrade-only file (sourced only by `intent_upgrade`) holding the two ledger-step trios plus `intent_relocate_dotintent` (moved verbatim from helpers). No step writes the version.
- **Whiteboard 3.0 skill completeness (ST0045)** -- the `in-whiteboard` skill now specifies inbox-file init (`# inbox: <sender> -> <recipient>` header + single-writer note + `_(empty)_` sentinel + creation on first `ask`), `.history/.gitkeep` scaffolding, the `hv` node variant (human-driven, `session_id` optional/`none`, advisory heartbeat, `## Standing directives`), and the message-entry format (required vs recommended fields).
- **Mechanical guards** -- `tests/unit/intent_upgrade_orchestrator.bats` + `intent_migrations_{relocate,languages}.bats` (ST0043); `tests/unit/whiteboard_protocol_3_guard.bats` (ST0045, pins out any live 2.0 reference); AT-01.8 in `intent_claude_upgrade.bats` (canon-engine portability + no `VERSION_BUMP`).

### Removed

- **All migration code below the v2.9.0 fleet floor (ST0043, fail-forward).** `bin/intent_helpers` shrank 2026 -> 369 lines: every `migrate_v*_to_*` (v0 through v2.8.2->v2.9.0), every `needs_v2_*` predicate, the pre-v2 YAML/JSON/structure converters, and the migration ceremony helpers (`show_migration_summary`, `count_migration_files`, `create_project_backup`, `needs_migration`, ...). `detect_project_version` (+ `detect_stp_version`) stays shared. The only upgrade-time `~/.intent/ext/` bootstrap went with `migrate_v2_8_2_to_v2_9_0` (verified safe: `intent ext` creates the dir on demand; the fleet is already v2.9.0+). Deleted tests: `ext_migration.bats`, `migrate_v2_9_0_to_v2_10_0.bats`, `migrate_v2_10_x_to_v2_11_0.bats`, and the `create_project_backup` test.

### Fixed

- **The acceptance close-gate no longer drops malformed contract lines (F1).** An AC/AT line that failed the strict numeric grammar (eg a letter-group id like `AC-U.1`) was silently dropped, which could make a gate vacuously pass. `bin/intent_acceptance` now detects malformed lines on every read path and blocks the gate loudly. (F6, a proposal to block `st done` when `acceptance.md` is absent, was deliberately declined: a thread with no contract has not opted into the AC regime, so the gate stays open -- opt-in by presence, unchanged for legacy threads.)

## [2.11.14] - 2026-06-14

Patch fixing `intent organize` on Linux. The command tallied moves with `((counter++))` under `set -e`; in bash, `((x++))` returns exit status 1 when `x` is 0 (post-increment yields the old value), and bash 5.x's `set -e` acts on that, aborting the script after the first thread. macOS bash 3.2 is lenient in that loop/case context, so the break stayed invisible behind macOS-green CI from v2.11.12 (when `intent organize` was resurrected) through v2.11.13 -- Ubuntu CI had been red the whole time. The defect class is converted to `x=$((x + 1))` (an assignment always returns 0) and pinned shut by a guard test.

### Fixed

- **`intent organize` no longer exits 1 on Linux** (or any modern-bash host). It had been moving only the first thread and then dying. The three `((moved_count++))` / `((kept_count++))` increments in `bin/intent_organize`, plus three latent same-class sites in `bin/intent_helpers` (the frontmatter parser's `((line_num++))`, which starts at 0 every call, and two `[ -f ] && ((count++))` legacy-config tallies), now use `x=$((x + 1))`. A new guard test (`tests/unit/set_e_increment_guard.bats`) greps `bin/` and `scripts/` for naked `((x++))` / `((x--))` and fails on any hit -- closing the class that macOS-only CI green had masked for four releases.

## [2.11.13] - 2026-06-14

Patch shipping ST0044: `acceptance.md` becomes the fifth default steel-thread document, and with it an Acceptance-Criteria / Acceptance-Test process that makes "done" an externally-verified event rather than a self-asserted checkbox. It ships as a patch on opt-in-by-presence grounds -- the close-gate and the `acceptance.md` contract are inert for any thread that does not adopt them, so behaviour is unchanged for non-adopting projects (the basis on which ST0040's whiteboard also shipped as a patch). The one non-additive change is `intent st edit`, which now prints a path rather than launching an editor. The thread was dogfooded on itself: ST0044's own build ran through the five-step with an independent verifier, and the thread closed through the very gate it introduced.

### Added

- **`acceptance.md` as a default steel-thread document.** `intent st new` stamps it alongside info / design / impl / tasks, via the existing `lib/templates/prj/st/ST####/*.md` glob -- no seam, default on. The template carries the acceptance contract: the AC section (ST-level + per-WP) and the AT section, with example lines indented under guidance so a freshly stamped thread carries no live column-0 entries and cannot self-gate-block. `info.md` and `WP/info.md` reference it and restate no ACs (one home, Highlander); `intent st show` learns the `acceptance` type (named and in `all`).

- **`intent ac` and `intent at` -- the acceptance contract CLI.** An **AC** (Acceptance Criterion) is a ratified completeness boundary; an **AT** (Acceptance Test) is a small red-to-green test that proves a slice of it. `ac list` / `ac status` / `ac satisfy` / `ac gate`; `at list` / `at red` / `at green` / `at na`, with `done` / `notdone` aliasing green / red. The AT state machine is `to-write -> red -> green` (+ `n/a`), and green is reachable only from red -- a test cannot claim proof without first having failed. A test-backed AC is satisfied by computation (iff a covering AT is green), never by hand; a non-test AC carries inline evidence and is signed off by the verifier. The grammar is column-0 and bash-3.2-greppable; all reads and writes target `acceptance.md` alone.

- **The acceptance close-gate.** `intent ac gate <stid>[/NN]` is the single authority on whether a thread or work package may close. `intent st done` and `intent wp done` consult it and refuse on BLOCKED, with the verdict computed from the coverage map rather than read from a hand-ticked box. It is opt-in and legacy-safe: a thread with no directory, no `acceptance.md`, or zero in-scope ACs exits 0, so existing threads close exactly as before.

- **The five-step process, documented.** `intent/docs/working-with-llms.md` D11 is the canon home: the AC / AT axes, the five-step (verifier ratifies ACs -> builder writes red-first ATs -> verifier witnesses RED -> builder builds to green -> repeat), the open-gate and close-gate, and the lifecycle mapping. Light pointers reference it from `/in-plan` (open-gate), `/in-verify` (red-first + witness RED), and `/in-finish` (close-gate), each referencing D11 and none restating it.

### Changed

- **`intent st edit <id> [type]` prints a path instead of launching an editor.** It now validates the file type and echoes the file's absolute path for every doc type (info / design / impl / tasks / acceptance), replacing the macOS `open` / `$EDITOR` launch. This is the one non-additive change in the release: scripting `intent st edit` to open a file must now pipe the path to an editor explicitly.

## [2.11.12] - 2026-06-11

Patch shipping the full ST0042 arc: a Fable 5 architectural review of the Intent codebase (run as the first deliberate MFIC exercise, ST0041) followed by execution of all nine work packages it produced. Theme: architectural integrity -- Highlander consolidation, no-silent-errors enforcement, canon-vs-reality reconciliation, dead-surface pruning, and a test suite that can actually refute the product. Includes one small addition (`intent st cancel`) and a set of removals (dead dispatchers, the retired `intent audit`).

### Fixed

- **`intent organize` dispatches and `intent llm usage_rules` displays again** (found adding ST0042 T10 coverage). The organize script was named `intent_organise` while every referencing surface (help text, `intent st organize`) uses the "organize" spelling, so the top-level command had errored with "Unknown command" since the dispatcher's `intent_$COMMAND` default arm was introduced -- the script is renamed to match. `intent llm usage_rules` still read the retired `intent/llm/usage-rules.md` location and errored on every invocation since the v2.10.0 root-canon move -- it now reads the root `usage-rules.md`.

- **Critic test files assert on real product behaviour** (ST0042 T10). `critic_report_format.bats` asserted on a heredoc fixture defined inside itself, `critic_dispatch.bats` tested a test-local reimplementation of the dispatch logic, and `critic_config.bats` only verified the host YAML parser works -- all three were green regardless of what the product did. They now drive the real surfaces: the headless runner's text/JSON reports and exit codes (via `--rules` with synthetic rules), the shipped pre-commit hook's per-language dispatch / fail-open / blocking behaviour (stub `intent` on PATH recording invocations), and the runner's `disabled:` + the hook's `severity_min:` consumption of `.intent_critic.yml`. The permanently-skipped cross-FS migration test (which skipped unconditionally on every platform) is deleted; behavioural coverage added for the previously untested `intent llm`, `intent organize`, and `intent claude prime` -- which is what surfaced the two dead commands above.

### Removed

- **Dead and legacy surfaces pruned** (ST0042 T6, fail-forward). `bin/intent_main` (dead second dispatcher, diverged from the real one, zero callers), `bin/intent_minimal` (alpha-versioned Phase-1 stub), and the `bin/stp` symlink (the retired STP name, a year post-rebrand) are deleted. `intent audit` is retired: its custom-Credo checks ran in parallel with the rule-library critics as a second Elixir enforcement engine (Highlander violation); `intent critic <lang>` is the canonical engine. The credo-check templates themselves survive -- `intent st zero` (brownfield retrofit) still installs them. Orphan template sets `lib/templates/eng/tpd/` and `lib/templates/usr/_user_guide.md` (no generator reads either) are deleted, and `intent help`'s footer no longer points at `docs/user_guide.md` / `docs/reference_guide.md`, which never shipped. MODULES.md, help text, README, and tests updated; `intent modules check` reports a clean registry.

### Added

- **`intent st cancel <ID>`.** The docs have mandated it as the compliant cancellation path since the status discipline landed, but the command never existed -- cancelling a thread meant manually editing `status:` frontmatter, which the same docs forbid. The new dispatch case mirrors `done`: stamps `status: Cancelled` (frontmatter + body), relocates the thread directory to `intent/st/CANCELLED/`, and updates the index (ST0042 T9 / F-DOCS-2).

### Fixed

- **Canon docs describe the as-built system again** (ST0042 T9). `working-with-llms.md`'s session-hook section showed a settings shape (`matchers` arrays, `$INTENT_HOME/lib/hooks/*.sh` scripts, a soft/strict script pair) that never shipped -- it now shows the real template (`matcher` string, `lib/templates/.claude/scripts/`, echo-based Stop hook) and an executable softening path instead of the phantom `user_prompt_submit_soft.sh`. `critics.md`'s `/in-review` dispatch table described filesystem-marker probing removed by ST0037; it now documents the `languages`-array dispatch the skill actually performs. The phantom `intent claude skills status` reference, the pre-v2.10 `.intent/config.json` path in `usage-rules.md`, stale `intent/st/ST0035|ST0040/` paths (now under `COMPLETED/`), `rules.md`'s "nine required sections" (the validator requires seven), README's v2.6.0 claim and nonexistent `intent/usr/` entry, `writing-extensions.md`'s "v2.10 will enforce" promises, and CLAUDE.md's v2.11.0 stamp are all corrected; `/in-whiteboard` joins the `usage-rules.md` skills table. Guard tests in `docs_completeness.bats` pin the phantom command, hook-key, hook-script, and legacy-path classes.

- **`intent modules check` honours `file::function` registry rows and the registry matches the filesystem again** (ST0042 T7, Highlander gate). Three live advisory subagents (`intent`, `socrates`, `diogenes`) were absent from MODULES.md; they now have rows. The `needs_v2_9_0_upgrade` row described config-file-reading behaviour the function (which takes a version-string argument) never had, and cited the pre-v2.10 `.intent/` path -- corrected. A dangling row pointed at a credo check (`dependency_graph.ex`) that does not exist, and the credo-check row listed stale `R2/R6/...` identifiers instead of the actual filenames -- both fixed; the `_default/AGENTS.md` row left over after that template's deletion is removed. Finally, `intent modules check` itself tested each `file::function` row as a literal path, so every function-qualified entry reported as permanently stale and eroded trust in the gate; the check now splits on `::` and verifies the function is defined in the file.

- **Rules-path drift finished off, with a guard so it cannot return** (ST0042 T2). v2.11.11 fixed the generated AGENTS.md/CLAUDE.md and the critic subagents but missed nine canon skills (`in-session` -- auto-loaded every session in every fleet project -- plus `in-standards`, `in-review`, `in-ash-ecto-essentials`, `in-elixir-essentials`, `in-elixir-testing`, and the three `in-tca-*` skills), all still steering agents to the dead local `intent/plugins/claude/rules/` path; all nine now route through `intent claude rules list` / `show`. The `[[LANG]]` placeholder in `_usage-rules.md`, which no installer substitutes and which shipped verbatim into consumers, is replaced with language-agnostic wording (the pre-commit line now also names the real `intent critic <lang>` entry point instead of an install-local `bin/intent_critic` path). Install-resident doc references (`working-with-llms.md`, `critics.md`, `rules.md`, etc.) in generated AGENTS.md/CLAUDE.md and the elixir template are now qualified "at the Intent install", matching `_usage-rules.md`'s already-correct form; the installed `.intent_critic.yml` no longer points at a sample file consumers don't have. A mechanical guard (`rules_path_guard.bats`) greps every propagated/generated surface for the dead path and for unsubstitutable `[[...]]` placeholders.

- **One definition per shared concern across the CLI** (ST0042 T5, Highlander). The `get_intent_version` fallback literal was repeated at ~20 call sites with drift (`2.2.1` / `2.3.x` / `2.6.0` / `2.8.x` / `2.9.0` / `2.11.0`), so a broken install reported a different stale version depending on which script was asked -- the fallback now lives in `get_intent_version` alone (with a warning to stderr, since a missing VERSION file means a broken install). Config-field parsing (three divergent implementations: jq field-read, grep/cut, grep|sed) consolidates on `read_config_field`; `find_project_root` (three copies, one with a latent walk-up bug in the pre-commit hook template) on the `bin/intent_helpers` original; the steel-thread directory resolver (three copies) on a shared `resolve_st_dir`; and the `~/.intent/ext` root walk (five inline expansions with divergent `INTENT_EXT_DISABLE` handling) on `ext_root_dir` / `ext_enumerate_names`. All shared primitives live in `bin/intent_helpers` and are registered in MODULES.md; mechanical guard tests pin the version-fallback and ext-root patterns against reintroduction.

- **Generated AGENTS.md lists the skills and subagents that are actually installed.** The Installed Skills / Installed Subagents section renderers read only `$PROJECT_ROOT/.claude/`, a location the installers never write (they install user-globally to `$HOME/.claude/`), so the sections said "No skills installed" on a fully provisioned machine. Both renderers now read the project directory and `$HOME/.claude/`, project first, deduped by name (ST0042 T4). The static `_default` AGENTS.md template -- which lacked three of the four validator-required sections and self-described as auto-generated while diverging from the generator -- is deleted; `intent agents init --template _default` now uses the generated content, so a fresh `_default` install passes `intent agents validate` (one content source, per Highlander).

- **Commands no longer report success after silently doing nothing** (ST0042 T3, No-Silent-Errors). Four paths fixed: `intent st new` on a legacy file-structure project errors honestly instead of printing `created:` after a guarded copy from a nonexistent template; `intent agents init --template {rust,lua,shell,swift}` now actually creates the root `AGENTS.md` (generated content) instead of printing "Created" while copying nothing; `intent init`'s interactive agent install invokes the real `intent claude subagents install intent` dispatcher instead of a script path that does not exist; and `intent upgrade`'s backup verifies every copy and aborts before any migration on failure, instead of `|| true` followed by an unconditional "Backup created successfully".

- **`intent st repair` / `organize` normalise status to the stored canon.** Both commands carried inline status tables that mapped `wip` to `In Progress`, while the canon written by `st new`/`st start` and matched by the path resolver is `WIP` -- so a repair could rewrite a thread's frontmatter to a value the rest of the tool does not recognise. One synonym table (`canonical_status`) now feeds repair, organize, and the list-filter normaliser (ST0042 T5); regression test pins `repair` writing `status: WIP`.

- **Config values are loaded verbatim, never evaled.** `load_intent_config` built `key="value"` shell assignments from raw JSON values with jq and `eval`ed them, so a config value containing `$(...)`, backticks, or `$VAR` in `intent/.config/config.json` (or `~/.config/intent/config.json`) executed arbitrary shell on the next project-scoped `intent` command. Config fields are now read individually with `jq -r` and assigned directly (ST0042 T1); a regression test proves shell metacharacters in config values are inert.

- **Test suite no longer writes to the real `~/.claude`.** Two `intent_upgrade_dispatcher.bats` tests ran `intent upgrade` without HOME isolation, so the upgrade tail-call (skills + subagents sync) overwrote the developer's real `~/.claude` mirrors on every suite run. The fake-HOME pattern, previously copy-pasted across six test files with drift, is promoted to a single `setup_fake_home` / `teardown_fake_home` pair in `tests/lib/test_helper.bash` (ST0042 F-TEST-1/F-TEST-9); all seven files now use it. Verified: a full suite run leaves the real `~/.claude` untouched.
- **`intent st new` stamps the current Intent version.** New steel threads were created with a hardcoded `intent_version: 2.4.0` in their frontmatter (and `2.0.0` on the no-template fallback path) -- the values frozen into the template and heredoc when they were last hand-edited. Both creation paths now substitute the live version from `get_intent_version` (single source: `$INTENT_HOME/VERSION`), per Highlander. Regression test proves the stamp matches `VERSION` and that no unsubstituted placeholder survives.

## [2.11.11] - 2026-06-03

Patch fixing rules-path drift in the LLM guidance Intent generates for consuming projects. The generated `AGENTS.md` (via `intent agents sync`) and `CLAUDE.md` (from `lib/templates/llm/_CLAUDE.md`), plus the `critic-<lang>` subagents, told agents the coding-rule library lives at a local `intent/plugins/claude/rules/` path. That directory exists only inside the Intent tool itself; in a consuming project the rules are reachable solely through the CLI (`intent claude rules list` / `show`). A field `critic-elixir` run looked for the local directory, failed to find it, and fell back with a confusing "rule library not installed at the expected path" diagnostic, reviewing at reduced fidelity.

### Fixed

- **Generated guidance points at the CLI, not a local directory.** The `intent agents sync` generator, the `_CLAUDE.md` / `_usage-rules.md` templates, and the `_default` / per-language `AGENTS.md` + `RULES.md` agent templates now describe rule access as served by the installed Intent tool via `intent claude rules list` / `show <id>`, with no reference to a local rules directory that does not exist in consumers.
- **`critic-<lang>` subagents resolve rules through the CLI.** All five critics (`elixir`, `rust`, `swift`, `lua`, `shell`) now enumerate via `intent claude rules list` and read each rule with `intent claude rules show <id>`, partitioning code-vs-test mode by the `category` column. The CLI already merges canon and `~/.intent/ext/` rules with provenance and id-shadowing, so the per-critic extension-merge step was removed (one enumeration path, per Highlander). The `elixir-test-critic` upstream probe, which sits outside the CLI's reach, is retained.

### Changed

- **`intent upgrade` re-syncs installed subagents.** The upgrade path now runs `intent claude subagents sync` alongside the existing skills sync (failure-tolerant, no `--force`), so the corrected critics reach every machine's `~/.claude/agents/` mirror on the next upgrade rather than requiring a manual sync.

## [2.11.10] - 2026-05-28

Additive patch extending the `/in-whiteboard` skill with a stream-role vocabulary and an optional handle. Both are field-proven in Lamplight and generalised here so every Intent project inherits them. Opt-in like the rest of the protocol -- a project that declares no Verifier and uses no handles sees zero behaviour change.

### Added

- **Verifier stream role.** A new `## Stream roles` section documents an optional, advisory-only Verifier stream: the independent check that another stream's claimed/landed work is correct, complete, consistent, and faithful to what the user asked. It triangulates the _ask_ (the peer's Claude Code session transcript), the _plan_ (`~/.claude/plans/*.md`), and _reality_ (the whiteboard + `intent/st/**` + the code + the tests); fires on a "done" claim rather than continuously; reads the as-built with `file:line` evidence; classifies findings expected-vs-real; self-refutes high-severity findings before posting; and outputs to `asks.md` with direct escalation for a compounding false-"done". The Verifier never mutates another stream's code and never blocks its progress -- the user adjudicates, the owning stream fixes. It is a role a stream adopts, designated in the project's `whiteboard/README.md`, not a subcommand.
- **Recommended baseline operating model.** Streams and handles are per-project configuration declared in the project's own `whiteboard/README.md` -- any number of streams, any handles. The skill now recommends a baseline shape of one Control stream (heavy lifting) plus one Verifier stream (the independent check), with additional streams project-specific. The baseline is a recommendation, not a requirement; peer-only and other rosters remain valid.
- **Optional `handle:` stream-frontmatter field.** Short shorthand for terse asks-routing (eg `CC`, `VC`, `IC`). Additive: `stream_id` remains the routing key, so adding handles never breaks `pickup` or `asks`.

## [2.11.9] - 2026-05-23

Additive patch extending the `/in-whiteboard` skill with an `archive` subcommand. Field-tested by hand in Lamplight first (whose per-stream files had grown into append-only logs spanning ~a week and were costing real tokens on every `pickup`), then encoded into canon so the procedure is repeatable across all Intent projects. Opt-in by directory presence like the rest of the whiteboard protocol — projects without `intent/whiteboard/` see zero behaviour change.

### Added

- **`/in-whiteboard archive [as-of <YYYY-MM-DD>]`.** Rolls DONE/superseded content older than 2 days out of the live whiteboard files (`<stream>.md` + `asks.md` + the shared `<platform>.md`; never `README.md` or live ledgers) into weekly, Monday-anchored `history/<YYYYMMDD>.<file>` buckets keyed by the ISO week of the archived **content**, not the run date — so one run can append to several weekly buckets. It is judgment-guided, not a blind date filter: frontmatter, the current RESUME/STATUS block, standing reference, and any still-open item stay regardless of age; resolved asks, superseded status blocks, and absorbed decisions move. A one-line `> **[archived]** ...` pointer is left where content was removed. Concurrency-safe by construction: archive only your own stream file, or sweep all streams when peers are paused, and always commit via an explicit pathspec. History buckets are append-only and never reloaded on `pickup`; git history remains the ultimate trace.

## [2.11.8] - 2026-05-21

Patch fixing a multi-session deadlock in the `/in-session` UserPromptSubmit gate. With two or more Claude Code sessions open against the **same** Intent project, the gate blocked every prompt and `/in-session` never released it — the user was forced to manually `touch` the expected sentinel on every turn. Lamplight, which runs concurrent streams in one project, hit this constantly.

The cause was an asymmetric source of truth for "my session id". The gate (`require-in-session.sh`) read the real `session_id` from its hook payload and checked `/tmp/intent/in-session-<session_id>.sentinel`. The releaser (`release-gate.sh`, run by `/in-session`) had no payload, so it read the id from a shared per-project state file written by `SessionStart`. Concurrent sessions all wrote that one file, it held some other session's id, the releaser touched the wrong sentinel, and the gate deadlocked.

### Fixed

- **Single source of session identity.** Both the gate and the releaser now resolve identity from `$CLAUDE_CODE_SESSION_ID`, the env var Claude Code exports into every hook and Bash tool invocation. The two sides agree by construction, with no shared mutable file between them. When the env var is absent (an older Claude Code build) both sides degrade to the same `unknown` sentinel, which the releaser always touches, so they still agree and the gate self-heals. Concurrent sessions in one project are now fully supported.

### Removed

- **Shared per-project session-id state file.** `session-context.sh` (the `SessionStart` hook) no longer persists the session id to `/tmp/intent-claude-session-current-id-<key>`; that file was the concurrent-session corruption source and is no longer load-bearing. `release-gate.sh` drops its state-file and legacy-file reads. Stale copies left in `/tmp` are inert (never read) and need no cleanup.

## [2.11.7] - 2026-05-18

Additive patch shipping the multi-session coordination protocol designed in a parallel Lamplight session and live-tested in `/Users/matts/Devel/prj/Lamplight/intent/whiteboard/`. Two concurrent Claude Code sessions on the same Intent project now have a real live-channel between them instead of "wait for the other session's `wip.md` to update at next session-end". ST0040 captures the design, alternatives considered, and the deliberate deferrals; this release rolls it into formal canon.

The protocol is opt-in by presence: a project gains coordination only after it creates `intent/whiteboard/`. Projects without the directory see zero behaviour change — the chained `/in-whiteboard` step from `/in-session` and `/in-finish` skips silently.

### Added

- **`/in-whiteboard` skill** at `intent/plugins/claude/skills/in-whiteboard/`. Subcommand surface: `pickup` / `claim` / `unclaim` / `touch` / `ask` / `decide` / `lamplight` / `release` / `status`. Each session belongs to a durable **stream** (eg `control`, `ia-ux`) that owns one file under `intent/whiteboard/`; shared `asks.md` carries cross-stream handoffs; a per-project shared `<platform>.md` file (eg `lamplight.md`, `core.md`) carries shared-platform-layer edit notices. Claims are by steel-thread ID only — no glob paths. Heartbeat-older-than-7-days marks a claim reclaimable; reclaim requires explicit user acknowledgement.

- **Chain integration**: `/in-session` step 5 auto-fires `/in-whiteboard pickup` (after gate release); `/in-finish` step 1 auto-fires `/in-whiteboard release` (before any `wip.md` / `restart.md` / `done.md` updates). Both opt-in by directory presence.

- **`asks.md` header conventions** layered on top of the required `to:` / `from:` line: optional `Re: <prior-ask-anchor>` for reply threads, optional `FYI only -- no response needed.` to mark info-dump asks the recipient stream should not queue a reply to. Borrowed from the cross-project LLMsend protocol (in-whiteboard is the intra-project sibling).

### Changed

- **`intent upgrade` auto-installs `in-whiteboard` and re-syncs the canon skill mirror.** Two calls inserted after the migration dispatcher completes: `intent claude skills install in-whiteboard` (idempotent for users without the skill) and `intent claude skills sync` (propagates the `in-session` / `in-finish` chain edits into any already-installed mirror). Both calls failure-tolerant — a missing `~/.claude/` mirror or a user "N" at an overwrite prompt should not break the upgrade. No `--force` is used, so user customisations are never silently lost.

- **Upgrade "Next steps:" output** gains a one-line pointer to the new "Multi-session coordination" section of `intent/docs/working-with-llms.md` so users know how to opt in.

### Caveat

A Claude Code session already running at upgrade time has the old `in-session` / `in-finish` prose loaded in context — the new chain only auto-fires from the **next** `/in-session` (after `/compact` or session restart). Manual `/in-whiteboard pickup` still works in the current session. New sessions started after upgrade get the chain.

### Tests

- `tests/unit/skills_commands.bats` enumerates `in-whiteboard` in the canonical roster covered by the `claude skills list shows available skills` invariant.
- `tests/unit/intent_upgrade_dispatcher.bats` gains a regression case asserting that a v2.10.x → current-target upgrade lands `in-whiteboard` at `~/.claude/skills/in-whiteboard/SKILL.md`. The test fakes `$HOME` so the install writes into a sandbox.

### Documentation

- `intent/docs/working-with-llms.md` gains a "Multi-session coordination" section after "Skills and /in-session auto-load". Covers the live-channel-vs-snapshot tense/reader/cadence distinction, the file layout, stream identity discovery, ST-only claims, shared platform layer pattern, chain integration, heartbeat semantics, and a pointer to the Lamplight live reference and ST0040 design rationale.

## [2.11.6] - 2026-05-15

Additive patch shipping one new Lua coding rule surfaced during Lamplight ST0163 WP-04 (Murder mechanic hook authoring). The rule formalises an idiom matts called canon-worthy after seeing it applied to `worlds/v4/murder/experiences/murder_on_the_weekend/{phase,night_kill,facts}.lua`: "way more readable than loads of imperative if/then blocks."

### Added

- **IN-LU-CODE-006 — Dispatch table over if-chain for value dispatch**. Lua has no pattern matching and no multi-head function definitions; the idiomatic substitute is a table-of-functions keyed by the discriminating value, with a single lookup + invoke at the call site. The rule forbids `if/elseif` chains dispatching on a value to different downstream function calls and prescribes the `HANDLERS` table idiom instead. Guard clauses on derived booleans (alive checks, nil checks, invariant violations) stay as `if`. Concretises IN-AG-PFIC-001; sister rule IN-EX-CODE-001 (Elixir multi-head dispatch). Enforcement is via the `critic-lua` subagent (prose Detection); no Greppable proxy block, in line with the existing Lua-pack convention.

### Tests

- `tests/unit/rule_pack_lua.bats` registers the new rule in its canonical-id enumeration; the existing presence + count + validator + list invariants now cover IN-LU-CODE-006.
- `tests/fixtures/critics/lua/code/would-catch/sample.lua` gains a `perturbation.tag` dispatch chain so the would-catch fixture exercises the new rule; `manifest.txt` lists IN-LU-CODE-006.

## [2.11.5] - 2026-05-05

Behavioural patch fixing three latent bugs surfaced by a Conflab session 2026-05-05. All three were shipped-as-broken; the first two silently produced output that looked plausible while dropping load-bearing content; the third silently regressed a project's recorded version stamp.

### Fixed

- **`intent treeindex` reported "empty response from Claude" for every directory** when run inside any v2.10.0+ Intent project. Root cause: the spawned `claude -p` session inherits the project's `UserPromptSubmit` hooks; the strict gate (`require-in-session.sh`) fires on the first prompt, sees no `/in-session` sentinel for the ephemeral session_id, and exits 2; the non-bare `claude -p` swallows the hook's stderr and exits 0 with empty stdout. Treeindex saw empty stdout and reported per-directory failures. The treeindex tool was fine; the gate was the silent killer.

- **`intent agents generate` produced a stripped AGENTS.md** (empty project name, no language scaffolding, no installed-skill enumeration, no conditional resource links) when invoked directly via the dispatcher. Root cause: the `generate` dispatch path did not call `load_intent_config`, leaving `PROJECT_ROOT` empty so every per-project detection (`mix.exs`, `Cargo.toml`, `.claude/skills/`, `CLAUDE.md`, `usage-rules.md`) silently failed. `intent agents sync` was unaffected because it pre-loads config. Latent since the dispatcher was first added 2025-08-20; surfaced today when `generate` was invoked standalone for a diff repro.

- **`migrate_v2_10_x_to_v2_11_0` hard-coded the target stamp to `"2.11.0"`** instead of the live Intent target. A project walked up from v2.10.x through the migration path would land with `intent_version = "2.11.0"` regardless of which v2.11.x patch was current. Field impact was muted because `needs_v2_11_0_upgrade` short-circuits projects already carrying the `languages` field, but the bug existed and would have stamped v2.11.5 projects as "2.11.0". Fix stamps `get_intent_version`.

### Changed

- **`require-in-session.sh` accepts `INTENT_SKIP_IN_SESSION_GATE=1` as an explicit bypass.** Non-interactive automation has no chat surface for `/in-session` to run in, so the gate has no UX affordance for those sessions. The env var is the opt-out wrappers set; the gate short-circuits to exit 0 before any other check. Interactive sessions and untagged automation continue through the normal sentinel-based gate.

- **`bin/intent_treeindex` sets `INTENT_SKIP_IN_SESSION_GATE=1` on its `claude -p` invocation.** Treeindex is automation by definition; the bypass is unconditional.

- **`intent_agents_generate_content` self-loads project context.** The fix moves the `load_intent_config` + `require_project_root` guard from the dispatcher branches into the function itself, so any caller (dispatcher, `init`, `sync`, future automation) gets a consistent project context without duplicating the load preamble. Highlander applied: one source of truth for "this function needs `PROJECT_ROOT`."

### Documentation

- `intent/docs/working-with-llms.md` D7 documents the `INTENT_SKIP_IN_SESSION_GATE` bypass and adds it to the FAQ fix list.
- `intent help treeindex` lists the env var under a new `ENVIRONMENT` section so future `claude -p` wrapper authors can replicate the convention.

### Tests

- `tests/unit/require_in_session_gate.bats` covers the bypass branch and the existing slash-command / sentinel pass-throughs as regression smokes.
- `tests/unit/intent_agents.bats` gains a regression case asserting `intent agents generate` populates project name, language detection, and installed-skill enumeration when invoked with `PROJECT_ROOT` unset.
- `tests/unit/intent_upgrade_dispatcher.bats` gains a regression case asserting that a v2.10.x project upgraded via the migration path lands with the live target stamp (read from `VERSION`), not a hard-coded literal.

## [2.11.4] - 2026-04-30

Docs-only patch following v2.11.3's field verification.

### Documentation

- **Critic runner code locality** — `intent/docs/critics.md` "Headless runner" section gains a note clarifying that the runner (`bin/intent_critic` + `critic_runner.sh`) and the canon rule library load from `$INTENT_HOME` (the Intent install on `$PATH`), not from each project's plugin tree. A fix to the runner or a canon rule applies to every Intent project the moment Intent itself updates. Per-project canon refresh (`intent claude upgrade`) keeps `intent/llm/RULES*.md` and `.claude/skills/` copies in sync with the Intent version, but is not a prerequisite for gate behaviour to change. The behaviour itself is unchanged from v2.11.3 — only the docs are clarified.

### Verification

- v2.11.3's strict-proxy fix smoke-tested in Conflab (the canonical field witness) on 2026-04-30. Previously-misfiring patterns clear: `IN-EX-CODE-004` no longer flags single-step `case ... do`; `IN-EX-TEST-003` no longer flags compliant `use ExUnit.Case, async: true`. Gate still produces signal on real violations (e.g. `IN-EX-TEST-001` weak assertions, `IN-EX-TEST-005` control flow in tests, `IN-EX-TEST-007`). No stderr `note: skipping` diagnostics — expected, since the stripped rules no longer carry proxy blocks for the runner to refuse.

## [2.11.3] - 2026-04-29

ST0039 ships: pre-commit critic gate stops emitting findings derived from `Greppable proxy` regexes the headless runner cannot honour. Defect fix to behaviour shipped as broken in v2.11.0; no new feature surface, no schema change.

### Fixed

- **Pre-commit gate false positives on `IN-EX-CODE-004` (with-for-railway)**. Field report from a Conflab session post-upgrade-to-v2.11.0: the gate flagged every `case ... do` line in two LiveView files (22 false positives in a 3-file diff), forcing back-to-back `--no-verify` commits. Root cause: `bin/intent_critic`'s parser extracted only the first quoted regex from a multi-line proxy block and ran it as `grep -nE`, silently dropping the `| wc -l` qualifier that made the line a counter heuristic rather than a detector. The rule's _actual_ detection — "two or more nested fallible calls without `with`" — is body-confirmation territory and not expressible as a single-file regex.

- **Pre-commit gate false positive on `IN-EX-TEST-003` (async-by-default)**. Same Conflab session: the gate flagged the _compliant_ `use ExUnit.Case, async: true` line. Root cause: the documented proxy uses `grep -rnL ... | xargs grep -l ...` (find files lacking `, async: true`); the runner extracted the first quoted argument and ran it forward as `grep -nE`, matching the compliant form.

- **Runner silently degraded complex grep proxies** (pipes, `xargs`, `grep -L`, `grep -v`, awk, multi-line continuations). `critic_pattern_from_grep_command` consumed the first single-quoted argument from any proxy bash block and emitted findings as if the runner had executed the full pipeline. Replaced with a strict-proxy contract.

### Changed

- **`bin/intent_critic` runner contract** is now strict. `critic_runner.sh` accepts only proxy lines of the form `grep [-r|-n|-E|-rn|-rE|-nE|-rnE|--include=GLOB ...] '<pattern>' [<path>...]`: single grep invocation, no pipes, no `xargs`, no `-L` / `-v` / `-B` / `-A` / `-l` / `-c` / `-o` / `-w` / `-x`. Multi-line proxy blocks are accepted as a union of simple lines (findings deduped on `(line, content)`). Lines the runner cannot honour are refused with a once-per-rule stderr diagnostic `note: skipping <rule_id> (proxy not headless-runnable)`. Loud > silent.

- **`intent/docs/critics.md`** "Mechanical subset only" paragraph rewritten to document the strict-proxy contract and the stderr diagnostic.

### Removed

- **Greppable proxy blocks** stripped from 8 Elixir rules whose detection cannot be expressed as a simple single-file regex. The rules themselves are unchanged in prose and still apply via `/in-review` (LLM `critic-elixir` subagent does the body confirmation):
  - `IN-EX-TEST-003` (async-by-default) — inverse semantics.
  - `IN-EX-CODE-003` (impl-true-on-callbacks) — line continuation + negative filter.
  - `IN-EX-LV-001` (two-phase-mount) — `-B5` context + negative filter.
  - `IN-EX-LV-003` (thin-liveviews) — awk state machine for line counting.
  - `IN-EX-PHX-001` (thin-controllers) — awk state machine for line counting.
  - `IN-EX-ASH-001` (code-interfaces-only) — callsite scope cannot be inferred per-file.
  - `IN-EX-ASH-002` (actor-on-query) — proxy was inverted (fired on compliant code).
  - `IN-EX-TEST-004` (start-supervised) — required `grep -v start_supervised` filter.

- **`IN-EX-CODE-004` (with-for-railway)** counter line — the `case.*do$ | wc -l` heuristic is gone; the rule now ships only the legitimate `error -> error` forwarder detector. Single-step `case` blocks are no longer mechanically flagged.

- **`critic_pattern_from_grep_command`** function in `critic_runner.sh`. Replaced by `critic_proxy_is_simple` predicate + `critic_patterns_from_grep_block` walker. No back-compat shim (fail-forward).

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
