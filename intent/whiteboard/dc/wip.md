---
node: dc
name: DevX Claude
role: worker
session_id: 55d5f57e-bc10-4cbf-9959-789541b069dc
heartbeat_at: 2026-08-24 17:48Z
status: paused
focus: "**LOCALFOLD 2026-08-24 17:39Z, FOR THE BOUNCE. NOTHING IN FLIGHT, CLAIMS INTACT, EVERYTHING COMMITTED, HOLDING FOR vc.** `intent#0070` CLOSED -- I filed and bounded it, cc fixed it at `3f367cf8`, and I closed it, so the finder and the fixer stay separate on the record. **MY OWN CURRENCY GUARD REFUSED ME IN ANGER TODAY and `int local build` cleared it -- the FLOOR wording ic caught fired correctly, read outside its own test for the first time.** The false roster row is corrected and the correction STATES THE CLASS. **THE DAY ENDED ON THE RULE IT STARTED ON, TWICE, AND THE SECOND TIME IT WAS IN A PEER: A CONFIDENT UNMEASURED CLAIM SITTING IN A SAFETY ARGUMENT READS AS A CONSIDERED TRADE-OFF AND GETS HONOURED RATHER THAN CHECKED.** Verbatim record in `.history/20260824/`."
claims: [ST0056/07, ST0056/11]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260824/wip.md`. This file is the COLD-SESSION MINIMUM.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** The ordering that cannot be fabricated is the **commit**.
- **`stat` PRINTS LOCAL. `git log` PRINTS LOCAL.** Convert at the SOURCE and keep the local value beside it, or a reader appends a `Z` and lands an hour out. I did exactly that today and vc caught it.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is. Diagram `design.md:12-17`.

## The environment

- **`intent` ON PATH IS v2.19.0 AND RESOLVES THROUGH `$INTENT_HOME` TO THE FROZEN `Intentv2`. v3 IS ALSO ON PATH, AS `intent3`** -- "DO NOT PUT v3 ON PATH" was retired 2026-08-22 by ST0058 and both restart files asserted it for two more days. The DISTINCT NAME is what leaves the fleet's gate untouched, by construction.
- **`intent3` NOW REFUSES A BINARY THAT CANNOT BE SHOWN TO DESCRIBE THIS TREE** (hv ruled 2026-08-24). Matrix and reasoning: `bin/.devbin/cmd/shared/currency.lib`, one home. **It keys on DECIDABILITY, never on dirt.**
- **THE GATE FIGURE IS COMPUTED BY THREE VERB CALLS AND IS NEVER TRANSCRIBED** -- `intent ac status ST0057`, `intent ac status ST0056/03`, `intent ac gate ST0057`. It lived in THREE homes at THREE values on 2026-08-24, one document disagreeing with itself twice. **Do not put the number on this board; put the calls.**
- **hv's FREEZE SCOPE (2026-08-24): Intentv2 is FROZEN FOR FEATURES and LIVE FOR SHIPPED-SURFACE DEFECTS.** A v3-only defect is a v3-only fix; a shipped-surface one lands in BOTH trees.
- **THE INDEX IS SHARED IN THIS CHECKOUT.** `git add` puts your file where a peer's bare `git commit` sweeps it. Always `git commit --only <paths>`; untracked paths must be staged first or `--only` refuses them.
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.** To ask about another clone, run ITS `bin/int`.
- **A DEVBIN COMMAND RESOLVES ITS PROJECT FROM SOMETHING OTHER THAN YOUR CWD -- BUT THE v3 BINARY RESOLVES FROM CWD.** Both are true and confusing them cost a live incident today.

## DOING

**Nothing in flight.** Everything below landed and is committed.

- **`intent#0070` CLOSED at 17:37Z.** cc fixed it at `3f367cf8`; I filed, reproduced, bounded and closed it, so **the finder and the fixer stay separate on the record.** Resolution note is in the issue body.
- **MY OWN CURRENCY GUARD REFUSED ME TODAY AND I TOOK ITS NAMED REMEDY.** cc's 11 files landed, `intent3` refused every verb, `int local build` cleared it, pair coherent at HEAD. **The FLOOR wording ic caught fired correctly in anger** -- the first time either refusal arm has been read outside its own test.
- **The false roster row CORRECTED**, and the correction states the class rather than quietly repairing it.

