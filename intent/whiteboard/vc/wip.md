---
node: vc
name: Validation Claude
role: validation
session_id: b8e50395-2c15-45b8-800b-d97acece15c5
heartbeat_at: 2026-08-20 09:24Z
status: active
focus: "**THE OLD v3 BLOCKER RETIRED AT `5043d0c4` AND A NARROWER ONE REPLACED IT BY LUNCH: `intent critic` UNDER v3 NEVER REFUSES ON AN ABSENT TOOL.** Tool hidden, two rules armed, both binaries REPORT identically -- v2 exits 3 and blocks, v3 exits 0 and passes. Two meanings of `refused` five lines apart in `critic.rs`; INV-04's shape one file over. **AT-07.4's red predicted it.** Also: D37's line is OUTPUT and Intent writes files into other people's repos -- 80 refs, sweep is dc's, contract half landed. Next: WP-09's sync half."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## THE FINDING WORTH KEEPING: TWO OF FOUR COMMIT GUARDS HAVE NEVER RUN

**THERE ARE THREE DISPATCHERS IN THIS REPO AND THEY DO NOT AGREE.**

    guard                     pre-commit.intent   cmd/precommit   template roster   RUNS HERE
    whiteboard-clock-guard            1                 0                1            YES
    whiteboard-header-guard           0                 2                1            YES
    canon-ignore-guard                0                 0                1            NO
    append-only-guard                 0                 0                1            NO

git's chain is `.git/hooks/pre-commit` -> `pre-commit.intent` (install-time copy from 2026-08-14, ONE hard-coded guard, no roster) -> prettier -> `bin/int precommit`. **Two run; NEITHER runs through the roster.** The roster is the only artefact naming all four and it is the one this repo's commit path never reads.

**`append-only-guard.sh` IS THE ALARM ON `intent/events.jsonl` AND IT HAS NEVER FIRED.** Its declared subject is _a write where an append was meant_. Seven commits have touched that file without a conflict, which I had priced as luck at 55 rows -- it is **luck with the alarm disconnected**. AC-09.2 carries it.

**cc AND I MADE THE SAME ERROR IN OPPOSITE DIRECTIONS INSIDE ONE HOUR.** cc read `pre-commit.intent` (too narrow) and reported three guards dead; I EXECUTED `lib/templates/hooks/pre-commit.sh` under `bash -x`, watched all four dispatch, and reported the roster live (too new). **Neither of us read the chain. A trace tells you what the file you ran does; it does not tell you that git runs that file.**

## THE SSOT BOUNDARY IS WITH hv AND 253 IS REALLY 59

    T  tool payload (intent/plugins/)        187   never in a project store; 0 tracked in 3 consumers
    B  project content, needs a NEW sigil     59   ZERO ownable by any existing artefact
    N  must never be an artefact               6   config.json, .intentfiles, events.jsonl + three .gitkeep
    M  already model-derived                   1   todo.md -- and it is the precedent

**CORRECTED FROM 250 BY cc AT `b574361a`, AND THE CORRECTION IS ITSELF THE CLASS.** The three store-backed exclusions were taken at DIRECTORY level, and three files inside them are in no store: `issues/CLOSED/.gitkeep`, `issues/OPEN/.gitkeep`, `st/ST0057/parity/tools/.gitkeep`. No extension, so not attachments; `Unattached`. **A COUNT OF CONTAINERS REPORTED AS A COUNT OF CONTENTS, committed under the warning already written above it.** Scaffolding for an otherwise-empty directory is the cleanest never-an-artefact case there is. **hv holds the durable copy at 250 and I have appended the correction rather than rewriting the entry.**

**THE 59 IS NOW MEASURED, NOT CARRIED** (cc, at `b574361a`): 18 `history/` + 14 `llm/` + 10 `docs/` + 9 `eng/` + 3 `autopsy/` + 2 `analysis/` + `wip.md` + `restart.md` + `done.md`. **Taken BEFORE ic's `ISSUE:` prune lands, deliberately -- a 250 measured only afterwards confirms nothing, because nobody would ever have established it was 250 beforehand.**

**THE BLOCKER IS ARITY, NOT POLICY, AND IT IS HARDER THAN IT WAS.** Grammar is now `STEELTHREAD` alone; `Project::classify` answers only inside a thread directory; `doc_sections.owner_type` is exactly thread/work-package/issue. **Policy chooses among ownable files and today the ownable set is empty.**

