---
verblock: "26 Apr 2026:v0.3: matts - Done"
wp_id: WP-06
title: "Ignore patterns: gitignore and treeindexignore"
scope: Extra Small
status: Done
---

# WP-06: Ignore patterns -- `.gitignore` and `.treeindexignore`

## Objective

Update Intent's canonical ignore patterns -- `.treeindexignore` and any `.gitignore` template -- to reflect the new `intent/.config/` layout. Specifically: replace blanket `.intent/` exclusions with granular `intent/.config/cache/` + `intent/.config/backup/` patterns so treeindex still descends into `intent/.config/config.json`.

## Context

Recon findings:

- **`intent/.treeindex/.treeindexignore`** (Intent's own, line 14): contains `.intent/`. Excludes the entire directory from treeindex generation. Post-relocation, this exclusion is wrong: `intent/.config/` lives inside `intent/`, which treeindex traverses by default.
- **No `lib/templates/_treeindexignore`** template exists (recon found nothing under `lib/templates/`). Downstream projects either inherit no `.treeindexignore` or copy Intent's own.
- **No `lib/templates/_gitignore`** template exists. Intent's own root `.gitignore` is _Elixir-flavoured_ (left over from a prior project root structure or copy-paste; contains `_build/`, `deps/`, `multiplyer-*.tar`, `.elixir_ls/`, etc.). It does **not** currently contain `.intent/` exclusions. Downstream projects manage their own `.gitignore`.

WP06 scope:

1. Flip Intent's own `.treeindexignore` from `.intent/` to granular `intent/.config/cache/` + `intent/.config/backup/`. This lets treeindex see `intent/.config/config.json` (a useful tree leaf -- describes project metadata).
2. **Decision pending in WP06**: should Intent ship a `lib/templates/_treeindexignore` template so downstream projects get the right pattern automatically? Provisional answer: **yes** -- ships a one-line `intent/.config/cache/` + `intent/.config/backup/` pattern. Adds a template; minimal new surface.
3. **Decision pending in WP06**: does Intent need a `lib/templates/_gitignore` template? Probably not for this ST -- downstream projects own their `.gitignore`. Defer to a follow-up if needed.
4. **Intent's own `.gitignore` Elixir-flavour cleanup**: out of scope for WP06 (it's pre-existing project drift; flagged for a separate concern).

## Deliverables

1. **`intent/.treeindex/.treeindexignore`** flipped: `.intent/` -> `intent/.config/cache/` + `intent/.config/backup/`. Comment line above the new patterns explaining the granularity.
2. **`lib/templates/_treeindexignore`** (new): canonical pattern for downstream projects. One file with the same granular patterns + minimal comments.
3. **`intent claude upgrade --apply`** (the canon installer; lives in `intent/plugins/claude/bin/intent_claude_upgrade`): if it currently installs `.treeindexignore` to downstream projects, ensure it picks up the new template. If it does NOT currently install `.treeindexignore`, decide: scope creep to add it, or defer. Provisional: decide in WP06; lean toward adding (small marginal cost, big value for fleet consistency).
4. **`CHANGELOG.md`** v2.10.0 entry mentions the ignore-pattern flip.

## Approach

