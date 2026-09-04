---
node: cc
name: Control Claude
role: control
session_id: 98a46c38-f370-4d67-b2c5-c2536e0ae8f9
commit_session_id: 01XYetoGJWvBxvL4PE8sGZTu -- HARNESS-ANNOUNCED ONLY, NOT WITNESSED. I have authored no commit this session, so there is no trailer to read it off and the honest provenance is the announcement alone. dc reads theirs off the artefact; mine is one source short of that until I commit. POINT-IN-TIME, one session; a restart mints a new one.
heartbeat_at: 2026-09-04 15:11Z
status: active
focus: "BOTH COMMITS LANDED ON matts APPROVAL: eaef2a04f (browsed honest refusal + the browse roster note) and 0b5d46c96 (ST0065 Option 2, index in both templates, mutation-tested drift arm). SPLIT because they are separate concerns, and EACH PATH SET VERIFIED INDEPENDENTLY first -- a set passing together is not evidence about its subsets. Post-verified: two files each, nothing swept. render.rs handed to ic and dc, both pinged. THE FREEZE WAS NEVER THE ROSTER GUARD: it was dc MM index/worktree divergence, and I quoted my one live roster reading for hours without re-taking it. ic found the sharp part -- git update-index --add reads the WORKTREE, so every synthesised index we built contained the fresh bytes BY CONSTRUCTION and could not represent the defect it was diagnosing. STILL UNCOMMITTED AND FLAGGED TO matts: 0232, my daemon-conformance gap issue, is UNTRACKED, so it exists in store and on disk but not in git. NO FIGURES FROM MEMORY; RUN THE VERBS."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**XS BUILT, DRIVEN, GREEN, AND UNCOMMITTED.** vc re-sequenced it ahead of WP-06 after reading my split -- their words: _my throwaway reasoning was right about one half and I applied it to both._ The serving half is still WP-08; the false claim was separable.

**WHAT LANDED IN THE WORKING TREE.** `render.rs` `browsed()`: the refusal now names what this build does not do (`--browser is not implemented in this build`) instead of asserting a world-state it never measured. The old remedy sent an operator to `intent daemon start` while one was already answering. Doc comment rewritten to record the premise that rotted rather than quietly dropping it. **`write_moves_only_what_changed.rs`'s `browse` roster note corrected too** -- my own change falsified it, and leaving it would be the same defect one level up.

**DRIVEN, NOT INFERRED.** `cargo check -p intent-cli --all-targets` and `-p intentsvcs --all-targets` clean; 11 suite tests green (`remedies_are_reachable`, `twin_spellings_agree`, the web-face browser test); and the message itself driven through `target/debug/intent` with `intent daemon status` answering rc=0 in the same minute. **The release pair was NOT rebuilt -- that is `0196` and outside my pen** -- so the shipped binary still emits the old message.

**THE PROBE IS DELIBERATELY NOT CALLED, AND FINDING THAT OUT IS WHY BUILDING BEAT RECOMMENDING.** My own recommendation to vc said call `running_daemon_pid()`. **Wrong.** With no serving half, both of its branches refuse identically -- so calling it today computes an answer the function discards, which is machinery wearing the costume of a check. It goes in with the arm that can act on it.

**TWO FINDINGS THE BUILD PRODUCED THAT THE READING DID NOT.**

- **`every_emitted_remedy_names_something_this_build_can_do` PASSED ON THE FALSE REMEDY, AND PASSES ON THE FIX.** It checks that the verb a remedy names EXISTS; it cannot check that doing it changes anything. `intent daemon start` is a real verb, so the unkeepable remedy was green. **The estate HAS a remedy test and this class walks straight through it** -- which is why the false message survived every run.
- **ONE CAPABILITY, TWO SPELLINGS, TWO EXIT CODES.** `intent browse` answers `rc=2`; `intent edit --browser` answers `rc=1`. `INV-09` says they ARE the same capability and `ST0058 AC-00.6` refuses one present by one spelling and absent by the other. **RECORDED, NOT TAKEN** -- an exit code is a surface decision and `2` is INV-04's usage code the pre-commit gate fails OPEN on, so this goes to ic rather than into my diff.

**BLOCKED ON EXACTLY ONE THING, AND IT IS MECHANICAL.** dc announced they are about to write `render.rs` for `skills sync --dry-run`. **My change sits uncommitted in that file, and `--only` is path-scoped, not hunk-scoped** -- their `add + commit --only render.rs` would carry my bytes under their signature, silently, nothing red. Same shape as hv's `29b045527`. **Told dc at 10:1xZ with both sequencings offered; awaiting matts on the commit, since a code commit to main is matts-gated here in a way board commits are not.**

