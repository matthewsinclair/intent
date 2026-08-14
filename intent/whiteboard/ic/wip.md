---
node: ic
name: Interface Claude
role: interface
session_id: 91f55ae4-3302-4f70-b68e-6b64e0115e6f
heartbeat_at: 2026-08-14T18:00Z
status: active
focus: "Folded, everything committed. Dispatch table complete (27 families, 92 entries) and charter ADOPTED under hv standing authorisation. Seven measurement rules in parity.md. Sweep queued behind cc holding the estate, carrying its drift-check requirement."
claims: []
---

# Interface Claude (ic)

## DOING

- Nothing in flight. Dispatch table complete, reviewed by vc, and all their rulings applied. The sweep is queued and blocked on the estate, not on a decision.

## DONE this window

**The dispatch-table SSOT is complete** -- `intent/st/ST0056/dispatch-table.json` (canon) + `dispatch-table.md` (generated view), `fadc526` -> `dd37eb7`. 27 families, 92 entries, 6 new-surface. Coverage checked mechanically against `bin/` rather than asserted; every discrepancy explained (`acceptance` = two nouns one binary; `helpers`/`migrations` are libraries; `agents`/`claude`/`version` are a plugin, a dispatcher arm, a global).

Four corrections to parity.md's command-level table, all measured: `at` has no `set` verb (7 verbs, `done`/`notdone` alias `green`/`red`); `lang` has 6 verbs not 3; `agents` has 5 not 1; `issues` has undocumented `new` and `help`.

**`intent config` is a parity HOLE, and it is the finding of the window.** It produces zero bytes on both streams at exit 0 AND nothing in the estate invokes it -- both halves of the safety net absent at one site, so v3 can do anything there and the suite stays green. The trap: `tests/unit/config.bats` exists and burns 5 of 7, testing config LOADING via `info` / `doctor` / `st list`, so the hole is invisible in a file listing. **RULED** by vc into a condition on AC-00.1 ("no command family has zero burning coverage") plus the work at AC-06.1 (a conformance test lands BEFORE the behaviour is designed, or the `undefined` ruling on it is unverifiable by construction). It also opened the fifth parity class, `undefined`: `corrected` needs an antecedent to correct, and silence is not one.

**Two exposures named against my own work** -- EXP-01 (the view is formatter-stable by luck, not construction; resolved by AC-07.6) and EXP-02 (the surface is now described twice, by the generated `cmd-*.md` inventory and by the authored table, with nothing checking they still agree). **Five measurement rules landed in `parity.md`**, where they outlive the sessions that earned them.

## TODO

**THE SWEEP -- one pass, three jobs, and it now has a hard requirement it did not have this morning.** Blocked on cc clearing the estate: the BATS suite is not parallel-safe (global `/tmp/intent/` sentinels), so this cannot run alongside another node's test run. Do all three in the same pass; each needs the same estate-wide measurement and running it twice is the only real cost.

1. **Re-run `burn.sh` + `gen_register.sh`**, and align the register's vocabulary to `keep/retire/deviate/pending` (vc ruling 2 -- the register moves to meet the table, `pending` explicit and never implied by omission). Unblocks promoting AC-05.3 from an eyeballed non-test AC to a mechanical one: no row carries `pending` at close.
2. **Re-run `gen_inventory.sh`** -- the 108-probe matrix is stamped `69d42a7` and four commits have touched `bin/` since. The most exposed column (the outside-a-project gate, post-0025) was re-run and holds; the rest are carried forward and NAMED as such in the table's `provenance` block.
3. **NEW, and required rather than optional -- the EXP-02 drift check.** Regenerating the inventory rewrites the 26 `cmd-*.md` files, which carry the same verb and flag sets as the dispatch table. **The pass must diff them and REPORT disagreement, never resolve it by picking a winner**: the inventory is measurement, the table is judgement, so a divergence means either the surface moved or a judgement in the table was wrong, and those want different responses. Without this the sweep updates one description and silently strands the other -- worse than today, because today they agree.
4. **Re-run `coverage_map.sh`** in the same pass; it reads the committed baseline, so it goes stale exactly as the baseline does.

### After the sweep, in this order

5. **Per-test rows for the 40 `split` files** -- DROPPED by dispatch, not finished. 487 tests, 239 burning. Needs the sweep's fresh baseline first, so it genuinely sequences after rather than merely being listed after. `ambient_project_root_guard.bats` is the worked example (2/4, both halves adjudicated).
6. **Guard the other invariant, or admit it is unguarded.** The `INTENT_BIN` guard covers the dispatcher path only. The ~146 `bin/intent_<sub>` direct calls are classified, not guarded; if WP-05 rules any must route through the binary, that decision needs its own guard in the same commit.

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
