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

---- fold 16 DOING, superseded at fold 17 ----

## DOING -- NOTHING IN FLIGHT, NOTHING BLOCKED, NOTHING OWED OUTWARD

**Five commits pushed since the compact, ending `abbcaff1`.** `c3a54446` (all 27 `no_op` values + 0053), `d56305cc` (0056), `1d37702b` (coverage tool), `29d6f250` (deleting it), `abbcaff1` (`wp rescope` measured + 0056 sharpened). Generator clean, `surface_check` exit 0, `self_loop_population` 3/3, `dispatch_ssot` 18/18.

**THE WHOLE SESSION IS ONE ARGUMENT FOR MEASURING RATHER THAN TRANSCRIBING, AND IT WAS PAID FOR THREE TIMES OVER.** cc sent 21 `target.no_op` values from a real commit with a real sha -- the cheapest possible handoff to accept. Driving the binary instead found: **two values flatly wrong** (`ac rescope`/`ac reinstate` refuse at exit 1, 0053); **two rows with no reachable self-loop at all** (`todo notdone`/`todo toggle` refuse before any state is read, a third shape the `unexamined` sentinel could not express); and **one token that appears in no canon file** (`at na` prints `n-a`), which unwound into 0056. **None of the three is visible from the verb's source, and all three would have entered the register as clean `ok:` lines.**

**0053 IS THE MAXIMAL FORM OF vc's CLASS AND THEY HAVE RECORDED IT AS SUCH: a value cannot carry its own subject.** cc reasoned from `AcState::entry(kind)` -- correct about what the SUCCESS arm hands the setter, on a path the self-loop never reaches. Within one node the derivation is still in reach; **across a handoff only the result crosses, and a wrong-subject result is indistinguishable from a right one precisely because the reasoning that produced it was sound.**

**I BUILT A SECOND INSTRUMENT FOR A QUESTION vc ALREADY OWNED AND DELETED IT ONE COMMIT LATER.** `prose_row_coverage.sh` reported 4 uncovered rows; vc's reported 2; **three of my four were false zeros.** My needle knew `run_intent` and `bin/intent` and not `"$INTENT_BIN"` or `"$INTENT"` -- and `claude start` is driven as `run env ... "$INTENT" claude start cc`, seven times, in a file named for it. **Deleted rather than repaired: two instruments for one question is the violation, and a fixed second one is still a second one.**

**AND THE BINARY CHANGED UNDER MY READING, WHICH IS THE PROVENANCE LESSON OF THE DAY.** I verified `git status` clean, built, and started measuring. A peer then edited `render.rs` in this shared checkout and something rebuilt the binary **at the path I was reading** -- two binaries, one path, same name, no record in either of which tree it came from, separated by an mtime 31 seconds apart. **31 seconds establishes ORDER and not CONTENT.** Everything was re-driven against a `git archive` extract and all 27 reproduced, so nothing shipped wrong -- but I could not have told anyone that from anything on disk.

## FOLD 18 -- the fold-17 DOING, superseded by `0f87fc2c`

Kept verbatim. The second paragraph is the one that went stale: it says ONE deliberate red, and closing TODO-3 made it two -- the corrected rows are now asserted against their ruling instead of being unasserted. The day's argument in the third paragraph is unchanged and now has a second instance.

**Eleven commits pushed since the compact, ending `23dab98e`.** All 27 `no_op` values measured both halves; issues **0053** (filed, and cc has FIXED it at `1721e4bb`) and **0056** filed; the literal rows made executable; the five voices ruled; four wrong `observed` values corrected; `arg_values_note` corrected 3 -> 5; two rows corrected from cc's measurements.

**THE RUST LEG WILL SHOW ONE FAILURE AND IT IS MINE, DELIBERATE, AND MUST STAY.** `literal_stdout_parity.rs::every_literal_as_observed_row_matches_the_v2_bytes_it_declares`, 3 red of 7. cc asked for it to stay red and vc agrees; cc attributed it in one command with `cargo test --workspace --no-fail-fast` (71 legs, 510 passed, 1 failed, named). **The three reds are the three parity breaks I ruled BUGS** -- `ac satisfy` dropping ` by evidence`, `at red` and `at na` dropping v2's `-> ` arrow. It goes green when cc fixes them, and **a green here before that would mean somebody rewrote a template from the binary**, which is how `at na` came to carry `n-a` in the column that records v2.

**THE DAY'S ARGUMENT IN ONE LINE: EXECUTION IS THE ONLY FORM THAT CAN BE WRONG OUT LOUD.** Nine rows now carry an executable declaration -- setup argv, invocation argv, v2's measured bytes -- and running it found **five** parity breaks where prose had recorded none. `at red`'s `observed` column was CORRECT the whole time and the divergence sat unrecorded beside an accurate record of what it should have been, because nothing had ever compared the column to the binary.

**AND THE REGISTER'S OWN GROUND TRUTH WAS WRONG AT 4 OF 8 ON THE ONLY SUBSET ANYONE CHECKED.** Three dropped a suffix; **`at na` carried `n-a` -- v3's token in the column that records v2, on an `as-observed` row -- so a check comparing the two would have found agreement and CERTIFIED 0056 as correct parity.** 50 of 93 rows with an `observed` block are still `evidence_class: read`. vc's framing, adopted: an uncovered row is unknown, **a wrongly-recorded row is confidently wrong in v3's favour**, and a wrong entry does not fail -- it silently redefines the target.

## FOLD 19 -- the fold-18 DOING, superseded by cc landing all five voices at `d14cd0b5`

Kept verbatim. Every red it describes is now green, VERIFIED rather than accepted: no `template` line changed between `0f87fc2c` and HEAD, and `native/`+`surface/` were byte-identical to `d14cd0b5` when the legs ran, so the binary is what moved and not the column that records v2.

## DOING -- TODO-3 CLOSED, TWO DELIBERATE REDS, NOTHING BLOCKED

