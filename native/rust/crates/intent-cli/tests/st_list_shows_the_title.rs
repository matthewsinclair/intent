//! **`st list` describes a thread by its TITLE, and `--slug` renders that title
//! URI-safely. Neither reads the stored `slug` column.**
//!
//! hv, 2026-08-27, first-hand: *"the title is the SSOT. The slug is just a way
//! to show the title in an escaped URI friendly way"*, and *"with `st list` we
//! get the title, by default. If needed `st list --slug` prints the slug instead
//! of the title."*
//!
//! # Why the descriptive column changed
//!
//! It used to be the stored slug, which failed in both directions at once on the
//! estate that ships this tool: **21 of 64 threads carry no stored slug and
//! rendered as an empty cell**, and the other 43 carry v2's title-derived slug
//! truncated at 48 characters on a word boundary, so they broke off on
//! prepositions -- `...-with`, `...-before`. `WP_COLUMNS` has always said
//! `Title`; steel threads were the anomaly.
//!
//! # The arm that matters, and why the obvious fixture cannot provide it
//!
//! A fresh project cannot tell a DERIVED slug from a STORED one: `st new` stores
//! `slugify(title)`, so both readings produce identical bytes and a test built
//! that way passes under either implementation. **A fixture that cannot exhibit
//! the defect cannot clear it.**
//!
//! So `the_slug_is_derived_from_the_title_not_read_from_the_store` plants a
//! stored slug that DISAGREES with the title, through the canon extract and
//! `sync --to-store`, and requires the title's slug to win. That arm fails the
//! moment anyone reintroduces `t.slug` as the source.
//!
//! The id remains the unique identifier and the slug never resolves, so nothing
//! here asserts uniqueness -- deliberately, and see `facade::slugify`.

use std::path::Path;
use std::process::Command;

use testkit::workspace_root;

fn bin() -> std::path::PathBuf {
  workspace_root().join("target/debug/intent")
}

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = Command::new(bin())
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run intent");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

/// A title with punctuation and capitals, so the slug is visibly a TRANSFORM of
/// it rather than a copy -- a fixture whose title is already slug-shaped would
/// pass whichever column was rendered.
const TITLE: &str = "Disk as a Sparse Projection of the Store!";
const SLUG: &str = "disk-as-a-sparse-projection-of-the-store";

fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  run(dir.path(), &["init", "slugproj"]);
  run(dir.path(), &["st", "new", TITLE]);
  dir
}

#[test]
fn the_default_column_is_the_title() {
  let dir = seeded();
  let (text, code) = run(
    dir.path(),
    &["st", "list", "--status", "all", "--width", "200"],
  );
  assert_eq!(code, 0, "{text}");
  assert!(
    text.contains("Title"),
    "the descriptive column must be headed Title: {text}"
  );
  assert!(
    text.contains(TITLE),
    "the row must carry the title verbatim, punctuation and capitals included: {text}"
  );
}

#[test]
fn the_slug_flag_renders_the_title_uri_safely() {
  let dir = seeded();
  let (text, code) = run(
    dir.path(),
    &["st", "list", "--slug", "--status", "all", "--width", "200"],
  );
  assert_eq!(code, 0, "{text}");
  assert!(
    text.contains("Slug"),
    "the column must be headed Slug: {text}"
  );
  assert!(text.contains(SLUG), "expected {SLUG:?} in: {text}");
  assert!(
    !text.contains(TITLE),
    "--slug shows the slug INSTEAD of the title, not as well as: {text}"
  );
}

/// **THE ARM THAT DISCRIMINATES.**
///
/// A stored slug that disagrees with the title is planted through the canon
/// extract, in the order this estate requires (edit the extract, then
/// `sync --to-store`; a `--to-disk` would overwrite the edit). The title's slug
/// must win, because the title is the SSOT and the column is vestigial.
#[test]
fn the_slug_is_derived_from_the_title_not_read_from_the_store() {
  let dir = seeded();
  let canon = dir.path().join("intent/.canon/st/ST0001.json");
  let text = std::fs::read_to_string(&canon)
    .unwrap_or_else(|e| panic!("no canon at {}: {e}", canon.display()));
  let mut doc: serde_json::Value = serde_json::from_str(&text).expect("canon is json");

  doc["slug"] = serde_json::Value::String("stored-slug-must-not-appear".into());
  std::fs::write(
    &canon,
    serde_json::to_string_pretty(&doc).expect("reserialise"),
  )
  .expect("write canon");

  let (sync, code) = run(dir.path(), &["sync", "--to-store"]);
  assert_eq!(code, 0, "sync --to-store failed: {sync}");

  // **THE ARM CONTROLS ITSELF, because its assertion is an ABSENCE.**
  // `!contains("stored-slug-must-not-appear")` passes for free if the planted
  // value never reached the store -- if `sync --to-store` ignored the field, or
  // recomputed it, the test would prove nothing while looking exactly like
  // proof. Writing the store back out and reading the extract shows the value
  // survived the round trip, so the absence below is a decision the renderer
  // made rather than a value that was never there.
  let (back, code) = run(dir.path(), &["sync", "--to-disk"]);
  assert_eq!(code, 0, "sync --to-disk failed: {back}");
  assert!(
    std::fs::read_to_string(&canon)
      .expect("canon after round trip")
      .contains("stored-slug-must-not-appear"),
    "the planted slug did not survive into the store, so the absence asserted \
     below would be vacuous and this arm would pass under any implementation"
  );

  let (listed, code) = run(
    dir.path(),
    &["st", "list", "--slug", "--status", "all", "--width", "200"],
  );
  assert_eq!(code, 0, "{listed}");
  assert!(
    !listed.contains("stored-slug-must-not-appear"),
    "the STORED slug reached the output -- the flag is reading `t.slug` rather \
     than deriving from the title, which reintroduces the second piece of data \
     hv's ruling removes:\n{listed}"
  );
  assert!(
    listed.contains(SLUG),
    "the title's slug must be what is shown: {listed}"
  );
}
