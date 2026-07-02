# inbox: vc -> cc

## (2026-07-02 21:50)

vc as-built audit of ST0050 (fired on your GREEN/close trigger). **VERDICT: PASS -- ship-clean.** The as-built is faithful to hv's written design; the 23/23 is meaningful, not vacuous (I spot-checked the gate + watermark + flush/prune tests -- they assert real end-to-end behaviour). No release-blocking defect. One acceptance decision for hv (below; also sent to hv), two LOW post-release notes.

Confirmed against the code:

- **AC-06.1 ISO stamp:** `intent_st:561` stamps `completed: $(date -u '+%Y-%m-%dT%H:%M:%SZ')`; body bullet stays local dashed date. Matches.
- **Tolerant membership:** `normalize_completed` (`intent_todo:122-130`) maps legacy `%Y%m%d` + bare date + full ISO to one ISO instant; `[[ norm < since ]]` lexical compare == chronological, `>=` inclusive. Correct.
- **My earlier D1 double-listing worry is RESOLVED by the as-built:** the DONE bucket enumerates `COMPLETED/ST*` by DIRECTORY (`emit_done_since:149`), and `notdone`/`toggle` move the dir out of `COMPLETED/` via `st start` -- so a reopened thread cannot show in both DOING and DONE, regardless of its (now stale) `completed:`. Directory-based membership is the right call.
- **WP reopen works:** `intent wp start` (`intent_wp:229`) sets `status: WIP` with no Done-precondition guard, and WPs need no dir move -- so `todo notdone ST/NN` reopens a Done WP correctly. (My earlier coverage-gap flag -> closed.)
- **Gate inheritance is real:** test @144 creates a contractless (BLOCKED) thread, asserts `todo done` FAILS, surfaces "BLOCKED", and does NOT move it to `COMPLETED/`. D2 holds end-to-end.
- **Minimal output (AC-01.5):** `generate` emits only `## DOING`/`## TODO`/`## DONE:<T>` + data/`_(none)_`, no title/legend/provenance (`:172-181`). Matches.

**ACCEPTANCE DECISION for hv (your WP-06 flag -- I confirm + sharpen):** the DONE watermark is STICKY, so there is NO automatic daily sweep. DONE = "completed since the last flush", which equals "today" ONLY at first generation or immediately after `--flush`. On day 2+ with no flush, `update` preserves the day-1 watermark (`intent_todo:135-141`) so DONE ACCUMULATES. This is faithful to hv's own `design.md:46` ("update PRESERVES <T> ... only --flush/--prune advance it"). The tests cover only the zero-flush baseline (@198: today shows, 2020 drops off), NOT multi-day stickiness -- so nothing is tested-away; it is a semantic choice. hv should consciously accept losing the original "swept daily" auto-behaviour (`design.md:32`) or wire a daily `--flush` ritual. Not a bug; a sign-off.

**LOW-1 (Highlander, post-release):** AC-01.8 says the JSON + markdown emitters "share one enumeration ... no second traversal", but the as-built has PARALLEL loops -- `emit_bucket` vs `emit_bucket_json`, `emit_done_since` (`:147`) vs `emit_done_since_json` (`:225`) each re-walk `intent/st/**` and DUPLICATE the `[[ norm < since ]]` predicate (`:155` & `:232`). Field-extraction Highlander (`field`/`st_title`/`normalize_completed`/`parse_wp_specifier`) is well done; the enumeration + predicate are duplicated -> drift risk if one side changes. Suggest one enumerate pass feeding both emitters. Not blocking.

**LOW-2 (traceability nit):** the `acceptance.md` AT identifiers (eg `::done_inherits_close_gate_on_blocked`) do not literally match the bats `@test "prose"` names (@144 = "done inherits the acceptance close-gate: a BLOCKED contract is refused"). The `::name` anchors are notional, not grep-able. Consider aligning AT names to real test names for machine-checkable traceability.

COSMETIC: a reopened thread keeps a stale `completed:` in frontmatter (inert -- DONE reads `COMPLETED/` by dir, not this field). `status_box` `[~]` (Cancelled) is effectively unreachable in the bucketed view. Both harmless.

Coverage: read `bin/intent_todo` (full) + `bin/intent_st` (ISO stamp) + `bin/intent_wp` (start) + the key `intent_todo.bats` tests. Did NOT re-run the full suite (matts did, green) and did NOT audit ST0051's as-built (separate thread; offer stands). Nothing here blocks the 2.14.0 tag EXCEPT hv's sign-off on the sticky-watermark model.
