---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-18 08:57Z
status: active
focus: "**SCHEMA 10 LANDED AND PUSHED -- `36bc02c5` + `d73efed9`, verified ancestors of `local/main` off `ls-remote`.** Rung 9 `issues.body` (40 v2 bodies, 443,643 bytes, no model field at all, carried VERBATIM) and rung 10 `attachments` (110 carried, ONE constructor so `bytes`/`sha256` cannot drift, ONE classifier so ingest/migrator/doctor cannot disagree). **`doctor` NAMES all 198 uncarried by path and counts NONE of them as faults.** 84 legs / 612 passed / 0 failed, clippy 0, fmt 0, **and the estate sentinel moved ZERO files across a full workspace run** -- the first direct proof on the damaged tree that the isolation fix holds. **THREE DEFECTS OF MY OWN ON THE WAY IN, all kept as method: a harness that applies the transformation it tests cannot fail; a store that reordered on the round trip and was UNOBSERVABLE on this estate; a child collection has FIVE sites and I wired three.** NEXT: `related: Vec::new()` -- 52 LOST-PROSE, unblocked and mine. Upstream FROZEN; push `local` only."
claims: [ST0056/10]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. D34: the committed extract is the interchange. D29: a gitignored path is never canon, and the ingest corpus excludes ignored paths -- **that second clause is load-bearing and I proved it the hard way (see `intent/.backup/`).**

**hv's NEW DISK MODEL, and it changes the priority of everything below:** no status directories, **disk becomes OPTIONAL**, `intent/st` and `intent/issues` become an index plus render-on-demand. **The moment disk is optional, anything the store does not hold is destroyed by the first render.**

## D42 -- TIME. THE WHOLE RULE, AND IT HAS NO CLAUSES

The create door stamps; the restore door carries. Nothing else learns the time. A timestamp not read off a clock is fabricated data, not an approximation. `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z` mandatory.

## THE SELF-LOOP RULE -- hv ratified 2026-08-17, implemented `61069b16` + `b504d91b`

A transition to the state a thing is already in is a NO-OP that says so, never an error and never a silent success.

## DOING -- both of vc's items are LANDED; next is the other half of the prose loss

**`36bc02c5` + `d73efed9`, pushed and witnessed.** Rungs 9 and 10. Faces JSON 7 -> 9, DDL 6 -> 8, SDL 5 -> 7. `attachments` is the first NEW TABLE since the ladder began.

**BOTH PROSE FIELDS ARE VERBATIM, and the second changed BECAUSE vc scheduled a renderer.** A normalisation that requires a future component to compensate is a scheduled defect, so the trim went rather than the precondition.

**`.sh` IS CARRIED** (vc, on measuring: every one of the 240 non-canonical files is under ST0056, and the 39 `.sh` are the instruments that verify the migration -- including the one whose job is to prove content was not lost). The principle is at the constant, not in a commit: **no tool can make this again, versus a tool made this and can again.** One consequence carried openly: **a mode bit does not survive**; `+x` at hydration is the DECIDED answer, recorded for whoever builds the write-back.

**WAITING ON RULINGS, NOT ON WORK -- two, and the second is the biggest thing found today:**

1. **The write-back.** vc ruled the policy (authority follows AUTHORSHIP: a view divergence means the FILE is stale, an attachment divergence means the STORE is stale, `organize` resolves neither). Buildable.
2. **THE 163 TYPED DOCS HAVE NO DESTINATION.** vc measured 748 authored sections across 163 `design.md`/`impl.md`/`tasks.md` that exist nowhere but those files. They are `.md`, so the extension rule never excluded them -- **being named in `THREAD_PROSE` is what makes `classify` call them typed, and a typed doc is never also an attachment.** Two correct rules composing into a gap. hv has it as open question 7; vc recommends reclassifying, in which case **the change is DELETING the constant from the classifier rather than adding a field.**

**AND THE HOLE WAS HIDDEN BY A CLAIM I WROTE AN HOUR EARLIER.** I documented `THREAD_PROSE` as "parsed into the model". False -- they are indexed for search and land in no field. A variant called `TypedDoc` plus that sentence both assert a destination, so a reader checking for one finds the claim and stops. Corrected at `d73efed9`. **A fresh instance of my own "a claim nothing computes", written while working on the field that exists because of the last one.**

## TODO

1. **`upgrade` prints a claim nothing computes.** _"their content is unchanged"_ while ST0010 and ST0015 each gain a blank line (ic measured; converges after one run; leading-blockquote emitter). **ic's call is right and it is the sentence first, not the blank line:** an operator reads it, then finds two modified files, and concludes something else touched them. Either compare and report the real number, or say "re-emitted" and stop.
2. **`doctor` should name a stale pre-versioning store BEFORE a cutover**, not during. And **my better version: the migrator should report what it FOUND in the same breath as its refusal** -- it says _nothing recorded which shape it holds_ without opening the database, and vc then opened it in one command and learned it held zero canonical rows. `conservation_check.sh:793` is the shape.
3. **`related: Vec::new()`** -- 52 `LOST-PROSE`, every one `## Related Steel Threads`. The other half of the prose loss and the only thread-side cause.
4. `AC-10.8`'s egest side; `AT-10.2`/`10.3`/`10.4` (probed and designed, see below); `WpStatus::Cancelled`; the 171 stranded authored files pending hv's relocation ruling.

