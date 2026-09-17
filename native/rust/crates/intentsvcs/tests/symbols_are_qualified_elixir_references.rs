//! AT-03.1 / AC-03.1 (ST0076 WP-03): an Elixir reference carries the module it
//! was written with, expanded by the file's own aliases and `__MODULE__` within
//! the enclosing do-block, and a directive names the whole module it brings in.

use intentsvcs::index::symbols::{Symbol, SymbolKind, symbols_of};

/// One reference, as a tuple a failure message reads at a glance: name,
/// subkind, qualifier, level, line, arity.
type Written<'a> = (&'a str, &'a str, Option<&'a str>, u8, u32, Option<u32>);

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
        s.arity,
      )
    })
    .collect()
}

fn elixir(src: &[u8]) -> Vec<Symbol> {
  symbols_of("elixir", "lib/shop/orders.ex", src)
    .expect("the elixir grammar is in the default build")
}

fn has_all(got: &[Written<'_>], want: &[Written<'_>]) {
  for expected in want {
    assert!(got.contains(expected), "{expected:?} missing from {got:#?}");
  }
}

#[test]
fn a_remote_call_keeps_the_module_it_was_written_with_and_its_arity() {
  let rows = elixir(
    b"defmodule Shop.Orders do
  def fetch(id) do
    order = Repo.get(Order, id)
    Map.get(order, :total, 0)
    Access.get(order, :total)
    :ets.lookup(:orders, id)
    Repo.all()
    local(order)
  end
end
",
  );
  let got = refs(&rows);
  has_all(
    &got,
    &[
      ("get", "call", Some("Repo"), 2, 3, Some(2)),
      ("get", "call", Some("Map"), 2, 4, Some(3)),
      ("get", "call", Some("Access"), 2, 5, Some(2)),
      ("lookup", "call", Some(":ets"), 2, 6, Some(2)),
      ("all", "call", Some("Repo"), 2, 7, Some(0)),
      ("local", "call", None, 1, 8, None),
    ],
  );
  assert_eq!(
    got.iter().filter(|r| r.0 == "get").count(),
    3,
    "each written `get` is one row, never a level-1 duplicate: {got:#?}"
  );
}

#[test]
fn a_remote_capture_and_a_remote_pipe_target_carry_their_module_and_arity() {
  let rows = elixir(
    b"defmodule Shop.Orders do
  def totals(orders) do
    orders
    |> Enum.map(&Shop.Format.price/1)
    |> Map.get(:total)
    |> :erlang.float_to_binary(decimals: 2)
    |> round_up()
    |> finish
  end
end
",
  );
  has_all(
    &refs(&rows),
    &[
      ("price", "call", Some("Shop.Format"), 2, 4, Some(1)),
      ("map", "call", Some("Enum"), 2, 4, Some(2)),
      ("get", "call", Some("Map"), 2, 5, Some(2)),
      ("float_to_binary", "call", Some(":erlang"), 2, 6, Some(2)),
      ("round_up", "call", None, 1, 7, None),
      ("finish", "call", None, 1, 8, None),
    ],
  );
}

#[test]
fn a_local_capture_is_an_unqualified_reference_with_its_arity() {
  let rows = elixir(
    b"defmodule Shop.Orders do
  def totals(orders) do
    Enum.map(orders, &price/1)
  end
  defp price(order), do: order.total
end
",
  );
  has_all(&refs(&rows), &[("price", "call", None, 1, 3, Some(1))]);
}

#[test]
fn an_alias_and_the_self_reference_expand_a_qualifier_within_their_do_block() {
  let rows = elixir(
    b"defmodule Shop.Orders do
  alias :crypto, as: Crypto
  def a(id) do
    alias Shop.Repo, as: R
    alias Shop.Accounts
    R.get(id)
    Accounts.User.fetch(id)
    __MODULE__.b(id)
  end
  def b(id) do
    R.get(id)
    Accounts.User.fetch(id)
    Crypto.hash(:sha256, id)
  end
end
",
  );
  let got = refs(&rows);
  has_all(
    &got,
    &[
      (":crypto", "alias", None, 1, 2, None),
      ("Shop.Repo", "alias", None, 1, 4, None),
      ("get", "call", Some("Shop.Repo"), 2, 6, Some(1)),
      ("fetch", "call", Some("Shop.Accounts.User"), 2, 7, Some(1)),
      ("b", "call", Some("Shop.Orders"), 2, 8, Some(1)),
      // Past the end of `def a`'s do-block, its aliases no longer apply.
      ("get", "call", Some("R"), 2, 11, Some(1)),
      ("fetch", "call", Some("Accounts.User"), 2, 12, Some(1)),
      ("hash", "call", Some(":crypto"), 2, 13, Some(2)),
    ],
  );
  assert!(
    !got
      .iter()
      .any(|r| (r.0, r.4) == ("Crypto", 2) || (r.0, r.4) == ("R", 4)),
    "the short name an `as:` gives is not a module reference: {got:#?}"
  );
}