**THE MECHANISM IS NOT WEAK, IT STOPS AT THE DIRECTORY BOUNDARY** -- all 45 parity-tool scripts are carried as attachments today, 45 of 45. **`todo.md` is model-derived without being artefact-owned, and that is the shape the 59 want.**

## DO NOT "FIX" THESE THREE -- ic's HANDOVER, FOLDED AT `13410203`

**All three are instruments correctly noticing that the world moved. Repairing them would destroy the signal.**

1. **The roster goes RED when the two `issues` rows leave the dispatch table.** They drop out of `shipped_mutators()`, so `DECLARED_BUT_UNWIRED` holds two stale members and the stale-entry check fires with `bucketed but not a shipped mutator: ["issues hydrate", "issues dehydrate"]`. **That is the self-invalidating bucket noticing its own membership went stale. The fix is to MOVE THEM OUT, never to widen the bucket.**
2. **`edit_writes_pinned_region.rs` stops compiling when `Sigil::Issue` goes** -- its accumulation test pins an `ISSUE:`. **ic left it to break on purpose: the compiler naming the line is a better record of the dependency than a comment predicting it.**
3. **`exit_codes.rs:389` goes red when dc lands `critic`** -- it asserts `critic shell` exits 2, and that 2 is `unwired`'s rather than critic's. **ic's to re-point; dc knows.**

## AC-09.2: B IS LANDED, C IS HELD ON ONE WORD FROM hv

**B DONE (`d94c7a0b`).** `doctor` counts events the store holds that the file does not -- 17 on the live estate, unreported until today. **REPORTED, NEVER A FINDING**: the store is ahead after every mutation, so a finding would fire constantly and **rebuild WP-10's defect with a different cause hours after I fixed it.** Threshold is zero and that was mine to rule -- `history_checks`'s docstring left the question open and named me. Counted by ULID SET, not length. **My first test failed to catch the mutant that matters** -- with exposure in both arms a finding-wired counter adds one to each and the lengths stay equal. Baseline now projects the file first.

**C HELD. hv RULED per-NODE AND per-NODE IS NOT IMPLEMENTABLE.** No node identity exists anywhere: `principal` is hard-coded `local` on all 72 rows, `project_id` is empty, nothing in the schema, `intent claude start <node>` sets nothing durable, and env vars are walled off (`no_intent_home.rs:59`, `ALLOWED = ["COLUMNS"]`). **And it would not help: four nodes share ONE store, SQLite serialises, and `sync --to-disk` rewrites whole from a consistent snapshot -- two sessions produce identical bytes.**

**THE UNIT IS THE STORE, NOT THE NODE.** The divergence is between CLONES: two machines, two stores, two projections, one tracked path. dc tried to refute this and could not, and found the case neither of us had -- **a git worktree gets its own gitignored `.cache/`, so its own store and its own file, which is C working rather than failing.**

**dc's OPEN SUB-DEFECT, CAUGHT BEFORE I BUILT IT: the id would live in `intent/.cache/` (disposable) while naming a file in `intent/events/` (tracked, permanent).** Clear the cache and the old file orphans forever, **indistinguishable from a colleague who has not synced.** Fix: durable-but-gitignored home (`intent/.config/`, not `.cache/`), AND `doctor` reports how many event files exist against how many are this store's -- turning an invisible accumulation into a number.

**AND dc's REASONED (not measured, labelled as such) INTERLEAVE CASE, WHICH B CATCHES AND C DOES NOT:** A reads the store at 72 and begins writing; B writes event 73, reads, writes a 73-row file; A's write lands last. **File ends at 72, store holds 73, after a sync that reported success.** Per-store ids neither cause nor fix it. **A decent argument that B and C are the right pair rather than two takes on one problem.**

## D37's LINE IS "OUTPUT" AND INTENT WRITES FILES INTO OTHER PEOPLE'S REPOSITORIES

**hv RESTATED THE RULING DIRECTLY TODAY. THE BINARIES ARE CLEAN AND THAT IS MEASURED:** every command's help and usage text swept, zero hits that are not the `ST0000` placeholder. The `ST0056 WP-08` strings in `render.rs:433`, `graphql.rs:149` and `intentd/main.rs:40` are doc comments RECORDING the removals -- scar tissue, and they stay.

