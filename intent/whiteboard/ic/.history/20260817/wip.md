# ic -- archived 2026-08-17

## fold 10 -- what landed 2026-08-16

## DOING -- NOTHING BLOCKING. AC-09.4 CLOSED; PICK THE NEXT THING UP FRESH.

**AC-09.4 IS COMPLETE END TO END.** `intent llm guide` renders 952 lines from the table compiled into the binary -- I built the renderer (`2a654db3`), **cc wired it (`e8f2e444`) rather than leave it unreachable**, and the exemption they added to my `unmigrated_surface.rs` is correct and stays as written. No committed guide file, no hand-maintained list; completeness is structural, and the spec records the one thing it asked for that could not be built (there is no generator run at which to refuse, so the refusal is a test).

**`recoverability` LANDED at `9cd9a9ba` -- field, consumer and checks in ONE change, which was vc's condition and it held.** Declared on all 63 shipped mutations; `check_vocabularies` refuses a mutation lacking it, a read carrying it, and any value outside `recoverability_values`, at BINARY LOAD, naming the row.

**THE OUTCOME THAT JUSTIFIES THE WHOLE DETOUR: the guide renders `intent at green` as ONE-WAY.** vc measured 1,447 characters of authored contract destroyed by that verb (issue 0033) and **14,253 standing at risk across 34 rows**. **An agent now meets that at the POINT OF CALL, in the guide, rather than in an issue nobody reading a guide will open** -- and nobody had to remember to write a warning, because the field derives it.

**`acts_upon` was DISPROVED by the canary before it shipped, and that is the canary working.** vc's condition exists for exactly this and it spent itself on their own proposal. The probe is DELETED, not kept: it carried 63 hardcoded paths the table now owns.

**Two rulings of vc's are implemented as ARMS and the second one is the one I would not have built**: an undeclared disagreement refuses, and a STALE anomaly -- a note surviving after its disagreement is gone -- ALSO refuses. That is the half a known-exceptions list never has.

## fold 10 -- closed questions, archived from the live board

Closed at fold 9, recorded only because re-opening them from the wrong end is the expensive mistake:

1. **The D45 safety residue is RULED, not pending** -- see TODO 1. What I sent as an open question came back as a field with a condition attached, and the condition is the deliverable-shaping half.
2. **0039 clause 2 is CLOSED, by me, at 19:32Z: `surface_check.sh` does NOT want its own copy of the class check.** cc's Rust guard asks serde what it deserialized; **a shell script can only search text, and a text search has to know its needle** -- which is exactly why `surface_check.sh` was blind to `aliases`. Not an oversight in the script, a limit of the mechanism. One witness, in the place that cannot go stale independently of the thing it measures.
3. **`critic`'s no-language divergence is recorded** at `bcfeb135` as `target.wp07_owes`, `target.state` left at `pending-hv`. v2 exits 2 from its own arg parsing, v3 exits 1 from clap under INV-02, and **v2's 2 was already in `observed.exit` -- what was missing was the v3 side and the obligation.**
4. **The window flag question is CLOSED and my question had a defective premise.** **All six `todo` verbs regenerate `todo.md`**, so a window flag on any single row is a silent-revert generator. hv wants a PERSISTENT PREFERENCE; vc ruled the home is `intent/.config/config.json`, default 24h, read by the one render path all six verbs share. **No surface row changes; the row that changes is a config key.**
