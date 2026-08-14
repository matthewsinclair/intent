---
node: vc
name: Validation Claude
role: validation
session_id: 163642f8-9332-469e-b2ca-0103b9ad309f
heartbeat_at: 2026-08-14 22:19Z
status: active
focus: "localfold before a compact, continuing after. WP-01..04 Done; WP-05 at 3/4 blocked on AC-05.3 (ic's register, one row). Contract 77 ACs. D22-D27 adopted."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **Localfold before a compact, not a release.** `/compact` does not end a session (protocol invariant 6), so status stays `active`; hv has said we continue after the fold. The day's detail is in `.history/20260814/wip.md`, PM-4.

## TODO

- **AC-05.3 is the live blocker on WP-05** -- the register carries 97 rows against 98 `.bats` files, missing `tests/unit/whiteboard_clock_guard.bats`. No row is UNCLASSIFIED. ic's lane, ic's queued sweep, estate now clear. If ic stays folded, carry it to hv as a named blocker rather than have them start a sweep they cannot finish.
- **Verify cc at the WP-06 claim.** Opening work-list is bucket 3b from `tests/conformance/BASELINE.md`: `st repair`, `st sync`, `st edit`, `wp show`.
- **The one question for hv, asked once**: the 17 `pending-hv` dispatch-table rows, of which 15 collapse to the single usage-convention ruling. **`intent critic`'s four-way exit-2 overload goes ahead of the bundle** -- it is the only one with a live consumer, since the pre-commit gate reads that code.
- **hv's own scope calls, deliberately NOT folded into the standing authorisation**: whether `fileindex` follows `treeindex` into retirement (WP-13 leaves it in WP-06 on purpose); whether `todo --flush` / `--prune` semantics survive at all, which decides whether the watermark field exists; WP-06's name now that AC-06.4 puts a non-parity command in it.
- **WP-10 precondition, from cc**: measure L2/L3 failures per fleet member at its named revision before anyone rules on whether a broken reference in a CLOSED thread carries or blocks. The number comes before the policy question; hv's carry ruling addressed legacy GRAMMAR, and a well-formed row citing a moved file is a different animal.

## Watch-outs

The measurement rules earned this session live in `intent/st/ST0056/parity.md` under `## Measurement rules` -- calibrate before believing a zero, clean-by-luck vs clean-by-construction, a file named after a command that does not test it, file-level classification is structurally blind, an instrument whose error strengthens your finding is not questioned, success is reported by the mechanism. They are on the thread rather than here because a board does not outlive the session that writes it. What remains below is operational to this node.

- **Confirming a peer's finding by re-running the peer's own command is not corroboration.** cc reported `intent/steel_threads.md` absent; I "confirmed" it on the same wrong path and ruled on it. The file is at `intent/st/steel_threads.md`. Test the premise, not the report -- with peers live, that hop is fast enough that a wrong premise reaches code before anyone re-reads it.
- **Correction standing on the record: `8abbbaf`'s message claims it also committed the formatted form of two files; it changed one.** The `MM` I read in `git status` was stale against a settled index and I acted on it without checking the three-way state. The mechanism that message then offers -- `git commit --only` bypassing a pre-commit hook's `git add` -- is **unverified and probably false**. Not amended, because two peers were committing to this branch and rewriting shared history for a cosmetic fix is the worse trade.
- **A generated view can be damaged by the formatter in ways alignment work cannot reach.** Three classes: layout the renderer controls; markup the renderer ADDS around data carrying its own delimiters; and markup the AUTHOR wrote (`*x*` -> `_x_`), which no renderer discipline reaches. AC-07.6 removes the second writer instead, converged at init and migration so it reaches consumer repos.
- **Whiteboard stamps carry a trailing `Z`, always**, read from `date -u` per stamp. The pre-commit clock guard refuses a commit adding an unmarked stamp, a stamp postdating the commit, or an inbox going backwards.
- **Re-read an inbox from disk immediately before appending** (cc's rule). It is the recipient's to empty; appending from context restores entries they already cleared. cc cleared theirs at this fold.
- **Never mutate `bin/**` or `tests/**` in place.** Two mechanisms: `~/.local/bin/intent` symlinks into this repo, AND the BATS suite reads the live working tree (`no_absolute_home_paths.bats:37,100,103`). Sacrificial worktrees only.
- **`git add <paths>` + bare `git commit` commits the WHOLE INDEX.** Use `git commit --only <paths>` verbatim, never `-A`. It has already cost once: cc's staged `bin/release` rename rode vc's `072d277`.
- **The machine-global gitignore ignores `*.sql`** -- committed faces need their `!` exception; `git check-ignore -v` any new non-json artefact.
- **This shell is zsh**: no word-splitting of unquoted parameters, and MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal.
- Release-window mechanics live in `intent/restart.md`'s checklist.

## Decisions

Working decisions are archived once they live in a committed artefact -- see `.history/20260814/wip.md` for this session's, each with the file that now carries it. What remains governs how this node behaves rather than what any file says.

- (2026-08-14) **Verify a claim by re-running its evidence, never by reading its account.** Both directions in one window: re-running ic's findings confirmed three and caught an overclaim; re-running cc's mutation turned my own D24 from agreed to mechanically protected. The single claim I did not re-run became a wrong ruling.
- (2026-08-14) **A record names what it covers -- and checking whether a stamp would be TRUE beats writing a fresh one.** ic applied this to an instruction of mine and produced a better artefact than the instruction asked for. Apply it to instructions, not only to files.
- (2026-08-14) **The contract leads the build or it trails it, and trailing costs more.** Three of four WP-03 rulings were surfaced by builders inside the first hour of touching code; none were visible from the documents alone. A contract is a live artefact during a build, not a gate before one.
- (2026-08-14) **hv standing authorisation is not review.** "Go with your recs, unless they're existential" authorises proceeding; it does not record hv reading each ruling. D22-D27 stay listed apart from D01-D21 for that reason, and hv's own scope calls are never folded into the grant.
- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc holds the ST0056 claim as steward and does not build. Reaffirmed under "you all have the pen(s)": vc's pen is the contract, not the crates.
- (2026-08-14) **hv bounce rulings, still standing**: `corrected` parity class ratified; migration carry policy ratified (CLOSED threads lossless-by-carrying, LIVE threads BLOCKED-until-clean, neither ever lossy); `organize` planned vestigial by construction; push authorised as soon as it makes sense; v2 maintenance default-defer, show-stoppers only.
- (2026-07-02) vc fires on a close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs (ST0056 stewardship excepted).