**`0f87fc2c` -- a ratified row is held to its RULING.** A `corrected` row now carries both templates: `observed.stdout_exact` is v2's measured RECORD, `target.stdout_exact` is v3's ruled REQUIREMENT. The target carries no `setup`/`argv` and **inherits the observed invocation**, so the two describe one command and their difference is the ratified deviation written in bytes instead of prose. Two tests, because the reds mean different things: an as-observed miss is a PARITY BREAK (fix the binary), a corrected miss is an UNIMPLEMENTED RATIFICATION (build the ruled voice), and restoring v2's bytes would be the wrong fix for the second.

**REQUIRING THE TWO TEMPLATES TO DIFFER IS WHAT FORCES THE RIGHT FIXTURE, AND THAT WAS NOT WHY THE GUARD WAS WRITTEN.** `ac rescope` on a NON-TEST criterion is ruled to print `back in scope (unsatisfied)` -- byte-identical to v2. Declared there the row would assert as-observed behaviour under a `corrected` label and look strict doing it. The difference-check drives the declaration onto the test-backed criterion, the only invocation where the correction is observable at all. **A corrected row must be declared where the correction is VISIBLE**, which is the fixture rule from this morning arriving on the other axis.

**AND RATIFYING THOSE TWO ROWS THIS MORNING KILLED `corrected_check.sh` AT EXIT 2 FOR ALL TWENTY CORRECTED ROWS.** It reads `target.ratified_in`; I invented `voice_ruling` where eighteen of twenty rows already used the canonical `ratification`/`ratified_in` pair. A Highlander violation in my own SSOT, and the blast radius was the whole axis rather than the two rows I touched. Fixed; rc=0, 11 cited / 24 claimed / 13 elsewhere.

**MY OWN RULING'S REASON WAS WRONG AND IT EXCUSED v2.** It said v2 "had no `computed` state to name". v2 HAS the concept -- `bin/intent_acceptance:16` and the refusal at `:1084`. What v2 lacks is the branch at the call site: `:1259` prints `(unsatisfied)` unconditionally under a comment at `:1252` reasoning only about the non-test case. **So v2 announces `unsatisfied` about a criterion whose satisfaction it computes elsewhere** -- vc's class again. Same destination, load-bearing difference in the grounds: v2 is wrong here, not model-limited. Measured against a fixture v2's OWN `at lint` validated first.

## FOLD 20 (2026-08-17 15:00Z) -- archived DOING

## DOING -- A ROSTER THAT REFUSES, AND A CORRECTION SWEPT ONE FIELD FURTHER THAN IT WAS REPORTED

**TWO LANDED AND PUSHED: `67814555` (0059) and `0a7cc84c` (0061).** Upstream still frozen at `5765c5da`; v3 still not on PATH.

**0059 -- THE GATE RUNS ELEVEN GUARDS NOW, AND IT RAN ITSELF ON ITS OWN COMMIT.** Six wired. But the mechanical half was never the point: 0059 says in as many words **do not add them without the disposition field**, so the deliverable is `runner_roster_check.sh` -- a `gated`/`manual` disposition per `*_check.sh` with a required reason, which **enumerates the directory every run and REFUSES on any tool it has no row for.** Naming the remainder in prose is a claim that rots on the day a tool is added; a roster fails on that day instead. Nine gated, four manual, zero undeclared.

**THE POPULATION HAD ALREADY MOVED BEFORE THE REMEDY WAS WRITTEN, WHICH IS THE ARGUMENT FOR THE REFUSAL.** 0059 counted eleven instruments at `0f87fc2c`; there are **thirteen**. `ratified_in_check.sh` landed hours after the census and `runner_roster_check.sh` is new -- both unwired, both unnoticed by any artefact. **My own tenth class, arriving inside the fix for it**: a correct measurement of a population that has silently acquired members. So the check never compares against a remembered count.

**DECLARED AND INVOKED ARE MEASURED SEPARATELY**, because the guard-0 rot in `precommit`'s own header was precisely their disagreement -- named in the roster, implemented inline, `int hooks` reporting three guards as two. It asks `--list-guards` rather than re-grepping the source (263ms of its 782ms), because re-grepping the source is what rotted the thing it is checking.

**AND `int hooks` IMMEDIATELY PAID FOR ITSELF: `corrected_check.sh` and `stale_at_check.sh` had no `+x`.** They run fine under `bash <path>`, which is exactly why nobody saw it -- the tool a human consults to find out was the thing that was wrong. Same shape as the rot above, two files over.

**0061 -- vc REPORTED TWO FIELDS; FOUR WERE STALE.** `at na`'s `help` was the only authored site left in the estate where the wire spelling reached a human, and it survived **because** it is authored: nothing renders help from the enum, so no code fix could pass through it. Corrected, then verified by **rebuilding from a clean extract of HEAD with only that edit applied** -- a grep proves the file changed, not the shipped string.

**`target.no_op` RE-MEASURED, NOT RETYPED, and the provenance rule earned itself twice in one day.** Three files under `native/` were dirty under cc, so the worktree binary would have attributed unlanded work to HEAD. Built from a `git archive` extract of `67814555`: movement `ok: AT-01.2 -> n/a`, self-loop `ok: AT-01.2 already n/a`.

**THE FOURTH FIELD WAS NOBODY'S REPORT: `at red`'s `voice_ruling` still asked for an arrow `d14cd0b5` had already restored.** Found by measuring the siblings of the row that WAS reported. **`at green` and `at red`'s `no_op` were checked the same way and are CORRECT** -- three rows measured, one stale, no assumption applied to a family. The line held throughout: **`target.*` must describe HEAD, `observed.*` must describe v2 as measured**, so the `n-a` in `observed.notes` stays.

**vc's OPEN QUESTION IS ANSWERED, AND THE ANSWER IS "NOT AS SPECIFIED".** 44 sha citations across 29 rows, 12 distinct shas. The cheap gate (`git log <sha>..HEAD` non-empty) would flag **11 of 12 shas, ~28 of 29 rows, on its first run** -- 0059's own warning turned on the remedy. **The discriminator it lacks is in the field names**: `no_op` (25) records a MEASUREMENT and goes stale; `ratification`/`ratified_in`/`scope_ruling` (12) record a DECISION and 94-commits-behind is their CORRECT state. Same cut as `target` vs `observed`, one level finer. Offered to vc, not built.

