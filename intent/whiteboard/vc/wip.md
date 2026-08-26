---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 13:47Z
status: active
focus: "**BOSS-VC for the v3 fleet cutover under hv's pen and hv's first-hand push grant (Intent + homebrew-intent). THE TAG IS ON MAIN: `v3.0.0` -> `80d8b2ca`, pushed by vc under hv's grant to `upstream` then `local` after `main` went first and CI came back green on macOS and Ubuntu. The verb aborted TWICE (drift guard: two skills declared v3-only by hv's ruling, `402fcbe7`; then `:734` on a `.gitignore` line v2's canon step wrote after the sidecar commit), committed `91d13531` and could not resume, so cc fixed two pre-existing clippy lints the client-side pre-push gate refused (`7553883b`) and regenerated five schema faces the bump had staled (`80d8b2ca`); the trio (fmt, clippy, `cargo test --workspace`: 1175/0) ran on that exact commit before the tag. FOUR VERB DEFECTS FILED FOR dc: sidecar list omits what the canon step writes; preflight lacks the push gate's clippy/fmt; the cut never tests the tree it tags (test gate must move after the sidecar sync); no workflow fires on a tag. FREEZE LIFTED. Then: dc builds on the tag -> pair sha read at hop 2 -> fleet on it (dc 4, ic 3 Molts, devbin-vc Devbin + Arca x3, vc Courses) -> vc drives the ST0057 gate -> dc prepare/formula, vc runs `int macos publish` (hv's grant) -> `brew install` FROM THE TAP over the network -> `brew link` on vc's word, named as its own timed event -> every project re-verified with v3 on PATH. ONLY THE FLIP EARNS DONE. Landed as files (verifier 0 failed): Baize, A3, Riffle, Courses/002, Prolix, Laksa. Courses reverted, re-runs. Lamplight waits for the post-fleet batch (cc's empty-subject fix, proven); Conflab HELD past today (61 rows need a schema migration, done with time; hv can overrule). Two migration defects found by driving and fixed as source; one collapse error of mine reverted and the script halted.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**THE CUT, then the fleet on its pair, then publish, then the flip.** Executable state: `intent/whiteboard/vc/cutover-runbook.md`. Instruments: `verify-canonical.sh` (12-arm self-test; counts chain markers; resolves `core.hooksPath`; demands the generated footer; refuses un-ignored `.backup/`; INFO lines for the user-block digest, the template-default detector, and pre-flip `intent st list` rc), `exposure.sh` (parenthetical sign-offs from history, positive-controlled), `collapse-buckets.sh` (HALTED at its head).

**STATE PER NODE AT THE FOLD (all writes to Intent frozen until the tag):**

- **cc** -> Baize closed (`5bea21c4` + `ab8172a`). Splice `8ba6c026`, chain-block fix `b319fa2a`, bracket-aware `field()` `56364200` landed. Empty-subject AT fix APPLY-READY in scratch (4 files: `legacy.rs`, `views.rs` `(no reference)` renderer, new `legacy_at_without_a_subject.rs`, +2 real rows in `parity/fixtures/rows.txt`), control-proven 884/20 vs 882/22, base digests at `402fcbe7`. `status:` verbatim field WITHDRAWN as a schema migration (user-version 13->14, table rebuild) -- done with time, ic reviews. Conflab's 8 vocabulary rows + 5 no-covers rows are content work on Conflab's turn (not today). Lamplight migrates on the empty-subject fix in the batch.
- **dc** -> keg proven and unlinked on `88b1c92c`; packaging fix `773fcae3`; `use dev|prod` chain driven (machine-wide by PATH; `intent3` is the project-scoped spelling -- with hv as a question). Anvil hops 1-3 dirty; Cdsync/Utilz/MicroGPTEx hop 1. hv ruled `--force` to dc first-hand. Builds on the tag; then prepare -> formula; `publish` is the one verb with three refusals, run by vc under hv's grant (hv or vc -- dc to say whether its confirm needs a TTY). `brew link` on vc's word only.
- **ic** -> Riffle `66b7fdd` + provenance restore `37c5dab`; gate in `90988faf`; ingest fix `2aa82d17`; Molt author placeholder fixed `fc20931`; fold `d232b8c9`. Molt-matts hops 1-3 dirty pending `--force`; Molt-flynn, Molt untouched. Owed to ic: the three Molts on the pair; review of cc's batch fixes on their bounce; the `install.rs` evidence line in the batch.
- **devbin-vc** -> Devbin renumber `19579c3`, hop 1 `75bb005`; Arca x3 (hv ruled them theirs): `project_name`/`author` = `matts` hand-added BEFORE hop 2 on arca_cli/arca_config; `## Migration Notes` carried by hand; arca_notionex's `languages` from hop 1, verified not assumed. Cellar positive control against the TAP keg after publish.
- **devbin-cc** -> baseline all rows; sweep `git log --grep='migrate to v3 canonical'` unprompted; asks the LINK be announced as its own timed event.
- **vc** -> A3 `f0c55ed`, Courses/002 `a50b682`, Prolix `bc620d4`, Laksa `2bf06a27` + `f5133dc8` landed; Courses reverted `aa25be1` (re-run on the pair: `.backup/` already ignored, hop 1 again from 2.14.0). AT-08.6/08.7 falsifiers 11/11 green at `2aa82d17`; the canon flip is PARKED at scratch `ST0057.flipped.json` with `note` (NOT `evidence` -- an AT row carries `note`); apply via `.canon` -> `--to-store` -> `--to-disk` -> `ac gate ST0057` once `intent3` is current.

