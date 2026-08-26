---
node: vc
name: Validation Claude
role: validation
session_id: 699601ed-7e13-4808-bb6c-e6a79d27c56e
heartbeat_at: 2026-08-26 11:25Z
status: active
focus: "**BOSS-VC: hv handed me the pen at ~10:56Z with authority over intent-{cc,dc,ic} and devbin-{vc,cc} for the fleet migration, then went AFK. THE FLEET IS 21, NOT 16 (devbin-cc's census; my sixteen was a depth-limited walk). ZERO PROJECTS MIGRATED, DELIBERATELY: three defects found before any live write, all by driving -- `claude upgrade` doubles the chain block in every v2-written hook (cc, own slice; and hop 1 CREATES the hook so nothing was safe); the brew keg installs, answers --version and cannot find its templates because brew strips the tarball's top-level lib/ (dc, own lane, fixed at 773fcae3); Devbin's hop 2 refuses atomically on a duplicate AT id (devbin-vc, independent drive). Every ruling made in hv's absence is in hv/inbox.vc.md for ratification. Runbook + verifier are the executable state; verifier self-test trips 11 arms including a doubled hook and resolves core.hooksPath. Riffle held DIRTY at hop 1. Waiting on: cc's fix sha -> dc's re-cut -> fleet resumes.**"
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**DIRECTING THE FLEET CUTOVER. The executable state is `intent/whiteboard/vc/cutover-runbook.md`; the instrument is `intent/whiteboard/vc/verify-canonical.sh`. Both current as of the header stamp.**

**ALLOCATION (all held on cc's chain-block fix + dc's re-cut; nothing resumes until I send the new pair sha):**

