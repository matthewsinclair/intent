# Why Claudes yak-shave, and a plan to stop it

hv, 2026-11-11: "I want to know _why_ Claudes keep ending up in this yak-shaving singularity NONSENSE. Again and again. ... We need to DRASTICALLY improve release performance of Intent projects."

This document is the diagnosis and the proposed plan, on the method of Devbin's ST0011: read-only evidence first, then a diagnosis with each mechanism and its measurement, then options ranked by what they save, and hv's ruling before any change. Nothing here has been built.

## The evidence

Three read-only reads, made on 2026-11-11 after 3.3.0 was cut:

- The instruction corpus every seat loads or can load: the global and project instruction files, the installed skills, the team terms and seats, the shared memory directory, the whiteboard README and boards.
- The four Intent seats' transcripts for 2026-11-10 and 2026-11-11, classified by tool and command: 28.9 seat-hours of active time. The classifier was hand-checked on 30 calls and misread 3, all on the build/read boundary.
- The five runs of the 3.3.0 cut, earlier cuts' notes, CI job timings and Devbin's ST0011.

## The diagnosis

### 1. The memory is a ratchet that only tightens

- 200 notes in the shared memory directory. Check terms (verify, measure, prove, judge, census, re-run, guard) appear 1,401 times; stop terms (one layer, do not repeat, minimal, stop) 67 times. 178 of the 200 notes push toward a further check.
- 188 of them cite a dated incident: one incident becomes one permanent rule. 165 were written after 2026-09-01, 15 on 2026-11-10 alone. One note has ever been marked superseded. The directory is not in git, so nothing records a removal.
- The directory is shared by every seat, so one seat's post-mortem becomes every seat's procedure.
- The notes that read as brakes ask for more checking: "stop at the first sufficient explanation" says to enumerate every commit and dirty file in the window; "careful measurement of an unasked question" says "the corrective is NOT to check less".
- The index loads in every session: 19 KB, 198 links, 34 that name a check and 3 that brake.

### 2. hv's rule reached the canon and not the sessions

- hv's one-layer-of-proof rule is in the shipped TERMS.md, seats.json and the canon in-whiteboard skill as of 3.3.0.
- The installed skills are older: the installed in-whiteboard still says a done claim is "the trigger to verify", and the installed in-verify ("No exceptions ... Run again ... Verify each step") is chained from in-finish, so it fires at every localfold, which is before every compact.
- About 15 loaded or index-linked places still order checks hv has since forbidden: tree-equality at landing, whole-suite judges, a separate judge run for a fix hv asked for, a new guard driven over every estate before READY. The whiteboard README still defined vc as "the independent check ... correct, complete, consistent, faithful".

### 3. The validator is paid in findings

- vc is "Validation Claude", role `validation`; the skill defines a validation node by its output, "posts findings"; vc "judges and claims nothing", so a judgement is the only work it can show. No remit has a stop condition.
- Measured: vc ran 17 full judges of about 22 minutes each in the one cargo slot. 12 came back green and re-proved what the builder had already shown green (at least 225 slot-minutes). 4 found reds, and 1 died on vc's own wrapper.
- While those judges held the slot, builders waited 881 minutes for a grant (cc 650).
- Turnaround of single fixes: 0667 took 71 minutes for about 3 minutes of editing; 0678 took 67 minutes, 33 of them a second proof after ic had run its whole suite twice; 0680 took 3h40m with grant waits of 81, 40 and 65 minutes.

### 4. Coordination is the largest single cost

- Of active seat time: coordination 39%, reading and investigating 23%, building 19%, re-verification 9%, session overhead 6%, wait loops 4%, talking to hv 1%. For vc alone, coordination was 43% to 54%.
- 1,899 inter-seat messages, 1.17 million characters, median 373. 57% were CHAIN START or CHAIN END broadcasts.
- 143 of the 270 commits on main were board, fold or pickup commits. The census ran 453 times.
- vc wrote 180,000 characters of narration in turns hv did not start, and hv still asked "where are we up to?" 11 times.
- Idle time dwarfs active time: 68.6 seat-hours waiting on another seat, 18.8 on hv, 7.8 on a run or a grant. The longest stall was 6h52m overnight, when the desk had no waiter armed for new asks.

### 5. Proof was spent in the wrong place