## (previous) DOING -- THE INSTRUMENT WENT GREEN FOR THE RIGHT REASON, AND THAT WAS CHECKED

**`literal_stdout_parity.rs` IS 4 OF 4 GREEN AT `d14cd0b5`, AND THE GREEN IS ATTRIBUTABLE.** cc landed all five voices: `ac satisfy` has ` by evidence`, the AT family has its `-> ` arrow, `at na` prints `n/a`, and both undo verbs print `ok: <AC> back in scope (<landing state>)` with the state computed from kind.

**A GREEN HERE IS THE ALARM CONDITION UNLESS IT IS ATTRIBUTED, AND THIS FILE SAYS SO IN ITS OWN DOC COMMENT.** Two cheap checks ruled it out rather than one expensive one: **no `template` line changed between `0f87fc2c` and HEAD** (so nothing was rewritten from the binary, which is how `at na` got `n-a` in the first place), and `git status --porcelain -- native/ surface/` was EMPTY, so the worktree WAS `d14cd0b5` and no extract was needed. **The provenance discipline earns its keep by telling the clean case from the dirty one, not only by refusing the dirty one** -- three times today it forced an extract, and this time it licensed the worktree in one command.

**cc BUILT THE RATIFICATION RATHER THAN RESTORING IT, WHICH IS THE DISTINCTION THE TWO TEST LEGS EXIST FOR.** The landing state is read back from the facade after the call rather than written as a literal, so the movement line and the no-op line take their spelling from one place and cannot diverge. No literal can name `entry(kind)`.

**AND THE THREE-SPELLINGS FINDING WAS FOUR.** cc found `views.rs::test_line` rendering `AtStatus` with `enum_str`, so the next projection over any thread with a non-test AT would have rewritten every such row from `n/a` to `n-a` -- **the wire form written into GENERATED CANON, on authored files, in a spelling v2's own linter rejects at L1.** 23 `status: n/a` against zero `n-a` across the estate. The other three sites damage a line of output; this one damages a file. **Their fix was unwitnessed until they checked** -- reverting it left 72 legs green -- and `view_determinism.rs` now derives the expected spelling from `display()`, so the mutant reds their view leg and my parity leg together. Two instruments, two layers, one spelling.

**FOUR OF SIX TODO ITEMS CLOSED THIS SESSION**, all pushed: the corrected-row template field (`0f87fc2c`), the arm- and line-scoped rows measured with two ruled `corrected` (`795a8ccb`), hv's 0052 ruling recorded (`5798af98`), and the provenance grammar guard (`81c3978c`, `38152015`). **Filed 0058 and 0060.**

## FOLD 21 (2026-08-17 15:35Z) -- archived DOING + TODO + open-with-others

## DOING -- FIVE LANDED, AND THE TRACKER WENT AWAY UNDER THE REGISTER THAT CITES IT

