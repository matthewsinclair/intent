# Tasks - ST0056: Intent v3.0.0

## Tasks

The work breakdown is the 12-WP ladder (`intent wp list ST0056` is the live status; WP info.md files carry objectives and deliverables). Sizes are T-shirt.

| WP  | Title                                                              | Size | Depends                 |
| --- | ------------------------------------------------------------------ | ---- | ----------------------- |
| 01  | Design canon: architecture, data model, migration and parity specs | L    | --                      |
| 02  | Workspace and reified model: intentsvcs types, schema faces, store | L    | 01                      |
| 03  | Ingest, views and sync engine                                      | L    | 02                      |
| 04  | intentsvcs facade: core command families                           | XL   | 03                      |
| 05  | CLI in-process mode and BATS conformance harness                   | L    | 04                      |
| 06  | CLI parity long tail                                               | XL   | 05                      |
| 07  | Canon and claude subsystem                                         | L    | 04                      |
| 08  | intentd daemon                                                     | XL   | 05                      |
| 09  | MCP server and agent guide                                         | L    | 05 (08 for bridge mode) |
| 10  | Migration and fleet ingest harness                                 | XL   | 06, 07                  |
| 11  | Distribution: cargo-dist, Homebrew, signing                        | M    | 06                      |
| 12  | Cutover and v3.0.0 release                                         | L    | all                     |

## Task Notes

- 01 -> 05 is the strictly ordered spine; 06/07 interleave freely after 04; 09's stdio mode needs only the facade and can land before 08.
- WP-10 is deliberately late (it migrates the whole surface) and consumes the fleet corpus: Intent's own tree first as canary, then Lamplight/Utilz/Baize at post-sweep revisions.
- Every facade verb lands with a dual-path conformance test (in-process vs intentd, identical results).
- WP-01 review by hv is the gate before any Rust exists (document-before-code).

## Dependencies

- **v2.19.0 consumer sweeps (cc's lane) precede the fleet corpus snapshot**: the sweeps change the estates to the v2.19 grammar floor, and the migration fixture must be the post-sweep trees at named revisions.
- v2 stays on the standing fix-under-issue ruling during the build; scope of v2 maintenance is hv's call (proposed: critical fixes only).
