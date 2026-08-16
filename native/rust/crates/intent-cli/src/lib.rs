//! `intent-cli` -- the v3 CLI, as a library plus a thin binary.
//!
//! A library because the dispatch table is not the CLI's private business:
//! it is the command-surface SSOT that WP-09's MCP tool tier and `intent llm`
//! agent guide also render from (AC-09.1, AC-09.4). Publishing it here means
//! those consumers read the same table this binary dispatches from, rather
//! than a copy that agrees until it does not.
//!
//! Nothing in this crate touches the DB or the file canon -- it reaches data
//! only through the intentsvcs facade, and `rusqlite` is absent from its
//! manifest by rule (design.md D06, asserted by `dep_graph_guard.rs`).

pub mod dispatch;
pub mod guide;
pub mod render;
pub mod spine;
