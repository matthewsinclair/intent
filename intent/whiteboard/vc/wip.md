---
node: vc
name: Validation Claude
role: validation
session_id: de387da0-feeb-49d4-ab72-9a0c46bb5fc7
commit_session_id: 01QowqYJaW1178GFgwUDcxaU -- POINT-IN-TIME. It rotated MID-SESSION with no bounce on 2026-09-04, so "a bounce mints a new one" is too narrow. RE-READ IT OFF YOUR OWN LAST COMMIT.
heartbeat_at: 2026-09-04 17:41Z
status: active
focus: "AGGRESSIVE LOCALFOLD 17:41Z AHEAD OF A COMPACT -- status stays active, a fold before a compact is NOT a session ending and this does NOT release. Pre-fold verbatim at .history/20260904/wip-prefold-1741Z.md, cmp-verified. ON THE BOUNCE: ping all three, then instruct. vc HOLDS THE PEN."
claims: [ST0056, ST0057, ST0060, ST0064, ST0068, ST0070]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`. Every incident narrative is in `.history/`. What follows is MECHANISMS and UNEXECUTED WORK.**

**PRINTING THE COMMAND IS NOT RUNNING IT.** A figure with its regenerating command beside it is AUDITABLE, not CURRENT. **A LIVE FIGURE APPEARS HERE AS THE COMMAND ALONE, WITH THE NUMBER DELETED.** **A HISTORICAL FIGURE IS DATED** and stops being a live claim.

## DOING

**FIRST ACT ON THE BOUNCE: PING cc, dc AND ic, THEN INSTRUCT. DO NOT SEQUENCE ANYONE OFF THIS BOARD** -- every node moved after it was written, and the whole lesson of 2026-09-04 is that a board read carries the timestamp of the board and never of the reader.

**vc HOLDS THE PEN** (hv, 2026-09-04). **A vc ruling declares `authority: vc`, NEVER `authority: hv`.** The pen does NOT cover hv's environment, the release, or `intent fc`.

**NOTHING IS WITH hv ON THE CUT FROM MY SIDE.** The two items that were became rulings by being DRIVEN -- neither was a question; both were measurements I had not taken. **In both cases the ASK was worse than the ANSWER, and that is the reusable part.**

**vc's OWN UNSTARTED WORK, UNCHANGED BY TODAY:** `WP-15` skills-catalogue triage (`ST0065`'s criteria are the bar); the estate-tree question, unsourced and born in a fold; `hv/inbox.vc.md`'s lifecycle; `0224`'s disposition (priced at FIVE estates with a discriminating negative -- an hv menu can be priced, not argued).

**UNFILED AND MINE TO FILE: the remedy-test hole and the existence-check defect, both under `## Open defects I own`.** Neither was filed while its finder was mid-turn; that reason expires on the bounce.

## Rulings 2026-09-04, all `authority: vc`

**ONE RULE COVERS THREE: CANON NAMES IT -> IT STAYS DECLARED AND REFUSES AT rc=2. CANON DOES NOT NAME IT -> IT COMES OFF THE DISPATCH TABLE.**

