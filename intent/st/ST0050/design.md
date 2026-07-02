# Design - ST0050: intent todo: a flat DOING/TODO/DONE view of steel threads and work packages

## Approach

`intent todo` renders `intent/todo.md`: a flat, nested GitHub-Flavored-Markdown checklist of every steel thread and its work packages, bucketed **DOING / TODO / DONE** and projected from real status. It is a projection, not an independent store, so it cannot drift from `intent/st/**`.

A working **read-path prototype already exists** at `bin/intent_todo` (authored in a downstream project's Intent checkout):

- Dispatches automatically via `intent`'s default `intent_<command>` rule -- no edit to `bin/intent`. It runs as a project command, so `intent` resolves `PROJECT_ROOT` and `cd`s into it before exec.
- Reuses `intent_helpers` (`error` / `info`; `resolve_st_dir` / `normalise_st_id` are available for the mutation verbs) -- Highlander, no re-implemented frontmatter parsing.
- Enumerates the status directories directly (`intent/st/ST*`, `NOT-STARTED/`, `COMPLETED/`) and reads `status:` / `title:` / `completed:` with the repo's standard `grep -m1 "^field:" | sed` idiom.
- Atomic write (temp file + `mv`); `shellcheck`-clean bar the repo-standard SC1091 source-follow info.

`update`, `list`, and `help` work today. `done` / `notdone` / `toggle` stub with a "not wired yet" error, pending the decisions below.

Proposed work packages to make it a first-class, tested, released feature:

- **WP-01 Read path** -- `update` / `list` / `help`; checkbox mapping; the three buckets; on-hold tag; DONE self-sweep; prettier-stable output. (Prototyped; needs tests + review.)
- **WP-02 Mutation verbs** -- `done` / `notdone` / `toggle` as thin wrappers over `intent st` / `intent wp`, then regenerate; share the WP specifier parser with `intent wp`.
- **WP-03 CLI integration** -- register `todo` in `intent_help` and the top-level usage; optional explicit dispatch case for discoverability.
- **WP-04 Tests** -- the `tests/` harness: projection correctness (a fixture project with WIP / not-started / completed STs + WPs -> asserted `todo.md`), prettier-stability, mutation round-trips.
- **WP-05 Docs + release** -- README / `usage-rules.md` entry, CHANGELOG, version bump + release.

## Design Decisions

- **Projection, single source of truth.** Every line's checkbox glyph is DERIVED from the thread's / WP's real `status:` (and its status-directory placement). There is no separately-stored checkbox state, so the file cannot lie. It follows that the mutation verbs must change _real_ status by wrapping `intent st` / `intent wp` and regenerating -- never hand-editing a checkbox -- which keeps the "never hand-edit `status:`" invariant intact.

- **Checkbox mapping** (`status_box`): `WIP` -> `[-]`; `Not Started` -> `[ ]`; `Completed` / `Done` -> `[x]`; `Cancelled` -> `[~]`. Steel threads use `WIP` / `Not Started` / `Completed` / `Cancelled`; work packages use `WIP` / `Not Started` / `Done` -- both vocabularies are mapped in the one helper.

- **Buckets**: DOING = `WIP` threads (+ WPs); TODO = `Not Started` threads (+ WPs); DONE = threads completed _today_. On-hold threads (`on-hold: TRUE` + `status: WIP`) render in DOING with an `(on-hold)` tag. Ordering is ST-number ascending. A steel thread's title is its `# ST####: Title` heading (no `title:` frontmatter on ST `info.md`); a WP's title is its `title:` frontmatter.

- **The DONE bucket self-sweeps.** DONE lists only threads whose `completed:` date is today; yesterday's completions drop off automatically on the next `update` (they live permanently in `COMPLETED/` and the monthly done-log). This delivers "swept to history daily" with no marker file and no history-copy machinery.

- **Output must be prettier-stable.** `intent/todo.md` is tracked, so a pre-commit prettier hook (or format-on-save) runs on it. Plain `[-] ST...` lines are not markdown list items -- prettier reflows each section into one run-on paragraph. The generator emits canonical GFM task-list items (`- [ ]` for threads, `  - [ ]` for WPs), which prettier preserves line-for-line and which render as a real nested checklist. The generator's output is already prettier-canonical so the hook is a no-op (verified: generated output == committed blob byte-for-byte).

- **Decisions (ratified by hv, 2026-07-02):**
  - **D1 -- `notdone` reopens to `WIP`.** The unit has started (it completed), so "back in flight" == `WIP`; `Not Started` would erase its history.
  - **D2 -- `done` inherits the acceptance gate; no bypass.** `intent todo done` wraps `intent st done` / `intent wp done` and inherits the ST0048 close-gate: a BLOCKED contract refuses the close and the wrapper surfaces the gate message verbatim. A bypass would be a backdoor closing contractless units -- the vacuous-green ST0048 killed.
  - **D3 -- self-sweep, no history file.** The completed-today self-sweep is sufficient; the permanent record is `COMPLETED/` + the done-log. No separate todo history store (it would re-introduce driftable state).
  - **D4 -- confirmed.** `intent st done` stamps `completed:` (`bin/intent_st:561,590`, `date +%Y%m%d`); `todo done` wrapping it inherits the stamp -- no separate stamping needed.

- **Minimal `todo.md` output (hv, 2026-07-02).** Strip the generated boilerplate: no `# Intent Todo` title, no `_Generated from..._` provenance line, no `_Legend..._` line. Emit ONLY the three bucket headings (`## DOING` / `## TODO` / `## DONE`) and their data (the `_(none)_` sentinel when a bucket is empty). The file is a scannable projection, not a document -- the boilerplate is noise. (The completed-today self-sweep semantic is unchanged; it is simply not restated in the DONE heading.)

- **`--json` export (hv, 2026-07-02).** `intent todo --json` emits the DOING/TODO/DONE structure as machine-readable JSON (each steel thread + its work packages, carrying id / title / status / bucket) for export to other systems. The markdown and JSON renderers share one enumeration of `intent/st/**` (Highlander); `--json` selects the emitter. Applies to `todo` / `todo list`; `update` writes the markdown file as today.

## Architecture

**Dispatch.** `intent todo <args>` falls through `bin/intent`'s default case to `bin/intent_todo`, which is a project command (requires `PROJECT_ROOT`; `intent` `cd`s into the project root before exec). No dispatcher change is required; WP-03 only adds help/usage registration for discoverability.

**Command surface:**

| Command                           | Behaviour                                       | State      |
| --------------------------------- | ----------------------------------------------- | ---------- |
| `intent todo`, `intent todo list` | Print `intent/todo.md` (generate it if absent)  | prototyped |
| `intent todo update`              | Regenerate `intent/todo.md` from current status | prototyped |
| `intent todo done <STID[/NN]>`    | Mark a steel thread or WP done, then regenerate | to build   |
| `intent todo notdone <STID[/NN]>` | Reopen a steel thread or WP, then regenerate    | to build   |
| `intent todo toggle <STID[/NN]>`  | Flip done / not-done, then regenerate           | to build   |
| `intent todo help`                | Usage                                           | prototyped |

Specifier syntax reuses `intent wp`'s: `ST0011` or bare `11`; `ST0011/01` or `11/01` for a work package.

**Rendering pipeline** (`generate`): enumerate each status directory in ST-number order -> per thread, read status + title + on-hold and emit a `- [box] STID: title` line -> per WP, emit a `  - [box] NN: title` line -> assemble under the three bucket headings with a generated-header + legend -> write atomically to `intent/todo.md`. `list` prints the file (generating on first use); the mutation verbs shell out to `intent st` / `intent wp` then call `generate`.

## Alternatives Considered

- **`todo.md` as a hand-flippable checklist independent of ST status.** Rejected. It allows casual toggling but re-introduces exactly the drift the command exists to eliminate -- the file would silently diverge from `intent/st/**`. The projection model keeps one source of truth and pays for it only by routing mutations through `intent st` / `intent wp`.

- **A separate history file + sweep marker for DONE.** Deferred (decision D3). The completed-today self-sweep achieves the same "recent-only, swept daily" effect with no extra state, relying on `COMPLETED/` and the monthly done-log as the permanent record.

- **A dedicated dispatch case + new library module.** Unnecessary. The default `intent_<command>` fall-through already routes `intent todo`, and the shared `intent_helpers` already provide status-dir resolution and ID normalisation, so the command is a single self-contained script in the established `bin/intent_*` style.
