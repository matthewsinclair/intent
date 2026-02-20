# Design - ST0022: Harden `st new` -- Special Characters, Slugs, and --start Flag

## Approach

Three independent fixes to `bin/intent_st`, phased linearly to avoid merge conflicts. Each phase adds a function, modifies the `new` command, and includes BATS tests.

## Design Decisions

### 1. Sed Escaping Strategy

The `new` command uses `sed -e "s/\[Title\]/$TITLE/g"` to substitute the title into template files. Characters `/`, `&`, `\`, and `$` have special meaning in sed replacement strings and must be escaped.

**Implementation**: Add `escape_sed_replacement()` function that escapes these four characters:

```bash
escape_sed_replacement() {
  local s="$1"
  s="${s//\\/\\\\}"   # \ -> \\  (must be first)
  s="${s//\//\\/}"     # / -> \/
  s="${s//&/\\&}"      # & -> \&
  s="${s//$/\\$}"      # $ -> \$  (Note: use single quotes around pattern)
  echo "$s"
}
```

Also fix the heredoc fallback path (lines 374-400): the `cat > "$ST_DIR/info.md" << EOF` uses an unquoted heredoc delimiter, which causes shell expansion of `$` and backticks in the title. Change to `printf` for the title line, or use a quoted heredoc (`<< 'EOF'`) with explicit variable substitution afterward.

### 2. Slug Algorithm

```
slugify(title):
  1. Lowercase the entire string
  2. Replace any run of non-alphanumeric characters with a single hyphen
  3. Strip leading and trailing hyphens
  4. If length > 50, truncate at the last hyphen before position 50
     (if no hyphen found, hard-truncate at 50)
  5. Strip any trailing hyphen from truncation
```

**Examples**:

| Title                                           | Slug                                 |
| ----------------------------------------------- | ------------------------------------ |
| My Cool Feature                                 | my-cool-feature                      |
| Ash/Ecto Database Layer                         | ash-ecto-database-layer              |
| Costs & Benefits                                | costs-benefits                       |
| Price is $100                                   | price-is-100                         |
| A Very Long Title That Exceeds Fifty Characters | a-very-long-title-that-exceeds-fifty |

### 3. Slug in Frontmatter

New `slug:` field added after `status:` in YAML frontmatter:

```yaml
---
verblock: "20 Feb 2026:v0.1: matts - Initial version"
intent_version: 2.4.0
status: Not Started
slug: my-cool-feature
created: 20260220
completed:
---
```

The slug is auto-generated at creation time and never modified afterward. It is informational only -- not used for lookups or as an alternative identifier.

### 4. Slug Replaces Title in Index

`st list` and `st sync --write` currently show a "Title" column. Replace with "Slug" column:

```
ID         | Slug                     | Status       | Created    | Completed
---------- | ------------------------ | ------------ | ---------- | ----------
ST0023     | my-cool-feature          | WIP          | 2026-02-20 |
ST0022     | harden-st-new            | WIP          | 2026-02-20 |
```

The slug is extracted from frontmatter (`grep -m 1 "^slug:" "$file"`). For steel threads created before this change (no `slug:` field), fall back to extracting the title as before.

### 5. `-s|--start` Flag

Parsed before the positional title argument in the `new` command:

```bash
# Parse flags
START_FLAG=0
while [ $# -gt 0 ]; do
  case "$1" in
    -s|--start) START_FLAG=1; shift ;;
    -*) error "Unknown option: $1" ;;
    *) break ;;
  esac
done
TITLE="$1"
```

After successful creation, if `START_FLAG=1`, invoke start logic inline (not a subprocess):

- Update `status:` frontmatter from `Not Started` to `WIP`
- Update `- **Status**:` body line if present
- Move directory from `NOT-STARTED/` to main `intent/st/`
- Update index

This reuses the same logic as the existing `start` command (lines 513-637) but avoids a subprocess call to keep output clean.

### 6. Template Update

Add `slug:` placeholder to `lib/templates/prj/st/ST####/info.md`:

```yaml
---
verblock: "[Date]:v0.1: [Author] - Initial version"
intent_version: 2.4.0
status: Not Started
slug: [Slug]
created: YYYYMMDD
completed:
---
```

The `[Slug]` placeholder is substituted alongside `[Title]` using the same sed pipeline.

## Architecture

```
bin/intent_st
  +-- escape_sed_replacement()     (NEW -- WP-01)
  +-- slugify()                    (NEW -- WP-02)
  |
  case "new")
  |   +-- parse -s|--start flag    (NEW -- WP-03)
  |   +-- escape title for sed     (NEW -- WP-01)
  |   +-- generate slug            (NEW -- WP-02)
  |   +-- sed with escaped title + slug  (MODIFIED -- WP-01/02)
  |   +-- heredoc fallback with printf   (MODIFIED -- WP-01)
  |   +-- inline start logic       (NEW -- WP-03)
  |
  case "list")
  |   +-- extract slug from frontmatter  (MODIFIED -- WP-02)
  |   +-- show "Slug" column instead of "Title"  (MODIFIED -- WP-02)
  |
  case "sync")
      +-- slug column in --write output  (MODIFIED -- WP-02)

lib/templates/prj/st/ST####/info.md
  +-- slug: [Slug] placeholder     (MODIFIED -- WP-02)
```

## Alternatives Considered

1. **Slug as alternative ST identifier** (e.g. `intent st show my-cool-feature`) -- Rejected. Scope creep; ST IDs are already short and unique. Slug is informational only.
2. **Additional Slug column instead of replacing Title** -- Rejected. The table is already 5 columns at 80 chars; a 6th column would overflow or force unreadable truncation.
3. **Subprocess call for --start** (`"$SCRIPT_PATH" start "$ST_ID"`) -- Rejected. Produces duplicate output messages and adds process overhead for a simple inline operation.
4. **Use `perl` instead of `sed` for escaping** -- Rejected. Adds a dependency; sed escaping with 4 characters is straightforward.
