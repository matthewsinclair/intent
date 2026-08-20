---
node: dc
name: DevX Claude
role: worker
session_id: baf3a3a8-2d05-4e9a-8170-c1bdf1f0753c
heartbeat_at: 2026-08-20 14:30Z
status: active
focus: "**FOLDED 2026-08-20. Five landed: the guard roster delegated, `core.hooksPath` (hooks tracked), the rust formatter stanza, AC-06.3 (`md` sayable), AC-06.4 + AC-07.1 (`init` from the binary alone).** Tree clean, main builds, clippy clean, 964 pass. **NEXT IS THE RECORDED-NOT-BUILT LIST AND ITS TOP HAS CHANGED: `int check format` and clippy both exist with nothing local dispatching them, and between them they cost two CI reds today** -- measured twice now rather than argued about."
claims: [ST0056/07, ST0056/11, ST0057/04, ST0057/06]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.**

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.**
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE.
- **hv's structural close: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times. **IN is forbidden, OUT is fine.**
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES.** A time-typed input parameter is a defect by inspection. **Caught me on 2026-08-20 in `init.rs`** -- `fn substitute(.., date: &str, ..)` directly under a module comment claiming no time enters. Now a closure over the store's returned stamp.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it is just a label".
- **A board stamp is a label, not data.** The ordering that cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The db is the durable SSOT, files are re-creatable; the typed API is the only door in; migrations are normal.**

## DOING

**Nothing in flight. Tree clean of my work; `main` builds; clippy clean under `-D warnings`; 964 pass / 0 fail.**

## TODO

### 1. Recorded, NOT built -- and the top one has now been measured twice

- **`int check format` AND CLIPPY BOTH EXIST AND NOTHING LOCAL DISPATCHES EITHER.** `config.yaml:133` is `cargo fmt --all --check`, non-mutating, with no trigger; clippy has none either. **Between them they cost two CI reds on 2026-08-20** -- 19 fmt divergences accumulated over four days because nobody pushed, and a needless borrow reached HEAD because no local gate runs clippy. **Recorded-not-built is now a measurement rather than an argument**, which is the standard I said I would hold it to before building it. The obstacle is real: both are workspace-wide, so with four writers live they would refuse on somebody else's file.
- **`int hooks` UNDER-REPORTS THE SHIPPED GUARDS BY FOUR.** It prints the repo-local roster (11) and is silent on the four shipped guards dispatched through `pre-commit.intent`. **The tool people consult to find out what the gate enforces is the thing that is wrong.** One more `--list-guards`-shaped call, not a new mechanism. (The GATE line I added covers presence, not roster.)
- **`intent claude upgrade --apply` IS ALL-OR-NOTHING.** On this tree three of four actions are unwanted, including **regenerating AGENTS.md from 3.0.0 DOWN to 2.19.0** on a self-hosted v3 tree. The new `GATE STALE` message points operators straight at that button, so it is a fleet question.
- **`bin/int SAYS` IS UNBUILDABLE WHERE hv PUT IT, AND IT IS BACK WITH HIM.** `bin/int` is stock vendored devbin; its own header records a consumer who edited it having _forked the dispatcher_, and `self_provenance_check.sh` gates that tree. `project.referent` is taken by `cmd/measured`; `SessionStart` runs shipped canon. **And structurally: if `core.hooksPath` is unset no hook runs, so no hook can report it.** Two options put to hv: a pre-dispatch hook in devbin upstream, or `int hooks` as a documented clone step.

### 2. Mine and small

- **`author`/`content` PRINT NOTHING WHERE v2 PRINTS 136 BYTES.** My clean no-op returns `Ok(())` before any output, so a prose language is indistinguishable from one that ran and found nothing.
- **CLI-level tests for `critic`** -- the module has 18; the command surface has none.
- **`critic --no-such-flag` EXITS 1 WHERE v2 EXITS 2 -- the gate BLOCKS on a typo.** Issue 0043 on the git side. **Not a one-token fix**: the spine's clap-error path plus INV-02's `critic` exception, so mine and ic's together.
- **`--languages` ARITY IS A PROPOSAL TO ic, NOT MY EDIT.** Fix site is the TABLE (`"arity": "1"` -> `"0..1"`), which is ic's SSOT.

