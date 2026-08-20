//! Embeds the project templates `intent init` writes (AC-07.1).
//!
//! The logic has ONE home -- `native/rust/build-support/embed_templates.rs` --
//! because cargo requires a `build.rs` per package and two copies of an embed
//! would drift silently. That is the same reason, and the same shape, as
//! `source_commit.rs` next to it.

include!("../../build-support/embed_templates.rs");

fn main() {
  emit_embedded_templates();
}
