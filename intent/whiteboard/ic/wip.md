---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 16:34Z
status: active
focus: "EXP-05 is RULED and ADOPTED -- author the flag `disposition` field, the refusal, and the classification. doctor: --fix is retire, --verbose/--quiet are pending (vc measured them)."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**EXP-05 is RULED and ADOPTED as proposed** (vc, 16:10Z). Three parts, exactly the EXP-03 shape:

1. **Author `disposition` on every flag**, in the vocabulary entries already use: **`keep`** ships and must be read; **`retire`** is recorded from v2 and never reaches clap; **`pending`** does not ship.
2. **Author the refusal**: every flag declares one or the run refuses -- so a flag cannot join the surface by being typed.
3. **Classify ~130 flags**, marking uncertainty the way EXP-03 did. vc will REVIEW, and review is anchored by the proposal.

**vc MEASURED my two sub-questions rather than returning them, so do not re-open them.** `bin/intent_doctor` really implements both -- `verbose()` at `:133`, colour suppression under quiet at `:91` -- so **`--verbose` and `--quiet` are `pending`, NOT `retire`**. And **`--fix` is `retire`**: v2 implements it at `:66`, so it is a genuine v2 behaviour we are deliberately not carrying, which is the distinction `retire` exists to draw and the thing AC-06.9 was missing.

**`pending` does NOT refuse the build** -- ruled against the stricter option, and the reasoning is the one that cost all four of us commits this afternoon: a guard that must be bypassed is a guard nobody keeps. **The quiet-absence risk is answered elsewhere: `doctor` reports the pending count.**

**Why this could not wait for the long tail:** 2 present violations and 44 declared-and-unread flags on commands with no renderer arm yet. Those arrive **one at a time as each command is wired** -- never a batch anyone confronts, each landing inside a commit about something else.

## TODO

1. **The agent guide's AUTHORED half (AC-09.4)**, when the v3 workflows settle. Spec written (`surface/agent-guide.spec.md`), control built and mutation-tested (`parity/tools/guide_refs_check.sh`). **Waiting on vc: does the authored half stay one file carrying `usage-rules.md`'s dual role, or split?** The measurement argues for splitting -- a document serving two readers was maintained for one of them.
2. **`gen_pertest.sh` and `gen_register.sh` still read `date -u`, and `measured_on` is an authored date in the canon.** Raise under D42. NOT fixed: those two generators cannot be re-run (their burn inputs are gone), so editing them strands their committed outputs -- **a generator I cannot run is one I cannot honestly edit.** `measured_on` needs the generator's refusal changed too, so it is a contract call.
3. **`gen_inventory.sh` still execs `$SP/extract_verbs.sh`**, so the tools must be COPIED into a scratch dir before it runs. A large part of why nobody re-ran it for a day.

## Open with others

1. **vc:** the `sync --to-store` vs `ingest` boundary is still undeclared, and `sync` is flagged for MCP review precisely because that boundary decides whether it stays exposed.
2. **cc:** the spine must honour the flag `disposition` once I land it. `--fix` on `doctor` is the mechanism's first user.
3. **D42 audit is DONE and reported** (`23319185`): the 27 inventories are clean -- v2 declares no time-bearing flag anywhere -- and all three findings are in the NEW surface. Flagged in place as `target.d42_exposure` on `todo done`, `doctor` and `backup`, plus EXP-06.

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it.

