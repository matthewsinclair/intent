# vc board content archived 2026-08-13 (superseded ST0050-era state, v2.14.0 cycle)

## DOING

- (idle) ST0050 validation complete. Awaiting hv's next instruction.

## TODO

- Watch for the 2.14.0 tag: hv's sticky-watermark sign-off is the only open gate (see Decisions). If hv revises the model, re-audit the change.
- Offer stands: independent read on ST0051 (width, closed 9/9) if hv wants it.

## Watch-outs

- 2.14.0 tag hinges on ONE hv decision: the DONE watermark is sticky (no automatic daily sweep) -- faithful to `design.md:46` but reverses the original "swept daily" (`design.md:32`). hv accepts as-is or asks cc for an auto-sweep. Full audit in `cc/inbox.vc.md`; escalation in `hv/inbox.vc.md`.
- Two LOW post-release notes with cc: JSON/markdown enumeration Highlander duplication (`intent_todo` `emit_bucket` vs `emit_bucket_json`); `acceptance.md` AT ids not matching the bats `@test` names.

## Decisions (superseded entries only -- the standing fire-on-claim decision stays live)

- (2026-07-02) hv RATIFIED cc's ST0050 D1-D4 rulings (in session).
- (2026-07-02) ST0050 as-built audit: VERDICT PASS/ship-clean. 23/23 verified meaningful (gate + watermark + flush/prune tests assert real behaviour); ISO `completed:` stamp + tolerant membership + directory-based DONE membership all correct; the earlier D1 double-listing worry is resolved by the as-built. One acceptance decision escalated to hv.

_(Archived at pickup 2026-08-13: all items resolved by v2.14.0-v2.18.0 having shipped; the AT-name traceability note graduated into issue 0017's link-by-id design.)_
