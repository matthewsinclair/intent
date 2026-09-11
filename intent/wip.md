---
verblock: "2026-09-11:v1.39: vc - THE WORK LIST. hv, 2026-09-11 09:14Z: the open defects, in 3.0.1 priority order, ARE the work. No new work is added. A row leaves this table when vc has driven its fix and closed the issue; the numbers do not shift. Pre-list verbatim at intent/.history/20260911/wip-prelist-0914Z.md."
intent_version: 3.0.0
---

# Work In Progress -- the v3.0.1 work list

## THE RULE (hv, 2026-09-11 09:14Z, verbatim)

> _THERE IS NO NEW WORK TO BE DONE. We are working on these items and these items ONLY._

**The work is the numbered list below and nothing else.** Work it top-down. hv cuts from the bottom when deciding what ships in 3.0.1 and what is pushed. A defect found while fixing an item is NOT added here: write it in the fixing commit message and move on. No new tests beyond the one that proves the item fixed. No new instruments, guards, criteria or threads.

**How an item moves:** claim it on your board (its id in DOING), fix it, commit with the id in the subject, and tell vc. **vc drives the fix, then closes the issue** with `intent issues close <id>`. One item in flight per node. Lanes are by area so two nodes do not edit the same files; when your lane is empty, take the next unclaimed item in order.

**Lanes:** `cc` ingest, migration and the store write path. `ic` the CLI surface: edit, st, wp, ac, at, search, TUI. `dc` docs, install, init, templates, config, daemon operations. `vc` drives every fix before it closes, keeps this list, holds hv's pen.

**The live state is the register, not this file.** `intent issues list` is what is still open; an item struck here and still open there is not done.

## The list

### P1 -- data loss or corruption. These decide whether 3.0.1 ships.

| #   | id     | sev    | lane | defect                                                                                        |
| --- | ------ | ------ | ---- | --------------------------------------------------------------------------------------------- |
| 5   | `0209` | high   | ic   | `st start` writes a smaller copy of an unhydrated thread over the authoritative path.         |
| 6   | `0082` | high   | cc   | A new attachment authored in canon never reaches disk (`sync --to-disk`).                     |
| 7   | `0276` | high   | cc   | A committed attachment whose bytes differ from canon enters canon with no warning.            |
| 8   | `0124` | high   | cc   | v2 ingest drops prose between two recognised fields; the survivor reads complete.             |
| 9   | `0126` | high   | cc   | v2 ingest can splice a row so the length delta nets to zero and reports clean.                |
| 10  | `0138` | high   | cc   | v2 ingest has two behaviours for one input shape; only the destructive one is detectable.     |
| 11  | `0129` | medium | cc   | v2 ingest rewrites an authored full stop into its own field delimiter.                        |
| 12  | `0216` | high   | cc   | A canon write reports ok, lands, and the daemon's disk ingest reverts it (under contention).  |
| 13  | `0212` | high   | cc   | A daemon ingest reverts a completed store write when the on-disk extract lags.                |
| 14  | `0206` | high   | cc   | Canon verbs are read-modify-write with no compare-and-swap; concurrent sessions lose a write. |
| 15  | `0131` | high   | cc   | Two concurrent `issues add` both report created; one silently replaces the other.             |
| 16  | `0135` | medium | cc   | Two facades can both take one child id.                                                       |
| 17  | `0226` | high   | cc   | `st new` fails on a watched project once the corpus is large (render race with the daemon).   |

### P3 -- commands that report success or state while wrong.

