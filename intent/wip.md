---
verblock: "2026-09-11:v1.44: vc - globalfold after the doc audit: what waits on hv, and the defects the audit found. The audit's orders are in intent/history/20260911-doc-audit.md."
intent_version: 3.0.1
---

# Work In Progress -- after 3.0.1

## Waiting on hv

**hv's 2026-09-11 doc audit is done.** Every lane landed and was verified by vc. Its orders are verbatim in `intent/history/20260911-doc-audit.md`, and the work is the commits from `9a1455a0f` on. The delivered pair names `2c3a7d2d4`, built through the fixed promote guard (`bc696da63`), and rhadamanth's skills and subagents are resynced. Nothing is claimed or in flight.

1. **The push.** The audit's commits are on `main` and not pushed. Both remotes wait on hv's approval.
2. **The tap README** (`c0e6ed9`, in the local tap checkout). It is one commit ahead of the tap's origin and unpushed; publishing it is its own outward action.
3. **Rulings on the lists below.** None is worked until hv rules.

**The live state is the register, not this file.** `intent issues list` is what is open; `intent ac gate <ST>` is where a thread stands.

## Open

- **`0177`** (medium): `ext` ships the creating half without the undoing half. Post-cut, no owner (hv, 2026-09-11). `config`, `ext` and `learn` ship declared-and-unbuilt (hv, 2026-08-31).
- **ST0056 stays open on AC-00.5 and AC-11.1**, and WP-11 with it. Both ask for a `brew install` on a machine that has never seen this repository. rhadamanth has, so its cold tap install (`int macos smoke --reinstall`, green at formula `fc32170`) does not satisfy them. Needs a clean Mac: install from the tap, run `intent` and the `intentd` lifecycle, then satisfy both by evidence and close WP-11 and ST0056.
- **Out of 3.0.1 by ruling:** ST0057, ST0060 (vault), ST0069 (post-cut), ST0070 (LLM config).

## Surfaced to hv at the cut, not worked

Each is recorded in the named commit's message. None is on a list until hv rules.

1. **A version bump turns every realised view into doctor view-skew** (`36839061a`). The banner carries the tool version; the finding blames a hand edit; for an unlisted thread its only remedy, `st hydrate`, also pins it. `intent doctor --scope all` lists the banner-only views here, and dc reproduced it on a clean project upgraded from 3.0.0, so a user upgrading will see the same. The release preflight's doctor refuses until this is resolved.
2. **A stranger's `brew tap` may refuse our tap as untrusted** (`549031396`, dc). Seen on Homebrew main with an empty trust store; not measured on stable. If stable does it too, the install docs need `brew trust --tap matthewsinclair/intent` first.
3. **Linux and Intel macOS binaries are not built.** hv, 2026-09-11: _"I only care about macOS and Linux."_ The formula is macOS arm64 by `depends_on`, and taps cleanly everywhere. A Linux build, package and publish is size L.
4. **CI did not run for 3.0.1** (no credit; hv waived it). The rust.yml clippy-to-zero and the tests.yml bats-on-v3 changes are unexercised by CI. The local evidence is fmt, clippy -D warnings, and both suites green at `1409aff70`, plus the release preflight's suites at the tag.
5. **ST0073 AC-05.1 names a WP-05 the thread lacks.** `doctor --scope all` shows it; the thread is closed.
6. **`set <ac> kind non-test` leaves state `computed`**, illegal for non-test, and only withdraw-then-reinstate moves it (`7c40da0ab`).

## Found by the doc audit: code defects surfaced to hv, not fixed

The docs now describe each as built. None is worked until hv rules.

0. **THE v3.0.1 TAG SHIPPED A RED SUITE** (cc, confirmed by vc). `v3.0.1:schema/*` carries `INTENT_VER: 3.0.0` beside `VERSION` 3.0.1, so five schema tests fail at the tag (`schema_faces_drift`, `schema_command`, `schema_versioning`). The release stamps the version without re-blessing the published schema faces. The faces were re-stamped by their generator in the audit (`474a24c98`); **the release step that should have done it is the defect, and it is hv's to rule.**

