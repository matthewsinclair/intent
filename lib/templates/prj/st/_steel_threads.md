---
verblock: "09 Jul 2025:v0.2: Matthew Sinclair - Updated for directory structure"
---
# Steel Threads

This document serves as an index of all steel threads in the project. A steel thread represents a self-contained unit of work that focuses on implementing a specific piece of functionality.

## Index
<!-- BEGIN: STEEL_THREAD_INDEX -->
| ID                      | Title   | Status   | Created  | Completed |
|-------------------------|---------|----------|----------|-----------|
| [ST0002](<./ST0002/>) | [Title] | [Status] | YYYYMMDD | YYYYMMDD  |
| [ST0001](<./ST0001/>) | [Title] | [Status] | YYYYMMDD | YYYYMMDD  |
| ...                     | ...     | ...      | ...      | ...       |
<!-- END: STEEL_THREAD_INDEX -->

## Steel Thread Status Definitions

- **NOT-STARTED**: Steel thread has been created but work has not begun (stp/prj/st/NOT-STARTED/)
- **IN-PROGRESS**: Work is actively being done on this steel thread (stp/prj/st/)
- **COMPLETED**: All tasks have been completed and the steel thread is finished (stp/prj/st/COMPLETED)
- **HOLD**: Work has been temporarily paused (stp/prj/st)
- **CANCELLED**: The steel thread has been cancelled and will not be completed (stp/prj/st/CANCELLED)

## Context for LLM

This document provides an overview of all steel threads in the project. It helps track the progress of individual pieces of work and serves as a navigation aid for finding specific steel thread documents.

### How to use this document

1. Update the index when creating new steel threads
2. Update the status of steel threads as they progress
3. Add completion dates when steel threads are finished
4. Use this document to quickly locate specific steel thread documents

The detailed information for each steel thread is contained in its directory (e.g., ST0001/) with multiple files:
- info.md: Main information and metadata
- design.md: Design decisions and approach
- impl.md: Implementation details
- tasks.md: Task tracking
- results.md: Results and outcomes
