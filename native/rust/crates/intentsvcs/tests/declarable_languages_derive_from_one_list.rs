//! **The declarable-language set is DERIVED, and these tests exist because a
//! fourth hand-written list would be invisible.**
//!
//! This estate already carries two deliberately distinct language sets --
//! `rules::LANGUAGES` and `critic::HEADLESS_LANGUAGES`, whose own doc comment
//! says in as many words that it must not be collapsed into the first. v2
//! answered "which languages may I declare" a third way, by listing directories
//! under `intent/plugins/agents/templates/`. All three are correct-looking lists
//! of language names, which is exactly why a disagreement between them would
//! survive review.

use intentsvcs::rules::{LANGUAGES, NON_DECLARABLE, declarable, is_declarable};

/// **THE CONTROL THAT MATTERS: an exclusion must exclude something.**
///
/// `NON_DECLARABLE` names members of `LANGUAGES` to hold back. If `LANGUAGES`
/// ever renames `prose`, this list silently stops excluding anything and
/// `declarable()` starts offering a pack that cannot be declared -- with no
/// error, no warning, and a correct-looking list as the only symptom.
#[test]
fn every_excluded_name_is_a_name_that_exists() {
  for excluded in NON_DECLARABLE {
    assert!(
      LANGUAGES.contains(&excluded),
      "NON_DECLARABLE names `{excluded}`, which is not in LANGUAGES -- the exclusion excludes nothing"
    );
  }
}

/// The derivation is exactly `LANGUAGES` minus `NON_DECLARABLE`, with nothing
/// added. Asserted as set arithmetic rather than against a literal list, because
/// a literal here would BE the fourth copy this module exists to avoid.
#[test]
fn the_declarable_set_is_languages_minus_the_excluded_and_nothing_else() {
  let got = declarable();
  assert_eq!(
    got.len(),
    LANGUAGES.len() - NON_DECLARABLE.len(),
    "derived set is {} long; expected {} - {}",
    got.len(),
    LANGUAGES.len(),
    NON_DECLARABLE.len()
  );
  for lang in &got {
    assert!(
      LANGUAGES.contains(lang),
      "`{lang}` is declarable but not a language"
    );
    assert!(
      !NON_DECLARABLE.contains(lang),
      "`{lang}` is both declarable and excluded"
    );
  }
  for lang in LANGUAGES {
    let expected = !NON_DECLARABLE.contains(&lang);
    assert_eq!(
      got.contains(&lang),
      expected,
      "`{lang}` should{} be declarable",
      if expected { "" } else { " not" }
    );
  }
}

/// `is_declarable` and `declarable()` are two spellings of one question, so they
/// are checked against each other rather than each against a literal -- the
/// property that failed for `version` when two spellings of one capability were
/// allowed to compose their answers separately.
#[test]
fn the_predicate_and_the_list_cannot_disagree() {
  for lang in LANGUAGES {
    assert_eq!(
      is_declarable(lang),
      declarable().contains(&lang),
      "`{lang}`: the predicate and the list disagree"
    );
  }
  assert!(!is_declarable("nope"), "an unknown name is not declarable");
  assert!(!is_declarable(""), "the empty string is not declarable");
}

/// Sorted and duplicate-free, because the list is printed to a user.
#[test]
fn the_list_is_sorted_and_carries_no_duplicates() {
  let got = declarable();
  let mut sorted = got.clone();
  sorted.sort_unstable();
  assert_eq!(got, sorted, "declarable() is not sorted");
  let mut seen = got.clone();
  seen.dedup();
  assert_eq!(got.len(), seen.len(), "declarable() carries a duplicate");
}
