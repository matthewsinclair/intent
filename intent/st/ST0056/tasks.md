# Tasks - ST0056: Intent v3.0.0

## Tasks

The work breakdown is the WP ladder below (`intent wp list ST0056` is the live status; WP info.md files carry objectives and deliverables). Sizes are T-shirt.

| WP  | Title                                                                                                   | Size | Depends                 |
| --- | ------------------------------------------------------------------------------------------------------- | ---- | ----------------------- |
| 01  | Design canon: architecture, data model, migration and parity specs                                      | L    | --                      |
| 02  | Workspace and reified model: intentsvcs types, schema faces, store                                      | L    | 01                      |
| 03  | Ingest, views and sync engine                                                                           | L    | 02                      |
| 04  | intentsvcs facade: core command families                                                                | XL   | 03                      |
| 05  | CLI in-process mode and BATS conformance harness                                                        | L    | 04                      |
| 06  | CLI parity long tail                                                                                    | XL   | 05                      |
| 07  | Canon and claude subsystem                                                                              | L    | 04                      |
| 08  | intentd daemon                                                                                          | XXL  | 05                      |
| 09  | MCP server and agent guide                                                                              | L    | 05 (08 for bridge mode) |
| 10  | Migration and fleet ingest harness                                                                      | XL   | 06, 07                  |
| 11  | Distribution: Homebrew tap, signing, notarisation                                                       | M    | 06                      |
| 12  | Cutover and v3.0.0 release                                                                              | L    | all                     |
| 13  | Project search: full-text, structural, and the agent search surface                                     | XL   | 03, 08, 09              |
| 14  | Coordination model: whiteboard and inboxes in the store, with a bounded API                             | L    | 02, 03, 13              |
| 15  | Skills catalogue triage: KEEP, UPDATE or RETIRE every Intent2-era skill                                 | L    | 06, 07; before 12       |
| 16  | Contract drift: a shipped field with no model row is refused                                            | S    | --                      |
| 17  | Form DSL: one declaration, TUI and WEB realisers, CRUD through intentsvcs                               | XL   | --                      |
| 18  | Store growth: the doc-section index duplicates on every mutation, and explore gets a progress indicator | S    | --                      |

## Task Notes

- 01 -> 05 is the strictly ordered spine; 06/07 interleave freely after 04; 09's stdio mode needs only the facade and can land before 08.
- **13, 14 and 16 are cancelled**, each with its criteria descoped to ST0069 (post-cut). WP-13's retirement half landed first, so WP-06 never ported `treeindex` or the `in-handoff` skill (`fileindex` was retired separately); its search half is ST0069's. WP numbers are creation order; the Depends column is the sequence.
- WP-10 is deliberately late (it migrates the whole surface) and consumes the fleet corpus: Intent's own tree first as canary, then Lamplight/Utilz/Baize at named revisions.
- Conformance as built: `dual_path_conformance.rs` drives the whole shipped surface through the `intent` binary and in-process and requires identical results, and `daemon_and_local_agree.rs` runs every daemon-servable verb (in 3.0.1, `st list`) locally and through a real `intentd`.
- WP-01 review by hv was the gate before any Rust existed (document-before-code).

## Dependencies

- **The WP-10 corpus is the fleet AS IT IS, at named revisions -- not "post-sweep trees".** The sweep program is dead: Lamplight is already at 2.19.0 and their hv ruled AT remediation on Done work dead outright (their `aaf4d3b2b`, widened `7f5c0bd9a`), so the legacy-grammar rows are the permanent state of that estate, not a transitional one. The migrator meets them for real -- refuse-and-name becomes MORE load-bearing -- and migration.md carries hv's ruling on the policy question this forced: closed threads carry, and live threads stay BLOCKED-until-clean.
- v2 was held on the standing fix-under-issue ruling during the build. The v2 Bash CLI has since been deleted from this repository (`d5998ac37`); v2.19.0 remains available as the `v2.19.0` tag.
