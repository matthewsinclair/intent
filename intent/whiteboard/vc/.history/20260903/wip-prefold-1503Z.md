---
node: vc
name: Validation Claude
role: validation
session_id: d6fb337d-3328-4360-865e-04ff4ba194e7
commit_session_id: 012urACYMDQ4oZhtofSEJzxg -- the Claude-Session trailer id on commits authored from THIS session; POINT-IN-TIME, one session per line, and the mapping accumulates in .history/ rather than growing here (0208)
heartbeat_at: 2026-09-03 07:33Z
status: active
focus: "STATUS ROUND TAKEN 2026-09-03 07:34-07:40Z, all three peers replied, all three clean and holding. THREE PREMISES DIED OVERNIGHT AND EVERY ONE WAS ON AN hv LIST: AC-17.1 (cc built Op::Set, so the reword-vs-descope menu has no subject), the built-pair lag (rebuilt, at edit --note is in the binary), and AT-07.5's window (an observable, not a consent). ONE NEW AND IT IS THE WORST KIND -- a SHIPPED-SHAPED artefact carrying a FALSE claim, dc's find, verified here. THE BOARD RULE: printing the command is not running it, so a live figure here is the COMMAND ALONE."
claims: [ST0056, ST0057, ST0060, ST0064, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`. Every incident narrative is in `.history/`. What follows is MECHANISMS and UNEXECUTED WORK.**

**AND THE RULE THAT GOVERNS THIS BOARD: PRINTING THE COMMAND IS NOT RUNNING IT.** A figure with its regenerating command beside it is AUDITABLE, not CURRENT, and no reader can tell which they are holding -- **proved by TN001's crate table going stale UNDER a caption boasting it was cargo-verified.** **A LIVE FIGURE APPEARS HERE AS THE COMMAND ALONE, WITH THE NUMBER DELETED.** **A HISTORICAL FIGURE IS DATED**, and stops being a live claim.

## DOING

**ROUTING AND ADJUDICATING; nothing of vc's uncommitted.** hv delegated the open set (_"go with your recs"_), so **a vc ruling under hv's pen declares `authority: vc`, NEVER `authority: hv`** -- and see `9c` below, which is vc getting that exactly wrong in the message that explained it.

**STATUS ROUND 2026-09-03 07:34-07:40Z: cc, dc, ic ALL replied, ALL clean, ALL holding.** Tree carried exactly one dirty path all morning and it was this file. cc is PARKED and the condition is hv's word, in hv's own voice -- not a peer's, not an idle tree, not a green gate. dc's whole open queue is the v3.0.1 release note. ic is the only node with an unsatisfied row still in its own scope (`AC-17.6`, gated on cc's WP-08).

**THE MORNING'S FOUR MEASUREMENTS, DRIVEN NOT RECALLED.** (1) The currency arm reads **ok** -- `intent --version` and the build-input rev-list name the same commit; **dc's 62-commits-behind figure is a count of differences, not of damage, and 0 of the 62 touch a build input.** (2) `intent ac gate ST0056` **REFUSES rather than counts** -- a contract finding short-circuits the satisfaction report, so `ac status` emits no `N/M` at all; `ac list` reads past it. **BLOCKED here does NOT mean one literal from PASS, and ic nearly reported that it did.** (3) `doctor` puts WP-05 AND WP-08 in status-gate-disagreement -- both WIP, both gates PASS, driven per-WP. (4) `intentd` is up (pid 66522), so **`0216` exposure is live for anything canon-touching today.**

**vc's OWN WORK, UNSTARTED: `WP-15` skills-catalogue triage** (`ST0065`'s criteria are the bar it works to); **the estate-tree question**, unsourced and born in a fold; and **`hv/inbox.vc.md`'s lifecycle**.

## Holds

**Each carries the CONDITION that releases it. A hold with no condition is an abandonment.**

- **`ST0068` AC-02.1 -- and the thread prefix is NOT decoration: `ST0056` ALSO has an `AC-02.1`.** vc read a bare note onto the wrong thread. **CONDITION: A RELEASE, NOT A BUILD.** `--note` landed `6fa22a79`; `docs/getting-started.md:124,126,135` instruct it; the keg has none.
- **`AT-07.5`'s behavioural arm is not re-verifiable while a daemon runs**, and one always is. Green with the caveat; the tool REFUSES rather than fails. **Condition: hv authorises a daemon-down window. A node must not take one.**

## hv items

**FOLDED 2026-09-02 23:17Z; pre-fold verbatim at `.history/20260902/wip-prefold-2312Z.md`. Everything discharged this evening is archived there with its reasoning.**

### hv's HANDS -- four

1. **`flip` THEN `burn`, ONE SITTING, IN THAT ORDER.** `flip` = rebind the default `INTENT_BIN` off `bin/intent`, the v2 SHELL SCRIPT (`tests/lib/test_helper.bash:21`). `burn` = re-run `burn.sh`, every `.bats` twice, classifying by the delta; hung 3.5h once. **A burn before the flip produces a baseline the flip invalidates. NEITHER NEEDS A BUILD.** Also `AC-06.1`'s remaining coverage limb.
2. **`ST0057 AT-07.5` NEEDS A STOP-THE-DAEMON WINDOW.** Arm A REFUSES rather than fails while an intentd is up, **so the green rests on a moment nobody can reproduce on demand.** **NO LONGER PAIRED WITH `0216`'s ARM** -- that arm wants the daemon RUNNING, so this is the only remaining window ask.
3. **`ST0064` 01.7 SIGNING NEEDS hv's ADC.**
4. **THE DOC GATE -- RE-RULE ON CORRECTED GROUND.** vc's _flip now, fix forward_ is WITHDRAWN: cc re-drove the class against the SITES rather than the NAMES and **five of six supposed absentees are present under another name.** **The class is ONE-TOKEN PATH REPAIRS, not missing mechanisms**, so _a gate waiting for zero on a growing class waits forever_ is false. **vc recommends the config's OWN condition: clear the class, then flip.** Held with dc; not built.

### For hv's morning, narrowed rather than guessed

5. **`intent claude skills sync` REPORTS `4 need a decision` ON EVERY RUN, AND NOBODY KNOWS WHAT THE FOUR ARE.** `list` shows all 23 `installed canon`. **vc narrowed it and did NOT resolve it: it is NOT extra installed skills and NOT orphans** -- `~/.claude/skills/` holds exactly 23 and the set difference against canon is empty both ways. **So it is a different population from the one `list` renders.** Seeing the message needs a `sync`, which is a WRITE. **ic flagged it and refused to guess; vc removed two candidates and did not improve on that.** **A standing count with no visible subject is the shape this whole day was spent catching.**
6. **`0218` -- `uninstall` LEAVES AN EMPTY DIRECTORY**, so a directory count disagrees with a skill count. hv killed the two residues; the tool is unchanged. ic filed at LOW.

### Open, lower rank

7. **`0216`'s PROVOCATION ARM (cc)** -- see `9a`. **It needs nothing hv has to grant**, but it must be FENCED: scratch entity, peers quiet and told, repair loop armed. **Provoking an ingest is firing the loss mechanism on purpose on a live tree.**
8. **A SECOND CENSUS, OFFERED BY dc AND NOT COMMISSIONED: 57 of 138 `v2:` entries declare NO `evidence_class`.** Larger gap than the collision class. **dc wants a positive control before believing any number it produces, and that condition is ratified in advance.**
9. **CANDIDATE, NOT FILED: a stale-heartbeat guard** comparing `heartbeat_at` against the board file's own last-commit time -- **about the ORDER of the two, never a GAP** (dc), or it fires on everybody mid-session.
10. **THE DAEMON-LOCK RACE STILL HAS NO ISSUE AND WANTS A RECORD** (cc's).
11. **`AC-10.5`'s RE-DISPOSITION IS RATIFIED AND UNSTARTED (cc's).** _Modelled in ST0069, not in this release._ **Second in cc's order, ahead of the `0216` arm** -- it should not queue behind work that waits on a window.
12. **`intent_agents.bats` STAYS `pending`** -- its failures are v2 WORDING, which the narrowed contract excludes. **If it is to go green the path is `wp_commands`': assert the CONTRACT, not the string.** Separate work; does not ride `0203`.

### Standing, and not vc's to unblock

13. **`AT-00.2`, `AC-00.8` AND `AC-00.10` SIT BEHIND cc's `WP-10`, WHICH IS BEHIND `WP-06` (cc's) AND `WP-07` (dc's).** Read off the cover; sequence around it.
14. **`AC-07.7` IS dc's GENUINE HOLD AND NOBODY HERE CAN UNBLOCK IT** -- it needs a keg built from fixed code, which needs a published tag.
15. **`WP-08` STAYS HELD.** 12/12 green with `wp done` an `XS` away, **and closing it marks the daemon DONE over zero conformance coverage.** cc's call, endorsed, waiting on a WORD rather than on work.
16. **`AC-02.3`'s fixture decline SURVIVES the scarcity strike**, **dc's `bin/` prune survives** on irreversibility plus the CI failure mode, and **`WP-13`/`WP-16` STAY IN ST0069 on hv's own dated sequencing sentence.**
17. **THE ACCEPTANCE GATE STILL CANNOT GO RED FROM CODE CHANGING.** baize-vc broke the test `AT-02.1` covers with a prediction written first and the gate returned all-pass; **`status: green` is a STORED STRING and the `at` family has no verify verb.** **`0207` now REFUSES a `--note` that would drop the existing one, so the free half is a GUARD rather than a discipline.** The verify half is unbuilt.
18. **`WP-14` RULING WITHDRAWN.** All 12 ACs were **descoped to ST0069 by hv 2026-08-30, `by: "hv via vc"` -- vc ruled against a descope vc relayed.** `AC-09.5`'s wip/boards half goes with it.

**Re-run the verbs; do not read figures off this board.**

## Standing directives from hv

- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate. **And PFIC means _Pure Function, Impure Coordination_ -- deterministic core, I/O at the boundary. NOT the idiom gloss six documents carried until 2026-09-02.**
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**
- **EVERY PROJECT GETS THE WRAPUP AS ITS OWN TECHNOTE** (2026-09-01). Sequence: pristine -> devbin-vc FIRST -> hv drives the devbin rollout while every other estate chills -> only then do the rust-using estates hear about it.

## Watch-outs

### 9s. `intent/llm/` IS WRITTEN ONCE AT INIT AND OWNED BY NO REFRESH VERB, IN EVERY ESTATE

**DRIVEN HERE, REPORT-ONLY, TREE UNCHANGED.** `intent claude upgrade` names its whole canon set: `.claude/settings.json`, `CLAUDE.md`, `AGENTS.md`, `usage-rules.md` (if absent), `.intent_critic.yml` (if absent), two githooks. **`intent/llm/` is not in it.** And `intent lang show rust` says `lang init` _declares the language in config.json and installs nothing into the project_ -- `rules.rs:67` states the retirement in code. **So `intent/llm/RULES.md`, any `RULES-<lang>.md` beside it, `MODULES.md` and `DECISION_TREE.md` are laid down once and never refreshed by anything.** Not a v2-residue edge case: the whole directory, every estate, this one included.

**THE COST IS NOT THE STALENESS, IT IS THAT THREE OUTCOMES COLLAPSE INTO ONE OUTPUT** (dc's framing, and the reason it earned a paragraph in the release rather than a clause): **a false positive, a fork a verb repairs, and a fork no verb owns all read to a reader as _the remediation is not working_.** A remediation that is correct for the files it names and SILENT about a class it cannot reach produces a reader who runs both verbs, re-greps, still sees hits, and has no way to tell which of the three they are holding.

**WORKED CASE -- THE PFIC FORK, AND IT COST FOUR OF FIVE HOMES IN A CONSUMER.** Lamplight carries five forked homes; **`intent claude upgrade --apply` reaches exactly one** (`AGENTS.md:131`). The other four are `intent/llm/RULES-{rust,lua,shell,swift}.md` -- hand edit or nothing. **Their estate has the TRUE rule in a log nobody reads** (`202605-done.md:3740`, a `DateTime.utc_now()` mid-pipeline finding **no idiom gloss could have produced**) **and the WRONG one in four files agents load at boot.**

**AND FOUR CANON TEMPLATES STILL CARRY THE FORK HERE:** `intent/plugins/agents/templates/{rust,lua,shell,swift}/RULES.md`, glossing PFIC as iterator chains / table destructuring / `case` branching / `switch`, **none naming I/O placement, determinism or the boundary**. `_default` is fixed. **Dead in v3 -- the consuming mechanism is retired -- which is exactly the argument that lets defective canon sit until somebody revives it intact.** ic's (`0219`); dc recommends fixing, four one-line edits; **vc concurs: low severity is not zero and the fix costs four lines.**

**A THEORY OF MINE KILLED BY ONE COMMAND, RECORDED AS A NEAR-MISS RATHER THAN A TIDY RESULT.** _The remediation re-installs the fork for four languages_ was half-written when `intent lang show rust` refuted it. **Third time in one morning a bad escalation was stopped by driving the disproof instead of writing the paragraph** -- and dc's version of the same defect had ALREADY SHIPPED: they held the four templates in their own opening grep, then took `six` out of ic's commit message and never reconciled the two. **A figure transcribed from a NARRATIVE into a shipped artefact, inside a release note whose subject is a claim that drifted from its owner.** The count is now DELETED rather than corrected -- ten would rot as six did -- because **a count in a release note is a claim about a corpus at a moment, and the reader's instrument is the grep, not the author's arithmetic.**

**AND THE DISCRIMINATOR NEEDED TWO ROUNDS, WHICH IS THE INSTRUMENT LESSON.** vc found the repaired file trips its own instruction (it QUOTES the gloss to explain the repair); **dc found a second innocent shape vc could not see -- `in-standards:61`, a red-flag row that was never defective.** The rule that covers both is about what a line CLAIMS TO BE: **a hit is the fork when it purports to say what the rule IS.** dc's control had been run against the BROKEN corpus only, which **cannot exhibit a false positive on the fix by construction** -- so the cure was not the positive control, it was controlling on BOTH populations.

### 9r. A GATE VERDICT READ FROM THE WORKING TREE IS NOT A PROPERTY OF A REVISION -- AND THE SIGN FLIP IS WHAT MAKES IT DANGEROUS

**`intent at lint` READS THE WORKING TREE, SO `ac gate`'s CONTRACT ARM CAN GO GREEN ON BYTES IN NOBODY'S GIT, AND NOTHING IN THE OUTPUT SAYS WHICH TREE IT READ.** Measured 2026-09-03 07:51Z: `gate: ST0056 118/142` and `lint: ok -- 173 conform` both read clean while `one_declaration_two_realisers.rs` was still uncommitted -- `git log` on that path ended at `b67a9bc7` and the citation existed only on disk. **A `git stash` or a checkout of that one path would have put the gate back to refusing.**

**vc HAD THE TWO CLEAN READINGS AND WAS COMPOSING THEM FOR hv.** What stopped it was `git status`, run for an unrelated reason. **Nothing in either verdict would ever have said so.**

**THE SIGN FLIP IS THE WHOLE FINDING.** `8ab+8ae` says _in a shared checkout, one node's ordinary in-progress state is another node's OUTAGE_ -- **and an outage gets investigated.** This is the same seam inverted: **one node's in-progress state is another node's GREEN, and a green gets RECORDED.** The failure direction that produces work has a natural corrective; the one that produces a durable figure has none. **RULE: before a gate figure goes anywhere durable, confirm the paths it read are COMMITTED.** Resolved at `e95ebcf0` -- verified here: HEAD carries the citation, `git status -- native/rust` is empty, and the verdict is a property of a revision again.

**AND ITS COMPANION, cc's, SELF-REPORTED: AN MTIME REPORTED AS AN ELAPSED TIME.** cc told hv a lock was _held since 08:50 local_; the real hold was about a minute. **`ls -la` answers _when was this last written_ and never _how long has this been held_.** Same family as the stamp rule and a NEW member of it: the existing entries are about a value carrying the wrong ZONE, this one carries the wrong QUESTION. **A time out of a tool answers that tool's question, and the sentence you put it in is where the substitution happens.**

### 9p. FOUR TOOL DEFECTS FOUND FROM OUTSIDE THE ESTATE (lamplight-vc), THREE CONFIRMED HERE

**A CONSUMER ESTATE IS AN INSTRUMENT THIS ONE DOES NOT HAVE.** All four arrived measured against `intent 3.0.0 (1c80a627)`, the build we ship.

- **`acceptance: exempt` HAS NO SETTER AND THE REFUSAL NAMES IT AS THE REMEDY.** `st` has 18 subcommands, `ac` has 11, neither carries `exempt`; no clap arg in `intent-cli/src/` spells it; `AcceptanceMode::Exempt` is reachable only by direct model mutation in tests. **And `acceptance_surface.rs:231` ASSERTS the refusal string**, so the dead end is pinned as correct behaviour by a test. **`intent#0145`'s vacuous-remedy class with a test holding the door shut.** BLOCKING three WPs in Lamplight.
- **A GREEN GATE OVER ROWS THE ENUMERATOR CANNOT READ.** Their `legacy.raw` bucket does NOT reproduce here. Ours: **406 test rows, 57 without `file`, of which 45 carry `prose`** (the declared non-test form) **and 12 carry NEITHER** -- 45+12=57, 57+349=406. **The claim generalises past their bucket and is the SECOND HALF of our own standing gate item:** ours cannot go red from code changing, and theirs shows it also passes silently over rows it structurally cannot see. **An enumerator's scope is part of its claim.**
- **THE FOUR-FIELDS GAP CONFIRMED IDENTICALLY: 505 criteria, ONE key-set, `['id','kind','state','text']`, reconstructing 505 = 505; `on` / `control` / `red-first` / `evidence` ZERO each.** So any ruling of the form _record X as evidence on a criterion_ has no slot to land in. **They retracted their own earlier `830 of 2616` figure as unreproducible; recorded as retracted, not dropped.**
- **THE SHELL CRITIC SCOPES ON A DIRECTORY, AND IT IS FIVE RULES NOT ONE.** `applies_to` carries a bare `"bin/*"` with no extension constraint, so a `.md` in any `bin/` matches -- `quote-expansions` (CRITICAL), `no-silent-exit-codes` (CRITICAL), `set-euo-pipefail`, `no-parse-ls`, `module-highlander`. **Canon, so the whole fleet inherits it.** The fix is not deleting the glob -- it arms extensionless scripts like `bin/intent` -- it wants an extension exclusion or a shebang test. **AND THE RUST PACK ALREADY WROTE THE CONFESSION: `error-types-thiserror-anyhow/RULE.md:68` states its own over-reach as measured. One pack states the cost, its neighbour does not.**

### 9q. A `cd` FROM AN EARLIER CALL VOIDED TWO PROBES AND BOTH RETURNED CLEAN ZEROS

**`cd` persists between Bash calls (8a), and a probe run from the wrong root does not error -- it AGREES WITH YOU.** Verifying lamplight's four, two probes returned `0` and `AC rows examined: 0`; both would have read as CONFIRMING the reporter. **A wrong-cwd probe and a true negative are the same bytes.** Caught only by positive-controlling on a row known to exist (`AC-17.1` -> `$.criteria[151]`). **lamplight-vc hit the identical shape the same morning on `0133` -- _my first grep returned nothing on a wrong path and I nearly read it as clean_ -- so this is two estates, one hour, one class.** **DETECTION: a probe whose failure mode is SILENCE must fire on a known positive before its zero is quoted.**

### 9m. A MENU PUT TO hv IS A CLAIM ABOUT THE OPTION SET, AND THE OPTION SET IS THE PART THAT EXPIRES

**`AC-17.1` went to hv as _reword or descope, both weak, one of them moves_. Neither moved. cc built a THIRD door while the menu sat unread**, and the menu was still the top item on `intent/wip.md`, on this board and in ic's hands this morning. **`Op` is now SIX variants -- `ThreadList`, `Graphql`, `Set`, `Registry`, `Subscribe`, `Shutdown` -- and `Set` is the write door.** `EmptyMutation` still ships, so **hv's reads-only GraphQL ruling was never in tension with the row: the mutation went on the WIRE, not into the schema.** The two things the estate spent a week calling incompatible were never competing for the same seam.

**AND THIS IS THE SECOND TIME IN TWO DAYS.** cc's own discharge notice [1] says it in cc's words about `sync`: _I put it to you as a scope question with no cheap middle... the code took a third option I said did not exist._ **Same author class, same shape, and the first one cc caught and this one nobody did.**

**DETECTION, AND IT IS NOT _RE-DRIVE THE RECOMMENDATION_.** A menu's recommendation and its grounds both get re-read, because they are the argued part. **The OPTION SET is the unargued part -- it reads as the statement of the problem rather than as a finding -- so it is the half that ages silently.** Before putting a menu in front of hv, or acting on one already there, drive the EXHAUSTIVENESS claim: ask what would have to exist for a third option, then look for it. **`grep` the enum, not the prose about the enum.**

**AND THE GREEN WAS REAL, WHICH IS NOT THE USUAL FINDING HERE.** `at` has no verify verb and `AtStatus::Green` is a stored string, so vc RAN it: `cargo test -p intentd --test suite one_declaration_two_realisers` -> **2 passed**, and the second is `the_diff_can_actually_fail`, a working negative control. **A green that survives being driven is worth recording precisely because this board mostly records the other outcome.**

### 9o. AN INSTRUMENT WITH THE HUMILITY AND NOT THE POPULATION LINE -- `currency.lib`'s REACH

**`_rust_source_changed` diffs `"$base..HEAD" -- native/rust` AND NOTHING ELSE. `intent/wip.md` documents the WIDER check** -- `git rev-list -1 HEAD -- native/rust surface docs/design`, three paths. **Three `include_str!` sites reach outside the arm**: `intent-cli/src/dispatch.rs:45` -> `surface/dispatch-table.json`, `intentsvcs/src/form.rs:79` -> `surface/forms.json`, `intentd/src/web.rs:57` -> `docs/design/intent-logo.svg`. `surface/` is at the REPO ROOT, measured (`ls -d native/rust/surface` -> No such file). **So a commit to `surface/dispatch-table.json` leaves the verdict `ok` while the binary carries stale compiled-in bytes.**

**NOT FIRING TODAY AND THAT IS THE PART TO STATE: the arm agrees with the truth BY LUCK.** `1c80a627..HEAD` is 63 commits; non-test under `native/rust` = 0; under `surface` + `docs/design` = 0. Both readings say current.

**CORRECTED 2026-09-03 (cc caught it, vc drove it): THE REACH _IS_ STATED, IN A FOURTH HOME, BY A DIFFERENT FILE.** `artefact_currency_verdict`'s success path is `printf 'ok'` -- bare (`currency.lib:181`). The sentence a reader sees -- _no non-test file under native/rust has changed_ -- lives at **`self_provenance_check.sh:388`**, a CALLER, hardcoding the scope in its own prose. **So the implementation and the claim about it are two hand-kept copies agreeing by coincidence: move the pathspec and the sentence keeps saying the old scope, correctly formatted, with nothing red.**

**FOUR HOMES FOR _WHAT IS A BUILD INPUT_, AND ARM 6 BINDS ONLY THE FIRST TWO:** `source_commit.rs:174` (3 paths, marker identity AND dirt), `sharedtarget.lib:124` (3 paths), `currency.lib:108` (1 path, the diff), `self_provenance_check.sh:388` (1 path, the prose). **The marker derives its BASE over three paths and currency diffs ONE against it -- not a narrow guard, a comparison whose two sides answer different questions** (cc's statement, verified here).

**THE EMBED CLASS IS RECOGNISED, ARMED AND CONTROLLED -- ON THE DIRT SIDE ONLY.** `shared_artefact_build_guard.sh:294-320` (arm 6b) enumerates every embed climbing out of `native/rust` and asserts scope coverage; **its control is real and was read: finding NO outside embeds is a FAIL**, on the stated ground that `dispatch.rs` is known to carry one. It states its own residual and names the safe failure direction. **Arm 6b checks SCOPE COVERAGE and never CURRENCY.** So a COMMITTED `surface/` edit passes every guard green while the binary carries stale bytes -- **and an UNCOMMITTED one is caught. The direction is inverted: committing is what makes a change shared, and it is the act the guard stops seeing.**

**ROUTING, dc's, AND IT IS THE REUSABLE PART: `bin/.devbin/lib/` IS VENDORED AND `bin/.devbin/cmd/` IS INTENT'S OWN.** One directory, two ownerships; `currency.lib` exists nowhere in `~/Devel/prj/devbin`. **A clobbered-on-upgrade rule applied to the whole path is wrong for half of it -- exactly the belief that makes someone decline to fix a file they own.** The row dc is filing: bind currency's population to `DIRT_SCOPE` as the dirt arm's already is, and make whatever binds the diff also supply the sentence, or the prose becomes home five.

**THE FILE SPENDS THREE PARAGRAPHS REFUSING TO OVERCLAIM ITS DISTANCE AND NONE BINDING ITS SCOPE** -- the `dirty-` floor note, the `grep`-exit-status note, the fail-open note. **A careful instrument is not a scoped one, and care is what stops anyone looking.**

**AND THE CLASS THE EXCHANGE ACTUALLY PRODUCED, cc's OWN WORDS, EARNED THREE TIMES IN ONE HOUR BETWEEN US: I ASSERTED WHAT AN INSTRUMENT COVERS WITHOUT OPENING IT.** cc twice (the population claim, then _nothing else covers this_ -- refuted by cc's own commit output eight minutes later), vc once (the committed-range claim). **In every case the disproof was already in hand or one command away.** cc's discharge of the `include_str!` lag was CORRECTLY GROUNDED and it is worth recording why: they drove the three-path command by hand and drove `at edit --help`, and neither depends on the population claim they bolted on. **The instance was genuinely closed; what was wrong was certifying an instrument they had not read** -- so the cost is not a withdrawn escalation, it is that the class was never visible to the thing meant to show it.

### 9n. A RELEASE NOTE IS AN INSTRUMENT WHOSE POPULATION IS THE TREE AT THE MOMENT OF WRITING (dc)

**`docs/releases/3.0.1/RELEASE_NOTES.md:51` says _No migration is required and no project data changes_, and it is FALSE.** Verified here rather than taken from dc: `v3.0.0` (`80d8b2ca`) reads `SCHEMA_VERSION: i32 = 13`, HEAD reads **17**, and `store.rs:1796` dispatches `v if v < SCHEMA_VERSION => Self::migrate(...)` -- **an older store is migrated, not refused.** The line was TRUE when written on 2026-08-29 and went false when the schema moved. **Nothing re-read it, and nothing could: a prose claim about code sits under no guard.**

**THE SENTENCE'S SECOND CLAUSE IS THE DANGEROUS HALF BECAUSE IT IS TRUE.** _A project already readable by v3.0.0 is readable by v3.0.1_ holds -- v3.0.1 reads a v13 store, by migrating it. **The converse is what breaks and the sentence does not mention it: `store.rs:1801` refuses a NEWER store outright, in its own words _there is no rung that could help... move the TOOL forward, never the data back_.** So the upgrade is a ONE-WAY DOOR and the note's own words tell a reader it is not a door at all. **A reader who CHECKS the stated direction confirms it and walks through anyway** -- which is worse than a plain falsehood, because verification rewards the wrong conclusion.

**THE RUNG COUNT BETWEEN 13 AND 17 IS NOT MEASURED AND IS NOT CLAIMED HERE** (dc's discipline, kept). **Local exposure is nil and that is deliberately not the finding** -- the obligation is to people who are not on this box.

**9-series is 2026-09-02's, folded into families. The 8-series below is older and already merged; full narrative for anything dated 2026-08-31 or earlier is in `.history/`.**

### 9a. `0216` IS AN ACTIVE LOSS CONDITION AND THE MITIGATION IS A LOOP, NOT A WAIT

**A canon write reports `ok`, LANDS, and `intentd`'s disk ingest REVERTS it ~1s later.** Not a failed write -- a second actor undoing a successful one, which is why the `ok` was honest. **IT FIRED THREE TIMES ON 2026-09-02: `AC-17.17` 18:07 (vc), `AT-00.7` 19:12 (ic), `AT-00.8` 21:34 (vc). Half the losses in the eight-day corpus are from that one session** -- the session spent analysing it.

**THE MITIGATION IS `READ THE TARGET BACK, COMPARE TO INTENT, RE-WRITE IF IT DIFFERS, REPEAT UNTIL IT HOLDS`.** **NOT `verify past the ingest`, which smuggles a constant nobody has** -- past WHICH ingest, and how long is past? With cross-node writes the debouncer keeps resetting, so the delay is bounded by nothing measured. **AND NOT `one verb at a time`, which is INSUFFICIENT: the debouncer sees WRITES, NOT AUTHORS, so a peer writing 2s before you puts YOUR write in the exposed position and you cannot see it in your own transcript.**

**THE ARM WANTS THE DAEMON RUNNING, WHICH INVERTS THE OLD ASK.** The gate is whether an ingest FIRES; 284 of 299 logged ingests were disk-edit triggered, so an ordinary disk touch arms it. **Fenced: scratch entity, peers told, repair loop armed** -- otherwise the experiment is indistinguishable from the defect.

**WHAT IS KNOWN AND WHAT IS NOT: the phenomenon is PROVEN, the population is COMPLETE, four readings are KILLED, and the MECHANISM IS UNKNOWN.** Killed: the flush-window (lost and survived gaps interleave completely), coalescing (no recorded loss happened inside one quiet period), writes-during-a-running-ingest (ingests essentially never overlap -- 252 of 298 gaps over a minute), and concurrency-manufactures-depth. **Count correlates -- depth 1 lost 0 of 5, depth 3 lost 5 of 5 -- and nothing measured explains why the LAST write.**

**AND IT IS REACHABLE BY AN ORDINARY SHELL LOOP, SO THE FIX CANNOT BE DISCIPLINE.** An `intent` invocation costs **23-26ms**; the 250ms floor observed all evening is **AGENT TURN LATENCY**, a property of who is driving. A shell loop over a shipped verb sits deep inside the debounce, and the 2026-08-18 burst was exactly that. **The fix is daemon-side: an ingest must not revert store state newer than the disk it read** -- and that collides with this board's own provenance finding, because `written_at` is the field the ingest REWRITES WHOLESALE, so it needs a monotonic version the ingest does not own. **MIGRATORS ARE SAFE and structurally so** (`migrate.rs`: the uncommitted return IS the atomicity, `AC-10.2`), **and the test suites are safe** -- watched iff the daemon opened it, and the suites never pass `--daemon`.

### 9b. THE BASE RATE IS NOT THE CONDITIONAL, AND FOUR MEASUREMENTS WENT TO THE WRONG ONE

**Both of cc's hypotheses and both of vc's measurements asked _how often does the risky shape occur_ when the question that decides a mechanism is _GIVEN it occurs, what happens_.** Tonight's rate of risky bursts is genuinely LOWER than the corpus and **that reassuring proportion was worthless while three writes were being eaten.** **A base rate tells you how worried to be; a conditional tells you what the mechanism IS.** The conditional sat in the same table every time and is nearly deterministic. **The base rate kept being asked because it LOOKED like the safety question.**

### 9c. vc TOLD ic TO STAMP `authority: hv` ON THE STRENGTH OF A RELAY, IN THE MESSAGE EXPLAINING WHY THAT IS THE SHAPE THE ARM PREVENTS

**ic REFUSED and got hv's word first-hand instead.** A relayed ruling stamped hv-first-hand is **indistinguishable in the artefact** from a delegated one wearing hv's name -- which is exactly why `ratified-in` was rebuilt from a regex to declared fields. **vc walked into it while citing it.** **DETECTION: the authority field records WHERE THE WORD CAME FROM, not whether it is true. A true relay is still a relay.**

### 9d. A DISPLAY FILTER IS A CLAIM ABOUT THE POPULATION, AND IT IS THE ONE NOBODY REVIEWS

**vc reported a "complete" sweep off a filtered view TWICE.** First a `d<1800` print filter described as a _repair-shaped interval_, which **dropped `0202`, the only live loss and the sweep's own positive control**; then an `if after` gate that **made every NEVER-REPAIRED candidate invisible -- 16 of 26.** **The query was argued, defended and re-run at four window sizes; both filters sat in the loop that PRINTED it.** So **THE REVIEWED ARTEFACT AND THE REPORTED ARTEFACT WERE DIFFERENT OBJECTS, and every review looked at the one that was right.**

**AND BOTH FILTERS ENCODED THE SAME HIDDEN ASSUMPTION -- THAT A LOSS GETS NOTICED -- WHICH IS THE VARIABLE UNDER STUDY.** A loss repaired in 30 minutes is one somebody noticed; `0202` sat 37.7 hours because nobody did. **A filter that operationalises `real` as `repaired` cannot see the population it was built to find.**

**THE CURE IS CHEAPER THAN A REVIEW PROCESS: MAKE THE SUMMARY RECONSTRUCT, AND SHOW IT RECONSTRUCTING FROM TWO DIRECTIONS.** 16 + 10 = 26 and 20 + 6 = 26, both stated. **A total that is PRINTED hides a miscount; a total shown CLOSING cannot.** Both of vc's summary failures were caught from outside by a reader adding up categories, never by re-running anything -- **and a summary that does not reconstruct is a defect in its own right, because it is the only artefact most readers ever see.**

### 9e. A CLAIM THAT SOUNDS LIKE PHYSICS IS EXEMPT FROM THE STOPWATCH

**_A process spawn costs more than 250ms_ survived FOUR assertions, two of them to peers as a system property, and died to a four-second measurement: 23-26ms.** **It reads as a fact about computers, so nobody measures it** -- where _our writes are 250ms apart_ reads as a fact about us and invites the question. **It was LLM turn latency wearing an operating system's clothes.**

**AND IT IS THE THIRD TIME THE SAME AUTHOR RESTATED A CONTINGENCY AS A PROPERTY** -- flagged as contingent in refinement 1, asserted absolutely one message later, corrected by ic, then re-asserted to cc as _process spawn cost settles it_. **A CONSTRUCTED VARIABLE STOPS BEING LABELLED CONSTRUCTED THE MOMENT IT IS QUOTED, INCLUDING BY ITS AUTHOR.** The label lives in the prose and the number travels alone; cc's crossed a session boundary, vc's crossed two paragraphs, **and the mechanism did not care about the distance.**

### 9f. A SURVIVAL IS NOT EVIDENCE THE DISCIPLINE WORKS WHEN THE MECHANISM WAS NOT ARMED (ic)

**ic's `issues.close 0181` sat 2.3s after a peer's write, in the exposed position, and lived because no ingest fired.** Reporting _verified, discipline holds_ from that would have been **a false confirmation built out of a true observation** -- the vacuous-instrument shape arriving inside a report about it.

**AND ITS INVERSE, WHICH ic NAMED BEFORE IT COST ANYTHING: A MECHANISM CORRECTION IS NOT A REFUTATION OF THE CONCLUSION IT SITS UNDER.** Three times in one evening someone was right about what to DO and wrong about WHY. **Tonight trained everyone to expect a broken premise to take its conclusion with it -- which is why the next one that does not will be mis-read.**

### 9g. A GUESSED FIELD NAME RETURNS `absent` FOR BOTH _NOT THERE_ AND _NOT WHERE I LOOKED_ (ic)

**ic read `c["satisfied"]` out of the extract, got `None`, and had a `0216` loss report half-written against a store saying `yes`.** The real path is `c["state"]["is"]`. **It produced the exact signature of the revert they were primed to find.** **Same class as vc ruling on `WP-14` after querying `text` when the descope lived in `state`** -- so the cure is stated once for both: **walk the objects AND NAME THE FIELD, and before ruling on any row, query its `state`.**

### 9h. A REFUSAL-AND-RETRY IS WHERE A DISCIPLINE GETS DROPPED (ic)

**The retry FEELS like a continuation and is a FRESH COMMAND.** ic used `git commit --only` correctly, was refused on FORMATTING, fixed the format, and **rebuilt the command from scratch as a plain `git commit` -- which took vc's staged canon file into ic's commit.** vc hit the same trap on the clock guard minutes later. **The gate's refusal is what puts you on the unsafe path**, and it generalises past git to every guarded verb here. **RE-ISSUE THE SAME COMMAND, never recompose it.**

### 9i. TWO MORE SECOND-ACTORS, AND THE ACTING NODE'S REPORT WAS TRUE AND INCOMPLETE EVERY TIME

**A plain `git commit` takes the index AS IT STANDS**, so one node's commit absorbed another's staged canon edit -- no data lost, provenance muddled, **corrected FORWARD because the tree is shared and HEAD had moved.** **And a shared gate reads a PEER's live working tree**, so two nodes asking minutes apart get different verdicts with nothing in either result saying so. **vc asserted the mechanism (a guard that cannot vouch for itself) WITHOUT OPENING THE GUARD; devbin-vc drove it and found a real new unguarded `cargo build --release` from that day.** The guard was working precisely. **On a shared tree a write's outcome is not determined by the writer.**

### 9j. THE STORE'S GRAMMAR REFUSES SCHEMES FASTER THAN A PARAGRAPH DEFENDS THEM

**`kind` is a property of the CRITERION, so a non-test row can never satisfy a test-backed AC** -- the store refused vc's _one property, two instruments_ housing after vc had written the full row text justifying it. **Second time in one day a vc scheme met the field's own grammar** (`legacy.rs:1965` killed the three-tier key scheme that morning). **DETECTION: when a ruling invents a HOUSING, drive the write BEFORE writing the reasoning.**

**WITHDRAWN 2026-09-03, MINE: _the pair will still read ok until someone rebuilds_ was wrong.** The arm diffs a COMMITTED range, so a committed source change turns it red; the real gap is REACH (`9o`), not staleness. cc had the mechanism right and vc asserted against it without opening the file -- **the third time this board records asserting a mechanism without opening it (`9i`).**

**AND THE SHARPER HALF: vc CHECKED THE RIGHT STRUCTURE FOR THE WRONG PROPERTY.** Many-to-one coverage is ordinary here (`AC-04.1` carries four), so vc checked the MODEL rather than the DISPLAY and was right to -- **and it did not help, because arity was never the constraint; KIND was.** **A structural check aimed at the wrong field returns a true answer to a question nobody was blocked on, and reads exactly like due diligence.**

### 9k. A PREMISE EXPIRES BETWEEN BEING FORMED AND BEING ACTED ON -- SIX TIMES IN ONE EVENING

`AT-00.6`'s v2-era checksum claim; vc's `WP-14` descope; `config`'s ordering limb; vc's `0181` ruling handed to ic two hours after dc had landed it; ic's own board reporting a cut that was already executed; and `0203`'s `WHAT IT UNBLOCKS` paragraph, **written by dc before the remedy existed and ruled on by vc without re-driving it -- the writing dc's, the not-checking vc's.** **EVERY ONE WAS CAUGHT BY THE PERSON ABOUT TO ACT ON IT, WHICH IS THE CONTROL WORKING RATHER THAN A NEAR MISS.** **AND A RULING WHOSE PREDICATE DISSOLVES WHILE ITS PRINCIPLE HOLDS MUST BE RECORDED AS SUCH, or it reads as a reversal.**

### 9l. AN ASSERTION THAT LIVES IN A COMMENT CANNOT BE CHECKED (dc)

**`wp_commands.bats` asserted usage strings and NOTHING about the exit code, under `# help exits with 1 (usage pattern)`.** **The only row with an opinion about rc carried it where nothing could check it, and it had been false since `245dcdbe` with no symptom.** **`AC-00.2`'s class landing in a TEST FILE -- the last place you would expect an unenforceable claim, which is precisely why it survived.**

### The 8-series -- claim and detection only; every incident narrative is in `.history/`

**MEASUREMENT AND INSTRUMENTS**

- **8t+8ac. A PROCEDURE I AUTHOR IS AN INSTRUMENT, AND AN INSTRUMENT GETS A POSITIVE CONTROL.** Applies to the probe as readily as to the subject.
- **8x+8af. A CONTROL THAT CAN ONLY PASS IS DECORATION, AND THE THING IT CATCHES MAY BE ITS OWN AUTHOR.** A control that would pass under the broken instrument too establishes nothing. **`AC-00.11`'s own founding defect (`EXAMINED 86 of 278`) has N < M and a closing remainder, so NEITHER mechanical arm fires on it** -- true of the ARMS and false of the CRITERION.
- **8o. A CONTROL THAT MOVES A VARIABLE THE INSTRUMENT IGNORES IS GUARANTEED TO REPRODUCE**, and proves nothing about the variable that matters.
- **8u. A FOLD CONTROL THAT COUNTS IDS DETECTS ONLY DELETIONS.** A fold that moves PROSE needs a MECHANISM census beside the id census.
- **8at. AND THE MECHANISM CENSUS HAS ITS OWN BLIND SPOT.** Its first regex required a capital start, so id-prefixed and marker-prefixed headers were invisible and it reported rewordings while whole entries vanished. **A CENSUS NEEDS A PLANTED POSITIVE: delete a known entry, confirm the census names it, restore.**
- **8k. THE PROXY THAT IS WRONG IN THE DIRECTION THAT PRODUCES WORK IS THE ONE NOBODY AUDITS.**
- **8i. A `head -10` TRUNCATION READ AS A COMPLETE POPULATION** almost put a false gap in the record.
- **8y. THE SEAM THAT MAKES A THING PROVABLE ALSO PUTS ONE CLASS OUT OF REACH.**
- **8am. THE ESTATE ITSELF HAS A REACH AND NOTHING OWNS IT** -- `AC-00.16`'s amendment, landed `54ea0cc6`. **Reach is bounded by what the ENVIRONMENT can exhibit, not only by what the instrument checks** -- worked case: in a self-hosted checkout the install root and the project root are one directory, so no drive here can exhibit their difference (`0215`).
- **8ah-bis. A REPORT-ONLY GATE WHOSE OUTPUT NOBODY DIFFS DECAYS INTO DECORATION.**
- **AN INSTRUMENT STATES ITS POPULATION BESIDE ITS VERDICT** -- POPULATION (derived, with the derivation named), FORMS (per-form counts, never a single total), REACH (`COVERS` / `DOES NOT` / `UNOWNED`). **THE CORPUS IS THE CLAIM; THE VERDICT IS NOT.**
- **COMPARE AGAINST SOMETHING INDEPENDENT OF THE THING UNDER TEST.** The disk for a write; the POPULATION for a diff. **A diff of one artefact across a verb cannot tell a REGRESSION from a MIGRATION.**

**PREMISES, STALENESS AND PROVENANCE**

- **8ai+8an+8ap. A PREMISE IS NEVER RE-DRIVEN UNTIL ITS REMEDY, AND IT CARRIES A VERSION.** A claim true of v2 re-drives CLEAN against the v2 line it cites, **so only a re-drive that checks the SUBJECT rather than the CITATION can find it.**
- **8b. I RULE AGAINST STATES THAT HAVE ALREADY MOVED.** Re-measure before ruling; when a premise dissolves, say which.
- **8g+8g-bis. A CLAIM ABOUT A FILE IS A CLAIM ABOUT A REVISION** and goes stale exactly like a claim about a tool. **The corrective is not more care at the moment of writing.**
- **8s+8w+8z+8ah. THE PROVENANCE OF A CLAIM IS NOT RECOVERABLE FROM THE CLAIM.**
- **8f. I STAMPED A PEN NOTE `driven` FOR A CLAIM I HAD NOT DRIVEN, AND THE MARKER IS THE WHOLE VALUE.**
- **8q. A RULING RECORDED IN A FOLD CARRIES A CRITERION'S AUTHORITY AND GETS NONE OF ITS REVIEW.**
- **8p. A STALE FIGURE MISINFORMS; A STALE TODO STOPS WORK, AND NOBODY RE-READS A TASK THEY HAVE NOT STARTED.** An untouched TODO earns a re-drive precisely BECAUSE it is untouched.
- **8e. AN OPEN ESCALATION AGES INTO AN ASSERTION.**
- **8h. A DOCUMENT APPLIES ITS OWN RULE TO CLAIMS ABOUT OTHERS AND NOT TO CLAIMS ABOUT ITSELF.**
- **8j. A TRUE FACT FROM AN ADJACENT CONVERSATION, HANDED TO A DOCUMENT THAT MAKES NO SUCH CLAIM**, arrives as evidence and is not.
- **8n. A WRONG MECHANISM PRODUCING THE RIGHT SYMPTOM HAS NO NATURAL CORRECTIVE.**
- **8d. A PHRASE OF MINE BECOMES A SPEC THE MOMENT SOMEONE BUILDS AGAINST IT.**
- **8ao. A STALE HEARTBEAT IS INVISIBLE TO EVERY GUARD, BECAUSE A HEADER NOBODY EDITS ADDS NOTHING.** vc's was ELEVEN HOURS stale while two peers read `status: active` at face value with no means to check. **A heartbeat is a CLAIM ABOUT LIVENESS and a stale one is indistinguishable from a current one by inspection.**

**CLOCKS AND SHELLS**

- **8h-ter. A STAMP TYPED IN THE SAME BREATH AS THE READ IS STILL TYPED BY FEEL.** `16:20Z` into a note whose turn read `16:17Z`; `21:58Z` rounded up off a `21:57:36Z` read, refused by the clock guard. **A clock value goes in only when the `date -u` that produced it is in this turn's output, verbatim.**
- **8h-bis. FOR A WORD-LEVEL CLAIM, COMPARE WORDS** (conflab-vc via devbin-vc).
- **8a. SHELL QUOTING EATS CONTENT AND THE COMMAND STILL SUCCEEDS.** Under zsh an unquoted glob ABORTS the whole call, an apostrophe inside single quotes runs NOTHING, and `cd` persists into the next call.
- **8r. A SUCCESS CHECK THAT PIPES REPORTS THE PIPE'S STATUS.** `if cmd | tail -2; then echo COMMITTED` prints COMMITTED whether or not the commit landed. **Capture the status before anything consumes it: `out=$(cmd 2>&1); rc=$?`. Anything downstream of the thing being tested becomes the thing being tested.**
- **8al+8aq. AN INSTRUMENT READS THE HARNESS, AND A SHELL REFUSAL ARRIVES AS AN ANSWER.**
- **8as. A `debug_assert` IS LOUD IN THE PROFILE NOBODY SHIPS AND SILENT IN THE PROFILE EVERYBODY RUNS.** **The profile is part of the instrument, and the two disagree in the direction that HIDES the defect.**

**SHARED TREE AND SCOPE**

- **8ab+8ae. IN A SHARED CHECKOUT, ONE NODE'S ORDINARY IN-PROGRESS STATE IS ANOTHER NODE'S OUTAGE.**
- **8l. ABSENT BINARIES ARE NOT AN OUTAGE UNTIL YOU HAVE ASKED WHETHER A BUILD IS RUNNING.**
- **8aa+8ad. A WINDOW IS SCOPED BY THE EXTRACT'S UNION, NOT BY THE VERB'S NAME** (`0210`).
- **8v. A CANON ATTACHMENT THAT IS ALSO A FORMATTED MARKDOWN FILE HAS TWO WRITERS AND THEY RUN AT DIFFERENT TIMES.**
- **8m. A GATE'S PROVENANCE LINE IS WRITTEN AFTER ITS LOG IS SEALED**, so it lands in a parent's record.
- **8aj. _NOTHING QUEUED FOR YOU_ IS A CLAIM ABOUT MY QUEUE, NOT THEIRS.**
- **8c+8c-bis. I MEASURE SOMETHING TRUE AND RULE ON SOMETHING WIDER.** And `--note` REPLACES a row's note wholesale and silently (`0207`, now guarded) -- **7803 characters destroyed before the refusal existed.**
- **8ak. A DEFAULT METHOD ON A TRAIT WITH FORWARDING IMPLEMENTATIONS IS A TRAP**, and the trap is in the forwarders.

## Decisions

**Standing rulings. Every entry was EXECUTED before it was archived; an UNEXECUTED ruling never leaves this board.**

- **A REFUSAL ADDED TO A SURFACE WITH NO INVERSE IS A ONE-WAY DOOR** (2026-09-01, on dc's drive). **`kind` is settable at MINT on both `ac` and `at` and changeable NOWHERE** -- `at edit` has no `--kind`, `ac edit` says _leaving its kind alone_ in its own help, `at new` refuses a taken id, and the `at` family has **no removal path**. **So `0146`'s fix (2) IS A REGRESSION IF IT LANDS ALONE**: on a mis-kinded row it closes the LAST door and freezes it permanently. **This is the INVERSE of _one field short_ and it is worse** -- that class under-reaches; this over-reaches into the escape hatch, wearing the clothes of hardening. **DETECTION: before adding a refusal, ask what verb undoes the state it will now trap. If none, you are not hardening the surface, you are welding it.** **RULED: fix (2) lands WITH `AC-04.6` or not at all; fix (1) is safe alone.**
- **`AT-07.7` STAYS RED AND INCONSISTENT; THE `na` EXIT IS REFUSED.** It would record _n/a -- nothing ran_ about a row whose instrument DEMONSTRABLY RUNS two-sided, **trading a TRUE inconsistency for a FALSE statement** -- and `na` reads as resolved. **The `doctor` finding IS the evidence for `AC-04.6`**, so the na exit makes the finding vanish while the gap stays open.
- **`AC-12.1` IS RED AND THE CRITERION IS NOT REWORDED** (2026-09-01). It says _nothing in the repo EXECUTES or EMITS a `bin/` intent script path_; **`bin/intent` stands with 26 v2 scripts, and the test estate is the largest executor** -- `EMITS` 114, `EXECUTES` 85 by two routes. **AT-12.1's evidence named two plugin directories: every word true, corpus narrower than the criterion it scored** -- and I took that row off dc to avoid a self-scored one, then scored it against the wrong population. **Rewording to what the current state satisfies is barred by `AC-06.3`'s own rule.**
- **AN INSTRUMENT STATES ITS POPULATION BESIDE ITS VERDICT -- ONE SHAPE, THREE PARTS, IN THE OUTPUT.** **POPULATION** (the set EXAMINED, derived, with the derivation named; where it cannot be derived, `RECORDED` sits AT the number with what would derive it). **FORMS** (per-form counts where the subject has more than one shape, never a single total). **REACH** (`COVERS` / `DOES NOT` / `UNOWNED`). Built on the existing convention -- 25 of 69 instruments already emit a REACH block. **THE CORPUS IS THE CLAIM; THE VERDICT IS NOT.**
- **THE `0206` CAS: (a) A `revision` COLUMN, REFUSE-AND-NAME, INSIDE `commit_mutation`'s TRANSACTION.** Not struct comparison -- **(b) makes the fix a member of the class it fixes**, because a struct equality is a HAND-MAINTAINED POPULATION that fails OPEN on every field added after it. **A counter enumerates nothing and cannot be too narrow.** Not retry: **the defect is the SILENCE, and a retry that succeeds quietly reproduces it through a different mechanism** -- a fix must not be observationally identical to the defect. A compare before the transaction narrows the window and does not close it; **compare-then-hope does not ship.**
- **0206's FRAME IS TOO NARROW AND I FOUND IT IN MY OWN HANDS.** `ac new` returned `rc=0`, `ok: created`, and the row was in NEITHER store NOR canon; the next verb in the same shell could not see it. **ONE node, ONE shell, sequential verbs, no peer -- with intentd running.** Retried in isolation, it persisted. **The criterion, the harness and hv's risk framing all describe two concurrent nodes; nobody has measured one process writing quickly with the daemon up, which is the configuration we are in all day.**
- **COMPARE AGAINST SOMETHING INDEPENDENT OF THE THING UNDER TEST.** The disk for a write; the POPULATION for a diff. **A diff of one artefact across a verb cannot tell a REGRESSION from a MIGRATION.** The general form of every 8c error.
- **THE ESTATE DOCUMENTS ITS MECHANISMS CORRECTLY AND APPLIES THE FIX ONE FIELD SHORT.** **Detection: ask which CALL SITES the remedy reaches.**
- **A RECLASSIFICATION IS A CLAIM ABOUT WHY; A CLASS CHANGE WITHOUT ITS REASON IS A DELETION WEARING A NEW LABEL.** `deviate` RETARGETS never deletes; `retire` CARRIES ITS BASIS.
- **UNWIRED IS NOT RETIREMENT.** `125f601d` deleted the v2 PLUGIN SCRIPTS, not the commands; `prime`/`subagents` return _a known command that is not implemented_. `surface/dispatch-table.json` disposes every `claude` verb `keep` and the register shares that vocabulary VERBATIM, so a register `retire` against a table `keep` is two artefacts contradicting each other. **Nothing cross-checks them -- `0204`.**
- **`D10` RATIFIES THE PLUGIN-SCRIPT PRUNE. TRACED, NOT PICKED** -- a shared operative clause word for word, plus a commit naming the AC. **Resemblance is not evidence.** Minting a NEW D-number is a design act and hv's; ruling that an existing one covers a case is adjudication and mine.
- **v2 MESSAGE STRINGS ARE OUTSIDE THE NARROWED PARITY CONTRACT.** `parity.md` names stderr VOICE, not wording, and cites `0023` -- the ruling that RETIRED the capitalised voice -- so it cannot also bind the pre-change literals.
- **ST0064 PROJECT ROOT: (a) -- the app stores a configured root and sets the child's CWD.** `D07` RATIFIES A REGISTRY and it is unbuilt, so the ground is SEQUENCING not novelty. **CONDITIONS: validate and refuse LOUDLY; mark the store IN THE CODE as INTERIM.**
- **ANY CANON COMMIT HERE IS SILENTLY A MULTI-NODE COMMIT.** Canon regenerates wholesale and cannot be split. **Whether it is DECLARED depends on whether the committer looks.**
- **THE MCP RULESET.** MCP tools call the FACADE, never the CLI dispatch arm. **The test for exposure is NEED, not provenance.** `severity` exposed (enumerated domain); `title` withheld (authored text).
- **`AC-06.3` IS REWORDED, NOT WITHDRAWN, AND THE NEW FORM IS HARDER.** **Every KNOWN deviation is recorded -- a `keep`/`as-observed` row found to differ is a recorded deviation or a filed defect, never silence.** **FILING IS RECORDING; a deferred filing is silence wearing a schedule.**
- **AN ISSUE'S AUTHOR DISPOSITIONS IT** -- and that is why ic wrote `0210` rather than me. **A title's job is not to state the final cause, it is to NOT MISDIRECT.**
- **A CRITERION THAT COULD FORCE SCOPE IS WRITTEN AS AGREEMENT, NOT COVERAGE.** ST0057's four new rows bind that a decision is TAKEN and BUILT without picking it.
- **A DELETE HAS THREE POPULATIONS AND EVERYONE ASKS ONLY THE FIRST:** what EXECUTES this; what CITES this as evidence -- **population is the STORE, not the tree**; what CHECKS this.
- **AN IDENTIFIER IS ONLY UNIQUE WITHIN ITS SCOPE, AND EVERY BOARD WRITES IT BARE.**
- **A SECOND HOME IS NEVER ACCEPTABLE AT A TAG.** The escape is not _accept two homes_, it is _need less machinery_.
- **A CITATION'S AUTHORITY COMES FROM ITS MEMBERSHIP RULE, NEVER FROM ITS NAME.** **DERIVED CENSUSES MULTIPLY FREELY; AUTHORITATIVE COPIES DO NOT.**
- **A TEST GOING RED BECAUSE A FIX LANDED IS THE NOTIFICATION WORKING.**
- **THE DAEMON'S PUBLISHED PORT SERVES BOTH PROTOCOLS, DISAMBIGUATED AT BYTE 0.**
- **THE MANIFEST IS A SHARED MEASUREMENT SURFACE, NOT A SINGLE-WRITER FILE.** Rows come from whoever drove the fact; the door is `st attach`.
- **`close --note` IS NOT BUILT.** `issues edit --from` then `issues close` IS that act. **I originally wrote that it ALREADY SHIPS and that was false.**
- **A CLONE AT A PINNED REVISION IS `FOR REAL`.** `for real` opposes SIMULATED, not CLONED.
