# The `bin/` prune census -- discovery narrative, 2026-08-31

Moved off the live board at the 09:31Z localfold. **The RULES stayed on the board; this is the reasoning that produced them, plus the numbers as they moved.** Nothing here is needed to act; everything here is needed to understand why the board says what it says.

Landed as `d42b4799`, `d64b40e6`, `c80867b1`.

## The order of discovery, which is the whole point

Each layer was found only because the one below it was already wrong. **No layer was reachable from the layer below by looking harder** -- each needed a different instrument aimed at a different object.

| layer           | dependency                                                                                                                                                                                         | how it was found                          |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| members         | 13 verdicted bats files                                                                                                                                                                            | per-arm property census (yesterday)       |
| **container 1** | `tests/run_tests.sh:24` sources `bin/intent_helpers` above every `@test`                                                                                                                           | reading the runner, not the tests         |
| **container 2** | `tests/lib/test_helper.bash:21` -- `INTENT_BIN="${INTENT_BIN:-${INTENT_BIN_DIR}/intent}"`, set NOWHERE in `run_tests.sh` or either CI workflow; 59 of 112 files reach it via `run_intent` (`:148`) | reading the helper the runner loads       |
| **container 3** | 147 direct `${INTENT_BIN_DIR}/intent_<sub>` sites in 5 files; 143 in `treeindex_commands.bats` (53 arms) and `fileindex_commands.bats` (47 arms)                                                   | a second instrument keyed on the VARIABLE |

**THE ESTATE HAD ALREADY DOCUMENTED CONTAINER 3 IN TWO PLACES.** `test_helper.bash:15-18` names it in prose -- the direct sites "bypass the dispatcher by design and have no single-binary equivalent" -- and `intent_bin_retarget_guard.bats`'s third arm exists precisely to prove the guard's needle does NOT match them, citing "~146 classified sites". My census rediscovered by measurement a thing the container documented about itself. **Read the container's own header before censusing it.**

## The three instruments, and the two that were wrong

1. **Path-keyed** (`bin/intent_`) -- **UNDER-counted.** Returned `any=0` for `treeindex_commands.bats` and `fileindex_commands.bats`, which carry 143 direct sites and 100 arms. Cause: they spell it `${INTENT_BIN_DIR}/intent_treeindex`, so the literal `bin/intent_` appears nowhere. **A grep for a composed string is a grep for a thing that does not exist, and it returns zero rather than an error.**
2. **Bare-basename keyed** -- **OVER-counted.** `intent_critic` matches `.intent_critic.yml`, a config filename. `pre_commit_hook.bats` scored 11 and **all 11 were the YAML file; its true edge count is zero.**
3. **Path-join keyed** (`/` immediately before the basename) -- **correct.** Every genuine spelling is a path join, so the `/` keeps all of them and drops the collisions. Controls: 68 on `fileindex_commands` (matching its known direct-site count exactly), 12 on `helpers`, 0 on `pre_commit_hook` where the raw key said 11.

**The counting bug that self-corrected, and why that matters.** The first full run used `grep -c ... || echo 0`, which emits TWO zeros on no-match (grep prints `0` AND exits 1). That produced 223 records from 112 files, and was caught in a minute because the totals did not reconcile. **A loud failure self-corrects; a silent zero does not.** That asymmetry is the morning's lesson in miniature and it is why W15's rule is stated as a withdrawal of every zero rather than of the checked ones.

## The numbers as they moved

| figure               | contaminated (basename) | corrected (path-join) |
| -------------------- | ----------------------- | --------------------- |
| files touching pop A | 83                      | **81**                |
| arms touching pop A  | 1,175                   | **1,135**             |
| no edge              | 29 / 296                | **31 / 336**          |
| dispatcher only      | 48 / 620                | **51 / 661**          |
| sub-script only      | 5 / 97                  | **5 / 97**            |
| both                 | 30 / 458                | **25 / 377**          |
| decomposition        | 620 / 555 / 296         | **661 / 474 / 336**   |

