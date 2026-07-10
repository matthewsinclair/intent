# Implementation - ST0055: Add `intent issues` command

## Implementation

As-built. `bin/intent_issues` is a thin coordinator (parse -> dispatch -> render), auto-dispatched by the `*)` default case in `bin/intent` (project command; no dispatcher edit needed). Five verbs plus a `new` alias for `add`.

- **Storage:** directory-per-issue `intent/issues/{OPEN,CLOSED}/NNNN/NNNN-slug.md` (D1). Bucket dir is authoritative status; frontmatter `status:` mirrors it. `close`/`open` move the whole `NNNN/` dir + rewrite `status:`.
- **Template:** Intent-owned at `lib/templates/issues/_ISSUE.md` (D4), stamped via `sed` substitution (`NNNN`, `[Title]`, `YYYY-MM-DD`, `[Reporter]`, `[Status]`, `[Severity]`) -- no heredoc.
- **Id allocation:** `issues_next_id` = max(existing across OPEN+CLOSED)+1, zero-padded, modelled on `get_next_steel_thread_id`.
- **Helpers (Highlander):** promoted `slugify` from `bin/intent_st` into `bin/intent_helpers`; added `sanitize_title` there. `bin/intent_issues` reuses `error()`, `escape_sed_replacement`, `get_terminal_width`, `render_table`, and the git-config author expression.
- **Status normalisation:** legacy `RESOLVED` never surfaces -- `bucket_status` derives OPEN/CLOSED from the directory, so listing/show/json always show two states.
- **Portability:** `status:` rewrite uses `mktemp` + `mv` (no BSD `sed -i ''`); increments are `x=$((x+1))` (no naked `((x++))`); `find -mindepth/-maxdepth` for the id scan.

### Companion chore -- pipe sanitisation

`sanitize_title` replaces `|` with `/` at the input boundary of every title-taking command. `render_table` splits rows on `IFS='|'`, so a raw pipe in a title corrupted every markdown table it landed in (`steel_threads.md`, `wp list`, `todo.md`). Applied at `bin/intent_st` (st new), `bin/intent_wp` (wp new), and `bin/intent_issues` (add). Surfaced when `intent wp new` for ST0055's own WP-02 corrupted the wp table.

## Code Examples

    $ intent issues add "Runtime gap"          # -> created: .../OPEN/0001/0001-runtime-gap.md
    0001:Runtime gap                            #    prints ID:Title
    $ intent issues add --severity high "Bug"   # severity flag; default is medium
    $ intent issues list --kind all             # OPEN + CLOSED, render_table
    $ intent issues show 1 --json               # jq object: id/title/status/severity/date/reporter/file
    $ intent issues close 1                      # OPEN/0001 -> CLOSED/0001 + status: CLOSED
    $ intent issues open 1                       # reverse

## Technical Details

- New: `bin/intent_issues`, `lib/templates/issues/_ISSUE.md`. Touched: `bin/intent_helpers` (slugify+sanitize_title), `bin/intent_st` + `bin/intent_wp` (sanitize call-site + slugify de-dup), `bin/intent_help` (Core listing + skip case), `intent/llm/MODULES.md` (registry row).
- Tests: `tests/unit/intent_issues.bats` (19), `tests/unit/title_pipe_sanitize_guard.bats` (3). No regression across st/wp/modules/global/docs/helpers/highlander suites.

## Challenges & Solutions

- **`run_intent` is not a `run` wrapper** (it execs `intent` directly). Bats assertions on `$status`/`$output` need `run run_intent ...`; the first pass asserted on empty state. Fixed by wrapping.
- **Status-organised ST paths:** `st new` files under `intent/st/NOT-STARTED/ST0001/`, not `intent/st/ST0001/`; the pipe-guard assertions were corrected to the real paths.
