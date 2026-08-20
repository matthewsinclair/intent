---
node: vc
name: Validation Claude
role: validation
session_id: b8e50395-2c15-45b8-800b-d97acece15c5
heartbeat_at: 2026-08-20 09:17Z
status: active
focus: "**LOCALFOLDED FOR COMPACT. Status stays ACTIVE -- a compact does not end a session.** WP-10 done and doctor went 235 findings at rc=1 to ZERO; WP-09 half done (organize + hydrate record what they did to the disk) plus B (doctor counts history held only by this store). **C IS HELD ON ONE WORD FROM hv: per-node is not implementable and the unit is the STORE.** Two of four commit guards still never run."
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

## THE SSOT BOUNDARY IS WITH hv AND 250 IS REALLY 59

    T  tool payload (intent/plugins/)        187   never in a project store; 0 tracked in 3 consumers
    B  project content, needs a NEW sigil     59   ZERO ownable by any existing artefact
    N  must never be an artefact               3   config.json, .intentfiles, events.jsonl
    M  already model-derived                   1   todo.md -- and it is the precedent

**THE BLOCKER IS ARITY, NOT POLICY.** Grammar is `STEELTHREAD | ISSUE`; `Project::classify` answers only inside a thread directory; `doc_sections.owner_type` is exactly thread/work-package/issue. **Policy chooses among ownable files and today the ownable set is empty.**

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

## WATCH-OUTS -- FOUR OF TODAY'S ARE MINE

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

## THE BLOCKER FOR "INTENT FULLY WORKING ON v3"

**`intent critic` answers rc=2 in all five declared languages, and `pre-commit.sh:367` FAILS OPEN on that code.** So v3 on PATH silently disables the critic gate here and in fifteen other projects through one symlink. **That is why the standing no-v3-on-PATH rule exists, and it makes `critic` a PRECONDITION of full self-hosting rather than merely its biggest row.** dc found it; I verified it end to end. It lives in ST0056/WP-07.
