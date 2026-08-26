---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 17:04Z
status: active
focus: "**BOSS-VC for the v3 fleet cutover under hv's pen and push grant (Intent + homebrew-intent). THE FLIP HAPPENED 14:00:46Z. hv's CALL: EVERYTHING INTO 3.0.1 TODAY. hv RULED FIRST-HAND (ic's session): bare `--default` NEVER removes a file; `--default --force` removes files AFTER A CONFIRM = the tty read AC-11.2 ratifies; AC-11.6 gives the destructive arm its criterion (`e8ba126b`). The sweep therefore SPLITS: bare `--default` on every project is mine (declaration, nothing removed); `--default --force` per project is hv's keystrokes. Rebuild 1 (99673f91) carries cc's parser fix + ic's runner refusal + default declaration; arca_cli, Utilz, Cdsync re-converted clean on it; Laksa PARKED (2 named carried rows), Lamplight RESTORED to v2 (7 named carried rows; NO non-UTF-8 text -- my iconv instrument was the defect; 27 images pinned Unattached), arca_config HELD by devbin/vc. REBUILD 2 SPLITS: 2a = cc's parser increment (Half A self-check, opaque arm as built, token rule + field_end, covers-less rows) the moment it is committed and quiet.sh reads clean -> re-run the three; 2b = ic's WIDE verb + st dehydrate (+ decode-decides attachments with a 1 MiB cap if S) -> the sweep -> hv's go -> dc cuts 3.0.1 -> publish -> fleet re-stamp on the keg.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**Bounced at 2026-08-26 17:17Z (compact done on hv's word); CONTINUE from CRITICAL PATH step 4.** Executable state: `cutover-runbook.md` (LEARNED, appended in order -- read the last forty bullets). Instruments beside this board: `quiet.sh` (dirt + live cargo EXECUTABLES, `pgrep -x`; QUIET only when both empty -- the word to dc before any shared build), `reconvert.sh <dir> [--commit]` (landed project -> v2 source -> hops -> accounting gate; `VC_PRE_HOP2=<script>` for a content fix in the restored source, eg `laksa-pre.sh`), `lamplight-run.sh [--inbox-flat] [--commit]` (fresh; Triage via canon after hop 2). devbin/vc's `at-accounting.sh` (Devbin repo) is the fleet's instrument: pass = `VERDICT:` at rc 0, source == store == view both arms, self-test in the same run.

**STATE PER NODE ON THE BOUNCE:**

