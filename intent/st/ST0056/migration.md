# Migration spec - ST0056: v2 -> v3 (WP-01 spec)

The migrator is the v3 binary's `intent upgrade` detecting a v2 project. Its parser IS the frozen legacy md parser (`intentsvcs/src/legacy.rs`); its discipline is the `at lint --fix` lesson with the destroyed-links scar: refuse what cannot convert without loss, name everything, guess nothing.

## Preconditions (refused by name, not worked around)

All three are checked only while the project is still unmigrated; a re-run over a project that already declares v3 is the convergent re-run and skips them (`facade.rs`, `Facade::upgrade`). Each refusal exits 1.

1. **Floor: `intent_version >= 2.19.0`** in config.json (`MIGRATION_FLOOR`, `project.rs:904`). Below the floor the refusal names the declared version and its remedy reads _run `install intent@2 && intent upgrade` first, then migrate it with v3_. No tap provides an `intent@2` formula: v2 is installed from a checkout of the Intent repository at `v2.19.0`. The v2 ledger is never reimplemented in Rust (D09).
2. **Clean git tree.** The migrator does not commit; it refuses to start over dirt, naming each uncommitted path, so that the operator's commit of its output holds the migration and nothing else (the `bin/release` lesson: a half-done abort over a dirty tree is worse than an early refusal).
3. **A git repository.** A project with no work tree is refused with the reason (rollback is git; migrating without an undo is a lossy operation by construction).

## The flow

