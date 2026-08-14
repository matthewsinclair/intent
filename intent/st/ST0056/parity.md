# Parity contract - ST0056: v2 conformance for the v3 CLI (WP-01 spec)

## The contract

**The v3 binary is certified by the incumbent**: the v2 BATS estate runs against it via an `INTENT_BIN` override, and v3 is green when v2's own suite cannot tell the difference -- except where this contract says so, in advance.

- **In scope (parity holds)**: stdout shape, stderr voice (`ok:`/`error:` lowercase, 0023), exit codes, command/subcommand/flag grammar, behavioural semantics.
- **Ratified deviation classes (decided here, never discovered in triage)**:
  - **File layout** -- structured canon appears (`thread.json`, `issues/<n>.json`); `info.md`/`acceptance.md` become generated views; frontmatter conventions on structured data retire (D01-D04). Tests asserting bytes-in-files that are now views retire with the layout.
  - **Issues directories** -- `OPEN/`/`CLOSED/` stop encoding status (status is data); index views replace directory browsing.
  - **Generated-view banners** -- views carry a generated footer; tests asserting their absence retire.
  - **Manual-edit workflows** -- tests that hand-edit structured md and expect the tool to honour it convert to mutation-based equivalents or retire (authored-once, D02).
  - **Corrected** (proposed by ic, hv-ratified at the bounce 2026-08-14) -- a v2 behaviour that is simply wrong and is fixed rather than faithfully reproduced. Known members from ic's census: unknown flags accepted silently with exit 0 (`info`/`config`/`version` -- clap exits 2, so v3 diverges on day one whether or not anyone decides to); `--help` reporting failure on 10 of 27 commands; the stderr/stdout misroute census (45 stderr-only / 12 stdout-only / 2 both on failing invocations -- larger than the three sites in cc's hv queue). Distinct from **deviate**: deviate is a design consequence of v3; corrected is a bug fix. Both carry a ratification reference per register row.
- **Explicitly out of parity scope**: `bin/release` and the test harness itself (repo dev tooling, not shipped surface).

## Parity properties (beyond output equality)

Output-equality across implementations cannot catch v3 faithfully reproducing a v2 bug -- the two would agree and the suite would go green. Properties asserted directly, per scoped verb:

- **Scope-honouring (issue 0024):** an instrument that accepts a narrowing argument answers the narrowed question, and its output names the resolved scope. Found in v2: `at lint <ID>/NN` and `ac gate <ID>/NN` silently dropped the WP scope, and a scoped `--fix` rewrote rows OUTSIDE the scope; an equality-only suite would have carried that into v3 as certified behaviour.

## The keep/retire/deviate register

One row per BATS test file (finer-grained per-test rows where a file mixes classes), maintained from WP-05 and complete at WP-06 close:

```
| tests/<file>.bats | <command(s)> | keep · retire · deviate | <deviation class or ratification ref> | <notes> |
```

- **keep** -- asserts command surface; runs unmodified against `INTENT_BIN`.
- **retire** -- asserts the old layout or a v2 implementation detail; retired with a named class from the list above.
- **deviate** -- asserts surface we are deliberately changing; each carries a D-number ratified in design.md before the port lands.

The register is the honest ledger of everything the rewrite changes. An unclassified failing test is a defect, not a candidate for reclassification-in-the-moment.

## Command-surface inventory (command/subcommand level)

Source of truth for the deep pass: `bin/intent` dispatch + per-binary help. This table is the WP-01 command-level inventory; the flag-level pass is the IC handoff below.

| Command      | Subcommands (v2.19.0)                                                                                                                          | v2 source            | v3 disposition                                                                                                       |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- | -------------------------------------------------------------------------------------------------------------------- |
| st           | new [-s], start, done, cancel, list [--status --width], sync [--write], organize [--write], show, edit, repair [--write]                       | intent_st            | facade family (WP-04)                                                                                                |
| wp           | new, start, done, list, show                                                                                                                   | intent_wp            | facade family (WP-04)                                                                                                |
| ac           | list, status, satisfy --evidence, gate, descope --to, rescope, withdraw --reason, reinstate                                                    | intent_acceptance    | facade family (WP-04)                                                                                                |
| at           | lint [--fix], set, list (verify exact set in deep pass)                                                                                        | intent_acceptance    | facade family (WP-04)                                                                                                |
| issues       | list [--kind], add [--severity] (alias new), show [--json], close, open                                                                        | intent_issues        | WP-06; directory deviation applies                                                                                   |
| todo         | (view), update, done [--flush --prune], notdone, toggle, --json                                                                                | intent_todo          | WP-06 (generated view + queries)                                                                                     |
| info         | (status display)                                                                                                                               | intent_info          | WP-06                                                                                                                |
| init         | [--lang]                                                                                                                                       | intent_init          | WP-06 (embedded templates, WP-07)                                                                                    |
| bootstrap    | (first-time setup)                                                                                                                             | intent_bootstrap     | WP-06; INTENT_HOME demotion (WP-11)                                                                                  |
| config       | (display)                                                                                                                                      | intent_config        | WP-06                                                                                                                |
| doctor       | (checks 1..4e + fixes)                                                                                                                         | intent_doctor        | WP-06 (integrity queries + skew/unparsed)                                                                            |
| upgrade      | (v2 ledger)                                                                                                                                    | intent_upgrade       | REPLACED by the v3 migrator (WP-10); v2 ledger never reimplemented                                                   |
| agents       | sync                                                                                                                                           | plugin (agents)      | WP-07                                                                                                                |
| claude       | subagents, skills (install/sync/uninstall/list), rules (list/show [--lang]), hook <name>, upgrade, prime, ws (new/list/archive/hygiene), start | intent (claude arms) | WP-07; `hook` byte-compatible day one                                                                                |
| critic       | <lang> (headless runner)                                                                                                                       | intent_critic        | WP-07 (embedded rules, strict-proxy)                                                                                 |
| lang         | init, remove, list                                                                                                                             | intent_lang          | WP-07                                                                                                                |
| llm          | usage_rules etc                                                                                                                                | intent_llm           | WP-06 + regenerated guide (WP-09)                                                                                    |
| learn        | (capture learnings)                                                                                                                            | intent_learn         | WP-06                                                                                                                |
| modules      | (registry guardrails)                                                                                                                          | intent_modules       | WP-06                                                                                                                |
| organize     | [--write]                                                                                                                                      | intent_organize      | planned retire -- vestigial by construction (hv 2026-08-14; note below)                                              |
| plugin       | (discovery)                                                                                                                                    | intent_plugin        | WP-06                                                                                                                |
| ext          | (user extensions)                                                                                                                              | intent_ext           | WP-06                                                                                                                |
| treeindex    | <dir>                                                                                                                                          | intent_treeindex     | WP-06 (content semantics unchanged; location decision deferred, D21)                                                 |
| fileindex    | (checkbox file indexes)                                                                                                                        | intent_fileindex     | WP-06                                                                                                                |
| help         | [command]                                                                                                                                      | intent_help          | WP-05 (generated from the dispatch table)                                                                            |
| st_zero      | (STZero retrofit)                                                                                                                              | intent_st_zero       | WP-06 tail; candidate for a ratified retire-deviation if unused by the fleet -- decide at port time, in the register |
| -- new in v3 | schema, search, export --format, ingest --from-md, daemon (start/stop/status/run), mcp                                                         | --                   | additive; no parity obligation                                                                                       |

