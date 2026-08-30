//! `intent tui` -- the terminal realiser (WP-17 piece 3).
//!
//! **THE MACHINE LANDS BEFORE THE REALISER, AND SEPARATELY.** [`mode`] is the
//! declared mode graph and its invariants; it compiles against nothing new, so
//! the properties the realiser must satisfy are provable before a single
//! terminal dependency enters `Cargo.lock` -- a file three nodes build against.
//! The realiser is checked AGAINST this module, so the order is not a
//! convenience: a machine that could only be exercised through `ratatui` would
//! be tested by the thing it exists to constrain.

//! [`focus`] is the third of these and it landed the same way: `AC-17.5` is a
//! property of a DECLARATION -- tab order is declaration order, total and
//! reversible -- so it needs no terminal either, and I had it queued behind
//! `ratatui` until I read the criterion instead of my summary of it.

pub mod focus;
pub mod mode;
pub mod terminal;
