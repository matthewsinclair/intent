# Migration spec - ST0056: v2 -> v3 (WP-01 spec)

The migrator is the v3 binary's `intent upgrade` detecting a v2 project. Its parser IS the frozen legacy md parser; its discipline is the `at lint --fix` lesson with the 87-destroyed-links scar: refuse what cannot convert without loss, name everything, guess nothing.

## Preconditions (refused by name, not worked around)

1. **Floor: `intent_version >= 2.19.0`** in config.json. Below the floor: refuse, print the two-hop instruction (`install intent@2 && intent upgrade`, then retry). The v2 ledger is never reimplemented in Rust (D09).
2. **Clean git tree.** The migration is one visible commit; it refuses to start over dirt (the `bin/release` lesson: a half-done abort over a dirty tree is worse than an early refusal).
3. **A git repository.** No-git projects are refused with the reason (rollback is git; migrating without an undo is a lossy operation by construction).

## The flow

**Phase A -- check (read-only).** Strict-parse the entire estate with the legacy parser. Produce the residue report. If ANY residue: **BLOCKED**, print the work list, exit non-zero, write nothing. Migration is atomic per project -- there is no partially-converted tree, no `--defer`, no mixed v2/v3 state. Residue is fixed under v2 tooling (the two-hop's other purpose: the last v2 release is the fixing environment), then re-run.

**Phase B -- convert (only from a clean Phase A).**

1. Emit structured canon: `thread.json` per ST (metadata + WPs + full acceptance contract), `issues/<n>.json` + `issues/<n>.md` body splits.
2. Regenerate all views (info.md covers, acceptance.md contracts, steel_threads.md, todo.md) from the model.
3. Prose carried verbatim into its authored homes -- byte-conserved, never reflowed.
4. Stamp config: `intent_version: 3.0.0` + `project_id` UUID (D15).
5. Build the DB from the emitted canon (first ingest).
6. Converge canon content (skills, AGENTS.md, gitignore gains `intent/.cache/`).
7. One commit, standard message naming the tool version and the artefact counts.

**Hooks continuity invariant (0016):** `.claude/settings.json` and `.claude/scripts/**` are byte-untouched by migration. `intent claude hook <name>` resolves to the v3 binary by PATH; consumer sessions must not notice the swap. Asserted by the harness, not assumed.

**Rollback:** `git revert <migration-commit>` + reinstall the v2 formula. Cheap because the DB is disposable (`rm intent/.cache/intent.db` loses nothing) and the commit is single and named.

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
2. **Semantic completeness**: every AC, AT, status, date, WP, coverage link and reference in the v2 estate lands in the model or is named. (The Lamplight fixture: 1639 AT rows at `15dbccc92`, plus cc's post-sweep revision once the sweeps land -- the fixture is the POST-sweep tree; the pre-sweep baseline stays as the sweep's own measure.)
3. **Prose conservation**: authored bodies land byte-identical in their v3 homes.

Corpus order: **Intent's own tree first (canary), then Lamplight, Utilz, Baize** at named post-sweep revisions. The canary run includes exercising rollback for real (AC-10.6): migrate, revert, assert tree-identical.

## What the migrator does not do

- Reformat, reflow or "improve" prose.
- Invent missing data (an absent date stays absent; an empty objective stays empty and keeps its 0010 warning).
- Migrate the whiteboard (D14 -- boards and inboxes pass through untouched).
- Touch anything outside `intent/**`, `.claude/**` (untouched but verified), `AGENTS.md`, `.gitignore`.
