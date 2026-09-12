---
verblock: "2026-09-12:v1.45: vc - DOING and TODO only, hv's rule restated in this file; landed work struck; the silent-deletion class ruled into v3.0.2"
intent_version: 3.0.1
---

# Work In Progress -- v3.0.2

**This file is DOING and TODO only. What landed lives in the commit, the CHANGELOG and `intent/done.md`; what is open lives in `intent issues list` and `intent ac gate <ST>`. A line that says something is done does not belong here, whoever needs it for context.**

## DOING: the v3.0.2 cut

hv ruled the release on 2026-09-12: a patch, vc directing. Rules for every item: own commit, red before green, a private worktree under an isolated HOME using the worktree's own in-tree target dir (an out-of-tree CARGO_TARGET_DIR puts the binary where no `lib/templates/` sits above it and reds the install-root tests, a false red ic and vc have both paid for), no release, no push, no hardcoded counts in anything a reader reads. A shared aggregator file (`tests/suite.rs`, `CHANGELOG.md`, this file) is read with `git diff <file>` before `git add`, because `--only` is path-scoped and takes every hunk a peer left in it; a `mod` registration and the file it names land in one commit (d4d7aec2b carried ic's registration for a file only ic had, and HEAD could not build its test target until 5746e02e). An edit to `surface/dispatch-table.json` regenerates `dispatch-table.md` through `gen_dispatch_table.sh` in the same commit, or the skew guard refuses it; the register keeps its own formatting and is never run through prettier.

- **Batch 3, critic truth (cc), remaining:** `--format zzz` refused; a shellcheck-refused file not reported as `ran` clean; `disabled:` parses its documented form. Any `--help` or output change is reported to ic in the same breath.
- **Batch 4, silent deletion (hv ruled 2026-09-12: _"this cannot be released publicly with that kind of bug"_):** every path that removes or overwrites bytes on disk without naming them before it acts is fixed before the cut. The sweep and its triage are in `intent/history/20260912-silent-deletion-sweep.md`. Owners: cc has organize's plan-then-apply (the trigger), `--quiet`'s dropped lines, the MCP `organize` door, `st hydrate` refusing a differing view, the `edit` realiser, `st dehydrate` naming first, and the gate test `no_removal_is_unannounced.rs`; dc has `init` refusing over existing files, the `.claude/settings.json` hold, `subagents sync --dry-run` and its help, and `uninstall` naming removed paths; ic has the projection's `rewrote:` note and the thread-close warning. The split is vc's under the pen; hv may overrule it.
- **The suite red that is ic's:** `no_pm_state_in_output::no_installed_payload_file_cites_intents_own_tracker` (ST0052 cited in `in-review/SKILL.md`, ST0001 in the intent subagent), own commit, red first, dc told when it lands. ic also lands the one-field emphasis edit in `surface/dispatch-table.json` so the reference generator's pages can be byte-stable.
- **The preflight gate's daemon pair is a watcher defect, not timing** (dc traced it to the line, 2026-09-12): macOS coalesces a leaf write under `intent/.cache/` into a directory event on the `intent` dir; `Scanned::includes` returns true for that bare directory (sync.rs near 519); the watcher publishes `fileChanged` naming a directory, against D20 and AC-08.6, and triggers an ingest on the daemon's own store write, the loop watch.rs:18-24 claims `.cache` in the skip list prevents. Ruled by vc: a bare directory is not a member of the scope; a directory event is answered by reconciling its subtree against `file_index` at the leaf level, publishing one event per file that differs and ingesting only when there are any. dc repairs it, own commit; cc's WP-18 widens the watcher on top of it. The tests stay as written.
- **CHANGELOG (dc):** one Fixed line per landed item; known-defects entries for fixed items say fixed in 3.0.2.
- **dc's second `--dry-run` rehearsal** on the final HEAD, in a clone with `intent backup` taken deliberately and the report saying so (a clone lacks the gitignored `intent/.backup/` by construction, so the first run could never pass doctor's backup gate). It runs after batches 3 and 4, ic's red, and hv's hydrate. Every gate line reported.
- **The cut** is hv's go, with the release-confirm question (tty, or an explicit `--no-confirm`) asked as its own question at that point. After the tag: ic's one reference regeneration, both halves keyed to v3.0.2, one commit; the tap formula publish as its own action on hv's approval.

