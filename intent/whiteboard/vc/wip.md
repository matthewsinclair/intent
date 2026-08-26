---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 16:17Z
status: active
focus: "**BOSS-VC for the v3 fleet cutover under hv's pen and push grant (Intent + homebrew-intent). THE FLIP HAPPENED 14:00:46Z (v3.0.0 on the keg, sixteen projects at 3.0.0). hv's CALL: EVERYTHING INTO 3.0.1 TODAY, DONE PROPERLY. STOP-THE-LINE DEFECT: hop 2 silently dropped AT/AC rows whose status/satisfied value was followed by a period, a sentence or a parenthetical -- 57 items across arca_cli, Utilz, Laksa, Cdsync, plus Lamplight (reverted to v2, uncommitted) -- cc's parser fix is green, sha pending; ALL MIGRATION STOPPED until it lands. Then: name cc's + ic's shas to dc -> ONE guarded rebuild -> both stamps -> re-convert the four with reconvert.sh --commit, Lamplight with lamplight-run.sh, arca_config by devbin/vc -> accounting both arms on every one -> organize --default sweep (narrow, on hv's ruling) -> hv's go -> dc cuts 3.0.1 -> publish -> fleet re-stamp on the keg. Three rulings open with hv.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**Folded at 2026-08-26 16:17Z for compact, on hv's word; HOLD ON THE BOUNCE for hv's instructions.** Executable state: `cutover-runbook.md` (LEARNED section, appended in order, each lesson with its finder -- read its last twenty bullets first). Instruments: `verify-canonical.sh` (does NOT count AC/AT rows), devbin/vc's `at-accounting.sh` (Devbin repo, `intent/whiteboard/vc/`, two arms, self-test, exit contract: rc 3 error / rc 0 NOT MIGRATED / rc 0 VERDICT = the only pass / rc 1 SHORTFALL), `reconvert.sh` and `lamplight-run.sh` beside this board (both tested on throwaways; both commit only on rc 0 AND a `VERDICT:` line; logs under `$VC_SCRATCH` or `/tmp/vc-scratch`).

**STATE PER NODE AT THE FOLD:**

- **cc** -> batch complete (`e696de15`, `80cb8509`). The AT/AC row loss DIAGNOSED and FIXED, green in cc's target, sha pending the full gate: `legacy.rs:1531` `field_end` runs to end of line, so `status: green.`/`status: green (…)`/`satisfied: yes. …` values match nothing and the row is silently rejected (23 of 26 with no line); fix = the `split_citation`+`append_note` shape generalised to status/test/evidence/satisfied. Then, in order: hop 2 refuses on ANY declared-in vs stored-out id shortfall (loud named residue first, count second); duplicate id as a named residue (tenth class); descent scan as ONE function with the guard (`thread_dirs` allowlist vs `collect_legacy` descent -- keep descent behind `is_thread_id`; Lamplight's `_inbox`); TODO 7 one canon emitter (attachments with bytes+sha); ST0057/13 bucket ingest + prune ONLY if hv rules wide. Hop 2 findings also filed: two-writers-one-thread class; BrokenReference residues manufactured by dropped ACs (arca_cli only).
- **ic** -> `critic --languages` arity fixed `e2a01fd1`. Runner refuses on an EMPTY census (`total() == 0`, exit 2, loud UNENFORCED via the hook's `*)` arm; `armed == 0` was wrong -- swift/lua arm 0 legitimately) built and green in isolation, sha pending the shared target. Building ST0057/11 AS WRITTEN: `organize --default [--force]`, one function in `intentsvcs::intentfiles`, three callers (init, migration, upgrade), DECLARATION ONLY -- refuses to add removal without hv's word (right). `install.rs` NotFound remedy held for hv.
- **dc** -> Anvil `b63ba9c`; build entrances unified `7456a158` (one guarded release path; `cli`/`daemon` refuse release into the shared target; `int cli` freshest-wins); `SUPPORT_PATHS` + coverage + smoke `0112b8c1` (rules + skills into the keg; smoke red on the keg, green on a fixture keg). WAITS for me to name cc's and ic's shas, then ONE rebuild (`guarded_release_build`), then `int build release v3.0.1` -> prepare -> smoke -> publish on hv's go. Four release-verb defects + `verify_pair` mid-compile redding queued after.
- **devbin/vc** -> `at-accounting.sh` `7aa1f21`+ (fleet table: 57 items); arca_cli `33e3c2d` LOSSY, committed, UNPUSHED, kept as evidence pending hv's revert ruling; arca_config untouched at 2.18.0; holding for the go with both stamps; bucket per-file table at Devbin `242013d` (34 of 54 files carry content the store lacks; migrator's reach = info.md + acceptance.md only).
- **devbin/cc** -> `gate_critic` refuses on 0 armed `455d3f0` (663/663, unpushed under the grant); the ten devbin estates' `check critic` returns on re-vendor after 3.0.1.
- **lamplight/vc** -> Lamplight back on v2 at THEIR HEAD `2379b6ea7` + my two duplicate-id demotions (verified by them: correct member demoted; zero further duplicates). Landing the pre-commit markdown-leg batching (prettier + git add per file made a 2,947-file commit take >4 min); all four Lamplight nodes frozen for the re-run; start notice OWED this time; `_inbox` -> Triage is hv's ruling.
- **vc** -> this board; runbook current to `9810d372`; rule `IN-AG-RED-CONTROL-001` landed with its evidence (`0efd0676`, `543b64a8`); ST0057/11-13 written; ST0064 (intentd + menubar app, Geodica design attached); issue 0097; seven canon pointer repairs (Cdsync `41fd2fb`, Riffle `5982ff7`, Prolix `1298531`); memory updated (shared-index rule; zsh third form).

