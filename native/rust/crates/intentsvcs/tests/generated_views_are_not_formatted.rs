//! ST0056 AC-03.17: **a generated view must be a fixed point of the project's
//! formatter**, and the only way to guarantee that for authored prose is to
//! keep the formatter off the file.
//!
//! # Why not the renderer
//!
//! The criterion's first-stated remedy was to make the renderer emit what the
//! formatter would leave alone. **That was tried and the population refused
//! it.** Over 6,095 authored lines the conservative rule -- rewrite matched
//! single-asterisk emphasis, collapse interior double spaces, both outside
//! inline code -- produced EIGHT changes where the formatter makes SEVEN. The
//! extra was ST0056 AC-10.9, whose prose runs backticks straight into words
//! (`` yet`followed by`commit ``) so the real parser's code-span boundaries
//! fall somewhere a hand-rolled segmenter cannot see. A renderer-side rule has
//! to model CommonMark's inline parsing to be safe, **and getting it wrong is
//! invisible: it rewrites a site the formatter leaves alone, which is the same
//! churn loop with the arrow reversed.**
//!
//! `view_determinism.rs` already recorded the constraint that settles it:
//! authored prose passes through the renderer VERBATIM (migration.md forbids
//! reflowing or improving it), so a renderer that normalised markup would be
//! doing the one thing it is forbidden to do.
//!
//! # One writer per file
//!
//! So the fix is structural and it is D02 applied to WRITERS rather than to
//! content: a tool-owned file has one writer, and for a generated view that
//! writer is the renderer. hv ruled it 2026-08-19 -- exclude the generated set
//! and keep formatting everything a human authors.
//!
//! # The roster is asked of the generator
//!
//! The set of paths checked here is [`views::render_all`]'s own output, never
//! a list kept in this file. A hand-kept roster is one someone must remember
//! to extend on the day they add a view, which is the day they are thinking
//! about something else -- the argument `openness.rs` makes for enumerating
//! tables from the DDL, and `no_view_claims_to_be_truth.rs` makes for its
//! artefact roster.
//!
//! # The oracle is prettier's own resolution
//!
//! Ignore status is read from `prettier --file-info`, so this test agrees with
//! the formatter by construction rather than by a matcher reimplementing
//! gitignore semantics here. The hook runs the same binary over the same tree.
//!
//! # The control is what stops `*` passing
//!
//! A `.prettierignore` of `*` would satisfy the assertion above completely.
//! So an AUTHORED file **in the same directory as the generated views** must
//! still come back formattable: that proves the rule discriminates by file
//! rather than by tree, which is the whole point of one-writer-per-file.

mod common;

use common::ctx;
use intentsvcs::project::Project;
use intentsvcs::{ingest, views};
use std::path::Path;
use testkit::repo_root;

/// Ask prettier whether it would format this path. `None` when prettier
/// cannot answer at all.
fn ignored_by_prettier(root: &Path, rel: &str) -> Option<bool> {
  let out = std::process::Command::new("prettier")
    .arg("--file-info")
    .arg(rel)
    .current_dir(root)
    .output()
    .ok()?;
  if !out.status.success() {
    return None;
  }
  let text = String::from_utf8_lossy(&out.stdout);
  // `{ "ignored": false, "inferredParser": "markdown" }` -- read the field
  // rather than the whole shape, so a future prettier adding a key does not
  // break this.
  let at = text.find("\"ignored\"")?;
  let rest = &text[at + "\"ignored\"".len()..];
  Some(
    rest
      .trim_start()
      .trim_start_matches(':')
      .trim_start()
      .starts_with("true"),
  )
}

#[test]
fn every_generated_view_is_excluded_from_the_formatter() {
  if std::process::Command::new("prettier")
    .arg("--version")
    .output()
    .is_err()
  {
    eprintln!(
      "SKIPPED AC-03.17: prettier is not on PATH, so its own ignore resolution cannot be asked. \
       This is the loud skip -- a quiet pass on a missing binary is the vacuous green this \
       project keeps paying for."
    );
    return;
  }

  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let canon = ingest::read(&project).expect("canon reads from the real estate");
  let rendered = views::render_all(&project, &canon, &ctx());

  assert!(
    !rendered.is_empty(),
    "precondition: the generator emitted no views, so this probe's population \
     cannot contain the failure it tests for"
  );

  let mut formatted = Vec::new();
  let mut unanswerable = Vec::new();
  for view in &rendered {
    let rel = view
      .path
      .strip_prefix(&root)
      .unwrap_or(&view.path)
      .to_string_lossy()
      .into_owned();
    match ignored_by_prettier(&root, &rel) {
      Some(true) => {}
      Some(false) => formatted.push(rel),
      None => unanswerable.push(rel),
    }
  }

  assert!(
    unanswerable.is_empty(),
    "prettier could not answer for {} path(s), so their status is unknown rather than good: {:?}",
    unanswerable.len(),
    unanswerable
  );

  assert!(
    formatted.is_empty(),
    "{} of {} generated view(s) are still formatted at commit time, so the renderer is not the \
     only writer of them. The formatter rewrites what the renderer just wrote, the skew check \
     then reports drift on files nobody edited, and regenerating restores the renderer's bytes \
     -- forever, with every pass looking like a legitimate repair:\n  {}",
    formatted.len(),
    rendered.len(),
    formatted.join("\n  ")
  );
}

#[test]
fn an_authored_file_beside_the_views_is_still_formatted() {
  if std::process::Command::new("prettier")
    .arg("--version")
    .output()
    .is_err()
  {
    eprintln!("SKIPPED the control arm of AC-03.17: prettier is not on PATH.");
    return;
  }

  let root = repo_root();
  let control = "intent/st/ST0056/design.md";
  assert!(
    root.join(control).exists(),
    "the control file {control} does not exist, so this arm proves nothing -- pick another \
     AUTHORED file that sits in the same directory as the generated views"
  );

  assert_eq!(
    ignored_by_prettier(&root, control),
    Some(false),
    "{control} is authored by a human and must still be formatted. It is ignored, which means \
     the exclusion is scoped to a TREE rather than to the generated set -- and a rule broad \
     enough to cover everything satisfies the other arm of this test completely while giving \
     up the formatting on every hand-written file in it."
  );
}