- **cc** -> parser fix IN `1583d1ad`; Half A (self-check, no persistence) on its last gate leg under build-lock contention with ic, sha pending. (5) opaque attachments are BUILT end to end but UNREACHABLE: `Project::classify` decides on a 3-entry extension allowlist (md/txt/sh) three steps before the decoder runs -- ships AS IS in 2a. RULED (vc, delegated): the replacement is decode-decides + a 1 MiB cap + refusal BY NAME (canon carries attachment bytes INLINE, so a cap is load-bearing until blobs have a home outside the extract -- 3.0.2, hv's word); in 2b only if cc sizes it S, else 3.0.2 with the allowlist's exclusions declared. (1) token rule + (b) covers id under the same sentence (a field's value is its leading well-formed token; lamplight-vc's `AC-00.3 clause 3 (...)` shape) + `field_end` for a `--` inside parens as a separate NAMED edit in the same commit; (2) covers-less AT rows arrive with `covers: []` + named residue. Then hold Intent commits until I lift.
- **ic** -> B4 was never greenfield: `st dehydrate` is declared and merely unwired (falls to `unwired()`, exit 2); ST0061 now carries its contract (`fecdd110`, gate BLOCKED 0/7). `ac gate ST0057` is 53/58 with only AC-11.1-11.5 open. hv's ruling closes ic's AC-11.4-vs-WIDE escalation: AC-11.4 stands; `--force` confirms on a tty, refuses without one, no flag; AC-11.6 is the criterion for the destructive arm. `install.rs` remedy still held for hv.
- **dc** -> holding for my QUIET; split confirmed (2a = HEAD at QUIET, by intent cc's increment; 2b = ic's verb; the sweep on 2b's pair); THREE reads lift a hold (intent `--version`, intentd marker, intentd `--version` rc 0), reported VERBATIM -- `dirty-<sha>` or `unknown` is the tell, never a confirmation; `source_commit.rs:190` is the one home for both envs.
- **devbin/vc** -> instrument `7aa1f21`/`293281f`/label fix; arca_config HELD UNCOMMITTED after hop 3 (ST0002 AT-05.3 comma/bold shape) -- on the go with 2a's stamps.
- **lamplight/vc** -> `d0f172876` landed as `AC-00.3 (clause 3)`, NOT the one-token swap I approved -- simulated against `covers()` first, the swap would have manufactured a BrokenReference at exit 0; CO-COVERS 0; AT rows lacking covers 6 -> 5 (all deliberate). Tree on v2 at their HEAD `3f2059f85` + my two demotions uncommitted. `_inbox/` now holds only `.DS_Store` -- FIND the four Triage threads before the run (see step 5).
- **vc** -> `e8ba126b` (AC-11.6 + hv's ruling on hv's board with provenance). Re-conversions on `99673f91`: arca_cli `0bde056`+`2c95d25`, Utilz `753a7c6`+`2955894`, Cdsync `3fbb8d0`; Laksa PARKED at `531de9e`. Corrected today: my "23 xlsx/pdf" Lamplight claim (zero exist; 27 png/gif) and my non-UTF-8-markdown claim (instrument defect).

## CRITICAL PATH (the bounce checklist; each step names what it waits for)

1-3. **DONE / PARTLY DONE.** Flip 14:00:46Z; rebuild 1 `99673f91`; arca_cli, Utilz, Cdsync re-converted and committed; Laksa, Lamplight, arca_config wait on 2a. 4. **Rebuild 2a trigger**: cc's Half A + (1)+(b)+`field_end` + (2) COMMITTED with shas and function names. Then `bash intent/whiteboard/vc/quiet.sh` -> QUIET -> tell dc with the shas tested by their DEFECTS -> HOLD Intent commits until dc's THREE reads agree -> lift. 5. **Re-run on 2a's pair**: `VC_PRE_HOP2=intent/whiteboard/vc/laksa-pre.sh bash intent/whiteboard/vc/reconvert.sh ~/Devel/prj/Laksa --commit`; Lamplight: locate the four Triage threads FIRST (`_inbox` is empty at their HEAD), start notice to lamplight-vc, then `bash intent/whiteboard/vc/lamplight-run.sh [--inbox-flat] --commit` (Triage via canon after hop 2; if refused, they migrate as Not Started and it is recorded); arca_config: devbin/vc on the go with the stamps. Each commits only on `VERDICT:` at rc 0; Lamplight's guards line present with ran+skipped == 4. 6. **Rebuild 2b** on ic's verb (+ the attachment classifier if S) -> **the sweep, SPLIT**: bare `intent organize --default` on every migrated project (tool-driven, declaration only, nothing removed, `--only` commits; hv's Devbin canary predicate first: `--default` changes exactly one path), then `--default --force` per project is hv's own keystrokes at a tty, whenever hv chooses. 7. **hv's go -> dc cuts 3.0.1** (`int build release v3.0.1`, prepare with rules + skills, smoke rules-count > 0, publish) -> `brew upgrade` -> `intent upgrade` per project on the keg (re-stamp; writes `.intentfiles` if absent) -> post-flip verifier. 8. **After**: ST0057/13 residue prune (accounting first), 3.0.2 items (blob home + cap for attachments, opaque rows + Unknown status, quote-awareness, `unclaimed_digest` relative paths, `build all` guards' staging-dir build, `int cli` freshness), hv's prune exam (`Intent/bin` minus devbin's needs; measured in the runbook), Conflab, Geodica menubar app (ST0064/01) when hv gets to intentd.

## HELD FOR hv

- **Delegated rulings hv can overrule**: shortfall gate (ii); the rebuild-2 parser scope and its split into 2a/2b; the accounting as the outer gate at the layer that commits; decode-decides attachments with a 1 MiB cap and refusal by name (2b if S, else 3.0.2).
- **hv's to decide**: WHEN to fire `--default --force` per project (the destructive half of the sweep is hv's keystrokes under hv's own ruling; the alternative hv did not ask for is a `--yes` flag a tool session could pass on an explicit per-run go); where attachment blobs live outside the canon extract (3.0.2).
- **Open**: the `install.rs` NotFound remedy (ic); Lamplight's Triage if the four threads cannot be set via canon tonight (they migrate as Not Started, recorded); Conflab; `ST0064/01` Geodica design when hv gets to intentd.

## Watch-outs

**These are vc's OWN -- durable cautions, standing, not archived.**

1. **AN INSTRUMENT'S OUTPUT READ AS THE SUBJECT'S ANSWER.** Today in my hands: a display alias (`{evidence: (.evidence // .note)}`) read back as the schema; a process grep matching dc's WATCH LOOP and reported as hv's release for four minutes; a catch-all in `legacy.rs` inverting eight hv sign-offs at exit 0 (ic). **A classifier whose DEFAULT BUCKET absorbs the unrecognised case cannot report that it failed.**
2. **MECHANISM BEATS A NOTE.** zsh does not word-split (four times, mine); BSD awk has no `\s`; `${PIPESTATUS[0]}` is empty; an rc read through `tail` is the pipe's. **Every rc to a file; every loop over a newline list in a bash FILE; every chain gated on the script's rc** -- my Prolix collapse printed `stop=1` and I committed.
3. **A PARITY SUITE PROVES TWO BINARIES AGREE; IT CANNOT PROVE EITHER IS USEFUL TO ITS CALLER** (dc, `0085`).
4. **A CLOSED LIST IS SAFE WHEN IT DECLARES WHY THE THINGS NOT IN IT ARE NOT IN IT** (dc). **AN ABSENT FIELD MUST BE REFUSED, NEVER RENDERED** (`0086`); its sibling: a known token with an EMPTY value substitutes silently where an unknown token refuses loudly (`rootfiles.rs:335`, devbin-vc).
5. **A BIDIRECTIONAL CLAIM IMPLEMENTED IN ONE DIRECTION IS GREEN FOREVER ON THE SIDE IT DOES NOT WALK** (dc).
6. **A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** A pair sha issued in advance is valid only until the next refusal; the body carries the pair that PERFORMED the migration, read at the time. **A number that travels through a message has a transcription hop nothing checks** (a digest wrong at character 17; a scope difference read as a mismatch). Full-length, by command, against the file.
7. **SHARED CHECKOUT: `--only` SEPARATES FILES, NOT AUTHORS; two reads in one command or they are two facts** (dc). A dirty `native/rust` REDIRECTS a build to a private target the wrapper never reads.

8. **AGREEMENT IS NOT CORROBORATION WHEN EVERY INSTRUMENT ASKS THE SAME QUESTION** (ic). **A change observed is not a landing. Only the flip earns "done"** (devbin-cc, hv). **A loop over a collection the defect EMPTIES cannot witness the defect** (cc) -- assert the length first.
9. **CANON IS THE SSOT FOR ROWS, NOT FOR PROSE.** Delete only a line-subset; otherwise keep. **The store's refusal is the oracle; nobody edits a ratified contract off a grep's prediction.** And the tail after a verdict token is OPAQUE -- read the verdict from it, never decompose and reassemble it (cc's withdrawn `status:` fix rewrote authors' bytes with no red anywhere).
10. **EVERY TIMESTAMP IS READ IN THE SAME COMMAND THAT WRITES IT. A NUMBER IN THE TERMINAL IS NOT A CLOCK** (cc). A fold parked outside the repo carries `2026-08-26 13:47Z` and stamps at apply.
11. **A PERMISSION BOUNDARY IS PER SESSION AND IS NOT ROUTED AROUND BY ANYONE, INCLUDING THE DIRECTOR.** Three refusals today were right: mine on taking dc's `--force`, mine on killing dc's process, mine on editing a gate's test without hv's word. **A relayed STOP is honoured on the relay; a relayed approval is not an approval; a lift carries hv's words verbatim; a peer's push authority is not mine to hand on.**
12. **A SUITE READING A LIVE ESTATE MEASURES A MOVING TARGET** (ic); only the failure TEXT tells a peer's mid-write from a regression. And the corpus cannot exercise the half the real data never reaches -- synthetic refuse cases, or the green proves nothing (cc, 1910 of 1910).

13. **AN INSTRUMENT'S DEFECTS FAIL TOWARD CALM** (devbin/vc, four in one day, zero false alarms): a run that measured nothing must print a verdict that is NOT a pass; a pass is a `VERDICT:` line at rc 0 and nothing else; a self-test proves each arm in both directions. And mine: an anchored edit (`^7\. `) hit a numbered watch-out instead of the step I meant -- verify the last step's effect (a grep count, a sha) after every multi-step call, because a zsh parse error or a wrong anchor runs NOTHING and prints little.

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **A CONTROL THAT MISSES ITS SUBJECT IS WORSE THAN NONE, BECAUSE IT WOULD BE BELIEVED.**
- **A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED** (hv). Files committed with a v2 binary on PATH is not a migration.
- **STATE THE INVARIANT, NOT THE MECHANISM.** **A CONVERGER IS NOT THE CURE FOR A SECOND HOME; ONE HOME IS.** **RECORD THE MENU, NOT ONLY THE SELECTION.**
- **A LATER FIRST-HAND RULING FROM THE SAME PRINCIPAL ON THE SAME SUBJECT SUPERSEDES. hv outranks the pen; the pen sequences.** hv's own rulings go on hv's board with their menus; rulings under delegation go to hv's inbox.
- **THE PEN MOVED THE AUTHORITY, NOT THE AIM -- AND NOW THE PUSH, SCOPED TO TWO REPOS.** The confirm gate stays human (hv fires the cut); `publish` is the one verb with its refusals; the flip is on my word; I tell hv before anyone tags if a gate does not close.
- **LEAD WITH THE DECISION THAT MAKES THE ENDGAME REACHABLE.** I put hv's own publish as item 2 of 6 in an inbox hv was not reading; hv stopped the fleet on an empty tap and was right.
- **A SWEEP TAKES ITS POPULATION FROM THE ARTEFACT'S OWN DECLARATION, NEVER BY TRAVERSAL; A SUBSET MEASURED IS NOT THE POPULATION; A DIRECTORY SUMMED IS NOT A THREAD.**
- **A SAMPLE THAT CANNOT EXHIBIT THE DEFECT IS NOT A WORST CASE. A SECOND OPERATOR ON A SECOND PROJECT IS A CONTROL.**
- **A CONVERGER AT THE WRONG FIXED POINT IS BLESSED BY THE IDEMPOTENCE CONTROL.** Only a COUNT sees a doubled block; only "is it THIS block?" sees a substituted one.
- **AN ARM THAT DEMANDS WHAT THE TOOL CANNOT PRODUCE IS A SPEC DEFECT; AN ARM THAT ACCEPTS WHAT THE TARGET STATE FORBIDS IS A FALSE GREEN.** Presence is not provenance; provenance (the footer) is not content (an empty H1).
- **`core.hooksPath` MAKES `.git/hooks/` INERT**; resolve the way git does. **A GUARD OVER ADDED LINES CANNOT TELL A RE-RENDER FROM A CLAIM** (dc): every first-render of a generated view trips it exactly once.
- **HOP 2 IS THE ORACLE.** It refuses atomically and names the pair. **A NARROWED HALT IS A MEASURED ONE.**
- **RULE ON THE MEASURED SIZE.** Three rulings today were made on a size cc gave me and re-made cheaply when cc measured before building; the one that was a schema migration did not ship on release day.
- **SOURCE COMMITS COST AN APPLE ROUND TRIP; TEST COMMITS DO NOT STALE THE PAIR.** Batch source after the fleet; one final re-cut. The migration-correctness fixes and the version bump were the exceptions.
- **READING THE WRITE-UP OF A CLASS IS NOT PROTECTION FROM IT.** The remedy is never care.
- **RULED UNDER THE PEN (vc, 2026-08-26 16:3xZ, delegated, `authority: vc`): hop 2's shortfall gate is (ii)** -- the migrator self-check (declared_in - stored_out == rejection findings recorded, else ERROR and refuse) blocks always, closed threads included; an ATTRIBUTABLE bad row on a CLOSED thread keeps the ratified carry, printed by id and reason, never as a bare count; the fleet's outer gate is the accounting (source == store == view, both arms, rc 0) before any re-conversion commits. Not (i): reversing the ratified residue/carried split on release night puts the cost on the archive and invites editing ratified files to pass a gate. hv can overrule.
- **A RUN THAT MEASURED NOTHING IS NOT A PASS; A PASS IS A `VERDICT:` LINE AT rc 0.** The instrument's own line goes into the commit message so a wrong calm has a reader.
