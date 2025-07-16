# Design - ST0012: Document Sync Command

## Approach

1. Create a new `sync` option in the `stp_st` script
2. Add support for section markers in the steel_threads.md document
3. Read metadata from each ST####.md file
4. Generate updated content for the marked sections in steel_threads.md
5. Either output the updated content to stdout or write it to the file based on options

