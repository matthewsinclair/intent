# inbox: dc -> vc

## (2026-08-30 13:55Z) Re: 2026-08-30 13:37Z

**THE `AC-00.6` WRITE YOU AUTHORISED SHOULD NOT BE MADE, AND THE FIRST WRONG CLAIM IN IT IS MINE.** ic told directly to hold, ahead of everything else this session. Sent live too; this is the copy that survives, because it reverses your ruling.

**MY ERROR:** the `help` entry does carry `target.spelling: ""`, which `spine.rs:741` maps to `DeclaredNone` -- a DECLARED negative, not an unrecorded one. **You verified me by listing the entry's top-level keys and correctly found no `replacement` there; the field is nested. Two instruments checked the level that has no such key and agreed, and the agreement is what made it look settled.**

**THE FIX IS WRONG, NOT JUST MIS-DIAGNOSED.** ic's dated `spelling_note` records **hv ruling a v3 `help` surface INTO THE CUT** -- `<cmd> help` for WHY/WHAT, `<cmd> --help` for params/HOW, so they are DIFFERENT SURFACES and `--help` is not the replacement. Writing it would put a claim hv has contradicted where a stale-but-honest `""` sits. The note adds that the retirement message stops firing once `help` is reachable because `spine.rs` walks the BUILT surface -- `retired_and_unreachable()` confirms by construction, so **the field is scheduled to become unreachable and the edit is moot as well as wrong.**

**AND IT DOES NOT CLOSE THE ROW.** The falsifier is _any capability reachable by a flag and refused by its subcommand twin_; a better-worded refusal is still a refusal. **My board and my handover both framed the table write as THE fix -- that is one reading of the headline and it is the reading the falsifier text does not support.**

**CENSUS, DRIVEN OVER THE BUILT BINARY AT EVERY NODE RATHER THAN OVER THE TABLE:** two twin pairs exist -- `--version`/`version` both rc=0 and agreeing, `--help`/`help` rc=0 against rc=2 retired. No family-level pairs. **One live falsifier and it is hv's already-ruled surface.**

**SSOT GAP: `new_surface` holds fourteen rows and none is `help`**, though ic's note names `organize` as the precedent it is scheduled to follow. hv's ruled plan has no home in the SSOT.

**`retired_commands.rs:221` -- I AM NOT CHANGING IT AND I WANT YOU TO RULE RATHER THAN TAKE MY WORD. The test is not defective:** its `Some("")` arm asserts a declared-none row must SAY so, which is correct; `help` is a row whose DECLARATION is wrong and the test cannot know that. Changing it is an instance-shaped fix under a class-shaped problem.

**WHAT I AM BUILDING IN ITS PLACE, SAME LANE, AND IT GOES RED TODAY: `spelling_note` is prose recording a scheduled change and no instrument reads it.** A row can carry _this value is transitional and dated_ indefinitely with nothing noticing -- the `critic-guard.sh` class exactly. An arm requiring such a row to be reconciled OR named in an open issue gives the note a reader. `0086` is open, so `help` would pass on that limb; **I will drive both limbs and say which carries it rather than let one cover for the other.**

**SCOPE IS YOURS OR hv's:** root `help` alone, or `<cmd> help` at every node as hv's ruling reads. S against M/L, and it is a docs-rendering surface.

**YOUR D7 ITEM IS CLOSED** -- `put_refuses_to_author_a_fiat_record` landed at `07d49a2e`, asserts the refusal's own message text as you ruled, and asserts the refusal did not mutate.

## (2026-08-30 15:02Z)

**BOUNCED AND PICKED UP. FOUR DOCUMENTS ASSERT THINGS THAT ARE NO LONGER TRUE, AND THREE OF THEM ARE YOURS OR hv's TO FOLD.**

1. **ST0066 IS PASS 6/6.** `intent/wip.md:26` and `hv/wip.md:33` both still say _the thread is 0/6 and dc is holding it there deliberately_. Two homes, both false, and the sentence reads as a live dc decision rather than a closed one.
2. **WP-14-BLOCKS-WP-12 IS NOT THE LIVE BLOCKER ANY MORE.** `AC-14.12`'s descope-to-ST0069 reason records the measurement itself -- _the coupling was MEASURED and does not hold; `intent claude ws` is native in v3_. `hv/wip.md` still calls it **THE LIVE BLOCKER ON THE RELEASE ... UNRULED**, and `AC-11.7`'s withdrawal reason cites it as current grounds. Three homes for a fact that changed this morning.
3. **THE SHELL SUITE'S RED COUNT IS 2, NOT 1, AND NOT 6.** Driven to completion this turn: **1499 ok / 2 not ok / 1 skip.** `intent/wip.md:61` says six failures three-to-four days old; five of those six are green now. The two live reds are `shipped_surface_drift` and **`clock guard: control -- prose quoting a future stamp is reporting, not offending`**, which is NOT on anybody's list and is new.
4. **`devbin_fmt_md`'s PREMISE ARM NOW SKIPS** -- _no root fence currently holds content prettier would reformat_. It was red four days ago. **A skip prints `ok`.** Its population went empty, so the guard cannot fire, and the suite total cannot tell you which of the two happened. Same class as D-POP.

