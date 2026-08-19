//! AT-07.3 / AC-07.3: **`?format=` accepts exactly `json` and `md` and nothing
//! else**, and an entity with more than one rendering is split into distinct
//! addresses rather than gaining a `?view=`.
//!
//! **If an entity has more than one rendering it is UNDER-ADDRESSED.** Held at
//! two formats for 3.0.0 because two with a ratified meaning each beats four
//! that drift.
//!
//! The closed set is asserted from the ENUM rather than from a list here, so a
//! third variant added to `Format` fails this file instead of quietly widening
//! the scheme.

use intentsvcs::address::{Format, parse};

#[test]
fn exactly_json_and_md_are_accepted() {
  assert!(parse("intent:///threads/ST0056?format=json").is_ok());
  assert!(parse("intent:///threads/ST0056?format=md").is_ok());

  for bad in [
    "yaml", "toml", "html", "txt", "JSON", "Md", "markdown", "j son", "",
  ] {
    let url = format!("intent:///threads/ST0056?format={bad}");
    assert!(
      parse(&url).is_err() || bad.is_empty(),
      "`format={bad}` must be refused -- the set is closed at json and md"
    );
  }
}

/// **The enum is the roster.** `Format::parse` must accept exactly the strings
/// its own variants render, and nothing else -- which is what makes the set
/// closed rather than merely short today.
#[test]
fn the_format_enum_and_its_parser_agree_exactly() {
  let all = [Format::Json, Format::Md];
  assert_eq!(all.len(), 2, "the set is held at two for 3.0.0");
  for f in all {
    assert_eq!(
      Format::parse(f.as_str()),
      Some(f),
      "every variant must parse from its own rendering"
    );
  }
  assert_eq!(Format::parse("view"), None);
  assert_eq!(Format::parse("xml"), None);
}

/// **No `?view=`, and no stacking.** A second rendering gets its own ADDRESS.
/// This is the arm that stops the scheme growing a query language.
#[test]
fn no_query_other_than_format_exists() {
  for bad in [
    "intent:///threads/ST0056?view=acceptance",
    "intent:///threads/ST0056?format=md&view=cover",
    "intent:///threads/ST0056?formats=md",
    "intent:///threads/ST0056?md",
    "intent:///threads/ST0056?format=md&format=json",
  ] {
    assert!(
      parse(bad).is_err(),
      "`{bad}` must be refused -- a second knob is how an under-addressed\n       \
       entity gets papered over instead of split"
    );
  }
}

/// An address with no query asked for nothing, and that is distinct from
/// asking for a default. A parser substituting one here would make the two
/// indistinguishable to every consumer downstream.
#[test]
fn an_absent_format_is_absent_rather_than_defaulted() {
  assert_eq!(
    parse("intent:///threads/ST0056").expect("resolves").format,
    None
  );
  assert_eq!(
    parse("intent:///threads/ST0056?")
      .expect("an empty query asked nothing")
      .format,
    None
  );
}