**WP-06 SIZED -- ITEM 2 DONE AND SENT TO vc. THE NINE ROWS ARE THREE BUILD UNITS AND MY `L` WAS THE WRONG SHAPE, NOT THE WRONG NUMBER.** All nine drive to three top-level verbs at `rc=2` unimplemented (`ext`, `config`, `learn`); every subcommand and flag form answers `rc=1 unrecognized subcommand` because clap has never heard of them. **Six of the nine are unreachable by construction and cannot be sized independently of their verb.**

- **`learn` -- S.** 203 v2 lines, one positional + two flags, storage `intent/.config/learnings.md` **consumed by `intent claude prime`** (the only non-obvious part). Conformance target: 1 bats file, 18 burning tests. **Startable today.**
- **`config` -- S to build and it is not a build problem.** v2 emits 0 bytes at exit 0 -- the `undefined` class, hv-ratified -- so v3 DESIGNS. `bin/intent_config` is primarily a LIBRARY (`bin/intent:211`), executable path near-vestigial, so 161 lines overstates it. **`bats_coverage: HOLE`** -- no conformance target, nothing to satisfy and nothing to confirm you. **The owed hv ruling is the gate; do not count it as build work until it lands.**
- **`ext` -- M, the only genuinely large one.** 820 v2 lines, 4 subcommands + a bare form defaulting to `list`, runs OUTSIDE a project under `~/.intent/ext/` (which exists here, with `worker-bee` in it). **41 burning tests across 3 files** -- much the largest target. Port hazard: v2 help leaks `validate [Session 3]` / `new [Session 4]`.

**A CORRECTION TO `restart.md`'s U3, ROUTED TO vc WHO OWNS THAT FILE.** U3 names four verbs "mandated in canon and unimplemented in v3": `lang`, `plugin`, `ext`, `version`. **Driven with a positive control first** (`intent st` 9, `intent wp` 8, so the instrument reaches): **`lang` 1 and `plugin` 1 are in canon; `ext` 0 and `version` 0 are NOT.** The eight bare-word `ext` hits are one seed README describing the `~/.intent/ext/` DIRECTORY and routing discovery through `intent claude subagents list`. **dc's _ext IS discharged_ is CONFIRMED.** Caveat stated against my own finding: `intent todo` also scores 0 and plainly ships, so canon-presence is a poor scope proxy -- this moves the JUSTIFICATION, not the inclusion.

**A HYPOTHESIS OF MINE THE EVIDENCE REFUTED, KEPT BECAUSE THE ELIMINATION IS THE RESULT.** I thought I had found my `implemented_check.sh` false-positive class: bare and armed probes disagree by construction (`claude subagents` bare is `rc=1` at clap with NO marker; `claude subagents list` is `rc=2` WITH it), so a bare-form probe would read no-marker as implemented. **REFUTED BY READING THE SCRIPT** -- it already fills leaf-row subcommand slots and its comments record hitting this exact thing three times, and the marker is DERIVED from `render.rs:92` with A/B1/B2 controls. **The XS stays open with one candidate eliminated.**

**ic's CATCH, AND IT CORRECTED MY OWN IN-FLIGHT DIFF.** ic measured all twelve `AC-08.*` rows satisfied while my `render.rs` comment called the same ground unbuilt. **`AC-08.9` is about the web face being an `intentsvcs` surface with two skins served by the same service call** -- the shell and `/op` ship, so it is honestly satisfied and says nothing about a per-entity page. **My sentence conflated a WORK PACKAGE with an AREA.** Corrected in the tree before it could land. **WP-08's contract is COMPLETE at 12/12, which is vc's standing close condition met, and the evidence is ic's.**

**AC-17.6 -- I ESCALATED IT AND I WAS WRONG, AND ic's REFUTATION IS BETTER EVIDENCE THAN EITHER OF US HAD.** I read `shell.html`'s _renders no view of your estate, deliberately_ as forbidding a browser face, and routed it to vc as a design collision. **The measurements were all correct and the INFERENCE was one step too far.**

**`intent/st/ST0056/tui-design.md` SECTION 10a IS TITLED _The renderers (D56 -- `intentd` emits JSON only)_ AND DESIGNS THE BROWSER FACE IN DETAIL.** Verified at source after my first grep returned zero against `docs/design/tui-design.md`, **a path that does not exist** -- the zero was my instrument, not ic's claim, and I positive-controlled before concluding anything. Line 361: _plain ES modules, served same-origin from the binary_. Line 375: _the browser splits vertically -- list left, detail right_. Line 368 even solves the auth constraint I had flagged as an obstacle: **the token lives in `localStorage`, with `../Conflab/assets/js/hooks/daemon_bridge.js` named as the precedent to copy.** Line 417 ties it to `AC-08.9` and D56 by name.

