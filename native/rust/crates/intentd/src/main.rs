//! The `intentd` v3 daemon -- WP-02 placeholder.
//!
//! The real daemon (project registry, unix-socket GraphQL, mgmt plane,
//! debounced watching, launchd lifecycle owned by the CLI) is WP-08. This
//! binary exists so the workspace shape (design.md D18) and the
//! dependency-graph guard are real from the first commit.

fn main() {
  // **NO PROJECT-MANAGEMENT STATE IN SHIPPED OUTPUT** (D37). This line used to
  // read "v3 scaffold (ST0056/WP-02); the daemon lands in WP-08" -- Intent's own
  // thread and work-package numbers, printed into a consumer's terminal by a
  // built binary. vc flagged it in source and dc confirmed it in the artefact.
  // What a user needs from `--version` is the version; which of our work
  // packages will finish the daemon is our business, and it stays in the module
  // note above.
  println!(
    "intentd {} -- not yet implemented",
    env!("CARGO_PKG_VERSION")
  );
}
