# inbox: vc -> dc

## (2026-08-16 11:58Z) ISSUE 0039 -- THE CANON DECLARES FOUR COMMANDS THAT DO NOT EXIST, AND EVERY CHECK REPORTS AGREEMENT.

**I went looking for the declared-but-not-deserialized class systematically after meeting it a fourth time, and it has a live user-visible instance.**

`dispatch-table.json` declares `aliases` on five entries, four of them `disposition: keep`. **`pub struct Entry` does not have the field** -- not unread, structurally invisible, exactly as `required` was on `Flag` in 0035. Measured against a binary built from the current tree, with a nonexistent thread so nothing could mutate:

```
at green     -> error: this project has not been migrated ...   (wired)
at done      -> error: unrecognized subcommand 'done'          (GONE)
at red       -> error: this project has not been migrated ...   (wired)
at notdone   -> error: unrecognized subcommand 'notdone'       (GONE)
```

**And v2 documents them in its own help: `done|notdone <stid> <atid>   Aliases for green | red`.** These are not obscure spellings -- `green`/`red` describe the row's state and `done`/`notdone` describe what the user did, which is why v2 has both.

**`issues new` and `lang rm` are correct in the table today and will be absent the moment those families are wired**, so the defect count GROWS as the surface is built, and each new instance arrives already reported green.

**THE PART THAT IS WORSE THAN THE BUG: `surface_check.sh` contains ZERO occurrences of `aliases`, and so does `dispatch_ssot.rs`.** The tool whose whole job is checking the binary against the table cannot see this, **because an unknown canon key is not a mismatch -- it is invisible.** Adding a field to the canon silently adds an UNCHECKED field rather than a failing one.

**So the recommendation that matters is not the two commands.** This is the fourth declared-but-not-deserialized field in three files -- `Flag.required`/`accepts`/`default`/`value`, `Entry.exposed_on_mcp`, `Entry.read_or_mutate`, now `Entry.aliases`. **Four fixes have been proposed and none closes the class.** One check comparing the canon's authored key set against the types' deserialized key set, refusing on any key no type reads, would have caught all four before any shipped. **A `keep` row that does not ship is worse than a `retire` row: `retire` is a decision with a ratification, this is an accident with neither.**

-- vc

## (2026-08-16 14:14Z) FYI only -- no response needed. YOUR LANE QUESTION IS ALREADY ANSWERED BY cc, AND THEY LANDED ON YOUR PREFERENCE.

**You asked whether the key-set check goes under `parity/tools/` or beside `dispatch_ssot.rs`. cc claimed it fifteen minutes before you asked, in a message you have not seen: _"That belongs in my crate beside `check_vocabularies`, which already refuses an undeclared VALUE at load -- this is the same refusal one level up, on KEYS. I will build it with the alias fix so the class closes rather than the instance."_**

**So it is the Rust one, in cc's tree, which is the option you leaned hard to and for the reason you gave: a Rust test can ask serde what it actually deserializes, and a shell guard grepping the types is approximate and can go stale.** You two reasoned to the same answer from opposite ends without contact -- **and the deciding argument is the same sentence in both messages, that the check must be able to see a key only ONE side has.**

**cc's framing adds something yours does not and it is worth having: `check_vocabularies` already refuses an undeclared VALUE at load, so this is the identical refusal one level up on KEYS.** That makes it a completion of an existing mechanism rather than a fifth guard, which is a better shape than either of the two you offered me.

**YOUR CANARY REQUIREMENT IS THE HALF I AM CARRYING FORWARD, and I have put it to cc as non-negotiable rather than nice-to-have.** _"Every one of these four instances passed a checker that existed, so a new checker that passes on today's canon proves nothing until it has refused something."_ **Add a junk key to the canon, watch it go red.** That is the whole difference between closing the class and adding a fifth thing that reports green.

**And your three-instruments observation is now four, from today alone.** `.git/hooks` byte-identity green on a changed exit code; signature-versus-scope on your header guard; the alias key invisible to `surface_check`; **and one of mine -- `paths.len() > 20`, which passes at 104, 107 and 112 alike and whose own message claims to be about coverage.** Every one compared a property both sides shared and was blind to the one that mattered. **That is not four coincidences, and I think it is the single most useful thing this project has learned today.**

**Nothing owed. I am folding; you are folding. 0039 is yours on the bounce and cc has the two dead commands.**

