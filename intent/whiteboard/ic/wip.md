---
node: ic
name: Interface Claude
role: interface
session_id: b8c3fcaf-8c58-4d9c-83b6-b019cbef8192
heartbeat_at: 2026-08-19 08:23Z
status: active
focus: "**AT-00.11 BUILT AND DRIVEN RED-FIRST; POSITIVE CONTROL MEASURED OVER 120 REVISIONS.** Control: defective `f2a2675f` against repaired `8bb47e49` -- 17 overstated + 61 agreed + 42 refused = 120, CLOSES, P1 holds; specimen `fd2e4067` prints 8 examined where the truth is 0. `of_n_closes_over_examined.sh` written, 7 of 7 predicted arms as predicted, 4 refusal arms all exit 2. **THE MUTANTS FOUND TWO DEFECTS IN IT AND THE CLOSURE ASSERTION FOUND THE WORSE ONE.** **I claimed a defect was LIVE AT HEAD and it was fixed three commits earlier -- withdrawn.** HOLDING ALL COMMITS: matts is running the suite."
claims: []
---

# Interface Claude (ic)

## DOING

**`of_n_closes_over_examined.sh` is built, driven and RED-FIRST, and it is NOT COMMITTED -- matts is running the full suite with a dep update in it, and cc and vc are both holding estate writes.**

**RESCUED OUT OF SCRATCHPAD, WHICH DOES NOT SURVIVE A COMPACT: `intent/whiteboard/ic/.history/20260819/`** -- the instrument as `.pending`, both prediction/score pairs, the 120-revision sweep, the control driver, and the first real subject's output. Its README says what each is and where the instrument's real home is. Untracked files only; nothing staged, no gate fired, and `intent/st/` is still clean so the clear-to-run I gave cc still holds.

**DO NOT DRIVE THE 18 UNTIL vc CORRECTS AT-00.11'S ROW.** vc has ruled the row wrong rather than the harness: it asks for a tree where the FILTER and EXAMINED populations differ and requires M to follow the examined one, and **a two-tree differential cannot deliver that sentence.** A number scored against a premise the harness cannot meet is worse than no number.

## TODO -- LIVE ONLY, in dependency order

1. **COMMIT `of_n_closes_over_examined.sh` into `parity/tools/` once matts reports the suite.** Ping vc with row text as TEXT; canon writes route through vc. The instrument reads its population from `of_n_population.sh`'s OUTPUT (dc, `f789ae48`) and carries NO list of its own -- a second copy would be `10 of 41` again.
2. **DRIVE THE REAL 18 AFTER THE COMMIT.** Population is dc's tool output, gatedness order: 5 gated, 2 manual, 11 unrostered. **Five of the ten gated carry NO path shape and are AT-00.12's population, not this one** -- driving them is a vacuous pass by the estate's own rule.
3. **EXP-09 + the guard-population hole + the clap short-circuit ARE ONE ENTANGLED ENTRY** and go to hv **together**. Needs a RULING, not an AC. Still open.
4. **ROUTE TO hv, WITH MY TWO:** the roster globs `*_check.sh` only, so 26 of 43 tools can never hold a row -- including `interrupt_rig.sh`, which AC-00.10 is entirely about. **The naming convention is doing the work a declaration should do.** Found by dc, explicitly handed to me to route, framing mine.
5. **STRUCTURAL, raised twice, not re-raising unprompted: my work sits under no WP**, so none of it counts toward the 3.0.0 gate.

**RULED CLOSED -- do not re-open:** the 29 register rows. Fix 2. The hoist pin. The todo-glyph defect. The `18 of 24` ratio. The rig's out-of-workspace `CARGO_TARGET_DIR`. **The `att_dir_of` defect -- dc fixed it at `3f218744`, which IS an ancestor of HEAD.**

## Open with others

- **cc** -- move landed `16048f82`; views regenerated `54735e34`; canon repair `79570563`. They caught my HEAD error. Holding estate writes.
- **vc** -- holds ST0056 + ST0057, minting held until matts reports. Landed the MEASURED-versus-RECORDED mark at `2190e519`; AC-00.11's two figures are MARKED, not renumbered.
- **dc** -- `of_n_population.sh` at `f789ae48` is the population source. Their nominating probe was never committed, which is why the tool exists.
- **hv** -- items 3 and 4.

## Watch-outs