**SO D56 FORBIDS SERVER-RENDERED HTML AND EXPLICITLY DESIGNS A CLIENT-RENDERED BROWSER FACE.** The browser renderer is DESIGNED AND UNBUILT, not forbidden -- ic measured that intentd embeds exactly two assets and no `.js` exists. **My _declared non-goal_ framing is struck.** vc reached the same conclusion independently from AC-17.6's text (no clause about a served page; a client on `/op` satisfies it), so nothing was spent on the wrong answer -- **but vc and I both reasoned from the criterion and the shell page, and ic went and read the design document neither of us opened.**

**THE REMAINING QUESTION IS SEQUENCING, NOT ARCHITECTURE: whose WP builds the ES renderer.** ic's framing, adopted. Cheaper than what I sent up, and it is vc's to route rather than hv's.

**`WP-08` IS CLOSED, AND THE GAP IT WAS PROTECTING IS FILED AS `0232` -- IN THAT ORDER, DELIBERATELY.** vc ruled the close on 12/12; **the evidence is ic's**, who measured it while working WP-17 and handed it over rather than resolving it. **`0232` was filed and verified BEFORE the close, because closing first and filing second makes the protection depend on the next step being taken.** Both writes verified past the ingest -- `0232` present in store and on disk at 3501 bytes, `wp show ST0056/08` reads `status: Done`, and the transition text (`ok: ST0056/08 done`, not `already done`) is the oracle that says a write happened.

**MY OWN HOLD IS DISCHARGED BY ITS OWN TERMS RATHER THAN OVERRIDDEN.** It read _released when conformance coverage exists for the daemon, OR the gap is explicitly accepted on the record_. `0232` is that second arm. vc adopted this framing: **a hold that erodes under pressure teaches every future hold to be softer; one discharged by the condition it named teaches the opposite.**

**STOPPED BEFORE WRITING `learn`: hv RULED ON 2026-08-31 THAT `config` AND `ext` SHIP DECLARED-AND-UNBUILT IN 3.0.1.** Found by accident -- I searched `MODULES.md` before creating a module, per Highlander, and the ruling was inside a registry cell about a test file.

**VERIFIED IN TWO CODE SITES, NOT ONE.** `render.rs:1202` (_hv ruled on 2026-08-31 that both SHIP declared-and-unbuilt in 3.0.1, so this is the shape that stays_) and `remedies_are_reachable.rs:116` and `:534`. **It is NOT on hv's board** -- grepped, no entry. **So a live hv ruling governing three WP-06 rows exists only in two Rust doc comments and one registry cell.** That is vc's own _one home and no copy_ defect from this morning, except worse: this one lives somewhere a node CAN reach and would never look.

**IT CONFLICTS WITH TWO OF THE THREE THINGS vc ORDERED.** `ext` was sized M and called real work -- building it reverses the ruling. `config` is being sent to hv as a decision hv **already made**, which is the repeated-question defect firing again. **`learn`'s membership is UNVERIFIED and is what I need**: both code sites name only `config` and `ext`, and the only thing putting `learn` in the ruling is `MODULES.md:321`, **a cell that also names `fileindex`, which hv separately RETIRED on 2026-08-26** -- so it is demonstrably stale on one member and cannot be trusted on another.

**AND THE RULING-ORDER QUESTION UNDER IT IS NOT MINE.** hv's 2026-09-01 standing ruling is _everything outstanding is going into 3.0.1, feature complete_. Does the general vacate the specific? **hv's OWN recorded rule says no** -- _a general policy stated after a specific ruling does not silently vacate it_ -- recorded on hv's board as the ruling-order gap committed by its own author within the hour. **The honest reading is that `config` and `ext` are still declared-and-unbuilt; I am not confident enough to act on it, which is why it went to vc rather than into my diff.**

**AND IT RE-READS MY OWN SIZING.** I sized `ext` M off 820 v2 lines and 41 burning tests. **That was a size for a BUILD nobody had established was wanted** -- the same shape as the `browse` XS I sized off `--help`. The number was fine; the question was wrong.

**ST0065 OPTION 2 IS BUILT, GREEN, AND MUTATION-TESTED.** The four-rule index now lives in `_CLAUDE.md` as well as `_AGENTS.md`, with a new arm in `agents_sync_parity.rs` holding them byte-identical. **Index lifted from `_AGENTS.md` in code rather than retyped**, so the copy was made from the source.

**I MUTATION-TESTED IT RATHER THAN SHIPPING A GREEN I HAD NEVER SEEN FAIL** -- dc's two-sufficient-guards finding this morning is why. Planted a change in one template, watched it go red naming both files and the drift, restored byte-identical, watched it go green. **The arm also asserts its POPULATION before its property** (two carriers minimum, and `_AGENTS.md`/`_CLAUDE.md` by name), because a scan matching nothing passes for free.

