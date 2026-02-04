# .treeindex Directory

> This file is auto-generated. Do not edit it by hand.

## What Are .treeindex Files?

`.treeindex` files are pre-computed directory summaries that let you quickly
orient yourself in a codebase without reading every file. Each file contains
a concise overview of a directory's purpose, subdirectories, and key files.

## Shadow Directory Structure

Treeindex files are stored in a **shadow directory** that mirrors the real
project structure. The real directory path maps to a shadow path like this:

    Real:   lib/my_app/accounts/
    Shadow: intent/.treeindex/lib/my_app/accounts/.treeindex

The shadow root is always `intent/.treeindex/` within the project.

## How to Use

Before exploring an unfamiliar directory, check for an existing `.treeindex`:

    intent/.treeindex/<dir>/.treeindex

If one exists, read it first. This avoids redundant file listing and reading
operations and saves context window space.

## Key Commands

    intent treeindex <dir>              # Generate indexes (default depth 2)
    intent treeindex <dir> --depth 3    # Deeper traversal
    intent treeindex <dir> --check      # Report stale indexes without regenerating
    intent treeindex <dir> --force      # Regenerate all indexes
    intent treeindex <dir> --prune      # Remove orphaned shadow entries
    intent treeindex <dir> --dry-run    # Preview what would be generated

## Staleness Detection

Each `.treeindex` file has an HTML comment header with a fingerprint hash:

    <!-- treeindex v1 fingerprint:abaae3bc generated:2026-02-04T11:48:23Z -->

The fingerprint is computed from filenames, file sizes, and subdirectory names
in the source directory. When the fingerprint changes, the index is stale and
will be regenerated on the next run. The fingerprint is git-clone-stable
(no mtime dependency).

## Important

- These files are **auto-generated** by `intent treeindex`. Do not edit them.
- The `.treeindexignore` file controls which files and directories are excluded.
- Run `intent treeindex <dir> --force` to regenerate after manual edits.
