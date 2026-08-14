//! The `intentd` v3 daemon -- WP-02 placeholder.
//!
//! The real daemon (project registry, unix-socket GraphQL, mgmt plane,
//! debounced watching, launchd lifecycle owned by the CLI) is WP-08. This
//! binary exists so the workspace shape (design.md D18) and the
//! dependency-graph guard are real from the first commit.

fn main() {
  println!(
    "intentd {} -- v3 scaffold (ST0056/WP-02); the daemon lands in WP-08",
    env!("CARGO_PKG_VERSION")
  );
}
