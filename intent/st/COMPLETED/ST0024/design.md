# Design - ST0024: Add work packages as first class citizens within a steel thread

## Approach

Add `intent wp` as a peer command to `intent st`. WPs live under `STXXXX/WP/NN/` with auto-assigned sequential numbers (01-99). Reuse existing shared helpers from `bin/intent_helpers` for ID normalization and special character escaping.

## Design Decisions

1. **Directory structure**: `STXXXX/WP/NN/info.md` -- numbered subdirectories under a WP/ parent, keeping the ST directory clean
2. **Specifier syntax**: Accept bare numbers (`5` = `ST0005`, `5/01` = `ST0005/01`) via `normalise_st_id()` from intent_helpers
3. **Auto-numbering**: WP numbers auto-assigned 01-99, scanning existing directories for the next available number
4. **Template-based creation**: Uses `lib/templates/prj/st/WP/info.md` with sed substitution, heredoc fallback for resilience
5. **Completion hints**: When the last WP is marked done, hints the user to close the parent steel thread
6. **Status tracking**: Frontmatter-based (`status: Not Started | WIP | Done`), updated via sed in-place
7. **Config via get_config_field()**: Shared helper replaces inline grep patterns (also used by intent_st)

## Architecture

```
bin/intent_wp                       # Command script
  sources bin/intent_helpers        # Shared functions
  uses normalise_st_id()            # ID normalization
  uses escape_sed_replacement()     # Special char handling
  uses get_config_field()           # Config extraction

lib/templates/prj/st/WP/info.md    # Template
lib/help/wp.help.md                 # Help text
tests/unit/wp_commands.bats         # Tests (29)
```

## Alternatives Considered

1. **Flat file tracking in tasks.md** -- Rejected. No structure, no individual status tracking, harder to reference.
2. **WP as separate command hierarchy** -- Rejected. WPs are tightly coupled to steel threads; nesting under `intent wp` with STID specifiers keeps the relationship clear.
