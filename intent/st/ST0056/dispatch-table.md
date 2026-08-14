# Command dispatch table -- Intent v3 (ST0056, AC-05.1)

> GENERATED VIEW -- the canon is `dispatch-table.json` beside this file. Regenerate with `parity/tools/gen_dispatch_table.sh`; do not hand-edit rows. Measured at `f7434f1` on 2026-08-14 by ic.

**Status:** WIP -- `st` family complete as the shape canary; 26 families outstanding

- THE command-surface source of truth for Intent v3 (AC-05.1). The clap surface, the help text, the MCP tool list and the `intent llm` agent guide all render FROM this file; nothing renders from `bin/**` and nothing describes the surface a second time.
- This is the AUTHORED artefact. `dispatch-table.md` beside it is a GENERATED view -- run `parity/tools/gen_dispatch_table.sh`, never hand-edit the view.
- `observed` records what v2 measurably does, at the revision stamped above. `target` records what v3 will do. They are separate fields because they genuinely differ, and a table that conflated them would launder a v2 defect into a v3 requirement -- which is the failure mode an output-equality parity suite cannot catch (parity.md, 'Parity properties beyond output equality').
- `target.state: pending-hv` is an honest blank, not an omission. hv ratified the `corrected` class at the 2026-08-14 bounce; WHICH v2 behaviours join it is a scope call still open. A guess recorded here would be indistinguishable from a ruling by the time WP-05 read it.

## Surface-wide invariants

Rules that hold across the whole command surface. They are stated once here rather than repeated on every entry, and WP-05 must honour them at the framework layer -- several are things clap does differently by default, so inheriting the default silently breaks parity.

| id     | invariant                                                        | v3 target   |
| ------ | ---------------------------------------------------------------- | ----------- |
| INV-01 | Voice: lowercase `ok:` / `error:` prefixes, no banners           | as-observed |
| INV-02 | Usage errors exit 1, NOT clap's default 2                        | as-observed |
| INV-03 | The project-context gate                                         | as-observed |
| INV-04 | Exit codes observed in the shipped surface are 0, 1 and 2 only   | as-observed |
| INV-05 | `error ...; usage` -- the second call is unreachable, everywhere | pending-hv  |
| INV-06 | About a fifth of v2 failure paths write to the wrong stream      | pending-hv  |
| INV-07 | `--help` reports failure on 10 of 27 commands                    | pending-hv  |
| INV-08 | Three commands accept an unknown flag silently at exit 0         | corrected   |

### INV-01 -- Voice: lowercase `ok:` / `error:` prefixes, no banners

Every failure writes `error: <message>` to STDERR. Every success line that announces an outcome writes `ok: <message>` to STDOUT. No banners, no unicode decoration.

- **v2:** bin/intent_helpers:7-11 (`error()` -- the single authority; 25 sites were moved to the lowercase voice in v2.19.0)
- **Target:** `as-observed` -- ratified: D17 -- binary voice and exit codes carry over from v2

### INV-02 -- Usage errors exit 1, NOT clap's default 2

A missing required argument, an unknown option, or an unknown subcommand exits 1.

- **v2:** bin/intent_helpers:7-11 -- `error()` is `echo >&2; exit 1`, and it is the only failure exit in the shipped surface bar `intent critic`
- **Target:** `as-observed` -- ratified: D17
- **Implementation constraint:** clap exits 2 for both `ErrorKind::MissingRequiredArgument` and `ErrorKind::UnknownArgument` by default. D17 rules the v2 code carries over, so WP-05 MUST override clap's exit code rather than inherit it. This is surface-wide -- it affects nearly every command -- and it is recorded here precisely so it is a build-time constraint rather than something discovered in test triage. Exception: `intent critic` genuinely uses 2 (see INV-04).

### INV-03 -- The project-context gate

Commands that need a project refuse outside one with exactly `error: not in an Intent project directory`, exit 1.

