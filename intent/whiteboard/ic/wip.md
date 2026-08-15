---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 16:51Z
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
2. **`gen_pertest.sh` and `gen_register.sh` still read `date -u` into a `DATE=` stamp.** **NOT a D42 matter -- withdrawn on that ground** (a generator stamping its own run time into its own generated output is the case hv explicitly permits). The argument that survives is the one `gen_inventory.sh` now carries: **a date in the output makes the artefact non-idempotent across days**, so re-rendering the same inputs at the same revision changes every file for no reason and destroys byte-identity, which is the only content check these have. Still NOT fixed, and the reason is unchanged: those two cannot be re-run (their burn inputs are gone), so editing them strands their committed outputs -- **a generator I cannot run is one I cannot honestly edit.** `measured_on` is a separate, smaller thing: an authored date in the canon, needing the generator's refusal changed, so a contract call.
3. **`gen_inventory.sh` still execs `$SP/extract_verbs.sh`**, so the tools must be COPIED into a scratch dir before it runs. A large part of why nobody re-ran it for a day.

## Open with others

1. **vc:** the `sync --to-store` vs `ingest` boundary is still undeclared, and `sync` is flagged for MCP review precisely because that boundary decides whether it stays exposed.
2. **cc:** the spine must honour the flag `disposition` once I land it. `--fix` on `doctor` is the mechanism's first user.
3. **D42 audit is DONE, and its D42 result is CLEAN -- all three findings withdrawn as D42.** `doctor` withdrawn outright; the other two survive under other names as `target.exposure`. **The `d42_exposure` key is GONE** (a census key with no members and a wrong name is worse than none) -- vc referenced it by name, so it is worth knowing it no longer exists. **`todo done` is the one that needs an owner: WP-03's renderer, not WP-06.** And vc's caution on the census stands and is the important half: a green here is about DECLARED FLAGS AND ARGUMENTS, while hv's rule is about function PARAMETERS, which never appear in a flag inventory. **This file cannot see the thing the rule forbids.** That needle is cc's guard.

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it.

- **D42 IS A RULE ABOUT SIGNATURES AND SOURCE DOCUMENTS, AND I OVER-APPLIED IT TWICE IN ONE DAY.** What it forbids: **confecting a time and injecting it into a source document**, and -- vc's sharpest form, which is the one to build against -- **no CLI or `intentsvcs` function TAKES a time as a parameter.** That form is a property of the API surface, so a time-typed input is a defect by inspection and nobody has to trace a value. What it PERMITS, all three ruled by hv: **RETURNING** a time (it went end-to-end through the DB where SQLite set it); **reading a clock to make a decision**; and **stamping when a command ran into a GENERATED artefact.** hv: _"there is no need to be pathological about it."_
- **BOTH TIMES I ERRED THE SAME WAY -- TOWARDS MORE PROHIBITION, WHICH LOOKS LIKE RIGOUR.** I filed three D42 findings; under the rule as it actually stands, **zero survive as D42**. `backup --list` I flagged for EMITTING a time, which is legal. `doctor` staleness I flagged for needing a now, which is the permitted decision case. `todo done --flush` I flagged for obtaining a now to stamp into `todo.md`, which is the permitted generated-artefact case. **An over-strict reading would have withdrawn `--list` and every `created`/`completed` a `show` prints -- exactly the surfaces D42 exists to make TRUSTWORTHY rather than remove.** The rule takes from the WRITE path and gives the READ path its guarantee.
- **AND THE AUDIT STILL PAID, BECAUSE CHECKING MY OWN WRONG CLAIM IS WHAT FOUND THE REAL DEFECTS.** Two survive under other names: `backup --list` displays a file mtime AS a record fact (provenance, now AC-02.8's fourth instance), and chasing the `--flush` claim into `bin/intent_todo` showed `generate()` reads the watermark **back out of `todo.md`** -- durable state whose sole store is a generated view, which the v3 truth model turns into a data-loss path the moment someone deletes a derived artefact to rebuild it. **A pattern match produced three wrong verdicts; reading the code under them produced two right findings.**
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
