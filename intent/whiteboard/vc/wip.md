---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 14:15Z
status: active
focus: "**BOSS-VC for the v3 fleet cutover under hv's pen and push grant (Intent + homebrew-intent). THE FLIP HAPPENED 2026-08-26 14:00:46Z: v3.0.0 -> 80d8b2ca tagged, published (three notarised assets; formula revision 1 after the 644 defect), brew link intent rc=0, sixteen projects at 3.0.0 with bare intent st list rc=0 in every one, ST0057 gate PASS 53/53. NOW the batch window: cc's empty-subject rows + AT-citation parenthetical; ic's install.rs evidence line + the critic --languages arity defect (its only consumer is devbin's check:118, which fails closed; the canon hook dispatches the positional and was never affected -- my fail-open claim corrected)
; converge_gitignore anchors; the template sentence -> dc rebuilds ONCE -> arca_cli reset + full sequence, arca_config, Lamplight, Anvil, seven committed file pointers via canon -> 3.0.1 re-cut on hv's go -> hv's prune exam. HELD for hv: Conflab (61 status rows, schema field, done with time), Devbin AC-10.5, arca_notionex shell, five shell back-fills.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**THE FLIP HAPPENED (2026-08-26 14:00:46Z). What remains: the batch, one rebuild, the stragglers, 3.0.1, hv's prune exam.** Executable state: `intent/whiteboard/vc/cutover-runbook.md` (LEARNED section appended in order, each lesson with its finder). Instruments: `verify-canonical.sh` (`0245d7d1`; 14-arm self-test; `INTENT_FLIPPED=1` turns the `st list` INFO line into a pass/fail arm), `exposure.sh` (positive-controlled), `collapse-buckets.sh` (HALTED at its head -- it deleted preambles canon never carried).

**STATE PER NODE (post-flip, read at 2026-08-26 14:15Z):**

- **cc** -> fix 1 (empty-subject AT rows: `legacy.rs` + `views.rs` `(no reference)` renderer + `legacy_at_without_a_subject.rs` + 2 real rows in `parity/fixtures/rows.txt`) and fix 2 (AT citation parser swallows a trailing `(N tests)` into `file`; option 1, declared-exception rows) -- fix 1 IN `e696de15` (five files `--only`; crate suite 906 passed; the corpus is a registered attachment so canon was synced in the same commit). Fix 2 PARKED in scratch `citefix/`: green in its earlier shape, then amended (DIVERGES_BY_RULING rows into `rows.txt` with reason, ruling and retirement condition; an every-declared-exception-MUST-fire arm) and NOT run since -- finishes FIRST on cc's bounce: build, suite, `sync --to-store ST0056`, `--only`; then ic reviews and dc rebuilds.
- **ic** -> reviews cc's two; owns `install.rs:56` (an ingested `evidence:` line the report never prints) and `intent critic --languages` (written, red-first honoured, green pending the shared target cc holds; its ONLY consumer is devbin's `check:118`, which fails closed -- the shipped hook dispatches the positional at `pre-commit.sh:368` and was never affected; vc's fail-open claim corrected in the runbook and hv's inbox)
  . Fold `wip-fold-1411Z` landed.
- **dc** -> `int macos smoke` MODULES row `83df6f71`; then Anvil (CLAUDE.md carry, `events.jsonl` untrack + ignore, `--only` commit); then ONE `int local build` once cc's two and ic's two are in -> pair sha to every node, read off `intent3 --version` in the same command as each hop 2. Four `int build release` defects filed for after 3.0.1. Rewrote `66c9493e` -> `94d03e9f` (the sweep).
- **devbin/cc** -> post-flip rows accepted. hv un-paused Devbin for one thing, relayed by me, DONE `455d3f0` (armed count read before the verdict, `--format json` predicate, 663/663 plan-matched; unpushed under the grant): `gate_critic` refuses on zero armed rules; `check:118` stays failing CLOSED until 3.0.1 ships the `--languages` fix; then the ten estates re-vendor.
- **devbin/vc** -> Cellar positive control done (`67c037e`; rebuilt with a stub `intent` on PATH after the negative I briefed was inert). Waits on the rebuild: arca_cli full reset (`git checkout -- .; git clean -fd; rm -rf intent/.cache`) then the full sequence; arca_config plain recipe.
- **vc** -> board, runbook, hv inbox surfaced (14:1xZ entry). The seven committed `file` pointers DONE via the canon route (Cdsync `41fd2fb`, Riffle `5982ff7`, Prolix `1298531`); three tool findings on the way (Finder droppings refuse the sync; two `.canon` emitters disagree on `attachments`; empty-title trailing space), in the runbook and filed for cc. After the rebuild: Lamplight (`--only`; hop 2 is the oracle); then the 3.0.1 re-cut on hv's go; then the prune exam, MEASURED under step 8.

## CRITICAL PATH

1-6. **DONE.** Tag `v3.0.0` -> `80d8b2ca` (pushed to both remotes after `main`, CI green macOS + Ubuntu); fold spliced; dc's build on the tag and the pair sha at every hop 2; ST0057 canon flip applied, gate PASS 53/53 (`5c3b1967`); prepare -> formula -> `int macos publish` under hv's grant -> tap install over the network (644 defect: generator `ded669c2`, formula `revision 1`, republished) -> arms a/b/c; **THE FLIP 2026-08-26 14:00:46Z**: `brew link intent` rc=0, `brew test intent` rc=0, `command -v intent` -> `/opt/homebrew/Cellar/intent/3.0.0_1/libexec/bin/intent` (`-rwxr-xr-x`, `intent 3.0.0 (80d8b2ca...)`); post-flip verifier: sixteen at 3.0.0, bare `intent st list` rc=0 in all; `use dev|prod` driven for real by dc. 7. **Batch window (OPEN):** cc's two, ic's two, `converge_gitignore` (`.backup/`, `intent/.backup/`, `intent/events.jsonl`), the template's "Preserved across regeneration." sentence -> dc rebuilds once -> stragglers on the new pair (arca_cli, arca_config, Lamplight, Anvil, seven canon pointers) -> 3.0.1 re-cut on hv's go. 8. **hv's final exam:** prune `Intent/bin` of every v2 script except what devbin needs (`bin/int`, `bin/.devbin/`, whatever devbin's `cmd/*` still `source` -- MEASURE it, do not guess); `tests/run_tests.sh` retires with its subject except what devbin keeps; the three v2 `intent` links on PATH go with it; then the tests run. Only then is it known to have worked. **Measured, read-only:** devbin's only hard v2 dependency is ONE file -- `cmd/build.d/release` sources `bin/intent_helpers` (:67) and calls `bin/intent` for `doctor` (:373), `agents sync` (:663) and `claude upgrade --apply` (:690); `tests/run_tests.sh` sources `bin/intent_helpers` too; `config.yaml:24` already plans `link_alias` to put `bin/intent` back as a link once the v2 file is gone. The bats estate: 60 files reach the CLI through `run_intent` -> `$INTENT_BIN` (default `bin/intent`; the override exists and is what ST0056 parity used) and flip with it; 30 call `bin/intent_*` directly and retire with their subject; 19 exercise no binary (skills, guards, templates) and stay; 13 name devbin.

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