- **v2:** bin/intent:206; measured uniform across 14 commands
- **Target:** `as-observed`
- **Exceptions:**
  - `claude` never reaches the gate -- plugin commands exec before the project check (bin/intent:188-191)
  - `agents` runs outside a project for the same plugin-bypass reason, despite not being in GLOBAL_COMMANDS
  - `upgrade` supplies its own message

### INV-04 -- Exit codes observed in the shipped surface are 0, 1 and 2 only

0 success; 1 every failure; 2 only from `intent critic` (findings-present) and from `intent claude hook`, which propagates the hook's own code by design.

- **v2:** bin/intent_critic:89,95; measured across 108 probes
- **Target:** `as-observed`

### INV-05 -- `error ...; usage` -- the second call is unreachable, everywhere

v2 sources read as though a missing-argument error prints the usage block. It never does.

- **v2:** `error()` ends in `exit 1`, so every `error "..."; usage` pair has a dead `usage`. Seven sites in bin/intent_st alone (311, 450, 540, 603, 1048, 1105, 1620).
- **Evidence:** `intent st` bare measured at exit 1, stdout 0B, stderr 40B -- no usage block reached stdout.
- **Target:** `pending-hv`
- **Open question for hv:** v3 renders help from this table, so printing usage on a usage error is nearly free. Do we (a) reproduce v2 -- terse `error:` line only, or (b) `corrected` -- error line plus the command's usage? (b) is what the dead code shows v2 INTENDED. Either way the exit code stays 1 per INV-02.

### INV-06 -- About a fifth of v2 failure paths write to the wrong stream

Across 108 probes, failing invocations split 45 stderr-only, 12 stdout-only, 2 both.

- **v2:** measured; larger than the three known `Error:`-on-stdout sites in the plugin bins already queued for hv
- **Target:** `pending-hv`
- **Open question for hv:** Ratify the whole census into `corrected` (errors are stderr, always), or enumerate site by site? An error on stdout interleaves with captured command output, which is how a voice becomes data.

### INV-07 -- `--help` reports failure on 10 of 27 commands

Two shapes: usage to STDOUT with exit 1 (`init`, `st`, `wp`, `todo`, `treeindex`, `fileindex`), and an error to STDERR (`ac`, `at`, `help`, `claude`). `intent at --help` is parsed as an unknown verb; `intent help --help` fails outright.

- **v2:** measured
- **Target:** `pending-hv`
- **Open question for hv:** Asking for help and being told you failed is a defect. Candidate `corrected`: `--help` always succeeds, exit 0, to stdout. Note this is NOT free of consequence -- scripts testing `intent st --help; echo $?` change answer.

### INV-08 -- Three commands accept an unknown flag silently at exit 0

`intent info --zzz`, `intent config --zzz` (zero output) and `intent version --zzz` all succeed.

- **v2:** measured
- **Target:** `corrected` -- ratified: hv, 2026-08-14 bounce -- the `corrected` class; this member is forced rather than chosen -- behaviour: Unknown arguments are refused, exit 1 per INV-02.
- **Note:** This one cannot be reproduced by accident: clap rejects unrecognised arguments by default, so v3 diverges here on day one whether or not anyone decides to. Recorded as ratified because the ALTERNATIVE (faithfully reproducing silent acceptance) would take deliberate work to build.

## Family: `st`

Manage steel threads for the project

- **v2 source:** `bin/intent_st`
- **v2 help file:** none
- **Owning work package:** WP-04

- `intent help st` falls through to the 'no help available' path (bin/intent_help:37) -- there is no `lib/help/st.help.md`. The usage() block at bin/intent_st:13-88 is the only authored help, and it is unreachable from `intent help`.
- The one-line help strings below are lifted verbatim from that usage() block where it has one, so v3's generated help stays recognisable to existing users. Where v2 has no line (`zero`), the help is newly authored and marked as such.

