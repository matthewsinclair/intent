//! The `intentd` v3 daemon -- WP-02 placeholder.
//!
//! The real daemon (project registry, unix-socket GraphQL, mgmt plane,
//! debounced watching, launchd lifecycle owned by the CLI) is WP-08. This
//! binary exists so the workspace shape (design.md D18) and the
//! dependency-graph guard are real from the first commit.

// NO `SOURCE_COMMIT` CONST HERE, DELIBERATELY, AND THE ASYMMETRY WITH
// `intent-cli` IS THE POINT RATHER THAN AN OVERSIGHT. There it is `pub` in a
// lib, so it is real API something can read. `intentd` has no lib, so a const
// here is unreadable by anything, forever -- `dead_code` said so under
// `-D warnings` and it was right. Silencing that with `#[allow(dead_code)]`
// would have kept a declaration whose only purpose was to look symmetrical.
// The marker below is the whole artefact-facing contract; it is what every
// consumer greps, and `#[used]` is what makes it survive.

/// The string `int macos publish` and `self_provenance_check.sh` grep out of the
/// ARTEFACT.
///
/// SELF-DELIMITING, and that is not cosmetic. Rodata packs string literals with
/// no separator between them, so an unterminated marker runs straight into
/// whatever the linker laid down next -- measured during this row's canary as
/// `intent-source-commit:<sha>unsafe`, with `unsafe` belonging to an unrelated
/// literal. The fix belongs here in the artefact rather than in each consumer's
/// pattern, because hardening one grep only moves the trap to the next consumer.
///
/// `#[used]` because the whole point is that it survives into the binary even
/// though no code path reads it: a provenance marker the linker is free to drop
/// is one that vanishes under `--release`, which is the one build where it
/// matters. IT LIVES IN `main.rs` BECAUSE `intentd` HAS NO LIB, and a lib target
/// is deliberately NOT added to give a static a home -- that would reshape the
/// crate for the sake of where a marker lives (cc's call, and the right one:
/// `intent-cli` having a lib is incidental rather than the pattern).
#[used]
static SOURCE_COMMIT_MARKER: &str =
  concat!("[intent-source-commit:", env!("INTENT_SOURCE_COMMIT"), "]");

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