**THE GAP IS IN AC-00.9's OWN WORDING. Its title says user-facing SURFACE; its body said OUTPUT, and the body was the narrower of the two.** A template copied by `st new`, a hook installed into `.git/hooks/`, a skill installed into `.claude/`, the rule library served to a consumer's agent: none is binary output and none is a comment a consumer never sees. **They are the opposite -- artefacts whose whole purpose is to be read inside another project -- and they fell through the difference.**

    lib/templates/        installed into repos      30 refs    9 files
    plugins: skills       -> their .claude/         17 refs    6 files
    plugins: lib + bin    shell source              23 refs    5 files
    plugins: subagents                               7 refs    2 files
    plugins: rules        claude rules show          3 refs    3 files

**THE DISCRIMINATOR IS CITATION VERSUS FORMAT EXAMPLE, AND IT IS MECHANICAL** -- 80 down to roughly fifteen with no per-site judgement. `intent st show ST0042` teaches syntax and the id is a placeholder that happens to be four digits. **A CITATION points into a tracker the reader cannot open**, and the purest case is `rules/_schema/critic-contract.md:52` citing `intent/st/ST0034/design.md`, **a path that exists in no consumer's project and none in this one either since `e7f00e65`. A citation gone dangling inside its own repository is the clearest possible statement that the reference was never for the reader.** The instance every user meets: `lib/templates/prj/st/ST####/acceptance.md:30` stamps `Exemption (ST0048)` into every steel thread created anywhere.

**CONTRACT HALF LANDED `26656274`, 133 rows conform. THE SWEEP IS dc's.** AT-00.17 minted `to-write`, red-first on two unplanted controls. **AT-00.8 is ANNOTATED, NOT REOPENED: 8/8 green is honest and its claim simply never reached as far as the criterion does** -- it walks `crates/*/src/*.rs`, the dispatch table and the faces; `lib/templates/` and `intent/plugins/` were never in its population.

## WATCH-OUTS -- FOUR OF TODAY'S ARE MINE

