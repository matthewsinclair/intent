---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-27 09:05Z
status: active
focus: "BOSS-VC. THE SERIAL PORT, plus hv's ruling that Intent stop being half-pregnant. DONE OVERNIGHT: WP-14 landed (9bd6b0a3, schema 13->14, both control halves green); ST0058/01 landed (c88679fd) -- FIVE answers to 'run the intent CLI' collapsed to one, int cli carries the pair guard and runs release by stated choice, intent3/intentd3/.envrc gone, Intent at doctor 0; Molt-matts e834d2a stops putting the frozen Intentv2 on PATH, which was giving a SECOND int that ran a 3.0.0-dev debug build from 21 Aug wherever a shell had not started in the Intent checkout (hv found it in Laksa). arca_cli re-convert REHEARSED CLEAN on a throwaway: 19 -> 6, verifier 0, cc's comma cut confirmed live (0 of 55 rows carry a clause). STILL hv's: Conflab's four ST0121 rows, and Laksa's attribution guard (292 findings, 110 of 110 archival, classifier refused my edit and I did not route around it). NEXT: real arca_cli, then Utilz, both needing Intentv2 hop 1."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**THE SERIAL PORT.** hv's tiers are on hv's board, verbatim. **The ledger, the procedure and every LEARNED bullet are in `cutover-runbook.md` under PORT LEDGER** -- read that, not this. Standing rulings: buckets -> `intent/history/`; `deps/`, `_build/`, `node_modules/`, `.worktrees/` are not projects; a WP recorded WIP over a passing gate is `wp done`; the backup schedule is configuration and is NEVER silenced; doctor advisories are printed only under `--verbose`; llm-tropes and Pplr are NOT Intent projects (hv, tonight -- my llm-tropes `init` was reverted, repo pristine at `8bac3c9`).

**THE PAIR:** `$S/pair-f7240814/{intent,intentd}`, and `target/{release,debug}` + `~/.local/bin` are all f7240814 after hv ran `int local use dev`. **Bare `intent` was the keg until that switch, which is why hv read the pre-fix doctor twice while I read the fixed one.**

