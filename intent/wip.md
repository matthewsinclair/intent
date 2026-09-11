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

### P3 -- commands that report success or state while wrong.

| #   | id  | sev | lane | defect |
| --- | --- | --- | ---- | ------ |

### P4 -- advertised but not built.

| #   | id     | sev    | lane | defect                                                                                                                                                                                     |
| --- | ------ | ------ | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 47  | `0154` | high   | ic   | No door to edit a WP body after creation (issue bodies and titles now have one).                                                                                                           |
| 48  | `0185` | medium | ic   | No verb writes a thread's title, objective, context or body.                                                                                                                               |
| 50  | `0139` | medium | ic   | `at lint --fix` is advertised and refuses.                                                                                                                                                 |
| 52  | `0177` | medium | --   | NOT WORKABLE IN 3.0.1 (vc, 2026-09-11): all of `ext` ships declared-and-unbuilt (hv, 2026-08-31), so no `ext new` ships without `ext remove`. Stays open as the constraint on ext's build. |
| 54  | `0140` | medium | ic   | An unsatisfied note is writable only by migration.                                                                                                                                         |

### P5 -- rough edges: defaults, doctor, internals.

| #   | id     | sev    | lane  | defect                                                                                                                                                                                                                                                                 |
| --- | ------ | ------ | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 61  | `0220` | medium | --    | HELD ON hv, decision 13: all three instances are fixed at the source, and the class ruling (per-project override or not) is yours.                                                                                                                                     |
| 64  | `0100` | high   | cc    | An unmappable WP status is defaulted silently. HELD ON hv, decision 2: shape ruled, needs store rung 17 -> 18.                                                                                                                                                         |
| 65  | `0084` | medium | cc    | A retired refusal left two survivors in ingest. HELD ON hv, decision 11: needs a byte write for opaque attachments.                                                                                                                                                    |
| 66  | `0066` | medium | ic    | `_inbox/` is invisible to st show/list, ac gate, at lint, todo.                                                                                                                                                                                                        |
| 68  | `0065` | medium | --    | HELD ON hv, decision 14: where an acknowledgement lives is reserved for you; dc's S design is recommended.                                                                                                                                                             |
| 70  | `0283` | medium | dc+cc | `doctor` and `organize` never consider the store being newer. RULED A+B (vc, 2026-09-11): dc makes both messages true; cc makes the projection refresh an undeclared on-disk view that is exactly the pre-change render. Closes when both land.                        |
| 71  | `0259` | high   | dc    | HALF 1 FIXED at d984b077 (vc re-drove it; a scoped restore no longer refuses on, or overwrites, another thread's view). HALF 2 OPEN: `doctor` reports a green computed from a store the same run flags stale.                                                          |
| 72  | `0145` | medium | ic    | `st edit` writes on its refusal, and the remedy names an empty list.                                                                                                                                                                                                   |
| 73  | `0146` | medium | ic    | `at edit` is kind-blind and its remedy produces a row lint cannot judge.                                                                                                                                                                                               |
| 74  | `0153` | medium | ic    | `intent edit` refuses the address form its own remedy recommends.                                                                                                                                                                                                      |
| 75  | `0150` | medium | dc    | `skills list` cannot see an orphaned skill directory.                                                                                                                                                                                                                  |
| 77  | `0176` | high   | ic    | `todo notdone`/`toggle` mutate around the Facade.                                                                                                                                                                                                                      |
| 79  | `0141` | medium | --    | NOT WORKABLE IN 3.0.1 (vc, 2026-09-11): no instance today, because both wildcard fields (AT status, WP scope) carry no payload and both self-loops are needed by rulings. The only fix is a new guard. Stays open as the constraint on either field gaining a payload. |
| 80  | `0114` | medium | --    | HELD ON hv, decision 12: cap a thread's inline attachment total, or close as by-design.                                                                                                                                                                                |
| 81  | `0172` | medium | dc    | intentd holds no persisted project registry.                                                                                                                                                                                                                           |
| 84  | `0231` | medium | ic    | TUI repaints can tear on real terminals.                                                                                                                                                                                                                               |

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

Not work items. Each is one word. 1-10 are from the cull; 11-14 were added on 2026-09-11.

1. **Push.** `git rev-list --count @{u}..HEAD`.
2. **The 13 -> 17 store migration is irreversible and nothing shipped says so.** Recommend a CHANGELOG line plus a backup sentence in the migration docs. **ADDED 2026-09-11 (vc): does 3.0.1 take a 17 -> 18 rung?** `0100` (your option 3, the `status_legacy` mirror) needs one column on `wps`, the first schema change since 3.0.0 shipped, so every 3.0.0 store takes it. vc ruled the shape (carry the v2 spelling, `status` unchanged) and held the build on your word. Recommend yes, under the same CHANGELOG and backup line.
3. **Three doors the canon mandates answer "not implemented"** (`st bootstrap`, `agents template`, `claude prime`; ST0058 AC-00.3). Recommend strike from the dispatch table and the templates.
4. **The 16 `collapsible_if` lints in intentsvcs.** Say go; until they land CI stops before `test`.
5. **The one red test, `mutation_completeness`, has no criterion behind it.** Recommend delete.
6. **The v2 bats suite dies with the trunk at the cut, EXCEPT TWO v3 FILES INSIDE IT.** 186 of CI's 216 `Intent Tests` failures are v2 tests of pruned v2 doors: prune, no port. **15 more are v3 tests** (`daemon_commands.bats`, `config_undefined.bats`, both written 2026-09-01 for v3's surface) that `test_helper.bash:21` points at v2's dispatcher; they are 15/15 green against v3. They are the only bats coverage of v3's `daemon` and `config`. Keep them by pointing them at v3 when the trunk goes, or delete them knowingly. Recommend keep. (dc, driven against both binaries.)
7. **ST0064 AC-01.7: sign and notarise the menubar app** with your credentials, or drop the app.
8. **ST0057 WP-12/13.** Recommend descope to ST0069.
9. **ST0068 AC-03.1/03.2 (the Laksa site).** Recommend descope to ST0069.
10. **Where the line falls in the list above.** Recommend: P1 and P2 in 3.0.1. **(vc, 2026-09-11: both are now empty, every item closed on re-drive, so the question is how far into P3-P5 the cut reaches.)**
11. **ADDED 2026-09-11 (vc): does 3.0.1 build the byte write for opaque attachments (`0084`)?** The issue leaves the behaviour call to you. A non-UTF-8 `.md`/`.txt`/`.sh` in a thread refuses `sync --to-store` for the whole thread. cc found that lifting the refusal alone makes canon name a sidecar nothing writes, because no door writes an opaque attachment's bytes into canon. **vc drove the same hole on the migrator: a v2 estate with one Latin-1 `notes.txt` migrates at rc=0 with canon naming `notes.txt` and no sidecar, and the next `sync --to-store` refuses `broken-reference` with a remedy that says "under v2 tooling".** (a) Build it (cc, M, store write path): byte writes in `WriteSet`, the projection emits the blobs, and the proving test is a restore-then-restore round trip. It closes `0084` and the migrator hole together. (b) Hold `0084` open. Recommend (a).
12. **ADDED 2026-09-11 (vc): should a thread's inline attachment total be capped (`0114`)?** A cap needs a value, and a choice between refusing over the total or spilling to sidecars. Spilling first needs 11's writer. The issue itself calls a per-thread bound a stopgap for your blob-home decision, and no ruling exists on either. Each file already has its own cap, so a total cap would refuse content the author put in the thread's directory, and the measured growth (94 -> 292 on ST0056) was a one-time rewrite. Recommend close as by-design (cc and vc).
13. **ADDED 2026-09-11 (vc): what does a consumer do when a shipped template or hook value is wrong for them (`0220`)?** The issue asks for a ruling on the class. All three instances it names are now fixed at the source: `_AGENTS.md`'s `bats -r` (dc, cd1601e1, vc checked the template at the fix and its parent), the clock guard's BSD seconds, and `/in-finish` step 3. A per-project override mechanism would be new work. Recommend: no override mechanism; the consumer reports it and the fix ships upstream. Close `0220` on that ruling.
14. **ADDED 2026-09-11 (vc): where does a doctor acknowledgement live (`0065`)?** The issue reserves this for you in so many words. dc's minimal design (S): project config, `"doctor": { "acknowledged": { "<finding-class>": "<reason>" } }`, keyed by the finding class's existing name. An acknowledged class still runs and still prints, as `acknowledged: <class> -- <reason> (N finding(s))`, and leaves the count and the exit code; json carries `acknowledged: true` and the reason; an acknowledgement naming no real class is itself reported. The write path is a hand edit (`config` is unbuilt; `0091` now keeps a hand-written key). One test. Recommend yes, as designed.

## What ships 3.0.1

ST0056 AC-00.5, AC-00.6, AC-07.7, AC-11.1, AC-11.4, AC-12.1, AC-12.4; ST0058 AC-00.1; ST0068 AC-04.2. All satisfied BY the cut. Run `intent ac gate ST0056`.

## Out of 3.0.1

ST0060 (vault), ST0069 (post-cut), ST0070 (LLM config). `config`, `ext`, `learn` ship declared-and-unbuilt (hv, 2026-08-31). The cull of 2026-09-11 is in commits `6918a2e5` and `0b7b24a4`.
