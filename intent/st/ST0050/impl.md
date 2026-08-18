# Implementation - ST0050: intent todo: a flat DOING/TODO/DONE view of steel threads and work packages

## Implementation

`bin/intent_todo` is a single self-contained script in the `bin/intent_*` style. It carries no dedicated dispatch case and no new library module: `bin/intent`'s default `intent_<command>` fall-through routes `intent todo`, and `bin/intent_help` auto-discovers it (promoted to a curated Core entry in WP-03). Shared concerns are sourced from `bin/intent_helpers` — `resolve_st_dir`, `normalise_st_id`, and `parse_wp_specifier` (extracted here from `bin/intent_wp` for Highlander, so `1/1` ≡ `ST1/1` ≡ `ST0001/01`).

**Projection model (the invariant).** `intent/todo.md` is derived, never authored. Each line's checkbox glyph comes from the unit's real `status:` (`status_box`) and its status-directory placement; there is no separately-stored checkbox state, so the file cannot drift from `intent/st/**`. The read path (`generate`) enumerates each status directory in ST-number order and emits canonical GFM task-list items (`- [ ]` threads, `  - [ ]` WPs) so a pre-commit prettier hook is a no-op. The `--json` path (`generate_json`) shares the same field extraction and bucket→directory mapping — only the renderer differs (Highlander: one enumeration, two emitters).

**Mutation verbs.** `done` / `notdone` / `toggle` change _real_ status by shelling out to `intent st` / `intent wp` (`todo_mutate` routes by the `/NN` in the specifier), then regenerate. `notdone` wraps `st/wp start` (reopen to WIP, D1); `done` wraps `st/wp done` and therefore inherits the ST0048 acceptance close-gate for free (D2) — a BLOCKED contract aborts under `set -e` with the gate message surfaced verbatim, status unchanged.

**DONE lifecycle (WP-06).** The DONE bucket is watermarked in its heading: `## DONE:<T>`, `<T>` an ISO 8601 UTC instant. Membership is `normalize_completed(completed) >= <T>` — lexical ISO compare, chronological — with `normalize_completed` mapping a legacy `%Y%m%d` (and a bare dashed date) to that day's `T00:00:00Z` and passing a full ISO timestamp through. `<T>` is sticky: `generate` reads it back from the existing file and preserves it (first generation defaults to start-of-today UTC, reproducing the "completed today" zero-flush view); only `flush_watermark` (the shared advance behind `done --flush` and `done --prune`) moves it to now. `--prune` writes the items being flushed to stdout (pipe-clean for `>> intent/done.md`) and its advisory note to stderr. `intent st done` was upgraded to stamp `completed:` as an ISO timestamp (`bin/intent_st`, both close paths) for exact flush ordering; the `steel_threads.md` render keys off the dashed body bullet and truncates an ISO frontmatter value to its date on the fallback path.

## Code Examples

```bash
intent todo                 # print intent/todo.md (generate on first use)
intent todo update          # regenerate from real status
intent todo --json          # { doing:[...], todo:[...], done:[...], done_watermark:"<T>" }
intent todo done ST0001/02  # wraps intent wp done (close-gate applies)
intent todo done --flush    # advance <T> to now (clear the DONE view)
intent todo done --prune    # emit DONE items to stdout, then flush
```

## Technical Details

- **macOS bash 3.2 safe.** `normalize_completed` uses `case` globs (no `declare -A`, no `${v^^}`); the `< since` skip is written `if [[ ... ]]; then continue; fi`, never `[[ ... ]] && continue` (which trips `set -e` when false).
- **Files:** `bin/intent_todo` (new); `bin/intent_helpers` (`get_default_width`, `parse_wp_specifier`); `bin/intent_wp` (parse_wp_specifier pointer); `bin/intent_st` (ISO `completed:` stamp + date-part render fallback); `bin/intent_help` (curated `todo` entry); `intent/llm/MODULES.md` (registry row).
- **Tests:** `tests/unit/intent_todo.bats` — the WP-04 harness: projection, minimal output, prettier-stability, DONE watermark + membership, mutation round-trips, gate-inheritance, `--json`, flush/prune, ISO stamp, help + fall-through guards.

## Challenges & Solutions

- **Auto-daily sweep vs. sticky watermark.** The WP-01 "completed today" self-sweep and the WP-06 "last-flush watermark" pull in opposite directions. Resolved by making `<T>` authoritative and sticky, with the first-generation default = start-of-today (so the zero-flush view still reads as "today"). "Swept daily" becomes "swept on flush"; an automatic daily sweep, if wanted, is a `--flush` from a daily ritual. Flagged for matts' acceptance-verify.
- **Legacy vs. ISO `completed:`.** Existing `COMPLETED/` threads carry `%Y%m%d`; new ones get ISO. `normalize_completed` tolerates both for DONE membership, and the `steel_threads.md` fallback render truncates to the date part — no data migration required.
