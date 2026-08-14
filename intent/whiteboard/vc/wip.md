---
node: vc
name: Validation Claude
role: validation
session_id: 163642f8-9332-469e-b2ca-0103b9ad309f
heartbeat_at: 2026-08-14 17:59Z
status: active
focus: "WP-03 CLOSED 6/6 (verified by re-run + mutation); ic delivered 27 families and folded; WP-13 specced; contract 62 -> 77 ACs. hv gave standing authorisation, rulings marked ADOPTED, three commits landed."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **Stewarding the AFK window.** hv asked how far the three nodes get on the Rust CLI + services layer unattended and handed all three the pen. vc does not build -- the pen I am using is the contract pen, because a verifier who authors the code cannot verify it, and one sentence in an AFK window is thin ground to overturn a same-day hv ruling.
- **cc is on WP-04** (facade, core families). **ic is on the register vocabulary alignment + re-sweep.** Both dispatched, both claimed once already this window.

## Delivered this window

- **WP-03 CLOSED 6/6** at cc's `476f1e1`. Verified by re-running, not reading: 15 targets / 60 passed / 0 failed, fmt + clippy clean, six AT files at the contract's named paths. Flipped the six ATs red-then-green through the CLI; `wp done` recorded.
- **D24 is now mechanically protected.** Mutated `sync.rs:161` to a stat gate in a sacrificial worktree; exactly one test red (`a_same_size_same_mtime_rewrite_is_detected`), prediction written first, substitution verified in the file.
- **WP-13 created and specced whole** -- project search in four tiers, retiring treeindex + `in-handoff`.
- **Contract 62 -> 76 ACs**, every one an addition: AC-03.6 (FTS), AC-06.4 (`search`), AC-06.5 (`intent schema`), AC-08.7 (policy-stamp), AC-09.5 (MCP resources), AC-13.1-13.9. `at lint` clean at 74 rows throughout.

## TODO

- **Verify cc at the WP-04 claim.** Two ACs that pass vacuously if written carelessly: AC-04.1's torn-state case wants failure INJECTED not reasoned about, and AC-04.4's "no same-text-for-different-causes" needs two distinct causes asserted distinguishable.
- **Ask hv the 19 `pending-hv` dispatch-table rows as ONE question** -- 15 collapse to the usage-convention ruling. Carry it with my own provisional list so hv is not asked the same thing from two nodes.
- **Escalate to hv**: D22-D25; the two-corpora qualification of D24; ic's charter; the README roster; the `undefined` fifth register class; the todo-watermark home; whether `todo --flush` semantics survive at all; whether `fileindex` follows treeindex; WP-06's name now that it carries a non-parity command.
- `intent critic`'s exit-2 overload is the one live-consumer defect in ic's set -- the pre-commit gate reads that code. hv first.

## Watch-outs

## Watch-outs

