# Design - ST0055: Add `intent issues` command

## Approach

`intent issues` is a thin coordinator (`bin/intent_issues`) auto-dispatched by the `*)` default case in `bin/intent` (project command; needs `PROJECT_ROOT`, no dispatcher edit). It manages issues on disk under `intent/issues/` using the **Lamplight directory-per-issue model**, with the issue template **owned by Intent** in `lib/templates/`. Five verbs: `list` (default), `add`, `show`, `close`, `open`. Implementation reuses established Intent patterns (id allocation, slug, jq `--json`, `error()`), never re-invents them (Highlander).

## Design Decisions

- **D1 -- On-disk shape: directory-per-issue (RATIFIED by hv).** Canon is `intent/issues/{OPEN,CLOSED}/NNNN/NNNN-slug.md`. The `NNNN/` directory is the issue; it can also hold attachments, `_sources/`, generated JSON/PDF, and (future) a per-issue `WP/` subtree -- the Lamplight model. `close`/`open` move the whole `NNNN/` directory between the two bucket dirs. Rejected: the flat `OPEN/NNNN-slug.md` form (Utilz) -- it cannot hold an issue's satellite files.

- **D2 -- Status model: exactly two states, OPEN and CLOSED.** The **bucket directory** (`OPEN/` vs `CLOSED/`) is authoritative; frontmatter `status:` is a mirror the verbs keep in sync. `close`/`open` are atomic (move `NNNN/` + stamp `status:`). Legacy `RESOLVED` (Lamplight drift) is normalised to CLOSED on read -- listing/show never surface a third state.

- **D3 -- Id allocation: next zero-padded 4-digit integer.** `NNNN = max(existing ids across OPEN + CLOSED) + 1`, mirroring `get_next_steel_thread_id` in `bin/intent_st` (same algorithm, no `ST` prefix). Slug from the title via the existing slug helper. Gaps tolerated (a deleted id is not reused-below-max).

- **D4 -- Template ownership: Intent owns it, single source (RATIFIED by hv).** The issue template lives at `lib/templates/issues/_ISSUE.md` (Intent's `lib/templates/` tree), NOT vendored as `_templ/0000-issue-title.md` in each project. `add` stamps it via `sed` substitution -- no inline heredoc (project rule 6). Frontmatter fields: `id`, `title`, `date` (today), `reporter` (config author), `status: OPEN`, `severity` (default `medium`, editable). Body sections carried from the ad-hoc template: Tags, Summary, Reproduction, Root Cause, Impact, Proposed Fix, Related, Resolutions.

- **D5 -- Scaffolding: lazy.** First `add` creates `intent/issues/{OPEN,CLOSED}/` (each with `.gitkeep`). `list` / `show` tolerate an absent tree (clean empty result, never an error). No per-project `_templ/` is written (D4).

- **D6 -- Thin coordinator, no silent errors.** `bin/intent_issues` parses -> dispatches -> renders; verb logic in dedicated functions. Every failure path (unknown id, bad `--kind`, missing title) surfaces via `error()` from `bin/intent_helpers` -- non-zero exit, clear message. `--json` reuses the jq field-extraction pattern from `bin/intent_todo` (one extraction shared by human and JSON renderers -- Highlander).

- **D7 -- v1 non-goals (documented futures, not built now).** No per-issue `WP/` verbs (the dir model leaves room; YAGNI for v1); no `intent todo` integration (issues have their own OPEN/CLOSED lifecycle, distinct from ST/WP status); no per-project template override; no `intent init` pre-scaffold (lazy is enough).

## Architecture

New / touched:

- `bin/intent_issues` -- new module (dispatch + five verbs). Register in `MODULES.md` FIRST.
- `lib/templates/issues/_ISSUE.md` -- new Intent-owned template (single source).
- `bin/intent_help` -- add the `issues` command entry.
- `intent/issues/{OPEN,CLOSED}/` -- lazily scaffolded in the target project (per-project data, not template).

Reused (Highlander -- do not duplicate):

- id allocation + slug: the `get_next_steel_thread_id` / slug algorithm in `bin/intent_st` (extract/share if it is not already a helper).
- `error()` / `ok()` / config author (`read_config_field`) / terminal width: `bin/intent_helpers`.
- jq `--json` field-extraction shape: `bin/intent_todo`.

## Alternatives Considered

- **Flat-file canon (Utilz shape).** Simpler, but cannot hold an issue's satellite files (attachments, sources, sub-WPs). Rejected in favour of D1 once hv confirmed the Lamplight model.
- **GitHub-issues bridge (`gh`).** Rejected: couples Intent to GitHub, breaks the self-contained / tool-agnostic ethos.
- **Fold issues into `intent todo`.** Rejected: issues have a distinct OPEN/CLOSED lifecycle and live outside any ST; keep them separate, integrate later only if wanted (D7).