## TODO

### LIVE, MINE, UNSTARTED

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` (AT-11.7) has a green to reach. **STILL DECLINED ON SCOPE** -- WP-11 is RELEASE and hv asked for local usability. **TRAP: `codesign --force` REWRITES THE BINARY IN PLACE**, so nothing may hash until `verify_notarised` passes; and `cmd/macos:1294` parses `commit:` with a `sed` -- ADD fields, never rename that one.
- **`thread_view_skew_check.sh` roster admission** -- held on a staleness refusal that does not exist. **THE HOLD RESTS ON A MEASUREMENT FROM 2026-08-20 AND MUST BE RE-DERIVED BEFORE IT IS ACTED ON IN EITHER DIRECTION.** Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`, never a copy.
- **AT-11.6** -- blocked on the contract conflict routed to vc.
- **NEW, AND IT IS MINE BECAUSE I FOUND IT IN MY OWN FILE: NOTHING VERIFIES THAT A ROSTER ROW DESCRIBES WHAT ITS RUNNER DOES.** `runner_roster_check.sh` verifies row-to-file EXISTENCE in both directions and is structurally blind to the row's CLAIM. cc asked that a mechanism, if one is built, be mine. **No mechanism proposed yet, and naming it is not building it.** **AND vc HAS PUT A GATE IN FRONT OF BUILDING IT, WHICH I AM RECORDING BECAUSE IT WOULD OTHERWISE DIE WITH THIS SESSION: DO NOT BUILD BEFORE hv HAS RULED WHETHER A DESCRIPTION IS IN THE ROSTER CHARTER AT ALL.** vc framing: this is a guard whose POPULATION IS FILES where the CLAIM IS BEHAVIOURS, and **widening a guard contract silently is how a roster becomes the mechanism.** So the next step is a ruling, never a checker.

### ROUTED, MEASURED, NOT MINE TO TAKE

- **THE FROZEN-`$INTENT_HOME` MECHANISM: THE DETECTOR HALF IS CLOSED, THE ROUTING IS NOT.** vc landed the ref fix and the CI arm (`a38e884b`, `18ccfbbc`), measured in CI's own log rather than in simulation. **vc states plainly that their reason for `not discharged` expired but the ROUTING did not, and a guard cleared by a peer saying the ruling happened is not a guard.** It sits with hv. **What the guard should ASSERT under the freeze scope is still the unsettled half.**
- **THE SUITE POPULATION CHECK.** `git ls-files` 112 vs `find` 113, gap named, two commands produce it and nothing runs them. **Needs an edit to `tests/run_tests.sh`, which matts runs externally -- scope, not doubt.**
- **THE ESCAPED-MUTATOR REVERT IS SETTLED: KEEP BOTH, and both are committed here.** `AGENTS.md` is a GENERATED view and the escape ran `agents sync` early, so reverting restores a STALE one; `config.json`s `project_id` is LIVE in the store, driven with `.dump` plus both controls. **`MODULES.md` WAS NEVER AN INCIDENT FILE** -- 13:30:54Z, thirteen minutes BEFORE the escape.

## Watch-outs

### FAMILY 1 -- THE INSTRUMENT THAT CANNOT VARY WITH ITS SUBJECT. **Compressed from nine separate rules; the instances are evidence FOR it, not rules beside it.**

**THE TEST: RUN IT WHERE THE ANSWER SHOULD DIFFER, OR YOU HAVE NOT TESTED IT AT ALL.** Not _check your instrument_, which is unactionable.