#[test]
fn a_directive_name_is_expanded_and_import_and_use_apply_nothing() {
  let rows = elixir(
    b"defmodule Shop.Web do
  alias Shop.Accounts
  import Accounts.Helpers
  use Accounts.Schema
  require Accounts.Audit
  def show(conn) do
    format_date(conn)
    Helpers.render(conn)
  end
end
",
  );
  has_all(
    &refs(&rows),
    &[
      ("Shop.Accounts", "alias", None, 1, 2, None),
      ("Shop.Accounts.Helpers", "import", None, 1, 3, None),
      ("Shop.Accounts.Schema", "use", None, 1, 4, None),
      ("Shop.Accounts.Audit", "require", None, 1, 5, None),
      ("format_date", "call", None, 1, 7, None),
      ("render", "call", Some("Helpers"), 2, 8, Some(1)),
    ],
  );
}

#[test]
fn a_directive_names_the_module_it_brings_in() {
  let rows = elixir(
    b"defmodule Shop.Orders do
  use Ecto.Schema
  import Ecto.Query
  require Logger
  alias Shop.Repo
end
",
  );
  let got = refs(&rows);
  has_all(
    &got,
    &[
      ("Ecto.Schema", "use", None, 1, 2, None),
      ("Ecto.Query", "import", None, 1, 3, None),
      ("Logger", "require", None, 1, 4, None),
      ("Shop.Repo", "alias", None, 1, 5, None),
    ],
  );
  assert!(
    !got
      .iter()
      .any(|r| r.1 == "module" && [2, 3, 4, 5].contains(&r.4)),
    "a directive's module is one row, with the directive as its subkind: {got:#?}"
  );
}

#[test]
fn a_brace_directive_names_each_entry_whole() {
  let rows = elixir(
    b"defmodule Shop.Web do
  alias Shop.{Repo, Mailer}
  alias __MODULE__.{Router, Endpoint}
  require Shop.{Audit, Metrics}
  def deliver(order) do
    Mailer.send(order)
    Router.path(order)
  end
end
",
  );
  let got = refs(&rows);
  has_all(
    &got,
    &[
      ("Shop.Repo", "alias", None, 1, 2, None),
      ("Shop.Mailer", "alias", None, 1, 2, None),
      ("Shop.Web.Router", "alias", None, 1, 3, None),
      ("Shop.Web.Endpoint", "alias", None, 1, 3, None),
      ("Shop.Audit", "require", None, 1, 4, None),
      ("Shop.Metrics", "require", None, 1, 4, None),
      ("send", "call", Some("Shop.Mailer"), 2, 6, Some(1)),
      ("path", "call", Some("Shop.Web.Router"), 2, 7, Some(1)),
    ],
  );
  assert!(
    !got
      .iter()
      .any(|r| r.1 == "module" && [2, 3, 4].contains(&r.4)),
    "a brace directive's base and entries are its rows, not module references: {got:#?}"
  );
}

#[test]
fn an_earlier_alias_does_not_rename_a_brace_entry() {
  let rows = elixir(
    b"defmodule Shop.Web do
  alias Other.Repo
  alias Shop.{Repo, Mailer}
  def list do
    Repo.all()
  end
end
",
  );
  let got = refs(&rows);
  has_all(
    &got,
    &[
      ("Shop.Repo", "alias", None, 1, 3, None),
      ("all", "call", Some("Shop.Repo"), 2, 5, Some(0)),
    ],
  );
  assert!(
    !got.iter().any(|r| r.0 == "Other.Repo" && r.4 == 3),
    "the entry is named from its own base: {got:#?}"
  );
}

#[test]
fn a_self_directive_is_named_from_the_enclosing_module() {
  let rows = elixir(
    b"defmodule Shop.Web do
  alias __MODULE__.Sub
  import __MODULE__.Helpers
  def go do
    Sub.run()
  end
end
",
  );
  let got = refs(&rows);
  has_all(
    &got,
    &[
      ("Shop.Web.Sub", "alias", None, 1, 2, None),
      ("Shop.Web.Helpers", "import", None, 1, 3, None),
      ("run", "call", Some("Shop.Web.Sub"), 2, 5, Some(0)),
    ],
  );
  assert!(
    !got
      .iter()
      .any(|r| [2, 3].contains(&r.4) && (r.1 == "module" || r.2.is_some())),
    "a self directive is one unqualified row: {got:#?}"
  );
}

#[test]
fn no_reference_is_a_definition_naming_itself() {
  let rows = elixir(
    b"defmodule Shop.Orders do
  def open(path), do: Repo.get(path)
  defp close(file), do: File.close(file)
end
",
  );
  let defs: Vec<(&str, u32)> = rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Def)
    .map(|s| (s.name.as_str(), s.span.start_line))
    .collect();
  // A def head read as a reference has no qualifier. `File.close` on the
  // `defp close` line shares the def's name and line and IS a reference.
  let selves: Vec<&Symbol> = rows
    .iter()
    .filter(|s| s.kind == SymbolKind::Ref && s.qualifier.is_none())
    .filter(|s| {
      defs
        .iter()
        .any(|(name, line)| *name == s.name && *line == s.span.start_line)
    })
    .collect();
  assert!(
    selves.is_empty(),
    "a def head or a defmodule's alias read as a reference: {selves:#?}"
  );
  assert!(
    refs(&rows).contains(&("close", "call", Some("File"), 2, 3, Some(1))),
    "a same-named remote call on the def's own line is still a reference: {rows:#?}"
  );
}
