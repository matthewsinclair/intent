---
node: dc
name: DevX Claude
role: worker
session_id: f396ca31-ec6f-459e-9b7c-40e87aa93efb
heartbeat_at: 2026-08-19 20:23Z
status: active
focus: "**WP-06 IS THE ARMING BLOCK: four of the six unmet dehydration preconditions are mine, AC-06.1 and AC-06.2 are landed.** Next is AC-06.3 (the third `Projection` variant -- small, fully ruled by vc) then AC-06.4 (`intent init`, the large one). **Today: the `organize` polarity (preview by default, `--apply` performs, one body taking a `Mode`), the text realisation, the `organise` hidden alias, six surface rows for hv's manual override, and the seventh reclaim site where a shipped verb told users to delete it from their scripts.** Board folded aggressively -- 53 decisions and 43 watch-outs archived to `.history/20260819/`."
claims: [ST0056/11, ST0057/04, ST0057/06]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.** hv ruled it four times and it was reinterpreted after three of them, twice by me inside ten minutes.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** Asking SQLite and writing the answer is still writing a time you obtained.
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE. Read-then-write leaves a gap two writers interleave in.
- **hv's structural close: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times. **Direction is not symmetric -- IN is forbidden, OUT is fine.**
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES, WHICH IS WHY THIS VERSION HOLDS.** A time-typed input parameter is a defect by inspection. Asking where a caller got a value is a discussion; asking whether a signature accepts one is a grep.
- **The defect is always one step earlier.** Reaching for a clock means you are about to write a time into something that is not a durable record.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it's just a label". The third fooled cc, vc and me independently.
- **SCOPE, hv: devbin is NOT Intent** -- external, vendored, no db. My whole D42 directive resolved to a no-op, and reporting that beat inventing work to look responsive.
- **A board stamp is a label, not data.** The ordering that exists and cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. Three points that change what I DO: **the db is the durable SSOT, files are re-creatable**; **the typed API is the only door in**; **migrations are normal.**

## DOING

**WP-06 (ST0057) -- THE ARMING BLOCK. Four of the six unmet dehydration preconditions are AC-06.1..06.4 and all four are mine.** vc owns the running total; I no longer infer it.

- **AC-06.1 + AC-06.2 LANDED** (`c840251a`). `src/realise.rs` + `tests/text_realisation.rs`, 7 tests, 2 mutations killed. Denominator asked of CANON while the numerator accumulates while writing, so the two numbers come from different places; the mutation making them one kills three tests. No time in any signature (D42) -- the facade derives the directory from `append_event`'s `RETURNING ts`.
- **AC-06.3 NEXT, and it is NOT a flag flip.** vc ruled (PROVISIONAL-PENDING-hv): _sparseness lands_ = the capability and its fallback exist, ie WP-06 itself. Needs a **third `Projection` variant** -- `md` cannot be `RoundTrips` (AC-06.2 forbids the reader) and cannot stay `Lossy` (that is the refusal). **The `because` clause survives verbatim; only `instead` goes.** md stops being REFUSED and stays non-authoritative.
- **AC-06.4 AFTER IT.** `intent init` from an empty directory. `init` is unimplemented and AC-06.4 makes it a PRECONDITION of the assurance, not a neighbouring gap.

## TODO

1. **AC-06.3, then AC-06.4.** In that order: 06.3 is small and fully ruled, 06.4 is a real build.
2. **`hydrate` ROW LANDED, `dehydrate` HELD** (`420794fc`). ic builds the manifest negative (WP-02); the row lands when the grammar does. **Declaring both together would put a verb in `--help`, the guide and the MCP list ahead of code that could honour it.**
3. **AWAITING vc: does `st done` WARN or GATE on unsynced attachments?** I withdrew my data-loss concern -- `organize.rs:695` is the only line that deletes an estate file and its per-file gate already refuses anything it cannot reproduce. **The live question is Highlander: a second gate would be a divergent copy; a pre-flight WARNING at the moment of closing is additive and better timed.** My `side_effect_ruled` note in `420794fc` states it as REQUIRED and currently describes a protection that does not exist -- correct it when vc rules.
4. **OWED TO ic:** hv's later spec (`intent st hydrate` + `intent issues hydrate`, per-family) SUPERSEDES their top-level `intent hydrate <address>` ruling, which they marked PROVISIONAL-PENDING-hv. I authored to hv's spelling and they do not know yet.
5. **`924d556b` CARRIES ic's WP-02 UNDER MY COMMIT MESSAGE.** Not amended -- rewriting a tip four nodes commit into is theirs to ask for.
6. **Standing, unstarted:** `canon_commit_check.sh`'s missing arm (bytes with no canon record) + its `--staged` mode (cc built it, I reviewed and approved, two findings returned); `output-contracts.md`; `doctor` v3 mirror (XS); the re-cut's `--skip-rust-tests` and Half B's shellcheck/clippy design question.

## Watch-outs

**Consolidated into classes at the 2026-08-19 localfold. Every instance is kept verbatim in `.history/20260819/watch-outs-full.md`** -- a class is what you remember, an instance is what you recognise, and the instances are why the classes are believable.