- **`intent info | head -1` is the PRODUCT BANNER, identical from every cwd BY DESIGN.** I nearly withdrew a CORRECT fix on it -- inside the investigation of an incident caused by this class, in the file whose header warns about it by name. **Pick a line that CAN vary, or diff the whole output and pick none.**
- **`--help` IS NOT A PROBE.** Under INV-07 it exits 0 whether or not a command is built.
- **A GREP'S ZERO IS A CLAIM ABOUT THE CORPUS.** Positive-control the instrument before believing it. I grepped a db for issue id `0001`, got 1609 hits, and the number meant nothing.
- **A NO-OP CANNOT FAIL, SO IT MEASURES NOTHING.** My v2 arm reported `5 -> 5` because upgrade SHORT-CIRCUITED; I had measured that a no-op destroys nothing. **Force the real path before believing a clean result.**
- **`find`-BASED POPULATIONS DESCRIBE THE WORKING TREE, NEVER THE COMMIT** (ic). `run_tests.sh:89`. **A missing test and a passing test are the same observation.**
- **A COUNT OVER A `dirty-` MARKER IS A FLOOR, NOT A DISTANCE** (ic). The uncommitted delta at build time lies outside the range in either direction.
- **A RANGE WITH NO PIN NAMES A DISTANCE FROM A MOVING POINT; A PIN WITH NO RANGE NAMES BYTES WITH NO CONSEQUENCE** (ic, theirs whole). Both are required; neither supports a claim alone.
- **A GUARD WHOSE PREDICATE DEPENDS ON WHEN IT RUNS RATHER THAN ON WHAT IT READS HAS A CATCHABLE WINDOW THAT CLOSES SILENTLY** (vc). Check C exists precisely because it does not.
- **A MEASUREMENT TAKEN BEFORE YOUR OWN WRITE IS STALE BY CONSTRUCTION.** It cost the attribution on today's incident: neither vc nor I had measured issues before writing.
- **TWO READINGS OF ONE SOURCE IS ONE INSTRUMENT COUNTED TWICE** (cc, and it is the sharpest sentence anyone said today). cc derived a table count by grepping `CREATE TABLE` out of `store.rs`; I could not reproduce it, and my own probe returned `if` as a table name. **I declined to CONTRADICT a figure I could not reproduce -- a probe with a proven false positive in its output has no standing to adjudicate the rest of that output** -- and that refusal is what sent cc to `sqlite_master` rather than to a better regex. **A better regex would have agreed with them and taught us nothing.** The live store settled it: 17 tables, 5 FTS5 shadows of `doc_sections`, 12 logical, 12 minus 7 is 5. **The `_vN` names we had both caught are create-copy-drop-rename scaffolding no database ever holds.**
- **A ` M` IN `git status` IS A CLAIM ABOUT THE INDEX, NOT ABOUT CONTENT -- AND AN INCIDENT FILE LIST ASSEMBLED FROM `git status` INHERITS THAT SILENTLY.** `AGENTS.md` sat on the escaped-mutator list for a DAY with **zero changed bytes**, presenting as ` M` through a stale stat entry. **`git diff --stat` separates them in one command, and for a day nobody ran it.** TWO INDEPENDENT INSTANCES, which is why it is a rule and not an anecdote: mine (a KEEP defended on a diff that does not exist) and **cc's, who binned the same file into `peers` twice while reporting scope to matts -- inferring AUTHORSHIP from a marker making no claim about content.** **cc's action was RIGHT and their reason was WRONG, so nothing broke and nothing would ever have surfaced it: A CHARACTERISATION WHOSE ACTION IS CORRECT IS INVISIBLE BY CONSTRUCTION.** **`git status` is not careless or misused -- it answers exactly what it was asked, and the question is about the INDEX.**
- **NEVER `$?` AFTER A PIPE. `cargo test` needs `--no-fail-fast`. `grep -c` exits 1 on zero. The Bash tool's shell is ZSH and does not word-split an unquoted expansion.**

### FAMILY 2 -- THE CLAIM EXCEEDS THE MEASUREMENT, AND THE TRUE HALF IS WHAT CARRIES THE FALSE ONE

**THE DAY'S CENTRAL FINDING, AND ALL THREE OF MY SHIPPED DEFECTS ARE INSTANCES: A CONFIDENT UNMEASURED NUMBER IN A _RATIONALE_ IS LOAD-BEARING IN A WAY ONE IN A _REPORT_ IS NOT -- A REPORT GETS CHECKED, A RATIONALE GETS HONOURED** (vc's formulation; the `multi-second` incident is mine).

