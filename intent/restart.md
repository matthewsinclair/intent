# Intent -- narrative state, traps and conventions

**Current as at `0411ea2b`, 2026-08-24.** Current work is `intent/wip.md`; the entry point is `.claude/restart.md`. **This file carries where-you-are-standing, the traps, and the conventions -- and nothing that belongs in the other two.**

## Where you are standing

**THIS CHECKOUT IS v3 ONLY. The v2 CLI the fleet runs lives in a SEPARATE checkout at `~/Devel/prj/Intentv2`, branch `v2-maintenance`.**

**THE HAZARD THAT OUTRANKS EVERYTHING ELSE HERE: the fleet resolves `intent` through `$INTENT_HOME` to that FROZEN checkout, so a shipped-surface fix landed in ONE tree reaches nobody and presents as done.** Four instances in one day on 2026-08-24. `tests/unit/shipped_surface_drift.bats` reddens on it now, unattended, and **its first catch was its own author.**

**hv's ruling, and it is SCOPED: Intentv2 is FROZEN FOR FEATURES and LIVE FOR SHIPPED-SURFACE DEFECTS.** That gives the guard its property -- not _these two trees agree_ but **_a shipped-surface change is either in both or declared_** -- which decides which of its two exception kinds a new entry gets.

**The branch point is the lesson worth keeping.** `v2-maintenance` was cut at main HEAD, **not at the `v2.19.0` tag**: the old symlink resolved into the working tree, so the fleet had NEVER run the tag, and branching there would have reverted **2027 commits** across every project on this machine while presenting as a symlink move. **A released tag is evidence about a release, never about a deployment.** And the binding was never the symlink -- three routes reached this checkout, and `bin/intent:26` self-resolves only `if [ -z "$INTENT_HOME" ]`, so **the exported var beats the symlink outright.** `env -u VAR <cmd>` tells an override from a resolution defect.

**THE ARCHITECTURE, INVIOLABLE AND UNCHANGED FOR THE WHOLE REWRITE.** The crates are `intent-cli`, `intentd` and `intentsvcs`. **`intentd` and `intent-cli` are BOTH clients of `intentsvcs`, which solely owns `intent/.cache/intent.db`.** `intentd` is not the SSOT and no read requires it. Diagram: `intent/st/ST0056/design.md:12-17`. **The word `intentdb` is retired and names no component** (hv, 2026-08-21) -- it was a typo that propagated corpus-wide. **D01 was REVERSED by hv 2026-08-15; do not reason from it.**

**THREE LAYERS, AND CONFUSING THEM IS THE RECURRING ERROR:** canon (`intent/.canon/st/<ID>.json`, committed, **never sparse**) / store (`intent/.cache/intent.db`, gitignored, **the durable SSOT**) / views (`info.md`, `acceptance.md`, committed, **generated**). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded.**

**`.intentfiles` IS DURABLE STATE**, the record of which database artefacts also have a realised form on disk. **Many writers, no recomputation** -- `st new` adds an id, `st done` removes it, a human may edit it, **nothing derives it from status**. **ABSENT IS NOT EMPTY:** a missing manifest keeps everything, a manifest declaring nothing keeps nothing.

**Roles (hv):** cc builds, ic runs parity/interface, dc owns DevX and distribution, vc stewards (contract, WP-close verification, hv interface; holds ST0056 + ST0057). **localfold = your own board; globalfold = project-wide docs, and it is vc's.**

## The local cutover -- v3 is usable across the estate (ST0058, 2026-08-22)

**hv's aim, verbatim: _"Not necessarily releasable to the public, but useable by me across the wider estate here locally."_ A DIFFERENT BAR from the 3.0.0 release gate, and it is met.** Full detail: `intent/st/ST0058/design.md`.

    int local build                     # coherent binary pair, VERIFIED -- never bare `cargo build --release`
    int local status                    # which `intent` wins on PATH, read-only
    cd <project> && intent3 upgrade     # the switch: explicit, per project, one at a time
    git checkout . && git clean -fd && rm -rf intent/.cache      # the way back

**THE ONE PRECONDITION IS THE WHOLE RISK: THE WAY BACK IS `git`, SO A PROJECT WITH UNCOMMITTED WORK HAS NO WAY BACK.** Commit or stash first.

