//! Embeds the commit this binary was built from (AC-11.5).
//!
//! The logic has ONE home -- `native/rust/build-support/source_commit.rs` --
//! because cargo requires a `build.rs` per package and two copies of a
//! provenance embed would drift silently. See that file for the reasoning: no
//! build time (D42), no `rerun-if-changed` on `.git/HEAD`, `dirty-<sha>` when
//! the tree is not clean, `unknown` when git cannot answer.
//!
//! `intentd` NEEDS THIS AS MUCH AS `intent-cli` AND ARGUABLY MORE: it is the
//! binary that was measured FORTY-TWO hours older than the commit it was
//! recorded under, and forty-two hours apart from its sibling. A check covering
//! one binary of a two-binary release reports on the release, so half the embed
//! would have left the pipeline's verdict reading as one verdict over an
//! artefact that could not answer.

include!("../../build-support/source_commit.rs");

fn main() {
  emit_source_commit();
}
