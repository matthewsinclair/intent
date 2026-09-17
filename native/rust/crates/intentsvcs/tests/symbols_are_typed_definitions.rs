//! ST0076 WP-01: a definition carries what it IS, where it sits and its arity,
//! and a store an older extractor wrote is re-extracted rather than mixed.

use crate::common::Fixture;
use intentsvcs::index::symbols::{EXTRACTOR_VERSION, Symbol, SymbolKind, symbols_of};

/// One definition, as a tuple a failure message reads at a glance.
type Typed<'a> = (
  &'a str,
  &'a str,
  Option<&'a str>,
  Option<&'a str>,
  Option<&'a str>,
  Option<u32>,
  Option<u32>,
);

fn typed(rows: &[Symbol]) -> Vec<Typed<'_>> {
  rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Def)
    .map(|s| {
      (
        s.name.as_str(),
        s.subkind.as_str(),
        s.container.as_deref(),
        s.container_kind.as_deref(),
        s.trait_name.as_deref(),
        s.arity,
        s.arity_min,
      )
    })
    .collect()
}

#[test]
fn rust_definitions_carry_their_kind_container_and_arity() {
  let src = b"mod store {
  pub struct Address {
    host: String,
  }
  pub enum AddressError {
    Refused { code: u16 },
  }
  impl Address {
    pub fn is_local(&self) -> bool { true }
    pub fn new(host: String, #[allow(unused)] port: u16) -> Self { todo!() }
  }
  impl<T> Resolve for Address {
    fn resolve(&self, with: T) {}
  }
  pub trait Resolve {
    fn resolve(&self, with: u8);
  }
  pub const LIMIT: u8 = 1;
  static SEEN: u8 = 0;
  pub type Port = u16;
  macro_rules! here { () => {} }
  mod inner {
    fn helper(a: u8, b: u8) {}
  }
}
fn top() {}
";
  let rows =
    symbols_of("rust", "src/store.rs", src).expect("the rust grammar is in the default build");
  let got = typed(&rows);
  let expected: Vec<Typed<'_>> = vec![
    ("store", "module", None, None, None, None, None),
    (
      "Address",
      "struct",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "host",
      "field",
      Some("Address"),
      Some("struct"),
      None,
      None,
      None,
    ),
    (
      "AddressError",
      "enum",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "Refused",
      "variant",
      Some("AddressError"),
      Some("enum"),
      None,
      None,
      None,
    ),
    (
      "code",
      "field",
      Some("Refused"),
      Some("variant"),
      None,
      None,
      None,
    ),
    (
      "is_local",
      "method",
      Some("Address"),
      Some("impl"),
      None,
      Some(0),
      Some(0),
    ),
    (
      "new",
      "assoc_fn",
      Some("Address"),
      Some("impl"),
      None,
      Some(2),
      Some(2),
    ),
    (
      "resolve",
      "method",
      Some("Address"),
      Some("impl"),
      Some("Resolve"),
      Some(1),
      Some(1),
    ),
    (
      "Resolve",
      "trait",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "resolve",
      "method",
      Some("Resolve"),
      Some("trait"),
      None,
      Some(1),
      Some(1),
    ),
    (
      "LIMIT",
      "const",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "SEEN",
      "static",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "Port",
      "type",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "here",
      "macro",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "inner",
      "module",
      Some("store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "helper",
      "function",
      Some("store::inner"),
      Some("module"),
      None,
      Some(2),
      Some(2),
    ),
    ("top", "function", None, None, None, Some(0), Some(0)),
  ];
  assert_eq!(got, expected, "{rows:#?}");
  assert!(rows.iter().all(|s| s.level == 1 && s.qualifier.is_none()));
}

#[test]
fn elixir_definitions_keep_def_defp_and_defmacro_apart() {
  let src = b"defmodule Intent.Store do
  defmodule Rows do
    defstruct [:path]
    def open(path, mode \\\\ :read) when is_binary(path), do: {path, mode}
    def open(nil), do: nil
    defp count, do: 0
  end
  defmacro trace(expr) do
    expr
  end
  defguard is_row(term) when is_map(term)
  defdelegate fetch(key), to: Rows
end
";
  let rows = symbols_of("elixir", "lib/intent/store.ex", src)
    .expect("the elixir grammar is in the default build");
  let got = typed(&rows);
  let expected: Vec<Typed<'_>> = vec![
    ("Intent.Store", "module", None, None, None, None, None),
    (
      "Rows",
      "module",
      Some("Intent.Store"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "Intent.Store.Rows",
      "struct",
      Some("Intent.Store.Rows"),
      Some("module"),
      None,
      None,
      None,
    ),
    (
      "open",
      "def",
      Some("Intent.Store.Rows"),
      Some("module"),
      None,
      Some(2),
      Some(1),
    ),
    (
      "open",
      "def",
      Some("Intent.Store.Rows"),
      Some("module"),
      None,
      Some(1),
      Some(1),
    ),
    (
      "count",
      "defp",
      Some("Intent.Store.Rows"),
      Some("module"),
      None,
      Some(0),
      Some(0),
    ),
    (
      "trace",
      "defmacro",
      Some("Intent.Store"),
      Some("module"),
      None,
      Some(1),
      Some(1),
    ),
    (
      "is_row",
      "defguard",
      Some("Intent.Store"),
      Some("module"),
      None,
      Some(1),
      Some(1),
    ),
    (
      "fetch",
      "defdelegate",
      Some("Intent.Store"),
      Some("module"),
      None,
      Some(1),
      Some(1),
    ),
  ];
  assert_eq!(got, expected, "{rows:#?}");
  let refs: Vec<&str> = rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Ref)
    .map(|s| s.name.as_str())
    .collect();
  for syntax in [
    "defmodule",
    "def",
    "defp",
    "defstruct",
    "defmacro",
    "defguard",
    "defdelegate",
  ] {
    assert!(
      !refs.contains(&syntax),
      "`{syntax}` is syntax, not a reference: {refs:?}"
    );
  }
}

