---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 17:32Z
status: active
focus: "Folded. EXP-05 built + adopted, surface_check.sh built (21 findings), INV-07 applied. OWED: the register-vs-truth control -- do not start it before vc answers on the AC."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**THE ONE THING I OWE, and vc called it the better finding of the day: nothing compares what the register SAYS against what is true elsewhere.** Two axes, both demonstrated by accident rather than by design:

- **state vs the BINARY** -- `surface_check.sh` covers the flag and arity half. It does NOT check `target.state` itself, which is the axis that let the contract say "open" for hours while the binary said "closed".
- **state vs `parity.md`** -- INV-07 and INV-06 were BOTH ratified there and the table went on asking hv for them. Two of my own artefacts, disagreeing for a day about what had been decided.

**The buildable design: `parity.md`'s ratified `Corrected` members should CITE the unit they cover** (INV-07, INV-06, ...), so the check is exact instead of prose matched against prose.

**DO NOT START IT BEFORE vc ANSWERS.** `parity.md` is the WP-01 spec, vc is ruling on its contents, and vc said this is worth an AC -- inventing one is not mine.

## TODO

1. **The agent guide's AUTHORED half (AC-09.4)**, when the v3 workflows settle. Spec written (`surface/agent-guide.spec.md`), control built and mutation-tested (`parity/tools/guide_refs_check.sh`). **Waiting on vc: one file carrying `usage-rules.md`'s dual role, or split?** The measurement argues for splitting -- a document serving two readers was maintained for one of them.
2. **`gen_pertest.sh` and `gen_register.sh` still read `date -u` into a `DATE=` stamp.** **NOT a D42 matter** -- a generator stamping its own run time into its own generated output is the case hv explicitly permits. The argument that survives is idempotence, which `gen_inventory.sh` now carries: a date in the output makes the artefact non-idempotent across days and destroys the byte-identity that is its only content check. **Still not fixed and the reason is unchanged: those two cannot be re-run (their burn inputs are gone), so editing them strands their committed outputs -- a generator I cannot run is one I cannot honestly edit.**
3. **`gen_inventory.sh` still execs `$SP/extract_verbs.sh`**, so the tools must be COPIED into a scratch dir before it runs. A large part of why nobody re-ran it for a day.

## Open with others -- LIVE ASKS ONLY

1. **vc, three answers outstanding.** (a) **INV-06 is ratified verbatim in `parity.md`** (the stderr/stdout misroute census -- same numbers, same parenthetical) and I RECORDED it as `target.ratified_elsewhere` without applying it; the state stays `pending-hv` until vc applies it. (b) **`st_zero` AND `st bootstrap` are both in the surface** -- hv ratified that the root spelling dies, the row carries `target.spelling` and nothing reads it, so a `corrected` row whose correction is a RENAME ships under both spellings. Only row with that shape; I lean `retire` over teaching the spine to read `target.spelling`. (c) **the two `init --help` assertion COMMENTS** -- vc is right that a stale comment asserting a retired rule is how the rule comes back, but I have not touched `tests/**` and want the word first.
2. **cc, four defects, one live.** The spine does not honour the flag `disposition` (`spine.rs:142`); `ac satisfy --evidence` is declared `required` and read as `unwrap_or_default()` (`render.rs:671`) so a missing citation silently becomes `""`; a family that HAS VERBS never gets its own declared flags (`intent todo --json` exits 1); and `subcommand_required(true)` is hardcoded against a declared `arity: "0..1"` on **8 of 8** reachable families. **All four reproduce with `bash intent/st/ST0056/parity/tools/surface_check.sh`.**
3. **The hv queue is 8 real units** (was reported as 14). `critic`'s exit-2 is still the only one with a LIVE CONSUMER -- the pre-commit gate reads it today.

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it.

