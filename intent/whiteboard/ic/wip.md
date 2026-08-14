---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-14T23:41Z
status: active
focus: "Register COMPLETE at cd490be -- 98 rows, zero UNCLASSIFIED, zero TIMEOUT. The sweep found four more defects in my own toolchain on the way. AC-05.3 now waits on two vc rulings, not on measurement."
claims: []
---

# Interface Claude (ic)

## DOING -- waiting on vc, not on measurement

**The register is COMPLETE at `cd490be`.** 98 rows against 98 on-disk `.bats`, zero UNCLASSIFIED, zero TIMEOUT, zero UNSTABLE. `keep` 31 / `pending` 40 / `out-of-scope` 21 / `retire` 5 / `deviate` 1. 1260 tests, 718 reaching the CLI. `coverage_map.sh` and `drift_check.sh` both clean against the fresh baseline. Committed `f11e200` + `bd5938f`.

**AC-05.3 is BLOCKED ON TWO RULINGS, both with vc, and they are not measurement questions.** Do not close it from this node.

1. **Does `pending` block the close?** The AC's literal text ("every file classified; no unclassified rows") is mechanically SATISFIED -- and vc's own falsifiability grep passes: zero `pending` rows carry `--`, every one carries `n/total` with `0 < n < total`. But `gen_register.sh` and the summary row both say the `pending` bucket must be EMPTY at close, and 40 rows say it is not. vc's live message reads the other way. **Two contracts, 40 rows turning on it, and the convenient reading is the one that closes -- which is exactly why this node does not get to pick.** Recommendation sent: the stricter one, argued from consequence (AC-05.2 needs the core families green on the narrowed contract, and a `pending` file is one where some tests are inside that contract and some outside, unseparated -- so 05.2's corpus is undefined until the split is done).
2. **What is the corpus?** AC-05.3 says "every file in the on-disk `tests/**` estate" = **153 files, 55 of them not `.bats`** (fixtures, README, test_helper.bash). The register covers the 98 `.bats`. vc fixed one literalism and introduced another; asked them to name the corpus rather than have me implement the reading I prefer.

`acceptance.md` is vc's file. Not touched.

## What the sweep actually bought -- four defects, none of them the one I went looking for

I justified the full re-sweep on provenance hygiene. That justification was thin; the sweep paid for itself another way.

**The finding that started it.** `burn-baseline.tsv` had 94 data rows against a 97-row register -- the artefact the register NAMES as its provenance could no longer reproduce it. Three files landed after the baseline. Nothing noticed, because a register built from a short TSV is not malformed, just silently smaller than the estate it claims.

**Then, in my own tools:**

1. **`gen_register.sh` had no `TIMEOUT` arm and no default arm.** So the timeout I added to `burn.sh` that morning -- to stop a sweep failing silently -- had installed a _second_ silent failure one stage downstream: a timed-out file was emitted NOWHERE. Proven against the pre-edit generator: 2 rows out for a 3-file TSV.
2. **The summary claimed "all N tests pass with the default `INTENT_BIN`" from a template, not the data.** A red baseline would have published a clean bill of health it had just measured to be false.
3. **Unmeasured tests were averaged into the ratio as zeroes** -- "no measurement exists" reported as "does not reach the CLI".
4. **`coverage_map.sh` SKIPPED files absent from the baseline** (`[ -n "$row" ] || continue`), so the three missing files were counted as neither REAL nor VACUOUS in any family they touched. They left the arithmetic entirely, under a confident verdict. It also crashed on a `--` burn cell (`[ "--" -gt 0 ]` is fatal under `set -e`).

**Two consumers of one artefact had independently grown two different wrong behaviours around the same missing check.** That is Highlander's case made by demonstration rather than by argument, and the comparison now lives once in `lib_corpus.sh`.

## TODO -- in order

1. **Per-test rows for the 40 `pending` files.** Blocked on ruling 1 above only in the sense that it decides whether they gate the close; the work is worth doing either way. `ambient_project_root_guard.bats` is the worked example (2/4, both halves adjudicated).
2. **`bats_coverage` in the dispatch table may be overstated and I have NOT re-derived it.** cc found the whole third level of the surface unbuildable by the v3 spine (`claude skills`/`rules`/`ws`, `agents template`, `st zero` -- the `subcommand`-kind arg with a `values` list was skipped). That does NOT touch the register's burn figures, which are measured against v2 where those commands dispatch fine. It touches `bats_coverage`, which counts files naming a family and says nothing about buildability. Asserting no corrected number.
3. **Guard the `bin/intent_<sub>` direct-call invariant, or admit it is unguarded.** Unchanged: the `INTENT_BIN` guard covers the dispatcher path only; ~146 direct calls are classified, not guarded.

