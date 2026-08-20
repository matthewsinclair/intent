---
node: cc
name: Control Claude
role: control
session_id: 32dc8880-9739-45ef-a496-70118b1d259b
heartbeat_at: 2026-08-20 09:08Z
status: active
focus: "**ST0057 IS 38/49 AND WP-01 IS 6 OF 7.** AC-01.2 and AC-01.4 built, driven and green (`7d20b666`, `9e96ac75`); the seventh is AC-01.5, which no edit to the guard, the roster or the template can meet -- with dc. Earlier: the gate went from 1 view to 268 (`1e2bc65e`). **I found the day's own class in my own instrument** -- an expectation that tracked its input and could not fail. doctor rc=0, view skew 268/268, workspace green, nothing of mine uncommitted."
claims: [ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. **D34: the committed extract is the interchange -- it TRAVELS while the DB never does.** D29: a gitignored path is never canon.

**D42 -- TIME, and it has no clauses.** `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z`. **Reading it is not enough: I reported file times with a `Z` while `stat` was printing LOCAL. The guard catches the stamp you write; it cannot catch the one you format wrong on the way out.**

## NEXT -- mine

**Fold 5 at 2026-08-20 09:08Z. Today's settled narrative is at `.history/20260820/wip.md`; this is only what is still owed.**

**STATE: ST0057 38/49, 1 withdrawn. WP-01 is 6 of 7. doctor rc=0, view skew 268/268, workspace green, nothing of mine uncommitted.**

**1. AC-01.5 -- BLOCKED ON dc AND NOT ON ME, AND THE REASON IS THE FINDING.** `canon-ignore-guard.sh` is built, mutation-proven and rostered, **in a roster this repo's commit path never reads.** The gate has THREE dispatchers and they agree nowhere:

    guard                     pre-commit.intent   cmd/precommit   template roster   RUNS?
    whiteboard-clock-guard            1                 0                1           YES
    whiteboard-header-guard           0                 2                1           YES
    canon-ignore-guard                0                 0                2           NO
    append-only-guard                 0                 0                1           NO

**Two of four run and neither runs through the roster.** `pre-commit.intent` is a COPY taken at install time -- guard BODIES resolve live from `INTENT_HOME`, the ROSTER does not. **dc ruled Shape 3**: the installed hook carries no roster and no guard name, resolving both live. Queued behind `critic` and the hooks work, on matts's ordering. **AC-01.5 is unmeetable by any edit to the guard, the roster or the template**, so do not go at those files.

**2. AC-03.6 -- NOTHING OWED FROM ME.** `--staged` landed at `19268867`. dc's admission condition was a planted-divergence control; **I produced an UNPLANTED one instead** -- `1e2bc65e` ADDS 1 of 1 rc=1, `aa4c3ac0` clean, `9e191824` ADDS 0 of 88. dc says that substantially meets it. vc put it in the AT row at `6ce27cab`.

**3. `thread_view_skew_check.sh` IS ROSTERED `manual` AWAITING dc.** Gated skew coverage is 1 of 269 until they admit it. **The parse in it has a NAMED EXPIRY**: when `doctor --json` lands (ic's surface row), DELETE the parse rather than keeping both.

**4. THE PARTITION'S GRAMMAR CHANGED UNDER ME AND THE CONCLUSION HARDENED.** hv ruled `ISSUE:` LEAVES the `.intentfiles` grammar -- `Sigil::Issue` goes, both issue hydrate/dehydrate withdrawn, the 40 legacy markdown files pruned as migration residue. **So the grammar is `STEELTHREAD` ALONE, not two sigils.** My partition (250 = 187 tool payload / 59 project content / 3 never / 1 model-derived, durable copy in vc's inbox) still holds: **the blocker is ARITY, and it is now ONE sigil rather than two.** vc owns the policy boundary and has this. **The 250 and 59 counts are unaffected by reasoning and I have NOT re-measured since the ruling** -- re-run `git ls-files intent/` against the four exclusions after ic lands the prune.

**NOT MINE: WP-09 and WP-10 are vc's. `Sigil`, the issue verbs and the dispatch table are ic's, sequenced behind dc's `critic`. The roster and the hook are dc's.**

## Watch-outs -- the live set

**Folded 21:41Z. Superseded ones are in `.history/20260819/`; these still bite.**

**AN ARM WHOSE EXPECTATION TRACKS ITS INPUT IS NOT A CONTROL -- AND I BUILT ONE.** `canon_concurrent_diff.sh`'s first cut compared the observed changed-path count against the number of edits MADE, so `--one-edit` reduced BOTH together: one against one, self-consistent, green, **structurally unable to fail.** I had spent the day finding this shape in other people's instruments and did not see it in my own until I drove it. **Pin the expectation to the PROPERTY, never to the input**, and drive every reduced-input arm expecting a FAILURE.

**I DIAGNOSED A MECHANISM I NEVER RAN, AND THEN REPORTED IT CLOSED.** I told dc and vc that `intentd` was refused at open on a schema mismatch -- store `user_version` 13 against a binary built at `SCHEMA_VERSION` 11. **`intentd/Cargo.toml` has NO `[dependencies]` at all, `strings` finds 0 `schema_version` in the artefact, and the binary prints _not yet implemented_ at rc=0.** I read a constant out of a crate I had never checked the daemon links, and got a number that was arithmetically fine and about nothing. **One run of the binary would have killed it.** The rebuild I then performed "verified" the fix by making the same non-observation twice, and calling it CLOSED is what made the invention durable. **The SKEW was real; the CAUSE was invented -- and a real finding is the best possible cover for a false mechanism attached to it.** dc's replacement is permanent where mine was stale: `intentd` depends on nothing, so it NEVER relinks and no rung will ever move it.

**`grep -c` EXITS 1 ON A TRUE ZERO, SO A `|| echo 0` FALLBACK FIRES ON THE ANSWER BEING ZERO -- AND IT BIT TWO NODES IN ONE HOUR.** My first dispatch table printed every zero TWICE and I read past it; vc fired the identical trap in the same hour with it already on their own watch-out list. **0 and 0-twice read the same, which is why it survives -- it would have corrupted any non-zero count.** `a=$(grep -c X f); a=${a:-0}`.

**I READ TOO NARROW AND vc READ TOO NEW, AND NEITHER OF US READ THE CHAIN.** I measured `pre-commit.intent` and said three guards never fire; vc traced `pre-commit.sh` under `bash -x` and said all four are invoked. **Both measurements were correct about the file measured and both conclusions were wrong**, in a check whose entire subject is dispatchers disagreeing. **`bash -x` tells you which file RAN, not what runs at commit time.** Follow the chain from `.git/hooks/<hook>` every time; the installed copy and the template are different files with the same name in different trees.

**`git commit --only` SEPARATES FILES, NOT AUTHORS -- AND IT BIT BOTH WAYS IN ONE DAY.** `924d556b` carried ic's WP-02 under dc's message; `b277013a` carried dc's live MUTATION under mine, so HEAD shipped a deliberate defect for thirteen minutes looking like a fix. **Neither of us misused the tool; the tool assumes one writer.** The guard is not "build your work" -- it is **build the WHOLE WORKSPACE before you commit, because what you are checking is the tree you are about to publish, not the edit you made.** Checking my own crate is what let it through.

**A ZERO IS NOT A RESULT UNTIL THE CHECK HAS PRODUCED A NON-ZERO.** Five instances today, the last one tonight: a `--workspace` run that summed to `passed: 0 failed: 0` because a peer's compile error aborted every target before any test ran. **A run that reports no suites is not a run that reports no failures.**

**A PEER'S RED CONCEALS MINE AND LOOKS EXACTLY LIKE AN ALL-CLEAR.** `cargo check --all-targets` aborts at the lib. I shipped `239238df` with a red target on this; tonight `organize::Plan` gaining a field stopped every `intentsvcs` test target compiling, for everyone.

**cwd PERSISTS BETWEEN TOOL CALLS AND I DRIFTED EIGHT TIMES TODAY.** A `cd` into `native/rust` made a later heredoc write fail, a binary path return `rc=127`, and a `grep` return nothing -- each of which reads as a result. **Absolute paths, always; there is no version of this I get right by remembering.**

**I MEASURED A PATH AND STATED A CONCLUSION ABOUT A SUBJECT.** `ls .intentfiles` at the repo root said no such file, and I told two peers in bold that the manifest was **absent**. It is at `intent/.intentfiles`. **Absent and present-and-declaring-nothing are opposite states** -- absent keeps everything, empty keeps nothing -- and the 545-file removal plan I was warning about was itself the proof my premise was wrong.

**I ASSERTED A COUNT I HAD NOT RUN, INSIDE THE ARGUMENT THAT THE LIST WAS TOO LONG TO READ.** Told dc the unclaimed files sat in two directories; it is four. vc caught it. **The best evidence for that finding was the mistake in the message making it.**

**A LOOSE PREDICTION IS ONE NOBODY CAN DRIVE.** I wrote that an unclaimed-set change is _not distinguishable at any scroll length_. vc drove it: ADD moves the count and is visible today; **only a SWAP at constant cardinality is invisible.** The remedy I proposed -- count plus directories -- carries exactly the two quantities a same-directory swap leaves untouched, **so it would have reproduced the defect and hidden the list behind a flag nobody passes.** A digest of the sorted set is what satisfies the property. **State the arm you would falsify, or the remedy gets built against the wrong sentence.**

**A CONSISTENCY CHECK CONFIRMS THE PARTS AGREE WITH EACH OTHER AND SAYS NOTHING ABOUT WHETHER THEY AGREE WITH THE WORLD.** `runner_roster_check.sh` went green at 11 gated on a wiring that judged the wrong commit. **A roster verifies a tool is DISPATCHED; nothing verifies that dispatching it MEASURES the right subject.** Third instance of the family today, counting vc's skew guard and the unclaimed report.

**A DETECTOR THAT CANNOT TELL A SUBJECT FROM A MENTION TAXES WHOEVER DOCUMENTS THE REPAIR.** My inbox broadcast matched the `_(empty)_` sentinel inside my own quoted prose and wrote hv's inbox back unchanged at rc=0, **silently dropping the human from two broadcasts.** The whiteboard header guard refuses to scan prose for exactly this reason.

**IN A FOUR-NODE CHECKOUT, PRESENCE IDENTIFIES A FILE AND NEVER ITS AUTHOR.** Three misattributions onto me in one day, each inferring the author from who was most recently active in the area. **The AT row that cites a file names its owner; `git log` and `git status` cannot, because the file is untracked precisely while it is in flight.**

**A COMMAND WHOSE BLAST RADIUS EXCEEDS WHAT IT WAS AIMED AT, RUN INSIDE A RIG.** `git reset --hard` to clear a planted divergence took my uncommitted prototype with it; scoped `git restore --staged <path>` was the right tool.

**`TZ=UTC git log --date=format:` DOES NOT RESPECT `TZ`. ONLY `--date=format-local:` DOES.** `format:` renders the commit's OWN recorded zone, so a careful command produces a `Z` that is a lie. Cross-check with `date -u -r $(git log -1 --format=%at)`.

**A CRITERION MUST CLOSE EVERY DEGREE OF FREEDOM THAT LETS A PASSING TEST COEXIST WITH THE DEFECT** (vc): **INSTRUMENT**, **DEPTH**, **EXTENT**, **PIN** (a named commit, never `HEAD`).

**ATTACHMENTS ARE DISK-FIRST; VIEWS ARE DISK-DISCARDED. SAME DIRECTORY, SAME APPARENT KIND, OPPOSITE DIRECTIONS, BOTH SILENT** (vc). The reason AC-03.4's remedy leads with _copy the file aside first_ -- and the reason ROOT_FILES are deliberately not views, which the estate tested tonight by removing 423 files around them.

## Standing rulings

- **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE.** Both threads carry an `AC-04.4`; four carry an unrelated `AT-03.6`. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE.**
- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything found building v3 is work.
- **An uncarried file is NOT a disposition** (vc). **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON -- and re-deriving the reason is how you find out the reason was wrong.**
- **A DEVIATION FROM AN `as-observed` ROW WANTS RATIFYING, NOT FIXING.** The `.bak` went because hv ruled, not because I judged it redundant. The grounds generalise: **a safeguard that guards a loss git already prevents is not a safeguard.**
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING. **`doctor --fix` is WITHDRAWN. `Outcome` is deliberately NOT `#[must_use]`.**
- **THE CRITIC GATE MAPS SEVERITY BACKWARDS, AND v3 ON PATH EXERCISES BOTH WRONG ARMS AT ONCE.** Measured at `6ce27cab`, 07:48Z: v3 `critic <lang>` answers **2** (_known command not implemented_) and the gate's `case` at `pre-commit.intent:147` sends everything but 0 and 1 to **`fail-open`** -- so the one condition meaning THE CHECKER DID NOT RUN is the one waved through, in all five languages. v3 `critic` with no lang answers **1**, which the gate **BLOCKS** on, printing a clap usage string dressed as critic findings. **The severe condition passes and the trivial one blocks.** **AND EXIT 1 IS OVERLOADED ACROSS THE TWO BINARIES** -- findings under v2, clap usage under v3 -- which is the half the DO-NOT-PATH ruling never recorded, and the half that fails LOUDLY and misleadingly rather than quietly. `critic` landing is therefore a PRECONDITION for dc's repoint, not one family among 32.
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.** **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`** -- and dc measured why that is not merely pending: v3 answers exit 2 for 14 of 32 families, and `intent claude` implements 1 of its 8 verbs against ~230 call sites in this repo's own machinery.
- **`config.json` DOES NOT MOVE WITH `intent_dir`** -- `Project::config_path` always answers `intent/.config/config.json`, because something must be findable before anything is configured.

## Lane boundary

`native/**` and the v3 crates are mine. `bin/**` is not vc's to edit. The parity harness is ic's; `canon_commit_check.sh` and the pre-commit roster are dc's. **Canon writes route through vc.** `CARGO_TARGET_DIR=native/rust/target/cc` **for tests only** -- a release build must land at `native/rust/target/release/` where four nodes read it.

**Every commit I make touching an attachment leaves canon divergent at that commit until vc syncs, and a later sync repairs the NEXT commit and never that one** -- so sync canon FIRST, then commit the file and canon together. Proven on myself: I committed `--staged` into `canon_commit_check.sh`, which IS an attachment of ST0056, and my own AT-03.4 caught the stale hash.
