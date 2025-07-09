---
verblock: "09 Jul 2025:v0.2: Updated for v1.2.1 directory migration"
---
# upgrade

@short:
Upgrade STP files to the latest format

@desc:
The upgrade command scans all STP files and brings them up to date with the latest version.
It adds or updates metadata and ensures all files follow the current format standards.

The upgrade process includes:
- Adding STP version information to files
- Adding or updating YAML frontmatter metadata
- Adding section markers to steel_threads.md for sync
- Ensuring all files have the correct structure and format
- Migrating steel threads from single files to directories (v1.2.0 → v1.2.1)

The command checks the version of each file and only upgrades files that need it.
For major version differences, a warning is displayed unless --force is used.

@usage:
stp upgrade [--force] [--organize]

Options:
  --force      Force upgrade even for major version differences
  --organize   Organize steel thread directories by status after upgrade

@examples:
# Upgrade all STP files
stp upgrade

# Force upgrade even for major version differences
stp upgrade --force

@notes:
- The upgrade process doesn't remove any content from your files
- All files are backed up before modification
- After upgrading, run 'stp st sync' to update the steel_threads.md file
- The current STP version is stored in each file's frontmatter
- v1.2.1 migration: Converts ST####.md files to ST####/ directories
  - Splits content into separate files (info.md, design.md, impl.md, etc.)
  - Backs up original files to .stp_backup/1.2.1/
  - Preserves all content and metadata