- **THERE IS NO CLOCK. D42, ratified four times: DB records have a timestamp field, that is the source of truth for time, nothing else, ever.** You never ask what time it is -- not the OS, not `date`, not the filesystem, **and not the store either**, because a read and a write are two acts with a gap, and two writers interleave in it. **The four things that are NOT exceptions: a test fixture; "I am only reading it"; "it came from the database"; "it is just a board label".** If you are about to write a time, the defect is one step earlier -- you are writing a time into something that is not a durable record.
- **AND D42 IS ASYMMETRIC -- I OVER-APPLIED IT WITHIN THE HOUR.** hv: no CLI or intentsvcs function **TAKES** a time; functions that **RETURN** times are fine, because those values went end-to-end through the DB where SQLite set them. **Hunt for a surface that must OBTAIN an instant or a duration; never for one that displays a stamp.** My EXP-06 said "takes or emits" and flagged `backup --list` for the act of emitting -- a permitted surface. Reading it the strict way would withdraw `--list` and any `created`/`completed` a `show` command prints, **which are exactly the surfaces D42 exists to make trustworthy rather than remove.** The rule takes something from the WRITE path and gives the READ path its guarantee.
- **A CHECK THAT CANNOT FAIL IS NOT A WEAK CHECK, IT IS A DECORATION -- and it hands you a reassuring result first.** Four instances today: the invariant-orphan check that scanned its own `id`; a comparison printing a clean 26/26 while every normaliser call had failed, so `diff` compared two EMPTY streams; a mutation harness scoring four kills that all died on an unrelated path error; a global read-set counting `--fix` as read for `doctor` because `at lint` reads it. **Assert the fixture reached the branch, and that both sides are non-empty, BEFORE believing a result.**
- **ENUMERATE THE POPULATION; DO NOT SNIFF FOR A MARKER -- and a structured query is a needle too.** `jq '.families[].entries[]'` missed a whole top-level array; a banner sniff would have covered 1 file in 30; the D42 audit's first pattern matched `manage`/`validate`/`update` on substrings of `age` and `date` and reported 25 rows. **Word boundaries cut it to five, of which three were real.**
- **"IT DOES NOT EXIST" IS A CLAIM ABOUT THE FILESYSTEM, SO GO AND LOOK.** I concluded the probe input was gone from `git log --all` being empty -- which answers "was this ever committed". It was on disk the whole time, and a rule in parity.md and a refusal in `gen_inventory.sh` were built on top. **One `find` beat all of it.** vc's form covers both their `git grep` miss and mine: **git answers questions about HISTORY; if the question contains "exists", the answer comes from the filesystem.**
- **A SWEEP DOES NOT MOVE A FILE, IT SPLITS A CHANGE.** My unqualified `--amend` took cc's test without the `store.rs` methods it calls, and **HEAD did not build for ten minutes**. Each half reads as finished alone, so there is nothing file-shaped to notice. **After a sweep the question is not "whose file is this", it is "does it still build".** And I reported "no data damage" from `git show --stat` -- **a stat says which files moved and cannot say whether the tree compiles.**
- **`--only` PROTECTS THE COMMIT AND NOT THE AMEND.** `git commit --amend` with no pathspec re-commits the whole shared index. And **`--only` never CLEARS the index**, so the shared index accumulates across every node -- the pile was loaded by four of us and tripped by one. **Name paths on the amend too.**
- **A SKIP LIST IS A PROMISE THAT SOMETHING ELSE RENDERS THE KEY.** Mine exempted four keys nothing rendered; `kind` on `st` had been invisible in the view for a day. **Reading the list produced the bad list; the mutation test found it.**
- **RE-DERIVABILITY IS NOT COMPLETENESS.** A lossy generator is a perfect fixed point with itself, so skew passes forever. It hid 15 of 20 authored fields.
- **A MISSING MEASUREMENT MUST PRESENT AS A REFUSAL TO MEASURE, NEVER AS A MEASUREMENT OF NOTHING.**
- **A QUOTE CHARACTER INSIDE A QUOTING CONTEXT, IN PROSE NOBODY PROOF-READS FOR SYNTAX.** Backticks in a double-quoted string are command substitution; an apostrophe in a single-quoted one CLOSES it (`vc's` inside `JQ_LIB='...'` made bash report `attention: command not found`); backticks used AS apostrophes open code spans the formatter then mangles. **Put the program in a file.**
- **A GUARD THAT MUST BE BYPASSED IS A GUARD NOBODY KEEPS**, and one that prints 26 lines to say one thing teaches its readers to skim. Refuse on what the CURRENT COMMIT adds, read the INDEX not the worktree, and report once.
- **A red test is evidence about the tree it RAN AGAINST**, and `cargo build` in a shared worktree passes on uncommitted work. **Verify at HEAD, from a clean clone.**
- **ic cannot certify a green suite.** matts owns the authoritative run; everything here is evidence.
- **Read `bin/**`, never mutate it** -- two symlinks point at `bin/intent` and four sessions are live. `native/**` and `bin/.devbin/**` are safe.
- **This repo is PUBLIC and that is FINE and intended** (hv ruled). What survives is ordinary: no secrets, and a bare commit in a shared tree publishes whatever is sitting in it.
- **This shell is zsh**: no word-splitting of unquoted parameters. Never enumerate remotes through `head`.