- **cc** -> land the `canon.rs` chain-block fix NOW (colon form emitted, both forms detected, anchored on the marker line, fixture = Baize's real hook). Then Baize, Conflab, Laksa (hand-fix `project_name`/`author` first), Lamplight last, `--only`, tell me first. `core.hooksPath` is honoured at the writer (`canon.rs:276`), no action.
- **dc** -> re-cut on cc's sha with the `cmd_stage` wrapper (`773fcae3`); prove (a) bare `claude upgrade` names five from a non-checkout dir, (b) `intent info` INTENT_HOME onto libexec with the guard runner under it, (c) hooks rc=0 / unknown rc=1, plus cc's stronger arm: `--apply` in a throwaway. **The flip (`brew link`) is on MY word only.**
- **ic** -> re-bucket + `mutation_creates_criteria_and_tests.rs` in one commit when `crates/**` frees; then I drive the AT-08.6/08.7 falsifiers; fleet slice Riffle (held dirty at hop 1), Molt-matts, Molt-flynn LIVE, Molt.
- **devbin-vc** -> hop-2 PRE-FLIGHT on a sandbox copy of every estate (the fleet count is an upper bound until it reports); diagnose Devbin's duplicate `(thread_id, id)` and PROPOSE, I rule; positive control against the re-cut keg when I say.
- **devbin-cc** -> devbin-side verification, 10 rows in my fleet, delta vs baseline as each landing is NAMED by me (a change observed is not a landing -- their Riffle "clean pass" was hop 1).
- **UNALLOCATED:** A3/a3-content, Anvil, Arca x3, Cdsync, Courses, Courses/002, Devbin (after its content fix), MicroGPTEx, Prolix, Utilz. Allocate on the new stamp, pre-flight results in hand.

## TODO

1. **THE MOMENT cc's SHA LANDS:** relay to dc as the build target. **THE MOMENT dc's PAIR LANDS:** send the sha to cc and ic; fleet resumes. Re-run the verifier on my canary AFTER re-applying with the new binary -- the doubled hook must converge to one (cc's fix detects both forms), and if it does not, the fix does not handle the already-doubled case.
2. **AC-08.6/AC-08.7:** drive ic's falsifiers at HEAD myself; `stale_at_check.sh` going 0 -> 2 exposed is the handoff signal, not breakage; a third name is real.
3. **CANON WRITES, HELD ON THE SUITE, IN THIS ORDER WHEN IT CLEARS:** `AC-00.6` satisfy on ic's evidence; `AC-11.7` RE-MINT (ruled in scope today); `AC-00.1` fix my wrong CLEAN-BUT-BEHIND sentence; FILE: chain-block doubling (cc), keg-cannot-find-itself (dc), Devbin duplicate AT id (devbin-vc), `AGENTS.md` derives project name from the DIRECTORY not `config.project_name` (devbin-vc), the three-finding `--only` paper (devbin-cc's framing), `write_config` materialises defaults (cc, 23:15Z, inbox).
4. **VERIFY EVERY PROJECT AS IT LANDS** with the script; a hand-finished project is NAMED as such in the commit body.
5. **OWED TO hv (in hv/inbox.vc.md, for ratification):** six rulings under delegation; the `use`-is-machine-wide question with my interim ruling (`intent3` is the project-scoped dev spelling).
6. **AFTER THE FLIP:** every project re-verified with the brew binary on PATH; devbin-cc's post-cut leg per estate; devbin-vc's positive control.
7. **POST-FLEET SOURCE BATCH -- one commit window after the fleet is through, then ONE final re-cut for the release keg (every source commit today costs dc an Apple round trip; test-only commits do not stale the binary):** (a) `install.rs:56` remedy discriminates its three causes -- outside the tree / inside at the wrong depth / copy-vs-symlink (ic, held on my ruling); (b) `canon.rs` preserves the `CLAUDE.md` user block across `--force` (if cc defers it); (c) anything else that surfaces.
8. **STILL MINE, NONE STARTED:** WP-15's split line (the migration unit is the machine, not the project -- and `use` being machine-wide is the third instance); triage WP-06's eight unwired families; retire `A5`/`A7`; the help surface (142 pages, XL) after migration lands; `sync` parked.

## Watch-outs

**These are vc's OWN -- durable cautions, standing, not archived.**

1. **AN INSTRUMENT'S OUTPUT READ AS THE SUBJECT'S ANSWER** (dc's generalisation, over SEVEN instances in one evening across four nodes and three languages). **The worst form is cc's: a mechanism answering in the subject's voice WITH THE CORRECT ANSWER** -- `BYTES IDENTICAL: yes` was TRUE, of two copies of a staleness refusal. **The remedy is cc's and it is the only one that closes it: AN INSTRUMENT MUST ASSERT THAT THE ARTEFACT IS THE ONE UNDER TEST, NOT MERELY THAT TWO OF ITS OUTPUTS AGREE.** dc's is the input half: **establish you HAVE a subject before saying anything about one** -- a floor on the population read. **ic's is the shape: a classifier whose DEFAULT BUCKET absorbs the unrecognised case cannot report that it failed.**
2. **MECHANISM BEATS A NOTE -- AND A CONTROL THAT FAILS FOR ITS OWN REASON IS WORSE THAN NO CONTROL** (ic). Every save tonight came from a control that made the wrong answer impossible; **not one came from having read the warning**, and the zsh word-splitting trap is written down verbatim in the estate's memory and bit ic three times. **But mechanism must be built against the ACTUAL SHAPE OF THE DATA, which is a second place to be wrong**: ic's control used space-separated data where the sweep used newline-separated, reported a healthy instrument as broken, and bought a real investigation with a false alarm.
3. **A PARITY SUITE PROVES TWO BINARIES AGREE; IT CANNOT PROVE EITHER ONE IS USEFUL TO ITS CALLER** (dc, `0085`). **No roster disagreed with any roster and the hook was still wrong.** Every instrument the estate owns compares an implementation to another implementation or to a declaration; **none asks whether the output serves the consumer receiving it.**
4. **A CLOSED LIST IS SAFE WHEN IT DECLARES WHY THE THINGS NOT IN IT ARE NOT IN IT** (dc). Declaring why it is CLOSED is a different claim and does not substitute. **Sibling: AN ABSENT FIELD MUST BE REFUSED, NEVER RENDERED** -- `0086`, where a passing TEST requires an omission to print as _no replacement exists_, against the preamble rule in the file the violation lives in.
5. **A BIDIRECTIONAL CLAIM IMPLEMENTED IN ONE DIRECTION IS GREEN FOREVER ON THE SIDE IT DOES NOT WALK** (dc, two instances in a week). **And the cure is not widening someone else's guard quietly -- that is the defect class rather than the fix.**
6. **A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** A workaround outlives its bug because **a defect announces itself and a defect's DISAPPEARANCE does not** (dc).
7. **SHARED CHECKOUT: `--only <paths>` IS THE ONLY FORM ATOMIC WITH RESPECT TO PEERS -- AND `--only` SEPARATES FILES, NOT AUTHORS** (cc, third instance): `render.rs` has four writers, so a shared-file edit is not safe to leave uncommitted for ANY interval while four nodes are live. Detection is `git show --stat HEAD` against the length of your own pathspec -- what the command DID against what it was TOLD. **`MM` is a claim about the INDEX's cached stat, not about content** (dc). **A `dirty-` binary may READ canon and may NEVER write it.** **UNDER LOCK CONTENTION GIT ACCUSES THE WRONG THING** (dc): the LAST error read as THE cause.
8. **AGREEMENT AND DISAGREEMENT ARE BOTH UNINFORMATIVE UNTIL YOU KNOW WHETHER THE TWO INSTRUMENTS ASKED THE SAME QUESTION** (ic). **Denominators must never be summed across questions.** **And three instruments agreeing on a wrong fixed point is not corroboration when all three ask the same question** (ic, today): `0 written`, hop 3's `written:` line and a `grep -q` all read a doubled hook as correct because none of them COUNTED.
9. **CANON IS THE SSOT.** Route: edit `.canon` -> `--to-store` -> `--to-disk`, **never `--to-disk` first.** **No CLI verb CREATED an AC or an AT** until ic's `ac new`/`at new` at `461aea84`. **And the gate refuses a green AT row whose file does not carry the literal id.**
10. **EVERY TIMESTAMP IS READ IN THE SAME COMMAND THAT WRITES IT.** Today I read `10:55Z` off `date -u`, then read `11:48`/`11:57` off `ls -la` -- LOCAL mtimes -- and wrote "driven at 11:55Z" into three peer messages. A stamp read off a file listing and given a Z. ic caught it against `date -u`. **And `${PIPESTATUS[0]}` is EMPTY in this shell** (zsh: `$pipestatus[1]`); an rc read through it reads as success. Every rc goes to a file.

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **A CONTROL THAT MISSES ITS SUBJECT IS WORSE THAN NONE, BECAUSE IT WOULD BE BELIEVED.**
- **A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED** (hv). **CAPABILITY EXISTS != ROUTE EXISTS ON THE SURFACE.**
- **STATE THE INVARIANT, NOT THE MECHANISM.**
- **A CONVERGER IS NOT THE CURE FOR A SECOND HOME; ONE HOME IS.**
- **A DETECTOR WHOSE GRANULARITY IS THE FAMILY FORCES THE LANDING GRANULARITY TO BE THE FAMILY** (cc).
- **I DO NOT ASK A PEER TO EDIT `CLAUDE.md`, PERMISSION SETTINGS OR CONFIG, AND I DO NOT DO IT MYSELF TO ROUTE AROUND THEIR REFUSAL.**
- **RECORD THE MENU, NOT ONLY THE SELECTION.**
- **A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL, AND IT BINDS ME TOO.** And its sibling today: **A LATER FIRST-HAND RULING FROM THE SAME PRINCIPAL ON THE SAME SUBJECT SUPERSEDES** -- hv's 00:19Z "macos out of scope" against hv's 10:56Z "the brew install version has been installed"; dc had both first-hand and asked me to rule the tension rather than pick.
- **THE PEN MOVED THE AUTHORITY, NOT THE AIM.** `publish`, tagging, pushing are hv's. **Rulings made under delegation go to hv's INBOX for ratification, never into hv's standing directives, which are hv's word only.**
- **A RULING THAT DISCHARGES AN ITEM ON ANOTHER NODE'S BOARD GOES IN THEIR INBOX, NOT ONLY OVER SendMessage.** Today: every ruling was live first and durable within the hour, and the durable copy is the one that survives the next compact.
- **A LABEL IS NOT AN ESCALATION, AND IT LOOKS EXACTLY LIKE ONE.** TRIAGE BEFORE RULING.
- **A REFERENCE THAT RESOLVES TO THE WRONG SUBJECT READS AS COVERAGE; ONE THAT DANGLES ANNOUNCES ITSELF** (`0088`).
- **AN INSTRUMENT MUST NOT BE ABLE TO OBSERVE ITS OWN EARLIER OUTPUT.** A fresh tree per arm, always.
- **A CRITERION'S TEXT IS A RECORD OF A PAST MEASUREMENT, AND THE TREE MOVES.** Re-drive the FALSIFIER, never re-read the EVIDENCE. **A ROW MUST NAME WHAT WOULD FALSIFY IT.**
- **CLEAN-BUT-BEHIND IS A THIRD BINARY STATE, AND IT IS MECHANISED IN THE `intent3` WRAPPER, NOT THE BINARY** -- the refusal carries a devbin remedy, which is how I found where it lives. Test files do not count (`currency.lib` filters `tests/`), so ic's ATs cannot stale the board.
- **THE CUT NEEDS A QUIET TREE, NOT JUST A GREEN ONE.**
- **A NUMBER THAT TRAVELS OUT TO A PEER AND COMES BACK READS AS CORROBORATION AND IS AN ECHO** (cc). I am the node that routes numbers between all the others.
- **A SWEEP TAKES ITS POPULATION FROM THE ARTEFACT'S OWN DECLARATION AND NEVER DERIVES ONE BY TRAVERSAL** (cc). **Three times today the population was wrong the same way: fifteen for sixteen (Riffle subtracted), sixteen for twenty-one (a depth-limited walk), and a hook survey taken BEFORE the hop that creates hooks.** The census predicate is now in the runbook with its filter, because the filter is where the next error lives.
- **A SAMPLE THAT CANNOT EXHIBIT THE DEFECT IS NOT A WORST CASE.** I picked Molt-flynn as worst case on the version axis; the fleet varies on the hook axis, and a 2.11.5 project with no hook is the one shape the doubling could not reach. Then hop 1 gave it a hook and hop 3 doubled it in my own sandbox, and I read `written: .git/hooks/pre-commit` as success. **A second operator on a second project is a control; one drive is a claim** -- devbin-vc's Devbin drive found the content refusal my drive could not.
- **A CHANGE OBSERVED IS NOT A LANDING.** devbin-cc's baseline instrument saw Riffle's hook appear and marked it "landed clean"; it was hop 1, held dirty. The landing signal is the director NAMING the project with its commit sha; a delta is a hop until then.
- **A CONVERGER AT THE WRONG FIXED POINT IS BLESSED BY THE IDEMPOTENCE CONTROL.** `0 written` on the second `--apply` is necessary and not sufficient; only a COUNT sees a doubled block.
- **AN ARM THAT DEMANDS WHAT THE TOOL CANNOT PRODUCE IS A SPEC DEFECT, NOT A PROJECT DEFECT.** `intent_dir` retired from the verifier: v3 defaults it and `stamp_version` never writes it, so an absent key IS the canonical configuration. "By the tool" can only reach what the tool writes.
- **`core.hooksPath` MAKES `.git/hooks/` INERT** (devbin-cc). Laksa (`bin/hooks`) and Intent itself (`.githooks`). The tool resolves it (`canon.rs:276`); my verifier did not, and reported Intent's own hook as absent all day. Resolve the way git does.
- **`use` IS MACHINE-WIDE BECAUSE PATH IS** (dc). There is no project-scoped swap through PATH; `intent3` is the project-scoped spelling. This is the third instance of _the migration unit is the machine, not the project_ and it goes to hv as a question, not a ruling.
- **A CORRECT DIAGNOSIS PLUS A REMEDY NOBODY DROVE END-TO-END IS STILL A BROKEN INSTALL** (dc, on reproducing their own header's trap by a different road). **And a simulation written from the same understanding as the fix confirms the understanding, not the fix** -- dc declined to count their hand-applied brew strip as proof for exactly that reason.
- **READING THE WRITE-UP OF A CLASS IS NOT PROTECTION FROM IT.** The remedy is never care. It is `<<'EOF'` by default, an rc to a file, a stamp read in the command that writes it, a fixture that trips every arm, a population from the artefact's own declaration.
