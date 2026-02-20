---
verblock: "20 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-04
title: Documentation and Wrap-up
scope: Small
status: Done
---

# WP-04: Documentation and Wrap-up

## Objective

Update all project documentation to reflect the three new features from WP-01 through WP-03, and verify the full test suite passes.

## Deliverables

- Update `intent/usr/reference_guide.md`:
  - Document `-s|--start` flag syntax in `st new` section
  - Document `slug:` frontmatter field in steel thread metadata section
  - Document `escape_sed_replacement()` and `slugify()` as internal functions
- Update `intent/usr/user_guide.md`:
  - Add examples for `st new -s "Title"`
  - Add note about slug generation
- Update `CHANGELOG.md` with version entry listing all three features
- Update test count in documentation after new BATS tests
- Run full test suite, confirm all pass (302 + new tests)

## Acceptance Criteria

- [ ] Reference guide documents `-s|--start` flag syntax
- [ ] Reference guide documents `slug:` frontmatter field
- [ ] User guide has examples for the new flag
- [ ] CHANGELOG entry lists all three features
- [ ] Test count in docs matches actual count
- [ ] All tests pass

## Dependencies

- WP-03 (all code changes must be complete before documenting)