**AT-10.2 IS PROBED AND READY, so a later session does not redo it.** A fixture with 3 convertible CLOSED threads + 1 LIVE thread carrying residue refuses at exit 1 with a two-class per-line report (`broken-reference: 1, unknown-status: 1`); **12 files before and 12 after, 0 canon, 0 db**; and the SAME fixture with the residue removed converts 4 threads and writes 15 files. **That control is what makes the atomicity arm non-vacuous.** Note `covers AC-01.1 (see the correction)` now parses fine -- `959b0190` fixed it -- so it is no longer a source of residue.

## Watch-outs -- the mechanisms, distilled

### A HARNESS THAT APPLIES THE TRANSFORMATION IT IS TESTING FOR CANNOT FAIL

**My own, 2026-08-18, and it is the sharpest form of the family below.** I checked the issue-body carry by comparing `canon.body` against `source.strip()` -- I trimmed the source, then declared the trimmed source and the trimmed field identical, and reported **40 of 40 byte-exact**. vc stripped only the frontmatter, hashed the raw remainder, and got **40 of 40 off by one byte**. Every mismatch was `\ No newline at end of file`.

**The comparison had no failing case in it.** Not a bug in the check -- the check was structurally incapable of returning no. It generalises past this: vc's schema-6 pinned binary, ic's reverted treatment, and vc's already-converged control were all subjects PRE-ADJUSTED to agree with the question.

### A CONVERGING WRITER WRITES NOTHING WHEN THERE IS NOTHING LEFT TO CONVERGE

vc measured `dispatch_ssot` dirtying the estate; I ran the same suite and got a CLEAN sentinel; both were correct. **vc restored the estate before every target, so the writer had work; mine ran against an estate already converged.** My clean run was never evidence against the finding -- it was evidence the subject was already in the state their run kept undoing. **Two measurements that look contradictory, distinguished by the STATE OF THE SUBJECT rather than by the method.**

### A GREEN SUITE THAT MUTATES THE USER'S DATA

`cargo test` rewrote Intent's own estate on every run and reported **598 passed / 0 failed / exit 0** doing it -- a test binary's cwd is the crate root, and `intent` walks UP to find a project. Fixed at `1ff7f2c1` (not mine) with a shared helper rather than a third correct call site. **The guard against exactly this already existed IN THE SAME FILE, using a tempdir, three lines from sites that did not** -- a guard stated against one mechanism does not bind a different mechanism with the same effect, and a reader who finds the guard stops looking.

**And I nearly published its opposite.** I ran the reproducer with a sentinel, got `files moved: 0`, and the suite had not compiled -- **6 errors, 0 legs, a perfect zero from a run that never happened.** ic's message is the only reason I checked.

### A HARNESS IS AN INSTRUMENT AND GETS THE SAME TREATMENT AS ANY OTHER

**I broke my own six times on 2026-08-18, all one family: the harness altered or hid the measurement, and four of the six wore the finding's face.** zsh does NOT word-split an unquoted parameter, so every multi-word verb came back `unrecognized subcommand` and I was one send from reporting **v3's entire command surface missing on hoist day**. `2>/dev/null` on a jq that was erroring on all 56 files -- **I muzzled the instrument and read its silence as "no findings"**. `| tail` twice, reporting the tail of a run as the run. An exit code read THROUGH a pipe -- **that is `tail`'s code, not the runner's**, same class as the `&&` that hid clippy. And a private `CARGO_TARGET_DIR` that moved the binary out of the tree, so install-resolution failures looked like the code's.

**Never suppress a harness's stderr. Never truncate its output before reading it. Never read an exit code through a pipe. And when a result would be BIG NEWS, test the harness before sending the news** -- two of the six were caught by that and by nothing else.

### THE REHEARSAL POPULATION CANNOT HOLD WHAT THE REAL POPULATION HAS

**Three of us hit this within two hours on hoist day, by three different mechanisms.** vc's clone could not hold `intent/.cache/intent.db` (gitignored, and it BLOCKED the migration). Mine could not hold dc's fix (uncommitted). dc's subject was **mutated by a peer between two readings**, so they told vc a live finding was an instrument artefact.

**dc's sentence is the keeper: "clone and test" READS AS THE CAREFUL OPTION.** It is the move you reach for when being responsible, which is exactly when you stop asking whether the copy carries the property under test. **The remedy is not a better rehearsal -- it is knowing which inputs your rehearsal is structurally incapable of holding, and going to look at those directly.** One minute of that turned "the database was the first of these" into "the database is the only one".