| #   | id     | sev    | lane | defect                                                                              |
| --- | ------ | ------ | ---- | ----------------------------------------------------------------------------------- |
| 23  | `0079` | high   | ic   | `st new` says created and writes no files.                                          |
| 24  | `0149` | high   | ic   | `intent edit` ignores the kind it was given and answers about a thread.             |
| 25  | `0291` | high   | ic   | `edit wp ST/NN --path` returns the parent thread's file.                            |
| 26  | `0103` | high   | ic   | `wp list` returns zero rows for 71 threads whose WPs are in the store.              |
| 27  | `0137` | high   | ic   | `ac list` shows a fiat-closed criterion as satisfied: no.                           |
| 29  | `0285` | high   | dc   | `--version` can name the wrong commit on a clean build.                             |
| 31  | `0194` | high   | ic   | `intent search` with a hyphenated term leaks a SQLite error.                        |
| 32  | `0268` | high   | cc   | A failed migration rung prints a raw SQLite error naming nothing; its remedy loops. |
| 34  | `0299` | medium | ic   | `at lint` says a file lacks an id it carries (the cross-thread case).               |
| 35  | `0223` | medium | ic   | A mistyped subcommand becomes the title of a real artefact at rc=0.                 |
| 36  | `0240` | medium | ic   | `edit` returns a path at rc=0 for an AC, AT or attachment that does not exist.      |
| 37  | `0078` | medium | dc   | `skills uninstall --force` says ok and leaves the skill loadable.                   |
| 38  | `0097` | medium | cc   | `ingest --from-md` prints ok and writes nothing.                                    |
| 39  | `0069` | medium | cc   | `sync` misdescribes its own scope, twice.                                           |
| 40  | `0302` | medium | dc   | A live daemon can outlive its socket and look like a clean machine.                 |
| 41  | `0235` | medium | dc   | A running intentd cannot say which build it is.                                     |
| 42  | `0195` | medium | ic   | `intent search` reports every hit at line 0.                                        |
| 43  | `0083` | low    | ic   | `st hydrate` reports exists for a file it just created.                             |

### P4 -- advertised but not built.

| #   | id     | sev    | lane | defect                                                                           |
| --- | ------ | ------ | ---- | -------------------------------------------------------------------------------- |
| 44  | `0162` | high   | dc   | `intentd --help` starts a daemon instead of printing help.                       |
| 47  | `0154` | high   | ic   | No door to edit a WP body after creation (issue bodies and titles now have one). |
| 48  | `0185` | medium | ic   | No verb writes a thread's title, objective, context or body.                     |
| 49  | `0168` | medium | ic   | `ac list` never shows criterion text, and there is no `ac show`.                 |
| 50  | `0139` | medium | ic   | `at lint --fix` is advertised and refuses.                                       |
| 51  | `0180` | medium | dc   | `agents init --template` is accepted and nothing reads it.                       |
| 52  | `0177` | medium | dc   | `ext new` exists; `ext remove` does not.                                         |
| 53  | `0143` | medium | dc   | No v3 equivalent of `claude upgrade --skip-settings`.                            |
| 54  | `0140` | medium | ic   | An unsatisfied note is writable only by migration.                               |
| 55  | `0067` | low    | dc   | `modules find` works in v2, not v3.                                              |

### P5 -- rough edges: defaults, doctor, internals.

| #   | id     | sev    | lane | defect                                                                   |
| --- | ------ | ------ | ---- | ------------------------------------------------------------------------ |
| 56  | `0091` | medium | dc   | Writing a default freezes it into config.json.                           |
| 57  | `0101` | medium | dc   | converge_gitignore omits the events.jsonl rule.                          |
| 58  | `0120` | medium | dc   | converge_gitignore omits intent/.backup/.                                |
| 60  | `0115` | medium | dc   | `claude upgrade` report mode prints the roster, not a diff.              |
| 61  | `0220` | medium | dc   | No per-project override of shipped templates and hooks.                  |
| 62  | `0224` | medium | dc   | `init` seeds an Elixir decision tree into every project.                 |
| 63  | `0080` | medium | cc   | Migrated threads carry no slug.                                          |
| 64  | `0100` | high   | cc   | An unmappable WP status is defaulted silently.                           |
| 65  | `0084` | medium | cc   | A retired refusal left two survivors in ingest.                          |
| 66  | `0066` | medium | ic   | `_inbox/` is invisible to st show/list, ac gate, at lint, todo.          |
| 67  | `0121` | medium | ic   | `st list` defaults to in-progress and does not say so.                   |
| 68  | `0065` | medium | dc   | `doctor` has no way to acknowledge a deliberately-kept state.            |
| 69  | `0256` | high   | dc   | `doctor` flags 62 deliberately retired rows as residue.                  |
| 70  | `0283` | medium | dc   | `doctor` and `organize` never consider the store being newer.            |
| 71  | `0259` | high   | cc   | A scoped sync refuses on another thread's view-skew.                     |
| 72  | `0145` | medium | ic   | `st edit` writes on its refusal, and the remedy names an empty list.     |
| 73  | `0146` | medium | ic   | `at edit` is kind-blind and its remedy produces a row lint cannot judge. |
| 74  | `0153` | medium | ic   | `intent edit` refuses the address form its own remedy recommends.        |
| 75  | `0150` | medium | dc   | `skills list` cannot see an orphaned skill directory.                    |
| 76  | `0159` | medium | cc   | A fiat close is never timestamped.                                       |
| 77  | `0176` | high   | ic   | `todo notdone`/`toggle` mutate around the Facade.                        |
| 78  | `0136` | high   | cc   | `deny_unknown_fields` does not reach `AcState::Computed`.                |
| 79  | `0141` | medium | cc   | A self-loop on a payload state enum becomes a silent overwrite.          |
| 80  | `0114` | medium | cc   | Attachments have a per-file cap and no per-thread total.                 |
| 81  | `0172` | medium | dc   | intentd holds no persisted project registry.                             |
| 82  | `0152` | medium | cc   | A contended write waits a 5s timeout nobody chose.                       |
| 83  | `0210` | high   | cc   | Concurrent canon commits on one thread deadlock under auto-ingest.       |
| 84  | `0231` | medium | ic   | TUI repaints can tear on real terminals.                                 |