- **THE PROBE'S POPULATION CAN EXCLUDE THE ANSWER, AND IT NEVER SAYS SO.** `cargo test` stops at the first failing binary (60 of 120 binaries, measured); `git log -3`; a `grep -oE` alternation of the two values you expect; `cargo clippy --release` skips test targets; a single `pgrep` sample. **Always ask what the probe could not have seen.**
- **THE EXIT CODE IS NOT WHERE YOU THINK.** A pipe eats it (`$?` is the LAST stage); `grep -c` exits 1 on zero; a background task's "exit code 0" is the whole chain's. **`$pipestatus` in zsh, or redirect and read.**
- **A CLAIM ABOUT A MUTABLE SUBJECT MUST NAME ITS REVISION, AND A PIN PROVES IDENTITY NOT CURRENCY.** I cited `intent_acceptance:252` all day; the line was 500. Same class as a stale hash matching its record exactly while the subject has moved.
- **A DOC COMMENT IS A CLAIM BY THE PERSON LEAST ABLE TO AUDIT IT.** `retired_refusal` opened by asserting an invariant it did not have. **My `FacadeError::Intentfiles` comment asserted a doubling that could not happen -- the format string had no `{0}` -- and I wrote it to justify silencing a warning.** Read the code, not the sentence above it. Three instances in one day.
- **A COMPILER WARNING ON THE PATH YOU JUST TOUCHED IS A FINDING, NOT NOISE.** `unused variable: cause` WAS the defect: the cause reached nothing. Underscoring it moved the defect and silenced the reporter.
- **THIS TREE HAS FOUR WRITERS AND THE TOOLS ASSUME ONE.** `--only <path>` commits the working-tree state of that path, so it separates FILES, not authors inside one file -- it never protected against a peer. `git stash push` with a bad pathspec creates nothing and the next `pop` takes someone else's stash; `git checkout --` and `git reset --hard` exceed what they were aimed at. **`bin/**` is live on PATH through a symlink, so a broken file here breaks every project on the machine.**
- **RUN `cargo build --workspace --tests` BEFORE EVERY COMMIT.** I broke HEAD twice in one evening (`FacadeError::Realise` with no arm; `Entry::hidden_aliases` with no test ctor) because I committed on targeted greens. **The full-suite failure set is genuinely unreliable on a shared tree -- disjoint across two runs an hour apart -- but that argues for the BUILD check, not for skipping verification.**
- **PRESENCE IDENTIFIES A FILE AND NEVER ITS AUTHOR.** Four misattributions in one day, every one inferring from recent activity. **The AT row that cites a file names its owner; `git status` structurally cannot, because a file is untracked exactly while it is in flight.**
- **STANDING CONSTRAINTS.** Push `local` ONLY -- `upstream` frozen at `5765c5da` (hv). NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. Never mutate a file in place while anything runs it -- atomic replace. Commit before any `intent at` status change. Timestamps are READ FROM `date -u`, never written from memory; `git log` prints LOCAL time.
- **ENVIRONMENT.** My shell is **zsh** -- an unquoted `$var` does NOT word-split. `bash -n` is not a syntax check for `.bats`. `cp` onto a symlink writes through to the target. The markdown formatter is a second writer and is not a peer.

## Decisions

**53 entries from 2026-08-19 archived to `.history/20260819/decisions.md`.** Kept here: the ones that govern what the next session does.

- (2026-08-19) **`intent organize` PREVIEWS BY DEFAULT AND `--apply` PERFORMS** (ic, AC-05.1). **Preview and apply are ONE BODY taking a `Mode`** -- a preview computed by its own path is a promise about what the other path would do, and the two drift where nobody tests. `TreeMoved` is apply-only. A preview carrying refusals exits 0; only `--apply` moves the code, because every preview on this estate carries one while preconditions are unmet.
- (2026-08-19) **A CHECK IS A CLAIM ABOUT A REACHABLE STATE; A CONSTRAINT REMOVES THE STATE** (generalising vc's _a lint reports a state the system can hold; a primary key means it cannot_). `Verdict`'s private fields and single constructor are the same move in a different medium. **This is the argument that decides `--keep`'s opt-out polarity is defensible where `organize`'s would not be.**
- (2026-08-19) **THE TEST FOR ACTING ON A RELAYED INSTRUCTION IS REVERSIBILITY AND DIRECTION, NOT THE PRESENCE OF A CANON RECORD** (vc, ratifying). The `upstream` push was outward-facing, irreversible, and a PERMISSION. `organise` was additive, internal, one commit to undo. **Same evidentiary standing, opposite risk profile, opposite correct answer.**
- (2026-08-19) **FIXING THE GENERAL SHAPE HELD A CASE THAT DID NOT EXIST YET.** `retired_refusal` walks the BUILT SURFACE token by token rather than matching the first token -- **the only reason hv's evening `organise` ruling did not silently un-retire `st organise` eight hours later.**
- (2026-08-19) **A ROW DECLARING A VERB THE MODEL CANNOT EXPRESS IS A ROW THAT LIES.** The table feeds clap, `--help`, the guide and the MCP tool list. **Table-leads-reader is right when the behaviour exists and only the surface lags; reversed, it ships a promise.**
- (2026-08-19) **ASK THE OWNER, NEVER RECONSTRUCT FROM THE WORK DONE.** `hydrate` returned six paths then four for the same tree because it derived the answer from its own steps; `realise`'s denominator is asked of canon for the same reason. **A number reconstructed from what happened answers a different question from the one asked.**