/// **A FILE AN OLDER EXTRACTOR WROTE IS RE-EXTRACTED BY THE NEXT RECONCILE.**
/// Its symbols are wiped to the pre-rung shape and its version cleared, which
/// is what a store migrated to rung 28 holds.
#[test]
fn a_file_an_older_extractor_wrote_is_extracted_again() {
  let fx = Fixture::new();
  std::fs::create_dir_all(fx.root().join("src")).expect("src");
  std::fs::write(
    fx.root().join("src/lib.rs"),
    "pub struct Address;\nimpl Address {\n  pub fn is_local(&self) -> bool { true }\n}\n",
  )
  .expect("lib");
  let mut f = fx.facade_on_disk();
  f.index_rebuild().expect("the index builds");
  let db = || rusqlite::Connection::open(fx.project().db_path()).expect("store");
  let version = || -> Option<i64> {
    db()
      .query_row(
        "SELECT symbols_version FROM index_file WHERE path = 'src/lib.rs'",
        [],
        |r| r.get(0),
      )
      .expect("the file is indexed")
  };
  assert_eq!(version(), Some(EXTRACTOR_VERSION));

  db()
    .execute_batch(
      "UPDATE index_file SET symbols_version = NULL WHERE path = 'src/lib.rs';
       UPDATE symbols SET subkind = '' WHERE path = 'src/lib.rs';",
    )
    .expect("age the file");
  f.index_refresh(None).expect("the reconcile runs");

  assert_eq!(
    version(),
    Some(EXTRACTOR_VERSION),
    "the file was not re-extracted"
  );
  let untyped: i64 = db()
    .query_row(
      "SELECT count(*) FROM symbols WHERE path = 'src/lib.rs' AND subkind = ''",
      [],
      |r| r.get(0),
    )
    .expect("count");
  assert_eq!(untyped, 0, "rows of the old shape survived beside the new");
}

/// **A DEFINITION'S OWN NAME IS NEVER A REFERENCE TO IT.** A `def` head with
/// parentheses has the shape of a call and a module's name is an alias, so a
/// broad reference pattern matches both; left in, every Elixir function and
/// module reads as referenced where it is defined. `defimpl`'s `for:` target
/// and an alias in a body are still references.
#[test]
fn an_elixir_definition_is_never_a_reference_to_itself() {
  let src = b"defmodule Intent.Store do
  def open(path) when is_binary(path), do: helper(path)
  defp helper(x), do: x
end
defimpl Inspect, for: Intent.Store do
  def inspect(store, _opts), do: Intent.Store.open(store)
end
";
  let rows =
    symbols_of("elixir", "lib/store.ex", src).expect("the elixir grammar is in the default build");
  let at = |kind: SymbolKind| -> Vec<(&str, u32)> {
    rows
      .iter()
      .filter(|s| s.kind == kind)
      .map(|s| (s.name.as_str(), s.span.start_line))
      .collect()
  };
  let (defs, refs) = (at(SymbolKind::Def), at(SymbolKind::Ref));
  let own: Vec<_> = refs
    .iter()
    .filter(|(name, line)| defs.iter().any(|(d, l)| d == name && l == line))
    .collect();
  assert!(
    own.is_empty(),
    "definitions read as references: {own:?}\n{rows:#?}"
  );
  for written in [("helper", 2), ("Intent.Store", 5), ("open", 6)] {
    assert!(
      refs.contains(&written),
      "{written:?} is a reference: {refs:?}"
    );
  }
}

/// **A REFERENCE SPANS WHAT WAS WRITTEN.** An impl names its type and trait on
/// one line, and every surface prints a reference's span.
#[test]
fn an_impl_references_its_type_and_trait_on_the_line_it_names_them() {
  let src = b"impl Resolve
  for Address
{
  fn resolve(&self) {}
}
";
  let rows =
    symbols_of("rust", "src/lib.rs", src).expect("the rust grammar is in the default build");
  let refs: Vec<(&str, u32, u32)> = rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Ref)
    .map(|s| (s.name.as_str(), s.span.start_line, s.span.end_line))
    .collect();
  for written in [("Resolve", 1, 1), ("Address", 2, 2)] {
    assert!(refs.contains(&written), "{written:?} missing from {refs:?}");
  }
}