**organize (both faces) is planned vestigial (hv, 2026-08-14).** A strictly structured model cannot, by construction, hold data in the wrong spot or the wrong format, so the disorder `organize` exists to repair cannot arise. Both `intent organize` and `intent st organize` are planned retires -- the pre-existing two-implementation Highlander dissolves with them -- confirmed finally at the surface cut (WP-05/06). Register rows: retire, citing this ruling.

## The IC handoff (deep pass)

Two deliverables, design-neutral, v2-side, feeding WP-05:

1. **Flag-level inventory**: for every row above, the full flag/argument grammar, stdout/stderr shapes, and exit codes -- from `bin/` source + help text, recorded per command in a `parity/` appendix. Verify the `at` subcommand set and every alias (`issues new`, `organise`).
2. **Harness retarget + classification**: make the BATS estate run against `INTENT_BIN` (audit how tests resolve `intent` today -- PATH vs relative -- and thread the override through the helpers); then produce the first-pass register with every test file classified keep/retire/deviate against the classes above. Flag any test that fits no class -- that is a contract gap for vc, not a judgement call for the classifier.

Working rules: sacrificial worktree for anything touching `bin/**` or helpers; mutation-check any harness change (a retargeted suite that cannot fail proves nothing -- point `INTENT_BIN` at `/usr/bin/false` and watch it burn before trusting a green). A mutation canary must come from the same fixture and branch the test drives -- **applied is not reached** (cc, 2026-08-14: a substitution can apply on a branch the test never walks and read as "does not discriminate").

## Measurement rules

Earned on this thread, 2026-08-14, each by a failure rather than by foresight. They belong here rather than on a node's board because they govern how the parity evidence is produced, and the boards do not outlive the sessions that write them.

**Calibrate an instrument against a known-good case before believing its output -- especially a zero.** A measuring instrument reporting zero is indistinguishable from a broken one until it has been shown to report non-zero somewhere it should. Two harness lies crossed nodes in one afternoon from a single root: zsh does not word-split unquoted parameters, so `intent $probe` with `probe="wp list"` passes ONE argument and the dispatcher correctly reports a command it does not know -- inventing a defect in `intent wp list` that does not exist. The control costs one command. Applied within the hour to the next instrument built: before believing `intent config` had zero test coverage, the same needle was run against `doctor` and returned 3.

**A file clean by luck and a file clean by construction look identical in a diff, and only one of them stays clean.** Record what an artefact is NOT protected against, not only what it currently passes -- a green check that cannot say which kind it is has told you less than it appears to. Three routes reached this idea in one day, which is usually the sign it is load-bearing: a mutation that applies on a branch the test never walks; a guard scoped to what is already clean, which certifies the status quo; and a generated view that is formatter-stable only because nobody happened to write emphasis into it yet.

**A file named after a command that does not test that command is worse than no file.** It answers "is this covered?" wrongly and confidently. `tests/unit/config.bats` exists, burns 5 of 7, tests configuration LOADING via `intent info` / `intent doctor` / `intent st list`, and never invokes `intent config` once -- so the command has zero coverage while the file listing implies otherwise.

**File-level classification is structurally blind to command-level coverage.** This is a stated limitation of the register, not an oversight in it: a `split` row is silent about whether any command family is exercised by nothing, because it classifies files and the question is about commands. `parity/tools/coverage_map.sh` answers that question instead, by joining the dispatch table's families against `burn-baseline.tsv` -- and the join is the method, since a naive grep reports `treeindex` as one of the best-covered families in the estate when all 53 of its tests exec the binary directly and the dispatcher sees none of them.

**A command family with zero burning coverage is a parity hole**: v3 can change it freely and the conformance suite stays green. Condition on AC-00.1; the work lives at AC-06.1.
