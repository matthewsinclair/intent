//! `intent-cli` -- the v3 CLI, as a library plus a thin binary.
//!
//! A library because the dispatch table is not the CLI's private business:
//! it is the command-surface SSOT that WP-09's MCP tool tier and `intent llm`
//! agent guide also render from (AC-09.1, AC-09.4). Publishing it here means
//! those consumers read the same table this binary dispatches from, rather
//! than a copy that agrees until it does not.
//!
//! Nothing in this crate touches the DB or the file canon -- it reaches data
//! only through the intentsvcs facade, and `rusqlite` is absent from its
//! manifest by rule (design.md D06, asserted by `dep_graph_guard.rs`).

pub mod dispatch;
pub mod guide;
pub mod mcp;
pub mod render;
pub mod spine;
pub mod tui;

/// The commit this binary was built from, embedded by `build.rs` (AC-11.5).
///
/// `dirty-<sha>` when the tree had uncommitted changes and `unknown` when git
/// could not answer -- neither of which any correct parser reads as a commit id.
/// The dirt is carried INSIDE the value rather than beside it in a second field,
/// so a consumer that forgets to read the second field cannot silently treat a
/// dirty build as a clean one (cc's framing, and it is the same shape as putting
/// a verdict's scope in the verdict line rather than in a footnote).
pub const SOURCE_COMMIT: &str = env!("INTENT_SOURCE_COMMIT");

/// The string `self_provenance_check.sh` greps out of the ARTEFACT.
///
/// SELF-DELIMITING, and that is not cosmetic. Rodata packs string literals with
/// no separator between them, so an unterminated marker runs straight into
/// whatever the linker laid down next -- measured during this row's canary as
/// `intent-source-commit:<sha>unsafe`, with `unsafe` belonging to an unrelated
/// literal. A consumer greedily matching the value would have captured the
/// neighbour SILENTLY. The fix belongs here in the artefact rather than in the
/// consumer's pattern: hardening one grep only moves the trap to the next
/// consumer, and the brackets give every extraction an end that does not depend
/// on what the linker happens to lay down afterwards.
///
/// `#[used]` because the whole point is that it survives into the binary even
/// though no code path reads it: a provenance marker the linker is free to drop
/// is a provenance marker that vanishes under `--release`, which is the one
/// build where it matters.
#[used]
static SOURCE_COMMIT_MARKER: &str = env!("INTENT_SOURCE_COMMIT_MARKER");
