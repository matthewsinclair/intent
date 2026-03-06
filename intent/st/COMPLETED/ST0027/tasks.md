# Tasks - ST0027: Add /in-cost-analysis skill

## WP-01: Steel thread docs + skill scaffold

- [x] Update ST0027/info.md with objective and context
- [x] Create ST0027/design.md with skill architecture
- [x] Create WPs via `intent wp new`
- [x] Create skill directory: `intent/plugins/claude/skills/in-cost-analysis/`
- [x] Write SKILL.md (139 lines) -- generic procedural guide
- [x] Write data/reference-rates.md (84 lines) -- rate tables and multipliers

## WP-02: Bash metrics script

- [x] Write scripts/cost-metrics.sh (429 lines), bash 3.x compatible
- [x] CLI flags: --dir, --exclude, --git/--no-git, -o
- [x] Extension-to-language mapping via case (25+ extensions, 22 languages)
- [x] LOC counting with comment detection per language family
- [x] Tier classification by file path patterns
- [x] Git history analysis (optional, --no-git to skip)
- [x] JSON output (manual construction via awk + temp files)

## WP-03: Integration testing

- [x] Verify `intent claude skills list` shows in-cost-analysis
- [x] Verify `intent claude skills install in-cost-analysis` copies full tree (SKILL.md + scripts/ + data/)
- [x] Verify cost-metrics.sh runs against Intent project (44 files, 10,665 code lines, 2 languages)
- [x] Verify JSON output is valid (python3 -m json.tool)
- [x] Verify find-based path resolution works from installed location
- [x] Verify `intent claude skills sync` detects changes
- [x] Update ST0027 docs with as-built state
