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
pub mod hatch;
pub mod mcp;
pub mod mcp_stdio;
pub mod render;
pub mod show;
pub mod spine;
pub mod tui;

/// What one `intent` invocation came to: the exit code, and the message that
/// belongs on stderr if there is one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
  pub code: i32,
  pub message: Option<String>,
}

/// **ONE `intent` INVOCATION, FROM ARGV TO AN EXIT CODE. THE ONLY PLACE THAT
/// SEQUENCE LIVES.**
///
/// It was `main`'s body until the explorer needed to run a command too
/// (`/{cmd} ...`, hv 2026-09-02). Writing those four lines again in the TUI
/// realiser would have been a second home for *how you run an intent command*
/// -- and the copies would agree right up until one of them learned something
/// about exit codes that the other did not (`IN-AG-HIGHLANDER-001`). So the
/// palette does not re-implement dispatch, it CALLS it, and the command an
/// operator runs from the explorer is by construction the command they would
/// have got from a shell.
///
/// **THE ARGV CARRIES THE PROGRAM NAME**, because `clap` reads argv[0] as the
/// binary and a caller that forgets it loses its first real argument silently.
///
/// Printing is NOT done here. The two callers want different things -- `main`
/// exits, and the TUI is mid-repaint with a terminal it has lent -- and a
/// function that both dispatches and reports would force one of them to work
/// around the other's choice.
pub fn dispatch(argv: Vec<String>) -> Outcome {
  let matches = match spine::parse(argv) {
    Ok(matches) => matches,
    // `parse` has already written clap's own error or help text to the
    // stream clap chose; there is nothing left to say and saying it twice
    // would be worse than saying it once.
    Err(code) => {
      return Outcome {
        code,
        message: None,
      };
    }
  };
  match render::run(&matches) {
    Ok(()) => Outcome {
      code: spine::EXIT_OK,
      message: None,
    },
    Err(failure) => Outcome {
      code: failure.code(),
      message: failure.message().map(str::to_string),
    },
  }
}

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
/// The artefact's own VERSION, embedded for the same reason as the commit
/// beside it: a reader holding a SIBLING binary can otherwise only substitute
/// its own `CARGO_PKG_VERSION`, which is a claim about the reader. `intent` and
/// `intentd` are separately-built artefacts that have been measured
/// forty-two hours apart, so a surface naming both must read both.
///
/// A SEPARATE MARKER rather than a wider commit one -- five parsers capture
/// `[intent-source-commit:...]` with `[^]]*`, and widening it would change what
/// every one of them captures.
#[used]
static SOURCE_VERSION_MARKER: &str = env!("INTENT_SOURCE_VERSION_MARKER");