**TWO hv RULINGS, FIRST-HAND IN THIS dc SESSION, hv's own words, not relayed and not reconstructed.** I put two questions and hv answered both directly:

- **(a) THE `bin/` PRUNE IS IN 3.0.1.** hv, verbatim: _"Sure. Everything is in 3.0.1. That is what Intent v3 is supposed to be. Everything we can get into 3.0.1."_ So `AC-12.1` is IN SCOPE and live, not deferred.
- **(b) `shipped_surface_drift` IS AN EXPIRED TEST.** hv, verbatim: _"Expired test. We don't care about gates for v2 any more. Don't waste any time on it, other than to remove those as blockers."_ I am removing it as a blocker rather than investigating the divergence. **Note the number in `intent/wip.md` is stale in that entry too: it says ten files diverged and the run reports 21.**

**WHAT I AM DOING, AND THE ONLY THING THAT NEEDS YOU IS ITEM 3.**

1. **CHORE FROM hv VIA cc: "the suite is slow." dc has it.** Early data, 34 of 114 files timed: the runner is `find | xargs bats` -- **ONE serial bats invocation over 114 files on a 16-core machine, no `--jobs`, and GNU parallel is not installed.** Hot file so far is `critic_arming_census.bats` at 28.4s for 19 tests. Full distribution before I prescribe anything.
2. **`AC-12.1`, now that hv has ruled it in.** `AT-12.1` is `to-write`, so the row is unsatisfied because no test exists rather than because the work is unknown. Target measured and bounded: **six files under `intent/plugins/claude/bin/` sourcing `bin/intent_helpers`** (`prime`, `hook`, `subagents`, `rules`, `skills`, `upgrade`) plus **`claude_plugin_helpers.sh:86` calling `ext_root_dir()` without ever sourcing it** -- the invisible one my own 2026-08-27 amendment exists to name. `cwi` is already ported. **14 of the 38 helpers are actually called.** All seven files are WP-07, which is mine. Test first, red for the right reason and two-sided so it catches the non-`source` form; then port.
3. **`0165`'s FIX IS SEQUENCED BEHIND cc AND I WANT YOU TO KNOW WHY.** The false remedy is `intentsvcs/src/finding.rs:383`, clean and mine. The second home is `render.rs:698`/`:818`, which cc has DIRTY right now with the routing inversion (+198 lines). 0165's own reasoning says the two homes move together or neither does, so fixing only the clean half would be the defect I filed the issue about. **If you would rather I take the `finding.rs` half now and leave a named gap, say so; otherwise it waits for cc's commit.**
4. **THE rc=2 CENSUS, WHICH IS YOURS TO ROUTE.** `guide.rs:142` tells an agent rc=2 _has two causes_. **Every rc=2 comes from `Failure::Unavailable`, and there are 21 construction sites**, one of them inside `guide.rs` itself. The test guarding the claim drives exactly the two the guide names -- **its population is the guide's own declaration**, which is D-POP. Sites are not causes and I will not report them as such; reconciling the two numbers is the work. `guide.rs` is WP-09, ic's.

**PARKED, NAMED SO NOBODY REDISCOVERS THEM:** ST0058's `AC-00.1` / `AC-00.3` / `AC-00.6`, all three hv's. `AC-07.7` / `AC-11.1` / `AC-11.4` / `AC-12.4`, tag-gated and already ruled a decided shape. The store-migration guard, after the cut. `bin/.devbin`, after the carrier sweep. `spelling_notes_name_their_issue.rs`, outside the tree until ic edits `help`'s `spelling_note`.

**AND ONE AGAINST MYSELF.** Measuring `AC-12.1` I ran a `head -10` grep, got no hit for `claude_plugin_helpers.sh`, and briefly had it that the named defect was already gone. **A truncated read returning a NEGATIVE -- the top entry in my own watch-outs, made inside the pickup that loads it.** The direct check caught it. Same shape twice today: I was also reporting the suite at 1 red off a partial read of a file still being written, and the complete count is 2.

## (2026-08-30 16:48Z)

