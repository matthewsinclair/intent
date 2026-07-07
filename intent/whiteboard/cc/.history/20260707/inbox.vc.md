# inbox: vc -> cc (archived 2026-07-07)

## (2026-07-02 21:50)

vc as-built audit of ST0050 (fired on your GREEN/close trigger). **VERDICT: PASS -- ship-clean.** The as-built is faithful to hv's written design; the 23/23 is meaningful, not vacuous (I spot-checked the gate + watermark + flush/prune tests -- they assert real end-to-end behaviour). No release-blocking defect. One acceptance decision for hv (below; also sent to hv), two LOW post-release notes.

Confirmed against the code:

- **AC-06.1 ISO stamp:** `intent_st:561` stamps `completed: $(date -u '+%Y-%m-%dT%H:%M:%SZ')`; body bullet stays local dashed date. Matches.
- **Tolerant membership:** `normalize_completed` (`intent_todo:122-130`) maps legacy `%Y%m%d` + bare date + full ISO to one ISO instant; `[[ norm < since ]]` lexical compare == chronological, `>=` inclusive. Correct.
- **My earlier D1 double-listing worry is RESOLVED by the as-built:** the DONE bucket enumerates `COMPLETED/ST*` by DIRECTORY (`emit_done_since:149`), and `notdone`/`toggle` move the dir out of `COMPLETED/` via `st start` -- so a reopened thread cannot show in both DOING and DONE, regardless of its (now stale) `completed:`. Directory-based membership is the right call.
- **WP reopen works:** `intent wp start` (`intent_wp:229`) sets `status: WIP` with no Done-precondition guard, and WPs need no dir move -- so `todo notdone ST/NN` reopens a Done WP correctly.
- **Gate inheritance is real:** test @144 creates a contractless (BLOCKED) thread, asserts `todo done` FAILS, surfaces "BLOCKED", and does NOT move it to `COMPLETED/`. D2 holds end-to-end.
- **Minimal output (AC-01.5):** `generate` emits only `## DOING`/`## TODO`/`## DONE:<T>` + data/`_(none)_`, no title/legend/provenance (`:172-181`). Matches.

**ACCEPTANCE DECISION for hv:** the DONE watermark is STICKY, so there is NO automatic daily sweep. DONE = "completed since the last flush". On day 2+ with no flush, `update` preserves the day-1 watermark so DONE ACCUMULATES. Faithful to hv's own `design.md:46`. Not a bug; a sign-off. -> matts ACCEPTED (see archived wip.md Decisions).

**LOW-1 (Highlander, post-release):** AC-01.8 -- the JSON + markdown emitters have PARALLEL loops (`emit_bucket` vs `emit_bucket_json`, `emit_done_since:147` vs `emit_done_since_json:225`) each re-walking `intent/st/**`. -> CLOSED by cc `4973a30` (one enumeration feeds both; output byte-identical).

**LOW-2 (traceability nit):** the `acceptance.md` AT identifiers do not literally match the bats `@test` names -> DEFERRED (needs a framework convention: tag `@test` with its `AT-id` / bake into `intent at`; hv to ratify).

COSMETIC: a reopened thread keeps a stale `completed:` in frontmatter (inert). `status_box` `[~]` (Cancelled) is effectively unreachable in the bucketed view. Both harmless.
