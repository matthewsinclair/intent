//! intentsvcs -- Intent v3's Highlander layer (ST0056, design.md D06).
//!
//! All of Intent's functionality lives here: the reified model (the single
//! authored master the schema faces generate from), the SQLite store, ingest,
//! view generation, sync, and the outer facade every skin (clap, GraphQL,
//! MCP) calls. The CLI and daemon never touch the DB or the file canon --
//! `rusqlite` appears in exactly this crate's Cargo.toml, and
//! `tests/dep_graph_guard.rs` asserts it.
//!
//! Truth model (design.md D01): the schema is the source of truth for
//! structure; committed JSON canon is durable truth; the SQLite DB is runtime
//! truth, rebuilt from canon at any time. `rm intent/.cache/intent.db` is
//! always safe, so there are no DB migrations, ever.

pub mod event;
pub mod faces;
pub mod finding;
pub mod graphql;
pub mod ingest;
pub mod model;
pub mod project;
pub mod prose;
pub mod store;
pub mod sync;
pub mod views;