## CRITICAL PATH (the bounce checklist; each step names what it waits for)

1. **/in-session, pickup, read hv's answers** to the three rulings below; read cc's/ic's shas if they arrived (SendMessage log or their boards). Re-read the runbook's last twenty bullets.
2. **Name cc's parser sha + ic's runner sha to dc** -> dc rebuilds via `guarded_release_build` -> read BOTH stamps off `bin/intent3 --version` yourself (never carry a stamp forward).
3. **Re-convert the four**: `bash intent/whiteboard/vc/reconvert.sh ~/Devel/prj/Arca/arca_cli --commit` (hv's revert ruling first: the script restores from the migration commit's parent, so `33e3c2d` is superseded, not reverted), then Utilz, Laksa, Cdsync. Each prints the accounting gate; commit only on PASS. Laksa `author` must stay `matts` (the script re-applies the landing's fields). Then `at-accounting.sh --self-test` + each project: source == store == view, both arms, rc 0.
4. **Lamplight**: start notice to lamplight-vc FIRST; `bash intent/whiteboard/vc/lamplight-run.sh [--inbox-flat] --commit` (flag per hv's `_inbox` ruling); the commit runs their hooks -- batched, or expect minutes; guards line must read `4 ran`. The script sets `status: Triage` in the four v2 info.md files before hop 2 (hv's ruling); `legacy.rs:792` maps TBC to NotStarted and names Triage, so if hop 2 REFUSES the word, leave `Not Started`, migrate, then set Triage on the four via canon (`.canon/st/<ID>.json` status -> `sync --to-store` -> `--to-disk`).
5. **arca_config**: devbin/vc, plain recipe, on the go with both stamps.
6. **`organize --default` sweep** across every migrated project once ic's verb is in (narrow: declaration only, unless hv rules wide) -- on the dev pair, `--only` commits; the fleet's canon is then made by 3.0.1's code.
7. **hv's go -> dc cuts 3.0.1** (`int build release v3.0.1`, prepare with rules + skills, smoke rules-count > 0, publish) -> `brew upgrade` -> keg on PATH -> `intent upgrade` per project on the keg (re-stamp; writes `.intentfiles` if ic's upgrade path is in) -> post-flip verifier on all.
8. **After**: WP-13 prune (ONLY after the accounting is clean everywhere -- the buckets are its only source surface), ST0061 dehydrate + preconditions in the tool, hv's prune exam (`Intent/bin` minus devbin's needs; measured under the old step 8 in the runbook), Conflab, Devbin AC-10.5 (done by devbin/vc `428da1f`), the geodica menubar app when hv gets to intentd.

## HELD FOR hv (rulings asked, not yet answered)

- **`organize --default --force` in 3.0.1: NARROW** (declaration + realise, remove nothing -- ic building this) **or WIDE** (also dehydrate closed threads, ingest bucket files per-file-verified, prune buckets/issues dirs/.treeindex)? vc recommends narrow tonight, removal first thing after; wide-wrong deletes files on nineteen estates.
- **RULED (hv, 2026-08-26, via lamplight-vc): Lamplight `_inbox/` threads land as Triage** -- `lamplight-run.sh --inbox-flat` moves them flat and sets `status: Triage` before hop 2.
- **arca_cli `33e3c2d` (lossy) superseded by re-conversion** -- yes?
- Geodica menubar-app design: `ST0064/01` (Triage) for when hv gets to intentd. Conflab held (61 `status:` rows). Devbin AC-10.5 landed by devbin/vc. `install.rs` remedy item (ic) awaiting hv's confirmation.

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
- **A RUN THAT MEASURED NOTHING IS NOT A PASS; A PASS IS A `VERDICT:` LINE AT rc 0.** The instrument's own line goes into the commit message so a wrong calm has a reader.