- All the second-proof effort went to code that was already green. The one path never run, the kovacs leg declared on 2026-11-10, got its first run during hv's cut and failed four ways, one per run: no previous tag, a tmpfs /tmp too small, a shared daemon socket under ssh, no release pair in the leg's tree. 1h34m of the 1h56m from first attempt to release was lost to those runs and the gaps between them.
- The clean cut itself is 22m07s, down from 61m44s at 3.2.8, matching Devbin ST0011's expected figure.

### 6. The release pipeline still repeats itself

- On the shipping commit, `cargo test`, clippy with doc, and bats each ran four times (local Mac, kovacs, CI macOS, CI Ubuntu). The release pair was built 8 times, 3 of which ship; locally it was built 3 times from the same sources.
- `gate rust` uses a fresh private worktree with an empty target, so it is cold on every cut on both hosts. bats runs serially.
- The critical path is CI's macOS test job, 13m22s: a 4m57s release build, then 7m21s of serial bats.
- `cut publish` after the tag takes about 5m20s: a third build of the pair, xcodebuild, and two notary submissions one after the other.

### The common root

Each cause above is the same shape: a check is added after an incident, and nothing removes it, prices it, or asks what it catches that the check before it did not. The memory, the skills, the validator's role, the chain protocol and the pipeline all grew that way. The tooling already keys nothing on "this commit was proven green"; the prompts key nothing on "this check has already been done".

## The plan

Two halves, ranked inside each by what they save. Every item cuts both time and complexity, which is Devbin ST0011's criterion as hv set it.

### A. The prompts, the memory and the roles

1. **Prune the memory to durable facts.** One pass, by vc with hv's review: keep environment facts, pointers and hv's standing rules; delete incident procedures, check rules and anything a tool now enforces. A ceiling: the index under 5 KB and notes under 50. New check rules go in only with hv's word. Saves: every seat's start-up load and every check the notes order.
2. **Sync the installed skills to 3.3.0 canon, and take in-verify out of in-finish's chain.** `intent claude skills sync` on every machine. Saves: a verify pass before every compact.
3. **vc stops judging.** vc reads the builder's evidence and lands; it runs no suite. A second run only when vc can name the fault the first could not have seen, which is already in TERMS. Rename the role's output from findings to throughput: vc is measured by time from a builder's green to main. Saves: about 225 slot-minutes and 881 minutes of builder waits in two days.
4. **Drop the chain broadcasts.** Each seat commits by literal path with `--only`; git's index lock already serialises commits. Pickups and heartbeats are not committed on their own; a seat's board files ride its next real commit or its fold. Saves: 57% of messages and about half of the commits on main.
5. **Short messages, by rule.** A message to a seat is at most five lines; a report to hv answers the question asked in at most ten. A status hv has to ask for is the failure.
6. **One check rule removed for each one added.** A standing rule in TERMS: before any new guard, test or check, name what it catches that the checks before it do not; a check that cannot name it is not added.

### B. The release pipeline

These land in devbin's core, for every project.

1. **Rehearse a new leg before the cut.** A leg added or changed since the last release runs `release gates` on its host before the cut is offered. Saves: the 1h34m lost today.
2. **Parallel bats.** Shard or `--jobs` in every leg; bats is on the critical path of all three.
3. **Warm gate trees.** A fixed gate-worktree path per host, or devbin's input-hash cache (Devbin ST0011 WP16), so `gate rust` is not cold every cut. Intent's own `cmd/cache` collides with devbin's `cache` verb and is settled first.
4. **Build the pair once.** Built once at the release commit, marked as a release, and reused by the gates and `cut publish`.
5. **Sign and notarise before the tag**, as a macOS leg, and submit the two notarisations together. Saves: most of the 5-minute tail after the tag.
6. **One release build per OS in CI**, shared by its jobs.

### Targets

- A cut that goes through on its first run, in under 20 minutes from the yes to the tap.
- A single fix from first edit to main in under 30 minutes, with one run.
- Coordination under 15% of active seat time; no status question from hv that a seat should have answered first.
- Measured the same way as this diagnosis, after the next release.

## What hv rules

- Whether this plan, or which parts of it, become work packages.
- Half A changes how every seat works, across the estate. Half B is devbin's core, so it is put to devbin-vc as a brief.
- The memory prune deletes notes, which are shared across every seat on every machine, so it runs only on hv's word and with hv reading the list first.
