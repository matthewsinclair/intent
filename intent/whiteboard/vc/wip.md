---
node: vc
name: Validation Claude
role: validation
session_id: 3bbcbe83-cf34-4903-b94d-cd7306a81aca
heartbeat_at: 2026-08-25 22:31Z
status: active
focus: "**FOLDED 2026-08-25 22:26Z FOR A COMPACT, ESTATE-WIDE ON hv's CALL. hv RULED TWICE TONIGHT AND BOTH RULINGS MOVED THE CUT AWAY, NOT CLOSER: the `claude ws` family survives the cut with an expiry the gate enforces, and ALL SIX ST0058 ROWS BIND, so a thread at 0/6 now gates the release.** Gate still PASS at 67 of 67 and CLOSED IS STILL NOT RELEASED. Day record in `.history/20260825/wip-fold-2225Z.md`."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**NOTHING IN FLIGHT. TREE CLEAN. GATE PASS AT 67 OF 67 AND THAT IS STILL NOT A RELEASE.**

**DRIVE THESE, DO NOT TRANSCRIBE THEM:** `ac gate ST0057`, `ac status ST0056/03`, `ac status ST0057`. At fold: PASS, 16/16, 51/51. **`ST0056` 64/134, `ST0058` 0/6, 22 issues open, 84 ahead of `upstream/main`.**

## TODO

1. **ST0058 GATES THE CUT NOW AND IT IS 0/6.** hv ruled ALL SIX BIND. `AC-00.6` is **verified and UNSATISFIED** -- cc's `version` work passes cleanly, and the OTHER twin fails it: `intent --help` rc=0, `intent help` rc=2 _retired_. **`0086` HIGH, three fixes ruled, unowned.** `AC-00.5` is ic's and waits on **the blast-radius count -- consumers that branch on exit 2** -- before its menu goes to hv. **U3 DAILY-COMPLETE is the broad one and `st dehydrate` is unbuilt.**
2. **WP-06 IS A QUEUE OF RULINGS WITH PORTING ATTACHED, NOT A PORTING QUEUE** (cc, having costed all 13 unwired families; `plugin` was the only one with no design question). **So the bottleneck on the cut is vc and hv, not the builders.** Open for hv: are the remaining families each worth their ruling before the cut, or are some out of scope for 3.0.0. **`lang` and `modules` are ruled, sequenced and unstarted.**
3. **TWELVE WITH hv, NONE MINE TO DECIDE:** commit trailer; WP-15 timing; `fileindex`; `--force` version mismatch; TODO 8 ordering; dc's three incl **AT-11.7, never actually asked**; the roster reading two populations from two trees; two estate-wide blocks; limb 2's denominator; the marker's `DIRT_SCOPE`. **Plus** the `st attach` SPELLING and ic's **`## Holds`** protocol change (shipped skill, fleet blast radius). **A2 needs no ruling, only a route: hv's word to dc, FIRST-HAND in dc's own session.**
4. **CARRIED:** `AT-11.7`; `~/.claude/skills/` as one machine-global dir upstream of 15 committed `AGENTS.md`, which reshapes WP-15; **ST0060 de-realised, only AC-00.16 open.** `VIEW_NAMES` routed to cc as _check whether this is still true_, never as a defect with an owner.

## Watch-outs

**These are vc's OWN -- durable cautions, standing, not archived.**

