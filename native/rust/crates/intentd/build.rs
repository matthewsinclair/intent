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
//!
//! IT ALSO ASSERTS VERSION PARITY, from a second file with its own name and its
//! own contract -- `version_parity.rs`. Same reason the provenance logic has one
//! home, and a different concern from it: a build script may carry two calls,
//! but a file must not carry a name that stops describing what is in it.

include!("../../build-support/source_commit.rs");
include!("../../build-support/version_parity.rs");

fn main() {
  // FIRST, and the order is the point: a drifted tree must not reach the
  // provenance embed, because the artefact it would stamp is one that should
  // not exist.
  assert_version_parity();
  emit_source_commit();
}
