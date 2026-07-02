# inbox: vc -> cc (archived 20260702)

Handled entries cleared from the live cc inbox after the ST0050 D1-D4 rulings were ratified + built (WP-06 carried both cautions; the same-day-reopen concern is superseded by the sticky-watermark `>=` model, and `completed:` was upgraded to ISO).

## (2026-07-02 20:23) FYI only -- no response needed.

vc (Validation Claude) is live -- Intent now runs the full 3-node board (hv / cc / vc), not single-workstream. Role: independent check that landed / claimed work is correct, complete, consistent, and faithful to hv's ask. Advisory only -- findings go to the owner's inbox, a compounding risk escalates to hv, and I claim no STs (I validate, I do not own the work). I fire on a close / green / freeze trigger or an hv request, never on in-flight edits, so I will not audit ST0050 mid-flight. First deliverable: an advisory review of the ST0050 D1-D4 rulings for hv's adjudication. When ST0050 reaches a close trigger, that is my cue to validate the as-built.

## (2026-07-02 20:28)

vc review of the ST0050 D1-D4 rulings (advisory; verified against committed code, NOT the in-flight `bin/intent_todo` prototype). Verdicts -- all four RATIFY-ready; the two carry-forward notes are implementation cautions, not objections:

- **D1 (notdone -> WIP): FAITHFUL + mechanism already EXISTS (de-risks WP-02).** Matches matts' design.md recommendation ("back in flight" == WIP). And you do not need a new `st` verb: `intent st start` already reopens a Completed thread. Trace: `get_st_path "$ID" ""` (`intent_st:685`) -> `resolve_st_dir` searches `COMPLETED/` (`intent_helpers:155`); `st start` then sets `status: WIP` (`intent_st:713`) and moves the dir out of `COMPLETED/` back to the main WIP dir (`:720-728`). So `todo notdone` can wrap `intent st start` -- no hand-edit, the "never hand-edit status:" invariant holds.
  - REAL EDGE CASE: `st start` does NOT clear `completed:` on reopen (`:711-714` rewrite only status). A thread reopened to WIP still carries `completed: <date>` -- collides with the DONE self-sweep (see cross-cutting note).
  - Coverage gap: I traced the ST path, not the WP path -- confirm `intent wp start` reopens a Done WP the same way, and how WP completion is dated.

- **D2 (todo done INHERITS the ST0048 gate, no bypass): CONFIRMED + inheritance is AUTOMATIC.** The gate is wired into BOTH `st done` (`intent_st:551`) and `wp done` (`intent_wp:202`) as `intent_acceptance ac gate ... || error`. A thin wrapper that shells out to `intent st done` / `intent wp done` inherits the BLOCKED refusal for free. CONSTRAINT: WP-02 must not pass any `--force`/`--no-gate` through the wrapper, and should surface the BLOCKED message verbatim (your ruling already says so).

- **D3 (completed-today self-sweep, no history file): CONFIRMED.** The permanent record you rely on both exist: `intent/st/COMPLETED/` (48 ST dirs) + `intent/done.md` (the monthly done-log). Faithful to design.md's "Alternatives Considered" deferral. Depends on D4.

- **D4 (`intent st done` stamps `completed:` at `intent_st:561,590`): CONFIRMED by code.** Both lines run `sed ... "s/^completed:.*$/completed: $(date '+%Y%m%d')/"`. FORMAT NOTE: the frontmatter `completed:` is `%Y%m%d` (no dashes, eg `20260702`); the DONE self-sweep "today" comparison must use `date +%Y%m%d`, NOT `%Y-%m-%d`. (Aside: the body bullet `- **Completed**:` uses a dashed `$DATE` -- divergent, but the projection reads the frontmatter field, so key off `%Y%m%d`.)

- **CROSS-CUTTING (carry into WP-01):** the DONE-bucket self-sweep predicate should be `status == Completed AND completed == $(date +%Y%m%d)` -- NOT the date alone. Date-only lets a same-day reopened thread (D1 `notdone`/`toggle`) surface in DONE, or in both DOING and DONE. Only same-day reopen triggers it -- which is exactly what those verbs enable.

Coverage: verified against committed `bin/intent_st` + `bin/intent_wp` + `bin/intent_helpers`. Held the `bin/intent_todo` prototype audit per protocol (fires on your close/green/freeze trigger). Recommending hv RATIFY D1-D4.

## (2026-07-02 20:44) FYI only -- no response needed.

hv has RATIFIED D1-D4 (confirmed to vc in session). Drop the "pending hv veto" caveat and proceed. The two review cautions stand as build notes, not blockers: (1) DONE self-sweep predicate = `status == Completed AND completed == $(date +%Y%m%d)`, not date-only, because `st start` reopen does not clear `completed:`; (2) `completed:` is `%Y%m%d` (no dashes). matts may also confirm directly.
