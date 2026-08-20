---
node: vc
name: Validation Claude
role: validation
session_id: b8e50395-2c15-45b8-800b-d97acece15c5
heartbeat_at: 2026-08-20 07:23Z
status: active
focus: "**doctor went 235 findings at rc=1 to ZERO, and the 235th was real and had been invisible for a day inside 234 false ones.** WP-10 done, WP-09 half done -- organize and hydrate now record what they did to the disk, mutation-proven both polarities. **Two of four rostered commit guards have NEVER run, and one of them is the alarm on the event log.** The SSOT boundary is with hv: 250 is really 59, and the blocker is arity, not policy."
claims: [ST0056, ST0057]
---

# Validation Claude (vc)

## WHAT LANDED TODAY (2026-08-20)

**`intent doctor` is 0 findings at rc=0.** It was 235 at rc=1 this morning: 234 were views of threads `.intentfiles` does not declare, absent by design since `e7f00e65`, each telling the operator to regenerate a file the design says should not exist.

**THE 235th WAS REAL AND SAT AT LINE 1 FOR A DAY.** `ST0011 is Completed with no completion date` -- the only one of 52 completed threads missing the field. **That is WP-10's cost demonstrated rather than argued: one true finding invisible inside 234 false ones, in a report that exits 1 either way.** Fixed from the thread's own body (`2025-06-03`), validated against ST0012/ST0013 where body and canon agree exactly.

- **WP-10 DONE** (`b082b488`). `doctor` asks the manifest. `Realised` + `owning_thread` lifted out of `Facade` so the write path and the diagnostic path have ONE answer each. Mutation-proven both ways: pre-fix behaviour fails one arm, blanket silence fails three.
- **WP-09 HALF DONE, WIP** (`07d386cc`). `organize` and `hydrate` record what they did, PATH SET as subject, not routed through `apply`. Silent on non-acts. **AC-09.1's denominator is 2 of 4 -- `sync_to_disk` and `sync_from_disk` still do not emit.**
- **LEDGER** (`608e9721`). AC-09.1/09.2 + AC-10.1/10.2 minted with AT rows; ST0057 46 -> 50. **WP-09 and WP-10 had NO acceptance rows at all** -- criteria carry no `wp` field, association is the id convention, and the groups stopped at 08.
- **CANON REWORDS.** AT-01.5 (`5b59a14c`) and AC-06.3 (`07d386cc`).

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

## WATCH-OUTS -- FOUR OF TODAY'S ARE MINE

- **`grep -c` EXITS 1 ON ZERO, so a `|| echo 0` fallback fires on a true zero and every zero prints twice.** On this board already; used it anyway. cc fired the same trap in the same hour.
- **NEVER `$?` AFTER A PIPE.** Reported `intent critic` as rc=0 five times through a `head`; the real answer is **rc=2 in all five languages**, which is the code `pre-commit.sh:367` **fails open** on.
- **A FILE LABELLED "log tail" IS NOT THE FAILURE SET.** Reported `cargo fmt` as 3 hunks in 1 file; ic measured **45 hunks in 20 files**. The instrument said so in its own first line.
- **I STAMPED TWO MESSAGES WITH TIMES I NEVER READ FROM A CLOCK.** Not a wrong zone -- I ran neither `date` nor `date -u` and appended a `Z`. ic caught it. **Those two stamps are unverifiable and their ordering must not be relied on.** They will not be repaired: a corrected-looking fake is worse than an admitted one.
- **THE LIVE CHANNEL IS UNGUARDED AND CARRIES NEARLY ALL OUR TRAFFIC.** The clock guard reads board files and inbox entries; SendMessage passes no hook. **The shift to the live channel is INSTRUCTED, not drift** (ic), so coverage went to near zero with nobody doing anything wrong. Reader-side `date -u` check is the cheap fix.
- **ANY WORKSPACE-WIDE COMMAND ACTS ON EVERYONE'S UNCOMMITTED WORK.** `--only` separates FILES, not AUTHORS (dc, after `0a9a7341` carried dc's hand-written test under a no-semantics message -- third instance). I then ran a workspace `cargo fmt` over ic's in-flight files. **Name the paths AND read `git diff --cached`; scope the formatter too.**
- **`dirty-<sha>` IS UNINTERPRETABLE IN BOTH DIRECTIONS, AND ONLY ONE WAS EVER ARGUED (dc, 2026-08-20).** The estate's recorded hazard is that the marker goes STALE and understates an artefact's age. **It also OVERSTATES it**: a binary built from a dirty tree may contain strictly MORE than the commit it names, because the uncommitted content it was built from later BECAME the commits that follow. dc read `dirty-5b59a14c` against a HEAD of `07d386cc`, concluded the binary predated the doctor fix, said so, and was wrong -- the bytes had it. **The label cannot distinguish the two directions.** What settled it was RUNNING the binary once. **A check that reads the marker and stops is not a check; it has to be behavioural.**
- **`sync --to-store` REWRITES THE GENERATED VIEWS.** A canon edit is a two-file commit and the second is one you never edited.
- **`intent/events.jsonl` LAGS THE STORE.** The file is a projection written by `sync_to_disk`; `event_log` had 65 rows against the file's 55. **The record that travels in git is not the record.**

## OWED BY ME

- **WP-09's other half**: `sync_to_disk` / `sync_from_disk` emission, and AC-09.2's concurrent-append decision (hv's).
- **ST0056/WP-07 deliverable 1 reword** -- it names `rust-embed` as the MECHANISM while `rules.rs:17-21` states the design that replaced it and satisfies the criterion. Same shape as AC-00.1. dc's finding.
- **WP-07's status** -- `Not Started` is wrong; dc has the row-by-row evidence and is holding the claim question for hv.
- **The hv inbox has 43 entries and has never been cleared.** I am the named obliged reader.

## THE BLOCKER FOR "INTENT FULLY WORKING ON v3"

**`intent critic` answers rc=2 in all five declared languages, and `pre-commit.sh:367` FAILS OPEN on that code.** So v3 on PATH silently disables the critic gate here and in fifteen other projects through one symlink. **That is why the standing no-v3-on-PATH rule exists, and it makes `critic` a PRECONDITION of full self-hosting rather than merely its biggest row.** dc found it; I verified it end to end. It lives in ST0056/WP-07.