### 3. Owed decisions I said I would rule once

- **`canon_commit_check.sh` ADMISSION.** cc has driven `--staged` and both recorded objections are dead: the narrow arm is 3.7s, the inherited clause is structural, `--exhaustive` sees what narrowed excludes. **The number I want to think about is 190 pre-existing divergent attachments of 286** -- a gate whose whole-set mode would refuse two thirds of the estate is one whose ADDS-only narrowing is load-bearing rather than a convenience, and that changes what admitting it means.
- **`thread_view_skew_check.sh` ADMISSION** -- gated skew stays 1 of 269 until I rule. The roster it was blocked on is now one live home, so the question has changed shape.
- **FIVE INHERITED FLAGS from cc's AT-06.8.** `st bootstrap --audit-only` / `--dry-run` / `--deliverable` are mine when `bootstrap` lands. The ratchet is equality, so wiring one reds the test and forces the list to shrink -- that is the mechanism, not a nuisance.

### 4. Standing

- **WP-07 hosting sweep**: `agents` (42 call sites), `lang` (44), `claude skills` (23) / `subagents` (25) / `ws` (13) / `prime` (10). `upgrade` (29) and `start` (6) **unmeasured**, excluded by name because they write outside the sandbox.
- **AT-11.6 / AT-11.7** still `to-write`. **AT-11.6 takes a row citing `prepush --force` and NO new code** -- the mechanism existed all along and I reimplemented it worse. 11.7's positive control is the `intentd` fossil marker.
- **D37 payload sweep, `AT-00.17`** at `no_pm_state_in_payload.rs`, minted `to-write`. Discriminator is CITATION versus FORMAT EXAMPLE; cuts ~80 sites to ~15 with no per-site judgement. Two unplanted positive controls verified at zero hops.
- `output-contracts.md`; `doctor` v3 mirror (XS).

## Watch-outs

**Today's instances are verbatim in `.history/20260820/wip.md`; 08-19's in `.history/20260819/watch-outs-full.md`. These are the CLASSES.**

