---
node: ic
name: Interface Claude
role: interface
session_id: 91f55ae4-3302-4f70-b68e-6b64e0115e6f
heartbeat_at: 2026-08-14T17:23Z
status: active
focus: "Authoring the dispatch-table SSOT on vc's dispatch. st family complete and claimed for shape review; 26 families outstanding. Charter ratified provisional-vc pending hv."
claims: []
---

# Interface Claude (ic)

## DOING

**The dispatch-table SSOT** (`intent/st/ST0056/dispatch-table.json` + generated `dispatch-table.md`), on vc's 17:16Z dispatch. `st` family complete at `fadc526` / `f037649` and claimed to vc for shape review before the remaining 26 go in.

- Format: JSON canon + generated view, because a hand-authored markdown table would have been the fourth instance of the row-grammar tax hv rejected md-as-truth over (design.md:158) -- in the artefact whose job is to stop v3 re-deriving its own surface.
- Awaiting vc on four shape questions (observed/target split; disposition vocabulary vs the register's; invariants at table level; `st zero` left `pending`). Do not run the other 26 until they answer -- a shape correction after 27 families is the expensive version.

## TODO

1. **The remaining 26 families**, once vc rules on shape. Order: `wp`, `ac`, `at` first (they share the acceptance model and are WP-04's input), then the WP-06 long tail, then `claude` (largest surface, WP-07).
2. **Per-test rows for the 40 `split` files** -- DROPPED for this window by vc's dispatch, not finished. 487 tests, 239 burning. Unblocked whenever it comes back: `corrected` is ratified and `ambient_project_root_guard.bats` is the worked example (2/4, both halves adjudicated).
3. **Re-sweep the register when the estate next settles.** Stamped `309d01d` and now stale -- `whiteboard_clock_guard.bats` has no row and `at_lint_wp_scope.bats` changed. That is the artefact working, not failing: re-run `tools/burn.sh` then `gen_register.sh`, never hand-edit.
4. **Guard the other invariant, or admit it is unguarded.** The `INTENT_BIN` guard covers the dispatcher path only. The ~146 `bin/intent_<sub>` direct calls are classified, not guarded; if WP-05 rules any of them must route through the binary, that decision needs its own guard in the same commit.

## Open asks for hv

1. ~~**What is ic's charter?**~~ **RULED by vc 17:16Z, provisional pending hv.** ic owns the dispatch-table SSOT and everything rendered from it: command surface, help text, voice, exit codes, MCP tool list, `intent llm` guide -- AC-05.1, AC-09.1, AC-09.4, and the register half of AC-05.3 / AC-06.3. hv can reverse it; the work survives either way because it is a spec artefact rather than a commitment in code. **Left on the board until hv confirms**, because provisional-vc is not ratified.
2. ~~**The whiteboard roster row.**~~ **Fixed by vc**, who correctly noted README.md is not a single-writer file -- that rule covers `<node>/wip.md` and inboxes only. Roster is four; the "deliberately no interface node" sentence is struck.
3. **The usage-convention ruling** -- **still open, routed, do not answer twice.** vc will not make a scope call on hv's behalf and is right not to. Evidence: 45 stderr-only / 12 stdout-only / 2 both; `--help` failing on 10 of 27; three commands taking unknown flags at exit 0. Now recorded as INV-06 / INV-07 / INV-08 in the dispatch table, with targets left blank and marked `pending-hv` -- a blank marked pending is honest, a guess is not. INV-08 is already `corrected` because clap forces it.
4. **NEW -- two forced fixes hv should see before WP-05, not during it.** (a) **INV-02**: v2 exits 1 on every usage error, clap exits 2 by default; D17 says v2 exit codes carry over, so WP-05 must override clap across nearly the whole surface. One deliberate decision now, or a hundred red conformance tests later that each look like an individual bug. (b) **`st repair`'s `[0-9]+)` arm is dead** (`bin/intent_st:1231`) -- in a case glob `+` is a literal, so `repair 5` and `repair 12345` both error and only the 4-digit form works. Reproducing it faithfully is unconstructible in clap, so this is a forced fix, not a choice.

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
