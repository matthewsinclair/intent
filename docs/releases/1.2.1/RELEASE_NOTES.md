# STP v1.2.1 Release Notes

## Release Date: July 9, 2025

## Overview

STP v1.2.1 introduces a major architectural change in how steel threads are organized. Steel threads are now structured as directories containing multiple files, providing better separation of concerns and supporting richer documentation.

## Breaking Changes

### Steel Thread Directory Structure

Steel threads have been converted from single markdown files (`ST####.md`) to directory structures (`ST####/`) containing multiple files:

```
ST####/
├── info.md      # Main information and metadata (required)
├── design.md    # Design decisions and approach
├── impl.md      # Implementation details
├── tasks.md     # Task tracking
└── results.md   # Results and outcomes
```

**Migration**: The `stp upgrade` command automatically migrates existing steel threads to the new structure, creating backups in `.stp_backup/1.2.1/`.

## New Features

### Enhanced Steel Thread Commands

- `stp st show ST0001 design` - View specific file from a steel thread
- `stp st show ST0001 all` - View all files from a steel thread
- `stp st edit ST0001 impl` - Edit specific file from a steel thread
- `stp st organize` - Organize steel thread directories by status

### Automatic Migration

- `stp upgrade` now handles v1.2.0 → v1.2.1 migration automatically
- Creates backups of original files before migration
- Preserves all content and metadata
- Splits content intelligently into appropriate files

### Backward Compatibility

- Version detection ensures commands work with both old and new structures
- Projects can remain on v1.2.0 if desired
- Migration is prompted but not forced

## Improvements

### Better Organization

- Separate files for different aspects of documentation
- Easier to work with specific sections without affecting others
- Reduced file size and improved readability
- Better git diff visibility for specific changes

### Enhanced Workflow

- Can edit design decisions without touching implementation notes
- Task tracking separated from main documentation
- Results and outcomes clearly separated

### Documentation Updates

- All documentation updated to reflect new structure
- Help files enhanced with new command options
- Blog posts updated with examples
- Reference guide comprehensively updated

## Bug Fixes

- Fixed template lookup issues in test environments
- Fixed recursive script calls in sync command
- Updated all tests to work with directory structure

## Upgrade Instructions

1. Ensure you have committed any pending changes
2. Run `stp upgrade` from your project root
3. When prompted, confirm the migration to v1.2.1
4. Review the migrated steel threads in their new directory structure
5. Original files are backed up in `.stp_backup/1.2.1/`

## Technical Details

### Version Detection

STP now uses a version file (`.config/version`) to detect the structure:
- v1.2.0 and below: Single file structure
- v1.2.1 and above: Directory structure

### Migration Process

The migration script:
1. Creates backup directory `.stp_backup/1.2.1/`
2. Copies original files to backup
3. Creates new directory structure
4. Splits content based on section headers
5. Preserves all metadata and content
6. Updates references in steel_threads.md

## Known Issues

- None at this time

## Future Enhancements

- Consider adding more specialized files (e.g., `testing.md`, `metrics.md`)
- Enhanced templates for each file type
- Better integration with external task management systems
- Automated status updates based on file content

## Contributors

- Matthew Sinclair
- Claude (AI Assistant)

## Support

For issues or questions, please refer to:
- STP Documentation: `stp/usr/reference_guide.md`
- Help System: `stp help st`
- GitHub Issues: [Create an issue](https://github.com/matthewsinclair/stp/issues)