**ic's `expand_tokens` MEASUREMENT IS CONFIRMED AT SOURCE, BOTH HALVES, AND vc's RULING STANDS UNCHANGED.** `value()` matches exactly `PROJECT_NAME`, `AUTHOR`, `INTENT_VERSION` and everything else reaches `Fault::UnknownToken`; `Block` is a two-variant enum (`Lang`/`NoLang`) whose `keeps()` retains INLINE content. **There is no file-inclusion form**, so single-sourcing needs a new token type. I said I would drive it rather than inherit it and would say so if ic was right -- ic is right.

**AND IT CAUGHT A FALSE CLAIM ON ITS WAY INTO EVERY PROJECT'S `CLAUDE.md`.** I was told `usage-rules.md` carries a third copy of the index and I wrote that into the template. **Driven: it does not.** It names the four principles in passing -- a skill description, a directory pointer, a rule-id format example -- and carries no index; the `_usage-rules.md` template carries nothing. **The real third home is `in-standards/SKILL.md`, as a TABLE with all four ids**, which cannot join a byte test because it is a different rendering by design and reaches projects through `claude skills sync` rather than `claude upgrade --apply`. Corrected before it landed.

**WP-06 IS STOOD DOWN ENTIRELY.** vc confirmed `learn` IS in the ruling, from dc's own 2026-08-31 08:30Z enumeration in `.history/`. **And my staleness inference was WRONG in an instructive way**: `fileindex` retired at `c6515ad6` THREE HOURS AFTER that enumeration, so the record was accurate as written and one member departed correctly later. **I read a member leaving legitimately as evidence the record was broken -- that inference would retire a lot of true records.** The binary discriminates, driven with a control: `config`/`ext`/`learn` all answer _known command that is not implemented yet_; `fileindex` answers _was retired in Intent v3_; `st` answers _requires a subcommand_. Three populations, and `learn` sits with the first.

**THE CANON EXTRACT CANNOT BE SPLIT AND dc ASKED ME TO CARRY IT.** `intent/.canon/st/ST0056.json` holds my WP-08 transition AND dc's two parity tools, **because the extract is regenerated WHOLE from the store and both changes are already in the store** -- no hunk answer, no ordering answer. `canon_commit_check.sh` refuses an attachment change whose canon does not land in the same commit, which is correct and is what surfaced it. **Agreed to take it, on dc's reasoning: a transition on the longest pole reading as a side effect of a de-canon commit is unreadable in six weeks.** Blocked on matts like everything else.

**THE TREE IS FROZEN, I MEASURED IT MYSELF, AND THE MEASUREMENT REFUTES THE PLAN IN FRONT OF hv.** dc reported `rc=1` and gave me the instrument; I ran it rather than taking the report, which is the day's lesson applied to a peer who had been right all afternoon.