## DONE this session

**The dispatch-table SSOT is complete** -- now at `surface/dispatch-table.json` + `surface/dispatch-table.md` (moved by cc under D26; `intent st done` relocates a thread's directory, so a table the binary `include_str!`s would have broken the build at the release). 27 families, 93 entries, 6 new-surface. Detail archived to `.history/20260814/`.

**`intent config` is a parity HOLE** -- no v2 behaviour AND no test invoking it. Ruled into AC-00.1 + AC-06.1; opened the fifth parity class `undefined`. **Two exposures** named against my own work (EXP-01, EXP-02). **Seven measurement rules** in `parity.md`. **Two new tools**, both registered: `coverage_map.sh` (parity-hole finder) and `drift_check.sh` (the EXP-02 mechanism, which found `todo list` missing from my own table on its first run).

## Open asks for hv

1. ~~**What is ic's charter?**~~ **CLOSED -- ADOPTED.** ic owns the dispatch-table SSOT and everything rendered from it: command surface, help text, voice, exit codes, MCP tool list, `intent llm` guide -- AC-05.1, AC-09.1, AC-09.4, and the register half of AC-05.3 / AC-06.3. vc ruled it at 17:16Z as provisional; hv's standing authorisation ("go with your recs, unless they're existential") moved it and nine other provisional rulings to ADOPTED, with the meaning defined once in design.md: vc made the call, hv authorised proceeding without reading each, reversible in one line. **No longer a question hanging over this node.** Kept struck-through rather than deleted, because how a lane got decided is worth more than the fact that it is.
2. ~~**The whiteboard roster row.**~~ **Fixed by vc**, who correctly noted README.md is not a single-writer file -- that rule covers `<node>/wip.md` and inboxes only. Roster is four; the "deliberately no interface node" sentence is struck.
3. **The usage-convention ruling -- ONE question that clears 15 of the 17 pending rows.** Still open, routed through vc, **do not answer it from two nodes.** vc will not make a scope call on hv's behalf and is right not to; they are carrying it to hv alongside their own provisional list. Evidence: 45 stderr-only / 12 stdout-only / 2 both; `--help` failing on 10 of 27; three commands taking unknown flags at exit 0. Recorded as INV-05 (usage never printed on a usage error -- the `error ...; usage` pairs are all dead code), INV-06 (stream misroutes) and INV-07 (`--help` failing), with targets blank and marked `pending-hv`: a blank marked pending is honest, a guess is not. INV-08 is already `corrected` because clap forces it.
4. **`intent critic` overloads exit 2 FOUR ways** -- findings-present (the meaningful one, INV-04), bare invocation, unknown flag, and bad positional; the unknown-flag path leaks grep's own error as the command's voice. **The only pending item with a live consumer**: the pre-commit gate reads this exit code, so "findings" and "you typed it wrong" are indistinguishable to it today. Worth hv seeing ahead of the usage-convention bundle -- everything else pending is a decision about what v3 should do, this is a defect in what v2 does now. (My first pass said three conditions; vc measured four. The undercount is named on the row rather than quietly replaced.)
5. **NEW -- two forced fixes hv should see before WP-05, not during it.** (a) **INV-02**: v2 exits 1 on every usage error, clap exits 2 by default; D17 says v2 exit codes carry over, so WP-05 must override clap across nearly the whole surface. One deliberate decision now, or a hundred red conformance tests later that each look like an individual bug. (b) **`st repair`'s `[0-9]+)` arm is dead** (`bin/intent_st:1231`) -- in a case glob `+` is a literal, so `repair 5` and `repair 12345` both error and only the 4-digit form works. Reproducing it faithfully is unconstructible in clap, so this is a forced fix, not a choice.

## Watch-outs

- **ic cannot certify a green suite.** matts owns the authoritative run. Everything ic reports is evidence; label it that way when handing it to a peer.
- **Read `bin/**`, never mutate it** -- `~/.local/bin/intent` symlinks into this repo, so every project on the machine runs whatever state those files are in. Sacrificial worktree for anything that writes. Note `intent claude rules index` mutates `INTENT_HOME` despite reading like a query.
- **A bulk rewrite is unfinished until something enforces it.** `87a315b` retargeted 979 call sites and regressed the same day, via a spelling written by someone with no reason to know the invariant existed. Guards now exist for the dispatcher path and for clock stamps; the pattern generalises to the next sweep.
- **The clock gate is live in the shared `.git/hooks` and BLOCKS.** Stamp every timestamp from `date -u +'%Y-%m-%d %H:%MZ'`, every time. If it ever refuses something honest that is a bug in my guard -- send it over rather than reaching for `--no-verify`.
- **A guard verified in one harness is not verified -- it is verified in THAT harness.** `corpus_require` tested green under `gen_register.sh` (`set -uo pipefail`) and was DEAD under `coverage_map.sh` (`set -euo pipefail`): the bare command substitution aborts the shell the instant it finds a disagreement, so the tool exited 1 with an EMPTY stderr against a baseline known to be four files short. **A guard that dies silently in the strict-mode caller is worse than no guard** -- it reads as a clean tool failure rather than a finding. Run the battery in every harness that sources it, canary in each.
- **`git commit --only <paths>` does NOT protect a file two nodes both edit.** It scopes to paths and then takes whatever is in the working tree there. `ab351a2` swept my uncommitted MODULES.md row into cc's doctor commit; content was fine, attribution was not, and the window was twenty minutes. For genuinely shared files (MODULES.md is the live one) commit the row in the same commit that creates the module.
- **A grep cannot tell a call site from a test fixture holding the same string.** Bit the register's classifier and the clock guard's check C in one afternoon. When widening a needle, add the complement case that proves it did not swallow the neighbouring class.
- **The two obvious sources for a command surface both lie.** The surface is files on disk (`bin/intent`'s `*)` default), not case arms; `bin/intent_help` hand-maintains its list behind a skip list and still calls `upgrade` an STP migration at v2.19.0. Enumerate and run; never read and transcribe.
- **This shell is zsh**: command-prefix assignments evaluate left to right, so `A="$A/x" B="$A/y"` gives B the already-reassigned A. Bash does not.

## Decisions -- this window (2026-08-14 PM, hv AFK, three nodes live)

- (2026-08-14) **A measuring instrument must be calibrated against a known-good case before its output is believed -- especially when it reports zero.** Two harness lies crossed nodes today from the same root: zsh does not word-split unquoted parameters, so `intent $probe` with `probe="wp list"` passes ONE argument and the dispatcher correctly reports a command it does not know. It invented a defect in `intent wp list` that does not exist. The control that would have caught it costs one command (`intent wp` alone, which proves the dispatcher reaches `intent_wp`). Applied immediately to the next instrument built: before believing `config` had zero coverage, I ran the same needle against `doctor` and got 3. **A zero and a broken instrument are indistinguishable until the instrument is shown to report non-zero somewhere it should.**
- (2026-08-14) **Three wrong premises crossed node boundaries today; three were caught in one hop; none became a ruling.** My clap finding recorded as "verified by execution" when the clap half could not have been (no clap dependency exists yet); vc's zsh probe; vc's use of a nonexistent `at set` verb, repeated from cc and caught by checking their own record against my table. Against a failure mode that is plausible and silent by construction, **that ratio is the measurement worth keeping from this window** -- artefact sizes are not. It also names what the audit is for: not catching bad work, catching confident wrong premises before they harden into rulings.
- (2026-08-14) **Evidence has classes, and "verified" is not one thing.** `measured` (a probe was run), `documented-default` (a framework's published behaviour -- correct today, changeable by a major bump or one builder setting), `read` (source, unexecuted). A documented default recorded as measured is a finding with a silent expiry date, so each one carries a `pinned_by` naming the test that will red when it moves.
- (2026-08-14) **A generated view must be idempotent THROUGH the formatter, not merely through the renderer.** Two independent classes, both found by committing and watching rather than by reasoning: layout the renderer controls (column widths), and markup the DATA carries (a value with its own backticks, wrapped again, then "normalised" by the formatter collapsing the spaces inside). A renderer idempotent only against itself yields a view that oscillates on every commit, and the first thing anyone does with a check that cries wolf is switch it off.
- (2026-08-14) **File-level classification structurally cannot see a hole at the command level.** `tests/unit/config.bats` exists, burns 5 of 7, and never invokes `intent config` -- it tests config LOADING via other commands. The register classifies it correctly and is silent about the fact that nothing covers the command. **A file named after a command that does not test that command is worse than no file**, because it answers "is this covered?" wrongly and confidently.

## Decisions -- standing

Working decisions are archived once they live in a committed artefact -- keeping a second copy here is the divergent-copy drift Highlander exists to stop. See `.history/20260814/wip.md` for this day's, each with the file that now carries it. Two remain live because they govern how this node behaves rather than what any file says:

- (2026-08-14) **Read the other boards before you speak.** Two of my three asks to hv were already routed through vc's agenda; reading first is the only reason hv was not asked the same question twice from two nodes. Costs one command.
- (2026-08-14) **Audit yourself before you confess, and check the audit with the same rigour either way.** Under a bollocking the reflex is to confess first; mine looked wrong and were not. A false admission is fabrication too, and being the humbler kind does not make it true.
