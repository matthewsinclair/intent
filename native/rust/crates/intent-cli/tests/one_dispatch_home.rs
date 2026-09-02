//! AT-17.15 / AC-17.15: the palette's CLI arm has ONE dispatch home.
//!
//! `intent_cli::dispatch` is the only place `spine::parse` is followed by
//! `render::run`. `main` calls it; the explorer's `/{cmd} ...` realiser calls
//! it; nothing performs the sequence for itself.
//!
//! **A SOURCE-READING GUARD, FOR THE REASON `dep_graph_guard.rs` IS ONE.** The
//! property is about how many places a thing is written, and no behavioural
//! test can see that: a second copy answers identically on the day it is
//! written, which is exactly why it gets written. **The failure mode is DRIFT,
//! NOT ABSENCE** -- the copies agree until one of them learns something about
//! exit codes that the other does not, and nothing fails while they agree.
//!
//! This is the shape the estate has already been bitten by twice on this
//! surface. `via_library` in `dual_path_conformance.rs` was a third hand-rolled
//! parse-then-run, in the file whose entire job is proving the library and the
//! binary agree -- so the route it compared against the binary was one NOTHING
//! SHIPPED. It agreed for as long as nobody changed either, which is the whole
//! defect stated as a duration.
//!
//! # What this cannot see, stated so a green is not read as more
//!
//! It matches TEXT. A caller that reached the same sequence through an alias,
//! a re-export, or a macro would not be found, and a comment naming the two
//! functions in prose is deliberately not a violation (comment lines are
//! stripped, as in `dep_graph_guard.rs`). It is a guard against the copy
//! somebody types, which is the one that actually gets typed.

use std::fs;
use std::path::{Path, PathBuf};

use testkit::workspace_root;

/// Every `.rs` file under a directory, recursively.
fn sources(dir: &Path) -> Vec<PathBuf> {
  let mut out = Vec::new();
  let Ok(entries) = fs::read_dir(dir) else {
    return out;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      out.extend(sources(&path));
    } else if path.extension().is_some_and(|e| e == "rs") {
      out.push(path);
    }
  }
  out
}

/// File content with `//`-comment lines removed, so a file may NAME the rule
/// in prose without violating it -- `dep_graph_guard.rs`'s convention, and the
/// reason this file's own header is allowed to spell the sequence out.
fn without_comments(path: &Path) -> String {
  fs::read_to_string(path)
    .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    .lines()
    .filter(|line| !line.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n")
}

#[test]
fn render_run_is_called_from_exactly_one_place_and_that_place_is_dispatch() {
  let root = workspace_root();
  let src = root.join("crates").join("intent-cli").join("src");
  let files = sources(&src);
  assert!(
    !files.is_empty(),
    "no sources were read from {}, so this test asserted nothing",
    src.display()
  );

  let mut callers: Vec<String> = Vec::new();
  for file in &files {
    // `render.rs` DEFINES `run`; calls to it are what this counts.
    if file.file_name().is_some_and(|n| n == "render.rs") {
      continue;
    }
    if without_comments(file).contains("render::run(") {
      callers.push(
        file
          .strip_prefix(&root)
          .unwrap_or(file)
          .display()
          .to_string(),
      );
    }
  }

  assert_eq!(
    callers,
    vec!["crates/intent-cli/src/lib.rs".to_string()],
    "`render::run` is called from somewhere other than `intent_cli::dispatch` \
     -- that is a second dispatch home, and it will agree with the first until \
     one of them learns something about exit codes that the other does not \
     (AC-17.15)"
  );
}

/// **AND THE HOME IS THE ONE `main` USES.** The assertion above would still
/// pass if `dispatch` existed, were the only caller, and `main` had quietly
/// grown its own copy of the sequence -- because `main` would then be calling
/// `spine::parse` and `render::run` and the second of those greps as a caller.
/// It is the case this guard is weakest against, so it is asserted directly:
/// the binary's entry point delegates rather than dispatching.
#[test]
fn main_delegates_to_dispatch_rather_than_performing_the_sequence() {
  let root = workspace_root();
  let main = root.join("crates").join("intent-cli").join("src/main.rs");
  let body = without_comments(&main);
  assert!(
    body.contains("intent_cli::dispatch("),
    "`main` does not call `intent_cli::dispatch` -- the binary and the explorer \
     no longer share an entry point (AC-17.15)"
  );
  assert!(
    !body.contains("spine::parse("),
    "`main` parses argv for itself again, which is the second dispatch home \
     arriving in the one file that used to be it (AC-17.15)"
  );
}