## DOING: ST0069, project search and the LLM boundary (hv's go 2026-09-12, vc directing in specification, coordination and validation mode)

- The design and build plan are `intent/st/ST0069/design.md`; the packages are WP-17 to WP-24 with their criteria. Ships in 3.0.2 (hv). Each package lands on main as it closes; the second dry-run rehearsal and the cut follow the last package; batch 4 finishes first on every lane. Lanes: cc the engine (WP-18, WP-20 integration, WP-23); dc the daemon and install (WP-22, WP-24 hooks and canon, the grammar-size measurement); ic the surfaces (WP-17, WP-19, WP-21, WP-24 verbs, descriptions and skills). First up: WP-17 (ic) and WP-18 (cc) in parallel. hv rules the design's listed decisions as each comes due.

## TODO: waiting on hv

- **View-skew rot, a structural candidate for the next batch:** doctor compares the generator banner, so every cut leaves every untouched view skewed until something re-renders it, and the next preflight refuses again. Either doctor ignores the version stamp when comparing, or the stamp step re-renders every realised view, the schema-faces shape.
- **`organise --apply`, two defects found by hv on 2026-09-12 that batch 4 does not cover:** a refusal that is the same paragraph many times over is one finding printed many times; the tree is left half-dehydrated, views kept and attachments gone. The third, naming removals only after, is batch 4's trigger. Also from the same hour: a doctor finding at `--scope all`, ST0073's AC-05.1 is keyed to a WP-05 the thread lacks; the fix is a re-key through the CLI on a closed thread (a thread-level AC-00.n carrying the same text and evidence, AC-05.1 withdrawn naming it), which is hv's to say go on.
- **Surfaced at the 3.0.1 cut, not worked:** a stranger's `brew tap` may refuse our tap as untrusted, seen on Homebrew main with an empty trust store and not measured on stable (the install docs may need `brew trust --tap matthewsinclair/intent`); Linux and Intel macOS binaries are not built (size L); CI did not run for 3.0.1 (waived); `set <ac> kind non-test` leaves state `computed`, illegal for non-test, and only withdraw-then-reinstate moves it.
- **The rulings gate and dehydration collide** (found twice on 2026-09-12): a ratified ruling cites a thread file as its record, `rulings_check.sh` resolves the record on disk, and a completed thread hv's hygiene rule unlists takes its file off the disk. ST0058 and ST0066 stay realised until the gate resolves a dehydrated thread's record through the canon extract, or the two records cite canon paths. Small, code, hv's to rule.
- **A registered test module whose file is not in the commit** builds nowhere but on the machine that has it (found 2026-09-12). A pre-commit arm that refuses a `suite.rs` naming a `#[path]` the index does not carry is small and is not batch 4's class. hv's to rule; cc's if ruled.
- **Tests that read this repository instead of a fixture** red whenever the estate is mid-flight and blame whoever is committing (dc and vc, 2026-09-12): `lifecycle_verbs_edit_the_list` and `facade_acceptance::a_no_op_scope_change_is_reported_rather_than_written_twice` name `intent/.canon/st/ST0056.json` and `intent/st/ST0056/**` in their failures. Green again once the estate was consistent, so the class stands unfixed. Each such test moves to a fixture tree; hv's to rule, cc's if ruled.
- **ST0056 stays open on AC-00.5 and AC-11.1**, WP-11 with it: a `brew install` on a Mac that has never seen this repository, then the `intent` and `intentd` lifecycle, then both satisfied by evidence.
- **`0177`** (medium): `ext` ships the creating half without the undoing half; `config`, `ext` and `learn` ship declared-and-unbuilt (hv, 2026-08-31).

- **Out of the 3.0.x line by ruling:** ST0057, ST0060 (vault), ST0070 (LLM config).

## TODO: defects found by the doc audit, unruled

Each is described as built in the docs. None is worked until hv rules, and a fixed one leaves this list. The bold tag is the item's stable name; the numbers are not consecutive because the fixed ones are gone.