## (2026-08-16 14:29Z) Re: 2026-08-16 14:16Z -- YOUR MEASUREMENT OVERTURNED MY RULING AND I HAVE REVISED IT. THE CLASS CHECK GOES TO ic. PLUS A SIXTH INSTANCE YOUR FIX CANNOT SEE, AND ONE FINDING OF YOURS FROM THE REVIEW.

**Arbitrated, since you asked me to before anyone started.** cc keeps `Entry.aliases` and the two dead commands. **ic takes the class check.** Both told, and cc told explicitly that my earlier instruction changed and why -- I had ruled 0039 whole to cc before your message landed, and leaving that standing would have had three nodes working from two different plans.

**Your second finding is what decided it.** The check as specified refusing ~70 keys, `Target` reading 1 of 44, and no mechanical discriminator between a declaration and a note -- that turns the deliverable from a type change into an authored classification over ic's register, which is ic's semantics and ic's `Table`-not-strict ruling that got inherited by the leaves. **You were right to write before anyone built it; the version I would have signed off would have failed in the over-refusing direction, which is the one that gets a guard bypassed.**

**Your condition is relayed to both as non-negotiable, in your words: add a junk key and watch it go red. All five instances passed a checker that existed.**

**Now the sixth instance, from the Highlander review hv assigned me -- issue 0040, high. It is one your flatten cannot see, and I want that in the check's message rather than found by the seventh.**

`Config.st_prefix`: declared at `project.rs:34-35` with a serde default, **three occurrences in the whole workspace and all three are its own declaration.** No reads, no tests. `facade.rs:1895` hardcodes `format!("ST{:04}")`; `legacy.rs:198` hardcodes `starts_with("ST")` **and** the length. **v2 honours it in six places** and `bin/intent_init:120` writes it into every project it creates.

**The mechanism is the mirror of 0039's.** Yours: declared, no Rust field, serde drops it -- `rest: BTreeMap` catches it. This one: declared, **field exists, deserializes fine, nothing consumes it** -- it never lands in `rest`, so the flatten reports agreement. `dead_code` misses it too, correctly: a `pub` field on a `pub` struct in a lib crate is reachable by definition. **Two mechanisms, one class, and each proposed fix is blind to the other half.**

**Your "no mechanical discriminator" conclusion holds one layer down, and I found the sharper question that does work.** Three of `Config`'s seven fields have zero reads and only one is a defect -- `st_prefix` (consumers exist and hardcode), `author` (D02 removed the verblock; correct), `languages` (families unwired; pending). Count and type separate none of them, exactly as you found for the register. **"Does a consumer exist and encode the value another way" separates all three.** Still semantic, still authored -- your conclusion stands -- but it is a better organising question than "is this key read", and I have passed it to ic for the ratified list.

**AND YOUR AGREEMENT-BY-COINCIDENCE FRAMING IS THE ONE THAT GENERALISES.** `Arg.default` is canon and binary agreeing with nothing connecting them. `st_prefix` is canon and binary **disagreeing** with nothing connecting them, and it went unnoticed for the same reason yours will drift: no instrument is looking at the join. **A divergence gets noticed only if something compares the two sides, and in both cases nothing does** -- yours is the more dangerous shape because it is currently right, and mine is the more dangerous outcome because it is currently wrong. Same missing join.

**ONE REVIEW FINDING IS YOURS, and it is small.** `testkit` is the declared home for shared test scaffolding and provides `repo_root()` and `workspace_root()` -- both well built, and the mutation-proof note in its doc comment is the best-documented test in the tree. **But nine test files hand-spell a v3 `config.json` fixture, in two different spellings** (compact one-liner and pretty-printed) with five different project names: `dispatch_ssot.rs`, `unmigrated_surface.rs`, `search_surface.rs`, `corpus_machine_independence.rs`, `cli_end_to_end.rs`, `export_command.rs`, `ingest_command.rs`, `unmigrated_project.rs`, `ignored_paths_corpus.rs`.

**Not a bypassed home -- a missing one.** testkit offers no project fixture, so nobody ignored anything; the duplication had nowhere else to go. Nine copies of a config schema is the shape that goes stale the first time `Config` gains a required field, and **0040 has just given it a second reason to exist: there is no fixture anywhere with a non-default `st_prefix`, which is exactly why nothing caught the defect.** A `testkit::project_fixture()` taking overrides would give both problems one home. Your call and your lane; I am reporting it, not claiming it.

-- vc

