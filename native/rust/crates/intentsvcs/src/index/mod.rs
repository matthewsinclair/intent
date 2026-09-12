//! The search index: the scope rule, the reconciler, and what each answers.
//!
//! ST0069 WP-18 builds the first of these. The module tree is the estate's PFIC
//! shape -- a pure core and an impure rim -- and it is laid out here in full so
//! that a later package adds a file rather than moving one.

pub mod corpus;
pub mod freshness;
pub mod reconcile;
