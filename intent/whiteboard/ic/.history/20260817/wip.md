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

---- fold 12, archived 2026-08-17 09:35Z ----

## DOING -- MIRROR ARM MEASURED AND DELIBERATELY NOT SHIPPED; NEXT IS THE MCP TOOL LIST

**The mirror arm is DONE as a measurement and REFUSED as a check, and the refusal is the finding.** Exactly **1 of 107** shipped rows promises a capability that exists nowhere: `backup` says `... for fast local restore` and no family ships a `restore`. Discriminator is structural, not a suppression list -- **the FIRST WORD of a help string is the command's own action, verified 107/107 with zero exceptions** -- so `ac rescope` / `ac reinstate` / `lang remove` correctly fall out and `backup` alone survives.

**NOT SHIPPED because neither available detector is defensible.** The DERIVED one (`a help word that is a shipped verb elsewhere but not here`) is blind to `backup`, since `restore` is shipped nowhere and there is no row to match. The one that SEES it needs a vocabulary of inverse verbs I invented, whose misses (`recover`, `roll back`, `undelete`) are silent. **You cannot derive, from a file describing what the tool HAS, a check for a promise of what it DOES NOT HAVE** -- the register is closed over its contents and the promise points outside it. Shipping it would assert coverage I cannot defend. Measurement sent to vc 03:28Z as evidence for hv's scope call.

**AC-09.1 AUDITED, AND THE ONE REAL GAP IS RAISED WITH vc (03:33Z) RATHER THAN EDITED.** Four things checked, three of them clean negatives I would otherwise have "fixed" redundantly:

- **The declaration half is SOUND at both ends.** All 112 rows carry `exposed_on_mcp` + `read_or_mutate`; both are deliberately non-`serde(default)` with the reasoning documented, AND `MCP_UNDECLARED` in the generator refuses type-or-value violations over the full `[.families[].entries[], .new_surface[]]` population with the trap named in its own comment. **I went looking for a missing refusal and found it asserted twice, covering different attack paths** -- a hand-edited JSON is caught by serde at load, a `serde(default)` added later is caught by the generator.
- **`guide_refs_check.sh`'s retirement hole is ALREADY FIXED** (`RETIRED` + `is_retired()`, both call sites). My board's note predated the fix; building the derived retired-command arm would have been a Highlander violation of my own making. **Measured the class anyway: ZERO live rows name a retired command in user-facing text.**
- **All four `recoverability_anomaly` rows RENDER IN FULL into the committed view**, note and all. The findings are visible, not buried in JSON.
- **THE GAP: AC-09.1 specifies `schemars-derived parameter schemas` and says NOTHING about the tool's own description.** `at green`/`at red`/`at na`/`ext new` are `exposed_on_mcp: true` AND `one-way`. **My guide projects safety FIRST so a CLI agent meets 0033 at the point of call; an MCP agent on the current AC text sees a parameter schema and silence.** The identical destructive verb warns on one surface and not the other, and the silent one has less context. **Free to close now because the tier is UNBUILT; a change to shipped agent behaviour after WP-09.** Recommendation invents no policy -- it applies the D45 projection order the guide already proves.

**THEN: vc's 0044 REVERSE SWEEP HAS THE POPULATION ERROR, AND I CLOSED IT RATHER THAN ONLY REPORTING IT.** The sweep says _"309 probes, the whole declared surface (104 entries)"_ -- **104 is `.families[].entries[]` alone; declared is 112 and SHIPPED is 107.** Too narrow by the 8 `new_surface[]` rows (all shipped, four of them `one-way`) and too wide by the 5 retired ones that cannot answer. **104 against 107, opposite signs, three apart -- no count-based check would flinch, which is vc's own sharper formulation of this class turned on vc's own rig.** Ran the 6 ordinary rows myself (18 probes, their method, sandboxed): **TYPE-A and TYPE-B both zero, so their RESULT holds** -- and the six **independently reproduce 0044's claim**, spending exit `1` on five distinct conditions (missing arg / unexpected arg / no project / no such schema face / file-not-found). `daemon` + `mcp` stay excluded for a stated reason. Sent 03:36Z + 03:38Z.

## DOING -- NOTHING IN FLIGHT, NOTHING BLOCKED, NOTHING OWED OUTWARD

**Nine commits pushed to local this session, ending `eea52021`.** Verified: generator clean at 105 entries with every refusal arm silent, nine parity checks exit 0, **surface_check exit 0 at 108 paths / 7-of-7 invariants from a `git archive` EXTRACT**, and `cargo test -p intent-cli --test self_loop_population` exit 0 at 3/3 with all three mutation-proved.

**THE EXTRACT IS THE POINT, NOT A WORKAROUND.** The working tree would not compile for most of this session -- cc's `Outcome` sweep had `render.rs`, `facade.rs`, `model.rs` and three test files dirty at once, and I hit E0533 then E0425. Rather than wait or measure their tree, I built from `git archive` of my own commit. **That is the discipline dc and vc used ON me this morning, applied to my own work for the first time.**

**hv's SUITE IS LIVE AS THIS IS WRITTEN and the FORMAT leg will go red.** `cargo fmt --check` shows diffs only in cc's in-flight `render.rs` and `self_loop_voice.rs`; my own files are clean. **Same shape as the 11:38 run: not a regression, cc's tree.**

**AND I HELD THIS WRITE FOR A REASON THAT TURNED OUT NOT TO EXIST, WHICH IS THE DAY'S CLASS ONE MORE TIME.** I stopped editing because `whiteboard_protocol_3_guard.bats` reads a board -- **it reads a board in a `create_test_project` temp fixture, and so do the clock and header guards. None of the three touches a live node's file.** Right instinct, unchecked premise. **Caution on an unverified premise is still an unverified premise; it just fails quietly** -- and had I not gone and looked, this watch-out would have entered the canon asserting the guard reads live boards.

**AND THE CORRECTED VERSION IS NOT "THE LEG CANNOT SEE MY WRITES", WHICH IS TRUE ONLY OF BOARDS.** vc measured five files matching `intent/whiteboard` and said so; the wider spelling (`whiteboard|wip\.md|inbox\.`) matches THIRTEEN, and their conclusion survives it -- no test reads a live node's `wip.md` or `inbox.*.md`. **But four DO resolve through `$INTENT_HOME` / `$INTENT_PROJECT_ROOT` to real files**: the `in-whiteboard` SKILL.md (`whiteboard_protocol_3_guard:22,140,142` and `claude_with_intent:172`), both hook templates (`whiteboard_clock_guard:21`, `whiteboard_header_guard:29,228`), and `usage-rules.md` (`docs_completeness:203`). **Those are exactly the files someone edits while thinking about whiteboard protocol, which is when they are least likely to ask.** Both of us verified the negative we were asked about; neither asked what the leg actually reads.
