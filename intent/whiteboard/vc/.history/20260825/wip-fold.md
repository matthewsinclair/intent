# vc fold -- 2026-08-25

**THE DAY WAS TWO DEVBIN-SOURCED SHIPPED-SURFACE DEFECTS, RULED BY hv INTO BOTH TREES, LOCAL ONLY.** Both landed and both verified by driving rather than reading. Long-form kept here; the live board carries only what is still open.

## What landed and how it was verified

**TEMPLATE -- `2fc66d8f` / `4836d667`.** `_CLAUDE.md` interpolated `[[INTENT_VERSION]]` into PROSE at `:3` and `:54`, `:3` outside the preserved region, so a consumer's correction was reverted on every upgrade. **It exhibited HERE across a MAJOR** -- our own `CLAUDE.md:3` said v2.19.0 against a 3.0.0-dev project. Fixed by dropping the token from `:3` (ic's argument, not mine -- see Decisions) and aligning `:54` to `_AGENTS.md`. Verified: same blob object in both trees, `bats` rc=0 27 ok, and **the mutation driven by me** -- old mark vs new template gives NO MATCH, so the new control fires red on the exact regression.

**GUARDS -- `e28c215b` / `db15e857`.** `whiteboard-clock-guard.sh:246` and `whiteboard-header-guard.sh:201` ran `printf | grep -q` under `set -uo pipefail`; grep exits on first match, printf takes SIGPIPE, pipefail promotes 141, the test reads FALSE, and **since that pipeline is the _did THIS COMMIT add it_ filter, a real violation was classified as inherited and passed through.** `canon-ignore-guard.sh:133` was the same idiom, safe only by omitting pipefail. Verified: five files identical by object id, both trees, and **driven end to end against a 170KB fixture that took the old guard from 0/5 to BLOCKED 5/5**, negative control silent-not-mute.

## The regression I verified green and missed

cc's template commit broke the marker the shipped tool uses to recognise its own output (`canon_claude_md_is_generated`, a CONTIGUOUS string). Every consumer's CLAUDE.md would have been declassified, and the documented recovery `--force` destroys the `user:start` block -- **worse than the defect it fixed.** devbin-cc found it. **I had checked that the change was what was intended and never asked what else READS the string.**

Repaired by anchoring on the PATH rather than the prose, which is **backward compatible** -- both old and new footers recognised -- where restoring the old sentence would not have been.

**AND MY DIAGNOSIS OF IT WAS WRONG TOO.** I told cc nothing under `tests/` referenced the recogniser. True of my grep, false of the property: I searched for two IDENTIFIERS and the test asserts the string's VALUE. `intent_claude_upgrade.bats:86` had asserted it all along and went red immediately. **cc's reframing is the keeper: a missing test is a backlog item, an unrun test is a discipline failure, and only one of those is fixed by writing more tests.**

## The verb neither ic nor I looked for

Both of us concluded no CLI verb regenerates `CLAUDE.md`, having grepped the `agents` and `sync` families. **`intent claude upgrade` was named in hv's own defect statement and twice more in devbin-cc's report.** It exists, is surfaced, names CLAUDE.md and the marker in `--force`, and is PARSED BUT UNWIRED (rc=2 against a `claude rules list` rc=0 control). ic caught it and inverted their own finding. **I had "independently confirmed" it, which made two nodes agreeing off the same wrong question look like corroboration.** The answer was in the question and neither of us re-read the question.

## Withdrawn, in full

I told hv the header guard was **one large fold from deterministic fail-open**. Withdrawn: I compared a real byte count against an onset measured on synthetic NARROW lines. Driven at real shape -- 29 real live-board payloads over 15KB, worst-case match position, 2900 trials -- **zero losses**; under 12-way load at ic's real payload, 4000 iterations, zero. devbin-vc challenged the instrument rather than the conclusion and the instrument was where the defect was.

**The surviving reason is better than the withdrawn one:** the trigger predicate is UNCHARACTERISED, so no under-the-limit argument of any form is available, and board SHAPE is a property nobody controls or reviews.

## Instrument failures, mine, this day

- **A grep for IDENTIFIERS reporting on a defect in a VALUE**, its silence read as absence -- and the bad premise propagated into devbin's diagnosis before cc killed it with the file. devbin-vc's generalisation, better than mine: **the negative result of a lookup by name is only as good as the name.**
- **A stale `Intentv2` HEAD** given to two peers, caught by both independently.
- **A sweep whose pathspec was not the guards' own**, returning `.history/` archives to 159KB that both guards exclude by construction -- population 4x too large, and **the only thing that made me look was that the numbers were better than expected.**
- **`bin/` searched when the tool lives in `intent/plugins/claude/bin/`**, nearly reporting a true and serious finding unreproducible.
- **An rc lost to `${PIPESTATUS[0]}` in zsh**, while verifying a defect about lost exit statuses. devbin-vc did the identical thing with `head -4` within the hour.

cc's correction, taken: reading `views.rs:222` for one question and not connecting it to another is **join-watching, not an instrument failure**, and no discipline fixes it. Counting it as an error made the real ones harder to see. Tally dropped.
