---
node: cc
name: Control Claude
role: control
session_id: 0bf64b47-09ab-4c8e-8e10-be9f54d29df7
heartbeat_at: 2026-08-19 21:41Z
status: paused
focus: "**FOLDED HARD (fold 4) FOR hv'S GLOBALFOLD. AC-00.4 SHIPPED BOTH HALVES AND THE DEHYDRATION WENT LIVE THE SAME EVENING** -- `e7f00e65` removed 423 files, `intent/st` holds three threads, and all three ROOT_FILES survived, which is precisely what the not-a-view call was for. Gate 50 of 64; precondition block 14, 0 unmet. **AC-03.6 is still owed as ordinary work and no longer gates anything.** Nothing of mine is uncommitted. **Tomorrow: 250 files under `intent/` are not in the store at all, and hv wants some of them in.**"
claims: [ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. **D34: the committed extract is the interchange -- it TRAVELS while the DB never does.** D29: a gitignored path is never canon.

**D42 -- TIME, and it has no clauses.** `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z`. **Reading it is not enough: I reported file times with a `Z` while `stat` was printing LOCAL. The guard catches the stamp you write; it cannot catch the one you format wrong on the way out.**

## NEXT -- mine

**FOLDED 2026-08-19 21:41Z (fold 4). Today's settled narrative -- AC-00.4, WP-03, the parity-form question, the bidi gate -- is at `.history/20260819/wip.md`.**

**1. THE ESTATE IS `intent/` ITSELF, AND IT IS NOT IN THE STORE.** hv, tonight: 250 files under `intent/` are in no artefact at all -- `docs/`, `llm/`, `history/`, `eng/`, `plugins/`, `done.md`, `wip.md`. _Not all of that should be in the db, but certainly some of it should._ **Start by asking which of the 250 an artefact could even OWN**, because the manifest names ARTEFACTS and never FILES, and a file with no owning artefact cannot be declared no matter what anybody decides about it.

**2. AC-03.6 -- OWED AS ORDINARY WORK, NO LONGER GATING.** `canon_commit_check.sh --staged` at `19268867`, byte-proven at five episode commits, dc approved. dc has taken the ruling and will drive the planted-divergence control themselves before admitting it to the roster. **Nothing is owed from me until they do.**

**3. `intent doctor` SEES VIEW SKEW AND IS NOT WIRED TO THE GATE.** It named both stale acceptance views tonight when nothing else did. `view_skew_check.sh` never covered `intent/st/**` in any mode -- its `CHECKABLE` is ONE triple in `surface/`. **The instrument exists and the wiring does not, which is the cheaper half of a fix nobody has scheduled.**

## Watch-outs -- the live set

**Folded 21:41Z. Superseded ones are in `.history/20260819/`; these still bite.**

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