**Phase A -- check (read-only).** Strict-parse the entire estate with the legacy parser. Produce the residue report. If ANY residue in a LIVE thread: **BLOCKED**, print the work list, exit non-zero, write nothing. Migration is atomic per project -- there is no partially-converted tree, no `--defer`, no mixed v2/v3 state. Live-thread residue is fixed under v2 tooling (the two-hop's other purpose: the last v2 release is the fixing environment), then re-run. Legacy-grammar rows in CLOSED threads are not residue -- they convert under the closed-thread carry policy below and are counted in the report as carried. `intent ingest [PATH]` runs Phase A on its own and writes nothing; its summary is `read: <n> thread(s), <n> work package(s), <n> criteria, <n> acceptance test(s)` and `residue: <n> blocking, <n> carried`, and a clean estate closes with `ok: this estate parses -- nothing was read into a store and nothing was written`.

**Phase B -- convert (only from a clean Phase A).** `migrate::plan` builds the whole write set and writes nothing; the facade then commits the files and runs the remaining steps, and a failure at any later step rolls the files back.

1. Emit structured canon: `intent/.canon/st/<ID>.json` per thread (metadata, WPs, full acceptance contract, and every carried attachment) and `intent/.canon/issues/<NNNN>.json` per issue, with the issue body carried inside the JSON.
2. Regenerate the views of every thread `.intentfiles` realises (its `info.md`, `acceptance.md` and WP `info.md` covers), plus `steel_threads.md` and `todo.md`. When `.intentfiles` is absent the default declaration -- every WIP thread -- decides.
3. Prose carried verbatim into canon -- authored sections into the modelled fields, other files as attachments -- byte-conserved, never reflowed.
4. Build the DB from the emitted canon (first ingest), recording the canon files' bytes.
5. Converge the project files: `.gitignore` gains `intent/.cache/`, `intent/events.jsonl` and `intent/.backup/` where absent; `.prettierignore` gains the generated views; `.intentfiles` is written with the default declaration if absent.
6. Stamp config LAST: `intent_version` set to the running binary's version, and a `project_id` UUID minted only if absent (D15). Stamping last keeps a half-finished migration reporting itself as unmigrated.
7. **No commit.** The run ends `ok: this project is now Intent v<version> -- commit the canon and the generated views`, and the operator makes the one commit.

**Hooks continuity invariant (0016):** `.claude/settings.json` and `.claude/scripts/**` are byte-untouched by migration. `intent claude hook <name>` resolves to the v3 binary by PATH; consumer sessions must not notice the swap. Asserted by the harness (`intentsvcs/tests/migrate_hooks_continuity.rs`), not assumed.

**Rollback:** `git revert` the operator's migration commit, and put v2 back on PATH from a checkout at `v2.19.0`. **Cheap because that commit sits over a v2 estate git holds whole** -- reverting restores every v2 artefact, and v2 tooling never consults the v3 store. The migrator does not create the commit, so "one commit, holding only the migration" is guaranteed by the clean-tree refusal plus the operator committing the output as it stands. The v3 store is left in place on rollback; nothing deletes it.

## Residue report format

Modelled on `at lint`: one line per finding, machine-parseable, human-actionable, exit non-zero when any blocks. `residue:` leads only a blocking finding, followed by its class remedy; a carried finding prints under a once-printed `carried (converts as-is, no action):` header, leads `carried:` and carries no remedy.

```
residue: <file>[:<line>] -- <class> -- <detail>
  remedy: <what to do>
carried: <file>[:<line>] -- <class> -- <detail>
```

| Class              | Meaning                                                                          | Fix environment           |
| ------------------ | -------------------------------------------------------------------------------- | ------------------------- |
| unparseable-row    | an AC/AT/claims/index row the legacy grammar cannot read                         | v2 `at lint --fix` / hand |
| unknown-status     | a status value outside what `canonical_status` ACCEPTS                           | v2 CLI                    |
| unknown-scope      | a `scope:` value outside the T-shirt enum, v2 having read the field as free text | carried, or v2 CLI        |
| conflict-markers   | git conflict markers present in an artefact                                      | resolve the merge         |
| unknown-file-shape | a file in a modelled location the parser cannot classify                         | hand                      |
| broken-reference   | an AT file reference / descope target that does not resolve                      | v2 CLI / hand             |
| duplicate-id       | two artefacts claiming one natural id (the 0011 class)                           | hand                      |
| field-not-recorded | a field the estate never recorded, the artefact predating the convention         | **nothing -- carried**    |
| unread-field       | a keyed field the v2 line carries and this grammar never reads                   | **nothing -- carried**    |
| retired-setting    | a config knob v3 has retired, set to a value v3 will not honour                  | hand, BEFORE migrating    |

The summary counts blocking and carried findings, and a refusal totals its findings per class; the report never truncates (the no-silent-caps rule -- a capped residue list reads as complete when it is not).

**`retired-setting` is the one class here that is not about an artefact, and it is the only one that must be fixed BEFORE the migration rather than during or after it** (added by cc 2026-08-16 with the code that emits it, under hv's `st_prefix` retirement -- vc to reword freely, the row is here so the contract and the migrator land together rather than one check apart). Its instance is `st_prefix`: v3 fixes the steel-thread prefix, so a project that set it to anything else has **none** of its threads recognised. That makes it the only class whose consequence is invisible by construction -- there is no residue to report, because there is no artefact the scanner can see to report it against, and every count reconciles perfectly against zero. It blocks for that reason, and the closed/live split does not apply to it: the finding is about the project, not about a thread, and there is no thread to attribute it to precisely when it matters most.

**`field-not-recorded` and `unknown-scope` were added 2026-08-16 after this table was measured against the implementation and found short by exactly those two** -- which are the only two classes Intent's own tree, the canary, actually produces. **An operator meeting either of the classes they will actually meet found neither in the contract, and every check reported agreement.** Guarded from here by `parity/tools/residue_class_check.sh`, which compares this table against the `FindingClass` variants `legacy.rs` constructs, in both directions, rather than trusting a transcription.

**`unread-field` was added 2026-08-26, and it is the CLASS the two rows above it are instances of.** Lamplight writes disposed criteria as `-- withdrawn: <reason>` and `-- descoped-to: ST0347 -- by: hv -- on: 2026-08-21`; the AC reader knows `evidence:` and `satisfied:` and nothing else, so those rows arrived with their disposition as prose and the migration exited 0. **Teaching the reader those two keys would close those rows and leave the next convention exactly as silent** -- the argument `thread_dirs` already lost, where an allowlist of bucket names closed the instance and left the class open. So any `-- key:` the grammar does not read is named here instead. **On an AT row the scan stops at `-- status:`**, because everything after it is the note, which v2 declines to parse (`AT_G_NOTE='( -- .*)?'`) and which is ruled to have no interior structure: most of Lamplight's AT occurrences sit in that region, and reporting them would be this check inventing the grammar that ruling refuses.

**The line number is reported when the finding has one.** Some classes are about a field that is ABSENT -- `field-not-recorded` most obviously -- and an absent field has no line to cite. `unknown-scope` does have a line and does not print one (driven at v3.0.1: `carried: intent/st/COMPLETED/ST0020/WP/09/info.md -- unknown-scope -- ...`); that is a real gap in the implementation.

## What the migration did NOT carry

Three lines, and they make three different claims. **Collapsing any two of them was the defect this section exists to prevent** -- issue `0183` was filed proposing the migrator declare its whole unaccounted file set as out-of-model, and most of that set is the whiteboard, which `data-model.md` moved INTO the model at D30. Implementing it as filed would have contradicted an hv ruling and closed by fiat a gap that closes on its own.

| Line                                                        | Stream | Claim                                                                                                          |
| ----------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------- |
| `not carried into the model: ...`                           | stderr | the model does not cover these. Closes by fiat, forever.                                                       |
| `not yet carried -- the model claims these ...: ...`        | stderr | the model covers these and no build carries them yet.                                                          |
| `not yet carried, per artefact (<n>):` + `not-yet-carried:` | stdout | the same claim as the line above, one record per file, by path, with the class remedy printed once after them. |

The first two are the migration describing its own REACH and are composed from `sync::NOT_CARRIED` and `sync::NOT_YET_BUILT`, each member pinned to the phrase in `data-model.md` that authorises it -- with a test that reds when the document stops carrying the phrase, because a citation without one is a copy and a copy of a ruling goes stale the day the ruling moves.

**The third is per-artefact because this estate already held that standard and was applying it to the smaller class.** `legacy.rs` names each oversized attachment individually, by path, with its own reason, while the entire whiteboard once reached the same report as a single directory noun. A collapse is least defensible in exactly the direction it was being applied, and "the report is the point, not the carry" is the rule the attachment path already states.

**One more stdout block sits beside these: `sections not carried as-is:`.** It names, per section, what the migration decided about a section it did not simply carry -- `dropped` (template boilerplate no author wrote, verified byte-identical to the template it came from) or `deferred` (a section the thread authors under a heading the renderer also generates, so the generated copy stands down) -- one line per section as `<file> -- ## <heading> -- <verdict> -- <reason>`, never a count.

**Its class is `modelled-not-built`, and it is deliberately NOT in the residue table above.** Residue is something a v2 AUTHOR left behind, and every row of that table owes a fix environment; nothing here is anyone's mistake, no v2 command touches it, and the gap is discharged by a build rather than by an operator. It is equally not `advisory`, which is the class of what is worth doing when an artefact is next touched -- touching one of these changes nothing. `residue_class_check.sh` reads `legacy.rs`, so the class correctly stays outside that check's population, exactly as `advisory` and `gate-not-running` do.

**And the records must never reach `Scan`.** Both of its buckets are wrong in a way worse than untidy: `residue` BLOCKS, so routing these through it would refuse a migration to every estate that has a whiteboard, permanently, on the ordinary shape of a project rather than on a defect; and `carried` prints under _converts as-is, no action_, which is the one thing these files do not do. The enumeration therefore lives at the `NOT_YET_BUILT` declaration (vc's ruling, 2026-09-05, under hv's pen) and is guarded by an arm that drives an estate which HAS the files.

**A declared exclusion silences `conservation_check.sh` for the paths it names, so this could have been the denominator attack** -- a migrator zeroing a counter by naming everything. What keeps it honest is that the claim is refutable in the direction that matters: `Verdict::Dropped` is corroborated by canon being EMPTY, and this by the file being PRESENT and unchanged. Driven on Lamplight `fe5dff3e6` (2.19.0) 2026-09-05: rc 0, a `not-yet-carried` record emitted per whiteboard file, and **no whiteboard path touched, every one still on disk**.

## The fleet corpus harness

The acceptance fixture for AC-00.2 / AC-10.5, built as three shell instruments under `intent/st/ST0056/parity/tools/`. `estate_corpus.sh` captures each member's v2 estate at a named commit, never a worktree. `conservation_check.sh` takes that member's census and the tree a real `intent upgrade` produced (plus the upgrade's stdout, via `--dispositions`) and asserts three generalisations of the Lamplight baseline conditions:

1. **Artefact conservation**: every v2 artefact is accounted for -- converted, relocated, named out-of-model, or reported as residue. Reachability is the predicate, not presence: "still on disk" is not a disposition.
2. **Semantic completeness**: every AC, AT, status, date, WP, coverage link and reference in the v2 estate lands in the model or is named, by id and not only by count. (The Lamplight fixture is `15dbccc92` -- and that estate is now known to be its PERMANENT shape, not a pre-sweep one; see the carry policy below.)
3. **Prose conservation**: every authored section has a destination, and the bytes that arrive are the bytes that left.

`fleet_corpus_conservation.sh` joins the members: it reads one recorded verdict per member from `parity/tools/fleet-runs/<member>.verdict` -- `conserved`, `named`, `refused` (with its reason) or anything else as `LOST` -- and a member with no recorded run is UNRUN, never passing.

Corpus order: **Intent's own tree first (canary), then Lamplight, Utilz, Baize** at named revisions. The canary run included exercising rollback for real (AC-10.6, recorded in `impl.md`): migrate, commit, revert, assert tree-identical.

## Closed-thread carry policy (hv-ruled 2026-08-14)

The forcing fact: the sweep program is dead. Lamplight's hv ruled AT remediation on Done work off outright, so their legacy-grammar rows -- most of that estate -- will never be brought to the v2.19 grammar. An unconditional BLOCKED-until-clean would meet an estate for which "fix under v2 tooling, then re-run" is refused by its owner, permanently. hv ruling:

- **CLOSED threads (Completed/Cancelled): lossless-by-carrying.** The legacy reference, `::name` citation and multi-file list are carried whole into the model -- marked legacy, nothing guessed, nothing dropped, nothing reformatted. The 0017 refusal was about a fixer that would have destroyed one end of a two-ended migration; carrying the whole row into a richer model destroys nothing, which is exactly the distinction between migrating data and improving it.
- **LIVE threads keep BLOCKED-until-clean.** Residue in a live thread is fixed under v2 tooling, then re-run.
- **Neither class ever gets a lossy path.**

**Model consequence:** carrying needs an explicit marked-legacy form on the AT row -- the raw v2 reference preserved verbatim beside the parsed fields, never reformatted. Built as `AcceptanceTest.legacy: Option<Legacy>` (`model.rs:1715`, the `Legacy` type at `:1729`), published in `schema/thread.schema.json`.

## What the migrator does not do

- Reformat, reflow or "improve" prose.
- Invent missing data (an absent date stays absent; an empty objective stays empty and keeps its 0010 warning).
- Migrate the whiteboard -- boards and inboxes stay on disk untouched and are reported as `not-yet-carried`.
- Relocate or remove v2's bucketed thread directories (`intent/st/COMPLETED/`, `CANCELLED/`, `NOT-STARTED/`). Canon is written at `intent/.canon/`, and the v2 files stay where they were; `intent organize` reports them as unclaimed and never removes them.
- Commit. The operator commits the output.
- Touch anything outside `intent/**`, `.gitignore` and `.prettierignore`. `.claude/**` and `AGENTS.md` are untouched; converging skills and `AGENTS.md` is `intent claude upgrade --apply` and `intent agents sync`, not the migration.

## The store's migration ladder starts at 1, and there is no rung below it (recorded 2026-08-15)

This bounds every schema migration anyone writes for this project, forever, so it is recorded here rather than left to be rediscovered by whoever writes the first one.

`AC-02.7` gave the runtime store a `user_version` stamp and an open path that refuses what it cannot read (`523b34e8`). **`SCHEMA_VERSION` started at 1 (its current value is the constant in `store.rs`), and version 0 is not "schema zero" -- it is the ABSENCE of a version**, permanently spoken for by SQLite's `user_version` default. A store written before the stamp landed therefore records nothing about which shape it holds, and on 2026-08-15 alone the schema moved several times, so its shape is not even inferable in principle. There is no state to migrate FROM. Those stores are refused at open and cannot be recovered by any migrator we write later.

Three consequences worth stating plainly:

- **The migration ladder runs from 1 upward, and there is no `0 ->` rung.** `Store::open` creates a fresh database at the current version, migrates an older stamped store rung by rung (`Store::migrate`), refuses a newer one (`SchemaMismatch`, remedy: move the tool forward, never the data back), and refuses a store stamped 0 that already has tables (`SchemaUnstamped`).
- **The stamp buys the future, not the past.** This is the correct outcome under D34 -- the DB is per-machine truth and the committed extract is the interchange -- and the `SchemaUnstamped` remedy says so honestly rather than inventing a recovery command it cannot honour. But **anything never synced out of a pre-stamp store exists only there.** dc's dogfood DB is in that set.
- **Read AC-02.7 as "no store is ever silently misread", never as "no store is ever lost".** The two are different promises and only the first was bought.

## State-vocabulary migration rules (ratified 2026-08-15)

The three state machines (`data-model.md`, State machines) change the state vocabulary, so v2 data needs explicit mapping rules. **Each of these exists because the honest mapping is NOT the obvious one.**

### The source vocabulary is what v2 ACCEPTS, never the set of values it PRINTS

**Port the vocabulary from `canonical_status`, not from a census of v2's canonical outputs.** (cc, 2026-08-16, measured on this estate while building Phase A; recorded here on vc's ruling because it is a migration rule rather than a code detail.)

A census of this repository flagged one work package at `status: Complete` as out-of-vocabulary. **It is not: `complete` is in v2's synonym table and always resolved to `Completed`.** The value was well-formed input that v2 accepted and normalised, and it never appeared in any output, so a vocabulary derived from observed outputs cannot contain it.

**A migrator built on the printed set files residue against data v2 considered correct**, and that is the expensive direction: the operator is told their estate is malformed when it is not, on a thread they may not be able to change. **The accepted set is a superset of the printed set, always, and the gap between them is exactly the synonyms** -- which is where hand-authored legacy data actually lives, because synonyms exist to let humans write what they meant.

Same family as the absent-field false findings the first Phase A run produced: **both are the migrator asserting a rule the source never enforced.** The general form is worth holding beyond `status`: **for any migrated field, the legal input set is the tool's parser, not its formatter**, and reading the formatter is the easy mistake because the formatter's output is what you have lying around.

### `TBC` maps to `NotStarted`, NEVER to `Triage`

v2's `bin/intent_helpers:544` (at `v2.19.0`; the v2 CLI is no longer in the tree) mapped `"tbc"` **and** `"to be commenced"` to the same canonical value, `Not Started`. **In v2, TBC means To Be Commenced.** The ratified `Triage` state means something different -- created but not yet reviewed or allocated -- and it reuses the three letters, not the meaning.

So: **every v2 `TBC` migrates to `NotStarted`, and `Triage` begins with zero legacy members.** Mapping v2 `TBC` to `Triage` would invent a triage decision nobody made, for every thread that ever carried the token. This is the one migration rule most likely to be got wrong by someone matching on the string.

**Independently witnessed by ic (2026-08-15), which raises this from defensible to documented.** The rule was derived here from `intent_helpers:544` alone; ic found two more sites without having seen that derivation. `bin/intent_st:120` (at `v2.19.0`) abbreviated `Not Started` to `TBC` **for the render column only**, and `bin/intent_st:46` -- the tool's own usage text -- spelled it **"To be commenced"** in words. So `TBC` is not a v2 state at all: it is a display abbreviation of `NotStarted`, and the tool has always said so about itself.

**The surface consequence, which is a SEPARATE rule and does not follow from the migration mapping** (raised by ic, whose lane it is): **v3 must not abbreviate `Triage` as `TBC`, and must not accept `--status tbc` as `Triage`.** Either would give a familiar token a second meaning in the render column and the status filter -- **the two places a v2 user checks fastest and questions least.** A correct migration that lands beside a colliding abbreviation is still a data-integrity failure at the point of reading, because the user sees `TBC` and applies v2's meaning to it. The mapping rule governs what is STORED; this governs what is SHOWN and what is ACCEPTED, and they have to agree or the mapping's correctness is invisible. As built, `intent st list --status tbc` lists `Not Started` threads, and the status column spells statuses out in full.

### `satisfied: no` maps to `Unsatisfied`

The AC enum replaces `satisfied: Option<bool>` plus `AcScope`. v2 rows carrying `satisfied: no` are ordinary unsatisfied criteria and map straight to `Unsatisfied`. **`Some(false)` and `None` rendered identically in v3 and nothing ever wrote `Some(false)`** -- three stored values, two meanings -- which the single enum removes by construction. No residue.

### Threads and WPs whose status disagrees with their gate

Migration must **not** silently reconcile these. Measured 2026-08-15 in Intent's own tree, work packages disagreed with their own gate in both directions -- `Done` against a BLOCKED gate, `WIP` against a PASSING one -- because `wp done` had no inverse and nothing re-checked a status after its contract changed.

**A disagreement is a finding, not a defect in the data.** The migrator leaves the status as authored and does not report the disagreement itself; `intent doctor` reports each one by name, with both values, as `status-gate-disagreement` from the first run after the migration (hv's ruling: refuse on the way in -- `wp done` is refused on a blocked gate -- and report afterwards). Reconciling silently would erase the evidence that the tracking data had been lying, which is the only reason anyone would look.
