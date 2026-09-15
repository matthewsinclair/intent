//! Embeds the project templates `intent init` writes (AC-07.1).
//!
//! The logic has ONE home -- `native/rust/build-support/embed_templates.rs` --
//! because cargo requires a `build.rs` per package and two copies of an embed
//! would drift silently. That is the same reason, and the same shape, as
//! `source_commit.rs` next to it.

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

include!("../../build-support/embed_templates.rs");

fn main() {
  emit_embedded_templates();
}