- **`bin/intent3:60-66` claimed a coherence check would be "a MULTI-SECOND gate on every command".** Driven: **~85ms end to end.** Wrong by two orders of magnitude, never measured, and it was the ENTIRE stated reason for not doing the thing hv later ruled I should do. **It held the design shut for three days.**
- **`cmd/hosting`'s header claimed "in a throwaway clone a mutator harms nothing".** Never driven. **The clone was real, the build was real, and every verb ran from the LIVE tree's cwd.** It wrote the durable store.
- **`currency.lib` PRINTED A FLOOR AS A DISTANCE** -- the overclaim sat in the error message of the file written to refuse overclaims, **and the fix for it then added an arm nothing drove** (ic). **A fix that adds an arm adds a thing to drive.**
- **A WARNING IS NOT DISCHARGED BY BEING TRUE -- IT IS DISCHARGED WHEN THE REMEDY IT INVITES IS ALSO CHECKED** (ic, against themselves). Generalises to design: my cost comment was TRUE about intent and FALSE about the remedy it invited.
- **MY TELL vs cc's, AND THEY ARE MIRRORS:** I publish the claim the measurement INVITES (goes wrong at the READING); cc drives the measurement and publishes a stronger claim about what it measured (goes wrong at the WRITING). **Both rest on a real number, which is why neither gets challenged.** I did mine TWICE in one hour on the same defect -- `0 issue(s)` means CARRIED none, not LEFT none.
- **A ROW CARRYING ONE TRUE SENTENCE AND ONE FALSE ONE IS HARDER TO CATCH THAN A WHOLLY WRONG ROW, BECAUSE THE TRUE HALF IS WHAT A READER CHECKS FIRST AND IT HOLDS** (cc, wholly theirs, about my roster row). Mine said `the script now forces the real path` of a v2 arm the committed script has never contained, while `Two arms, both v3 binaries` in the same row is exact. **The v2 measurement WAS real; the arm died in a scratchpad and the row went on describing the investigation.**
- **`checked against the schema` VERSUS `checked against a grep of the source` IS THE WHOLE DISTANCE, AND NEITHER OF US COULD SEE IT IN OUR OWN SENTENCE AT THE TIME OF WRITING.** cc's `no third instance is waiting` was sound in its conclusion and stronger than the instrument behind it. **My class, firing in someone else's work, inside a safety argument.** Neither of us has a mechanism for it.
- **A STABLE CONCLUSION ACROSS THREE WRONG POPULATIONS IS NOT CORROBORATION -- IT IS THE POPULATION NOT BEING LOAD-BEARING FOR THAT CONCLUSION** (cc, on vc's withdrawal, and worth keeping SEPARATELY from the error). vc escalated the escaped-mutator set to hv three times -- three, then two -- and the verdict never moved. **The unchanging verdict reads as confirmation and is actually evidence that the thing being argued about was never doing any work.**
- **A HAND-MAINTAINED SET THAT NOTHING CHECKS IS THE ROSTER PROBLEM.** I built one while explaining the roster problem. **Manifest plus a drift check, and a path in NEITHER the manifest nor a declared-exclusions list is an ERROR rather than a judgement call** (vc's declared-disposition rule).

### FAMILY 3 -- ROUTING, RELAY AND ATTRIBUTION

- **THE OFFER IS THE MOMENT TO CHECK, NOT THE SEND.** Once two messages are in flight the only tools left are racing and deduping, **and only one of those works.** Three instances today, one mechanism.
- **A FORWARDING OBLIGATION THAT IS DISCHARGED BUT NOT CANCELLED MANUFACTURES FALSE CORROBORATION.** Routing through a coordinator fails by NEVER ARRIVING; fixing it mid-flight fails by arriving TWICE wearing TWO authorships. **The second announces itself as agreement.** **Tell the RECIPIENT to expect one copy -- the recipient can dedupe, the relay can only be beaten.**
- **VOLUNTEERING A RELAY _IS_ CREATING THE OBLIGATION, AND THE COORDINATOR HABIT IS TO VOLUNTEER** (vc's half). **A rule only the receiver can apply arrives one step too late.**
- **NAME WHICH _HALF_ YOU ARE ATTRIBUTING** (vc, wholly theirs). An incident and its generalisation are separable and usually have different authors. **Over-attribution travels through the ENVELOPE, under-attribution through HEDGING, and the corrective for one is not the other rotated.** A wrong byline reads as a fact ABOUT the record rather than a claim needing evidence -- **nobody audits an attribution.**
- **A PEER TELLING YOU WHAT ONLY THE SOURCE CAN SETTLE IS TELLING YOU WHAT _THEY READ_** (ic). Applies to fold instructions, quoted rows, and relayed rulings alike.
- **A RULING DELIVERED AS A SELECTION AMONG OPTIONS YOU AUTHORED IS ONE WHOSE WORDING IS YOURS AND WILL BE QUOTED AS THEIRS.** **RECORD THE OPTIONS, NOT THE OUTCOME**, in an inbox where it survives the session. **Never reconstruct a menu from memory.**
- **THE BLOCKED PARTY TELLS THE BLOCKER, NOT THE SEQUENCER.** A coordinator carries no obligation to fan out unless someone names it.
- **A PROOF ONLY ITS AUTHOR CAN REPRODUCE IS NOT YET A PROOF THE ESTATE HOLDS** (ic). Different objection from _is it sound_, and it survives the soundness being conceded. **Script the repro.**

### FAMILY 4 -- THE SHARED CHECKOUT

- **FOUR WRITERS, ONE TREE, ONE INDEX.** Peers' dirty files are in every `git status` you read. **Name yours before committing; `--only` is PATH-scoped, never hunk-scoped.**
- **AN UNCOMMITTED EDIT WAS ERASED HERE WITH NO REFLOG TRACE AND hv RULED IT ACCEPTED RATHER THAN INVESTIGATED.** The mechanism is live and undiagnosed BY DECISION. **A protective copy OUTSIDE the tree costs nothing; staging is NOT the remedy, because the index is shared.**
- **A FIGURE ABOUT HEAD BELONGS IN THE SINGLE-WRITER CLONE** (`int suite`, `int hosting`). **A figure about the WORKING TREE must be defended by attribution instead.**
- **`bin/` IS dc's LANE** (hv). `bin/.devbin/cmd/**` is Intent's own; **`bin/devbin` and `bin/.devbin/lib/**` are VENDORED and not this repo's to edit.**

## Decisions

- (2026-08-24) **A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL.** cc reported that matts had ruled 0070 mine to close; I declined to act on it as relayed and **cc withdrew the framing themselves** -- it answered a question asked in THEIR session and was never an instruction to me. The commit needed matts directly, and it came directly.
- (2026-08-24) **THE BUILDER CARRIES THE ROW, REAFFIRMED AGAINST THE OTHER DIRECTION.** cc offered me their v2-estate arm to PLACE because it landed in my lane, reading my wall warning as territorial when it was PROCEDURAL. **A row does not transfer because it landed in someone's lane.** Landed as `v2_estate_issue_carry.sh` under cc's authorship with cc's own row.
- (2026-08-24) **hv RULED: `intent3` MAY REFUSE.** The branch not taken was _keep it a reporter, warn but exec_ -- **vc preserved the menu this time, which is the remedy for the `sync`-skip provenance gap.**
- (2026-08-24) **hv RULED: the local `v2-maintenance` branch DELETED.** A silent wrong answer became a loud absent one. **Fast-forwarding was the option that LOOKS equivalent and re-arms the moment upstream moves.**
- (2026-08-24) **hv's FREEZE SCOPE: frozen for features, live for shipped-surface defects.** `0070` is v3-only; `0071` is shipped-surface and lands in both.
- (2026-08-24) **THE BUILDER CARRIES THE ROW** (vc, applying my own rule to me). vc built four things in my lane today and carries all four. **A row does not transfer because it landed in your lane.**
- (2026-08-24) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE.** `currency.lib` is EXEC-time; `cmd/local`'s `verify_pair` is BUILD-time and demands `sc == HEAD` exactly. **The convergence is RECORDED AND DELIBERATELY NOT DONE.**
- (2026-08-22) **A CLAIM OF UNIQUENESS IS A MEASUREMENT AND MUST BE GREPPED, NOT ASSERTED.**
- (2026-08-22) **SOUND-BUT-UNNECESSARY AND UNSOUND ARE DIFFERENT VERDICTS, AND ONLY ONE IS FAIR TO THE PROPOSER.**
- (2026-08-21) **A ROSTER ROW AND ITS RUNNER MUST BE ONE COMMIT.** Either disagrees alone.
- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES.** `intent3` is an ACTOR, on PATH.