| command                             | args         | flags                                      | help                                                                                                    | disposition |
| ----------------------------------- | ------------ | ------------------------------------------ | ------------------------------------------------------------------------------------------------------- | ----------- |
| `st`                                | <command>    | help/--help/-h                             | Manage steel threads for the project                                                                    | keep        |
| `st new`                            | <title>      | -s/--start                                 | Create a new steel thread                                                                               | keep        |
| `st start`                          | <id>         | --                                         | Mark a steel thread as in progress                                                                      | keep        |
| `st done`                           | <id>         | --                                         | Mark a steel thread as complete                                                                         | keep        |
| `st cancel`                         | <id>         | --                                         | Mark a steel thread as cancelled                                                                        | keep        |
| `st list`                           | --           | --status <status>, --width <n>, --markdown | List steel threads (default: in progress only)                                                          | keep        |
| `st show`                           | <id> [file]  | --                                         | Show details of a specific steel thread                                                                 | keep        |
| `st edit`                           | <id> [file]  | --                                         | Print the absolute path to a steel thread file                                                          | keep        |
| `st sync`                           | --           | --write, --width <n>                       | Synchronize steel_threads.md with individual ST files                                                   | keep        |
| `st repair`                         | [id]         | --write                                    | Repair malformed steel thread metadata                                                                  | keep        |
| `st organize` (alias `st organise`) | --           | --write                                    | Organize ST files in directories by status                                                              | retire      |
| `st zero`                           | [subcommand] | --                                         | STZero retrofit -- install the zeroth steel thread (newly authored; v2 has no usage line for this verb) | pending     |

### `st`

Manage steel threads for the project

- **v2:** bin/intent_st:1614-1616, 1618-1621
- **Arguments:**
  - `command` (subcommand, arity `1`)
- **Flags:**
  - `help`, `--help`, `-h` (bool) -- Print the usage block
    - `help` is a bare word arm, not a flag; grouped here because the three spellings share one arm (bin/intent_st:1614)
- **Exit codes:**
  - `1` -- bare -- `error: Steel thread command is required`
  - `1` -- --help / -h / help -- usage block printed, exit 1
  - `1` -- unknown verb -- `error: Unknown command: <verb>`
- **stdout:** usage block (2330B) on the --help path only
- **stderr:** `error: ...` on the bare and unknown-verb paths (40B / 41B)
- **Defects observed in v2:**
  - INV-05: bare invocation reaches `error` then a dead `usage`
  - INV-07: --help exits 1
- **Target:** `pending-hv`
- **Open question for hv:** INV-07 -- see the invariant; the bare-invocation shape follows INV-05

### `st new`

Create a new steel thread

- **v2:** bin/intent_st:296-445
- **Arguments:**
  - `title` (string, arity `1`)
    - v2 collects ALL non-flag args into ARGS and uses only ARGS[0] (bin/intent_st:305, 314) -- surplus positionals are silently discarded. v3 should refuse them (INV-02); flagged, not assumed.
- **Flags:**
  - `-s`, `--start` (bool) -- Mark the new thread in progress immediately
- **Exit codes:**
  - `0` -- created
  - `1` -- no title -- `error: Steel thread title is required`
  - `1` -- unknown option -- `error: Unknown option: <opt>`
  - `1` -- templates missing -- names INTENT_HOME and states that nothing was created (bin/intent_st:375)
- **stdout:** the created id and path
- **stderr:** `error: ...`
- **Side effects:**
  - Writes `intent/st/<ID>/` from `lib/templates/prj/st/ST####`
  - Triggers the index resync (`intent st sync --write`, bin/intent_st:287)
- **Defects observed in v2:**
  - surplus positional args discarded silently
- **Target:** `as-observed`

### `st start`

Mark a steel thread as in progress

- **v2:** bin/intent_st:599-716
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Exit codes:**
  - `0` -- started
  - `0` -- ALREADY in progress -- prints `skipped: <ID> already in progress` and exits 0 (bin/intent_st:634-637, 678-681)
  - `1` -- no id / thread not found
