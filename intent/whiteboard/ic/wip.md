---
node: ic
name: Interface Claude
role: interface
session_id: 0482e68a-709f-45b1-ab98-44bc9c962bd1
heartbeat_at: 2026-08-14T15:18Z
status: paused
focus: "Released at the EOD/EOW fold. Parity inventory, register and two guards delivered; three asks still unratified with hv, and the charter one decides whether this node has a lane at all."
claims: []
---

# Interface Claude (ic)

## DOING

- Nothing in flight. Day archived to `.history/20260814/`.

## TODO

1. **Per-test rows for the 40 `split` files** -- unblocked: vc ratified `corrected` at the bounce. The register stops at file level by design; these are the second pass. `ambient_project_root_guard.bats` is the worked example (2/4, both halves already adjudicated).
2. **Re-sweep the register when the estate next settles.** It is stamped `309d01d` and already one commit stale -- `whiteboard_clock_guard.bats` has no row. That is the artefact working, not failing: re-run `tools/burn.sh` then `gen_register.sh`, never hand-edit.
3. **Guard the other invariant, or admit it is unguarded.** The `INTENT_BIN` guard covers the dispatcher path only. The ~146 `bin/intent_<sub>` direct calls are classified, not guarded; if WP-05 rules any of them must route through the binary, that decision needs its own guard in the same commit.

## Open asks for hv

1. **What is ic's charter?** Proposed: the dispatch-table SSOT and everything rendered from it -- command surface, help, voice and exit codes, MCP tool list, `intent llm` agent guide. Maps onto WP-05's SSOT half, WP-06's register, WP-09 entirely. **Unratified is itself an answer being given by default**: without it ic takes errands and nobody owns the interface as a whole.
2. **The whiteboard roster row.** `intent/whiteboard/README.md:13` still says there is deliberately no interface node and the roster is three. False since ic was scaffolded; hv's file, so untouched. Either strike the sentence or strike the node.
3. **The usage-convention ruling** -- **routed, do not answer twice.** vc drafted it as the `corrected` class (`parity.md:13`) and hv ratified that class at the bounce; what remains is which v2 behaviours join it. Evidence: 45 stderr-only / 12 stdout-only / 2 both, `--help` failing on 10 of 27, three commands taking unknown flags with exit 0. clap forces the last one whether or not anyone decides.

## Watch-outs

- **ic cannot certify a green suite.** matts owns the authoritative run. Everything ic reports is evidence; label it that way when handing it to a peer.
- **Read `bin/**`, never mutate it** -- `~/.local/bin/intent` symlinks into this repo, so every project on the machine runs whatever state those files are in. Sacrificial worktree for anything that writes. Note `intent claude rules index` mutates `INTENT_HOME` despite reading like a query.
- **A bulk rewrite is unfinished until something enforces it.** `87a315b` retargeted 979 call sites and regressed the same day, via a spelling written by someone with no reason to know the invariant existed. Guards now exist for the dispatcher path and for clock stamps; the pattern generalises to the next sweep.
- **The clock gate is live in the shared `.git/hooks` and BLOCKS.** Stamp every timestamp from `date -u +'%Y-%m-%d %H:%MZ'`, every time. If it ever refuses something honest that is a bug in my guard -- send it over rather than reaching for `--no-verify`.
- **A grep cannot tell a call site from a test fixture holding the same string.** Bit the register's classifier and the clock guard's check C in one afternoon. When widening a needle, add the complement case that proves it did not swallow the neighbouring class.
- **The two obvious sources for a command surface both lie.** The surface is files on disk (`bin/intent`'s `*)` default), not case arms; `bin/intent_help` hand-maintains its list behind a skip list and still calls `upgrade` an STP migration at v2.19.0. Enumerate and run; never read and transcribe.
- **This shell is zsh**: command-prefix assignments evaluate left to right, so `A="$A/x" B="$A/y"` gives B the already-reassigned A. Bash does not.

## Decisions

Working decisions are archived once they live in a committed artefact -- keeping a second copy here is the divergent-copy drift Highlander exists to stop. See `.history/20260814/wip.md` for this day's, each with the file that now carries it. Two remain live because they govern how this node behaves rather than what any file says:

- (2026-08-14) **Read the other boards before you speak.** Two of my three asks to hv were already routed through vc's agenda; reading first is the only reason hv was not asked the same question twice from two nodes. Costs one command.
- (2026-08-14) **Audit yourself before you confess, and check the audit with the same rigour either way.** Under a bollocking the reflex is to confess first; mine looked wrong and were not. A false admission is fabrication too, and being the humbler kind does not make it true.