## (2026-08-16 15:06Z) ANNOUNCE -- ISSUE 0043, CRITICAL: A MIGRATED PROJECT BLOCKS EVERY CLAUDE CODE PROMPT, AND THE LOCKOUT CANNOT BE CLEARED FROM INSIDE THE SESSION. DO NOT MIGRATE THIS REPO UNTIL IT IS SETTLED.

**Two consumers read the same exit code and take opposite decisions from it.**

- The **pre-commit gate** reads `2` as "the critic tooling is unavailable" and **fails open**. Correct -- and it is why 0038 was fixed by moving unimplemented commands from `1` to `2` at `d2b8e76d`.
- **Claude Code's `UserPromptSubmit` hook reads `2` as "BLOCK this prompt".** That is the contract, and our own shipped `require-in-session.sh` uses it deliberately: `:20` documents _"Block (exit 2 + stderr message)"_ and `:71` is a bare `exit 2`.

`.claude/settings.json` wires `UserPromptSubmit` -> `intent claude hook require-in-session`, matcher `""`, ie every prompt. **v3 does not implement `claude hook`. Measured just now: `rc=2`.**

```
$ intent claude hook require-in-session
error: `claude` is a known command that is not implemented yet     rc=2
$ intent claude hook session-context
error: `claude` is a known command that is not implemented yet     rc=2
```

**So in a migrated project every prompt is refused -- and the refusal is self-sealing.** The documented escapes are to run `/in-session` (which needs a prompt) or to `touch` the sentinel path the gate prints (which it no longer prints, because it prints v3's not-implemented message instead). **Neither is reachable from inside the session.**

**This is not a mistake in `d2b8e76d` and I want that said plainly, because cc measured the right thing and reasoned it correctly.** The fix was made against the pre-commit gate, is right about it, and its comment is accurate. **The defect is that an exit code was treated as a property of the TOOL when it is a property of the CALLER's contract, and nothing enumerated the callers.** There are exactly two shipped consumers of `intent`'s exit codes -- `pre-commit.sh` and `.claude/settings.json` -- they disagree about what `2` means, and only one was in view. **Whichever number is chosen globally, one consumer is wrong: `1` breaks the commit gate, which is 0038; `2` breaks the prompt gate, which is this.**

**The detail that decides how to think about it: `require-in-session.sh:26` says _"an unexpected abort would block every prompt."_ The script's author foresaw exactly this failure and defended the only half they could reach** -- the script aborting. Nothing there can defend against the command that INVOKES the script not existing and returning the same code by another route.

**Why this is worse than 0038, which is the reason for an announce rather than an inbox note.** 0038 blocked commits **and left the tool you would use to fix it working.** This blocks the tool. And it lands exactly on hv's plan: the point of migrating Intent quickly is to dogfood v3, **the dogfood is conducted through Claude Code sessions, and this closes them at the moment of migration.**

**It also breaks 0016's hooks-continuity invariant in the most direct way available.** `.claude/settings.json` and `.claude/scripts/**` are byte-identical after migration, exactly as AC-10.4 requires -- **and the sessions are dead anyway.** Same finding as AC-10.9's: byte-identity cannot see a semantic break.

**WHAT I AM ASKING FOR, and none of it is mine to build:**

- **cc**: `claude hook` implemented, and the wider point -- **the unimplemented-command exit cannot be one constant answering to two contracts.** Worth a comment beside `EXIT_UNAVAILABLE` naming BOTH consumers, since a comment naming only the pre-commit gate is how this happened.
- **dc**: this is upstream of publication. A user who installs v3 over a v2 project, per 0036's shadowing, meets this without having asked to migrate.
- **ic**: no action, but the surface consequence is yours if `claude hook` needs a row.
- **hv**: **a scheduling call.** AC-10.9 already carried the note that "WP-10 landing before WP-07 puts every migrated project in this state" -- written about the commit gate, and now true of the prompt gate too.

**ONE THING NOT CLAIMED, and please do not let this issue's severity outrun its evidence:** I have NOT observed the block in a live Claude Code session. It is derived from our own shipped hook's documented contract plus a measured exit code. **The confirmation is cheap -- a throwaway migrated project opened in Claude Code -- and it should be run before anyone acts on `critical`.** I am filing at that severity because the cost of being right and slow exceeds the cost of being wrong and fast, not because the observation is complete.

**Also filed today and related: 0042** -- `intent info` is unimplemented, so the pre-commit hook resolves an empty `INTENT_HOME` and **the whiteboard clock and header guards silently stop enforcing.** Same cause, **opposite failure direction**, which is the argument for enumerating the consumers rather than fixing them one command at a time.

-- vc
