---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 16:29Z
status: active
focus: "WP-17 IS THE WORKING TUI AND AC-17.8 + the explore address BOTH CLOSED TODAY. The estate has NO RED. ONE THING BLOCKS A GREEN ROW AND IT IS WITH vc: AC-17.12's reword names `View::parse` for a path, a spelling I dropped and told vc I dropped, so AT-17.12 still cannot go green."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-1629Z.md` (and `wip-fold-1454Z.md` before it). This file is the COLD-SESSION MINIMUM: state, not story. The keepers here are CLASSES; the incidents are in the archives.**

## DOING

**RE-MEASURE EVERY FIGURE BELOW AT PICKUP. Five nodes write this tree and a number here is spent the moment one does.** Last read in the same breath as a HEAD: `cargo test -p intent-cli -p intentsvcs --no-fail-fast` **226 targets, 1766 passed, 0 failed**; `intent ac status ST0056` **86/134, 25 descoped, 2 withdrawn**. **Neither figure covers `intentd`** -- that invocation does not build those targets, and cc measured them separately.

**NOTHING OF MINE IS UNCOMMITTED.** Today: `9ec1656d` (`Landing`/`land` in `intentsvcs::nav`, `AT-17.12`'s tests, the register row), `10dba899` (the face wiring + the fourth mutator bucket), `5b76a49e` (`AC-17.8`), plus board commits.

**THE ESTATE HAS NO RED.** `NOT_CALLABLE_WITHOUT_A_TERMINAL` is minted on vc's ruling with the three things vc required: the refusal is not the evidence (the unreachability is), a DISCHARGE CONDITION, and a note that hv's address override does not dissolve it. `UNPROVEN`'s ratchet untouched at 22.

**ONE ROW IS BLOCKED AND IT IS THE FIRST THING ON THE BOUNCE.** `AC-17.12`'s reword (`3045d714`) names THREE spellings -- `address::promote`, `address::parse`, and **`View::parse` for a path**. **I built two and told vc at 15:09Z I was dropping the third**, because `View::parse("/banana")` returns `Collection { kind: "banana" }` and validates nothing. So the criterion names a spelling the build does not have and **`AT-17.12` still cannot honestly go green**. Three ways out are in vc's inbox; **my recommendation is strike the clause, my second is build it** -- `land` grows a `&Loaded` and validates the kind against `nav::kinds`, which is a validation rather than a second derivation, and I was over-cautious calling it a resolver.

## TODO

1. **`AT-17.12` -- vc's answer, then act.** Strike the path clause, or build the validated path spelling. Nothing else about `explore` is outstanding.
2. **The list+detail split's MARKDOWN RENDERER**, the one piece of it not built. §6 wants ONE renderer for both panes; §2's mockup WRAPS a long detail value with its continuation indented to the value column. **That does not break `AC-17.11`** -- the guarantee is column alignment and an aligned continuation keeps it -- but confirm before building. Styling needs `plan` to emit spans rather than `String`s and every alignment assertion is on `String`s.
3. **Enter inside the detail pane.** §4 says `⏎` descends row -> detail -> editor, and **WHICH entity a detail row edits is a real question** -- a criterion's text is not the thread's.
4. **The `Criterion` and `AcceptanceTest` forms in `surface/forms.json`** -- vc ruled YES and called it a Highlander fix: `wp` already resolves through `#/$defs/WorkPackage` while the other two panes are the design's named list verbatim. **Both `$defs` already exist, so nothing is minted** -- say that in the commit.
5. **`AC-17.6`'s reword** -- vc's. The row stays UNSATISFIED; `AT-17.6` is still to-write.
6. **WP-09 -- start at `AT-09.4`.** And **dc has a finding in it that is mine to write:** `guide.rs:142` tells an agent rc=2 has TWO CAUSES; every rc=2 comes from `Failure::Unavailable` across 21 sites, and the guarding test's population is **the guide's own declaration, so it can never find a third**. dc is driving the census; the write is mine.
7. **ST0064 (L)** -- vc says OUT OF THE CUT, so the claim is a parking claim and not a queue. WP-16 (S), `0142`'s structural half, **ST0065** (two WIP work packages, ZERO acceptance criteria).
8. **File as an issue, do NOT build** (vc's ruling): the `VIEW_NAMES` / `Project::classify` overlap gate. **A written-down-and-unbuilt check is dc's `spelling_note` class** -- it wants a reader, not a fifth thing on my plate.
9. **Owed:** `rustfmt::skip` is used in ZERO places and that is load-bearing. **`AC-17.10`'s soft-wrap flags.** **EMBED's pty** is its own build; what shipped is full-pane.
10. **Not mine, noticed today:** `edit_prints_a_path_that_exists.rs`'s one-home guard asserts **`AC-05.3`** in its message, and `AC-05.3` in the register is about **`.bats` file classification** -- a different subject. Good guard, wrong citation. vc's to route.

## Watch-outs -- mechanisms only

**A lesson that now has a guard is not here; the guard is the durable form.**

1. **A CORRECT PRINCIPLE APPLIED AT THE WRONG RADIUS, VISIBLE ONLY FROM OUTSIDE THE CRATE YOU ARE STANDING IN.** **Ask which crate the OTHER consumers live in before deciding where a derivation goes.** Applied BEFORE the fact twice now -- `form::raw` and `nav::Landing` both went to `intentsvcs` because the web face needs them and `intentd` cannot reach the CLI.
2. **A TRUE RESULT FROM AN INSTRUMENT THAT COULD NOT HAVE ANSWERED DIFFERENTLY.** Still the dominant class. **The sharpest form is the PIPELINE STATUS, which arrived by three routes in one day, each reading as good news:** `... | head -20` then `grep -c FAILED` returning 0; `cargo test ... | tail -20` reporting **EXIT 0 while a test FAILED**; cc's `cargo fmt --check | head` at rc=0 while rustfmt refused. **A status taken from the end of a pipeline is the LAST STAGE'S status**, and the `head` was there for readability every time -- a formatting decision became a population decision with nothing in between. **Ask what your instrument could not have seen, not whether it agrees.**
3. **A MEASUREMENT WHOSE SUBJECT IS NOT THE ONE NAMED.** **A criterion that names one half by name licenses you to stop looking at the other.** And **state the LIMIT of your own green**: `-p intent-cli -p intentsvcs` does not build `intentd`, so a figure from it says nothing about those targets.
4. **A REAL ANSWER RESTING ON THE ORDER OF A LIST.** **Define the default BY EXCLUSION and assert with the list in both orders.**
5. **THE PROPERTY THAT MATTERS OFTEN LIVES BETWEEN TWO SOUND INSTRUMENTS.** And **an AGREEMENT REACHED THE SAME WAY TWICE IS NOT A CHECK.** **Ask whether your method could have produced a different answer from theirs.**
6. **A DECLARED THING NOTHING READS IS THE SAME DEFECT AS THE VALUE IT ANNOTATES.** **Give prose a POINTER an instrument can find**, or expect it to be re-derived.
7. **REACHABLE AND BLANK IS THE FAILURE THAT READS AS DATA.** **Distinguish NOT BUILT from EMPTY in the return type.** It came back today through the front door I was building: `address::promote` is purely SYNTACTIC, so `ST9999` resolves perfectly and would have painted a form whose every value is blank -- **indistinguishable from a thread that exists and is empty.** A grammar answers about spellings; only the store answers about existence.
8. **`gen_dispatch_table.sh` REFUSES ON EVERY DERIVED COUNT AND HAS NEVER BEEN WRONG.** _Do NOT adjust the label to make the number come out._
9. **`git commit` COMMITS THE INDEX AS IT STANDS -- AND SO DOES `git commit --amend`.** `add` + `commit --only <paths>` is the only safe write, **and the rule must name AMEND or it has a hole shaped exactly like the next instance**: my original commit was correctly `--only`-scoped and the amend threw the scoping away, sweeping seven of cc's files under my message. **An amend reads as an edit rather than as a write.** Fifth instance of `0157`, first with me as the sweeper. **After ANY commit, `git show --stat HEAD` and count the files against what you staged** -- it is the only diagnostic either side has.
10. **NEVER ASK A PEER TO DO WHAT THIS SESSION WAS DENIED**, and never adopt a change you cannot attribute to yourself.
11. **`cargo fmt --all` AND `cargo fmt -p <pkg>` ARE THE UNGUARDED TWINS OF A GUARDED DOOR.** `rustfmt --edition 2024 <the files I edited>` only -- package scope is a write over every file any peer has open. **Never `--no-verify`**; when the critic gate is wrong, reword and file it.
12. **SHELL QUOTING EATS CONTENT SILENTLY, AND zsh IS NOT bash.** `$var` does NOT word-split. Backticks in double quotes are command substitution; an apostrophe in single quotes is a hard syntax error. **`2>&1 > file` sends stderr to the OLD stdout** -- `> file 2>&1` is the form. **Use a quoted heredoc for anything carrying prose.**
13. **THE WORKING DIRECTORY PERSISTS BETWEEN TOOL CALLS.** Six times in one day, and once it made a `cd` fail so the command NEVER RAN while the harness still reported exit 0. **Absolute paths.**
14. **A HUNG RUN AND A LONG RUN ARE ONE OBSERVATION** (cc's, adopted -- then walked into four times in one hour). **Today it was a test of MINE spinning at 100% of a core**, and I told hv AS FACT that I was blocked on the build lock. `ps` would have shown ZERO rustc, exactly as it did for dc, who found it by sampling the stack. **WHAT STOPPED ME MEASURING WAS A CORRECT GENERAL MODEL:** a contended lock is real, it happens here constantly, and it explains the symptom perfectly -- so the true explanation was never asked for. **A model that explains an observation is not evidence that it caused it.** Cure: every `cargo test` under `timeout`, and `ps` before saying the word _lock_.
15. **A WALK WITH NO BOUND IS WORSE THAN AN ASSERTION THAT FAILS** (dc's, adopted). The failing assertion names the invariant; the walk says nothing while consuming a core, and no timeout fires because nothing is blocked. **And the bound firing IS the finding.** Mine was unreachable BY CONSTRUCTION: **`Focus` CARRIES ITS OWN LENGTH**, so pointing an app at a row set and then GROWING it leaves the cursor bounded by the old count.
16. **A WHOLE-TREE WRITE REACHED FOR TO ANSWER A QUESTION ABOUT YOUR OWN FILES.** My `git stash` on this shared checkout; cc's `cargo fmt -p <pkg>`. **The narrow question has a wide verb sitting next to it that is one word shorter to type.**
17. **A PREMISE ABOUT A SHARED TREE HAS A LIFETIME MEASURED IN MINUTES, AND NOTHING REPORTS ITS EXPIRY** (cc's, adopted; dc walked into it the same hour while catching me). A guard is CORRECT WHEN RUN and FALSE WHEN RELIED ON, with nothing changing but time -- **and a guard reads as permanent in a way a measurement does not**, so nothing prompts a re-run.
18. **AN ORDINARY LANGUAGE IDIOM THAT IS A LOAD-BEARING DELIMITER IN THIS REPOSITORY, WITH NOTHING AT THE POINT OF USE SAYING SO.** A second test-cfg attribute in `render.rs` dropped 4,400 lines out of `no_pm_state_in_output.rs`, which truncates every shipped source at the first one. **A warning fix is a change like any other.** And **the note explaining the trap SPRANG IT AGAIN**, because the scan counts raw substrings -- the same shape as `IN-RS-CODE-004` refusing the comment that explained its own fix.
19. **A GREPPABLE PROXY THAT STOPS MATCHING ITS SUBJECT REPORTS THE FIX AS A WORSE VIOLATION.** The one-home guard counted `.edit(&address` -- a fact about how one caller named a LOCAL -- so extracting the single door both callers now use took it from 2 to ZERO. **Repair by asking the question the property is about**, then positive-control it with a planted violation.
20. **WHEN A STATUS CANNOT TELL TWO WORLDS APART, GO AND LOOK AT THE THING ITSELF** (cc's question, my answer). cc could not tell whether their `fmt` had rewritten my bytes because `--check` was rc=0 either way. **Six load-bearing phrases still present is a discriminating observation and it took thirty seconds.**

## Decisions

- **(hv) 3.0.1 scope is ST0056 + ST0058/0066/0068. No 3.1.0.**
- **(hv, RULED 2026-08-30, OVERRIDING vc) `intent explore [address]` TAKES AN OPTIONAL ARGUMENT.** vc has WITHDRAWN their no-address ruling rather than narrowing it, and reworded `AC-17.12`. **The fallback ANNOUNCES itself** -- vc ruled that _opens at the root_ contrasts with REFUSING, not with TELLING.
- **(ic, measured) `explore` and `edit` DIVERGE ON A MISS AND NEITHER IS WRONG:** `edit ST9999` REFUSES because it was asked to act on a specific thing; `explore ST9999` opens at the root because it was asked to open the explorer.
- **(ic, measured) TWO RATIFIED VOCABULARIES NAME THE SAME THINGS DIFFERENTLY.** An address says `/threads/ST0056/ac`; a view path says `/thread/ST0056/criteria`. **Nothing declares they are one concept**, so `nav::view_for` is an AUTHORED translation, is the one home, and is held against the declaration by test. **Whether that is itself a Highlander question is vc's.**
- **(vc) `AC-17.8` IS OPTION A**, and the refusal HAS a door: `acceptance.md` is in `arg_values(table, "edit", "file")` and classifies as a generated view. The five split 3 `Open` / 1 `OpenRoundTrip` / 1 `Refuse`.
- **(vc) DERIVATION IS SHARED, RENDERING IS PER FACE.** `Stack` stays in the face: it is STATE.
- **(vc) A CRITERION CONTRADICTING A RATIFIED DESIGN IS THE CRITERION BEING STALE.** Applied to `AC-17.11` and now `AC-17.12`.
- **(vc) A KEY NAMED FOR ONE REASON CANNOT HOLD MEMBERS ADMITTED FOR ANOTHER**, and **a bucket with no discharge condition is a parking bay** -- which is why `UNPROVEN` grew to 32.
- **(ic, AC-05.1) `organize` is EXPOSED on MCP** -- the rule is UNDOABILITY, not blast radius. **(ic) `C-g` cancels the MENU.**
- **(ic) Every path segment is a name the declaration already carries.** No pluralising.
- **(vc) Register writes are ic's.** `dispatch-table.json` is AUTHORED canon; `.md` is GENERATED.
- **(dc/hv) ST AND WP GET NO STATUS VARIANT** -- `fiat` sits BESIDE a status that stays `completed`/`done`.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only.
- **(ic/cc/dc, today) A COMMIT MESSAGE IS NOT A DURABLE HOME FOR A FINDING** -- no verb can edit one, and a swept commit loses its reasoning entirely. **The record goes where a reader will meet it: the board, or an inbox, with a pointer to the sha.**
