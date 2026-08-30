//! `intent tui` -- the terminal realiser (WP-17 piece 3).
//!
//! **THE MACHINE LANDS BEFORE THE REALISER, AND SEPARATELY.** [`mode`] is the
//! declared mode graph and its invariants; it compiles against nothing new, so
//! the properties the realiser must satisfy are provable before a single
//! terminal dependency enters `Cargo.lock` -- a file three nodes build against.
//! The realiser is checked AGAINST this module, so the order is not a
//! convenience: a machine that could only be exercised through `ratatui` would
//! be tested by the thing it exists to constrain.

pub mod mode;
pub mod terminal;