- **Confirming a peer's finding by re-running the peer's own command is not corroboration.** cc reported `intent/steel_threads.md` absent; I "confirmed" it with `ls` on the same wrong path and ruled on it. The file is at `intent/st/steel_threads.md` and hv caught it. Bad evidence became a ruling in one message hop. Test the premise, not the report -- and with peers live, that hop is fast enough that a wrong premise reaches code before anyone re-reads it.
- **zsh does not word-split unquoted parameters.** `for c in "wp list"; do intent $c; done` passes ONE argument, so the dispatcher sees a command literally named `wp list` and correctly says it does not know it. bash would have split it. This produced a fabricated finding AND a weak verification: my three INV-02 probes all hit the same unknown-command path and NONE tested missing-required-argument, which is the invariant's main claim. ic caught it from the far side of the wire. **A probe harness is a measuring instrument and must be calibrated against a known-good case before its output is believed** (ic's rule, adopted) -- here `bin/intent wp` alone was the one-command control that would have caught it.
- **Success is reported by the mechanism; the property has to be checked by something else.** Four routes to this in one day: a mutation that applies on a branch the test never walks; a guard scoped to what is already clean; a view formatter-stable only because nothing has written emphasis into it yet; and ic's **an edit that removes something is not verified by the edit succeeding** -- two successful Edit calls, and the day's biggest finding silently gone from their board, caught only by grepping for it afterwards.
- **An instrument whose error makes your finding STRONGER does not get questioned** (ic's observation on my two near-misses, and the sharper half of the calibration rule). Both my broken greps would have produced a more interesting result than the truth -- a real divergence in `cmd-lang.md`, an empty measurement-rules section. Calibration catches the zero that is too weak; nothing but reading the artefact catches the hit that is too good.
- **A generated view can be damaged by the formatter in a way alignment work cannot reach.** Two separate classes: layout the renderer controls (column widths, blank runs, trailing space) and **markup the DATA carries**. A canon value containing backticks, wrapped in backticks by the renderer, interleaves delimiters -- the formatter re-emits its own reading and eats spaces. Found live in ic's `dispatch-table.md` where canon, generator and index were all correct and only the worktree was damaged. Never fix this in the canon; the renderer owns markup.
- **Whiteboard stamps carry a trailing `Z`, always.** The pre-commit clock guard (`ddac6ba`; `Re:`-anchor fix `98ce764`) refuses a commit ADDING an unmarked stamp (check B), a stamp postdating the commit (check A), or an inbox going backwards (check C). Stamp from `date -u`, never rounded up; pre-existing unmarked entries are not fired on and are never rewritten.
- **Re-read an inbox from disk immediately before appending** (cc's rule, adopted for all three). It is the recipient's to empty; appending from context restores entries they already cleared.
- **Never mutate `bin/**` or `tests/**` in place.** Two distinct mechanisms: `~/.local/bin/intent` symlinks into this repo, AND the BATS suite reads the live working tree (`no_absolute_home_paths.bats:37,100,103` -- ic's correction, the better statement). Sacrificial worktrees only. `crates/**` and `schema/**` are cc's lane.
- **The machine-global gitignore ignores `*.sql`** -- committed faces need their `!` exception; `git check-ignore -v` any new non-json artefact.
- **This shell is zsh; MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal.**
- **`git add <paths>` + bare `git commit` commits the WHOLE INDEX -- a peer's staged work rides along.** It happened: cc's staged `bin/release` -> devbin rename rode vc's `072d277`. Use `git commit --only <paths>` verbatim, every time, never `-A`. Three nodes are live right now, so this is at its most dangerous today.
- Release-window mechanics live in `intent/restart.md`'s checklist.

## Decisions

- (2026-08-14) **Deliverable lists are not gated, and nothing in the process reads them.** WP-02 closed 5/5 with `intent schema` unbuilt -- a named deliverable, no AC anywhere, so the gate read a complete AC set over an incomplete WP and correctly returned PASS. Trivial instance, structural class: it holds for all twelve WPs. Cross-checking all twelve deliverable lists against all sixty-two ACs in one pass found four uncovered deliverables. That cross-check should be a standing step at WP close, or deliverable lists should stop being written as commitments.
- (2026-08-14) **Verify a claim by re-running its evidence, never by reading its account.** Applied to both builders in one window with opposite results: re-running ic's three findings confirmed all three AND caught an overclaim (the clap-exit-2 half could not have been executed -- no clap dep in the workspace); re-running cc's WP-03 mutation on my own D24 ruling turned "agreed" into "mechanically protected". The one claim I did NOT re-run -- cc's absent-file report -- was the one that became a wrong ruling.
- (2026-08-14) **A record names what it covers, and checking whether a stamp would be TRUE beats writing a fresh one.** I told ic to re-stamp their table at completion; ic checked first, found four `bin/` commits since the probe matrix, refused a stamp that would have claimed a measurement against a tree it never saw, and carried both revisions plus a named list of columns not re-run. Strictly better than the instruction. Apply the rule to instructions, not only to files.
- (2026-08-14) **The contract leads the build or it trails it, and trailing costs more.** Three of the four WP-03 rulings were surfaced by the builders inside the first hour of touching the code -- the info.md mixed file, the todo.md render stamp, the AC-03.3/design.md contradiction. None were visible from the documents alone, and all three would have been discovered by the migrator seven WPs later. A contract is a live artefact during a build, not a gate before one.
- (2026-08-14) **Derive the law; do not audit the instances.** cc found one render-time stamp in one view. The fix is not "drop that timestamp" -- it is that the renderer has no clock, because AC-03.4 (regenerate, require empty diff) cannot coexist with render-time values ANYWHERE, and one of the three live instances was inside the generated-banner pattern the data model ratifies. Fixing the reported file would have left the defect in every view not yet written.
- (2026-08-14) **When the contract and the architecture narrative disagree, the contract governs.** AC-03.3 requires catching a same-size same-mtime rewrite; design.md specified rehash-on-stat-change, which cannot. That is what a contract is for, and the narrative is what gets corrected.
- (2026-08-14) **A divergent copy proves itself the moment the original moves.** data-model.md's WP-01 draft schema went stale the instant `objective`/`context`/`related`/`legacy` landed in the types. Pruned, pointing at the committed schemars face -- WP-02 had already superseded it, exactly as the document's own header said it would.
- (2026-08-14) **Unratified is an answer being given by default** (ic's framing, adopted). With hv AFK, leaving ic's charter open would have spent the window on bookkeeping. Ruled PROVISIONAL and marked as such, apart from the ratified log, so hv reverses a decision rather than discovering one.
- (2026-08-14) **hv bounce rulings**: (1) `corrected` parity class RATIFIED; (2) migration carry policy RATIFIED -- lossless-by-carrying for CLOSED threads, BLOCKED-until-clean for live, neither ever lossy; (3) organize (both faces) planned VESTIGIAL by construction, retire confirmed at surface cut; (4) push authorized "as soon as it makes sense"; (5) v2 maintenance DEFAULT-DEFER, show-stoppers only.
- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc keeps the ST0056 claim as steward (contract, verification at WP closes, hv interface) and does not build. Reaffirmed under "you all have the pen(s)": vc's pen is the contract, not the crates.
- (2026-08-14) **Contract changes route through vc even when proposed by builders.**
- (2026-08-14) **Stamp heartbeats from `date -u`, never a local clock reading suffixed `Z`.**
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs (ST0056 stewardship excepted).
