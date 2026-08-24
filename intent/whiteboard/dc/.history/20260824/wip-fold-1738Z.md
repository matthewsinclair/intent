---
node: dc
name: DevX Claude
role: worker
session_id: 55d5f57e-bc10-4cbf-9959-789541b069dc
heartbeat_at: 2026-08-24 16:25Z
status: active
focus: "**POST-COMPACT 2026-08-24 16:25Z. THE COMPACT WAS TRANSPARENT AND THAT IS MEASURED RATHER THAN ASSUMED: the live `$CLAUDE_CODE_SESSION_ID` STILL EQUALS THE ID ON THIS BOARD, which is the ONLY discriminating field -- `ListAgents`' `started` column is SOCKET age and all four nodes read it wrong on 2026-08-21.** Nothing in flight, claims intact, holding for instructions. **Today is committed through `eabdd639`; the verbatim record is `.history/20260824/wip.md`.** **THE ONE RULE: EVERY DEFECT I SHIPPED TODAY WAS A CONFIDENT UNMEASURED CLAIM SITTING IN A SAFETY ARGUMENT, WHERE IT READS AS A CONSIDERED TRADE-OFF AND GETS HONOURED RATHER THAN CHECKED.**"
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

- **`intent3` currency refusal** -- `currency.lib` (new, one home), `bin/intent3` wired, `tests/unit/artefact_currency_verdict.bats` 13 arms MUTATION-PROVEN RED, `MODULES.md` row added BEFORE the file existed (first time in this lane).
- **`int hosting` containment defect** -- found by its own escape, fixed with a `cd` into the clone, **proven by tree fingerprint plus a positive control**, and the clone-consumer population swept: `suite` and `prepush` were always correct.
- **`intent#0070` / `0071` / `0072` filed with bodies and repros.**

## TODO

### LIVE, MINE, UNSTARTED

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` (AT-11.7) has a green to reach. **STILL DECLINED ON SCOPE** -- WP-11 is RELEASE and hv asked for local usability. **TRAP: `codesign --force` REWRITES THE BINARY IN PLACE**, so nothing may hash until `verify_notarised` passes; and `cmd/macos:1294` parses `commit:` with a `sed` -- ADD fields, never rename that one.
- **`thread_view_skew_check.sh` roster admission** -- held on a staleness refusal that does not exist. **THE HOLD NOW RESTS ON A MEASUREMENT FROM 2026-08-20 AND MUST BE RE-DERIVED BEFORE IT IS ACTED ON IN EITHER DIRECTION.** Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`, never a copy.
- **AT-11.6** -- blocked on the contract conflict routed to vc.

### ROUTED TO hv, MEASURED, NOT TAKEN

- **THE FROZEN-`$INTENT_HOME` MECHANISM IS NOT DISCHARGED.** hv declined direnv (does not reach automation) and hand-refresh (an advisory that must be remembered is not a control). **vc's `shipped_surface_drift.bats` fails the SAME test by a third route: `_v2_root()` returns 1 when the v2 checkout is absent and all THREE tests SKIP, and CI has ZERO references to any v2 checkout.** `INTENT_V2_CHECKOUT` is already honoured and `v2-maintenance` is a branch of THIS repository, so the wiring is cheap -- **but what the guard should ASSERT under the freeze is the unsettled half, and wiring it first buys a green about the wrong property.**
- **THE SUITE POPULATION CHECK.** `git ls-files` 112 vs `find` 113, gap named. **The point is that two commands produce it and nothing runs them.** It would print `THIS RUN'S POPULATION IS 113 AND THE COMMIT'S IS 112` to whoever is reading a green. **Needs an edit to `tests/run_tests.sh`, which matts runs externally -- scope, not doubt.**
- **THE ESCAPED-MUTATOR REVERT.** `AGENTS.md` **KEEP** (a GENERATED view; the escape ran `agents sync` early, so reverting restores a STALE view). `config.json` **KEEP** (`project_id` is LIVE in the store, driven with `.dump` plus both controls; reverting diverges disk from store). **`MODULES.md` IS NOT AN INCIDENT FILE** -- 13:30:54Z, thirteen minutes BEFORE the escape; a revert of "the three" as vc grouped them would destroy my registration row.

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
- **NEVER `$?` AFTER A PIPE. `cargo test` needs `--no-fail-fast`. `grep -c` exits 1 on zero. The Bash tool's shell is ZSH and does not word-split an unquoted expansion.**

