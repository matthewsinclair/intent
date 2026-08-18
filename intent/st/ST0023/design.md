# Design - ST0023: Remove Backlog from Intent

## Approach

DOC FIRST methodology: update all documentation first (WP01), then execute code changes (WP02-WP07), then release (WP08). This ensures documentation is reviewed before any code is modified.

## Design Decisions

1. **Blog posts get editor's notes, not full removal** - Blog posts are historical artifacts. Rather than rewriting them, add editor's notes indicating the feature was removed.

2. **TPD files get annotations, not deletion** - Technical Project Documentation is historical. Add "[Removed in v2.5.0]" annotations to backlog sections.

3. **Node.js removed from CI entirely** - Node.js was only used in CI for `npm install -g backlog.md`. Safe to remove completely.

4. **CHANGELOG history preserved** - All historical entries referencing backlog in previous versions are left untouched.

5. **Archive files untouched** - `.archive/tpd-backup-20250717/` is historical and left as-is.

6. **Config field consolidation** - Discovered both `version` and `intent_version` in config.json. Consolidated to `intent_version` as canonical, with `jq '.intent_version // .version'` fallback for legacy configs. Migration functions now `del(.version)` instead of writing both.

7. **Test sandbox for source modification tests** - Tests that simulate "source changed" (sync, status) need to modify agent.md. Rather than modifying the real source (which caused `git checkout` in teardown to revert uncommitted work), created a sandbox: temp INTENT_HOME with copied subagent sources and symlinked bin/ scripts. Key insight: manifest stores absolute paths, so sandbox must be used from install-time onward.

## Alternatives Considered

- **Deprecation period**: Could have deprecated first, then removed. Rejected because the feature has no active users and adds complexity.
- **Feature flag**: Could have hidden behind a flag. Rejected as over-engineering for a simple removal.
- **Test save/restore**: Instead of sandbox, could backup the real file before modification and restore after. Rejected because the user's requirement was "tests should NEVER work on the actual intent config."
- **Environment variable override for source path**: Could add `INTENT_SUBAGENTS_SOURCE` override to production code. Rejected as changing production code just for testing.
