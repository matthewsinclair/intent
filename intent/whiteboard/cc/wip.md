---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-18 07:29Z
status: paused
focus: "PAUSED at an aggressive localfold, 07:29Z. **INTENT IS SELF-HOSTED ON INTENT3** -- vc hoisted at `0ec2ac79`; I verified it independently rather than took it. **My part: I killed my own top blocker by RE-MEASURING it instead of citing it** (v2 writes ZERO files into a v3 estate; `53f88757` had closed it and my board carried the stale reading as live), and I enumerated the fifteen gitignored artefacts a clone cannot hold, migrating one carrying fourteen -- **56/40/352, byte-identical to a plain clone -- so the stale store was the ONLY blocker rather than the first.** **Landed and published: `de9292a4` doctor 77 -> 5, `ba63b344` doctor 10.6s -> 0.68s (vc's hypothesis REFUTED -- the view-skew arm was 8.7ms of a 7.40s problem; `sync::scan` walked 613,811 paths to answer about 1,511, and the excess was UNBOUNDED and MACHINE-LOCAL), `cf3ca82e` the first hoist casualty, `b37efea7` the ignore rule that covered nothing, `43e08208` the board.** 83 legs / 598 passed / 0 failed, fmt 0, clippy 0, every exit code read WITHOUT a pipe. **NEXT IS vc's TWO BUILD ITEMS AND THEY ARE NOW A GATE, NOT A NICETY: hv has ruled disk OPTIONAL, so anything the store does not hold is destroyed by the first render.** Upstream FROZEN; push `local` only."
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

## DOING -- vc's two build items, and hv's disk ruling makes them the gate

**1. `attachments`** -- specced at `1ffe8868` in `data-model.md`; **read the spec before building.** Fields: `path` (relative to the THREAD root, so `WP/01/notes.md` needs no WP-level collection), `text` (VERBATIM -- never parsed, never normalised, never section-split), `bytes`, `sha256`.

- **DECLARED EXTENSION, not everything.** 304 files under thread dirs are none of the canonical five: 196 `.tap`, 66 `.md`, 38 `.sh`, 2 `.txt`, 2 `.tsv`. **`.md` + `.txt` to start = 68 files.** Attaching generated baselines and executable shell would need mode bits, binary payloads and a merge story -- **which is a VCS, and there is one a directory up.**
- **BUILD FIRST, because it is the part that matters: a file that does NOT attach must be NAMED by `doctor`, never silently skipped.** Otherwise the rule reproduces the defect of the week -- disk goes optional and something vanishes because no surface said it was uncovered.
- **Two constraints, not notes.** A file is a typed doc **or** an attachment, never both (the Highlander violation this introduces if you let it). And `text` is OPAQUE: **an attachment has no fields to parse INTO, and parsing into nothing is exactly how `## Related Steel Threads` became 52 rows of `LOST-PROSE`.**

**2. The issue `body` field** -- 503 `LOST-PROSE`, all `kind=issue`, my own two-derivation figure (hand count off the files, and the acceptance instrument, agreeing). vc measured on the live estate: **an issue body probe returns ZERO from the store, and `intent/issues/CLOSED/NNNN/*.md` is the only copy of all 40.** Residue under the old model; **data loss on the first render under hv's.**

**MEASURED GREEN LIGHT, so nobody has to judge when this is done: when `LOST-PROSE` and `UNACCOUNTED` hit ZERO on this estate, disk is safe to make optional.** `conservation_check.sh` already asks both questions; ic runs it.

## TODO

1. **`upgrade` prints a claim nothing computes.** _"their content is unchanged"_ while ST0010 and ST0015 each gain a blank line (ic measured; converges after one run; leading-blockquote emitter). **ic's call is right and it is the sentence first, not the blank line:** an operator reads it, then finds two modified files, and concludes something else touched them. Either compare and report the real number, or say "re-emitted" and stop.
2. **`doctor` should name a stale pre-versioning store BEFORE a cutover**, not during. And **my better version: the migrator should report what it FOUND in the same breath as its refusal** -- it says _nothing recorded which shape it holds_ without opening the database, and vc then opened it in one command and learned it held zero canonical rows. `conservation_check.sh:793` is the shape.
3. **`related: Vec::new()`** -- 52 `LOST-PROSE`, every one `## Related Steel Threads`. The other half of the prose loss and the only thread-side cause.
4. `AC-10.8`'s egest side; `AT-10.2`/`10.3`/`10.4` (probed and designed, see below); `WpStatus::Cancelled`; the 171 stranded authored files pending hv's relocation ruling.

**AT-10.2 IS PROBED AND READY, so a later session does not redo it.** A fixture with 3 convertible CLOSED threads + 1 LIVE thread carrying residue refuses at exit 1 with a two-class per-line report (`broken-reference: 1, unknown-status: 1`); **12 files before and 12 after, 0 canon, 0 db**; and the SAME fixture with the residue removed converts 4 threads and writes 15 files. **That control is what makes the atomicity arm non-vacuous.** Note `covers AC-01.1 (see the correction)` now parses fine -- `959b0190` fixed it -- so it is no longer a source of residue.

## Watch-outs -- the mechanisms, distilled

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