1. **AN INSTRUMENT'S OUTPUT READ AS THE SUBJECT'S ANSWER** (dc's generalisation, over SEVEN instances in one evening across four nodes and three languages). **The worst form is cc's: a mechanism answering in the subject's voice WITH THE CORRECT ANSWER** -- `BYTES IDENTICAL: yes` was TRUE, of two copies of a staleness refusal. **The remedy is cc's and it is the only one that closes it: AN INSTRUMENT MUST ASSERT THAT THE ARTEFACT IS THE ONE UNDER TEST, NOT MERELY THAT TWO OF ITS OUTPUTS AGREE.** dc's is the input half: **establish you HAVE a subject before saying anything about one** -- a floor on the population read. **ic's is the shape: a classifier whose DEFAULT BUCKET absorbs the unrecognised case cannot report that it failed.**
2. **MECHANISM BEATS A NOTE -- AND A CONTROL THAT FAILS FOR ITS OWN REASON IS WORSE THAN NO CONTROL** (ic). Every save tonight came from a control that made the wrong answer impossible; **not one came from having read the warning**, and the zsh word-splitting trap is written down verbatim in the estate's memory and bit ic three times. **But mechanism must be built against the ACTUAL SHAPE OF THE DATA, which is a second place to be wrong**: ic's control used space-separated data where the sweep used newline-separated, reported a healthy instrument as broken, and bought a real investigation with a false alarm.
3. **A PARITY SUITE PROVES TWO BINARIES AGREE; IT CANNOT PROVE EITHER ONE IS USEFUL TO ITS CALLER** (dc, `0085`). **No roster disagreed with any roster and the hook was still wrong.** Every instrument the estate owns compares an implementation to another implementation or to a declaration; **none asks whether the output serves the consumer receiving it.**
4. **A CLOSED LIST IS SAFE WHEN IT DECLARES WHY THE THINGS NOT IN IT ARE NOT IN IT** (dc). Declaring why it is CLOSED is a different claim and does not substitute. **Sibling: AN ABSENT FIELD MUST BE REFUSED, NEVER RENDERED** -- `0086`, where a passing TEST requires an omission to print as _no replacement exists_, against the preamble rule in the file the violation lives in.
5. **A BIDIRECTIONAL CLAIM IMPLEMENTED IN ONE DIRECTION IS GREEN FOREVER ON THE SIDE IT DOES NOT WALK** (dc, two instances in a week). **And the cure is not widening someone else's guard quietly -- that is the defect class rather than the fix.**
6. **A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** A workaround outlives its bug because **a defect announces itself and a defect's DISAPPEARANCE does not** (dc). **Caught myself tonight: I told hv the hook surface was a reason the cut date moves, dc swept it to zero, and I struck it the same evening.**
7. **SHARED CHECKOUT: `--only <paths>` IS THE ONLY FORM ATOMIC WITH RESPECT TO PEERS.** Reading `git diff --cached` measures a MOMENT; the commit happens at another. **`MM` is a claim about the INDEX's cached stat, not about content** (dc) -- diff before you remediate. **A `dirty-` binary may READ canon and may NEVER write it or certify a criterion about the code.** **AND UNDER LOCK CONTENTION GIT ACCUSES THE WRONG THING** (dc): a peer's commit held `.git/index.lock` through its whole pre-commit gate, dc's `git add` failed on the lock, and `--only` then refused with _pathspec did not match any file(s) known to git_ -- **a message that points at YOUR path and says nothing about a lock.** Family 1's head arriving through git: **the LAST error read as THE cause.** Gate the commit on the `add`'s own rc; wait on a LIVE lock rather than clearing it.
8. **AGREEMENT AND DISAGREEMENT ARE BOTH UNINFORMATIVE UNTIL YOU KNOW WHETHER THE TWO INSTRUMENTS ASKED THE SAME QUESTION** (ic). **Denominators must never be summed across questions:** ic's 10 is FAMILIES, cc's 37 is ENTRIES.
9. **CANON IS THE SSOT.** Route: edit `.canon` -> `--to-store` -> `--to-disk`, **never `--to-disk` first.** **No CLI verb CREATES an AC or an AT** -- demonstrated again tonight writing `AC-14.12` by hand. **And the gate refuses a green AT row whose file does not carry the literal id.**

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **A CONTROL THAT MISSES ITS SUBJECT IS WORSE THAN NONE, BECAUSE IT WOULD BE BELIEVED.**
- **A CAPABILITY THE NORMAL ENTRY POINT CANNOT REACH IS NOT DELIVERED** (hv). **CAPABILITY EXISTS != ROUTE EXISTS ON THE SURFACE.**
- **STATE THE INVARIANT, NOT THE MECHANISM.** A sentence describing a mechanism with two implementations across a cut is wrong on one side whichever side you pick. Applied to two SKILL.md lines and `CLAUDE.md:52`.
- **A CONVERGER IS NOT THE CURE FOR A SECOND HOME; ONE HOME IS.** Ground for retiring `lang sync`.
- **A DETECTOR WHOSE GRANULARITY IS THE FAMILY FORCES THE LANDING GRANULARITY TO BE THE FAMILY** (cc, two instances). Every remaining family is priced as one atomic commit.
- **I DO NOT ASK A PEER TO EDIT `CLAUDE.md`, PERMISSION SETTINGS OR CONFIG, AND I DO NOT DO IT MYSELF TO ROUTE AROUND THEIR REFUSAL.** cc refused me tonight and was right; **a guard bypassed when the reasoning is good is not a guard.** It went to hv, who approved it.
- **RECORD THE MENU, NOT ONLY THE SELECTION.** hv rules by choosing among options vc authors; **an option never on the menu cannot be told apart from one declined.**
- **A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL, AND IT BINDS ME TOO.**
- **THE PEN MOVED THE AUTHORITY, NOT THE AIM.** _Get the release done_ is not _release it_. **hv owns tagging, pushing and publishing.**
- **A RATIFIED RULING IS NOT AN EXECUTED ONE.**