**METHOD (dc's, adopted):** a synthesised index -- `GIT_INDEX_FILE` in scratch, `read-tree HEAD`, `add` of exactly my two files -- then `bash .githooks/pre-commit` against it. **Ambient index confirmed untouched afterwards.** Building the tree a `commit --only` would actually construct is a far better instrument than reading `git status` and reasoning about it.

**RESULT: `rc=1`, AND THE GUARD IS THE ROSTER GUARD** -- _78 parity file(s) in this commit; 18 gated + 40 manual + 20 not-an-instrument = 78; the roster and the runner DISAGREE_, summarised as `runner_roster_check.sh`. **THE DECISIVE DETAIL: my synthesised commit contained NO parity files at all** -- two Rust files, nothing under `parity/`. Refused anyway. **So the guard reads the WORKING TREE, not the commit, and its own message says _in this commit_** -- a wording defect worth filing on its own.

**SO APPROVING MY `browsed()` DIFF UNFREEZES NOTHING.** dc recommended it to hv as the option that costs nothing; **I would take the approval and hit `rc=1`.** The head of the chain is dc's in-flight parity pair, and the only thing that actually unfreezes the tree is that pair reaching internal agreement -- finished or parked. **Told dc rather than going around them to hv: they put the options up and it is theirs to correct.**

**AND ONE DIAGNOSTIC FROM THE SAME RUN, REPORTED SO NOBODY READS IT LATER AS A SECOND PROBLEM.** `self-provenance: currency REFUSING -- 1 non-test file(s) under native/rust changed since c5db8b8ac2ba`. That file is my `render.rs`. It never fails the gate and it clears when the commit lands.

**`cargo fmt --check` IS CLEAN WORKSPACE-WIDE.** ic found a rustfmt diff at `agents_sync_parity.rs:420` and flagged rather than fixed it; dc did the same. **It was mine -- my own let-else in the ST0065 arm.** Fixed BY HAND, one hunk: running `cargo fmt` would have reformatted every dirty file in the shared tree, which is the collision both of them declined to cause, with a bigger blast radius.

**OPTION B VERIFIED ON MY OWN RUN AND I DO NOT OBJECT.** dc measured the option set down to one; I rebuilt B myself and got `rc=0`, all guards passed, ambient index untouched. **I was not going to green-light the fix to an unverified premise using the move that created it.** Confirmed the transition really is in the extract: the `wps` list is POSITIONAL with no id field, so a grep for the WP name finds nothing -- the 8th entry, `intentd daemon`, reads `done`. **My own first query returned zero and the zero was about my query.**

**MY OBJECTION IS SATISFIED, NOT OVERRULED.** I objected to the transition being UNFINDABLE in six weeks; dc puts it in the subject line. **An argument that was right against a better alternative does not survive the alternative not existing** -- C and D were dc trying to land without touching my work, and the canon guard refuses both.

**WHAT I WILL NOT TELL hv: that B is fine because dc and I agree.** Two nodes agreeing is not corroboration. What makes B trustworthy is that we each built the index and ran the gate on DIFFERENT paths and got the same rc from the same named guard.

**ic WANTS A HUNK IN `explore()` AND I DECLINED TO TAKE IT FOR THEM**, though I am in the file and it would be easy -- that is the trap: it would put ic's work inside my commit under my name. **dc declined that same favour on my fmt diff, and ic declined it before that.**

**FLAGGED TO ic, NOT AN OBJECTION:** their store-growth fix takes the pathological project to 54 ms against a 150 ms threshold, so **the indicator now shows nothing on any project anyone can measure** -- a safety net that cannot be observed firing. Their pure function is unit- and mutation-tested so the LOGIC cannot rot; **the WIRING can, and nothing goes red if a refactor drops the call.** This crate already has the pattern: `the_renderer_calls_the_edit_door_exactly_once` COUNTS CALL SITES.

**BOTH COMMITS LANDED, ON matts' EXPLICIT APPROVAL.** `eaef2a04f` -- `browsed()` honest refusal plus the `browse` roster note my own change falsified. `0b5d46c96` -- ST0065 Option 2, the four-rule index in both root templates with a mutation-tested drift arm.

**SPLIT INTO TWO, AND EACH PATH SET RE-VERIFIED ALONE BEFORE COMMITTING.** I already had a combined `rc=0` across all four files and did not lean on it: **a set passing together is not evidence about its subsets**, and splitting after a combined verification is exactly where that assumption bites. Both subsets `rc=0` independently. Post-verified: each commit carries exactly its two files, and ic's board was committed by ic (`e2ab827e3`), not swept by me.

**THE FREEZE WAS NEVER THE ROSTER GUARD AND THE CORRECTION IS MINE TO CARRY.** It was dc's `MM` index/worktree divergence: the index held the old blob, the worktree the new arm, and the canon extract recorded the WORKTREE hash, so canon named bytes the index did not carry. **I took one live roster reading and then quoted it for hours -- to dc, to matts, on this board -- while the tree moved underneath.** That is D2 from this morning firing on a guard verdict instead of a daemon pid: **a fresh measurement feels like it licenses the claim indefinitely, and it does not.**

**AND ic FOUND THE PART THAT MAKES THE INSTRUMENT ITSELF SUSPECT.** `git update-index --add` reads the WORKTREE, **so every synthesised index all three of us built contained the fresh bytes by construction** -- the instrument could not represent the defect it was being used to diagnose, and returned a clean `rc=0` that looked like an answer. **KEEPING THE METHOD WITH THE LIMIT WRITTEN DOWN: it answers _would this path set pass_ and CANNOT diagnose any defect whose subject is the difference between index and worktree.**

**STILL UNCOMMITTED, FLAGGED TO matts RATHER THAN ABSORBED: `0232` IS UNTRACKED.** My daemon-conformance gap issue exists in the store and on disk but **not in git**, so the thing `WP-08`'s hold was protecting is one `rm` from gone. The canon files are entangled across several nodes' writes, so this is not mine to sweep up unilaterally.

## TODO -- startable, mine, smallest first

- **XS** `0095`/`0096` -- CLOSE as never-specified, reason on the record. Driven: empty in title AND body. **They are `0223` debris, not rows anybody failed to specify.**
- **XS** File the `implemented_check.sh` false-positive class.
- **XS** `browsed()` HONEST REFUSAL -- **BUILT, DRIVEN, GREEN, UNCOMMITTED.** In `render.rs` + the `browse` roster note. Awaiting matts on the commit and sequencing with dc, who is about to write the same file.
- **S** `0063` -- FIELD MIGRATION: the title's 187 chars into the body, short title left. vc's ruling; no knowledge recovered, nothing invented. **NOT a close.**
- **S** `0205` -- vendored fourth block ACCEPTED with its reason at `bin/.devbin/lib/builtins:66`.
- **S** Migrator-commit -- `migration.md` Phase B step 7 and `AC-00.8` stop claiming _one commit_. vc ruled: correct doc and row, do NOT build the commit.
- **S** `implemented_check.sh` fix -- classify on marker AND rc=2.
- **M** `0192` RULED IN -- refusal in `info_read_back`, placement already decided.
- **S-M** `browse` daemon half -- an entity page and an open path, so `AC-17.6`'s one-model-one-service holds. **This is the item I sized XS off `--help` and got wrong; this size is off the code and is still a judgement.**
- **S-M** `SERVED_BY_DAEMON` is ONE entry (`render.rs:235`). **RECOVERED FROM vc's INBOX DURING THIS FOLD -- it was not in the report I sent hv.** Not a hazard: exclusions refuse loudly, and the discharge condition is already in the code -- it becomes a projection of the dispatch table rather than a second home. Size JUDGED.
- **L, AND THE SIZE IS A GUESS** WP-06's 9 unmet CLI rows (`ext` 5, `config` 3, `learn` 1). An aggregate never individually sized since the audit. Treat as unsized.

## Holds -- mine, with the condition that releases each

- **M** `AC-06.1`'s coverage half -- RELEASED WHEN a burn TSV covering the estate exists AND `INTENT_BIN` resolves to one binary rather than three. `coverage_map.sh` refuses to publish and is RIGHT to.
- **L** `0216`/`0226` fix -- RELEASED WHEN a monotonic version the ingest does not own exists. The obvious fix collides with `written_at`, which the ingest rewrites wholesale.

## Decisions owed by hv -- question, options, recommendation

- **Should `at green` run the L3 arm?** (i) warn, do not refuse (ii) refuse (iii) leave. **REC (i)** -- refusing breaks the legitimate write-then-cite order, which is the order that produced this morning's outage.
- **`INTENT_BIN` flip and re-baseline -- which order?** (i) flip then re-baseline (ii) re-baseline then flip (iii) neither this cut. **REC (i)** -- the default is `bin/intent`, the v2 SHELL SCRIPT, and it is three binaries not two; the other order pays the wall time twice.
- **`burn.sh` re-run, or accept `AC-06.1`'s coverage half red?** (i) run (ii) accept red and say so on the row (iii) descope the half. **REC (i), and it is hv's because full-suite runs are.**
- **`config` bare resolves to `target: undefined` -- what should it do?** (i) print the resolved config (ii) print help (iii) refuse with a remedy. **REC (i), FLAGGED: this rests on my reading of the surface, not on a census of bare noun verbs.**
- **`agents` bare is `pending-hv`.** Same options. **REC: whatever (d) gets** -- two bare nouns answering differently is the defect, not either answer.
- **`WP-08`: endorse vc's hold or override?** **REC endorse.** A blocker that erodes because the blocked party built one piece of the thing is how conditions stop meaning anything.
- **Flip `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"` to a gate?** (i) clear the class then gate (ii) stay report-only (iii) gate now. **REC (i)** -- five of six targets I once called absent exist under another name. **PREMISE MOVED: the account lived in `0214`, which is now CLOSED.** Lint half is dc's.

## Open, no owner

- **Something WALKS the CLI surface.** Seven CLI-token-titled creations in `event_log`, two episodes eight days apart, machine-paced, `st` then `issues` both times. ic found no generator and named no suspect.
- **Is my unfiled daemon-lock race a duplicate of `0210`?** Adjacent ground, NOT compared.
- **Does one fix serve both `0216` and `0226`?** Same collision, opposite symptoms. Not driven.
- **Which symptom you get may depend where in the window you land** -- live tree showed the silent form, the harness at high contention the refusing form. Hypothesis.

## Watch-outs

**Folded into families. The rules are kept; the worked examples are in `.history/20260903/wip-prefold-1718Z.md` and are not repeated here.**

**A. THE INSTRUMENT ANSWERED A QUESTION ADJACENT TO THE ONE ASKED, AND ANSWERED IT CORRECTLY.** The dominant family and the one that keeps arriving while I am being careful. Instances: `--help` read as an arm; a RENDERED issue read as a field; a green gate read as behaviour; a name census promoted to a claim about mechanisms; `grep -i FAILED` matching `0 failed`; a gate verdict read from the working tree with nothing saying which tree; a display filter (`if d<1800`) standing as a claim about the population; a constructed variable quoted as measured, including by its author; a base rate wearing the safety question's clothes; a claim that sounds like physics exempted from measurement. **RULE: name the FIELD you read and the INSTRUMENT you read it with. `--help`, a clap subcommand and a declared row are statements about the PLAN; only an arm or a drive is evidence about the BUILD.**

**B. CONTROLS, OR THE READING IS NOT EVIDENCE.** A control that cannot distinguish _safe_ from _never tried_ is not a control, and it must vary the axis the check actually reads. **An instrument that is part of its own population is worse than a noisy one -- it is CONFIRMATORY** (`Op::Registry` is in `wire::UNCOUNTED` for exactly this reason; before reading a meter in a loop, establish that reading it is not an event the meter counts). **RULE: positive-control the instrument before its silence means anything.**

**C. ARITHMETIC AND SAMPLE SIZE.** **n=2 is not a result about a stochastic process** -- I fired a pre-committed disconfirming condition on two samples, into an artefact, with the variance already visible in a sweep I had printed. **A total you did not enumerate is not a total you may publish**, and adding to someone else's total requires reading what is in theirs. **RULE: state n and state the variance; below n=5 on a process you have watched vary, the honest sentence is _observed twice, not characterised_.**

**D. PREMISES.** **Drive a ruling's premise before building on it** -- of four build rulings in one day, THREE had false premises, and today `0086` (closed), `0214` (closed) and `0063` (a field migration, not a rewrite) all moved under rulings that named them. **Assert the premise your fix rests on as a test, in the direction that would embarrass you.**

**A2. THE CLOCK RULE IS A RULE ABOUT IDENTIFIERS, AND I FOUND THAT OUT BY BREAKING IT ON A SHA AT THIS BOOT.** `restart.md` generator 2 is _fabrication with the correct value present_ and it is written up entirely in terms of TIMESTAMPS. **It is not a fact about clocks. It is a fact about any opaque token a reader cannot check by looking at it** -- I typed HEAD as `f1ff2f81` into a message to vc with the real `f1ff2f824` four lines up in my own tool output, and it was caught only because I re-ran `git rev-parse` for an unrelated reason. **A wrong sha is worse than a wrong stamp: a stamp lands in a range a reader can smell, and a sha resolves or it does not, so a plausible one sends the reader to `git cat-file` and not to me.** RULE: **substitute the command, never the value** -- the same remedy the clock rule already gives -- and that covers shas, pids, issue numbers and line numbers, none of which the written rule names.

**D2. A MEASUREMENT IS RE-TAKEN AT THE MOMENT IT IS QUOTED, NOT AT THE MOMENT IT IS TAKEN -- AND THE RELAY IS WHERE IT ROTS.** I measured `intentd` at 07:02Z and quoted it to vc at 09:59Z. **The process had died sixty seconds earlier.** Family D is about a ruling's premise; this is the same defect one step downstream, where the premise is my OWN earlier reading and the act that breaks it is a peer rebuilding. **The tell is a possessive tense: _the daemon IS stale_ is a claim about now, sourced from a reading about then, and nothing in the sentence marks the gap.** RULE: **a figure crossing into a message gets re-driven in the turn that sends it**, exactly as a clock stamp does -- and for the same reason, that the reader cannot tell a fresh read from a stale one by looking.

**E0. `cmd | head` REPORTED head's STATUS TWICE TODAY, AND THE SECOND TIME I HAD ALREADY CAUGHT THE FIRST.** `intent browse ST0001 2>&1 | head -5` printed `rc=0` for an rc=2 refusal; I caught it, re-drove without the pipe, and then did the identical thing to `intent claude subagents list` forty minutes later -- reading `rc=0` on what is really `rc=2`. **Both times the WRONG number was the reassuring one**, which is why neither felt like an error at the time. The board has carried this rule for days and carrying it is not the same as holding it. **RULE: a probe whose EXIT CODE is the finding never goes through a pipe -- redirect to a file and read the file.** zsh has `pipestatus`, not `PIPESTATUS`, and reaching for either is already the wrong shape here.

**E1. AN UNQUOTED HEREDOC RAN MY OWN BOARD PROSE AS COMMANDS, AND `intentd`'s GUARD IS WHAT SAVED ME.** I wrote `<< PYEOF` instead of `<< 'PYEOF'` to get a date into a python script. **Board prose is dense with backticks, and an unquoted heredoc does command substitution on every one of them** -- so zsh executed `usage-rules.md`, `wps`, `WP-08` and, worst, **`intentd daemon`**, against this machine's real HOME. **Nothing bad happened for exactly one reason: `intentd` REFUSES an unrecognised argument rather than falling through to serving, and says in its refusal that the fall-through is withheld deliberately because starting a daemon on the real HOME makes every session's store verbs refuse.** A design decision somebody made on purpose absorbed a mistake I made by accident.

**THE PYTHON NEVER RAN, SO THE BOARD WAS UNTOUCHED RATHER THAN CORRUPTED** -- the failure was loud and total instead of partial, which is the lucky direction. **And I checked the daemon pid rather than assuming**: it had changed, and it would have been easy to write that up as my doing. It started 4h30m before the slip. **The near-miss was not the daemon; it was nearly filing a coincidence as a consequence.**

RULE: **the heredoc that carries prose is ALWAYS `<< 'EOF'`, and a clock value goes in through a `__NOW__` placeholder plus `sed`** -- the pattern that worked six times today, abandoned once for brevity.

**E. THIS BOX AND THIS SHELL.** `cmd | head` reports head's status -- **done again today.** The Bash tool's shell is zsh: unquoted `$var` does not word-split and an unmatched glob (`--include=*.md`) aborts the whole command -- **hit again today.** A stale binary cannot answer a question about HEAD. `cargo check --workspace --all-targets` -- the flag is the half memory drops.

**F. THE SHARED CHECKOUT.** Canon cannot be split, so every canon commit is silently multi-node. **`add` + `commit --only` is the only safe write; a live `index.lock` is a WAIT, never a removal, and the retry is the SAME command re-issued, never a recomposed one** -- both exercised today. Two correct rules can take the machine down: a release build DELETES the shared pair before building (`0196`) while the dirty-tree guard REDIRECTS. **Currency is a PROPERTY, never a value; a pin trailing HEAD is the correct steady state.**

**G. `0216` AND WRITING CANON.** **The read-verify-retry loop is a REQUIREMENT, not good practice** -- it replaces _one verb at a time_, because **the debouncer sees WRITES, NOT AUTHORS, so spacing protects only against your own burst.** Verify on an OBSERVABLE, never on a duration, and never on the tool's `ok`. **After a revert the DISK WINS, so store and disk agreeing AT THE OLD VALUE is the signature, not the exclusion** (vc). `intent st attach` writes store AND canon and NEVER the disk file (`0082`).

## Decisions

- (2026-09-03, ic->cc) **THE CLASS IS THE UNDRIVEN NUMBER, NOT THE WRONG ARTEFACT.** ic's framing supersedes mine and carries their name.
- (2026-09-03, cc+vc) **CONTENTION IS `0216`'s VARIABLE -- not spacing, not corpus size.** And **refusals (`0226`) and silent losses (`0216`) TRADE OFF**, so a single counter prints _fewer losses under load_ and reads as the defect improving.
- (2026-09-02, cc+vc) **`0216`'s FIX IS DAEMON-SIDE AND NOT USAGE DISCIPLINE.** A hazard reachable by an ordinary shell loop cannot be mitigated by how carefully nodes write.
- (2026-09-02, vc) **A RIDER CANNOT BE VIOLATED BY A CASE ITS OWN HAZARD CANNOT REACH.**
- (2026-09-02, vc) **`WP-14` AND ALL 12 OF ITS ACs WERE DESCOPED WHOLE TO ST0069** by hv on 2026-08-30. `AC-09.5`'s wip/boards half goes with it.
- (2026-09-02) **TWO MACHINE PROJECTIONS OF ONE VALUE MUST NOT DRIFT; A HUMAN RENDERING OF IT IS NOT A COPY AT ALL.**
- (2026-09-02) **CONTENT COMPARISON DOMINATES A VERSION COUNTER FOR A COMPARE-AND-SWAP.** A dominance argument ends a design debate that competing assertions cannot.
- (2026-09-02, vc) **AN EXCLUSION MUST BE VISIBLE WHEREVER THE BEHAVIOUR IS CLAIMED, NOT ONLY WHERE THE CHECK LIVES.**
- (2026-09-01, hv) **v3.0.1 IS FEATURE COMPLETE, THERE IS NO TAG WINDOW AND NO EXTERNAL CONSUMER, AND COST IS NOT A CONSTRAINT.** The scarcity register is retired as a class.
- (2026-09-01) **A REFUSAL THAT CANNOT SAY WHAT IT FOUND MAKES ITS OWN DEFECT UNDIAGNOSABLE.**
- (2026-09-01) **A REMEDY INHERITS ITS BRANCH'S ASYMMETRY.** Confirm-before-refuse is SAFE on the lock and WRONG on the probe.
- (2026-08-31, vc) **A CRITERION IS NOT REWORDED TO WHAT THE CURRENT STATE SATISFIES.** The ruled form must be HARDER.
- (2026-08-31, vc) **A CLASS CHANGE WITHOUT ITS REASON IS A DELETION WEARING A NEW LABEL.** A declared exclusion carrying its reason is the cure; a silent drop is the denominator attack.
- (2026-08-31, ic correcting me) **AN OWNERSHIP SPLIT IS A PURPOSE, NOT A BOUNDARY.** Two hands in one file IS `0206` in miniature.
- (2026-08-31, CORRECTED) **`close --note` STAYS RULED OUT ON A CONTINGENT FOOTING.** The keg has no `edit`; the ruling survives because v3.0.1 ships from the tree.
