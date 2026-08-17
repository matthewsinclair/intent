# cc board, folded 2026-08-17 -- sections retired from the live board

## Corrections this session, both from checking a premise at the moment of acting

- **"0039 CAN BE CLOSED" was queued on this board and the premise was never true.** ic ruled clause 2 with no work -- quoted onto the issue -- but a SECOND outstanding item sat in the same section all along. Re-measured rather than re-read. The issue now says so explicitly rather than relying on whoever next reads it noticing.
- **vc's `finding_remedies` blast-radius guard had a substring false positive and my change found it.** `contains("rm ")` fired on the word **"form"**, and would have fired on "confirm", "perform", "term". **Fixed at the check, not by rewording around it** -- a two-letter needle without token boundaries is a trap for whoever next writes an ordinary English word, and it blames their remedy for a defect in the guard. The loosening carries its own two-way proof.

## Shared-clone hazards, both hit today

- **A lint or test failure in a peer's file may be a READ RACE.** `cargo clippy` reported an error at `guide.rs:496` while the file was 485 lines long -- cargo read it mid-write. Cleared on re-run with no change to anything. **Re-run before reporting.** ic hit the mirror image within the hour (they added a line referencing a function that did not exist and my tree stopped compiling for a minute).
- **`cargo fmt --all` reformats peers' in-flight files.** Check `git status` before running it, or scope it.

## Settled while I folded -- nothing of mine is waiting on a peer

- **vc: the extract MAY LAG, so "entities present, no history" is NOT provable loss.** Requiring every mutation to leave the extract current puts a second write on the critical path of every mutation, which is the double truth D01's reversal removed. **My narrow check is the most that can be said and the fourth test pinning the gap is the right artefact** -- and it agrees with a ruling already in `migration.md` reached from the other end. Nothing to change.
- **vc: `data-model.md:45` fixed** -- the watermark section marked RETIRED BY D44 with the archaeology kept, because a retired mechanism whose reasoning is deleted gets reinvented.
- **ic: `critic`'s 2-vs-1 divergence is in the register** (`bcfeb135`), on the row as `target.wp07_owes`, recorded and explicitly NOT ruled. **Whoever wires critic's language validation will be looking at clap behaviour that is correct by the general rule, with nothing on that path saying this command is the exception.**
- **ic: my `Arg.default` discriminator preferred over their proposed carve-out** -- `init` falls out of the rule instead of being an exception to it.

## Verified for me, not by me -- vc re-ran rather than agreeing

**Phase A on this tree, from their own count of the filesystem rather than from my output**: 56 threads / 140 WPs / 280 criteria / 227 ATs, all four matching exactly, 0 blocking, 9 carried. **A conservation claim checked against the run that produced it is circular; this is the other side.** Distribution worth keeping for the harness: 52 under `COMPLETED/`, 2 `CANCELLED/`, 1 `NOT-STARTED/`, and **exactly 1 live at `intent/st/`** -- so a harness globbing only `intent/st/*/` reads 1 of 56 and reports success.

## Held for WP-10 -- vc's migration ruling, provisional pending hv (21:30Z)

**A blank-evidence v2 satisfaction does NOT convert to `Satisfied`, and the dilemma I sent them dissolved on their reading: carrying it as `Satisfied` is the LOSSY option**, because it destroys the distinction between "satisfied, here is the evidence" and "satisfied, with nothing behind it" -- a fact that exists in the v2 artefact and would not exist in v3. Losslessness is about information, and that conversion is where the information dies.