- **`claude rules validate` STAYS** (`usage-rules.md:140` names it). **`subagents status` COMES OFF** (B1 took it out of canon). **`subagents init` COMES OFF** (canon never named it; it survives only in `lib/help/`, a v2 artefact the v3 binary does not read). Landed by dc at `8783243ce`.
- **`A1` IS ALL FIVE MIRRORED VERBS.** **A VISIBILITY LIMIT IS NOT A SCOPE LIMIT** -- `canon_mandated_verbs_check.sh` skips the write verbs, so a green instrument after A1 says nothing about three of the five. **That caveat belongs in the instrument's OUTPUT, not on a board**; dc put it there.
- **`--all`: a v2-to-v3 REGRESSION, not a doc error.** Filed as `0236`, landed, closed. **rc=2 _known command, not implemented yet_ is a coherent shipping state; rc=1 _unexpected argument_ reads as the operator's typo and is not.**
- **THE canon-git SWEEP LANDS AT `bin/.devbin/cmd/`** -- it outlives `ST0056`, which `st/ST0056/parity/tools/` does not. **RECORDED, NOT BUILT: the durable home is `intent doctor`** -- every Intent project has this exposure; `bin/.devbin/` reaches us alone (`0237`).
- **ic's TOKEN DELIVERY: URL FRAGMENT, PAGE STRIPS IT WITH `history.replaceState` ON LOAD.** ic's never-sent-never-logged argument is about THE WIRE and is true; **it says nothing about the address bar, the history entry, or a URL copied out of the bar.**
- **A WORK PACKAGE IS AN ITEM, NOT A ROW, AND THE BROWSER MUST BE ABLE TO ADDRESS IT.** `View::Item` carries ONE id; a wp needs TWO. **The missing shape is the ITEM OF A CHILDREN COLLECTION** -- `Collection -> Item` exists, `Children -> nothing`. ic's `View::Child { kind, id, field, item }` -> `/thread/ST0056/wps/17` reuses the declared `wps` descent rather than inventing a segment. **SIZE IS M WITH THE TUI ARM UNMEASURED -- ic's honest figure, and they must not revise it until they have driven whether the existence defect is in `--path` alone or in resolution both arms share.** **ic IS HOLDING AND MUST NOT START: whether this rides 3.0.1 is hv's, because it is work discovered after the cut was declared feature-complete.**
- **`at green` L3 arm and the rustdoc gate are MINE, not hv's** -- tooling posture, inside the pen. Taken off cc's hv list.

## Holds

**Each carries the CONDITION that releases it AND the COMMAND that checks it. A hold with no condition is an abandonment; a condition with no check gets recalled instead of driven.**

- **`ST0068` AC-02.1** -- and the thread prefix is NOT decoration, `ST0056` ALSO has an `AC-02.1`. **CONDITION: A RELEASE, NOT A BUILD. CHECK IT: `git tag --sort=-v:refname | head -1`** -- met when it reads past `v3.0.0`. Re-driven 16:01Z: NOT MET.
- **`AT-07.5`'s behavioural arm. CONDITION -- THE CLAIM, NOT THE STATE AND NOT THE ACT: the arm needs a moment when nothing holds the socket it probes. CHECK IT: `pgrep -fl intentd`** -- silence is the condition met; a pid is not, and **A RESTART IS NOT A STOP** (three restarts observed 2026-09-04, zero stops, and the hold never released).
  - **AN UNMEASURED ROUTE THAT WOULD RETIRE THE WINDOW: an isolated instance -- its own `INTENT_HOME`, its own socket, no shared daemon.** Stated as unmeasured; **I still have not driven it.** Taking a window on the SHARED daemon remains hv's and is NOT the condition.

## hv items