- **D42, SETTLED: it is a rule about SIGNATURES and SOURCE DOCUMENTS. I over-applied it TWICE IN ONE DAY, both times towards MORE PROHIBITION, which looks like rigour.** Forbidden: confecting a time into a source document, and -- vc's sharpest form -- **no CLI or `intentsvcs` function TAKES a time as a parameter** (a property of the API surface, so a time-typed input is a defect by inspection). Permitted, all three ruled by hv: **RETURNING** a time; **reading a clock to make a DECISION**; **stamping when a command ran into a GENERATED artefact**. hv: _"there is no need to be pathological about it."_ **All three of my D42 findings fell.** An over-strict reading would have withdrawn `backup --list` and every `created`/`completed` a `show` prints -- **exactly the surfaces D42 exists to make trustworthy rather than remove.** The rule takes from the WRITE path and gives the READ path its guarantee.
- **AND THE AUDIT STILL PAID, BECAUSE CHECKING MY OWN WRONG CLAIM IS WHAT FOUND THE REAL DEFECTS.** A pattern match produced three wrong verdicts; reading the code under them produced two right findings -- `backup --list` displaying a file mtime AS a record fact, and `todo.md` being the sole store of its own watermark.
- **A CHECK THAT CANNOT FAIL IS NOT A WEAK CHECK, IT IS A DECORATION -- and it hands you a reassuring result first.** Six instances now. The sharpest was the check written to CLOSE this class: the flag-completeness loop went green while 5 of 93 flags rendered nothing, because it greps for the LABEL and 88 flags supplied it. **Presence-of-label and completeness-of-population are different questions.** The subtlest: a TSV field shift made the surface check report **MORE coverage and FEWER findings at once** -- 59 probed against 46, 11 findings against 13 -- which reads as a better run, and was caught only because the earlier output was still on screen. **Count the population, assert both sides are non-empty, and keep the previous result to compare against.**
- **A CONTROL THAT DIFFERS FROM THE MUTANT IN A SECOND WAY IS NOT A CONTROL.** Twice at the same spot in one afternoon: I copied a generator to a temp dir to mutate it and it died on an unrelated path check, because `REPO_ROOT` derives from the script's own location. **Run the copy as a SIBLING of the original, and print the control result FIRST.**
- **`read -r a b c d` WITH `IFS=$'\t'` COLLAPSES AN EMPTY FIELD** -- bash and zsh alike, both verified. Every column after the empty one shifts. **Emit a `-` placeholder, never an empty field**, and refuse on a row whose shape is wrong: a row that yields nothing to check is indistinguishable from a clean one.
- **ENUMERATE THE POPULATION; DO NOT SNIFF FOR A MARKER -- and a structured query is a needle too.** `jq '.families[].entries[]'` missed a whole top-level array; the D42 audit's first pattern matched `manage`/`validate`/`update` on substrings of `age` and `date` and reported 25 rows. **Word boundaries cut it to five, of which three were real.**
- **"IT DOES NOT EXIST" IS A CLAIM ABOUT THE FILESYSTEM, SO GO AND LOOK.** git answers questions about HISTORY; if the question contains "exists", the answer comes from the filesystem. One `find` beat a rule and a refusal built on top of a wrong `git log --all`.
- **A SWEEP DOES NOT MOVE A FILE, IT SPLITS A CHANGE.** My unqualified `--amend` took cc's test without the `store.rs` methods it calls and **HEAD did not build for ten minutes**. Each half reads as finished alone. **After a sweep the question is not "whose file is this", it is "does it still build"** -- a `git show --stat` says which files moved and cannot say whether the tree compiles.
- **`--only` PROTECTS THE COMMIT AND NOT THE AMEND**, and never CLEARS the index, so the shared index accumulates across every node. **Name paths on the amend too.**
- **A SKIP LIST IS A PROMISE THAT SOMETHING ELSE RENDERS THE KEY**, and it is only as good as that promise. Mine exempted four keys nothing rendered. **Reading the list produced the bad list; the mutation test found it.**
- **RE-DERIVABILITY IS NOT COMPLETENESS.** A lossy generator is a perfect fixed point with itself, so skew passes forever. It hid 15 of 20 authored fields.
- **A MISSING MEASUREMENT MUST PRESENT AS A REFUSAL TO MEASURE, NEVER AS A MEASUREMENT OF NOTHING.**
- **A QUOTE CHARACTER INSIDE A QUOTING CONTEXT, IN PROSE NOBODY PROOF-READS FOR SYNTAX.** An apostrophe in a single-quoted jq program CLOSES it -- `vc's`, then `run's`, the second one inside a comment explaining a different bug. Backticks in a double-quoted string are command substitution. **Put the program in a file.**
- **A GUARD THAT MUST BE BYPASSED IS A GUARD NOBODY KEEPS**, and one that prints 26 lines to say one thing teaches its readers to skim. Refuse on what the CURRENT COMMIT adds, read the INDEX not the worktree, and report once. **A report is the right shape mid-ladder: `surface_check.sh` reports 21 findings and gates nothing.**
- **A red test is evidence about the tree it RAN AGAINST**, and `cargo build` in a shared worktree passes on uncommitted work. **The release binary was an hour stale when I first measured against it.** Verify at HEAD; `int build cli` takes 30 seconds.
- **ic cannot certify a green suite.** matts owns the authoritative run; everything here is evidence.
- **Read `bin/**` and `tests/**`, never mutate them** -- two symlinks point at `bin/intent`, the BATS estate defaults to `INTENT_BIN=bin/intent`, and four sessions are live. `native/**` and `bin/.devbin/**` are safe.
- **This repo is PUBLIC and that is FINE and intended** (hv ruled). No secrets, and a bare commit in a shared tree publishes whatever is sitting in it.
- **This shell is zsh**: no word-splitting of unquoted parameters. Never enumerate remotes through `head`.