**The headline barely moved while the JUDGEMENT burden fell by 81 arms** -- which is the number anyone would have planned against, and the reason the correction had to land before the figure left the board.

## The member table's 13 verdicts, kept because the retirement census still needs them

Superseded as the POPULATION, still good as VERDICTS.

| v2 file                                | arms     | verdict                     | v3 home / reason                                                                                                                                             |
| -------------------------------------- | -------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `no_template_fallback.bats`            | 7        | RETIRE, strongest reason    | `embedded_init.rs` -- templates live INSIDE the binary, so the failure mode is structurally impossible, not merely covered                                   |
| `helpers.bats` id arms                 | 4        | RETIRE                      | `operator_id_spellings.rs` names v2 parity explicitly                                                                                                        |
| `helpers.bats` `list_st_dirs`          | 3        | RETIRE with subject         | v3 enumerates from `.intentfiles` + store; scan-dirs-for-`info.md` is a mechanism that no longer exists                                                      |
| `intent_migrations_languages.bats`     | 7        | RETIRE with subject         | v2 ledger-step machinery gone; `migrate_v2_project.rs` + `unmigrated_project.rs`                                                                             |
| `intent_migrations_relocate.bats`      | 7        | RETIRE with subject         | `.intent/` -> `intent/.config/` was v2.9->v2.10, below v3's floor. `migrate_refusal.rs` is about RESIDUE -- different subject                                |
| `intent_upgrade_dispatcher.bats`       | 6        | RETIRE                      | `upgrade_command.rs` + vc's nine files                                                                                                                       |
| `intent_upgrade_orchestrator.bats`     | 11 of 15 | RETIRE                      | same                                                                                                                                                         |
| `output_width.bats`                    | 5 of 6   | RETIRE                      | `output_shape.rs` maps arm for arm                                                                                                                           |
| `output_width.bats` `dft_width`        | 1        | RETIRE with subject         | `dft_width` is ABSENT from v3 source -- a v2-only config key                                                                                                 |
| `intent_critic.bats`                   | 21       | **MIGRATE**                 | v3 ships `critic` with `--files --staged --severity-min --format --rules --languages`, confirmed by driving; v3 has 6 arms, all on the empty-library refusal |
| `no_absolute_home_paths.bats`          | 2        | MIGRATE (vc ruled)          | to `no_pm_state_in_output.rs`; its vehicle was already hollow                                                                                                |
| `helpers.bats` `stamp_project_version` | 4        | MIGRATE                     | tests move with the carried code, to `release_sidecars.bats`                                                                                                 |
| `helpers.bats` error/warning voice     | 3        | 2 RETIRE, 1 MIGRATE         | `:170`/`:182` die with their subject; the corpus arm `:198` migrates -- see below                                                                            |
| 4 partial-arm files                    | 8        | arm deletions               | `ac_offscope_states` (2), `acceptance_close_gate` (1), `at_grammar_lint` (3), `ambient_project_root_guard` (2)                                               |
| `release_sidecars.bats:114/:116`       | 2        | RETIRE with subject         | greps `bin/intent_upgrade`; name it retired-with-subject                                                                                                     |
| `treeindex_commands.bats`              | 53       | RETIRE with subject         | v3 refuses the verb BY RULING, in its own voice, and `retired_commands.rs` guards the refusal                                                                |
| `pre_commit_hook.bats`                 | 29       | **NO EDGE**                 | its 11 apparent hits were all `.intent_critic.yml`; true edge count zero                                                                                     |
| `intent_bin_retarget_guard.bats`       | 4        | **SURVIVES AS THE ENABLER** | it is what stops a test bypassing `$INTENT_BIN`, so it is the mechanism the 661-arm mechanical route depends on                                              |

### The corpus arm, closed in full

