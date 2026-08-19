//! intentsvcs -- Intent v3's Highlander layer (ST0056, design.md D06).
//!
//! All of Intent's functionality lives here: the reified model (the single
//! authored master the schema faces generate from), the SQLite store, ingest,
//! view generation, sync, and the outer facade every skin (clap, GraphQL,
//! MCP) calls. The CLI and daemon never touch the DB or the file canon --
//! `rusqlite` appears in exactly this crate's Cargo.toml, and
//! `tests/dep_graph_guard.rs` asserts it.
//!
//! Truth model (design.md D01, REVERSED by hv 2026-08-15): the schema is the
//! source of truth for structure, and **the intentdb is the durable SSOT --
//! everything on disk is a secondary artefact.** `thread.json`, the generated
//! `.md` views and the event log's file form are extracts of the same kind;
//! none of them is truth. Re-creating the DB from an extract is a CAPABILITY,
//! not a licence to treat the DB as disposable, so `rm intent/.cache/intent.db`
//! is NOT always safe -- it discards whatever the extract does not carry.
//!
//! **DB migrations are normal.** hv: *"If we have to do a db migration, we
//! have to do a db migration. That is standard fare."* The old "no migrations,
//! ever" was a CONSEQUENCE of the disposable-DB model that had been recorded
//! beside the decisions and acquired their authority; it was never a
//! constraint anyone asked for.

pub mod address;
pub mod backup;
pub mod contract;
pub mod doctor;
pub mod event;
pub mod export;
pub mod facade;
pub mod faces;
pub mod finding;
pub mod graphql;
pub mod ingest;
pub mod install;
pub mod intentfiles;
pub mod legacy;
pub mod migrate;
pub mod model;
pub mod organize;
pub mod preconditions;
pub mod project;
pub mod prose;
pub mod realise;
pub mod remedy;
pub mod rootfiles;
pub mod rules;
pub mod store;
pub mod sync;
pub mod transitions;
pub mod views;
pub mod write_set;
