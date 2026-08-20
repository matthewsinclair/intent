---
node: cc
name: Control Claude
role: control
session_id: 32dc8880-9739-45ef-a496-70118b1d259b
heartbeat_at: 2026-08-20 07:46Z
status: active
focus: "**THE GATE SEES 268 VIEWS INSTEAD OF 1.** `thread_view_skew_check.sh` at `1e2bc65e`, rostered `manual` awaiting dc -- 130-150ms against the sibling's 2860-2940ms, ten arms driven. Only honest downstream of vc's WP-10. **AC-03.6 THEN FIRED ON ME UNPLANTED**: `1e2bc65e` committed an attachment without syncing canon, `ADDS 1 of 1` at rc=1, permanently divergent; `9e191824` demonstrates the compliant order at `ADDS 0 of 88`. dc says an unplanted control beats a planted one and their admission condition is substantially met. doctor rc=0, workspace green, roster 18/18."
claims: [ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. **D34: the committed extract is the interchange -- it TRAVELS while the DB never does.** D29: a gitignored path is never canon.

**D42 -- TIME, and it has no clauses.** `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z`. **Reading it is not enough: I reported file times with a `Z` while `stat` was printing LOCAL. The guard catches the stamp you write; it cannot catch the one you format wrong on the way out.**

## NEXT -- mine

**Fold 4's narrative is at `.history/20260819/wip.md`. Today reshaped it by 06:47Z.**

**0. THE GATE ARM LANDED -- `1e2bc65e`, rostered `manual` awaiting dc.** `intent/st/ST0056/parity/tools/thread_view_skew_check.sh`. **Gated skew coverage was 1 of 269**: the sibling's `CHECKABLE` is ONE triple under `surface/`, and the missing 268 are the thread covers, acceptance contracts and WP covers. **130-150ms at `f0c2805c` on one machine against 2860-2940ms for the sibling**, so it would be the CHEAPEST gated instrument rather than the most expensive. **It forms no verdict** -- `views::skew` stays the single home and this parses one answer rather than computing a second. **It refuses at exit 2 when it cannot read doctor's summary**, because a text-reading gate whose needle stops matching goes green forever and nothing says so. Ten arms; the last two are a MENTION/SUBJECT pair taking identical decoy lines to opposite verdicts. **The parse carries a NAMED EXPIRY** (`9e191824`): when `doctor --json` lands, delete it rather than keeping both. **Ungateable before vc's `b082b488` -- the blocker was never the wiring.**

**0b. AND AC-03.6 FIRED ON ME, UNPLANTED, WHILE DOING IT.** `1e2bc65e` edited `runner_roster_check.sh` -- an ATTACHMENT of ST0056 -- and committed it with canon naming the old bytes. `canon_commit_check.sh`: `ADDS 1 of 1` rc=1 at `1e2bc65e`, clean at `aa4c3ac0`, `ADDS 0 of 88` at `9e191824`. **`1e2bc65e` is permanently divergent in history.** **dc corrected my framing and the correction is the finding**: the rule being on my board is why the instrument exists, not why the commit diverged -- **a control that depends on the author remembering is not a control, it is a hope with a filename.** `manual` costs a divergent commit roughly whenever anyone is busy. dc: an unplanted positive control is strictly better evidence than a planted one, and their admission condition is substantially met.

**1. THE GATE HAS THREE DISPATCHERS AND THEY AGREE WITH EACH OTHER NOWHERE.** Measured at `483fbcfe`, 06:45Z, and vc converged on the identical table independently:

    guard                     pre-commit.intent   cmd/precommit   template roster   RUNS?
    whiteboard-clock-guard            1                 0                1           YES
    whiteboard-header-guard           0                 2                1           YES
    canon-ignore-guard                0                 0                2           NO
    append-only-guard                 0                 0                1           NO

**Two of four run and neither runs through the roster** -- each reaches its guard by a hard-coded path in a different file. **The roster is a THIRD OPINION about what runs, agreeing with neither dispatcher**, and it is the only artefact naming all four. `GUARDS_APPLY` occurs in exactly one file in the tree and in nothing under `.git/hooks/`. **The roster and the dispatcher it describes are ONE file in the repo and TWO on disk, so every check comparing them passes.** With dc; three shapes offered, none picked by me.

**2. AC-01.5 IS UNMEETABLE BY ANY EDIT TO THE GUARD, THE ROSTER, OR THE TEMPLATE.** `canon-ignore-guard.sh` is built, mutation-proven, rostered, and has zero call sites on git's path here. AT-01.5's `red` is correct; both recorded reasons are wrong -- not _not yet wired_, not _wired into a stale file_, but **wired into the roster in a repo whose commit path does not read the roster.** vc holds the canon reword. **A consumer on a fresh install is probably fine and I have NOT measured that** -- do not repeat it as though I had.

**3. #144 FIXED, MUTATION-PROVEN, UNCOMMITTED.** `claude_md_template.bats` asserted four placeholders; `b277013a` removed `[[DATE]]` deliberately. Split into a positive test over the three and **a negative assertion carrying the refusal.** The mutation is the finding: **planting `[[DATE]]` back leaves the three-placeholder test GREEN and moves only the negative one.** A trim records a removal; only a negative assertion defends it.

**4. THE 250-FILE OWNABILITY PARTITION -- DELIVERED 06:55Z, counted at `5b59a14c`, dirty 10. Durable copy in vc's inbox.**

    T  tool payload, not project content    187    intent/plugins/
    B  project content, needs a NEW sigil    59    docs 10, llm 14, history 18, eng 9, autopsy 3, analysis 2, wip/restart/done 3
    N  must never be an artefact               3    .config/config.json, .intentfiles, events.jsonl
    M  already model-derived                   1    todo.md
                                             ---
                                             250

**hv's 250 IS REALLY 59.** `intent/plugins/` resolves from `$INTENT_HOME` and this repo has it only because it IS its own -- **0 tracked in Lamplight, Laksa and Anvil.** **My first hypothesis died in the same probe: I expected `intent/docs/` to be tool payload too, and those consumers carry 61, 4 and 2 there** (`llm/` 21/12/6, `eng/` 0/38/11). A count varying by two orders of magnitude across consumers is project content by definition; only `plugins/` is uniformly absent. Three consumers, one machine -- a probe, not a fleet survey.

**OF THE 59, ZERO ARE OWNABLE BY AN EXISTING ARTEFACT, AND IT IS STRUCTURAL RATHER THAN A JUDGEMENT.** 58 of 59 are `.md`, so `ATTACHMENT_EXTENSIONS` is not the constraint. **Ownership flows ARTEFACT -> ITS OWN DIRECTORY**: `classify` answers only inside a thread dir, a thread realises `intent/st/<ID>/**` and nothing else, and none of the 59 belongs to ONE thread. **THE BLOCKER IS ARITY, NOT POLICY** -- two sigils, and a file with no owning artefact cannot be declared whatever anyone rules.

**`todo.md` IS THE PRECEDENT THE OTHER 59 WANT**: a `View` in `render_all` (`views.rs:951`) -- model-derived WITHOUT being artefact-owned, project-scoped, no manifest entry. **The sole non-`.md` is the known naming violator**, the `.webloc` in `docs/exemplars`, unownable twice over.

**5. AC-03.6 UNCHANGED.** `--staged` landed at `19268867`; nothing owed from me until dc drives the planted-divergence control. **AT-03.6's row text is stale** -- it still says `--staged` is what it needs. vc has it.

**NOT MINE ANY MORE: WP-10 and WP-09 are vc's** (vc moved WP-09 ahead -- `append-only-guard.sh` is one of the two that never fires and `events.jsonl` is the one artefact no rebuild can reconstruct). ST0011's missing completion date is vc's.

## Watch-outs -- the live set

**Folded 21:41Z. Superseded ones are in `.history/20260819/`; these still bite.**

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
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.** **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`** -- and dc measured why that is not merely pending: v3 answers exit 2 for 14 of 32 families, and `intent claude` implements 1 of its 8 verbs against ~230 call sites in this repo's own machinery.
- **`config.json` DOES NOT MOVE WITH `intent_dir`** -- `Project::config_path` always answers `intent/.config/config.json`, because something must be findable before anything is configured.

## Lane boundary

`native/**` and the v3 crates are mine. `bin/**` is not vc's to edit. The parity harness is ic's; `canon_commit_check.sh` and the pre-commit roster are dc's. **Canon writes route through vc.** `CARGO_TARGET_DIR=native/rust/target/cc` **for tests only** -- a release build must land at `native/rust/target/release/` where four nodes read it.

**Every commit I make touching an attachment leaves canon divergent at that commit until vc syncs, and a later sync repairs the NEXT commit and never that one** -- so sync canon FIRST, then commit the file and canon together. Proven on myself: I committed `--staged` into `canon_commit_check.sh`, which IS an attachment of ST0056, and my own AT-03.4 caught the stale hash.
