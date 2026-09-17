//! AT-04.2 / AC-04.1 (ST0076 WP-04): **`--subkind` speaks the index's own words, and an
//! unknown one is refused with the roster of the languages in scope.**
//!
//! The roster is derived from the compiled queries rather than listed (vc,
//! 2026-09-17), so the property worth holding is the one a derivation can get
//! wrong without anyone noticing: **every subkind the extractor actually writes
//! is in the roster.** A clause subkind read from `elixir.scm`'s `#any-of?` list
//! is the part read from source text rather than from the compiled query, and
//! it is the part this arm exists for.

use intentsvcs::index::symbols::{subkinds, symbols_of};
use intentsvcs::search::FilterWords;

const RUST: &[u8] = b"mod store {
  pub struct Address { pub host: String }
  pub union Bits { a: u8 }
  pub enum Mode { Read, Write }
  pub trait Resolve { fn resolve(&self); }
  pub type Name = String;
  pub const LIMIT: usize = 3;
  pub static GREETING: &str = \"hi\";
  macro_rules! noisy { () => {} }
  impl Address {
    pub fn new(host: String) -> Self { Address { host } }
    pub fn is_local(&self) -> bool { helper(); noisy!(); true }
  }
  impl Resolve for Address { fn resolve(&self) {} }
  fn helper() {}
}
";

const ELIXIR: &[u8] = b"defmodule Intent.Store do
  defstruct [:path]
  def open(path, mode \\\\ :read), do: {path, mode}
  defp count, do: 0
  defmacro trace(expr), do: expr
  defmacrop quiet(expr), do: expr
  defguard is_row(term) when is_map(term)
  defguardp is_col(term) when is_map(term)
  defdelegate fetch(key), to: Rows
  def run(x), do: x |> count() |> Enum.map(&open/1)
end
defmodule Intent.Oops do
  defexception [:message]
end
defprotocol Intent.Named do
  def name(thing)
end
defimpl Intent.Named, for: Intent.Store do
  def name(_), do: \"store\"
end
";

#[test]
fn every_subkind_the_extractor_writes_is_in_its_languages_roster() {
  for (lang, path, src) in [
    ("rust", "src/store.rs", RUST),
    ("elixir", "lib/store.ex", ELIXIR),
  ] {
    let rows = symbols_of(lang, path, src).expect("the grammar is in the default build");
    let roster = subkinds(lang);
    let written: std::collections::BTreeSet<&str> =
      rows.iter().map(|row| row.subkind.as_str()).collect();
    assert!(
      written.len() > 5,
      "the control: the fixture must write a spread of subkinds or the check below is vacuous: {written:?}"
    );
    for subkind in &written {
      assert!(
        roster.iter().any(|word| word == subkind),
        "{lang} writes `{subkind}` and its roster does not name it: {roster:?}"
      );
    }
  }

  let elixir = subkinds("elixir");
  for clause in [
    "def",
    "defp",
    "defmacro",
    "defmacrop",
    "defguard",
    "defguardp",
    "defdelegate",
  ] {
    assert!(
      elixir.iter().any(|word| word == clause),
      "the clause subkinds come from elixir.scm's `#any-of?` list: {elixir:?}"
    );
  }
  assert!(
    !elixir.iter().any(|word| word == "function"),
    "every Elixir function pattern writes its clause's spelling, so `function` is a word it never writes: {elixir:?}"
  );
  assert!(
    subkinds("shell").is_empty() && subkinds("cobol").is_empty(),
    "a language with no query here writes no subkinds"
  );
}

#[test]
fn an_unknown_subkind_is_refused_with_the_roster_of_the_languages_in_scope() {
  let refused = FilterWords {
    subkinds: vec!["strcut".to_string()],
    ..FilterWords::default()
  }
  .check()
  .expect_err("a subkind no language writes is refused");
  assert!(
    refused.problem.contains("`strcut`") && refused.choices.iter().any(|c| c == "struct"),
    "the refusal names the word and offers the roster: {refused:?}"
  );

  let refused = FilterWords {
    subkinds: vec!["method".to_string()],
    langs: vec!["elixir".to_string()],
    ..FilterWords::default()
  }
  .check()
  .expect_err("`method` is Rust's word, not Elixir's");
  assert!(
    refused.problem.contains("elixir") && refused.choices.iter().any(|c| c == "defp"),
    "the roster is the one for the languages in scope: {refused:?}"
  );

  let ask = FilterWords {
    kinds: vec!["def".to_string()],
    subkinds: vec!["def".to_string()],
    langs: vec!["elixir".to_string()],
    ..FilterWords::default()
  }
  .check()
  .expect("`--kind def --subkind def` is two words in two vocabularies");
  assert_eq!(ask.subkinds, vec!["def".to_string()]);
  assert_eq!(ask.kinds.len(), 1);
}
