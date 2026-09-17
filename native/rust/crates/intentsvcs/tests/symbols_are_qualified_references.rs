//! AT-02.1 / AC-02.1 and AT-02.2 / AC-02.2 (ST0076 WP-02): a reference carries
//! the path it was written with, a reference inside a macro invocation is a
//! reference, and every reference spans the name it wrote.

use intentsvcs::index::symbols::{Symbol, SymbolKind, symbols_of};

/// One reference, as a tuple a failure message reads at a glance.
type Written<'a> = (&'a str, &'a str, Option<&'a str>, u8, u32);

fn refs(rows: &[Symbol]) -> Vec<Written<'_>> {
  rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Ref)
    .map(|s| {
      (
        s.name.as_str(),
        s.subkind.as_str(),
        s.qualifier.as_deref(),
        s.level,
        s.span.start_line,
      )
    })
    .collect()
}

fn rust(src: &[u8]) -> Vec<Symbol> {
  symbols_of("rust", "src/lib.rs", src).expect("the rust grammar is in the default build")
}

#[test]
fn a_scoped_call_keeps_the_path_it_was_written_with() {
  let rows = rust(
    b"fn f() {
  let a = AddressError::new(1);
  let b = crate::views::render(a);
  let c = Vec::<u8>::with_capacity(2);
  plain(c);
}
",
  );
  let got = refs(&rows);
  for expected in [
    ("new", "call", Some("AddressError"), 2, 2),
    ("render", "call", Some("crate::views"), 2, 3),
    ("with_capacity", "call", Some("Vec::<u8>"), 2, 4),
    ("plain", "call", None, 1, 5),
  ] {
    assert!(
      got.contains(&expected),
      "{expected:?} missing from {got:#?}"
    );
  }
}

/// **ISSUE 0429: A REFERENCE INSIDE A MACRO INVOCATION IS A REFERENCE.** A
/// macro's arguments are an unstructured token tree, so the shape of the tokens
/// is all there is: a name before `(` is a call, after `.` a method call, after
/// `::` a path with that one segment as its qualifier, and any other identifier
/// is a `token` name match.
#[test]
fn a_reference_inside_a_macro_invocation_is_a_reference() {
  let rows = rust(
    b"#[test]
fn nearest() {
  assert_eq!(nearest_project(root), Some(addr.is_local()));
  let s = format!(\"{}\", views::render(&addr));
  assert!(matches!(e, AddressError::Refused { .. }));
}
",
  );
  let got = refs(&rows);
  for expected in [
    ("assert_eq", "macro", None, 1, 3),
    ("nearest_project", "call", None, 1, 3),
    ("root", "token", None, 1, 3),
    ("Some", "call", None, 1, 3),
    ("addr", "token", None, 1, 3),
    ("is_local", "call", None, 1, 3),
    ("render", "call", Some("views"), 2, 4),
    ("matches", "macro", None, 1, 5),
    ("Refused", "path", Some("AddressError"), 2, 5),
  ] {
    assert!(
      got.contains(&expected),
      "{expected:?} missing from {got:#?}"
    );
  }
  assert!(
    !got
      .iter()
      .any(|(name, sub, ..)| *name == "views" && *sub == "token"),
    "a qualifier is not also a token: {got:#?}"
  );
}

/// **A TYPE IS REFERENCED WHEREVER IT IS WRITTEN**, and a path keeps its
/// qualifier: in a signature, a body, an impl, a `use`, and a path that is not
/// called. The name a type is declared with is its definition, never a use.
#[test]
fn a_type_use_and_a_path_are_references_with_their_qualifier() {
  let rows = rust(
    b"use crate::store::{Address, Port};
use std::fmt::Display;
struct Wrapper<T: Display>(Vec<T>);
impl Resolve for Wrapper<u8> {}
fn f(a: store::Address) -> AddressError {
  let e = AddressError::Refused;
  e
}
",
  );
  let got = refs(&rows);
  for expected in [
    ("Address", "use", Some("crate::store"), 2, 1),
    ("Port", "use", Some("crate::store"), 2, 1),
    ("Display", "use", Some("std::fmt"), 2, 2),
    ("Display", "type", None, 1, 3),
    ("Vec", "type", None, 1, 3),
    ("T", "type", None, 1, 3),
    ("Resolve", "type", None, 1, 4),
    ("Wrapper", "type", None, 1, 4),
    ("Address", "type", Some("store"), 2, 5),
    ("AddressError", "type", None, 1, 5),
    ("Refused", "path", Some("AddressError"), 2, 6),
  ] {
    assert!(
      got.contains(&expected),
      "{expected:?} missing from {got:#?}"
    );
  }
  let wrapper_on_3: Vec<_> = got
    .iter()
    .filter(|(name, _, _, _, line)| *name == "Wrapper" && *line == 3)
    .collect();
  assert!(
    wrapper_on_3.is_empty(),
    "a declared name is not a use: {wrapper_on_3:?}"
  );
  for segment in ["crate", "store", "std", "fmt"] {
    assert!(
      !got.iter().any(|(name, ..)| *name == segment),
      "`{segment}` is a path segment, not a reference: {got:#?}"
    );
  }
}

/// **A REFERENCE SPANS THE NAME IT WROTE**, not the call around it.
#[test]
fn a_call_with_arguments_over_many_lines_spans_its_name() {
  let rows = rust(
    b"fn f() {
  render(
    a,
    b,
  );
}
",
  );
  let render: Vec<_> = rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Ref && s.name == "render")
    .map(|s| (s.span.start_line, s.span.end_line))
    .collect();
  assert_eq!(render, vec![(2, 2)]);
}

/// **THE TYPE OR MODULE A QUALIFIER ENDS IN IS ITSELF A REFERENCE.** A struct
/// used only as `Foo::new()` is used, and the segments before the last one are
/// part of its path, not references of their own.
#[test]
fn the_last_segment_of_a_qualifier_is_a_reference() {
  let rows = rust(
    b"fn f() {
  let a = Foo::new();
  let b = crate::store::Address::new();
  let c = std::fmt::Error;
}
",
  );
  let got = refs(&rows);
  for expected in [
    ("Foo", "path", None, 1, 2),
    ("new", "call", Some("Foo"), 2, 2),
    ("Address", "path", Some("crate::store"), 2, 3),
    ("new", "call", Some("crate::store::Address"), 2, 3),
    ("fmt", "path", Some("std"), 2, 4),
    ("Error", "path", Some("std::fmt"), 2, 4),
  ] {
    assert!(
      got.contains(&expected),
      "{expected:?} missing from {got:#?}"
    );
  }
  for segment in ["crate", "store", "std"] {
    assert!(
      !got.iter().any(|(name, ..)| *name == segment),
      "`{segment}` is a path segment, not a reference: {got:#?}"
    );
  }
}

/// The same inside a macro, where the tokens show one segment at a time.
#[test]
fn the_type_a_path_inside_a_macro_is_qualified_by_is_a_reference() {
  let rows = rust(
    b"fn f() {
  assert!(matches!(e, AddressError::Refused { .. }));
}
",
  );
  let got = refs(&rows);
  for expected in [
    ("AddressError", "path", None, 1, 2),
    ("Refused", "path", Some("AddressError"), 2, 2),
  ] {
    assert!(
      got.contains(&expected),
      "{expected:?} missing from {got:#?}"
    );
  }
}