**FIVE FORMS OF ONE MIGRATOR DEFECT, ALL WITH cc (assigned on hv's word):** (1) `unread_field_keys` slices bytes, panics on `✓` -- Conflab, rc 101, rehearsed on a clone so the estate is untouched; (2) `path, describe "..."` kept whole as `file` -- arca_cli ST0011, 16 rows, every file exists; (3) a prose `covers` clause carried as an id -- arca_cli, 3 rows; (4) `is_path` = `contains('/') && !contains(':')` takes any prose with a slash for a path -- Utilz ST0009 (3 rows, `bin/`), Laksa ST0081 (a `mix test .../...` command line); (5) a whole sentence as `file` with empty covers -- Laksa ST0090 AT-00.2. **Nothing is hand-edited in the estates that prove it: they are re-converted from the v2 source after cc's sha.**

**WHAT REMAINS:** Lamplight (the biggest: 3740 unclaimed, 10 lost citations, 3 view-skew, `.worktrees` at 2.19.0 for hv); Intent itself (ST0055's five `wp done`, ST0057/WP-08 which is ic's, the two shell reds in my lane, the PASSES-branch remedy text); then the five estates that unblock on cc's fix.

## HELD FOR hv

- **Laksa's own work state, 14 packages** (the mechanical port is committed; these are not mine): eight recorded WIP/Not Started over a PASSING gate (ST0079/01, ST0084/12,13,14,16, ST0086/08, ST0101/02,03, ST0102/01) -- the shape hv ruled `wp done` on in Baize; four Done over genuinely unsatisfied criteria (ST0087/14,15 and ST0097/08,15). Utilz ST0009/WP-01 and Intent's ST0055 WPs are the same question.
- **The formatter is a second writer in six more projects** (Conflab, Laksa, Lamplight, MicroGPTEx, Molt, Utilz run `prettier --write` over staged markdown with no view exclusion; Laksa and Prolix are fixed under hv's own 2026-08-19 ruling). Laksa and Lamplight get it as part of their port; the other four are preventive and need hv's word.
- **ic's D53 question** (does a flush survive a git clone -- both options carry a cost, neither is mine).
- The daemon's scheduled snapshot vs manual; `intent config get|set` and `st repair` are stubs; `finding.rs:348` prints the BLOCKED remedy under the PASSES finding.
- Lamplight's 22 non-UTF-8 files refusing `--to-store`; the two stale `.worktrees/{cc,ic}` at 2.19.0.

## Watch-outs

**These are vc's OWN -- durable cautions, standing, not archived.**

1. **AN INSTRUMENT'S OUTPUT READ AS THE SUBJECT'S ANSWER.** Today in my hands: a display alias (`{evidence: (.evidence // .note)}`) read back as the schema; a process grep matching dc's WATCH LOOP and reported as hv's release for four minutes; a catch-all in `legacy.rs` inverting eight hv sign-offs at exit 0 (ic). **A classifier whose DEFAULT BUCKET absorbs the unrecognised case cannot report that it failed.**
2. **MECHANISM BEATS A NOTE.** zsh does not word-split (four times, mine); BSD awk has no `\s`; `${PIPESTATUS[0]}` is empty; an rc read through `tail` is the pipe's. **Every rc to a file; every loop over a newline list in a bash FILE; every chain gated on the script's rc** -- my Prolix collapse printed `stop=1` and I committed.
3. **A PARITY SUITE PROVES TWO BINARIES AGREE; IT CANNOT PROVE EITHER IS USEFUL TO ITS CALLER** (dc, `0085`).
4. **A CLOSED LIST IS SAFE WHEN IT DECLARES WHY THE THINGS NOT IN IT ARE NOT IN IT** (dc). **AN ABSENT FIELD MUST BE REFUSED, NEVER RENDERED** (`0086`); its sibling: a known token with an EMPTY value substitutes silently where an unknown token refuses loudly (`rootfiles.rs:335`, devbin-vc).
5. **A BIDIRECTIONAL CLAIM IMPLEMENTED IN ONE DIRECTION IS GREEN FOREVER ON THE SIDE IT DOES NOT WALK** (dc).
6. **A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** A pair sha issued in advance is valid only until the next refusal; the body carries the pair that PERFORMED the migration, read at the time. **A number that travels through a message has a transcription hop nothing checks** (a digest wrong at character 17; a scope difference read as a mismatch). Full-length, by command, against the file.
7. **SHARED CHECKOUT: `--only` SEPARATES FILES, NOT AUTHORS; two reads in one command or they are two facts** (dc). A dirty `native/rust` REDIRECTS a build to a private target the wrapper never reads. **AND A `git status` LINE NAMES PATHS, NOT AUTHORS**: I read four dirty paths and assigned all four to ic; two were cc's live migrator mid-gate, and "park ic's four" would have parked them. Nobody parks a path they did not write; the owner names the tree clean by their own hand.

8. **AGREEMENT IS NOT CORROBORATION WHEN EVERY INSTRUMENT ASKS THE SAME QUESTION** (ic). **A change observed is not a landing. Only the flip earns "done"** (devbin-cc, hv). **A loop over a collection the defect EMPTIES cannot witness the defect** (cc) -- assert the length first.
9. **CANON IS THE SSOT FOR ROWS, NOT FOR PROSE.** Delete only a line-subset; otherwise keep. **The store's refusal is the oracle; nobody edits a ratified contract off a grep's prediction.** And the tail after a verdict token is OPAQUE -- read the verdict from it, never decompose and reassemble it (cc's withdrawn `status:` fix rewrote authors' bytes with no red anywhere).
10. **EVERY TIMESTAMP IS READ IN THE SAME COMMAND THAT WRITES IT. A NUMBER IN THE TERMINAL IS NOT A CLOCK** (cc). A fold parked outside the repo carries `2026-08-26 13:47Z` and stamps at apply.
11. **A PERMISSION BOUNDARY IS PER SESSION AND IS NOT ROUTED AROUND BY ANYONE, INCLUDING THE DIRECTOR.** Three refusals today were right: mine on taking dc's `--force`, mine on killing dc's process, mine on editing a gate's test without hv's word. **A relayed STOP is honoured on the relay; a relayed approval is not an approval; a lift carries hv's words verbatim; a peer's push authority is not mine to hand on.**
12. **A SUITE READING A LIVE ESTATE MEASURES A MOVING TARGET** (ic); only the failure TEXT tells a peer's mid-write from a regression. And the corpus cannot exercise the half the real data never reaches -- synthetic refuse cases, or the green proves nothing (cc, 1910 of 1910).

13. **UNTIL 2b'S PAIR IS BUILT, NO `intent` WRITE VERB AT ALL ON A RE-CONVERTED TREE; UNTIL 3.0.1'S KEG, WRITE VERBS ONLY ON THE VERIFIED PAIR BY ABSOLUTE PATH** (Lamplight, 2026-08-26, local `stat` 19:45:00 BST -- the Z form I wrote earlier was DERIVED from that local read, not read, and is retracted as a stamp; settled 19:03Z): READ verbs on the keg (`st list`, `ac gate`, `todo`) write nothing -- proven on clones of Lamplight and Cdsync and on dc's fixtures with a write control. What rewrote 167 tracked views on Lamplight at 19:45:00 BST by `stat` (local; not read in UTC) was the KEG via a WRITE verb: cc showed statically that the current tree cannot emit `(no reference)` since `e696de15`, that there is ONE emitter in production (`views::render_all`), and that `strings` finds the literal in `/opt/homebrew/bin/intent` (`80d8b2ca`) and not in `bin/intent3`. Which verb is the filer of Lamplight's `.canon/issues/0005.json`'s to say (a real defect report, untracked, a peer's live work -- hands off). `bin/intent3` refuses whenever native/rust moves past its stamp (the guard working). Check `git status` after the first call, whatever the verb.

14. **AN INSTRUMENT'S DEFECTS FAIL TOWARD CALM** (devbin/vc, four in one day, zero false alarms): a run that measured nothing must print a verdict that is NOT a pass; a pass is a `VERDICT:` line at rc 0 and nothing else; a self-test proves each arm in both directions. And mine: an anchored edit (`^7\. `) hit a numbered watch-out instead of the step I meant -- verify the last step's effect (a grep count, a sha) after every multi-step call, because a zsh parse error or a wrong anchor runs NOTHING and prints little.

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