**`intent3` IS A WRAPPER AND MUST `exec`, NEVER BE COPIED.** A bare copy has no `lib/templates/` marker above it, so `install::home()` fails, every hook refuses at **exit 1**, and **Claude Code blocks on 2 and NOT on 1** -- the strict `/in-session` gate would silently stop enforcing in every project at once.

**`cargo build --release` DOES NOT RELIABLY PRODUCE A COHERENT PAIR, DELIBERATELY.** `build-support/source_commit.rs` omits `rerun-if-changed` on purpose, because emitting any would REPLACE cargo's default of re-running on package change and make the embed stale on CODE changes, silently, in the worse direction. **Nobody "fixes" it. `int local build` forces both crates and verifies the set.**

**THE MIGRATION FLOOR IS EXACTLY 2.19.0 AND 11 OF 16 PROJECTS ARE BELOW IT**, so they need a v2 `intent upgrade` first. **The canary order INVERTS from the obvious one: everything small and dormant is below the floor, so the first switchable projects are the LARGE ACTIVE ones.** Baize first, then Conflab, then Lamplight; Laksa only after its dirty paths are committed. **`Intentv2` MUST NEVER BE MIGRATED** -- it carries a config so every census finds it and it looks like the ideal canary, and it is the v2 CLI fifteen projects RUN. **A census that finds projects by config presence cannot tell a consumer from the tool.**

## Live and unfixed -- read before driving this repo

- **`intent edit` AND `intent st edit` WRITE ON THEIR rc=1 REFUSAL PATH.** They mutate the store and append to **TRACKED** `intent/.intentfiles`, putting a realisation-policy diff into your next commit that you never made. **THE PRECONDITION IS THE FINDING: unrealised CLEAN, realised-without-manifest CLEAN, manifest-present WRITES** -- two nodes independently swept the two conditions that hide it and both published clean. **A VERB IS NOT READ-ONLY; IT IS READ-ONLY IN A CONDITION.** Affected population is exactly one project and **it is this one**.
- **EVERY ROUTE TO `claude skills sync` SOURCES SKILLS FROM THE FROZEN Intentv2 CHECKOUT** -- including running the dispatcher in THIS tree, because the exported `INTENT_HOME` beats self-resolution. **NOT YET ARMED** (0 skill commits since the split, 0 files differing) **and the first skill edit reverts silently while the sync reports success.** Route B works today (`env -u INTENT_HOME <this repo>/bin/intent claude skills sync`) and **is an advisory with an expiry** -- it runs the v2 shell dispatcher that WP-12 prunes.
- **U3 IS A CONTRACT GAP, NARROWER THAN IT LOOKS.** Five verbs are mandated in canon and unimplemented in v3 -- `claude skills`, `lang`, `plugin`, `ext`, `version` -- **all dispositioned `keep`, so all are UNBUILT rather than retired.** `treeindex` is the only RETIREMENT and the canon still mandates it in 3 files. **DO NOT EDIT THE CANON: every one of those verbs WORKS in v2, so the mandates are correct for 16 of 17 projects. The canon is not defective, it is not VERSION-AWARE.**
- **`intent/.backup/db/` IS EMPTY**, so there is no pre-incident snapshot of the store. `intent#0072`.

## Measuring anything here

**`int suite` RUNS THE SUITE IN `prepush`'s SINGLE-WRITER CLONE AND IS ATTRIBUTABLE BY CONSTRUCTION.** It prints `DESCRIBES=<sha>` and cannot be perturbed by a node editing the tree mid-run. It measures HEAD, not the working tree.

**WHILE ANYTHING IS UNCOMMITTED, NO SUITE FIGURE CAN NAME A REVISION BY CONSTRUCTION** -- the clone cannot contain what HEAD does not. **Commit first, then measure.**

**RUN THE BATS SUITE THROUGH `tests/run_tests.sh`, NEVER `bats` DIRECTLY.** The runner exports `INTENT_FIXTURE_VERSION` from `VERSION`; a direct invocation builds a **v3** fixture against the **v2** binary and every test dies on the version guard -- 302 failures once, 300 of them that one refusal and none of them real. **`tests/lib/test_helper.bash:93` still defaults fixtures to `3.0.0`**, so a direct single-file run hits it; dc's one-line convergent fix is written and held on hv.

