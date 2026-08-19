---
node: cc
name: Control Claude
role: control
session_id: 706144a4-c6ed-49be-8c4d-8835993e7930
heartbeat_at: 2026-08-19 07:15Z
status: active
focus: "**WP-01 CODE IS COMMITTED (`f41d6760`) AND THE 57 + 40 CANON FILES HAVE NOT MOVED. `intent/.canon/` DOES NOT EXIST.** Landed today: ST0056 AC-03.14 (the WriteSet unchanged-skip, measured live by vc at 0 of 557), its 30-of-31 derived coverage, hv's todo tree bug, ST0057 AC-01.7 (openness), AT-01.1 (the location oracle) and AT-01.6 (resolver singularity). **Workspace 647/0/0 across 88 suites, fmt clean, native/ and schema/ CLEAN.** Remaining in WP-01: **THE LIVE MOVE** and AC-01.5's commit guard. Binary `f2e4d1f9` -- vc holds pins; ANNOUNCE BEFORE ANY REBUILD."
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. **D34: the committed extract is the interchange -- it TRAVELS while the DB never does.** D29: a gitignored path is never canon. **ST0057 is INSIDE the 3.0.0 gate** (hv).

**D42 -- TIME, and it has no clauses.** The create door stamps; the restore door carries. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z`. `one_clock.rs` enforces it structurally and has caught me.

## NEXT: THE LIVE MOVE, AND EVERYTHING IT NEEDS IS BELOW

**The code is committed and the files have not moved.** 57 `intent/st/<ID>/thread.json` -> `intent/.canon/st/<ID>.json`, 40 `intent/issues/<NNNN>.json` -> `intent/.canon/issues/<NNNN>.json`. All 97 tracked; use `git mv` so history follows. **vc CLEARED it (`25605e6b`) and will not touch ST canon until I say it landed. ANNOUNCE FIRST; ping vc and ic after.**

**MY BOARD WAS WRONG ABOUT THE ISSUES DIRECTORY AND THE CORRECTION MATTERS: `intent/issues/` ALSO HOLDS 42 TRACKED v2-LEGACY FILES** under `OPEN/` and `CLOSED/`. **ONLY THE 40 CANON FILES MOVE.** The legacy stays, correctly: **`legacy.rs:565` composes `intent_dir().join("issues").join(bucket)` ITSELF and never goes through `issues_dir()`** -- AC-01.6 governs CANON paths, and v2's layout is a historical fact that does not move. A migrator resolving v2 paths through the v3 resolver would hunt the old estate in the new place.

**THE REQUIRED STEP, AND THE FORM I FIRST WROTE COULD NOT GO RED** (vc caught it, ic owned it). `st list` DEFAULTS TO IN-PROGRESS AND RETURNS **2** -- run it bare before and after and you read **2 and 2 and call it a pass** while the 52 Completed threads it protects never enter the output. **RUN `intent st list --status all` -> 57, AND PRINT THE BREAKDOWN: 52 Completed / 2 WIP / 2 Cancelled / 1 Not Started**, because a Done->WIP corruption preserves the total. **No shape check substitutes -- the binary that wiped the estate had a PERFECT surface.** Expect `surface_check.sh` rc=2 naming `intentsvcs/src/project.rs` (ic widened its reach at `7964a467`): **that is the tool working.**

**CLASSIFY THE TWO-ENDED SITES BEFORE MOVING, on the discriminator now in canon at `c88a6f06`: WHETHER THE UNMIGRATED END CAN STILL PRODUCE A VALUE.** A missing directory cannot -- it fails loudly with `NotFound`. A string operation always can: ic measured `${var#pat}` returning the string UNCHANGED, so a half-migrated extraction emits `ST0056.json` as a steel-thread id with nothing complaining, **and fixing either end alone produces no observable change.** So two-ended sites are NOT uniformly risky. **The move happens ONCE; classifying costs less than driving all of them and strictly less than missing one.**

**Then AC-01.5's commit guard** -- refuse at COMMIT the edit that adds `intent/.*/` to `.gitignore`. **Every other `intent/.<x>/` IS gitignored, so the convention reads "a dot directory under intent/ is local".** `.canon/` is the single deliberate exception, and a tidy-up would silently un-commit the whole estate. AC-01.2 checks the STATE by cloning; AC-01.5 refuses the EDIT.

**ic is holding `realise_plan.sh:44` until I ping** -- the shape is confirmed: `"$ROOT"/intent/.canon/st/*.json`, flat, id read from content by `jq -r '.id'`. **dc's `canon_commit_check.sh` breaks THREE ways and is theirs.**

## OPEN -- mine

1. **The live move + AC-01.5, above.**
2. **ST0057 AT-03.15's ten-verb hold** (vc, `11fdc168`): `at lint`, `ingest`, `st bootstrap`, `st repair`, `st sync`, `todo`, `todo done`, `todo list`, `todo notdone`, `todo toggle` -- each must become driven or carry a stated reason. **Start with `todo list` and `at lint`: both are classified `mutate` while their names say read, and either answer is worth having.** The other 22 unproven are a `dispatch-table.json` debt, ITS OWN THREAD, explicitly not WP-01.
3. **ST0056 AC-03.16 / AT-03.17 (vc, `37295d62`, hv-ruled): `views.rs:329` + `:653` emit "_ACs and ATs live in `acceptance.md` -- the single source of truth_". `acceptance.md` IS A GENERATED VIEW.** It earns a row because it is a **WORK-LOSS INSTRUCTION**: a reader who acts on it authors a row that the next `--to-disk` discards. **206 of 207 covers; the one that escapes is my `carries_heading` guard working, so DO NOT "fix" it. Do not assert 206** -- assert the property, print the denominator. Read EMITTED VIEWS, never the generator's string literals. **`views.rs:382` must STILL PASS.**
4. **THE STALE-AT GUARD'S MIRROR BLIND SIDE (vc found it, mine).** It catches a `to-write` row citing a file that EXISTS and **cannot catch one citing FICTION** -- it passed clean all evening while AT-03.15 named `write_exactness.rs`, which never existed. **The green was TRUE and answered a question nobody had.** Wants its own red-first, as a row rather than a tidy-up.
5. **The cold-warm `doctor` arm stays PARKED behind WP-01** -- "a file on disk canon does not know about" CHANGES MEANING under sparseness, and building it now encodes today's dense-disk assumption into the one check meant to police the sparse one.
6. **`critic rust` and `critic shell` arm ZERO rules.** A green from either means "nothing asked a question". dc owns Half B.

## Watch-outs -- the mechanisms, distilled

**A CRITERION MUST CLOSE EVERY DEGREE OF FREEDOM THAT LETS A PASSING TEST COEXIST WITH THE DEFECT** (vc). Four limbs, one rule, every one with a live instance from a different node inside two days:

| limb                                                      | the freedom                                                                 | today's instance                                                                  |
| --------------------------------------------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| **INSTRUMENT** -- which tool                              | git and mtime give OPPOSITE answers                                         | AC-01.4; the fix was in the criterion, not my reading                             |
| **DEPTH** -- how far in, of SUBJECT and DENOMINATOR alike | an internal subject lets the test reach PAST the thing tested               | `view_determinism.rs` drove `write_all` and stayed green while the estate churned |
| **EXTENT** -- how much of the right KIND                  | a subset of the right kind, **both figures observable, so DEPTH passes it** | 20 of 20 views against a writer population of ~364                                |
| **PIN** -- at which revision                              | measure at a NAMED COMMIT, never `HEAD`                                     | three `intent` binaries carried `dirty-18197aaf` in one day                       |

**IDENTITY AND EXPECTED VALUE MUST COME FROM OUTSIDE THE THING UNDER TEST** (ic's unification). Content-derived identity survives a relocation; path-derived does not. A written-down expected value is a check; one derived from the subject is a tautology. **Take either from the subject and the artefact loses the ability to disagree with it.**

**A PROBE WHOSE POPULATION CANNOT CONTAIN THE FAILURE IT TESTS FOR.** `sync` printing _the store and the extract agree_ over **0 == 0**; `git status` vouching for an mtime-only prediction it is blind to; an all-changed subset vouching for a skip that never fired; **and my own required step, which I minted deliberately after naming the class.**

**A TOOL'S DEFAULT VIEW IS A FILTER, and a command passed on without the flag it was MEASURED with silently inherits that default** (ic). Nothing about the transmitted form looks incomplete, which is why it survived three readers.

**A CHECK THAT VALIDATES THE FORM OF A CLAIM AND NEVER ITS REFERENT.** `openness.rs` required a declaration to start with `carried by ` and never looked at what it named. The drift checker would have blessed `intent/.canon/<ID>/thread.json` -- right prefix, wrong flat shape -- **because the wrongness is in a dimension it does not inspect.**

**MUTATION-TEST EVERY GUARD, AND RE-TEST WHEN THE MECHANISM CHANGES.** **Twice in one day a correct guard sat on a path the failure does not take** -- the AC-03.14 skip on `write_all`, and AT-01.6's first version, which stayed green with the historical bug re-introduced. **Both found by mutating, neither by reading.**

**THIS WORKSPACE HAS TWO CRATES AND ITS MEASUREMENTS KEEP HAVING ONE** (vc). I asserted "the fixtures are fine" from `intentsvcs`'s shared fixture; `intent-cli` has SEVEN private `seed()` copies. ic retracted a claim the same evening whose reach was also one crate of two, opposite direction. **Until an instrument states its crate coverage in its own output, both of us keep paying.**

**`target/release/` IS A SHARED MUTABLE ARTEFACT four nodes read.** Snapshot and sha256 before measuring; **announce before rebuilding**. `int prepush` does it right: clone to a temp dir, build there. **A rig assembled by SYMLINKING into the real tree is NOT isolated** (dc): `cp` onto a symlink follows it and writes through.

**`cd` DRIFT: use absolute paths.** Five times today. **NEVER TRUNCATE A TEST RUN** -- sum every `test result:` line. **`git commit --only` ON AN UNTRACKED PATH STAGES NOTHING** and reports a true count about what it DID commit.

## Standing rulings

- **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE.** Both threads carry an `AC-04.4`; four carry an unrelated `AT-03.6`. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE, which is the one failure mode that stops a reader looking further.**
- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything found building v3 is work.
- **An uncarried file is NOT a disposition** (vc). **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON -- and re-deriving the reason is how you find out the reason was wrong.**
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING. **`EdgeKind::Incidental` STAYS with no user. `doctor --fix` is WITHDRAWN. `Outcome` is deliberately NOT `#[must_use]`.**
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.** **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`.**
- **`config.json` DOES NOT MOVE WITH `intent_dir`** -- `Project::config_path` always answers `intent/.config/config.json`, because something must be findable before anything is configured.

## Lane boundary

`native/**` and the v3 crates are mine. `bin/**` is not vc's to edit. The parity harness is ic's; `canon_commit_check.sh` is dc's. **Every commit I make touching an attachment leaves canon divergent at that commit until vc syncs, and a later sync repairs the NEXT commit and never that one** -- commit and ping vc; it is AC-08.5's missing operation, not my failure.

**dc's 88-binary consolidation: they STOOD DOWN.** Their premise was refuted -- matts measured Lamplight's suite at ~10 minutes against our 1m56s, so **we are the faster project and binary count does not explain the slowness.** Their real signal is that the identical suite ran in 16.12s and 100.10s back to back: **contention across nine sessions on sixteen cores.** `CARGO_TARGET_DIR=native/rust/target/cc` is worth taking **for tests only** -- a release build must keep landing at `native/rust/target/release/` where four nodes read it.