Four clauses, and the third is the one someone will reach for: **no false `Satisfied`; NO SYNTHESISED EVIDENCE TEXT** (`"migrated from v2, no evidence recorded"` reads as evidence forever after and nothing downstream can tell it from the real thing -- **D42's family, one field over**); no silent drop of the claim either; **so the criterion arrives `Unsatisfied` and the v2 claim is recorded in the EVENT LOG**, which is already the durable non-reconstructible half and the right home for "this is what the source said and this is why it did not convert". **Refusing the migration outright is wrong** for a closed thread -- the ruled policy is carry-lossless and the above IS lossless. **Mechanism is mine; the ruling is that no false `Satisfied` and no confected evidence may exist.**

## Held -- vc's measurement for the WP-04 rewrite (21:30Z)

**`intent ac list ST0056` takes 2 minutes 24 seconds in v2.** 109 rows, exit 0, 102% CPU, roughly 1.3s per row; `ac status` and `at lint` return promptly on the same file, so it is not the parse. **The v3 verb must not inherit whatever this is** -- a command a user runs constantly is where a per-row shell-out becomes visible.

## TODO -- in order; the top two are DONE and the rest re-ranked around what peers landed

0. ~~**ISSUE 0043 -- CRITICAL.**~~ **DONE, `c6aee944`.** `claude hook` implemented; it EXECS the shipped script, so stdin flows through and the exit code is always the script's own. Driven end to end: pass-through 0, deliberate block 2, **and the sentinel path prints again** -- which was the self-sealing half, invisible precisely because the script that prints it never ran.
   **vc routed the general conclusion to me and it turned out structural rather than a choice.** They measured FOUR meanings for `2` (fail-open / block / advisory / refuse-to-stop), so no global constant can be right. But `claude hook` is the SINGLE door Claude Code reaches this binary through and it **delegates** -- no path inside `render::hook` produces `Unavailable`; an unknown name and a missing name both answer 1, which their ARM1 measured as non-blocking. **The constant is never reached from that side.** Held by `the_hook_door_never_answers_in_the_callers_refusal_code`.
   Consumer enumeration is beside `EXIT_UNAVAILABLE`, swept rather than inherited, carrying vc's four measured rows. dc's fourth caller does NOT hold (`int prepush` never invokes the binary); their premise does, and the sweep found three they had not named.
1. ~~**ISSUE 0042.**~~ **DONE, same commit.** `info` implemented and it NEVER GATES. **This is the row that settles the argument**: the gate reads no exit code at all, it parses `INTENT_HOME:` out of stdout -- **some callers have a stdout contract, not an exit-code contract** -- so 0042 was unfixable from the constant in either direction. Driven with the gate's own `sed`: both guards ENFORCING.
   `install.rs` is THE install-home resolver, registered before written, deliberately NOT in `project.rs` (0025's class). **`INTENT_HOME` is not read at all** -- more than AC-11.3 asks. ic's control refused my first version's dev override and the refusal was the prompt to ask whether it was worth having: it is not, because `lib/templates/.claude/scripts/` DIFFERS between a v2 and a v3 install, so a stale export would make v3 exec **v2's hook scripts** with nothing reporting a version mismatch.

2. ~~**D44's WINDOW.**~~ **DONE, `def9ca44`.** Terminal-only per vc; committed `todo.md` carries everything. One generator with a `TodoWindow` allowlist parameter -- the cutoff resolves inside SQL so `views::` never learns a time.
   **Two things the ruling did not cover and I decided, both flagged to vc.** The window is over `completed`, the DOMAIN date -- a record-stamp window would show the whole estate as just-finished after every rebuild (D36) and nothing after a quiet week. And **the unit is hours while the resolution is a DAY**, because `completed` is `YYYY-MM-DD` with no time component; `date()` not `datetime()`, so the comparison is like with like.
   **A mutation escaped and that is the finding worth keeping**: `todo.md` has TWO writers (`todo_update` and `views::render_all`), and windowing the projection passed all five tests because every one reached the file through the first. **A ruling enforced on one of two writers is enforced on neither.** Sixth test added.
   **Owed to ic and sent**: `todo done --flush` / `--prune` are still `disposition: keep` in the table and D44 retires both.
3. **WP-10 PHASE B** -- built-and-tested-only, and the boundary is now WIDER than my own hold. **dc measured 0043's door as PATH, not migration**, so "do not migrate this repo" was right and insufficient: **v3 does not go on PATH here.** Remaining: emit `thread.json` per thread, split issues, regenerate views, stamp `project_id`, build the DB, one commit. Fixtures and a sacrificial copy only.
4. **`issues` (6 rows)**, then the **AC-06.1 surface tail** (NOT the installer/canon block), then **AC-04.1's `TornRollback` arm**, which AT-04.1 holds at `red` for.
5. **Three small ones left of the original four** -- `finding.rs:22-23`'s false comment (says 8, the enum has 15+); `cli_end_to_end.rs:401` asserting `code == 0 || code == 1` while its message says 101, so a legitimate `EXIT_UNAVAILABLE` fails it with a panic-flavoured message; AT-10.9's id not inside `exit_codes.rs`. **The fourth is done** -- the `AcceptanceTest` `file`/`legacy` guard is still owed, see 7.
6. **vc Highlander finding 4** -- `remedy()` five times with no trait, `"\n  remedy: {}"` six times in three files, three of seven error enums with no `remedy()` at all.
7. **The `AcceptanceTest` `file`/`legacy` precedent is still UNGUARDED** (`model_laws.rs:104` generates both independently, so all four combinations are legal). I guarded the same invariant for `scope`; this is the half vc asked me not to skip.
8. **`Arg.default` is validated, not rendered** -- eight rows declare one, `Flag.default` reaches clap at `spine.rs:444` and `Arg.default` does not. **This is why 0039 did NOT close today.**

## Watch-outs retired at this fold -- both now closed by a mechanism rather than by care

- **AN EXIT CODE IS A PROPERTY OF THE CALLER'S CONTRACT, NOT OF THE TOOL, AND I FIXED ONE CONSUMER INTO ANOTHER.** 0038 was real, the measurement was right, the fix was right about the pre-commit gate -- and moving unimplemented commands to 2 handed `UserPromptSubmit` the code it reads as BLOCK, closing every Claude Code session in a migrated project. **There are exactly two shipped consumers of v3's exit codes and they want opposite things from 2, so no constant is correct**; the fix is that the unimplemented path must not be reachable from the caller that cannot tolerate it. **My own comment beside `EXIT_UNAVAILABLE` names one consumer, which is precisely how this happened.** Before changing a code, enumerate who reads it -- the list is short and nobody had written it down.
- **THE LOUD FAILURE GETS FIXED FIRST AND THE QUIET ONE SITS BEHIND IT.** 0038 failed CLOSED: blocking, obvious, fixed in a day. 0042 fails OPEN in the same hook from the same cause -- `intent info` unimplemented, so `INTENT_HOME` resolves EMPTY and both whiteboard guards silently stop enforcing -- and it was live the whole time, underneath the noisy one. **When you fix a failure, ask what else the same cause does in the other direction.**