- **THE REBUILD.** Guarded scope is clean and `dvb build all` will take the shared path. **The delivered pair misses `Op::Form`, A1 and `0236`.** `0196` is why the timing is hv's: `guarded_release_build` DELETES the shared pair before it builds with no restoring failure path, and `~/.local/bin/{intent,intentd}` are symlinks INTO that directory -- **so a failed build leaves a dangling symlink on hv's own PATH.** ic inflicted exactly that earlier today.
- **THE STORE MIGRATION.** 13 -> 17, ladder complete, each rung transactional with an FK check INSIDE it before `user_version` moves. **The refusal already exists** (`SchemaMismatch` names the remedy and points at `intent doctor`), so only the backup step is missing. **FIX: wire the existing `intent backup` into `migrate()`. XS, not M** -- the verb exists and the backup-log table has been in the schema since rung 3->4.
- **DOES ic's `wp` ADDRESSING SHAPE RIDE 3.0.1?** Build it / reword `AC-17.6` / ship the row unsatisfied and say so. **Not the reword** -- rewording an acceptance row to match what got built is how a contract stops being one.
- **`flip` THEN `burn`, ONE SITTING, IN THAT ORDER.** `flip` rebinds the default `INTENT_BIN` off `bin/intent`, the v2 SHELL SCRIPT (`tests/lib/test_helper.bash:21`). **A burn before the flip produces a baseline the flip invalidates. NEITHER NEEDS A BUILD.** cc's `AC-06.1` coverage half hangs off it.
- **`intent claude skills sync`** -- unblocked (`--dry-run` at `9fe2ee464`, backup verified at `~/.claude/skills-backup-20260904T100500Z.tgz`). A bare `sync` HOLDS on `ModifiedLocally` and `Conflicted`, so the run that names them cannot destroy them.
- **`ST0064` 01.7 SIGNING NEEDS hv's ADC.** An action, not a decision.
- **`ST0070` Phase 0 (`3d68c40a`) -- does it ride the tag?** Its attached `design.md` IS THE HOME; the published page is a rendering.
- **`ST0065`: does `AGENTS.md` exist at fresh init, and what is it a mirror OF?** Option 3 is unwritable until the second is answered. **The proposal sat unrouted 2026-08-28 to 2026-09-04 because its own section 7 said _via vc_ -- that is vc's failure and is recorded as such.**
- **cc's SEVEN, now four:** `at green` L3 arm and the rustdoc gate are mine; `[4]` WP-08 is moot (reads Done); `[5]`/`[6]` WITHDRAWN by cc after the bare-noun census showed four live states their three-option menu did not contain.
- **`0218`** -- what does `uninstall` PROMISE, the files it wrote or the directory it emptied? ic filed at LOW; ic's hold 6 hangs on it.
- **cc's WORD ON `0232`**; **palette Home/End** (product feel); **the `overhead` meter, W46**.

## Standing directives from hv

- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate. **PFIC is _Pure Function, Impure Coordination_** -- deterministic core, I/O at the boundary. NOT the idiom gloss six documents carried until 2026-09-02.
- **THE MENUBAR ICON IS THE INTENT TURTLE**, state DERIVED at paint time.
- **FULLY SHIP v3. intentd is a priority. Then tree-sitter and full search. Push.**
- **DO NOT REINVENT THE WHEEL** -- port from `../Gtools`, `../Conflab`. **Read the thread's own attachments first.**
- **EVERY PROJECT GETS THE WRAPUP AS ITS OWN TECHNOTE** (2026-09-01). Sequence: pristine -> devbin-vc FIRST -> hv drives the devbin rollout while every other estate chills -> only then do the rust-using estates hear about it.

## Watch-outs

**ONE SECTION. IT WAS THREE ON 2026-09-04 -- `## Watch-outs` plus two dated siblings -- which is a Highlander violation in the board that carries the Highlander rule, AND cc's prefix bug live in my own data: `grep -c '^## Watch-outs'` returned 3, so a fold keying on that prefix would have matched the first and corrupted the rest.** cc warned me the bug was latent in my script and I banked the warning without fixing the script. **A HEADING NAME IS A KEY. KEEP IT UNIQUE OR STOP KEYING ON IT.**

**A RULE YOU HAVE TO REMEMBER AT THE KEYBOARD IS NOT A CONTROL** (dc, 2026-09-03). dc hit one instrument defect three times in a session with the rule already written down. **vc has now done it repeatedly in one day, and the tally is the argument: FIVE instrument failures on 2026-09-04 and NONE OF THEM ERRORED** -- a piped `$?` reading the pipe, a wrong spelling (`rules` lives under `claude`) returning the undeclared shape, `--help` structurally unable to see the axis while its negative control passed on a different one, a substring colliding with printed CONTENT, and `stat` on a symlink reading the LINK. **THE KNOWLEDGE LIVED IN A WATCH-OUT AND NOT IN THE SHAPE OF THE COMMAND.**

**IS THE CLAIM ABOUT THE STORE, OR ABOUT THE BINARY?** cc's discriminator, and it applies AT THE MOMENT OF WRITING A FINDING rather than after. Store reads survived a stale binary all day because the schema had not moved; **every binary-BEHAVIOUR claim did not.** A read of an SSOT file -- `dispatch-table.json`, `wire.rs`, `register.md`, `address.rs` -- is not a claim about any binary at all.

