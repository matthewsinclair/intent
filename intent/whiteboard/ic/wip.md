---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15T00:08Z
status: paused
focus: "Register + per-test register both COMPLETE and committed. AC-05.3 is blocked on two vc RULINGS, not on measurement -- do not close it from this node. Six negative-assertion rows need adjudication."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**Nothing here is blocked on measurement. Both open items are RULINGS sitting with vc, and neither is this node's to make.**

**The register is COMPLETE at `cd490be`** (98 rows = 98 on-disk `.bats`, zero UNCLASSIFIED/TIMEOUT/UNSTABLE) and **the per-test register is COMPLETE at `b697874`** (`parity/pertest.md`, 487 rows, 40 files split, none refused, `keep` 238 matching the independent burn total exactly). The sweep reproduces byte-identically across two independent runs.

`intent ac gate ST0056/05` still reads `BLOCKED -- 3/4; unsatisfied: AC-05.3`, and that is CORRECT, not stale.

### The two rulings, and why this node must not resolve them

1. **Does `pending` block the close?** The AC's literal text is mechanically SATISFIED and vc's own falsifiability grep passes (zero `pending` rows carry `--`). But `gen_register.sh:94` and the summary row both say the bucket must be EMPTY at close. vc's live message reads the other way. **Two contracts; the convenient reading is the one that closes.** My recommendation is the stricter one, argued from AC-05.2's corpus being undefined while files stay unsplit -- and note the cost objection is now GONE, because the per-test work is done. That removes a reason to say no; it is not a reason to say yes.
2. **What is the corpus?** "Every file in the on-disk `tests/**` estate" is **153 files, 55 of them not `.bats`**. The register covers the 98 `.bats`. vc must name it rather than have me implement the reading I prefer.

`acceptance.md` is vc's. Not touched, and should not be touched from here.

### If the stricter ruling comes back, the work is a short hop

The 40 `pending` file rows resolve from `pertest.md`, which is committed and cross-checked. **Six rows need human adjudication first** -- the negative-assertion tests (below); they are a contract question about whether a failure-asserting test counts as conformance coverage, not a measurement gap.

## Live findings a fresh session should not rediscover

**The burn ratio is BLIND TO NEGATIVE-ASSERTION TESTS.** A test asserting a failure passes under both bindings, because `/usr/bin/false` fails too. Six such tests across three files, surfaced as the six `UNCLASSIFIED` per-test rows. **One-directional**: the method under-counts CLI reach and never over-counts, so every burn figure is a FLOOR. Eighth measurement rule in `parity.md`.

**`surface/dispatch-table.md` was stale against its own canon** -- cc's `sync` row existed in the JSON and not in the view since `f0d6e64`. Repaired at `b697874`. Nothing caught it because **AC-03.4's skew check is not wired up yet**, on the very artefact that ratifies the generated-view pattern.

## TODO -- in order

1. **Adjudicate the six negative-assertion `UNCLASSIFIED` rows** (`intent_upgrade_orchestrator.bats` x3, `intent_upgrade_dispatcher.bats` x2, `subdir_invocation.bats` x1). My view: they DO exercise the CLI and should be `keep` -- but the burn evidence cannot demonstrate it, so the row would rest on READING the test, a different evidence class. Route through vc.
2. **Guard the `bin/intent_<sub>` direct-call invariant, or admit it is unguarded.** 147 sites across 5 files. The existing guard's third test EXPLICITLY excludes them, so the gap is documented rather than accidental. **Build it as a tool under `parity/tools/`, not a `.bats` file** -- a new test file moves the corpus and forces a register regeneration before the close.
3. **Wire AC-03.4's skew check** for `surface/dispatch-table.md`, or hand it to whoever owns AC-03.4. The stale-view incident is the argument.

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
- **Write the stamp AFTER reading the clock, and never correct a bad stamp from memory.** Caught myself twice in ninety seconds on 2026-08-15. First I typed `00:04Z` into a heredoc I composed BEFORE the `date -u` in the same command returned `00:00Z` -- a stamp four minutes in the future, which check A would have refused. Then I "corrected" it to `2026-08-14 00:00Z` by ASSUMING UTC had not rolled over. It had. That second fix was worse than the bug: it put the entry a full day behind the one above it and would have broken inbox monotonicity, check C, the one test that needs no clock at all. **Correcting a fabricated stamp with a second guess is the failure the protocol names, and the guard would have caught only the first of my two attempts.** Read `date -u`, then write. Local was already 2026-08-15 while UTC was not -- that hour is live every night.
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