- **stdout:** `skipped: ...` on the idempotent path
- **stderr:** `error: ...`
- **Observed notes:** `skipped:` is a THIRD voice prefix alongside `ok:` and `error:`; INV-01 does not currently name it. Carried into v3 or folded into `ok:` is a decision, flagged.
- **Target:** `pending-hv`
- **Open question for hv:** Does `skipped:` survive as a first-class prefix, or become `ok: <ID> already in progress`? Scripts matching on it exist in the BATS estate.

### `st done`

Mark a steel thread as complete

- **v2:** bin/intent_st:446-535
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Exit codes:**
  - `0` -- closed
  - `1` -- no id / thread not found
  - `1` -- acceptance contract BLOCKED -- `error: cannot close <ID>: acceptance contract is BLOCKED (run 'intent ac status <ID>')` (bin/intent_st:470-471)
- **stdout:** closure confirmation
- **stderr:** `error: ...`
- **Side effects:**
  - Consults the close-gate by shelling to `bin/intent_acceptance ac gate <ID>` (ST0044/ST0048 fail-by-default)
  - Relocates the thread directory and resyncs the index
- **Target:** `as-observed`
- **Note:** The gate becomes an in-process facade call in WP-04 (AC-04.3), not a subprocess. Behaviour and message are parity-bound; the mechanism is not.

### `st cancel`

Mark a steel thread as cancelled

- **v2:** bin/intent_st:536-598
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Exit codes:**
  - `0` -- cancelled
  - `1` -- no id / thread not found
- **stdout:** cancellation confirmation
- **stderr:** `error: ...`
- **Observed notes:** No close-gate consultation -- cancel is not a close. Deliberate (feedback: use the existing Cancelled status, never invent a new one).
- **Target:** `as-observed`

### `st list`

List steel threads (default: in progress only)

- **v2:** bin/intent_st:717-1043
- **Flags:**
  - `--status` `<status>` (string) -- `all`, or a comma-separated list rendered in the order given
    - Accepts: wip|in progress -> WIP; tbc|not started -> TBC; completed|done -> COMPLETED; cancelled|canceled -> CANCELLED; hold|on hold -> HOLD (case-insensitive, normalised to caps)
  - `--width` `<n>` (integer) -- Render at a fixed column width; 0 or absent means terminal width
  - `--markdown` (bool) -- Emit canonical GFM instead of terminal rendering
    - UNDOCUMENTED in v2's usage() block; consumed by `st sync --write`. Real surface, so it is in the table.
- **Exit codes:**
  - `0` -- listed (including an empty list)
  - `1` -- unknown option -- `error: Unknown option: <opt>`
  - `1` -- `error: Steel threads directory not found`
- **stdout:** the table
- **stderr:** `error: ...`
- **Defects observed in v2:**
  - `--status` and `--width` consume the next positional blindly (shift; VALUE="$1"). A trailing `--status` with no value silently yields an empty filter rather than a usage error. Same for `--width`.
- **Target:** `as-observed`
- **Note:** clap makes the missing-value case an error for free -- that is a `corrected` consequence rather than a choice, same shape as INV-08. Byte-exact column padding is parity-bound: `tests/unit/output_width.bats:44-140` pins it.

### `st show`

Show details of a specific steel thread

- **v2:** bin/intent_st:1044-1100
- **Arguments:**
  - `id` (st-id, arity `1`)
  - `file` (enum, arity `0..1`), default `info` -- one of: `info`, `design`, `impl`, `tasks`, `acceptance`, `all`
- **Exit codes:**
  - `0` -- printed
  - `1` -- no id / thread not found
  - `1` -- `error: Unknown file type: <t>`
  - `1` -- `error: File not found: <t>.md for steel thread <ID>`
- **stdout:** file contents; `all` concatenates with `-- <file>` separators
- **stderr:** `error: ...`
- **Target:** `as-observed`
- **Note:** `info.md` and `acceptance.md` become GENERATED VIEWS in v3 (D01/D04). `show` reads the view, so its output is unchanged in kind -- but the view's bytes are v3's to define, and every BATS test asserting v2's exact info.md bytes retires under the ratified file-layout class.

### `st edit`

Print the absolute path to a steel thread file

