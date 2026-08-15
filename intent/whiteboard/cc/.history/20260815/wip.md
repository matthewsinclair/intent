<!-- folded 2026-08-15 08:27Z -- the overnight WP-06 session, board state at 02c2f08 -->

## Watch-outs

- **A mutation that does not apply is indistinguishable from a test that legitimately caught nothing.** Mine did tonight -- `\n\n` inside a `python -c` double-quoted string became real newlines, the needle never matched, the assert fired, the shell carried on, and the suite reported ok. Heredoc plus an explicit non-zero exit, every time.
- **`cmd | head; echo $?` reports HEAD's exit.** Three times tonight, with it already on this board. ic's `burn.sh` guard is the habit: `out="$(cmd 2>&1)"; rc=$?` and only then filter.
- **`git commit --only <paths>` does NOT protect a file two nodes are editing.** `intent wp start` left `acceptance.md` dirty with vc's prose and I nearly swept it; reading the diff first is the only thing that saved it.
- **A test written from the same misreading as the code cannot catch the misreading.** `both_spellings_of_sync_are_wired_and_agree` passed and confirmed nothing but its own premise. The incumbent's behaviour is the independent check.
- **`intent at green` checks EXISTENCE, never tracked-ness** (vc). A green AT can cite a file present in one working tree and absent from a fresh clone. Filed as a v3 requirement.
- **NEVER mutate `bin/**` in place while anyone else is live** -- `~/.local/bin/intent` symlinks INTO this repo. Sacrificial `git worktree` only.
- **Do not use `git stash` in this repo** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **The markdown formatter is a second writer and it wins.** `TableMode::Markdown` keeps a minimum column width of three for that reason; `Terminal` does not.
- **Every timestamp is read from `date -u +'%Y-%m-%d %H:%MZ'`, per stamp.**
- **v3 REFUSES in this repository**, correctly -- it is an unmigrated 2.19.0 project. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.

## Decisions -- this session

The ratified ones are `design.md` D22-D30; these are the working lessons.

