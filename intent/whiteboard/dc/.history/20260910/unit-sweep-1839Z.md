# dc -- `tests/unit/` attribution sweep, 2026-09-10 18:39Z

Driven at HEAD after the freeze lifted, against the DEFAULT `INTENT_BIN`
(`test_helper.bash:21` -> `bin/intent`, which is v2's shell dispatcher).

**1518 tests, 216 failing, 186 citing the pruned dispatcher, 30 OTHER.**
Zero cite a missing plugin script -- `d8a8c070` changed the text exactly as predicted.

| file | failing | pruned-door | other |
| --- | --- | --- | --- |
| `agent_commands.bats` | 50 | 50 | 0 |
| `skills_commands.bats` | 39 | 39 | 0 |
| `test_diogenes.bats` | 19 | 19 | 0 |
| `ext_discovery.bats` | 12 | 11 | 1 |
| `test_autopsy.bats` | 12 | 12 | 0 |
| `daemon_commands.bats` | 10 | 0 | 10 |
| `rule_validator.bats` | 9 | 9 | 0 |
| `rule_index.bats` | 8 | 8 | 0 |
| `config_undefined.bats` | 5 | 0 | 5 |
| `intent_upgrade_orchestrator.bats` | 5 | 5 | 0 |
| `au_language_code_guard.bats` | 3 | 3 | 0 |
| `co_language_code_guard.bats` | 3 | 3 | 0 |
| `intent_upgrade_dispatcher.bats` | 3 | 3 | 0 |
| `plugin_commands.bats` | 3 | 0 | 3 |
| `pr_language_code_guard.bats` | 3 | 3 | 0 |
| `rule_pack_rust.bats` | 3 | 2 | 1 |
| `claude_prime.bats` | 2 | 2 | 0 |
| `no_absolute_home_paths.bats` | 2 | 0 | 2 |
| `release_sidecars.bats` | 2 | 0 | 2 |
| `rule_pack_agnostic.bats` | 2 | 2 | 0 |
| `rule_pack_author.bats` | 2 | 2 | 0 |
| `rule_pack_content.bats` | 2 | 2 | 0 |
| `rule_pack_elixir.bats` | 2 | 2 | 0 |
| `rule_pack_lua.bats` | 2 | 2 | 0 |
| `rule_pack_prose.bats` | 2 | 2 | 0 |
| `rule_pack_shell.bats` | 2 | 2 | 0 |
| `rule_pack_swift.bats` | 2 | 2 | 0 |
| `ambient_project_root_guard.bats` | 1 | 1 | 0 |
| `claude_md_template.bats` | 1 | 0 | 1 |
| `config.bats` | 1 | 0 | 1 |
| `devbin_seal_disagreement.bats` | 1 | 0 | 1 |
| `intent_critic.bats` | 1 | 0 | 1 |
| `release_script.bats` | 1 | 0 | 1 |
| `wp_commands.bats` | 1 | 0 | 1 |
| **TOTAL** | **216** | **186** | **30** |

## 15 of the 30 OTHER are pointed at the WRONG BINARY, not defects

`daemon_commands.bats` fails 10 of 10 on the default, `daemon --help` included.
Ruled out as mine before reporting: `daemon)` case count is ZERO in both
revisions of `bin/intent`, so v2 never handled `daemon` and `d8a8c070` touched
only the `claude)` case.

    default INTENT_BIN (= bin/intent, v2)  ->  10 of 10 FAILING
    INTENT_BIN = target/release/intent     ->  10 of 10 GREEN

`config_undefined.bats` is the same shape (5 on default, 0 against v3). That is
15 of the 30.

**THE REMAINING 15 ARE NOT ATTRIBUTED AND NO NUMBER IS OFFERED FOR THEM.**
Several get WORSE against v3 -- `wp_commands` 1 -> 29, `config` 1 -> 5,
`plugin_commands` 3 -> 6 -- so those are correctly-wired v2 files and pointing
them at v3 is the wrong move. The suite is a BLEND of v2- and v3-targeted files
with no per-file declaration of which binary each expects.

**BEARS ON `AC-06.1`:** if the burn baseline was taken with the default
`INTENT_BIN`, the v3-targeted files contributed a full burn for a reason that is
not parity.

