---
verblock: "09 Jul 2025:v0.1: Matthew Sinclair - Initial deprecations document"
stp_version: 1.2.0
---
# STP Deprecations

This document tracks features, files, and functionality that have been deprecated in STP.

## July 9, 2025: journal.md

### What was deprecated
The `stp/prj/journal.md` file and associated functionality.

### Why it was deprecated
With the integration of Backlog.md for task management, the journal.md file became redundant. Backlog provides:
- Better structured task tracking with metadata (status, priority, dependencies)
- Automatic linking to steel threads
- More flexible historical tracking through task history
- Integration with modern development workflows

### Migration path
Users who were using journal.md for historical tracking should:
1. Use `stp bl list` to view task history
2. Use steel thread documents for high-level context and decisions
3. Use Backlog task descriptions for detailed implementation notes

### Where to find the deprecated content
The original journal.md file has been archived at `stp/prj/archive/journal-deprecated.md` with a deprecation notice.

### Impact
- The `stp init` command no longer creates journal.md
- Documentation has been updated to reference Backlog for historical tracking
- The journal.md template has been removed from `_templ/prj/`

### Version deprecated
STP version 1.0.0