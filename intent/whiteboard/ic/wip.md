---
node: ic
name: Interface Claude
role: interface
session_id: 0ccc7c30-24c1-48ce-b698-ab212286083e
heartbeat_at: 2026-08-20 09:08Z
status: paused
focus: "**FOLDED. FOUR hv RULINGS IN HAND, ONE EXECUTED (`c58e8bbb`), THREE SEQUENCED BEHIND dc.** Today: `st hydrate` wired, the roster stopped disagreeing with itself, the FORMAT leg closed, and **INV-04 had `intent critic`'s exit 2 backwards -- a silent gate bypass that would have been conformant to my own table** (dc found it, fixed at `dc8ee802`). **NEXT: ruling 1, drop `ISSUE:` -- grammar and dispatch-table halves are free; the `render.rs` half waits for dc.**"
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT.** Everything of mine is committed. The tree's dirt is dc's critic build (`render.rs`, `spine.rs`, `critic.rs`, both `Cargo.toml`) and vc's WP-09/10 (`doctor.rs`, `doctor_checks.rs`, `lib.rs`).

## ON RESUME -- read this first

1. **FOUR hv RULINGS ARE IN HAND. THEY ARE IN `ic/.history/20260820/` AND IN EVERY PEER INBOX; DO NOT RE-ASK THEM.**
   - **(1) Issues are canon-and-store only.** `ISSUE:` leaves the `.intentfiles` grammar, both `issues hydrate|dehydrate` rows are WITHDRAWN from the dispatch table, and the 40 legacy markdown files under `intent/issues/{OPEN,CLOSED}/` are pruned as MIGRATION RESIDUE (vc's classification, precedent `1af21f4e`).
   - **(2) AC-05.2 is the next unit** -- lifecycle verbs edit the list, plus the WARNING over unsynced bytes. Never a gate.
   - **(3) Top-level `intent edit <ID>` is the ONE home; `st edit` becomes a thin delegate.**
   - **(4) DONE at `c58e8bbb`** -- `render`/`Generated` deleted, orphan test deleted, `edit_writes_pinned_region.rs` rewritten.
2. **START WITH RULING 1's FREE HALVES: `intentfiles.rs` (drop `Sigil::Issue`) AND THE DISPATCH TABLE (withdraw two rows).** `render.rs` is dc's until the critic lands -- **do not open it.** Removing `Sigil::Issue` breaks `edit_writes_pinned_region.rs`'s accumulation test on purpose; that was left to break rather than pre-empted.
3. **EXPECT THE ROSTER TO GO RED AND DO NOT "FIX" IT BY RE-ADDING ROWS.** Withdrawing the two `issues` rows drops them from `shipped_mutators()`, so `DECLARED_BUT_UNWIRED` holds two stale members and the stale-entry check fires with `bucketed but not a shipped mutator`. **That is the self-invalidating bucket working.** Move them out; do not widen the bucket.
4. **DO NOT REBUILD TODAY'S WORK.** `st hydrate` (`21c0ccf2`), the roster report + assertion + `DECLARED_BUT_UNWIRED` + `declared_but_unwired.rs`, the fmt sweep (`0a9a7341`), INV-04 (`dc8ee802`) and the two-region deletion (`c58e8bbb`) are landed and mutation-proven.

## TODO -- LIVE ONLY

1. **Ruling 1: drop `ISSUE:`.** Free halves first (see ON RESUME 2), `render.rs` half last. Also removes `Facade::hydrate`'s Issue arm -- the one resolving into CANON.
2. **Ruling 2: AC-05.2.** `st new` adds / `--dehydrate` skips; `st done`/`st cancel` remove / `--keep` skips; `st reopen`/`st reinstate` add back; WARNING via `Facade::sync_uncommitted`. **Needs an `unpin` beside `pin`.** Then rewrite `edit_writes_pinned_region.rs` for real and green AT-05.2.
3. **Ruling 3: `intent edit <ID>`** -- AC-05.1 / AT-05.1 (`to-write`), `st edit` delegates.
4. **OWED TO dc, BOTH MINE, BOTH FIRE WHEN THE v3 CRITIC LANDS:** `exit_codes.rs:389` asserts `critic shell` exits 2 **because it is unwired today**, so implementing it turns that red and it will look like dc broke it -- re-point it at a genuinely unwired verb. And `guide.rs:142` claims 2 has _exactly one cause_ in this build, which becomes false the same commit.
5. **`doctor --json` surface row** (cc's ask). cc's gate arm parses text and refuses at 2 as a workaround; **delete that parse when the face lands, do not keep it beside.** `Finding` already derives `Serialize`/`JsonSchema` and `FindingClass` is kebab-case, so `Report` needing `Serialize` is the whole model change. Trap: declare `--json` at BOTH family and verb level.
6. **AC-08.4 / AC-08.5** (WP-08) still red, untouched.
7. **STILL OPEN, NOT MINE TO RULE:** whether the `BEGIN/END INTENT` marker grammar survives at all. hv deliberately did not fold it into ruling 4; vc raises it.

## Watch-outs

- **ANY WORKSPACE-WIDE COMMAND IN A SHARED CHECKOUT ACTS ON EVERYONE'S UNCOMMITTED WORK.** This is the general form and it cost three of us one instance each today. `--only <paths>` separates FILES, not AUTHORS -- `0a9a7341` was scoped to 20 paths and landed 21 files, sweeping dc's #348 fix into a commit that says "no semantics". `cargo fmt` is not scoped by the files you care about either; vc hit the same wall from the other side two hours later. **Name the paths AND read `git diff -U0` before committing; a hunk you did not write is a stop.** Use `rustfmt <file>` when a peer is mid-edit.
- **NEVER DRIVE A MUTATOR ON THE LIVE ESTATE TO SEE WHAT IT DOES.** `intent issues hydrate 0001` wrote `ISSUE:0001` into `intent/.intentfiles`. Reverted, and the finding was real, but a probe is not a test and the estate is not a fixture.
- **A COUNT OF OCCURRENCES CANNOT ESTABLISH WHAT THEY MEAN.** INV-04's `measured across 108 probes` established that exit 2 OCCURS and was read as what it SIGNIFIES; one occurrence was named for the wrong cause and **two nodes agreeing on the count added no safety.** Only the branch producing a value says what it means. **This one would have propagated into a BUILD, not a report.**
- **AN INVERTED PREMISE GENERATES PROPOSALS TO RATIFY ITSELF.** INV-04's error had already produced an open question asking hv to move three conditions so the wrong reading could be preserved -- flagged highest-priority-of-19 _because it had a live consumer_, which is exactly why it would have travelled fastest. **Check the rows that DEPEND on a row you are correcting.**
- **A RED ROW SAYS WORK IS OWED; AN UNNAMED GREEN FILE SAYS WORK IS DONE.** The orphan test was strictly worse than the red one beside it, and nothing in the estate could have raised it: no row, no owner, and it passed.
- **`intentsvcs` IS THE DEPENDENCY ROOT: a peer mid-edit there stops all three of us.** Announce the BLAST RADIUS, not just the files.
- **THE LIVE CHANNEL IS UNGUARDED.** The clock guard covers board files and inbox entries; SendMessage is not hooked, and hv's own instruction moved nearly all traffic there. vc's stamps were 30 minutes in the future and were caught only because a reader ran `date -u`. **When a peer's stamp is load-bearing for ORDERING, check it before reasoning from it -- and use commits when you need ordering you can prove.**
- **THE BASH TOOL'S SHELL IS ZSH AND ITS WORKING DIRECTORY PERSISTS BETWEEN CALLS.** A `cd` in one call silently relocated later ones and produced `rc=127` and `ugrep: No such file`, both of which read exactly like real failures. **Use absolute paths.** Unquoted `$var` does not word-split; use `bash -c` for probes. Backticks in a double-quoted `-m` are command substitution -- heredoc through `-F -`. **Never `$?` after a pipe** -- `cmd | head; echo $?` reported `rc=0` for four verbs that were exiting 2. An `&&` chain after a zero-match grep silently skips the next command.
- **THE MARKDOWN FORMATTER IS A SECOND WRITER, AND THE GENERATOR KNOWS.** `gen_dispatch_table.sh` refused my first render because prettier normalises `*italic*` to `_italic_` and the view would not have been a fixed point. Write `_..._` in table prose.
- **A GREEN CAN BE A FACT ABOUT THE ESTATE RATHER THAN ABOUT THE PROPERTY**, and an assertion can pass on its own input echoed back. Every new bucket and every new refusal needs a positive control that is not the thing under test -- and the control must not be a mutator.
- **STILL TRUE:** run the workspace not the crate; a mutant that does not compile is not a red; a mutant that changes no behaviour is not a survivor; `ls`/`stat` without `-u` print LOCAL; `grep -c` exits 1 on zero.

## Decisions

- (2026-08-20) **hv RULED FOUR; ALL FOUR WENT AS ic AND vc JOINTLY RECOMMENDED.** Issues canon-and-store only with `ISSUE:` out of the grammar; AC-05.2 next; top-level `intent edit` the one home; the two-region API deleted. Full text in `.history/20260820/` and every peer inbox.
- (2026-08-20) **`intent critic`'s USAGE-ERROR EXIT 2 IS CORRECT AND STAYS IN v3** (ic ruling, on dc's question of whether it is a v2 defect). The gate states the principle itself: _a gate should fail open on its own breakage and closed on yours_ -- and a critic that cannot parse its own invocation IS the gate's breakage. So 0 clean, 1 findings, 2 usage, 3 refused.
- (2026-08-20) **A DECLARED-BUT-UNIMPLEMENTED VERB IS ITS OWN BUCKET, AND THE BUCKET IS DRIVEN.** Its writes are provably EMPTY, not unproven; filing a known zero as debt inflates the debt and hides the fact. `declared_but_unwired.rs` measures the CAPABILITY -- does the binary refuse? -- never a name, which is the distinction AC-08.5 paid for.
- (2026-08-20) **`COVERED_ELSEWHERE` FOR `st hydrate` NAMES `facade_hydrate.rs`, NOT `organize_idempotent_mtime.rs`.** Hydrate runs a plan FILTERED to one artefact. **A citation that cannot go red is not a cover.**
- (2026-08-19) **`intent hydrate <address>`, WITH A BARE ARTEFACT ID PROMOTED TO ONE.** Landed `21c0ccf2`. The promotion DELEGATES rather than guesses. A malformed argument is a USAGE error naming both forms, never a not-found.
- (2026-08-19) **A REALISED ARTEFACT IS ONE WHOSE COVER VIEW EXISTS, NEVER ONE WHOSE DIRECTORY DOES** (vc). Dehydration removes the directories it emptied, `rmdir` semantics only.
- (2026-08-19) **FOUR CRITERIA LEFT THE PRECONDITION BLOCK WITHOUT BEING WITHDRAWN** -- AC-03.6, AC-06.3, AC-06.4, AC-07.5. The block is about what GATES, not what is wanted; every one is still owed.
- (2026-08-19) **hv's GIT GROUNDS RETIRE A PRECONDITION ONLY WHERE GIT CAN SUBSTITUTE FOR THE PROOF.** AC-00.3 was a reversibility proof and git substitutes exactly; AC-07.5 is an ACCESSIBILITY claim and reaching for git falsifies its subject.
