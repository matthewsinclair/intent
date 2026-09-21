# Triage of the Conflab and Baize holds "until hv rules" (2026-09-21)

**UNVERIFIED EVIDENCE, NOT A RULING.** hv ruled that vc triages these holds first, strikes the ones that have resolved, and brings hv the rest as TUI questions (hv decision 26, item 8). This file is the evidence a read-only subagent gathered for vc on 2026-09-21. vc has not re-driven it. Before any row reaches hv as settled, vc re-measures that row. The measurements ran on the pair then installed (`intent 3.2.0 (850918a7...)`), with sources read at Intent HEAD f4b54ab52. No `intent` command ran inside Conflab or Baize, and no write verb ran anywhere; a row whose evidence is a source read says so.

Row format: **Q** the question, **M** the measured state, **V** the verdict, **Choice** the decision for hv.

## Conflab

**C1. conflab/cc/1, cc preamble (a), and ic's contract-prose hold (merged).** Q: release or keep the contract-prose hold, which asks whether prose runs at all before Intent ships an edit verb. M: the verb exists. `intent ac edit` rewords a criterion or writes an unsatisfied one's note, and `intent at edit` takes `--file`, `--prose`, `--kind`, `--covers` and `--note`. The refusal and the edit verbs shipped together in Intent 8aebe2ce. No Conflab row has been repaired yet: 14 unsatisfied ACs carry 0 notes. V: JUDGEMENT ONLY; the missing-verb premise is resolved. Choice: release the hold so rows are repaired with `ac edit` and `at edit` (with or without vc's fixture proof first), or keep it.

**C2. conflab/cc/2.** Q: must Intent 0131 ship refusal and an edit verb as one change? M: it did (8aebe2ce), and 0131 closed on 2026-09-11. V: RESOLVED.

**C3. conflab/cc/3.** Q: should the four `acceptance.v2.md` files stay unarchived, since only they hold the evidence clauses the hop destroyed? M: all four are tracked, byte-identical to their canon attachments, and fenced by `.prettierignore`. Canon has 14 unsatisfied ACs with no `state.note`, and the v2 files carry `-- evidence:` on all 14. 0133's fix protects only later hops, and Conflab hopped on 2026-08-28. V: STILL OPEN. Choice: keep them until the 14 notes are restored with `intent ac edit <ST> <AC> --note` and then archive; keep them indefinitely; or archive them now.

**C4. conflab/cc/4.** Q: Intent 0118 calls `st repair` an rc 2 stub, but the verb was retired. M: 0118 is closed (4a477acae), and `intent st repair --help` answers that the verb was retired in v3. V: RESOLVED. Its closing claim about `--date` is wrong for Conflab (see C6).

**C5. conflab/cc/4a.** M: c57c8c8f replaced the stale line. V: RESOLVED.

**C6. conflab/cc/5.** Q: `st done --date` cannot write Conflab 0010's 50 undated threads. How do the dates get written, and does this go upstream? M, by source read: the self-loop return still comes first (facade.rs:11732, `if from == status { return Ok(Outcome::AlreadyThere {..}) }`), ahead of the date arm. dc drove it on 2026-08-28 and got rc 0, `ok: ST0071 already Completed`, with nothing written. No test covers `--date` on a Completed thread, and no Intent issue tracks the gap. Conflab canon has 49 completed and 1 cancelled thread with no `completed` date. `intent set <ST> completed <YYYY-MM-DD>` now exists (1f2f8f6a2); by source read it does no date validation, and it was not driven. V: STILL OPEN, and an Intent defect: a stated date is dropped at rc 0. Choice: apply the 50 dates through `intent set` (drive one first); file an Intent issue and wait for the fix; or both.

**C7. conflab/cc/6.** Q: `/in-finish` step 1 had one verb for session end and localfold. M: fixed in 63e19f603, and the installed skill matches canon. V: RESOLVED.

**C8. conflab/cc/7.** Q: `pickup` never reads the roster README. M: `Facade::wb_pickup` returns the node's own board, hv's live directives, watch-outs and decisions, and peer headers, but never the README. Conflab has no hv board, so its pickups show no standing items. V: STILL OPEN. Choice: move the README's rules onto an hv board as directives (needs C9); add a README step to a procedure; or accept it.

**C9. conflab/cc/8.** Q: scaffold an hv node or not? M: `intent/whiteboard/` holds cc, dc, ic and vc, and no hv. The README names `intent claude ws new`, which is retired (its remedy is `intent wb register`). V: STILL OPEN, and it merges with C8. Choice: register an hv node, or keep escalation in-session only. Either way the README's scaffold command is dead.

**C10. conflab/cc/9.** Q: ic is blocked until cc writes ST0121/AT-06.1 to AT-06.4. Dispatch cc? M: none of that code exists; AT-06.1 to AT-06.6 are `to-write`, and WP-06 is not started. V: STILL OPEN. Choice: dispatch cc, or leave ST0121/WP-06 parked.

**C11. conflab/cc/10.** Q: go or no-go on ST0124/AT-01.7 and `Run::add_attempt_usage`? M: both are unbuilt, and WP-01 is done, so AT-01.7 would be a new row under a closed package. The obstacle cc named (`--no-fail-fast`) is gone. V: STILL OPEN. Choice: go or no-go on each unit.

**C12. conflab/cc/11.** Q: fence the live inboxes from prettier. M: `.prettierignore` fences `intent/whiteboard/`, and `prettier --file-info` reports the boards as ignored. V: RESOLVED.

**C13. conflab/ic, ST0121/AC-03.5 "renders".** M: every covering AT is on the daemon side, and none renders. AT-03.18's note is still damaged, and the one renderer still collapses the distinction (C14). V: STILL OPEN, gated behind C1. Choice: reword AC-03.5 to what the daemon tests prove, or keep "renders" and add an app-side AT.

**C14. conflab/ic, the two provider-key questions.** M: `ModelsStepView.runProbe` is unchanged since 2026-08-05. It has one red arm for four statuses, speaks the daemon's reason in the app's own voice, and never uses `ProviderKeyOutcome`. V: STILL OPEN. Choice: on attribution, attribute the daemon's reason or keep absorbing it; on taxonomy, split rejected from indeterminate or keep one arm.

**C15. conflab/ic, ST0123/WP-06's precondition.** M: tasks.md:15 still gates WP-06 behind ST0121/WP-02's live check, which was closed as "ACCEPTED UNVERIFIED" with no sitting recorded. V: STILL OPEN. Choice: treat hv's D2 as discharging the precondition, or keep WP-06 waiting on a live sitting.

**C16. conflab/ic, the 0011 correction.** M: `intent issues edit` exists (dbd0e5ab4). Conflab 0011 is open and unchanged since 2026-08-28. V: JUDGEMENT ONLY; the "cannot be edited" premise is resolved. Choice: correct it in place, file a separate correction record, or leave it.

## Baize

**B1. baize/cc, ST0025 WP-02 and the AC-02.3 narrowing.** M: hv ruled Held-1 on 2026-08-24, and WP-02 is done. What remains is cc's cleanup: AC-02.3's canon text still says "PENDING hv", and AC-01.3 was never narrowed. V: RESOLVED; a stale "PENDING hv" record remains.

**B2. The allowlist unit.** M: Held-2 was ruled on 2026-08-24. V: RESOLVED.

**B3. `hv/wip.md:21-22`.** M: Held-8 was ruled and executed. V: RESOLVED.

**B4. baize/ic's standing rule "an escalation is an inbox entry".** M: hv's Held-8 ruling partly supersedes it. V: RESOLVED.

**B5. baize/vc, the devbin channel.** M: vc withdrew it on 2026-09-02. V: RESOLVED.

**B6. baize/vc, Finding C's fleet broadcast.** M: Baize's own gate was fixed at 8a27312, and no Intent issue records the finding. V: JUDGEMENT ONLY. Choice: send it to intent-vc to broadcast or file, or drop it.

**B7. baize/vc, does the full stop still bind Baize?** M: Intent's vc recorded the identical hold, then "HOLD RELEASED 2026-09-01 17:58Z by hv, unqualified". Baize vc's board has not been written since 2026-09-03, and Baize's hv board never recorded the stop. V: STILL OPEN on Baize's record. Choice: confirm that the 17:58Z release covers Baize, or restate a Baize-specific stop with a new condition.

**B8. baize/vc, Held-16.** M: it was parked "Re-ask before Baize work resumes", and the pause was lifted on 2026-08-27, but hv/wip.md separately files it as deferred with no trigger. V: JUDGEMENT ONLY. Choice: ship Phase 1+2 as a unit; ship serially after WP-03; or re-park it with a trigger.

**B9. baize/vc, "the acceptance gate cannot fail".** M: `intent at` has no verify or run mode, and `ac gate` reads stored state. Intent carried the finding with three options until the 2026-09-11 cull, and there is no ruling and no issue. V: STILL OPEN. Choice: file it upstream for an execution path; accept `ac gate` as a contract-consistency gate and say so; or rely on discipline.

**B10. The third stamp surface.** M: already upstream in whiteboard-clock-guard.sh (27b13f932), with a bats test. V: RESOLVED.

**B11. baize/vc, TODO 1 under the stop.** V: JUDGEMENT ONLY, and moot if B7 is released.

**B12, B13.** RESOLVED (vc had already marked one; the other is history).

**B14. The heartbeat inside `run_gate`.** NOT MEASURED; this is vc's own verification work.

**B15. `dormancy` is inert.** STILL OPEN as a true statement with no ask.

## Dangerous or urgent, in the subagent's order

1. Baize vc may be idle under a stop hv lifted on 2026-09-01 (B7). One word from hv settles it.
2. `st done --date` silently writes nothing on a Completed thread (C6). It returns rc 0, while Intent's closing record for 0118 says the loop is fixed.
3. The only copy of 14 destroyed evidence clauses is the `acceptance.v2.md` set (C3, C1).
4. A stale "PENDING hv" sits in Baize's contract (B1), and Conflab's README names a retired command (C9).