### P6 -- to CLOSE on hv's word, not to work.

| #   | id     | sev    | lane | defect                                                                                                   |
| --- | ------ | ------ | ---- | -------------------------------------------------------------------------------------------------------- |
| 85  | `0163` | high   | --   | ALREADY FIXED: `daemon start` and `daemon stop` are both wired (render.rs `daemon_start`/`daemon_stop`). |
| 86  | `0072` | medium | --   | Not product: this repo's .backup/db is empty.                                                            |
| 87  | `0092` | medium | --   | Not product: process note about a withhold.                                                              |
| 88  | `0196` | high   | --   | Not product: `int build all`, our own build tooling.                                                     |
| 89  | `0294` | high   | --   | Not product: `int cli`, our own build tooling.                                                           |
| 90  | `0292` | high   | --   | Not product: this repo's commit history and a missing commit-msg hook.                                   |
| 91  | `0222` | low    | --   | Not product: a feature request (explorer columns).                                                       |
| 92  | `0068` | high   | --   | Not product: a design constraint on future work.                                                         |

## hv's decisions that unblock the cut

Unchanged from the cull, and not work items. Each is one word.

1. **Push.** `git rev-list --count @{u}..HEAD`.
2. **The 13 -> 17 store migration is irreversible and nothing shipped says so.** Recommend a CHANGELOG line plus a backup sentence in the migration docs.
3. **Three doors the canon mandates answer "not implemented"** (`st bootstrap`, `agents template`, `claude prime`; ST0058 AC-00.3). Recommend strike from the dispatch table and the templates.
4. **The 16 `collapsible_if` lints in intentsvcs.** Say go; until they land CI stops before `test`.
5. **The one red test, `mutation_completeness`, has no criterion behind it.** Recommend delete.
6. **The v2 bats suite dies with the trunk at the cut, EXCEPT TWO v3 FILES INSIDE IT.** 186 of CI's 216 `Intent Tests` failures are v2 tests of pruned v2 doors: prune, no port. **15 more are v3 tests** (`daemon_commands.bats`, `config_undefined.bats`, both written 2026-09-01 for v3's surface) that `test_helper.bash:21` points at v2's dispatcher; they are 15/15 green against v3. They are the only bats coverage of v3's `daemon` and `config`. Keep them by pointing them at v3 when the trunk goes, or delete them knowingly. Recommend keep. (dc, driven against both binaries.)
7. **ST0064 AC-01.7: sign and notarise the menubar app** with your credentials, or drop the app.
8. **ST0057 WP-12/13.** Recommend descope to ST0069.
9. **ST0068 AC-03.1/03.2 (the Laksa site).** Recommend descope to ST0069.
10. **Where the line falls in the list above.** Recommend: P1 and P2 in 3.0.1.

## What ships 3.0.1

ST0056 AC-00.5, AC-00.6, AC-07.7, AC-11.1, AC-11.4, AC-12.1, AC-12.4; ST0058 AC-00.1; ST0068 AC-04.2. All satisfied BY the cut. Run `intent ac gate ST0056`.

## Out of 3.0.1

ST0060 (vault), ST0069 (post-cut), ST0070 (LLM config). `config`, `ext`, `learn` ship declared-and-unbuilt (hv, 2026-08-31). The cull of 2026-09-11 is in commits `6918a2e5` and `0b7b24a4`.