**And it bit me twice more the same morning on my own perf fix**: findings 5 -> 3 and files 1408 -> 1215 were vc's staged deletions, not my change; then ST0010's `info.md` went 2466 -> 2467 between two readings (vc's `1af21f4e` regenerating views). **Fix: run both binaries BACK TO BACK on one tree, both printing their counts, so movement shows in both.**

### AN INSTRUMENT THAT NAMES ITS OWN BLIND SPOT -- the counter-pattern, and it is one line

`conservation_check.sh:793` prints _DECLARED-DROP not measured -- no `--dispositions` given, so every removal above is counted as loss whether or not the migrator named it._ **That line is the only reason I did not send vc a loss figure of 112** -- all 112 were ratified template drops whose canon hash is `e3b0c442...`, the SHA-256 of the empty string. **Every other defect we found that week was an instrument whose output could not tell you what it was FAILING to measure. This is the inverse and it costs one line.** It is also the design for the migrator's residue report (TODO 2).

### A FIGURE WITH ONE DERIVATION IS A REPORT; TWO ONLY FALSIFY IF THEY SHARE A SUBJECT

**And a CONSTANT corroborates nothing.** STRANDED 192 on vc's estate and 192 on mine are not two estates agreeing -- they are **the same files** (54 `tasks.md` + 54 `impl.md` + 54 `design.md` + 30 one-offs), so 192 is a property of the estate's STRUCTURE and a migrator that did nothing at all returns it. vc adopted this over their own "unestablished", which wrongly implies it could be established.

**A correction is only as good as the measurement it cites.** I wrote _only four hand runs_; vc corrected it citing ic's gate; ic then found their rig was signalling the wrong process and withdrew the evidence. Three links, one retraction.

### AN INVENTED CAUSE THAT RECONCILES IS THE HARDEST KIND TO DOUBT

`ef1e2474` was a fabricated reconciliation that closed perfectly. The near-miss the day after: `9f768a80` added one bats file, its CI run was CANCELLED at exactly 6h0m25s (GitHub's job ceiling), and 29 reds followed. **Perfect correlation, sitting exactly on the boundary.** I read the file before sending it as a cause -- it runs no cargo and cannot hang -- and dc then killed it outright: `grep -c devbin_rust_gates` over 391KB of failure log returns **0**. The real cause was `58f9fdb5`, a bare `git init` taking its branch name from `~/.gitconfig`.

### A CLAIM NOTHING COMPUTES, IN A LINE EVERY USER READS

`upgrade`'s _"their content is unchanged"_. `gh release create`'s _"Both binaries are Developer ID signed and notarised"_ while `publish` measured neither. A test asserting `templates not found` from a fixture that broke version resolution too -- **a fixture that breaks two things tests neither.** And a test that BORROWED its precondition from this repository being unmigrated, which the hoist falsified (`cf3ca82e`).

### GIT, PUSHES AND THE FOUR-NODE CLONE

**The push exit code carries no information in either direction** -- `git ls-remote` plus `merge-base --is-ancestor` is the only witness. **`git commit --amend` rewrites whoever committed LAST.** Always `git commit --only <paths>`; a bare commit sweeps a peer's staged index. **`cd` persists between Bash calls but resets after a subshell -- it drifted SEVEN times in one session; put the `cd` in the same command as the work.** `git archive` run from a subdirectory silently archives only that subdirectory.

### PROSE THAT IS A BUILD INPUT

**A `///` doc comment is SHIPPED OUTPUT (D37)** -- schemars lifts type and variant docs into JSON Schema, async-graphql into SDL. Plain `//` for reasoning and provenance. Intent's own ST/WP/AC ids never reach output.

## Lane boundary

`dc` owns dev-x, build, CI, release, git workflow, install. **cc is services and app functionality**: intentsvcs, the facade, the model, ingest/views/store, **and the CLI's behaviour, including wiring dispatch rows to the facade.** `surface/dispatch-table.json` is ic's; `acceptance.md` + `design.md` + `data-model.md` are vc's. **`bin/intent*` is cc's and FROZEN -- AND THAT DOES NOT GENERALISE: `lib/templates/` is NOT frozen** (dc refuted my claim with three commits, two of them the same day). Frozen and default-defer are different walls.

## Standing rulings

- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything we find building v3 is work: fix it inline, put the reasoning in the commit against the diff, message the owner if it crosses a lane.
- **The moratorium is SPENT by its own terms** -- it ran until the hoist landed, and it has. vc put that to hv rather than declaring it, on ic's point that a general directive read as answering a specific question is how a decision nobody made gets cited by two sources.
- **An AT for a row already at `to-write` was never covered by it** (vc's ruling): it discharges an obligation rather than extending one. Conditions: the row's cited file path and covered AC stay exactly as written.
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING, not absent. `fileindex` is NOT covered.
- **`EdgeKind::Incidental` STAYS despite having no user.** **`owner_wp` stays carried and unread.** **`doctor --fix` is WITHDRAWN, not deferred.** **`Outcome` is deliberately NOT `#[must_use]`** -- read it with `.already()`.
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.**
- **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`** -- it currently symlinks `bin/intent`, which now refuses this tree by name. **That symlink is the last mechanical step of self-hosting.**
