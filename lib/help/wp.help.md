@short: Manage work packages within steel threads

# intent wp

Manage work packages within steel threads.

## Synopsis

```
intent wp <command> [options] [arguments]
```

## Description

Work packages (WPs) are sub-units of work within a steel thread. Each WP lives in a numbered subdirectory under `STXXXX/WP/NN/` and contains an `info.md` with metadata and documentation.

## Commands

### new

Create a new work package for a steel thread.

```
intent wp new <STID> "Title"
intent wp new 11 "Implement core logic"
intent wp new ST0011 "Implement core logic"
```

Automatically assigns the next available WP number (01-99). Creates the `WP/` directory if it doesn't exist.

### done

Mark a work package as Done.

```
intent wp done <STID/NN>
intent wp done ST0011/01
intent wp done 11/01
```

Updates the `status:` frontmatter to `Done`. Prints a hint when all WPs in the steel thread are complete.

### start

Mark a work package as WIP (Work In Progress).

```
intent wp start <STID/NN>
intent wp start ST0011/01
intent wp start 11/01
```

Updates the `status:` frontmatter to `WIP`.

### list

List all work packages for a steel thread.

```
intent wp list <STID>
intent wp list ST0011
intent wp list 11
```

Displays a table with columns: WP, Title, Scope, Status.

### show

Display a work package's info.md content.

```
intent wp show <STID/NN>
intent wp show ST0011/01
intent wp show 11/01
```

### help

Display usage information.

```
intent wp help
```

## Specifier Syntax

The ST part accepts bare numbers or full IDs:

- `11` or `ST0011` both resolve to `ST0011`

The WP part is a 1-2 digit number:

- `1` or `01` both resolve to `01`

Combined specifiers use a slash:

- `ST0011/01` or `11/1` both resolve to `ST0011/01`

## WP Directory Structure

```
intent/st/ST0011/
  WP/
    01/
      info.md          # WP metadata and documentation
    02/
      info.md
```

## Examples

```bash
# Create a work package
intent wp new ST0005 "Implement core logic"

# List all WPs for a steel thread
intent wp list ST0005

# Start working on a WP
intent wp start ST0005/01

# View WP details
intent wp show ST0005/01

# Mark WP as done
intent wp done ST0005/01

# Use bare numbers
intent wp new 5 "Second work package"
intent wp list 5
intent wp start 5/02
intent wp done 5/02
```

## See Also

- `intent help st` - Steel thread management
- `intent help` - General help