## BOUNCE CHECKLIST (in order; each step names the thing it waits for)

1. `git tag -l v3.0.0` in Intent. If PRESENT: freeze lifts. If NOT: hv's terminal is the only source -- ask hv what it printed; if it aborted at `:734` on dirt, find the writer, clean, re-fire (third time).
2. Splice this fold into `intent/whiteboard/vc/wip.md` with `heartbeat_at` read from `date -u` at that moment; commit `--only` it. Then the runbook additions below, one commit.
3. dc: `int local build` on the tag -> pair sha off `int local status` -> send it to cc, ic, devbin-vc, dc; every migration body carries the pair read off `intent3 --version` in the same command as hop 2.
4. vc: apply the parked canon flip -> sync -> `ac gate ST0057`; tell hv if it does not close before anyone publishes. Laksa `author` -> `matts` + regenerate (follow-up commit). Courses re-run (phaseA.sh, no collapse). Verifier arms: empty H1; empty-or-placeholder `author`/`project_name` READ FROM THE CONFIG FIELD, never a grep (exact directory match is NOT the test).
5. dc: prepare -> formula -> stop; vc runs `int macos publish` (or hv, if it needs a TTY) -> `brew install matthewsinclair/intent/intent` FROM THE TAP -> arms (a) bare `claude upgrade` names five from a non-checkout dir, (b) `intent info` INTENT_HOME onto libexec with `lib/templates/hooks/pre-commit-guards.sh` under it, (c) hooks rc=0 / unknown rc=1, + `--apply` in a throwaway.
6. THE FLIP on vc's word: `brew link`, announced as its own timed event to every node; then `INTENT_FLIPPED=1` verifier re-run on every landed project; devbin-cc's post-cut rows; devbin-vc's positive control; `use dev|prod` driven in Intent.
7. Batch window (fleet through on the pair): cc's empty-subject fix (ic reviews), ic's `install.rs` evidence line, `converge_gitignore` two anchors, the template sentence; dc rebuilds; Lamplight migrates on it; final re-cut -> 3.0.1.
8. hv's final exam: prune `Intent/bin` of every v2 script except what devbin needs (`bin/int`, `bin/.devbin/`, whatever devbin's `cmd/*` still `source` -- measure it); `tests/run_tests.sh` retires with its subject except what devbin keeps; then the tests run.

## RUNBOOK ADDITIONS OWED (apply at step 2)

- A constraint names the STATE a gate reads, never the ACTION a human pictures (ic): "no commits" forbade the thing that removes dirt and permitted the write that creates it; `release:734` reads dirt. Freeze = no writes of any kind inside the repo; park outside.
- A node holding through a release cut cannot signal it is alive (cc): heartbeat is a write, the preflight refuses dirt; the board loses for the length of the cut, correctly.
- `shipped_surface_drift.bats` runs BEFORE a single-tree skill edit, not at the tag (cc, on `b60f9ebb`); a frozen v2 makes every such edit drift by construction; the hatch is `_is_v3_only_by_ruling` with the ruling cited.
- A dry-run of a long gate is redundant only if it is BEHIND the run you keep (dc): dc's was seven minutes ahead and carried the answer.
- `--force` means three different things on three files spelled the same way (devbin-vc): template by position, authored by content; the only instrument that separates them is a line-by-line diff against a before-state.
- Conflab is HELD past today with its reason (61 `status:` rows need a schema migration; hand-normalising is the estate conforming to the tool); hv can overrule.
- hv's final exam: prune `Intent/bin` (AC-12.1) after the flip.

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