- **v2:** bin/intent_st:1101-1144
- **Arguments:**
  - `id` (st-id, arity `1`)
  - `file` (enum, arity `0..1`), default `info` -- one of: `info`, `design`, `impl`, `tasks`, `acceptance`
    - NO `all` -- unlike `show`. Asymmetry is deliberate and correct: there is no single path for `all`.
- **Exit codes:**
  - `0` -- path printed
  - `1` -- no id / thread dir not found / unknown file type
- **stdout:** the absolute path, one line, nothing else
- **stderr:** `error: ...`
- **Observed notes:** Pure emit-path: it never launches an editor and never creates the file. The name is a historical misnomer the docs already work around (`$EDITOR "$(intent st edit ST0001 acceptance)"`). The thread DIRECTORY must exist; the file need not.
- **Target:** `as-observed`
- **Note:** Under authored-once, `edit` on a GENERATED view (info/acceptance) hands the user a path to a file their edits will lose at the next regeneration. v3 emitting the path unchanged is defensible only if the skew check (AC-03.4) catches the edit. Flagged for the WP-05 surface cut, not decided here.

### `st sync`

Synchronize steel_threads.md with individual ST files

- **v2:** bin/intent_st:1145-1211
- **Flags:**
  - `--write` (bool) -- Write the index; without it the command is a dry run
  - `--width` `<n>` (integer) -- Render at a fixed column width
- **Exit codes:**
  - `0` -- synced or dry-run reported
  - `1` -- unknown option / `error: Steel threads directory not found` / `error: Steel threads index file not found`
- **stdout:** the diff or the confirmation
- **stderr:** `error: ...`
- **Observed notes:** Called internally by every mutating verb via `"${BASH_SOURCE[0]}" sync --write > /dev/null` (bin/intent_st:287). stdout is suppressed there but stderr FLOWS deliberately -- `2>&1 >/dev/null` once hid every sync failure (issue 0019).
- **Target:** `as-observed`
- **Note:** In v3 `steel_threads.md` is a generated view and the resync is part of the transactional write path (AC-04.1), not a subprocess. The COMMAND survives for explicit regeneration; the implicit call has no v3 analogue because it cannot fall out of date.

### `st repair`

Repair malformed steel thread metadata

- **v2:** bin/intent_st:1212-1433
- **Arguments:**
  - `id` (st-id, arity `0..1`)
    - Parsed inside the FLAG loop, not positionally -- accepted spellings are `ST0001` and `0001` ONLY
- **Flags:**
  - `--write` (bool) -- Apply the repairs; without it the command is a dry run
- **Exit codes:**
  - `0` -- repaired or dry-run reported
  - `1` -- `error: Unknown option or invalid steel thread ID: <arg>`
  - `1` -- `error: Steel thread not found: <ID>`
- **stdout:** per-thread findings
- **stderr:** `error: ...`
- **Defects observed in v2:**
  - DEAD ARM at bin/intent_st:1231: `[0-9]+)` is a `case` GLOB, where `+` is a literal character -- it matches a single digit followed by a `+`, never a run of digits. So `intent st repair 5` and `intent st repair 12345` both fall through to the error arm. Verified by executing the case statement in isolation, not by reading it. The intended bare-number form is unreachable; only the 4-digit `0001` arm works.
- **Target:** `pending-hv`
- **Open question for hv:** Does v3's id parser accept a bare number of any length (what :1231 evidently INTENDED), or only the two spellings that measurably work? Reproducing the dead arm faithfully is not an option -- it is unconstructible in clap. Same class as INV-08: a defect whose fix is forced.

### `st organize`

Organize ST files in directories by status

- **v2:** bin/intent_st:1434-1609; the `organise` -> `organize` normalisation is at bin/intent_st:289-292
- **Flags:**
  - `--write` (bool) -- Move the files; without it the command is a dry run
- **Exit codes:**
  - `0` -- swept
  - `1` -- `error: <n> steel thread(s) could not be moved (see above); the rest of the sweep completed`
