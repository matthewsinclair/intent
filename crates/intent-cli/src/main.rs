//! The `intent` v3 binary -- WP-02 placeholder.
//!
//! The clap spine over the dispatch-table SSOT lands in WP-05; until then
//! this binary exists so the workspace shape (design.md D18: two shipped
//! binaries) and the dependency-graph guard are real from the first commit.
//! It is not installed anywhere; the v2 shell CLI remains the working tool.

fn main() {
  println!(
    "intent {} -- v3 scaffold (ST0056/WP-02); the installed v2 CLI remains the working tool",
    env!("CARGO_PKG_VERSION")
  );
}
