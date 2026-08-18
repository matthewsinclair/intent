# Implementation - ST0012: Document Sync Command

## Implementation Notes

The implementation uses section markers in the form of HTML-like comments to identify the parts of the document that should be updated:

```
<!-- BEGIN: STEEL_THREAD_INDEX -->
(content will be replaced during sync)
<!-- END: STEEL_THREAD_INDEX -->
```

This allows for automatic updates to specific sections while preserving the rest of the document.

### Key Implementation Details

1. **New `sync` Command**: Added a new `sync` subcommand to the `stp_st` script that reads the individual steel thread files and updates the steel_threads.md document.

2. **Metadata Extraction**: The command extracts metadata (status, created date, completed date) from individual ST####.md files using both:
   - Metadata in the YAML frontmatter (with keys `status`, `created`, and `completed`)
   - Information in the document body (with lines like `- **Status**: In Progress`)

3. **Section Markers**: Added HTML-style comment markers to designate sections in the steel_threads.md file that can be automatically updated.

4. **Configurable Output**: Added options to customize the output:
   - `--write`: Updates the steel_threads.md file directly
   - `--width N`: Specifies the width of the output table for proper column alignment
5. **Format Consistency**: Ensures proper table formatting and column alignment for better readability.

This approach makes the system more maintainable by ensuring the steel_threads.md document stays in sync with the individual ST files, reducing manual maintenance work.
