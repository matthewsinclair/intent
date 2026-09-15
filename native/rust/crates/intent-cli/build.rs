//! Embeds the commit this binary was built from (AC-11.5).
//!
//! The logic has ONE home -- `native/rust/build-support/source_commit.rs` --
//! because cargo requires a `build.rs` per package and two copies of a
//! provenance embed would drift silently. See that file for the reasoning: no
//! build time (D42), no `rerun-if-changed` on `.git/HEAD`, `dirty-<sha>` when
//! the tree is not clean, `unknown` when git cannot answer.
//!
//! IT ALSO ASSERTS VERSION PARITY, from a second file with its own name and its
//! own contract -- `version_parity.rs`. Same reason the provenance logic has one
//! home, and a different concern from it: a build script may carry two calls,
//! but a file must not carry a name that stops describing what is in it.

// IN-RS-CODE-001 governs library code, and a build script's panic fails the build
// the way `main.rs` exits (vc, 2026-09-15). CI's step denying the rule's lints for
// the library targets reaches this script only because cargo lints a package's
// build script under the same flags, so the scope is stated here, once.
#![allow(
  clippy::unwrap_used,
  clippy::expect_used,
  clippy::panic,
  reason = "IN-RS-CODE-001 covers library code, and a build script is not library code"
)]

include!("../../build-support/source_commit.rs");
include!("../../build-support/version_parity.rs");

fn main() {
  // FIRST, and the order is the point: a drifted tree must not reach the
  // provenance embed, because the artefact it would stamp is one that should
  // not exist.
  assert_version_parity();
  emit_source_commit();
}
