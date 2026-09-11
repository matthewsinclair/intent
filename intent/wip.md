---
verblock: "2026-09-11:v1.41: vc - THE WORK LIST. hv, 2026-09-11 09:14Z: the open defects, in 3.0.1 priority order, ARE the work. No new work is added. A row leaves this table when vc has driven its fix and closed the issue; the numbers do not shift. Pre-list verbatim at intent/.history/20260911/wip-prelist-0914Z.md."
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
| 52  | `0177` | medium | --   | NOT WORKABLE IN 3.0.1 (vc, 2026-09-11): all of `ext` ships declared-and-unbuilt (hv, 2026-08-31), so no `ext new` ships without `ext remove`. Stays open as the constraint on ext's build. |
| 54  | `0140` | medium | ic   | An unsatisfied note is writable only by migration.                                                                                                                                         |

### P5 -- rough edges: defaults, doctor, internals.

| #   | id     | sev    | lane | defect                                                                                                                                                                                                                                                                 |
| --- | ------ | ------ | ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 68  | `0065` | medium | dc   | hv said go (decision 14): dc's S design as written. The acknowledgement lives in project config under `doctor.acknowledged`, and an acknowledged class drops OUT of the finding count and the exit code. After 0259 half 2. LANDED at 5493dd28; vc to drive.           |
| 79  | `0141` | medium | --   | NOT WORKABLE IN 3.0.1 (vc, 2026-09-11): no instance today, because both wildcard fields (AT status, WP scope) carry no payload and both self-loops are needed by rulings. The only fix is a new guard. Stays open as the constraint on either field gaining a payload. |
| 81  | `0172` | medium | --   | NOT WORKABLE IN 3.0.1 (vc, 2026-09-11): hv ruled it NARROW, DO NOT BUILD on 2026-08-30. The reopen condition (a user reporting missed backups while intentd ran the whole time) is unmet, and a fix needs a new per-user registry file. Stays open.                    |

## hv's decisions that unblock the cut

All fourteen are ruled (hv, 2026-09-11). Only the work they created is listed below. The closes and descopes they ordered are done and are in the register.

### Ruled, now work

- **1. Push (hv).** Push to `local` (Dropbox) now. Push to `upstream` (GitHub) at the 3.0.1 cut.
- **2. Store migration notice (hv: go). Lane dc.** One CHANGELOG line and one backup sentence in the migration docs, saying that neither 13 -> 17 nor 17 -> 18 can be undone. The 17 -> 18 rung (`0100`) is in at 9046156b. LANDED at c2ea14c1; vc to drive.
- **3. Three doors that answer "not implemented" (hv: strike). Lane dc (moved from ic 2026-09-11; ic is on 0140).** Remove `st bootstrap`, `agents template` and `claude prime` from the dispatch table and from the templates that mandate them (ST0058 AC-00.3).
- **4. The 16 `collapsible_if` lints in intentsvcs (hv: go). Lane cc.** A mechanical fix, so that CI reaches `test`. LANDED at 7d3ffe61; vc to drive.
- **5. The red test `no_service_call_can_set_an_edgeless_field` in `mutation_completeness.rs` (hv: delete). Lane cc.** LANDED at 32958364; vc to drive.
- **6. The two v3 bats files (hv: keep). Lane dc, at the cut.** When the v2 trunk goes, point `test_helper.bash` at the v3 binary. That keeps `daemon_commands.bats` and `config_undefined.bats`, the only bats coverage of `daemon` and `config`.
- **7. The menubar app SHIPS in 3.0.1 (hv), signed and notarised (ST0064 AC-01.7). Lane cc.** `int macos` signs and notarises the CLI pair already. The app pipeline built `app-build/run/test/install/verify` and declared `app-sign` and `app-notarize` as its chunk 2, but never built them. Build them by porting Lamplight's Wrighter.app flow (same Geodica ADC, team 76BQL8L47U). Then rewrite the header lines that still say Intent ships no .app bundle.
- **10. The cut (hv).** 3.0.1 ships every workable row. It cuts when every row not marked NOT WORKABLE is closed.

## What ships 3.0.1

ST0056 AC-00.5, AC-00.6, AC-07.7, AC-11.1, AC-11.4, AC-12.1, AC-12.4; ST0058 AC-00.1; ST0068 AC-04.2. All satisfied BY the cut. Run `intent ac gate ST0056`.

## Out of 3.0.1

ST0060 (vault), ST0069 (post-cut), ST0070 (LLM config). `config`, `ext`, `learn` ship declared-and-unbuilt (hv, 2026-08-31). The cull of 2026-09-11 is in commits `6918a2e5` and `0b7b24a4`.