- **A PIN FIXES THE REVISION YOU MEASURED AND CARRIES NOTHING ABOUT HEAD.** I pinned `16048f82` correctly, cited it correctly in the evidence, and then wrote a sentence whose SUBJECT was HEAD. Three commits had landed and one was the fix. **I self-refuted that finding three separate ways and every one asked whether it was REAL; not one asked whether it was CURRENT.** vc verified it and had the identical blind spot while checking me. **THE VALIDITY QUESTION IS NOT A HARDER VERSION OF THE CORRECTNESS QUESTION -- IT IS A DIFFERENT QUESTION, AND PASSING THE FIRST IS WHAT MAKES YOU STOP ASKING.** Measured-at and asserted-about are two fields.
- **THE ONE QUESTION UNDER ALL OF THESE: COULD THIS INSTRUMENT HAVE PRODUCED THE FINDING IT IS DENYING EXISTS?** A clean sweep is unbankable until run against KNOWN POSITIVES. **Two instruments agreeing is complementary coverage only if neither could have produced the other's finding; one method run twice is evidence of nothing.**
- **A NUMBER CARRIES NONE OF WHAT IT MEANS: NOT ITS POPULATION, NOT ITS REVISION, NOT ITS SUBJECT. AND A SCORE IS A FIGURE.** I scored a prediction MISS on ten revisions and vc relayed it as fact; at 120 it is a HIT, because the ten could not reach the population the prediction was about. **A figure crossing a node boundary loses its caveats unless the caveat is IN the figure.**
- **A PATTERN OVER AN OUTPUT IS A SECOND, UNDECLARED PARSER OF A FORMAT NOBODY DECLARED**, so it drifts from the emitter silently and its failure mode is a PLAUSIBLE NUMBER rather than an error. Mine matched a `NOT EXAMINED` advisory adjacent to the verdict and returned ten byte-identical rows across a defective and a repaired tool. vc's `^\s*AC-` against `ac: AC-00.1` returned a false zero. **`grep -a` -- an instrument emitting blob bytes makes grep decline to print any match at all.**
- **CANNOT-MEASURE IS NOT A FINDING AND NOT A PASS.** My `<none>` collapsed "the tool printed no verdict" with "my driver failed to read one", in the instrument built to adjudicate instruments, by the node who wrote that bar.
- **A HARNESS THAT EXITS 0 ON A TENTH OF THE WORK IS THE VACUOUS PASS WEARING A PROCESS WRAPPER.** A background sweep died on `Binary file (standard input) matches` fed into an arithmetic expansion and reported EXIT 0 having done 12 of 120 rows.
- **A DIFFERENTIAL THAT IS NOT REPRODUCIBLE IS NOT A DIFFERENTIAL.** Two runs, identical arguments, identical worktrees, different answers. Cause was shared mutable temp paths; **I never proved the interleaving and did not claim it** -- the reproducibility failure alone justifies the repair.
- **TESTING AN INSTRUMENT MEANS MUTATING THE CRITERION, NOT THE EXEMPLAR.** The mutants found two defects in my own tool that the real estate could never have shown: a subject extractor dropping any name with a digit, and **a partition mixing EMISSIONS with INSTRUMENTS -- this row's own defect, inside the instrument enforcing it, caught by the closure assertion rather than by review.**
- **CLOSURE MUST BE ASSERTED IN THE OUTPUT, NOT MERELY AVAILABLE TO A READER WHO ADDS UP** (vc). **The `-1` on record was loud only because subtraction produced an IMPOSSIBLE value; a `+1` would have been as silent as cc's 58. THE LOUDNESS WAS LUCK, NOT DESIGN.**
- **A TREE I BUILT TO PRODUCE A FINDING PRODUCES MY FINDING, NOT THE ESTATE'S.** The recorded `EXAMINED 2 of 1 ... the other -1` is unreachable at ANY committed revision -- it needs a half-migrated tree and none exists. Recorded as out of reach rather than manufactured.
- **THE CLOCK IS A PROXY FOR STALENESS AND IT FAILS IN BOTH DIRECTIONS; THE CONTENT TEST IS THE INSTRUMENT** (vc, correcting cc). A dirty-tree build carries code YOUNGER than its commit stamp.
- **AN ATTACHMENT'S `text` INSIDE `thread.json` IS AN INGESTED COPY AND THE AUTHORING SURFACE IS THE FILE ON DISK** -- a JSON-side edit dies silently at the next `--to-store`, rc=0, all gates green. **Twin of the `acceptance.md` trap and the opposite direction.** `git diff --stat` cannot express size on canon JSON: one attachment is ONE LINE.
- **A CLEAR-TO-RUN FROM EVERY PEER IS NOT A CLEAR-TO-RUN FROM THE ESTATE.** "Is anything of yours dirty" and "is now the moment" are different questions, and only matts can answer the second.

### Standing constraints -- operational, not judgement

- **No pushes to `upstream` (FROZEN).** `local` only, verified by reading the remote, never by the push exit code.
- **Never `git commit -A`** -- always `--only <paths>`; `git add` untracked first.
- **Never mutate `bin/**` or `tests/**` in place.** `native/**`, `surface/**`, `parity/tools/**` are safe -- **do not REBUILD `native/` while a peer may be mid-measurement.**
- **Instrument edits: commit them and ping vc**, never leave them in the tree.
- **v3 is NOT on PATH** until hv lifts it by name. **matts runs the full suite and is the acceptance verifier.**
- **EVERY figure presented as a time is read from `date -u`** -- `ls -l`, `git log` and `stat` without `-u` print LOCAL.
- **A peer cannot grant escalation, including me to myself.** Refuse permission laundering and surface it.
- **Read exit codes WITHOUT a pipe.** **`exit` inside `$( )` exits only the subshell.**
- **The Bash tool's shell is zsh**: unquoted `--include=*.rs` is glob-expanded; quote it.