1. **The 3.0.1 keg ships no subagents tree** (dc, confirmed by vc). `bin/.devbin/cmd/macos:177` SUPPORT_PATHS omits `intent/plugins/claude/subagents`. The coverage guard reads only literal joins, and `payload.rs:590-592` builds that root with a non-literal one. On a brew install, `intent claude subagents list` says `no subagents in this install`. **High: a shipped omission of the same class as 3.0.0's missing rule library.**
2. **`intent init` does not keep the store out of git** (vc). There's no `.gitignore` for `intent/.cache/`, so `git add .` stages `intent.db`, which D34 says never enters history.
3. **`st done` ignores work-package status** (vc). A thread closed with a `not-started` WP; the gate is over criteria only.
4. **`at new --covers <missing AC>` refuses correctly, but with a PUT/POST remedy** unrelated to the error (vc).
5. **The critic prints `ok:` and then refuses at rc=2 over an empty rule library** (dc). `render.rs:10437-10441` prints before the exit-code match at `:10272`.
6. **`intent upgrade` reports a whiteboard "still on disk" in a project that has none** (dc). `sync.rs:201-213` prints the static NOT_YET_BUILT list from `sync.rs:168` without checking for it.
7. **The app-not-installed remedy names `bin/devbin`, which a Homebrew user does not have** (dc, from reading the code at `macapp.rs:178`). Their route is `Intent.app.zip` from the release.
8. **The critic, from cc's engine lane:**
   - A shellcheck-armed rule on a `.zsh` file reports `ran`, clean, at rc 0 while shellcheck refused the file (`critic.rs:743-777`, `:833-836`). Driven with a bash control.
   - In a mixed proxy block, the refused lines are dropped silently (`critic.rs:841-846`).
   - `--format zzz` renders text at rc 0 instead of refusing (`render.rs:10243-10246`).
   - Usage errors exit 1, which the gate reads as findings, while handler errors exit 2 (`exit_codes.rs:195-206`, dc's to rule).
9. **Rule proxies that contradict their own rule** (cc):
   - `test-highlander-shared-setup` and `real-code-over-mocks` fire on the pattern their Good prescribes.
   - `no-silent-failures` is single-line and never matches its own multi-line Bad.
   - `no-parse-ls` claims only SC2012 and misses SC2045, SC2011 and SC2010.
   - Every swift and lua rule is undeclared (`critic.rs:855-872`).
   - The gate lints the library's own Bad examples, so `strong-assertions/bad_test.exs` can't be committed without `--no-verify`.
10. **Parity tools that still point at the deleted v2 estate** (cc):
    - `read_claim_probe.sh:45` defaults to `bin/intent`.
    - `coverage_map.sh` refuses at HEAD.
    - `fixture_probe.sh`'s canaries are deleted.
    - `of_n_population.sh:249-252` contradicts `runner_roster_check.sh:255`.
11. **Dead artefacts held in place by code or an open ruling** (cc; each needs hv):
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
12. **The interface, from ic's lane:**
    - `claude skills|subagents|rules|ws --help` show blank subcommand descriptions: `values` is a bare `Vec<String>` (`dispatch.rs:345`), and `spine.rs:530` sets no `.about()`.
    - `spine.rs:532` hangs family flags on every leaf, so `skills uninstall --help` advertises a `-f` it ignores.
    - `guide.rs:156` renders a hardcoded count into the agent guide, and its module doc pins more.
    - `st show` doesn't display the objective.
    - `cost-metrics.sh` uses `--` as Elixir's comment prefix, and `tca-report.sh` has an unemitted `DEDUP_RATE` beside its dead checkbox guard.
13. **`intent claude skills uninstall <name>` WITHOUT `--force` deletes a skill the user edited after install** (dc, re-driven by vc under an isolated HOME). It prints `removed (1 file(s))` at rc 0 and leaves an empty directory, and `--force`'s own help says it is the flag for "a skill that was changed here". **High: it loses user data.**
14. **Driven by dc on the keg:**
    - `at new` fails in the argument order its usage prints, because the `--covers` variadic swallows the ids.
    - A project stamped below v2.19.0 is told to `install intent@2`, which no tap has.
    - A pre-v2.10 project (a top-level `.intent/`) is told "no Intent project found" (`project.rs:866`).
15. **Reported by dc's agents, not yet re-driven:**
    - `intent edit <ac address> --path` returns info.md, but the criterion renders in acceptance.md.
    - The `edit wp` remedy names `intent wp`, which has no body writer.
    - The `edit` kind refusal offers `issue`, then refuses it.
    - `intent set ... acceptance exempt` succeeds, though `transitions.rs` declares the field Immutable.
    - A losing concurrent write surfaces a raw `sqlite: database is locked`.
    - The generated info.md Acceptance paragraph routes readers to hand-edit canon plus `sync --to-store`, as if the verbs did not exist.
16. **CI, Swift and skill scripts** (dc):
    - `tests.yml` puts `bin/` on PATH, never the built `intent`, and its shellcheck selector `find bin -name "intent*"` matches nothing. `pr-checks.yml` watches `bin/` for source changes.
    - `IntentCLI.swift:192-194` reads stdout to EOF before stderr, so it can deadlock. `DaemonService.swift:69` sequences a restart that `daemon restart` ships.
    - `tail-orphan-probe.sh:135,182` reports a false LEAKED in its guarded arm.
    - `in-tca-init/scripts/tca-init.sh` makes WP directories the store never registers, and `tca-report.sh:128-144`'s guard can never fire.
17. **`.intent_critic.yml`'s `disabled:` list disables nothing in its documented form** (cc, driven). `critic.rs:579-593` `parse_disabled` enters block mode only when nothing follows `disabled:`. The form every doc showed, `disabled:  # comment` followed by `- ID`, yields `"disabled": []` with the rule still armed. The docs now put the comment on its own line.
18. **A seeded `usage-rules.md` carries a literal `[[PROJECT_NAME]]`** (cc). `canon.rs:350-364` copies `_usage-rules.md` raw, with no token substitution.
19. **The acceptance verbs enforce less than the docs promised** (vc's working-with-llms audit, driven):
    - `at green` from `to-write` succeeds. Red-first is an owed guard (`transitions.rs:666`).
    - `at na` accepts a test-backed row, and `at red|green` accept a non-test one.
    - `st done` and `wp done` close silently on a placeholder objective, though `model.rs:497` computes the condition and `/in-finish` promises the warning.
    - `.intent_critic.yml`'s `show_all` is read only by the subagent prompts.
20. **ST0057's projection, found auditing its design** (vc):
    - `organize --apply` and `st hydrate` report `hydrated:` for an opaque attachment and write nothing, because they pass only inline `text` (`organize.rs:577`, `:589`, `:1075`). The dehydration gate then fails on the working copy with a UTF-8 error and a misleading remedy.
    - D57-9's two-region manifest code is not removed (`intentfiles.rs:52`, `:135`, `:164`), and the parser still accepts a BEGIN/END pair.
    - Cross-project addresses parse but are refused by every door (`facade.rs:7235`, `:7385`, `:8071`, `:3072`), **while AC-07.6 reads green**. AC-03.1 likewise claims a working-copy round trip that fails for opaque attachments. **These are contract findings: a satisfied row the as-built does not meet.**
    - **ST0057 cannot close.** Its gate is BLOCKED only on AC-12.x and AC-13.x, whose WPs are both cancelled.
21. **ST0056, found auditing its design docs** (vc, `120c8b2ae`):
    - `todo.window_hours` is validated by doctor and applied by nothing (`doctor.rs:369`, `facade.rs:2289`).
    - The export bundle claims schema `intent/export@3.0`, which nothing publishes (`export.rs:42`).
    - The migrator's last line tells the user to commit, while AC-00.8 promises one visible commit (`render.rs:3558`).
    - `IN-RS-CODE-001` (critical) is enforced by nothing: the critic's clippy arm is a no-op (`critic.rs:987`).
    - The shim's absent-pointer remedy says "reinstall Intent", but only `intent bootstrap` writes the pointer, and the formula has no `post_install`. So after a fresh install, every commit is refused with the wrong fix.
    - `at new --status red|green|n-a` creates a row at a non-initial state; `st list --status tbc` is accepted; `AcceptanceTest.kind` is still marked Unbuilt.
    - `st/ST0056` is documented as an address and refused by the resolver; `intentd`'s `shell.html` carries its own palette.
22. **The v2 exit tables in the dispatch register** (ic): its `as-observed` rows claim v3 reproduces v2's exits, and five of six sampled are false. The pages stop publishing them; re-measuring the register is size L+ and waits on hv.