- (2026-08-15) **A guard on reads is not a guard.** AC-10.7 looked like a bad-message defect until I mutated the guard away and ran it: `intent st new` on an unmigrated project SUCCEEDED and rendered a stub over an existing v2 thread's authored `info.md`. Reads lie; mutations destroy.
- (2026-08-15) **Two independent signals cover for each other, and that is also how one of them hides being broken.** The declaration caught the data-loss case after the unguarded run destroyed the evidence -- and separately covered for an evidence scan seeing 1 of this repo's 56 threads. A redundant check makes a partial check look whole.
- (2026-08-15) **The corpus must be a property of the repository, not the machine** (vc). The ignore walker honoured the operator's global gitignore, so what counted as canon differed per developer -- and AC-10.2 turns that into a migration that blocks for one and not the other.
- (2026-08-15) **A fixture that cannot fail proves nothing, and the second attempt is where you find out.** The first global-ignore test used `*.sql`, which is in my own real global. It now uses an extension no real global carries AND asserts `git check-ignore` agrees before asserting Intent disagrees.
- (2026-08-15) **Classify by the exit code, not the printed text.** Judging commands on what they printed meant adjudicating whether "`ST0001` is not a work package" counted as guarded, and every such judgement widens the pass condition until the test agrees with the code.
- (2026-08-15) **Derive the sweep from the table, never a hand-list.** A hand-list is complete the day it is written and silently incomplete from the next verb on.
- (2026-08-15) **Interleave and take the minimum, or do not report the number.** My first cost measurement made the UNCHECKED build look slower; spawn noise exceeds the effect.
- (2026-08-15) **Port the incumbent's ONE function, not its output.** v2's width algorithm is a single `render_table` shared by three commands "so the two tables cannot drift apart" -- reproducing the look would have passed the tests and drifted at the first change. Porting it made `wp list` free.
- (2026-08-15) **"As observed" cannot mean reproducing an absence of behaviour.** v2 reads `scope:` as free text and this repo carries ELEVEN spellings across 129 work packages; there is no observed behaviour to reproduce, which is exactly what modelling the field fixes.
- (2026-08-15) **A confident claim about a corpus you do not own is a defect handed to someone else.** "Tempdir fixtures are unaffected" was true of my Rust fixtures and false of the BATS estate; ic measured instead of believing me.
- (2026-08-15) **Check whether an apparatus you depend on has the hole you would expect, before it costs anything** (vc's framing). I found the `to-write` case closed and stopped; vc found the tracked-ness case beside it open.
- (2026-08-15) **Truncating a frequency-sorted list removes exactly the evidence you were counting for** (vc, on my `| head`). It is not bad luck that the cut took the eleventh of eleven: `sort -rn` puts the RARE value last, and the rare value is the one that decides an enum rule. The common values were never in question.
- (2026-08-15) **A rule that depends on its author remembering it at the moment of use is not a control -- it is a hope with good phrasing** (vc). Both of tonight's failures were by authors who had written the rule THAT DAY: my `| head`, three lines from my own note about it; vc's four fabricated stamps while enforcing the clock rule on ic. The two remedies that worked were mechanical and needed nobody to remember anything -- the clock guard REFUSED the commit, `lib_corpus.sh` REFUSED the register. The two that failed were documentation. **My Watch-outs section is documentation**, so it is evidence for WP-14 rather than a defence.
- (2026-08-15) **`| head` truncates a measurement silently, and it took the row that mattered.** I reported TEN scope spellings; there are eleven, and the eleventh -- `Medium-Large`, which maps to nothing in the enum and forced vc's marked-legacy ruling -- was cut off by exactly one line. Fourth instance of the needle class in one session, and the only one that reached a source comment and a commit message rather than a terminal. **`| head` was already on this board as a watch-out when I did it.** The rule being written down was not sufficient; printing the size of what you matched is (`sort -u | wc -l` would have said 11).
- (2026-08-15) **A query must prove it matched something before it is allowed to report nothing.** My built-but-unflipped AT list returned ZERO and I nearly sent it: the regex captured `covers` as `([^-]+)`, which stops at the hyphen in `AC-06.2`, so it matched no rows at all and printed a clean result. Five rows were outstanding, one of them AT-10.7. **Third instance of this shape across the three of us tonight** -- vc's `*.sql` fixture, ic's header needle, my regex -- and every one was a check answering confidently about a set it never looked at. Assert the parse is non-empty, and print the count so the coverage is arguable.

<!-- folded 2026-08-15 08:56Z -- the dc handover detail; durable copy lives in dc/inbox.cc.md -->

## Handed to dc (DevX Claude), 2026-08-15

hv brought `dc` online for dev-x / build / git, which leaves cc on services and app functionality. Moved out of my TODO and into theirs, with everything they need already measured:

- **Binary flavour switching + staleness reporting** (hv's ask, port from Conflab). `Conflab bin/.devbin/cmd/use` switches via `brew link`/`unlink` exploiting PATH order (`/opt/homebrew/bin` at position 1, dev symlinks at 17+); `Conflab bin/.devbin/cmd/cli` selects among reachable copies with `--bin auto|brew|local|repo` and reports staleness; the two stay orthogonal. **Where Intent differs and it is not a detail**: Conflab switches two builds of ONE program, while Intent's `~/.local/bin/intent` points at the **v2 bash CLI** -- a different program from the v3 binary. The axis is three-valued during the rewrite and "out of date" is ambiguous across two of the three. No Homebrew tap yet either.
- **`bin/.devbin/cmd/{cli,build,build.d/*}`** -- I wrote these this morning (hv's items 3 and 4) and they work; they are dc's to own now.
- **The build system and CI** -- `native/rust/**` layout, `.github/workflows/rust.yml`, the `native/{platform}/` convention.
- **Both rules learned this morning are dev-x rules and belong on their board**: the half-committed move (`--only` names two facts) and the stale build cache. Both are in my Watch-outs below; they are duplicated there deliberately because they bit ME while doing build work, and I will still trip them if they are only recorded elsewhere.

<!-- archived 2026-08-15 10:57Z: the D01-reversal day -->

## DOING

- **AC-04.6's SERVICE half is landed at `acf8491`** -- `transitions.rs` (the declared graph) + `AT-04.6` green, 245 tests, fmt and clippy clean. **Surface half is blocked on ic** and correctly so: the spine is built from the dispatch table, so `intent ac unsatisfy` needs ic's row first. Told ic at 09:25Z with the addition recorded BEFORE the wiring (AC-06.3). vc has the gate row and the one judgement call: whether five `Unbuilt` fields owing mutations leave AC-04.6 short of closing.
- **CLOSURE IS NECESSARY AND NOT SUFFICIENT** -- the morning's real finding, and it is against my own instrument. Mutation-testing showed that once scope changes cleared satisfaction, deleting `ac.unsatisfy` STILL left `satisfied: true` formally leavable via descope-then-rescope, so the closure check went green over the exact defect hv ruled on. **My own fix is what disarmed the test.** Edges are now Direct or Incidental; incidental counts for reachability and never discharges a trap. The rule: _a state you can only leave by changing a different field is still a state you cannot leave._ Six mutations, all caught.
- Gates: 01, 02, 03, 05 PASS. WP-04 pending vc on AC-04.6; WP-06 4/7 (AC-06.1, AC-06.6 mine; AC-06.3 is vc's and ic's).
- Session detail is in `.history/20260815/`.

## TODO -- in this order

1. **The marked-legacy `scope` field.** Shape DECIDED, so this is a build: keep `scope` a **unit-only, non-optional** enum, carry the out-of-enum spelling in a **sibling optional field**. Unit-only because `TShirt` derives async-graphql's `Enum`; non-optional because `Option<TShirt>` would make it nullable for all 129 well-formed work packages and admit an invalid both-none state. Requirement: **the value is neither guessed nor dropped**. Driven by `Medium-Large` (1 of 129, `intent/st/COMPLETED/ST0020/WP/09/info.md`). Ruling: `data-model.md:83-89`.
2. **AC-06.6 -- `intent export --format <fmt>`.** Round-trip to byte-identical canon, OR refuse the format BY NAME rather than emit lossily. Settle first: whether `md` can round-trip at all, or must be refused despite D03 naming it.
3. **AC-06.1 -- the surface tail.** `st edit`, `st repair`, **`st bootstrap`** (hv RULED the verb at `c1cca8c` -- not `initzero`, not the incumbent `st zero`; `zero` was never a verb, it is the NAME of the thing, so the real verb was `install` hiding a level down. `install` is COLLAPSED into the bare form, flags `--audit-only`/`--dry-run`/`--deliverable`, root face DELETED. **Watch when wiring**: `st_zero`'s row is `corrected`, so `is_shipped()` is true for a deliberately deleted face and it is today indistinguishable from a merely-unbuilt one); `issues`, `todo`; `info`, `version`, `config`, `init`, `bootstrap`; then `claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, `fileindex`. **`intent config` lands a conformance test BEFORE its behaviour is designed**, or the `undefined` ruling on it is unverifiable. And `bin/intent_st:1231` is `[0-9]+)` -- `+` is literal in a `case` glob, so only the 4-digit form of `st repair` has ever worked.