### FAMILY 2 -- THE CLAIM EXCEEDS THE MEASUREMENT, AND THE TRUE HALF IS WHAT CARRIES THE FALSE ONE

**THE DAY'S CENTRAL FINDING, AND ALL THREE OF MY SHIPPED DEFECTS ARE INSTANCES: A CONFIDENT UNMEASURED NUMBER IN A _RATIONALE_ IS LOAD-BEARING IN A WAY ONE IN A _REPORT_ IS NOT -- A REPORT GETS CHECKED, A RATIONALE GETS HONOURED** (vc's formulation; the `multi-second` incident is mine).

- **`bin/intent3:60-66` claimed a coherence check would be "a MULTI-SECOND gate on every command".** Driven: **~85ms end to end.** Wrong by two orders of magnitude, never measured, and it was the ENTIRE stated reason for not doing the thing hv later ruled I should do. **It held the design shut for three days.**
- **`cmd/hosting`'s header claimed "in a throwaway clone a mutator harms nothing".** Never driven. **The clone was real, the build was real, and every verb ran from the LIVE tree's cwd.** It wrote the durable store.
- **`currency.lib` PRINTED A FLOOR AS A DISTANCE** -- the overclaim sat in the error message of the file written to refuse overclaims, **and the fix for it then added an arm nothing drove** (ic). **A fix that adds an arm adds a thing to drive.**
- **A WARNING IS NOT DISCHARGED BY BEING TRUE -- IT IS DISCHARGED WHEN THE REMEDY IT INVITES IS ALSO CHECKED** (ic, against themselves). Generalises to design: my cost comment was TRUE about intent and FALSE about the remedy it invited.
- **MY TELL vs cc's, AND THEY ARE MIRRORS:** I publish the claim the measurement INVITES (goes wrong at the READING); cc drives the measurement and publishes a stronger claim about what it measured (goes wrong at the WRITING). **Both rest on a real number, which is why neither gets challenged.** I did mine TWICE in one hour on the same defect -- `0 issue(s)` means CARRIED none, not LEFT none.
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

- (2026-08-24) **hv RULED: `intent3` MAY REFUSE.** The branch not taken was _keep it a reporter, warn but exec_ -- **vc preserved the menu this time, which is the remedy for the `sync`-skip provenance gap.**
- (2026-08-24) **hv RULED: the local `v2-maintenance` branch DELETED.** A silent wrong answer became a loud absent one. **Fast-forwarding was the option that LOOKS equivalent and re-arms the moment upstream moves.**
- (2026-08-24) **hv's FREEZE SCOPE: frozen for features, live for shipped-surface defects.** `0070` is v3-only; `0071` is shipped-surface and lands in both.
- (2026-08-24) **THE BUILDER CARRIES THE ROW** (vc, applying my own rule to me). vc built four things in my lane today and carries all four. **A row does not transfer because it landed in your lane.**
- (2026-08-24) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE.** `currency.lib` is EXEC-time; `cmd/local`'s `verify_pair` is BUILD-time and demands `sc == HEAD` exactly. **The convergence is RECORDED AND DELIBERATELY NOT DONE.**
- (2026-08-22) **A CLAIM OF UNIQUENESS IS A MEASUREMENT AND MUST BE GREPPED, NOT ASSERTED.**
- (2026-08-22) **SOUND-BUT-UNNECESSARY AND UNSOUND ARE DIFFERENT VERDICTS, AND ONLY ONE IS FAIR TO THE PROPOSER.**
- (2026-08-21) **A ROSTER ROW AND ITS RUNNER MUST BE ONE COMMIT.** Either disagrees alone.
- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES.** `intent3` is an ACTOR, on PATH.