`helpers.bats:198` -- _no shell command emits a capitalised `Error:` prefix_. **MIGRATE.** `error_literal_shape.rs` reads exactly ONE file (`renderer()` at `:80` returns `CARGO_MANIFEST_DIR/src/render.rs`) and polices Rust literal ORDER -- nothing to say about shell. The arm's corpus is `bin/` + both plugin bin dirs, which post-prune is population B + `intent_agents` + `intent_claude_cwi` + `intent_claude_hook`. Live, currently green, **and green inside population A too, so the delete changes only what is WATCHED.** Positive-controlled: the pattern fires in 4 files elsewhere (two RULE.md `## Bad` examples, the autopsy `.exs`, and helpers.bats' own grep string), all correctly outside the arm's declared scope.

## The wiredness partition, and the correction that nearly did not happen

Raw partition over all 39 declared verbs on the **stale** release binary: six unwired -- `agents`, `config`, `ext`, `fileindex`, `learn`, `mcp`.

**vc's correction on `agents`, verified and taken.** My ground was _its implementation is the plugin, not population A_. Wrong: bare `intent agents` is rc=2 **with that plugin present in this project**, so the plugin is not what answers it. It is a deliberate unwired DISPATCHER whose SUBVERBS are wired -- `agents validate` is rc=0 and validates. Right conclusion, wrong grounds (W13).

**Verifying that correction, I broke my own check.** `for a in "agents validate"; intent $a` -- zsh does not word-split, so the phrase went as ONE argv token, and the refusal read `unrecognized subcommand 'agents validate'` **with the space inside the quotes**. I momentarily had a peer's correct claim reading as false on my own broken instrument. **Read the refusal's own echo of what it received before disbelieving anyone.**

**The consequence, chased before it reached hv.** If a bare family verb can be an unwired dispatcher over wired subverbs, the partition measured BARE FAMILY VERBS, not capabilities. Re-driven with correct argv, rc unpiped, in a scratch dir AND in this project:

    config get <key>         rc=2 unwired
    config set <key> <val>   rc=2 unwired
    ext list / ext validate  rc=2 unwired
    ext show <name>          rc=2 unwired
    ext new <name>           rc=2 unwired
    learn                    rc=2, declares no subcommands
    fileindex                rc=2, declares no subcommands, writes nothing

**The four hold. `agents` is the only one of the six with a wired subverb.**

**vc's re-drive discharged the staleness caveat** on `target/debug/intent`, current **by CONTENT** (it carries `db3f947a`; the release pair does not). The four did not move; **`mcp` did** -- rc=2 on the stale binary, rc=0 on the current one. That makes `mcp` the two-sided control the partition never had, and **a control that MOVES while the subject does not is worth more than a clean sweep.** I had been treating `mcp` as noise to discount rather than as the discriminator.

**vc corrected their own first pass too**: it took `rc=$?` after `| head -1` and reported rc=0 for all six. The pipe returns `head`'s status. **The whole partition is rc=2 against rc=0, so a piped rc collapses it into one bucket silently.** Third node on that trap in two days.

## The gate outage, from both ends

vc wrote `instrument_currency_check.sh` at 08:01Z and landed it at `0fe044ef`; the window between was an estate-wide outage. While `AT-00.15` cited nothing on disk, `declared_kind_check.sh` declined to judge it -- its header explicitly excludes the absent case. **The file merely EXISTING made the row judgeable AND disagreeing**, so the gate went red for every node with nothing staged and nothing committed.

The roster guard then diagnosed its own case better than I did: _"the two halves are IN FLIGHT, not severed. DO NOT DELETE THE ROW. `git commit --only` builds HEAD plus the named paths, so a path-scoped commit by ANY node drops a staged sibling it did not name. If the tool is not yours, its author is mid-landing -- wait."_

`--no-verify` was offered by the gate itself and declined. Two independent witnesses confirmed the clearing: cc's own roster population going 68 -> 69 across `a2a51938` and `20f9c8c8`, and vc's rc=0 re-drive. **I asked instead of assuming on a file that was blocking me, and guessing would have picked cc, who did not write it.**
