//! Issue `0713` (ST0095 WP-10): **a door that reads an id reads it through the
//! waist, so a short spelling names the same record everywhere.** `todo done`
//! handed its specifier to `scope_of` raw, so `todo done 56/02` named "no steel
//! thread 56" where `wp done 56/02` works; `ac descope --to` compared its
//! target raw, so `--to 59` was "not a steel thread" where every other door
//! reads 59 as ST0059. (`events --subject`, the issue's third door, reads an
//! untagged number as every record it could name, by cc's meaning of
//! 2026-11-20, and is driven where that lands.)
//!
//! The short spellings are derived from the minted ids: the tagged one from the
//! waist (`ThreadId::tagged`), and the bare number and the unpadded package by
//! trimming the waist's own outputs, never from a literal.

use crate::common::{created, ok, run};
use intentsvcs::id::{ThreadId, WpId};

/// A minted thread's bare number, eg `1` for `ST0001`: its tagged spelling
/// with the tag trimmed off.
fn bare(stored: &str) -> String {
  let thread = ThreadId::parse(stored).expect("a minted thread id");
  thread
    .tagged()
    .trim_start_matches(char::is_alphabetic)
    .to_string()
}

#[test]
fn todo_done_reaches_the_package_a_short_spelling_names() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let st = created(&ok(root, &["st", "new", "A thread the doors name"]));
  let wp = created(&ok(
    root,
    &["wp", "new", st.as_str(), "A package the doors name"],
  ));
  let (_, package) = intentsvcs::id::split_wp_address(&wp).expect("a package address");
  let package = WpId::parse(package).expect("a minted package id");
  let short_wp = format!("{}/{}", bare(&st), package.shown().trim_start_matches('0'));

  // The package is not started, so `wp.done` is refused by the transition
  // machine, and the refusal names the package it read: before the fix it
  // named "no steel thread" by the short spelling's head.
  let (out, err, code) = run(root, &["todo", "done", short_wp.as_str()]);
  assert_eq!(code, 1, "`todo done {short_wp}`:\n{out}{err}");
  assert!(
    err.contains(&format!(
      "`wp.done` is not a legal transition for {wp}, which is `not-started`"
    )),
    "`todo done {short_wp}` reaches the package `{wp}`:\n{out}{err}"
  );
}

#[test]
fn ac_descope_takes_a_bare_thread_number() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let from = created(&ok(root, &["st", "new", "The thread a requirement leaves"]));
  let to = created(&ok(root, &["st", "new", "The thread that will hold it"]));
  ok(
    root,
    &[
      "ac",
      "new",
      from.as_str(),
      "AC-01.1",
      "--text",
      "a requirement that moves",
    ],
  );

  let out = ok(
    root,
    &[
      "ac",
      "descope",
      from.as_str(),
      "AC-01.1",
      "--to",
      bare(&to).as_str(),
    ],
  );
  assert!(
    out.contains(&format!("descoped to {to}")),
    "`--to {}` names the thread `{to}`, as at every other door:\n{out}",
    bare(&to)
  );
}