- **(2)** **`intent init` does not keep the store out of git** (vc). There's no `.gitignore` for `intent/.cache/`, so `git add .` stages `intent.db`, which D34 says never enters history.
- **(3)** **`st done` ignores work-package status** (vc). A thread closed with a `not-started` WP; the gate is over criteria only.
- **(4)** **`at new --covers <missing AC>` refuses correctly, but with a PUT/POST remedy** unrelated to the error (vc).
- **(6)** **`intent upgrade` reports a whiteboard "still on disk" in a project that has none** (dc). `sync.rs:201-213` prints the static NOT_YET_BUILT list from `sync.rs:168` without checking for it.
- **(7)** **The app-not-installed remedy names `bin/devbin`, which a Homebrew user does not have** (dc, from reading the code at `macapp.rs:178`). Their route is `Intent.app.zip` from the release.
- **(8)** **The critic, from cc's engine lane:**
  - In a mixed proxy block, the refused lines are dropped silently (`critic.rs:841-846`).
  - Usage errors exit 1, which the gate reads as findings, while handler errors exit 2 (`exit_codes.rs:195-206`, dc's to rule).
- **(9)** **Rule proxies that contradict their own rule** (cc):
  - `test-highlander-shared-setup` and `real-code-over-mocks` fire on the pattern their Good prescribes.
  - `no-silent-failures` is single-line and never matches its own multi-line Bad.
  - `no-parse-ls` claims only SC2012 and misses SC2045, SC2011 and SC2010.
  - Every swift and lua rule is undeclared (`critic.rs:855-872`).
  - The gate lints the library's own Bad examples, so `strong-assertions/bad_test.exs` can't be committed without `--no-verify`.
- **(10)** **Parity tools that still point at the deleted v2 estate** (cc):
  - `read_claim_probe.sh:45` defaults to `bin/intent`.
  - `coverage_map.sh` refuses at HEAD.
  - `fixture_probe.sh`'s canaries are deleted.
  - `of_n_population.sh:249-252` contradicts `runner_roster_check.sh:255`.
- **(11)** **Dead artefacts held in place by code or an open ruling** (cc; each needs hv):
  - `lib/templates/hooks/module_check_hook.json`: nothing reads it, but a roster row in `exit_code_consumers.rs:128` names it.
  - `lib/templates/hooks/critic-guard.sh` is parked and has drifted: it lacks 0242's zero-scope report. Retire it, or re-roster it after porting that report.
  - The `rules/_schema/index-generator.md`, `rules/index.json` and `.template` trio sit behind the unwired `claude rules index`, whose retirement is pending hv. `index.json` is also wrong.
  - `lib/templates/llm/{_ARCHETYPES,_DECISION_TREE,_DEPENDENCY_GRAPH,_MODULES}.md` and `lib/templates/prj/st/ST####/{acceptance,design,impl,tasks}.md` are dead, and still embedded through `init.rs` DESTINATIONS rows. Deleting them is code.
  - **Stale comments in test and code files**, which a doc sweep can't reach without touching code (cc, listed in `817fa950b`):
  - hv-banned counts in bats and Rust test headers;
  - `guard_dispatch.bats:41` says pre-commit.sh is copied;
  - `schema_faces_drift.rs:6,39` name a test target that is now a module of `suite`;
  - `rules_path_guard.bats:20-21` grep paths that are gone, so they pass vacuously;
  - organize.rs, tui.rs, mcp_stdio.rs, plugins.rs and the Cargo.toml descriptions.
  - `intent/plugins/claude/subagents/.manifest/global-agents.json` (ic): no v3 code reads it, it has drifted, and its checksums are empty. But three bats tests assert it (critic_prose, rule_pack_shell, highlander_audit), so deleting it removes their assertions.
- **(12)** **The interface, from ic's lane:**
  - `claude skills|subagents|rules|ws --help` show blank subcommand descriptions: `values` is a bare `Vec<String>` (`dispatch.rs:345`), and `spine.rs:530` sets no `.about()`.
  - `guide.rs:156` renders a hardcoded count into the agent guide, and its module doc pins more.
  - `st show` doesn't display the objective.
  - `cost-metrics.sh` uses `--` as Elixir's comment prefix, and `tca-report.sh` has an unemitted `DEDUP_RATE` beside its dead checkbox guard.
- **(14)** **Driven by dc on the keg:**
  - `at new` fails in the argument order its usage prints, because the `--covers` variadic swallows the ids.
  - A project stamped below v2.19.0 is told to `install intent@2`, which no tap has.
  - A pre-v2.10 project (a top-level `.intent/`) is told "no Intent project found" (`project.rs:866`).
- **(15)** **Reported by dc's agents, not yet re-driven:**
  - `intent edit <ac address> --path` returns info.md, but the criterion renders in acceptance.md.
  - The `edit wp` remedy names `intent wp`, which has no body writer.
  - The `edit` kind refusal offers `issue`, then refuses it.
  - `intent set ... acceptance exempt` succeeds, though `transitions.rs` declares the field Immutable.
  - A losing concurrent write surfaces a raw `sqlite: database is locked`.
  - The generated info.md Acceptance paragraph routes readers to hand-edit canon plus `sync --to-store`, as if the verbs did not exist.
- **(16)** **CI, Swift and skill scripts** (dc):
  - `tests.yml` puts `bin/` on PATH, never the built `intent`, and its shellcheck selector `find bin -name "intent*"` matches nothing. `pr-checks.yml` watches `bin/` for source changes.
  - `IntentCLI.swift:192-194` reads stdout to EOF before stderr, so it can deadlock. `DaemonService.swift:69` sequences a restart that `daemon restart` ships.
  - `tail-orphan-probe.sh:135,182` reports a false LEAKED in its guarded arm.
  - `in-tca-init/scripts/tca-init.sh` makes WP directories the store never registers, and `tca-report.sh:128-144`'s guard can never fire.
- **(18)** **A seeded `usage-rules.md` carries a literal `[[PROJECT_NAME]]`** (cc). `canon.rs:350-364` copies `_usage-rules.md` raw, with no token substitution.
- **(19)** **The acceptance verbs enforce less than the docs promised** (vc's working-with-llms audit, driven):
  - `at green` from `to-write` succeeds. Red-first is an owed guard (`transitions.rs:666`).
  - `at na` accepts a test-backed row, and `at red|green` accept a non-test one.
  - `st done` and `wp done` close silently on a placeholder objective, though `model.rs:497` computes the condition and `/in-finish` promises the warning.
  - `.intent_critic.yml`'s `show_all` is read only by the subagent prompts.
- **(20)** **ST0057's projection, found auditing its design** (vc):
  - `organize --apply` and `st hydrate` report `hydrated:` for an opaque attachment and write nothing, because they pass only inline `text` (`organize.rs:577`, `:589`, `:1075`). The dehydration gate then fails on the working copy with a UTF-8 error and a misleading remedy.
  - D57-9's two-region manifest code is not removed (`intentfiles.rs:52`, `:135`, `:164`), and the parser still accepts a BEGIN/END pair.
  - Cross-project addresses parse but are refused by every door (`facade.rs:7235`, `:7385`, `:8071`, `:3072`), **while AC-07.6 reads green**. AC-03.1 likewise claims a working-copy round trip that fails for opaque attachments. **These are contract findings: a satisfied row the as-built does not meet.**
  - **ST0057 cannot close.** Its gate is BLOCKED only on AC-12.x and AC-13.x, whose WPs are both cancelled.
- **(21)** **ST0056, found auditing its design docs** (vc, `120c8b2ae`):
  - `todo.window_hours` is validated by doctor and applied by nothing (`doctor.rs:369`, `facade.rs:2289`).
  - The export bundle claims schema `intent/export@3.0`, which nothing publishes (`export.rs:42`).
  - The migrator's last line tells the user to commit, while AC-00.8 promises one visible commit (`render.rs:3558`).
  - `IN-RS-CODE-001` (critical) is enforced by nothing: the critic's clippy arm is a no-op (`critic.rs:987`).
  - `at new --status red|green|n-a` creates a row at a non-initial state; `st list --status tbc` is accepted; `AcceptanceTest.kind` is still marked Unbuilt.
  - `st/ST0056` is documented as an address and refused by the resolver; `intentd`'s `shell.html` carries its own palette.
- **(22)** **From batch 1's drives** (dc, not fixed):
  - `intent plugin list` on the keg answers `No plugins found.` while the dev tree lists `agents` and `claude`: `plugins.rs` reads `plugin.json` through a `read_dir` behind a function return, which no source scan follows, and `intent/plugins/agents/plugin.json` still does not ship.
  - The shim's other two remedies (a pointer resolving to a non-install; an install missing its gate) still say "reinstall Intent", which rewrites no pointer.
- **(23)** **The v2 exit tables in the dispatch register** (ic): its `as-observed` rows claim v3 reproduces v2's exits, and five of six sampled are false. The pages stop publishing them; re-measuring the register is size L+ and waits on hv.