- **MARK PROVENANCE PER CLAIM, NOT PER MESSAGE -- driven, read, or inferred.** cc sent four claims in one voice; three were driven in code and one read off a directory listing, and nothing said which, so I spent a careful check on the wrong one. **The cost lands on the READER, which is why the writer never feels it.** True of most of my messages today -- _the guard is perfect and has nowhere to run_ went to hv in the same register as things I had driven. **Reasoning wearing a measurement's clothes.**
- **VERIFY THE RETRACTION, NOT JUST THE CLAIM.** A self-correction is still a report, from the one person already wrong about that subject, and it arrives with the momentum of having just been careful.
- **`git commit --only <paths>` IS PATH-SCOPED, NOT HUNK-SCOPED.** It defends against a peer's STAGED index and does nothing about their UNSTAGED edits in a file you also touched. It put ic's caller at HEAD without its callee and **`main` did not compile**. **THE THREE REMEDIES ARE NOT INTERCHANGEABLE:** building the workspace first FAILS BY CONSTRUCTION (the shared tree compiles either way); `git diff --cached -U0` catches it and needs a human to recognise a stranger's hunk; **a detached worktree at the named revision catches it MECHANICALLY and is the only one that does.**
- **A FIX THAT CHANGES A TYPE LEAVES EVERY USE OF THAT VALUE UNVERIFIED, and the ones that still compile are exactly the ones nothing reports.** One edit produced two silent survivors: `.ok()` swallowing clap's `UnknownArgument`, and `&author` outliving the `String` it was written for.
- **AN INSTRUMENT KEYED ON A CODING IDIOM DEPENDS ON NOBODY IMPROVING THE CODE** (cc's name for it). One level below ST0039's _substring standing in for a syntactic fact_: the syntactic fact was itself an accident of style. My loop avoided duplicating a string -- right in that file -- and made the id invisible to a scanner keyed on literals in another.
- **SYNTHESISE THE INSTANCE _AND_ KNOW THE LIVE POPULATION.** A red-first arm needs a SYNTHETIC instance or the estate is not free to fix its own defect (vc). **But a green suite over synthetics alone cannot tell you whether the feature has ever RUN** -- five green tests, live population zero.
- **A POPULATION CAN GO OUT FROM UNDER A CHECKER, AND A NAME CAN SPAN POPULATIONS.** Four instances in a day: a dispatch test blind to the two guards that never ran; `emitting_names`/`refused_names` losing a format to a new variant; `error_remedies.rs` provoking a refusal with a format that stopped refusing; a `withheld > 0` guard whose subject shipped. **And `grep -c` on a field name with three subjects is a count of nothing until you say which level.**
- **SILENCE ON SUCCESS IS INDISTINGUISHABLE FROM NOT RUNNING.** A guard that passes prints nothing; so does a runner nothing dispatched. **THREE INSTRUMENTS EXISTED WITH NO TRIGGER IN ONE DAY**: `canon-ignore-guard.sh` (no roster), `prepush` (nobody reached it), `check format` (no dispatcher).
- **THE PROBE POPULATION CAN EXCLUDE THE ANSWER AND NEVER SAYS SO.** `cargo test` stops at the first failing binary; a `head -20` hides the twenty-first; a failing assertion hides the ones after it. **Read the target list, not the last `test result:` line.**
- **THE EXIT CODE IS NOT WHERE YOU THINK.** A pipe eats it. **`grep -c` exits 1 on zero and a pipe eats THAT** -- empty output reads as absence, and it nearly reversed a correct correction. **Drop the `-c` and read the lines.**
- **MEASURE IN THE SHELL THAT WILL RUN THE CODE.** My shell is **zsh**; the hooks run under **bash**, which strips NUL bytes in command substitution and says so. A `-z` pipeline measured clean in zsh and was broken in bash.
- **A CLAIM ABOUT A MUTABLE SUBJECT MUST NAME ITS REVISION.** Name revision, clock and dirty count on every measurement.
- **STANDING CONSTRAINTS.** Push `local` only unless hv says otherwise; `upstream` runs the only CI and its freeze is a **CI/CD BUDGET** freeze, so pushing spends the thing it protects. NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. Timestamps READ FROM `date -u`; `git log` prints LOCAL time. The markdown formatter is a second writer.

## Decisions

- (2026-08-20) **THE COPIED FILE NAMES NO GUARD AND HOLDS NO ROSTER.** The bodies resolved live and the array naming them did not, so canon gained guards and consumers did not. Roster lives in `pre-commit-guards.sh`, read live, never copied. **Adding a guard is one line and reaches every consumer with no reinstall.**
- (2026-08-20) **HOOKS ARE TRACKED VIA `core.hooksPath`, NOT VIA AN INSTALLER THAT COPIES** (hv). The copy mechanism is the disease just cured. `.githooks/` is tracked; `pre-commit.intent` is **gitignored inside it** because it is canon the installer writes, and tracking it would be a second home for canon.
- (2026-08-20) **`export --format md` DELEGATES TO `Facade::realise()`** (hv). **The premise that expired is the POPULATION, not the fact** -- markdown still cannot be read back. A verb returning a destination for tree-shaped output and a document for document-shaped output is describing reality, not compromising.
- (2026-08-20) **A TEMPLATE IS EMBEDDED BECAUSE `init` WRITES IT; A GUARD IS READ LIVE BECAUSE THE GATE DISPATCHES IT.** Two mechanisms, two populations, stated rather than implied -- which is why `lib/templates/hooks/**` is deliberately NOT in the embed.
- (2026-08-20) **THE ENV READ BELONGS IN `bootstrap`, NOT `init`.** v2 already puts it there. Removing it beat getting an ALLOWED row: **no ruling needed is a better outcome than a ruling.**
- (2026-08-20) **A REFUSAL WHOSE REASON EXPIRED IS A LIVE DEFECT, NOT AN OWED CAPABILITY.**
- (2026-08-20) **AN INSTRUMENT THAT REPRODUCES THE DEFECT IT WAS BUILT TO CATCH IS WORSE THAN ONE THAT UNDER-REPORTS AND SAYS SO.** The two errors are not comparable, so the conservative instrument wins and its false negatives get named in the file.
