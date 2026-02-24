# WP08: Version Bump, CHANGELOG, Treeindex, and Release

## Scope

Version bump to 2.5.0, changelog entry, treeindex regeneration, tag, and release.

## Files to Edit

- VERSION - 2.4.0 -> 2.5.0
- .intent/config.json - Update version and intent_version
- CHANGELOG.md - Add v2.5.0 entry
- intent/wip.md - Update with ST0023 completion
- intent/restart.md - Update restart context

## Post-Edit Actions

1. Regenerate treeindex
2. Run full test suite
3. Final grep sweep for stragglers
4. Commit, tag, push, release

## Acceptance Criteria

- VERSION reads 2.5.0
- CHANGELOG complete
- All treeindex regenerated
- 14 test files, all passing
- Tag pushed, GitHub release created
