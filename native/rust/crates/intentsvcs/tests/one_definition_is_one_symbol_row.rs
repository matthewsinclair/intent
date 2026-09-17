//! A definition written once is ONE symbol row (ST0076's measured duplicate
//! definitions, and issue 0358's `DISTINCT`).
//!
//! **THE DUPLICATE WAS MADE AT EXTRACTION, SO IT IS HELD THERE.** rust's
//! `tags.scm` names a method twice -- `(declaration_list (function_item))` as
//! `@definition.method` and `(function_item)` as `@definition.function` -- and a
//! query cursor reports both matches for the one node. Swift's query has the
//! same shape for a class's methods and properties. A `DISTINCT` at read hid it
//! from two surfaces and left every other reader of the table counting it twice.

use intentsvcs::index::symbols::{Symbol, SymbolKind, symbols_of};

fn defs_named<'a>(rows: &'a [Symbol], name: &str) -> Vec<&'a Symbol> {
  rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Def && s.name == name)
    .collect()
}

#[test]
fn a_rust_method_defined_once_is_one_definition() {
  let rows = symbols_of(
    "rust",
    "src/address.rs",
    b"pub struct Address;\n\nimpl Address {\n  pub fn is_local(&self) -> bool {\n    true\n  }\n}\n",
  )
  .expect("the rust grammar is in the default build");
  assert_eq!(
    defs_named(&rows, "is_local").len(),
    1,
    "one method, one definition row: {rows:#?}"
  );
}

#[test]
fn a_swift_method_defined_once_is_one_definition() {
  let rows = symbols_of(
    "swift",
    "Sources/Address.swift",
    b"class Address {\n  func isLocal() -> Bool {\n    return true\n  }\n}\n",
  )
  .expect("the swift grammar is in the default build");
  assert_eq!(
    defs_named(&rows, "isLocal").len(),
    1,
    "one method, one definition row: {rows:#?}"
  );
}
