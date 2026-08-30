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
//! source of truth for structure, and **the SQLite db is the durable SSOT --
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

// **`FacadeError` IS LARGE BY CONSTRUCTION, AND THE LINT IS RIGHT ABOUT THE
// FACT WHILE BEING WRONG ABOUT THE PRIORITY.** Its largest variant is
// `Organize(#[from] OrganizeError)`, whose `PreconditionsUnmet` carries a
// `Verdict` plus the threads it names -- because a refusal that cannot say
// WHICH preconditions are unmet, on WHICH threads, is exactly the collapse
// `error_remedies.rs` exists to refuse. Shrinking the error by dropping what
// makes it actionable would trade a real property for a size.
//
// So this is a DEFERRAL, not a dismissal: boxing is the correct fix and it is
// sequenced after the v3.0.1 tag rather than taken mid-cut, on hv's ruling and
// in the same shape as 0136. `PreconditionsUnmet` is 14 sites across 5 files;
// the wider `Verdict` ripple is UNMEASURED, because that name is overloaded in
// this workspace and a count by analogy would be a number nobody drove.
//
// Scoped to this crate and named here rather than silenced at the gate, which
// is what `bin/.devbin/cmd/prepush` asked for when it said to justify a lint
// with a scoped allow naming the reason.
#![allow(clippy::result_large_err)]

pub mod address;
pub mod backup;
pub mod bootstrap;
pub mod canon;
pub mod contract;
pub mod critic;
pub mod daemon;
pub mod doctor;
pub mod event;
pub mod export;
pub mod facade;
pub mod faces;
pub mod finding;
pub mod form;
pub mod graphql;
pub mod ingest;
pub mod init;
pub mod install;
pub mod intentfiles;
pub mod legacy;
pub mod migrate;
pub mod model;
pub mod modules;
pub mod nav;
pub mod organize;
pub mod output;
pub mod plugins;
pub mod preconditions;
pub mod project;
pub mod prose;
pub mod realise;
pub mod remedy;
pub mod rootfiles;
pub mod rules;
pub mod skills;
pub mod store;
pub mod sync;
pub mod transitions;
pub mod userstate;
pub mod views;
/// The CLI/daemon request envelope. Here for `daemon`'s reason: two binaries
/// must agree, and this is the only crate both depend on.
pub mod wire;
pub mod write_set;