**NAME THE BINARY IN EVERY REPORTED DRIVE. A BARE `intent` IN A REPORT IS A CLAIM NOBODY CAN REPRODUCE.** `~/.local/bin/intent` is a SYMLINK. **CHECK IT: `stat -Lf '%m'` -- the `-L` is the whole point -- and `git rev-list --count <marker>..HEAD`.**

**A WELL-FORMED ANSWER ABOUT THE WRONG SUBJECT HAS NO TELL AT ALL** (ic). The instrument answered TRUTHFULLY, about a DIFFERENT OBJECT than the one asked about. **That is why all five failures above looked fine.**

**THE DISPROOF IS USUALLY ALREADY IN HAND, AND RELAYING LAUNDERS IT.** Five instances on 2026-09-04: cc cited `register.md` past the paragraph that answered them; dc nearly adopted cc's instrument without driving it against their own case; **I recommended building a currency check that was QUOTED TO ME IN THE MESSAGE I WAS ANSWERING**; I called `edit wp -> thread` a failure to reach the model when `address.rs:199` documents it as deliberate; I read `AC-17.6` as falsified when the row glosses itself in its next sentence. **MINE PROPAGATED -- cc adopted _nine days old_ from me inside one exchange -- because AN OVERSTATED FINDING TRAVELS FASTER THAN A CORRECT ONE.** **dc's limb: a BORROWED claim is where the discipline is cheapest and hardest to remember, because being wrong costs the courier nothing.** **cc's limb: TWO WRONG NUMBERS THAT AGREE ARE ONE WRONG FACT STATED TWICE** -- a date fused onto a sha reads as a date AND an identifier agreeing, inheriting the sha's authority and none of its checkability.

**MY OWN METHOD FIX, AND IT IS AN ORDER CHANGE RATHER THAN MORE CARE: READ THE DOCUMENTED REASON BEFORE WRITING THE FINDING, NOT AFTER.** Every one of my five was the same sequence -- took a measurement, formed an explanation, wrote it up without checking the explanation against the code that documents it.

**A SEAM BETWEEN TWO SOUND INSTRUMENTS IS INVISIBLE TO BOTH, AND NO CONTROL ON EITHER FINDS IT** (dc). Populations drawn from different sources; the defect lives in the DISAGREEMENT, so it is a member of neither subject. **ONLY AN INSTRUMENT WHOSE SUBJECT IS THE BOUNDARY SEES IT.** `0236`'s static arm closes ONE pairing of three -- `uninstall --all` is named by the parity BATS suite and never by canon, and that pairing still has no instrument.

**A CHECK CANNOT FIRE ON A FLAG WHOSE ID APPEARS IN THE SOURCE FOR AN UNRELATED REASON** (dc, 2026-09-04, driven -- cite dc, not the estate). `--all` sits in `flag_reachability`'s SHIELDED bucket BY CONSTRUCTION, because the three letters occur all over `render.rs` innocently; driven, **a declared-and-INERT `--all` passed the check.** Any future flag whose id collides with a common word inherits the hole, and **the only protection is a BEHAVIOURAL test.** Same family as the substring collisions above, but shipped and load-bearing rather than ad hoc.

**A CONTROL THAT APPENDS TO THE CORPUS CANNOT DETECT AN EMPTY CORPUS** (dc, 2026-09-04). dc's static arm shipped a false green over a population of ONE -- they fed `grep` a filename instead of the file -- and **both controls "fired" anyway, because they APPEND their injected lines and pass whether or not the real corpus is empty.** **PRINTING THE DENOMINATOR is what caught it and is the cheapest habit available.**

**I HAD NEVER READ THE hv INBOXES I AM THE ROSTERED READER OF.** Seven live decisions from cc since 2026-09-03; five from dc including an irreversible-migration warning from 2026-09-02. Every write succeeded, no delivery happened, **nothing reported the difference. CHECK IT EVERY PICKUP: `for f in intent/whiteboard/hv/inbox.*.md; do grep -c '^## (' $f; done`** -- the count is deliberately not written here.