1. Read `intent/.treeindex/.treeindexignore` end-to-end; understand each existing entry.
2. Edit line 14 (`.intent/`) -> `intent/.config/cache/` and add `intent/.config/backup/` immediately below.
3. Add a brief comment explaining the granularity (config.json is intentionally indexed; cache + backup are not).
4. Create `lib/templates/_treeindexignore` with the same content (sans Intent-specific entries like `_build/` if present).
5. **Decide on canon-installer integration**: read `intent/plugins/claude/bin/intent_claude_upgrade` for any `treeindexignore` action codes; if absent, add `INSTALL_TREEINDEXIGNORE` action + helper invocation.
6. Run `intent treeindex intent` (regenerates Intent's treeindex); verify the diff: `intent/.config/config.json` now appears as a tree leaf; `intent/.config/cache/` + `intent/.config/backup/` do not.
7. CHANGELOG entry.

## Acceptance Criteria

- [ ] `intent/.treeindex/.treeindexignore` shows `intent/.config/cache/` + `intent/.config/backup/`; the `.intent/` line is gone.
- [ ] `lib/templates/_treeindexignore` exists with the canonical pattern.
- [ ] `intent treeindex intent` (or `intent treeindex .` -- verify call signature) runs cleanly; output reflects new layout.
- [ ] Decision documented in WP06: canon-installer integrates `_treeindexignore` install, or the decision defers it (with a follow-up note).
- [ ] CHANGELOG breaking-changes entry mentions the ignore-pattern change (or is satisfied by WP03's broader entry).

### Tests to add

- BATS scenario in `tests/unit/intent_claude_upgrade.bats` (or a new file): canon installer writes `.treeindexignore` to a fresh project; content matches the template. Owned by WP05 if the canon-installer integration lands; otherwise skipped here.

### Tests to update

- None at WP06 layer beyond the BATS-fixture flips already in WP05.

## Dependencies

- **Blocks**: WP08 (self-apply runs treeindex; needs the new ignore).
- **Blocked by**: WP03 (literal sweep precedent).

## Implementation Notes

- The granular pattern (`cache/` + `backup/`) is more useful than the blanket `intent/.config/` exclusion because `config.json` is the kind of file treeindex _should_ index (it tells the LLM what version + author the project is at).
- Existing `.treeindexignore` patterns like `_build/` and `node_modules/` are language-specific and stay -- they came from the Intent project's Elixir history, but the same patterns are useful in any project where treeindex runs. No cleanup in scope.
- For `lib/templates/_treeindexignore`, the template must work for any downstream project (bash, Elixir, Rust, etc.). Keep it minimal: just the Intent-specific patterns (`intent/.config/cache/`, `intent/.config/backup/`, `intent/.treeindex/`). Downstream-project authors layer their own ignores on top.
- Canon-installer integration is the marginal-but-meaningful piece. If WP06 declines to add it, downstream projects don't get the new pattern automatically and WP08's self-apply leaves the rest of the fleet to handle this manually -- which contradicts the "fleet rollout in one pass" design.

## Risks and Edge Cases

- **Treeindex regen produces a noisy diff**: pre-existing project state may have stale entries that surface only post-flip. Mitigation: run `intent treeindex intent --prune` after the pattern flip to remove orphaned summaries.
- **Granular patterns don't match BSD vs GNU `find`**: trailing `/` semantics in ignore files vary. Test on macOS (BSD) -- Intent's primary dev environment.
- **Canon-installer scope creep**: adding `INSTALL_TREEINDEXIGNORE` to `intent_claude_upgrade` follows the existing pattern (other action codes work the same way) but adds surface. Keep the action minimal: install if absent; refresh if present and divergent (per the existing canon-install helpers).

## Verification Steps

1. `cat intent/.treeindex/.treeindexignore` -- shows new patterns.
2. `intent treeindex intent` -- runs without warnings; treeindex of `intent/.config/` now includes `config.json` as a leaf.
3. (If canon-installer integrates) `intent claude upgrade --dry-run` on a scratch project shows `INSTALL_TREEINDEXIGNORE` action.
4. (If canon-installer integrates) `intent claude upgrade --apply` on the scratch project produces `.treeindexignore` matching the template.

## Size and Estimate

- **Size**: XS (Extra Small) if canon-installer integration deferred; **S** if integration is in. Lean to XS for the minimum, with the integration as a stretch goal in the same sitting.

## Exit Checklist

- [ ] `intent/.treeindex/.treeindexignore` flipped.
- [ ] `lib/templates/_treeindexignore` created.
- [ ] Canon-installer integration decided (in or deferred with note).
- [ ] CHANGELOG entry (if WP03 didn't cover).
- [ ] Treeindex regen verified.
- [ ] Committed: `chore: ST0036/WP-06 ignore patterns for intent/.config`.
