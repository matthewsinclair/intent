# Migration spec - ST0056: v2 -> v3 (WP-01 spec)

The migrator is the v3 binary's `intent upgrade` detecting a v2 project. Its parser IS the frozen legacy md parser; its discipline is the `at lint --fix` lesson with the 87-destroyed-links scar: refuse what cannot convert without loss, name everything, guess nothing.

## Preconditions (refused by name, not worked around)

1. **Floor: `intent_version >= 2.19.0`** in config.json. Below the floor: refuse, print the two-hop instruction (`install intent@2 && intent upgrade`, then retry). The v2 ledger is never reimplemented in Rust (D09).
2. **Clean git tree.** The migration is one visible commit; it refuses to start over dirt (the `bin/release` lesson: a half-done abort over a dirty tree is worse than an early refusal).
3. **A git repository.** No-git projects are refused with the reason (rollback is git; migrating without an undo is a lossy operation by construction).

## The flow

**Phase A -- check (read-only).** Strict-parse the entire estate with the legacy parser. Produce the residue report. If ANY residue in a LIVE thread: **BLOCKED**, print the work list, exit non-zero, write nothing. Migration is atomic per project -- there is no partially-converted tree, no `--defer`, no mixed v2/v3 state. Live-thread residue is fixed under v2 tooling (the two-hop's other purpose: the last v2 release is the fixing environment), then re-run. Legacy-grammar rows in CLOSED threads are not residue -- they convert under the closed-thread carry policy below and are counted in the report as carried, per class.

**Phase B -- convert (only from a clean Phase A).**

1. Emit structured canon: `thread.json` per ST (metadata + WPs + full acceptance contract), `issues/<n>.json` + `issues/<n>.md` body splits.
2. Regenerate all views (info.md covers, acceptance.md contracts, steel_threads.md, todo.md) from the model.
3. Prose carried verbatim into its authored homes -- byte-conserved, never reflowed.
4. Stamp config: `intent_version: 3.0.0` + `project_id` UUID (D15).
5. Build the DB from the emitted canon (first ingest).
6. Converge canon content (skills, AGENTS.md, gitignore gains `intent/.cache/`).
7. One commit, standard message naming the tool version and the artefact counts.

**Hooks continuity invariant (0016):** `.claude/settings.json` and `.claude/scripts/**` are byte-untouched by migration. `intent claude hook <name>` resolves to the v3 binary by PATH; consumer sessions must not notice the swap. Asserted by the harness, not assumed.

**Rollback:** `git revert <migration-commit>` + reinstall the v2 formula. **Cheap because the migration is ONE named commit over a v2 estate git holds whole** -- reverting restores every v2 artefact, and v2 tooling never consults the v3 store. The original reason given here was that "the DB is disposable (`rm intent/.cache/intent.db` loses nothing)", and it is VOID twice over: D01 is reversed, so the DB is durable truth, and D36 rules that operation out of the estate entirely. The v3 store is left in place on rollback; nothing deletes it.

## Residue report format

Modelled on `at lint`: one line per finding, machine-parseable, human-actionable, exit non-zero when any exist.

```
residue: <file>:<line> -- <class> -- <detail>
```

| Class              | Meaning                                                     | Fix environment           |
| ------------------ | ----------------------------------------------------------- | ------------------------- |
| unparseable-row    | an AC/AT/claims/index row the legacy grammar cannot read    | v2 `at lint --fix` / hand |
| unknown-status     | a status value outside the v2.19 vocabulary                 | v2 CLI                    |
| conflict-markers   | git conflict markers present in an artefact                 | resolve the merge         |
| unknown-file-shape | a file in a modelled location the parser cannot classify    | hand                      |
| broken-reference   | an AT file reference / descope target that does not resolve | v2 CLI / hand             |
| duplicate-id       | two artefacts claiming one natural id (the 0011 class)      | hand                      |

Every class carries the exact file:line; totals print per class; the report never truncates (the no-silent-caps rule -- a capped residue list reads as complete when it is not).

## The fleet corpus harness (`fleet_corpus_ingest.rs`)

The acceptance fixture for AC-00.2 / AC-10.5. A corpus manifest names `{project, git revision, path}` per member; the harness checks out each at its named revision (read-only), runs Phase A + a dry Phase B into a sandbox, and asserts three generalisations of the Lamplight baseline conditions:

1. **Artefact conservation**: every v2 artefact is accounted for -- converted, or named in the residue by class. Counts reconcile exactly; nothing disappears.
2. **Semantic completeness**: every AC, AT, status, date, WP, coverage link and reference in the v2 estate lands in the model or is named. (The Lamplight fixture: 1639 AT rows at `15dbccc92` -- and that estate is now known to be its PERMANENT shape, not a pre-sweep one; see the open policy question below.)
3. **Prose conservation**: authored bodies land byte-identical in their v3 homes.

Corpus order: **Intent's own tree first (canary), then Lamplight, Utilz, Baize** at named current revisions. The canary run includes exercising rollback for real (AC-10.6): migrate, revert, assert tree-identical.

## Closed-thread carry policy (hv-ruled 2026-08-14)

The forcing fact: the sweep program is dead. Lamplight's hv ruled AT remediation on Done work off outright, so their ~1158 legacy-grammar rows (70% of the estate) will never be brought to the v2.19 grammar. An unconditional BLOCKED-until-clean would meet an estate for which "fix under v2 tooling, then re-run" is refused by its owner, permanently. hv ruling:

- **CLOSED threads (Completed/Cancelled): lossless-by-carrying.** The legacy reference, `::name` citation and multi-file list are carried whole into the model -- marked legacy, nothing guessed, nothing dropped, nothing reformatted. The 0017 refusal was about a fixer that would have destroyed one end of a two-ended migration; carrying the whole row into a richer model destroys nothing, which is exactly the distinction between migrating data and improving it.
- **LIVE threads keep BLOCKED-until-clean.** Residue in a live thread is fixed under v2 tooling, then re-run.
- **Neither class ever gets a lossy path.**

**Model consequence (spec before WP-08):** carrying needs an explicit marked-legacy form on the AT row -- the raw v2 reference preserved verbatim beside the parsed fields, never reformatted. Lands in data-model.md before the migrator is built.

## What the migrator does not do

- Reformat, reflow or "improve" prose.
- Invent missing data (an absent date stays absent; an empty objective stays empty and keeps its 0010 warning).
- Migrate the whiteboard (D14 -- boards and inboxes pass through untouched).
- Touch anything outside `intent/**`, `.claude/**` (untouched but verified), `AGENTS.md`, `.gitignore`.

## The store's migration ladder starts at 1, and there is no rung below it (recorded 2026-08-15)

This bounds every schema migration anyone writes for this project, forever, so it is recorded here rather than left to be rediscovered by whoever writes the first one.

`AC-02.7` gave the runtime store a `user_version` stamp and an open path that refuses what it cannot read (`523b34e8`). **`SCHEMA_VERSION` is 1, and version 0 is not "schema zero" -- it is the ABSENCE of a version**, permanently spoken for by SQLite's `user_version` default. A store written before the stamp landed therefore records nothing about which shape it holds, and on 2026-08-15 alone the schema moved several times, so its shape is not even inferable in principle. There is no state to migrate FROM. Those stores are refused at open and cannot be recovered by any migrator we write later.

Three consequences worth stating plainly:

- **A migration ladder can only ever be `1 -> 2 -> 3`.** The older-store direction of a version mismatch is not reachable through `open` until `SCHEMA_VERSION` reaches 2; until then it is asserted against the error value directly. Do not write a `0 ->` rung; it cannot be dispatched.
- **The stamp buys the future, not the past.** This is the correct outcome under D34 -- the DB is per-machine truth and the committed extract is the interchange -- and the `SchemaUnstamped` remedy says so honestly rather than inventing a recovery command it cannot honour. But **anything never synced out of a pre-stamp store exists only there.** dc's dogfood DB is in that set.
- **Read AC-02.7 as "no store is ever silently misread", never as "no store is ever lost".** The two are different promises and only the first was bought.

## State-vocabulary migration rules (ratified 2026-08-15)

The three state machines (`data-model.md`, State machines) change the state vocabulary, so v2 data needs explicit mapping rules. **Each of these exists because the honest mapping is NOT the obvious one.**

### `TBC` maps to `NotStarted`, NEVER to `Triage`

`bin/intent_helpers:544` maps `"tbc"` **and** `"to be commenced"` to the same canonical value, `Not Started`. **In v2, TBC means To Be Commenced.** The ratified `Triage` state means something different -- created but not yet reviewed or allocated -- and it reuses the three letters, not the meaning.

So: **every v2 `TBC` migrates to `NotStarted`, and `Triage` begins with zero legacy members.** Mapping v2 `TBC` to `Triage` would invent a triage decision nobody made, for every thread that ever carried the token. This is the one migration rule most likely to be got wrong by someone matching on the string.

**Independently witnessed by ic (2026-08-15), which raises this from defensible to documented.** The rule was derived here from `intent_helpers:544` alone; ic found two more sites without having seen that derivation. `bin/intent_st:120` abbreviates `Not Started` to `TBC` **for the render column only**, and `bin/intent_st:46` -- the tool's own usage text -- spells it **"To be commenced"** in words. So `TBC` is not a v2 state at all: it is a display abbreviation of `NotStarted`, and the tool has always said so about itself.

**The surface consequence, which is a SEPARATE rule and does not follow from the migration mapping** (raised by ic, whose lane it is): **v3 must not abbreviate `Triage` as `TBC`, and must not accept `--status tbc` as `Triage`.** Either would give a familiar token a second meaning in the render column and the status filter -- **the two places a v2 user checks fastest and questions least.** A correct migration that lands beside a colliding abbreviation is still a data-integrity failure at the point of reading, because the user sees `TBC` and applies v2's meaning to it. The mapping rule governs what is STORED; this governs what is SHOWN and what is ACCEPTED, and they have to agree or the mapping's correctness is invisible.

### `satisfied: no` maps to `Unsatisfied`

The AC enum replaces `satisfied: Option<bool>` plus `AcScope`. The 13 v2 rows carrying `satisfied: no` are ordinary unsatisfied criteria and map straight to `Unsatisfied`. **`Some(false)` and `None` rendered identically in v3 and nothing ever wrote `Some(false)`** -- three stored values, two meanings -- which the single enum removes by construction. No residue.

### Threads and WPs whose status disagrees with their gate

Migration must **not** silently reconcile these. Measured 2026-08-15 in Intent's own tree, three of five WPs disagreed with their own gate -- two reporting `Done` against a BLOCKED gate, one reporting `WIP` against a PASSING one -- because `wp done` had no inverse and nothing re-checked a status after its contract changed.

**A disagreement is a finding, not a defect in the data.** The migrator reports each one by name with both values and leaves the status as authored; `doctor` reports them thereafter (hv's ruling: refuse on the way in, report afterwards). Reconciling silently would erase the evidence that the tracking data had been lying, which is the only reason anyone would look.