- **stdout:** `Already organized: <ID> in intent/st/<STATUS>` per thread (73B for one thread in a fresh project)
- **stderr:** `error: ...`; a move collision also prints mv's raw stderr
- **Observed notes:** `organise` is an alias ONLY here, one level down. `intent organise` at top level is `error: Unknown command 'organise'` -- measured both.
- **Target:** `retire` -- ratified: hv, 2026-08-14 -- organize (both faces) is planned vestigial by construction; a strictly structured model cannot hold data in the wrong spot, so the disorder it repairs cannot arise. Confirmed finally at the surface cut (WP-05/06).
- **Note:** Retiring this face also dissolves the pre-existing Highlander violation: `bin/intent_organize` and `bin/intent_st organize` are two implementations of one concern, both registered in MODULES.md, and they print different things against the same input (117B vs 73B, no shared output).

### `st zero`

STZero retrofit -- install the zeroth steel thread (newly authored; v2 has no usage line for this verb)

- **v2:** bin/intent_st:1610-1612 -- `exec "$INTENT_HOME/bin/intent_st_zero" "$@"`; the real surface is bin/intent_st_zero
- **Arguments:**
  - `subcommand` (subcommand, arity `0..1`) -- one of: `install`
- **Exit codes:**
  - `0` -- bare -- prints `Usage: intent st zero install` and exits 0
- **stdout:** the usage line
- **stderr:** --
- **Defects observed in v2:**
  - UNDOCUMENTED: absent from bin/intent_st's usage() block entirely. It was missing from parity.md's command-level table until the deep pass measured it.
  - Exits 0 on a bare invocation that printed only usage -- the opposite of INV-07's defect, and inconsistent with every other family in this file.
- **Target:** `pending-hv`
- **Open question for hv:** parity.md already flags `st_zero` as a candidate for a ratified retire if the fleet does not use it. That ruling decides this row too -- both faces (`intent st zero` and `intent st_zero`) or neither.
- **Cross-reference:** The top-level `st_zero` family covers bin/intent_st_zero; this entry is the alias face only.

## Families outstanding

Not yet authored. Named individually rather than counted, so a family that quietly never gets written is visible as a gap rather than absent from a total.

- `wp`
- `ac`
- `at`
- `issues`
- `todo`
- `info`
- `init`
- `bootstrap`
- `config`
- `doctor`
- `upgrade`
- `agents`
- `claude`
- `critic`
- `lang`
- `llm`
- `learn`
- `modules`
- `organize`
- `plugin`
- `ext`
- `treeindex`
- `fileindex`
- `help`
- `st_zero`
- `version`

## New surface (no v2 antecedent, no parity obligation)

| command  | args         | flags          | help                                                                     | owning WP | basis                                                                                    |
| -------- | ------------ | -------------- | ------------------------------------------------------------------------ | --------- | ---------------------------------------------------------------------------------------- |
| `search` | --           | --             | Full-text search across all authored prose                               | WP-06     | design.md:68 -- FTS5 across all bodies, from CLI and MCP. There is no bin/intent_search. |
| `schema` | --           | --             | Print the generated schema faces (JSON Schema, DDL, GraphQL SDL)         | WP-06     | design.md:43                                                                             |
| `export` | --           | --format <fmt> | Project the canon into another format                                    | WP-06     | design.md:57 -- YAML/md/anything else are export projections                             |
| `ingest` | --           | --from-md      | Rebuild the canon from markdown (the recovery path, and the v2 migrator) | WP-03     | design.md:66; WP-03 deliverable, shared with the WP-10 migrator                          |
| `daemon` | <subcommand> | --             | Manage the machine-level intentd                                         | WP-08     | design.md:73-74, D07/D08/D19                                                             |
| `mcp`    | --           | --             | Serve the MCP surface over stdio                                         | WP-09     | design.md:84, D11                                                                        |

- `search` -- acceptance: AC-06.4 (added by vc, 2026-08-14, on the finding that all 62 ACs had zero coverage of search)