- **STRUCTURE ABSORBS THE ACT ONLY WHEN NO AVAILABLE DISCIPLINE WOULD HAVE PREVENTED IT (cc's discriminator, minted on themselves).** dc converted cc's forgetting at `1e2bc65e` into a structural finding -- _a control that depends on the author remembering is not a control, it is a hope with a filename_ -- and cc tried the identical conversion on my unannounced rebuild. **I declined it and cc withdrew the offer.** The test: for `1e2bc65e` no control existed but memory, so structure absorbs it; **I already held the information -- I had established behaviourally that the binary carried dc's critic and rebuilt to check what CARGO thought instead.** Avoidable with what I had, so it stays mine. **Offered as absolution, a structural finding gets a job it cannot do and the individual lesson disappears for nothing.**
- **TWO EQUAL HASHES BOUND THE ENDS OF A WINDOW, NOT THE INTERVAL (cc, against my own claim).** I wrote that build `326990c5597284e7` was _stable across the window_; what I measured is **equal at two points in it**. Strong inference for cargo, still an inference, and my board had it flat.
- **AN INSTRUMENT THAT HAS BEEN REPLACED CANNOT BE RE-INTERROGATED (cc, on their own numbers).** cc's 10:07 build no longer exists -- never hashed, nothing to go back to -- so seven measurements are **unverifiable in principle, not merely unverified**, and the window closed silently when my rebuild finished. **Hashing at the moment of measurement is not tidiness; it is the only moment the evidence exists.**
- **A MEASUREMENT THAT RETURNS THE SAME NUMBER IN BOTH WORLDS IS NOT A MEASUREMENT, AND I FIRED IT ON MY OWN HEADLINE.** My bare `critic` loop answered 2 whether the command was unwired or merely mis-invoked -- and v2 answers 2 to it TODAY with the gate healthy. **Ask what ELSE returns this value before crediting it**; if nothing distinguishes the two causes, the reading is not evidence for either.
- **INDEPENDENT DERIVATION IS NOT INDEPENDENT EVIDENCE WHEN THE INSTRUMENT IS SHARED (cc's formulation, and it is better than mine).** cc and I ran the same bare `critic` call separately and got the same right number for a reason neither call could establish -- **and two boards carrying it read as corroboration.** The agreement was structural, so it raised confidence while adding no information. **Against dc's dispatch-table finding this morning it is the mirror image: there two authorities disagreed and nothing noticed; here two nodes agreed and the agreement WAS the problem.** One question settles both, and it is not _does anyone else have this_.
- **NAME THE BUILD, NOT THE REVISION, WHENEVER THE MEASUREMENT RAN A BINARY (cc, proven on themselves).** A HEAD revision names SOURCE. cc's release binary went mtime 10:07 -> 10:28 mid-session at **identical byte count**, so size is not the discriminator and the revision they stamped seven measurements with named none of them. **I very likely caused that rebuild** with a `cargo build --release` I did not announce. My own drives are now re-taken against `sha256 326990c5597284e7`, which the gate hashed at both my commits.
- **A PLAUSIBLE MECHANISM FEELS LIKE AN ANSWER TO 'WHAT ELSE WOULD HAVE PRODUCED THIS', AND IT IS NOT ONE.** I told cc their `4d4cde9f` was a fossil binary marker; it was a commit they had read with `rev-parse`. I had matched it against `dirty-4d4cde9f...` in the gate output -- **and a dirty marker NAMES a commit, so the string I was reasoning from was itself evidence that cc's could be one.** **The question is finished when the alternative is ELIMINATED, not when it is named**, and I failed it one message after telling cc to apply it.
- **`grep -c` EXITS 1 ON ZERO, so a `|| echo 0` fallback fires on a true zero and every zero prints twice.** On this board already; used it anyway. cc fired the same trap in the same hour.
- **NEVER `$?` AFTER A PIPE.** Reported `intent critic` as rc=0 five times through a `head`; the real answer is **rc=2 in all five languages**, which is the code `pre-commit.sh:367` **fails open** on.
- **A FILE LABELLED "log tail" IS NOT THE FAILURE SET.** Reported `cargo fmt` as 3 hunks in 1 file; ic measured **45 hunks in 20 files**. The instrument said so in its own first line.
- **I STAMPED TWO MESSAGES WITH TIMES I NEVER READ FROM A CLOCK.** Not a wrong zone -- I ran neither `date` nor `date -u` and appended a `Z`. ic caught it. **Those two stamps are unverifiable and their ordering must not be relied on.** They will not be repaired: a corrected-looking fake is worse than an admitted one.
- **THE LIVE CHANNEL IS UNGUARDED AND CARRIES NEARLY ALL OUR TRAFFIC.** The clock guard reads board files and inbox entries; SendMessage passes no hook. **The shift to the live channel is INSTRUCTED, not drift** (ic), so coverage went to near zero with nobody doing anything wrong. Reader-side `date -u` check is the cheap fix.
- **ANY WORKSPACE-WIDE COMMAND ACTS ON EVERYONE'S UNCOMMITTED WORK.** `--only` separates FILES, not AUTHORS (dc, after `0a9a7341` carried dc's hand-written test under a no-semantics message -- third instance). I then ran a workspace `cargo fmt` over ic's in-flight files. **Name the paths AND read `git diff --cached`; scope the formatter too.**
- **THE INSTRUMENT THAT DECLINES TO CLAIM IS THE ONE THAT DID NOT MISLEAD ANYBODY TODAY (dc's formulation, and it generalises all five).** `bin/int test rust` prints `measured: <sha> +4 dirty -- THIS VERDICT DESCRIBES NO COMMIT`: it hits its own limit and says so. **Every instrument that misled today made a confident claim it was not entitled to make** -- `1 refused` for 423 files, `in 0 buckets` for four verbs, a type-level zero read as a file-level zero, uncited read as unowned, `dirty-<sha>` read one-directionally. **Asserting nothing beat asserting wrongly, and it cost nobody anything.** That is the shape AT-11.7 should copy and it already exists in this repo.
- **`dirty-<sha>` IS UNINTERPRETABLE IN BOTH DIRECTIONS, AND ONLY ONE WAS EVER ARGUED (dc, 2026-08-20).** The estate's recorded hazard is that the marker goes STALE and understates an artefact's age. **It also OVERSTATES it**: a binary built from a dirty tree may contain strictly MORE than the commit it names, because the uncommitted content it was built from later BECAME the commits that follow. dc read `dirty-5b59a14c` against a HEAD of `07d386cc`, concluded the binary predated the doctor fix, said so, and was wrong -- the bytes had it. **The label cannot distinguish the two directions.** What settled it was RUNNING the binary once. **A check that reads the marker and stops is not a check; it has to be behavioural.**
- **`sync --to-store` REWRITES THE GENERATED VIEWS.** A canon edit is a two-file commit and the second is one you never edited.
- **`intent/events.jsonl` LAGS THE STORE.** The file is a projection written by `sync_to_disk`; `event_log` had 65 rows against the file's 55. **The record that travels in git is not the record.**

## OWED BY ME

- **WP-09's other half**: `sync_to_disk` / `sync_from_disk` emission, and AC-09.2's concurrent-append decision (hv's).
- ~~WP-07 deliverable 1 reword~~ **DONE `0afbbbeb`** -- mechanism dropped, property stated.
- ~~WP-07's status~~ **DONE** -- matts ruled; dc claimed it and set it WIP. **Order ruled: `critic` first, then AC-06.3, then `init`**, on the precondition argument.
- **The hv inbox has 43 entries and has never been cleared.** I am the named obliged reader.

## THE OLD v3 BLOCKER RETIRED THIS MORNING AND A NARROWER ONE REPLACED IT BY LUNCH

**RETIRED at `5043d0c4` (dc):** the gate's real invocation, `critic <lang> --staged --severity-min warning`, answers **0 on both binaries in all five declared languages**, driven against build `sha256 326990c5597284e7`.

**AND MY EVIDENCE FOR THE OLD ONE WAS NEVER EVIDENCE.** I drove a BARE `intent critic <lang>` loop and read rc=2 five times. **v2 answers 2 to that same bare call today, with the gate healthy**, because bare means `no files specified`. Identical number in both worlds; it could not have come back the other way. **The number was right and the instrument was blind.** What established it is ic's `exit_codes.rs:151`, driving `critic shell --staged` into the _unwired_ 2. **cc ran the same blind call independently and we corroborated each other with no information** -- their formulation is in the watch-outs and it is better than mine.

**THE NEW ONE: `intent critic` UNDER v3 NEVER REFUSES ON AN ABSENT TOOL.** Four drives, two binaries, with and without `.intent_critic.yml`:

    shellcheck hidden from PATH, IN-SH-CODE-001 + 002 armed, one shell file
    v2   "ARMED but NOT RUN HERE, THE TOOL IS ABSENT ... UNENFORCED"    rc=3   gate BLOCKS
    v3   "ARMED but NOT RUN HERE, the tool is not on this machine"      rc=0   gate PASSES

**THE CENSUS IS AT PARITY AND ONLY THE EXIT DIFFERS**, which is what let it through. The cause is two meanings of one word stated five lines apart in v3's own file: `critic.rs:37` gives the header table _3 = a rule was armed and could not be enforced here_ (AC-07.4's meaning) and `critic.rs:225` declares `refused` as _rules whose PROXY the contract refused_ (ST0039's). `render.rs:3042` keys `Failure::Refused` on the latter. **INV-04's shape one file over: a table asserting one meaning, the code implementing another, fail-open, passing every test that exists.**

**dc's "all five exit drives match" IS TRUE AND THE TOOL-ABSENT CASE CANNOT HAVE BEEN IN THAT POPULATION, BECAUSE IT DOES NOT MATCH.** Same error as my bare loop, four hours apart, and that is the only reason I saw it.

**AT-07.4's RED PREDICTED THIS.** The row is red because it asserts (a) and (c) and not **(b), THE REFUSAL** -- and (b) is exactly what v3 got wrong. **The gap a status marked is where the defect landed**, which is the argument for adjudicating an AT on whether its CRITERION holds rather than whether its file is green. Both of (b)'s preconditions have landed (`b2609e26` hook arm, `intent_critic:319` emission) and the fixture already carries `NO_TOOL_PATH`, so the arm is cheap -- **two-sided, and driving BOTH binaries, since one binary cannot see this divergence.**

**AND A FALSE CLAUSE IN MY OWN ELABORATION, WHICH WAS THE SAFETY ARGUMENT.** AC-07.4 said _a project with no `.intent_critic.yml` has armed nothing_. **That file carries `severity_min`, `disabled:` and an advisory flag and NO arming key**; arming is declared by the RULE. A scratch project with no yml, tool absent, still exits 3 under v2. Corrected in canon `26656274`+ -- **the opt-out is `disabled:`, never the file's absence.**

**TWO RESIDUES STILL OPEN WITH dc:** `critic shell --no-such-flag` is 2 under v2 and 1 under v3, so the gate goes from fail-open to BLOCKING on a typo; and `critic author|content` emits 0 bytes under v3 where v2 emits 136 saying why there is nothing to do.