**AN ASK THAT IS NOT ON THE BOARD WILL BE ANSWERED BY SOMEBODY ELSE OR NOT AT ALL.** cc asked me a direct question at 10:03Z; I left it five hours and cc shipped their own recommendation (`eaef2a04f`). **It arrived in an inbox my fold did not sweep and my board did not carry, so every later reading of my own state was silently short one open decision.**

**A FOLD PRESERVES A CLAIM'S WORDING AND NEVER ITS WARRANT.** This board asserted _WP-06 has no build work_ through a fold; WP-06 is XL across sixteen command families and the claim generalised three rows to a package fifteen times their size.

**AN ACCUSATION AIMED AT A PEER IS WHERE A STALE INSTRUMENT DOES THE MOST DAMAGE.** I measured A1 as not landed -- against the pre-A1 binary -- and was one message from telling dc their commit did not function, over staleness ic had caused. **Nothing in the drive said the binary was old.**

**TWO PEERS AGREEING IS EXACTLY WHEN TO DRIVE IT** (ic's W27). Both told me the currency flip was ic's rather than A1's. It was -- and driving it was still right.

## Open defects I own

- **A TEST WHOSE NAME CLAIMS MORE THAN ITS BODY CHECKS: `every_emitted_remedy_names_something_this_build_can_do` (`intent-cli/tests/remedies_are_reachable.rs`).** It asserts a remedy's verb is WIRED; **it never asks whether following the remedy changes the situation the remedy was emitted for.** Passed on `--browser`'s false remedy and passes on the fix. **MEASURED 2026-09-04: 186 remedy emissions; 24 name a verb whose precondition may already be satisfied.** **UNMEASURED: whether anything drives a remedy and re-checks the original error** -- the grep that would answer it lists files MENTIONING the word. Regenerate: `grep -rc remedy native/rust/crates/intent-cli/src/*.rs`.
- **AN ADDRESSED ENTITY IS NEVER CHECKED TO EXIST BEFORE A PATH OR URL IS HANDED BACK -- in `edit --path` AND in `browse`.** `edit wp ST0056/99 --path` returns rc=0 and the thread's `info.md` for a work package that does not exist; `wp show ST0056/99` refuses correctly. **`browse` has the twin** -- `browser_url` composes a URL with no existence check, so `browse st ST9999` would open a browser on a page that then errors. ic disclosed the second while correcting me on two other points, in code they landed the same hour. **Small, contained, two places, and a defect whichever way the `wp` addressing question goes -- DO NOT BUNDLE IT WITH THAT, or the small one waits for the large one** (ic's framing, better than mine).
- **`intent agents` bare answers rc=2 `is a known command that is not implemented yet`, while `agents --help` lists init/generate/sync and `agents sync` is the verb `in-essentials` rule 2 ORDERS EVERY AGENT TO USE.** The family declares itself unbuilt while its leaves work. **cc has claimed it** -- same class as `browsed()`, same file, `render.rs` is theirs.

## Decisions

- **2026-09-04: the pen is vc's for DIRECTION** (hv). Not for hv's environment, the release, or `intent fc`.
- **2026-09-04: `edit wp <thread>/<n> --path` resolving to the THREAD is DELIBERATE, not a defect** -- `address.rs:199`: a `Wp`, `Ac`, `At` and `AcCollection` have no files of their own, so realising any means realising the thread, and **`.intentfiles` names ARTEFACTS and nothing finer.** I filed this as a defect and ic refuted it from the source. **Whether `--path` should prefer the WP's own realised file, which EXISTS at 4090 bytes, is a live DESIGN question and a different one.**
- **2026-09-04: `AC-17.6` is NOT falsified.** _THE KIND IS WHAT DISAMBIGUATES_ is glossed in the row's own next sentence as being about `intent edit 1` being ambiguous between a thread and an issue. **It is not a claim that different kinds reach different artefacts.**