## Traps that cost real time

- **THE GATE, AND ANY N-OF-M, IS COMPUTED BY A VERB.** Hand-tallying produced three wrong numbers in three days. **A number with more than one home drifts in every home and nothing compares them** -- Highlander applies to a figure in prose exactly as it applies to code.
- **THE POPULATION IS THE CLAIM.** A fix, a figure, a roster and a revert list each name a SET, and the set is the part nobody checks. **A fix that reached one site of three, a three-file incident list that was one, a roster that outlived its instrument, a precision figure over a corpus that could not exhibit the failure.** One grep for the discriminator settles it, **and the discriminator is rarely the obvious token.** **It reaches RULES too: a remedy asserting completeness is a population claim about the failure modes it covers, and a remedy that names a failure mode without covering it is a population claim of ZERO dressed as a finding.**
- **A ZERO FROM YOUR OWN INSTRUMENT IS A CLAIM ABOUT THE INSTRUMENT.** Three failure sites, and knowing which you are in is the diagnosis: **it cannot reach the subject** (wrong flag, wrong path, an upstream that CONTAINS what consumers only reference); **it matches real strings that are not the thing** (branch order; ANSI escapes between `Running` and a path; a `"text"` key that `criteria` and `attachments` SHARE, separable only by PATH); **or its input is not what you think** (`gh run view --log` on an in-flight run returns one line saying the log is not ready). **Ask it to find something you KNOW is there before believing it when it finds nothing.** **AND ic's SHARPENING NAMES WHY THIS FAMILY IS WORSE THAN OTHER WRONG ANSWERS: a query against the wrong FIELD answers with an ABSENCE, and an absence is the one result that never looks like a bug in the query.** Their case: the WPs key on `seq` and `id` is null for all sixteen, so keying on `id` returned nothing -- which reads exactly like _the work package does not exist_. **Every other wrong answer invites a second look; this one closes the question.**
- **A TRUE MEASUREMENT OF A DIFFERENT PROPERTY, OFFERED AS PROOF, IS THE HARDEST TO SEE -- the evidence being real is what makes it persuasive.** Correctness and currency are independent and only one is ever checked. **A precondition that holds today is not a property.** A branch point is a fact about history and never an answer about now. **And it is most dangerous wearing rigour: in a clean security sweep, or in the act of correcting a peer's arithmetic** -- the slots we have agreed not to re-examine are exactly where a wrong answer survives.
- **SILENCE AND SUCCESS ARE IDENTICAL UNLESS SOMETHING DISTINGUISHES THEM.** A path-filtered workflow that never sampled the change; an escalation written to an inbox with no named reader; **a guard printing `All tests passed!` while skipping every test** -- the last is an ACTIVE false positive rather than a merely absent signal. **Count the runs, not the colour.**
- **A CI RUN'S SUBJECT IS THE PUSH, NEVER YOUR COMMIT.** In a five-node checkout the push carries whatever peers landed since your last one, and the head sha names the PUSHER. Twice on 2026-08-24 this produced a wrong attribution, once nearly blaming a peer's fix for a failure it had nothing to do with. **Read `git log <lastpush>..HEAD` before attributing a run to anybody.**
- **NO INSTRUMENT HERE CATCHES AN EXPIRED CITATION -- only a builder trying to satisfy the row does.** `at lint` exempts `to-write` from L2/L3, correctly. **The cheap split: does the cited file carry the row's own literal id?** 0 hits means the citation is wrong; **1 hit is neither answer and usually means a shared header.**
- **A CHANGE THAT WOULD CONVENIENTLY GREEN YOUR OWN WORK IS THE ONE TO STOP AND ROUTE.** The tell, not the virtue.
- **THE REVISION IS PART OF THE FINDING.** Name revision, clock and dirty count on every measurement. **ZERO FAILURES ACROSS A WORKSPACE DOES NOT PROVE A BINARY RAN** -- confirm each subject appears in the `Running` list.
- **THE SHARED OBJECTS ARE THE INDEX, CANON, AND THE GUARD SCRIPTS THEMSELVES.** `git commit --only <paths>` bounds what is COMMITTED and bounds NOTHING about the GATE, **and it cannot reach an untracked file at all** -- so the honest rule is an ORDERING: stage the narrowest pathspec and commit in the same breath, never leaving the index dirty across a pause. **Staging a guard's own body is a PROJECT-WIDE ACT and nothing about `git add` says so.**
- **`sync --to-store` IS DISK-AUTHORITATIVE FOR ATTACHMENTS** -- a canon-only edit to a realised attachment is discarded in silence at rc=0. **For a typed field CANON wins.** The read verbs and `at lint` read the STORE: edit canon, `--to-store`, THEN lint, and **check the sync's rc, not its tail.** **v3 `issues` has NO body setter** (`list/add/show/close/open`) -- canon plus a sync is the only route, verified by fingerprint.
- **`intent st list` defaults to in-progress and returns 2; `--all` is NOT a flag** -- use `st list --status all`. **But `intent issues list` uses `--kind all`, and the two verbs MIS-TEACH EACH OTHER**: `all` is legal in both vocabularies and each refuses the other's flag BY NAME rather than by concept, so the refusal never points at the sibling. **This very sentence taught a node the wrong flag one verb over** -- `issues list --status all` exits 1 with EMPTY STDOUT, which reads as a true zero.
- **A ` M` IN `git status` IS A CLAIM ABOUT THE INDEX, NOT ABOUT CONTENT** (dc). A file can present as modified for a day through a stale stat entry with **zero changed bytes**, and an incident list assembled from `git status` inherits that silently. **mtime does not rescue it -- that is a different true property of the same unchecked set.** `git diff --stat` separates them.
- **A QUOTATION IS TESTIMONY ABOUT A DOCUMENT, NOT THE DOCUMENT** (cc, generalising ic's fold rule past fold instructions). **Quotations are the commoner case BECAUSE THEY LOOK LIKE EVIDENCE RATHER THAN TESTIMONY.** A comment quoted as a criterion travelled three documents deep before six words were checked. **THE ENVELOPE BEATS THE BYLINE** -- attribution fails in both directions through the same channel, so **NAME WHICH HALF**: an incident and its generalisation are separable and usually have different authors.

- **`FIXED` IS NOT A STATE -- worktree, index, HEAD and pushed are FOUR.** A peer saying _I fixed it_ reports the first while the reader hears the third, **and that is this estate's default condition.** Ask which.

- **A RECORDED REASON IS RETIRED BY AN UNRELATED CHANGE, AND NOTHING WATCHES THE JOIN** (cc). Eight instances in a week, and **every one surfaced because a builder picked the reason up in order to USE it -- which is scheduling, not an instrument.** The verdict outlives its argument and nothing flags it: **an answer surviving a change of premise is not the same answer, and re-putting it is how you find out.**

- **A CORRECT REFUSAL IS NOT A SAVE.** A refused `--to-store` leaves the bytes only on disk, so the next `--to-disk` destroys them at rc=0. **The refusal GUARANTEES the loss and arrives wearing the costume of a save.**

- **ORDER IS FORMAT, THEN SYNC, THEN COMMIT.** `prettier` re-stages inside the commit window, after `sync` has already hashed the worktree.

- **FIVE SESSIONS SHARE ONE WORKING TREE, SO TWO ORDINARY GIT INSTRUCTIONS HAVE NO REFERENT HERE.** **"Pull before you read this" is meaningless** (ic) -- a peer's commit is in your HEAD the moment they make it; `git merge-base --is-ancestor <sha> HEAD` is the one-command check and it is always already true. **And the symmetric half: "I will push only my own commits" is equally meaningless**, because one branch in one tree means a push carries every peer commit made since the last one. **That is not a hazard to avoid, it is the mechanism** -- it is how a whiteboard-only commit fired the rust workflow, and how a peer's Rust change arrived under another node's head sha. **Both halves fail the same way: an instruction written for many clones, applied to one tree.**

- **MECHANICAL.** `--no-fail-fast` always. **Never `$?` after a pipe** -- `||` binds to the last stage, so a confirmation arm can be dead code. **`grep` is ugrep here** (`-E` throughout; `grep -c` exits 1 on zero). **zsh does NOT word-split unquoted `$var`** -- a path list in a variable reaches `git commit` as ONE argument. **`bash -n` cannot parse a `.bats` file.** **Isolate the target dir, keep it inside the checkout, use an absolute path** -- `install::home()` walks `current_exe()` ancestors for a marker. **The shell cwd persists between calls.**
- **FOUR SHELL CRITIC FINDINGS ARE DELIBERATELY NOT FIXED AND MUST NOT BE.** `bin/intent_st:1187`/`:1208` (`$LIST_ARGS`) and `bin/intent_treeindex:220` (`$prune_expr`) are **intentional word-splitting**; `bin/intent_st:1353` is a fragment of a multi-line `sed` script the line-based proxy cannot parse. **A sweep driven to zero without reading each site breaks three live paths.**

## The clock, and it has THREE generators with three different remedies

**Every stamp is READ FROM A CLOCK: `date -u +'%Y-%m-%d %H:%MZ'`.** A stamp you did not read is fabricated data, not an approximation. `git log` and `stat` both print LOCAL -- reading one and appending a `Z` produces a stamp wrong by exactly the local offset and looking perfect.

The commit guard's three checks catch a **future** stamp, a **missing `Z`**, and an **inbox going backwards**. They do not close the class, and the three generators are not the same defect:

1. **ARITHMETIC** -- read the clock ONCE, then advance by feel. **Monotonic BY CONSTRUCTION, so it satisfies check C perfectly.** Remedy: **read per stamp, never per session.** A second read does defeat it.
2. **FABRICATION WITH THE CORRECT VALUE PRESENT** -- typing a plausible number while a correct one sits four lines above. **No read defeats it, because the read already happened.** Remedy: **never type the value -- substitute it from `date -u` into the edit so the hand is out of the path.**
3. **A STALE REFERENCE, WHICH ONLY EVER ACCUSES THE OTHER PARTY** (ic). Judging a peer's stamp against your own last read makes their correct stamp indistinguishable from a fabricated one. **A stale reference never accuses itself.** Remedy: **judging a stamp IS a stamp-sized act and owes its own read.**

**THE LIVE CHANNEL HAS NO DOOR AT ALL** -- all three checks run at commit. **The hazard is transcription: a peer quoting a live stamp into their board launders it into the committed record, past a guard at the wrong door, wearing their authorship. Attribute a peer's live stamp; never assert it.**

## Conventions

T-shirt sizing only. **ALWAYS use the intent CLI for ST/WP.** NEVER manually wrap markdown. **NO Claude attribution in commits**; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. **Commit to `main` only when matts asks; always `git commit --only <paths>`.** matts runs the full suite externally and is the acceptance verifier. **NEVER `--no-confirm` on the release.** Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/int build release` date them at cut time -- **and a defect introduced and fixed inside one unreleased cycle gets NO entry, because there is no reader for it.**

**v3 IS on PATH as `intent3`, a DISTINCT NAME**, so `intent` still resolves to v2 and the fleet's gate is untouched BY CONSTRUCTION rather than by anyone remembering. `intent3` -> `bin/intent3` -> `native/rust/target/release/intent`. **Note which binary that is** -- the release build can lag the debug one, and the gate reports it as built from an uncommitted tree, so **pin by hash, never by the marker.** **`upstream`'s push freeze was LIFTED by hv 2026-08-20** with an empty `FROZEN_REMOTES`.

## Why this file is short now

**It was 224 lines, `.claude/restart.md` was 147 and `intent/wip.md` was 133 -- three copies of one narrative, each opening with a banner saying it superseded everything below it.** That banner is the tell: **nobody was deleting, only prepending.** `END OF DAY 2026-08-21`, `End of day 2026-08-20`, the v2/v3 split and one 30-line clock note appeared in **all three, verbatim**.

**It is the gate-figure defect at document scale** -- the same three homes carrying three values, found on 2026-08-24 when the number turned out to be 62 here, 62 there and 65 in the third, one of them disagreeing with itself inside one document. **Each file now has ONE job. If you find yourself writing a supersedes banner, delete what it supersedes instead.**