**PUSHED THIS SESSION: `67814555` (0059 + the roster), `0a7cc84c` (0061), `8a8d6179` (mutation proofs + a live false positive), `fef54072` (two half-applied closes), `337d7d84` (hv's `agents template` remedy).** Upstream still frozen at `5765c5da`; v3 still not on PATH.

**THE GATE RUNS ELEVEN GUARDS AND `runner_roster_check.sh` REFUSES ON ANY `*_check.sh` IT HAS NO ROW FOR.** 0059 asked for the disposition field in as many words, so the deliverable was never the six invocations -- it is that the remainder cannot silently become "covered". Nine gated, four manual with measured reasons, zero undeclared. **dc reviewed it and asked for no changes**, including the one I offered to reverse (do NOT quieten `ratified_in_check.sh`).

**0059 COUNTED ELEVEN INSTRUMENTS AND THERE WERE THIRTEEN.** Two landed after the census, unwired, unnoticed by any artefact -- my own tenth class arriving inside the fix for it. That is why the check enumerates rather than remembers.

**MUTATION PROOFS FOR THE TWO I GATED, AND ONE WAS RED.** `stale_at_check.sh` read the first backtick anywhere on a row as the citation, so an UNCITED to-write row with a backticked NOTE was reported as `cites <path> -- the file EXISTS`, pointing the reader at a test the row never named. Now scoped to the row head, before the first `--`. **Measured exposure when fixed: 0 of 53 -- latent, not live.** Prompted by cc from the other end: nine captured v2 rows carry a note that itself contains `--`. `class_vocab_check.sh` fired all four paths clean, including the heading mutation that would otherwise report every correct class as ungrounded.

**hv RULED THE ISSUE TRACKER OUT OF EXISTENCE AND THEY WERE RIGHT.** _"Issues are (generally) for other, external users of Intent, using a released version. We can just fix things inline as we find them."_ Then: _"Just clear the tracker, findings live in commits."_ vc cleared all 21 at `9a9c7799` -- REMOVED, not closed, because closing would assert they were resolved. My classification carried it: **19 of 21 were ST0056-internal, and the honest count of external reports against a release was zero.**

**MY SHARE OF THAT WAS REAL. 0060 AND 0061 WERE BOTH FIXED AND LEFT OPEN.** cc corrected my account of 0060 and the correction makes it worse, not better: they fixed it AFTER I filed, at `ffcc8764`, **with a comment citing the issue number** -- so the code knew and the tracker did not, and only re-driving the binary could tell the two apart. Finder and fixer both awake, both done inside the hour, neither closing.

**AND THE CLEARING BROKE A CITATION FORM THE REGISTER DEPENDS ON -- MEASURED, NOT SUSPECTED.** 24 `target` fields cite an issue number; **THIRTEEN cite only an issue with no sha, five of them `ratified_in`**, the field whose entire job is provenance. `ratified_in_check.sh` still accepts `issue NNNN` as a valid external record, **so it reports those five as conforming while their record leads nowhere.** The instrument has been confidently wrong since `9a9c7799`. One row re-anchored; twelve and the checker are TODO 1.

**hv's `agents template` REMEDY: `list` reports what each directory actually offers, `show` serves both kinds.** No name `list` prints is a name `show` refuses, and nothing becomes undiscoverable to achieve it -- narrowing `list` would have traded a false claim for a silent omission. Class and remedy kept as separate fields, because the class was decidable long before the remedy was.

## TODO -- ONE IS URGENT AND MINE; THE REST ARE SMALL

1. **RE-ANCHOR THE TWELVE `target` FIELDS THAT CITE ONLY AN ISSUE, AND TIGHTEN `ratified_in_check.sh` -- MINE, AND IT IS THE URGENT ONE.** The tracker is gone, so `issue NNNN` no longer resolves and the checker still calls it a valid record. **Five of the twelve are `ratified_in` itself.** Fix the checker first (an issue ref alone stops being a record; a sha beside it is fine), then re-anchor each field on the commit that holds the content -- recoverable with `git show 9a9c7799^:intent/issues/OPEN/<n>/...`. **Fixing the rows first would leave the instrument still wrong and green.**
2. **MEASURE THE OTHER TWO LAYERS OF THE OLD 0035 CLAIM, WHEN NEXT IN THAT CODE.** I drove the CLI arm of `ac satisfy --evidence ""` and it refuses at exit 1; the claim said "at every layer" and the declaration and facade were not measured separately. vc's read and mine agree it is strong evidence and not proof. **Not worth a special trip.**
3. **`st start` REFUSES FROM `triage` WHILE `st new --start` COMPOSES PAST IT. hv's, not mine.** Measurement still holds. Still the likeliest first-command bug report in v3.
4. **THE REGISTER-LAG DISCRIMINATOR -- MEASURED, OFFERED, DELIBERATELY NOT BUILT.** The cheap gate flags ~28 of 29 rows. The cut is measurement-vs-decision and probably wants a `records:` key declared on the field rather than a heuristic over field names -- vc agrees a declaration beats a guess, same argument as the roster. Schema change to my SSOT, so it wants a ruling before code.
5. **STANDING: mutation proofs CO-LOCATED, as each file is next touched.** `class_vocab_check.sh` and `stale_at_check.sh` came off the list this session; `runner_roster_check.sh` shipped with two in it.

## Open with others -- LIVE ASKS ONLY

**All four inboxes cleared to the sentinel at fold 20.** Peer traffic is on the agent channel at hv's instruction, and the inbox files are now durability only. **NOTHING IS OWED TO ME AND I OWE NOTHING BLOCKING.**

- **dc, CLOSED OUT AND THE BEST EXCHANGE OF THE DAY.** They reviewed the wiring, asked for no changes, and **agreed with all three of my push-back invitations including the one I offered to reverse** (do NOT quieten `ratified_in_check.sh`). Then they measured declared-vs-invoked themselves, got **9 against 11 from a hand-built pattern**, and were one step from reporting the guard-0 rot in the file that fixes it -- calibrating against the real command corrected it. Their three fixes at `79ca7eca` include one worse than what I reported: **`int hooks` was CLONING AND BUILDING to ask what was wired**, because its `--list-guards` probe matched the words in a COMMENT in `cmd/prepush`. 0.8s now.
- **cc, ONE CORRECTION FROM THEM THAT I ACCEPT AND SHOULD CARRY.** I said 0060 was already fixed when I raised it; **the order was the reverse** -- they fixed it after, at `ffcc8764`. So it is not a stale list, it is finder and fixer both awake and neither closing, which is the sharper version. They also warned that nine captured v2 AT rows carry a note containing a further `--`; **that warning is what found the `stale_at_check.sh` false positive.**
- **vc, LEVEL.** They cleared the tracker at `9a9c7799` and reversed themselves on my point 4 (whoever fixes it closes it). My register-lag measurement is recorded as the resolution rather than a note.
- **hv, ONE ANSWERED AND ONE STANDING.** `agents template`'s remedy is RULED and recorded (`337d7d84`). `st start` from `triage` is still theirs (TODO 3).

## FOLD 22 (2026-08-17 15:47Z) -- archived DOING + TODO + open-with-others, at the pivot to WP-10

## DOING -- THE DANGLING CITATIONS ARE GONE, AND THE ROOT COMMAND HAS NO ROW

**PUSHED: `b865db12` (the checker), `4cad12b8` (the provisional bucket + the re-anchors), `bd5dc51e` (EXP-09 + vc's `ac gate` rulings).** Upstream still frozen; v3 still not on PATH.

**TODO 1 IS CLOSED AND THE ORDER MATTERED.** Checker first, rows second. `ratified_in` now reads: 26 declared, **14 conform, 0 hv, 1 PROVISIONAL, 0 dangling, 0 non-conforming, 11 sentinel.** The only field waiting on anyone is `ac gate`, waiting on hv.

**THE ISSUE ARM BECAME A LOOKUP RATHER THAN BEING REVOKED, AND ONLY MEASURING SHOWED WHY.** `CLOSED/` survives at 40 and `at green` cites issue 0015, which is still on disk. A blanket revoke would have accused a live citation -- the false-alarm direction, the expensive one, and a failure this file already records itself making once. The test is whether the NUMBER resolves, which self-maintains: prune `CLOSED/` and the rows citing it turn amber with no edit.

**MY 13-AND-5 WAS TWO POPULATIONS IN ONE SENTENCE.** 13 is the no-SHA count; the grammar accepts a FILE too, so **7 fields lacked any record and 5 of those were `ratified_in`**. Both true of different sets, reading as 5-of-13. The denominator rule from `835bf848`, one hour old. Corrected in the register and in the commit.

**vc HELD MY REMEDY AND WAS RIGHT.** `ac gate` is `provisional pending hv`; a sha in `ratified_in` asserts a ratification happened, so the fix the report told me to make would have certified an unmade ruling -- **doing deliberately what the `AUTHORITY_HV` anchor had just stopped by accident, an hour apart, in the same field.** Third bucket built on their ruling, tested AHEAD of conformance so a provisional row with a good sha still does not read as conforming.

**AND vc TRACED AN hv ATTRIBUTION TO NOTHING, AGAINST THEMSELVES.** `why_corrected_and_not_as_observed` said "hv's own wording on issue 0032". Under it: the sentence is in the FILING text, the issue carries `reporter: matts`, vc's ruling quoted it as hv's, the field repeated it. Verified at source before I wrote the strike -- the diff introducing the sentence is `e23a8453`, a node's own commit. **It matters most because that row awaits an hv ruling, so the field manufactured the adjudicator's agreement on the question they have not been asked.**

**EXP-09, FOUND BY ANSWERING A QUESTION OF cc's.** They asked whether build metadata extends `intent --version --verbose` or takes a new row. **Neither: no entry declares the root intrinsics at all.** Measured against the built binary -- `--version` short-circuits before validation and **swallows everything after it at exit 0** (`--version --zzz`, `--version NOSUCHTHING`, `-V --zzz`), while `version --verbose` correctly exits 1. INV-02 unmet on the root command, and the `version` row's own `behaviour` asserts the opposite. Third instance after `info`'s catch-all slot and `st show`'s positional, and this one is the command every user types first.

## TODO -- HELD. hv HAS CALLED US OFF TRACK AND A RE-PRIORITISATION IS COMING

**0. THE MORATORIUM IS THE FIRST ITEM AND IT LANDS ON ME HARDEST.** vc gave hv a plain-language review and did not defend us: **210 commits today touched no code, 33 touched Rust; nothing moved from Not Started to In Progress; and the thing hv asked for -- Intent running on Intent3 -- needs WP-10, the migrator, which nobody is on and nobody ever has been.** vc's diagnosis is _we have built a very good immune system and it has started consuming the host_, and **my lane is the clearest exhibit**: every finding today generated a peer message, a class entry, a re-anchor and an instrument with eight mutation proofs. Correct for a shipping product, inverted for a rewrite that is half-built. **NOTHING BELOW IS OPENED UNTIL hv RULES.** Findings go in the commit that fixes them -- hv's own rule, which I have been decorating.

1. **THE ONE THING THAT IS ON THE SHIP PATH: declare `--verbose` on the `version` row when cc confirms with dc.** cc found the way through EXP-09 -- **`version --verbose` already exits 1**, so AC-11.5 lands on the subcommand, not on the swallowing root intrinsic, and it is a flag declaration on a row that already exists. **One row, no new class, no instrument.** Waiting only on cc confirming the spelling satisfies dc's provenance contract.
2. **HELD -- EXP-09's remainder.** (a) A home for the root intrinsics; (b) **`nothing_reaches_the_surface_that_is_not_in_the_table` passes with `-V` on the surface and undeclared, so its population excludes root flags** -- one line in the guard when it is touched, not a project; (c) the swallow-is-defect-or-intrinsic RULING, which is no longer on anyone's critical path now that cc is not building on it.
3. **HELD -- one schema change, two fields, vc has RULED YES and asked me not to build it yet.** `records:` on the field, and a declared `provisional` rather than a token in prose. Their reasoning is my own `AUTHORITY_HV` fix pointed at myself: a token in prose is substring-anywhere, and **a declared field cannot be negated into invisibility.** _Delete the word, do not negate it_ is the interim rule -- and it is a rule, which today established is not a control.
4. **HELD -- the other two layers of the old 0035 claim**, when next in that code. Not worth a special trip and never was.
5. **`st start` REFUSES FROM `triage` WHILE `st new --start` COMPOSES PAST IT. hv's, not mine.** Still the likeliest first-command bug report in v3.
6. **STANDING: mutation proofs CO-LOCATED, as each file is next touched** -- and **proportionate to the file's blast radius**, which is the correction the moratorium makes to this item. Eight on one register checker was more than the artefact was worth.

## Open with others -- LIVE ASKS ONLY

**Inboxes are durability only; peer traffic is on the agent channel at hv's instruction.**

- **vc, LEVEL AND THE BEST EXCHANGE OF THE DAY.** They held my `ac gate` remedy before I took it, then answered both follow-ups and struck their own attribution. Owed to them: nothing blocking. Owed by them: the schema ruling in TODO 2.
- **cc, AND THEY SOLVED THE THING I HANDED THEM.** I answered AC-11.5 with EXP-09 and three entangled blockers; **cc re-measured against their own binary rather than taking my message for the artefact, and found `version --verbose` exits 1** -- so AC-11.5 goes on the subcommand, off the swallowing root, and my three blockers come off their critical path. They are confirming the spelling with dc before I declare the flag. **Owed by me: one row, the moment they confirm.** AT-05.7 / `inert_arg_check.sh` is theirs, and they have already noted EXP-09 is a member it cannot reach, so the root intrinsics must be NAMED as excluded rather than silently outside its denominator.
- **hv, TWO STANDING AND ONE INCOMING.** `ac gate`'s ratification is provisional pending them; `st start` from `triage` is still theirs; and **the re-prioritisation is theirs to make** -- vc has recommended naming the hoist as the milestone with a moratorium on new classes and criteria until it lands.

## FOLD 23 (2026-08-17 17:37Z) -- WP-10 Phase B built; the accretion found, fixed by cc, verified by me

## DOING -- THE ACCRETION IS CLOSED AND THE CUTOVER GATE IS MEASURED PASSING

**hv gated the cutover on it: a second migration over an INTERRUPTED estate must reach the same end state as a clean one. It does.** Measured against committed HEAD `85ef4a72` with `git status --porcelain native/ schema/` returning **0 modified**, so no peer's in-flight work is in the reading -- the discipline I got wrong twice earlier today and right here.

```
CLEAN: 93 files written, 144-file tree
plain re-run 2 / 3        wrote 93, 10 already  ->  IDENTICAL
kill early (1st canon)    123 at kill; re-run 93,  1 already -> IDENTICAL
kill mid   (last canon)   132 at kill; re-run 93, 10 already -> IDENTICAL
kill late  (into views)   142 at kill; re-run 93, 10 already -> IDENTICAL
```

**Real `kill(SIGKILL)` on a child mid-`WriteSet::commit`, confirmed by `signal() == Some(9)`, not a simulated partial write.** Two attempts: killing on a GUESSED DELAY never worked -- 0 files at 12.8ms, because harness process startup outlasts every delay -- and killing on a SENTINEL FILE APPEARING lands it every time. No stray `.intent-tmp` survived any kill.

**AND THE REALISTIC SHAPE, WHICH MY FIRST FIXTURE COULD NOT SEE.** Mine put threads at the flat path; the real estate has 55 of 56 in `COMPLETED/`, and cc warned that is exactly what collides on 0011:

```
5 threads, all in COMPLETED/ -- run 1 wrote 55, 0 already; run 2 wrote 55, 5 already -> IDENTICAL
after: 5 thread dirs at the flat path, 5 still in COMPLETED/
```

No duplicate-id block, so cc's dedup holds. **And that last line is the bucket finding measured from the other end: the estate is doubled after a clean migration and the migrator is content.**

**WHAT IT DOES NOT ESTABLISH, RECORDED BEFORE ANYONE QUOTES IT.** Five and ten threads from ONE repo, all COMPLETED -- not the 56-thread canary, not the fleet. **And the verdict rests on my own throwaway tree diff, not on dc's `same_end_state_check.sh`**, which is me doing what I told dc three hours ago I would not do. Nothing landed, the scratch is deleted, but the gate has not been run under the gate's own tool and I am not reporting it as though it had.

**MY OWN MESS TODAY, BOTH OWNED:** a `zz_ic_*.rs` survived a 7-minute timeout in the shared `tests/` dir because my cleanup never ran -- my poll loop walked the whole tree 400k times, my bug, and it cost the sweep. Removed. And `74c4b357` **broke HEAD**: I committed the consumer of `Scan.already_migrated` while cc held the producer, both worktrees green because the tree held both halves. cc's `7628a02b` fixed it. **cc's generalisation is the durable one: every green here is a claim about the union of five people's uncommitted work.**

**LANDED AND PUSHED TODAY:** `ca71bd61` roster reads the index; `96d2bd4c` `migrate.rs`; `e591c9c4` `assemble` private; `94ef68b7` WP zero-padded path; `e0c813a8` `Blocked`'s remedy proof; `cac74720` **the residue check reads the index too -- the same defect in the sibling tool, which I fixed this morning in one and did not transfer**; `74c4b357` `already_migrated` through the join.

## TODO -- THE RE-RUN CONTRACT IS THE LANE

1. **THE RE-RUN CONTRACT IN `migrate.rs`, WAITING ON cc's `legacy.rs` HALF.** What `plan()` does over a partially-migrated estate. If the canon-wins design lands as agreed it should be a no-op on my side -- **and that is a claim to TEST, not to assert**, because "no change needed" is the shape that ships unverified.
2. **THE INTERRUPTION TEST, AND IT MUST BE A REAL KILL** (cc's condition, accepted): run, `SIGKILL` between writes, re-run, and the twice-run tree must equal a clean single run. Reading the code establishes nothing here -- the accretion was invisible to three careful readers and took one execution.
3. **HOLD BOTH TESTS UNTIL cc LANDS.** They are red until then, and I have landed red once today already for a reason that does not apply twice.
4. **DELETE THE COLLISION PRE-SCAN when `WriteSet::add` becomes fallible.** Still cc's, still recorded in the variant's doc.
5. **THE EVENT LOG QUESTION IS STILL UNASKED AND STILL MINE.** Deliberately not sent -- it blocks nothing today, and the moratorium is about not generating questions that block nothing.

## Open with others -- LIVE ASKS ONLY

- **cc -- owed to me: `export.rs` (minutes), then the sections fix.** Owed by me: nothing. Boundary holding well; they took all four `Blocked` variants and moved their guard on my placement argument, I took their `BTreeMap` correction and their `issue_add` framing (the door is a property of the ACT, not of the entity).
- **vc -- on the tools, and we are working the same seam from both ends.** They carry the `organize` warrant to hv. Their `conservation_check.sh` now predicates on reachability rather than presence, which is the correction that came out of Finding 1.
- **dc -- two-ended migrations.** Their AC-10.4 half and my Finding 2 are the same axis in opposite directions: **a record whose INTERIOR is dropped (mine) and a record that survives byte-for-byte whose REFERENTS stop resolving (theirs).** A conservation check built for either is silent about the other. vc has it.
- **hv -- FOUR, all via vc's batch or held for conversation.** The `organize` warrant; contract ownership; `ac gate`'s ratification still provisional; `st start` from `triage`.

## FOLD 24 (2026-08-17 18:47Z) -- the gate ran on a commit and PASSED on files

## DOING -- THE GATE RAN ON A COMMIT, AND THE ONE DIFFERENCE WAS A PREDICATE PROBLEM

**THE RUN, EVERY INPUT A REVISION.** Binary from HEAD `4770b6d9`, cc's real `intent upgrade` (`8770cea3`), dc's comparator `11f66894`, vc's pinned canary `42fb5269`. Real `SIGKILL`, not a simulated partial write.

```
arm A      wrote 295 files (1077 -> 1372)
arm B      SIGKILL at 293 of 295   ("Killed: 9", exit 137 asserted)
re-run     exit 0
clean-only 0   rerun-only 0   differing 1   ->  intent/.cache/intent.db
```

**1371 OF 1371 NON-STORE PATHS IDENTICAL after a real kill at 99% depth.** Repeated at a second depth (263 of 295), same result.

**I RAN THE TWO-CLEAN-RUNS CONTROL BEFORE THE GATE AND IT IS THE ONLY REASON THE EXIT 1 DID NOT READ AS A RECOVERY FAILURE.** Two migrations, no kill anywhere, also differ in `intent.db`. **The verdict was unreachable by construction** and nothing in the output said so. vc's framing, taken: had the kill run first, that exit 1 would have been believed by me and by everyone I told.

**THEN I GOT THE DIAGNOSIS WRONG AND dc CAUGHT IT.** I argued the store should leave the gate's subject, on two grounds, **both VOID in canon**: D29's _"a path git can never commit can never be canon"_ is marked VOID at `design.md:243`, and _"`rm intent.db` is safe"_ is on D01's do-not-cite list word for word. **D01 is REVERSED -- the DB is the SSOT and the FILES are re-creatable** -- so excluding the store would have dropped the one artefact the model calls truth. I verified both by reading the file rather than accepting the report.

**dc's `.dump` IDEA FOUND THE REAL ANSWER AND IT IS NOT PAGE LAYOUT.** Measured:

```
bytes                        DIFFER
sqlite3 .dump                DIFFER          <- so not a container artefact
.dump, ISO8601 ms normalised IDENTICAL       <- 827 lines, ZERO residual
created_at / updated_at      DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ'))
705 rows carry a stamp of when the migration ran
```

**The SSOT records wall-clock at insert, so two runs of a PERFECT migrator can never produce the same database** -- permanently, by construction. Not a subject to be narrowed: **a subject with the wrong predicate.** dc reproduced every number from a different harness (same byte 4796, same 705) and landed it at `884fd97f`, keyed on the file's magic bytes rather than its path.

**AND IT IS MY OWN PARKED QUESTION ARRIVING WITH TEETH.** Item 4 on this board all day -- _migrated threads restore an `st.new` carrying the authored date; `Envelope.ts` is ms-precision; `created` is a DATE_ -- filed as blocking nothing. **It blocked the gate**, and three of us hunted the symptom from outside for an afternoon while the cause sat on my board. dc's read of the tell is the durable part: **I could not say what would happen if the answer went either way**, which is a better test than "does this block anything".

**dc ASKED THE QUESTION THAT COULD UNPROVE THEIR OWN FIX AND IT HAS AN ANSWER.** A field that SHOULD hold an authored timestamp and instead holds the run time is swallowed: both runs substitute identically, both normalise, the gate goes green. General form -- **the normaliser cannot tell "correctly a machine stamp" from "should have been authored and was overwritten".** It does not unprove the fix: a two-run comparison cannot catch ANY deterministic defect, and this widens that pre-existing blind spot by exactly one bounded class. The right instrument is estate-versus-canon, which is one tree, and I am taking it.

**MY OWN RIG HAD A DEFECT OF THE SAME SHAPE AND I MEASURED IT ON MYSELF.** I replaced the `find`-based count with a single `test -e` for speed **and left a `date +%s` fork in the same loop** -- milliseconds, in a window where a file lands every 0.25ms. `--fraction 25` and `--fraction 90` both killed at 293. `$SECONDS` moved it to 263. **And the header claimed a control the rig does not have**: `--fraction` selects by MTIME order, but `commit()` temp-and-renames, so mtime is when the TEMP was written and `test -e` sees the rename -- the percentile lands late and approximately. Corrected in the file rather than reworded, because the claim had already gone out in a commit message.

## TODO -- ONE RE-RUN AND ONE TEST

1. **RE-RUN AGAINST `884fd97f` FOR A BARE VERDICT.** dc's content comparison should turn the single store difference into a match, and the result into exit 0 or a real finding with nothing to interpret. **Blocked only on the parity tools being clean** -- `same_end_state_check.sh` is dirty again as I write this. cc's floor fix has landed and `native/rust/` is clean.
2. **THE `intentsvcs` STORE TEST, cc's offer, taken.** `intent-cli` may never depend on rusqlite (D06, guarded by `dep_graph_guard.rs`) and shelling to `sqlite3` re-introduces the dependency canon removed (`design.md:290` -- Intent bundles SQLite precisely so it does not need the machine's binary). So the store comparison belongs in `intentsvcs`, which owns the store and may use rusqlite: compare row content directly, no shell, no regex over text. **Carries my parked event-log question with it, since it is the same measurement from the same side.**
3. **BACK cc's ARCHIVE-GROWTH GUARD and vc's CANON-HYGIENE ASK.** Mine is clean (checked by running the command, not by reasoning about my fold script).

## Open with others -- LIVE ASKS ONLY

- **dc -- owed to me: parity tools clean for one run.** They landed the content comparison at `884fd97f`, canaried three ways, and **the load-bearing arm is the one they nearly skipped**: a planted content change still caught. Checking what a filter neutralises proves nothing; checking what it must still catch is the whole thing. They also detect SQLite by magic bytes rather than path -- my own derive-don't-name advice applied one layer below where I had applied it.
- **cc -- owed to me: nothing.** Door at `8770cea3`, floor fix landed. **They cited D36 as ground for treating the store as disposable and D36 exists to forbid exactly that**; D34 (per-machine, never committed) carries it alone and always did. They found the void D29 reasoning at `sync.rs:132` four days ago and then rested a doc comment on the adjacent dead ruling -- **finding a struck premise once buys no immunity to the next one**, their words.
- **vc -- nothing owed either way.** Lamplight BLOCKED, both classes DECLARED, **5613 of 5613 byte-identical after the refusal** -- AC-10.2's atomicity turned from my argument-from-a-type-signature into a measurement, by an instrument built for something else. They traced the void-premise class to their own memory file still serving the pre-reversal model verbatim from a stale record.
- **hv -- bucket relocation (via vc); contract ownership; `ac gate` provisional.** New from today, vc holding: **how canon records a REPLACED derivation.** A struck premise beside a live conclusion has now caught three of five nodes in three places, and none of us was careless -- all three went looking for a citation first.

---

# FOLD 25 -- 2026-08-17 20:33Z

## DOING -- THE GATE I PASSED YESTERDAY CERTIFIED A DEAD ESTATE, AND THAT IS THE SESSION

**`intent upgrade` EXITS 0 AND LEAVES A PROJECT THE TOOL WILL NEITHER READ NOR WRITE.** Measured on the gate's own kept `a-clean` tree with a HEAD `53057087` extract build, then on Baize, then from scratch:

```
rc=1   st list / todo / export --format json / search / ac list / wp list / st new
rc=0   info      -- the only verb that answers, and it reads no project state
```

**THE REMEDY THE ERROR PRINTS IS A CLOSED LOOP.** A second `intent upgrade` exits 0, reports `311 file(s) written` and `ok: this project is now Intent v3.0.0-dev`, and the estate still refuses. A command that reports success, writes 311 files, and changes nothing about the operator's situation -- recommended by the error it does not clear. (It also narrates `a previous run of this command was interrupted` on a tree that completed cleanly: a true observation with an invented cause, and arm B prints the same sentence where it is correct.)

**MECHANISM.** v2 buckets a thread by status FROM CREATION, the migration writes canon to the flat `intent/st/<ID>/` and correctly leaves the v2 originals alone, and `Project::legacy_thread_ids` then counts those same originals as unmigrated -- it asks whether a `thread.json` sits BESIDE each `info.md` rather than whether the thread's id has canon anywhere. cc's note is the one to keep: the one-level descent was a CORRECT fix for a real hazard, and the migrator's ordinary output now puts every project into exactly the shape it was built to catch.

**CONFIRMED BY BOTH PEERS ON THEIR OWN SUBJECTS, AND vc's NUMBER IS THE HEADLINE: bucketed-only = 0 on utilz, baize and this repo.** Not one thread named `carrying v2 canon this binary cannot read` is unmigrated; every one has a `thread.json` written by the run being refused. **100% false positives, fleet-wide.**

**MY MINIMAL REPRO IS FOUR COMMANDS AND IT CORRECTED THE POPULATION.** `intent init` / `st new` / `upgrade` / `st list` -- 6 files, N=1, no corpus, no capture, no extract build. The empty project is the positive control: 0 buckets, migrates, reads clean. So the population is **not** "every project that has completed a steel thread" (cc's first reading, and mine when I repeated it) but **every v2 project with any thread in any status** -- the only unaffected estate is one nobody has used. vc led the hv escalation with it.

**AND I NEARLY SHIPPED THE OPPOSITE FINDING.** My bucket-free control returned exit 0 with a rendering table, and had migrated **1 thread instead of 56** because the buckets ARE the canon pre-migration. A green produced by an emptied subject; the naive reading of it is advice to delete 55 threads. **Caught only by checking what the run MIGRATED rather than what it RETURNED.** vc took it to hv as mine. Their correction completes it: deleting buckets AFTER migration leaves every thread readable, so the flat canon is complete and the two experiments differ by instant -- **delete before, lose the threads; delete after, lose the 192 STRANDED files.**

**THE STANDING ITEM IS DONE -- `38d5cda7`. THE SUBJECT IS A CLONED REVISION AND A DIRTY WORKTREE NO LONGER MATTERS.** `--rev` (default HEAD) is cloned into the workdir and built there, so the subject is a named commit by construction rather than by inspection. **The dirty-tree refusal is GONE**: right about the hazard, wrong about the remedy -- it made the gate unrunnable exactly when the estate was busiest, fired four times in one day, and I routed around it by hand-building extracts four times in one session. **A guard routed around that often is telling you where it should have been.** **dc caught a factual error in the design BEFORE it was built**: `git archive` leaves no `.git`, so cc's embed would read `unknown` every run and the provenance cross-check would return one answer regardless of input -- **I had inferred the mechanism from the OUTPUT of a worktree build.** It is a clone. Instruments come from a revision too (`--instruments-rev`, defaulting to the subject's, both named when they differ). **And I hit one of dc's own hazard class on the way, in the false-green direction**: `estate_corpus.sh` derives ROOT from its own location, so from a clone lamplight/utilz/baize resolve `no-repo` -- **but CANARY RESOLVES `here` EITHER WAY**, because canary IS this repo and a full clone carries its pin, so the default member could have gone green with the bug latent for every other one. dc's form: **when a fixture passes, ask whether it could have FAILED.** The durable half is the line it forced: **an instrument's LOGIC belongs to the pinned revision; its view of where repositories live on this machine does not.** Eight controls, and the one that matters is the gate green while the worktree was dirty with three files across two nodes.

**AND NOBODY WHO MIGRATED BEFORE THE FIX NEEDS TO RE-MIGRATE** -- measured on a Baize tree written at 19:02Z by the PRE-FIX binary and never touched since: same 25 `thread.json`, pre-fix binary refuses at rc=1, post-fix names all 25. **Same bytes, different reader**, so the estates written this morning were always correct and only the reader was wrong. vc's isolation (re-read, never re-migrate) answers that and my own method -- re-capture and re-migrate -- structurally could not. Getting there I hit **zsh word-splitting for the FOURTH time today**, and this instance returned `rc=126, threads named=0` on BOTH binaries, which reads as "the fix does not work on pre-fix trees": a false finding, in the expensive direction, distinguished from a real one only by the `126`.

**LANDED: `d487d30f` (the two arms) and `1898c22d` (the bound).**

- **LIVENESS** -- probe the migrated arm A with a read verb before the sentinel. **Refuses at exit 2, not exit 1, and the distinction is load-bearing**: exit 1 asserts non-idempotence, which this does not support -- the interruption property may hold on an unreadable estate, and yesterday it did. The refusal names the current cause and says to trust the output over the note when it goes stale.
- **STORE** -- closes dc's `1 store(s) NOT JUDGED BY THIS TOOL` with no `sqlite3` and no rusqlite in the CLI. `intent export --format json` reads the STORE (`facade.rs:1146`), not the files. Byte-deterministic across two independent migrations: 225553 bytes. **Bounded after dc's question to `the store as export sees it`** -- `file_index`, `doc_sections`, `snapshots` are named as out, all empty today, which is why the bound is written now.
- **FOUR CONTROLS, BOTH DIRECTIONS EACH**, because an arm seen in one state cannot be told from a hard-coded one: real `st list` refuses at exit 2 before the kill dance; `READ_CMD=true` passes through; an agreeing store read reports IDENTICAL at exit 0; a disagreeing one reports DIFFERENT at exit 1 **and overrides a files-identical verdict**.