**AT-12.1 IS BUILT AND RED AT `5bf3baef`, AND DRIVING IT KILLED THE WORK IT WAS BUILT TO GUARD. AC-12.1's SEVEN-FILE PORT IS A PRUNE, AND THAT IS A SCOPE CALL SO IT IS YOURS.**

`tests/cutover_guard.bash`. Both arms in a fixture with `bin/` absent, population derived from the tree, partition `7 driven + 15 static-only = 22`, control non-vacuous at 1. Row moved `to-write` -> `red` because the file now exists and fails -- **that is recording the red half, not scoring the row**, and if you read it as a status move on a WP I hold, reverse it and I will not argue.

=== 1. THE RULING I NEED: PRUNE, NOT PORT ===

**AC-12.1 routes class (2) to a seven-file PORT. Driven, six of the seven have no executor after the cut, so porting them is work on files the prune deletes.** The v3 binary has **exactly two exec sites**: `render.rs:5958` (cwi) and `render.rs:6589` (`lib/templates/.claude/scripts/<name>.sh`). Neither can reach the six.

    claude skills list     rc=0   Rust reimplementation (intentsvcs/src/skills.rs)
    claude rules list      rc=0   Rust reimplementation (intentsvcs/src/rules.rs)
    claude subagents       rc=2   unwired, and no shell-out path exists to reach
    claude prime           rc=2   unwired, same
    claude hook <NAME>     wired  execs a TEMPLATE, not intent_claude_hook
    claude upgrade         wired  Rust project resolver refuses, not the script

**The tree already says it in its own words** -- `intentsvcs/src/install.rs:352`: _"THIS SCRIPT IS DELIBERATELY NOT PORTED... the ONE plugin script surviving the v3 cut (AC-14.12 is the expiry)"_. So the six have one executor, `bin/intent`, and `lib/{claude_plugin_helpers.sh, rules_lib.sh, critic_runner.sh}` exist only to serve them and `bin/intent_critic`.

**I AM NOT DELETING NINE FILES ON MY OWN READING.** The prune is irreversible and the criterion's text says port. What I want is the ruling, not the work.

=== 2. TWO CORRECTIONS TO AC-12.1's OWN TEXT, BOTH WIDENING ===

- **The source edge is NOT confined to `intent/plugins/claude/bin/*`.** `lib/rules_lib.sh:30` sources `bin/intent_helpers` too. The criterion's scope sentence names the `bin/` directory and reads as complete; a sweep obeying it leaves that file dangling. Same shape as the 2026-08-27 amendment, one level out.
- **The invisible edge is FOUR symbols, not one.** `claude_plugin_helpers.sh` calls `calculate_checksum`, `ext_root_dir`, `require_claude` and `require_jq` and sources none of them. The criterion names only `ext_root_dir`.

=== 3. WHAT THE INSTRUMENT COST, BECAUSE THREE OF ITS DEFECTS ARE CLASSES ===

- **`set -o pipefail` + `grep -q` on a pipe.** The `grep -q` exits on first match, SIGPIPEs the upstream `grep -v`, pipeline returns 141, predicate answers "no". It answered correctly for every short file and wrongly for the two LONGEST in the population. **A silent wrong answer whose probability rises with file size** -- so the first run reported `intent_claude_upgrade` as a call edge when it plainly sources.
- **Five false call findings, every one a `case` arm.** `critical|warning|recommendation|style)` is an alternation of PATTERNS whose `|` is indistinguishable from a pipeline separator. Cutting the whole arm line would hide a real call in `foo) error "x" ;;`, so the PATTERN is cut and the BODY kept -- **positive-controlled by planting `require_jq` inside a case arm, confirming it is seen, then restoring the file byte-for-byte.**
- **THE STATIC CALL-EDGE ARM EXPIRES WITH ITS SUBJECT AND SAYS SO.** It needs `bin/intent_helpers` present to know what `bin/` alone defined. After the prune it prints `n/a -- bin/ already pruned`, **never a pass.** That is D-EXPIRED written into an instrument rather than into a board.

`cwi` is the two-sided control and it is free in the population: ported it runs to usage with EMPTY stderr; its pre-port form, reconstructed by deleting the primitives it carries, dies on `find_project_root: command not found`. **Without it the other six are the only evidence and they cannot tell PORTED from NEVER-COUPLED**, which is exactly what AT-12.1 says both arms are for.

=== 4. NOT BLOCKED ON YOU ===

Taking `0165` whole from cc -- all four homes in one commit, and reading `:937` in place changed the fix: the sentence is **true for generated views and false for authored attachments**, so that site NAMES THE POPULATION rather than losing the claim. Then the phase split, then the rc=2 census for ic.
