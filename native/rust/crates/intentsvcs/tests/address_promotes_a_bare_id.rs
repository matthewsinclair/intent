//! **A BARE ARTEFACT ID IS PROMOTED TO AN ADDRESS, AND NOTHING ELSE IS.**
//!
//! The command line takes `<address>` because the services refuse in address
//! terms -- `Facade::hydrate` has two refusal arms (a foreign authority, a
//! non-artefact entity) that a bare id cannot reach, and taking `<id>` would
//! make both unreachable and their messages dead text. But `address::parse`
//! demands the `intent://` scheme, so a literal reading makes the everyday
//! invocation `intent hydrate intent:///threads/ST0057`, which nobody types.
//!
//! `promote` is the door: a URL parses as written, a bare artefact id is
//! promoted, anything else is refused BY NAME.
//!
//! # The refusal is the half that gets built wrong
//!
//! `intent hydrate ST57` must not report a missing thread. It is a typo in the
//! argument grammar and the estate was never consulted; sending an operator to
//! look for a thread that was never addressed is a worse answer than no answer.
//! It equally must not say "an address begins intent://" -- that answers a
//! question a caller who typed a bare id did not ask.

use intentsvcs::address::{AddressError, Entity, promote};
use intentsvcs::remedy::Remedy;

/// **A THREAD ID.**
#[test]
fn a_bare_thread_id_becomes_this_project_s_thread() {
  let a = promote("ST0057").expect("a thread id is addressable");
  assert_eq!(
    a.authority, None,
    "an empty authority means THIS project -- a promotion that invented one \n       \
     would silently address someone else's estate"
  );
  assert_eq!(
    a.entity
      .artefact()
      .map(|(s, id)| (s.as_str().to_string(), id.to_string())),
    Some(("STEELTHREAD".to_string(), "ST0057".to_string())),
    "and it resolves to the artefact the manifest names with that sigil"
  );
}

/// **AN ISSUE ID**, which is the case that makes `<address>` necessary at all:
/// `st` cannot carry issues, so the verb is top-level and the argument must
/// span both forms.
///
/// # The spelling moved from `0042` to `i42`, and BOTH assertions survived
///
/// This drove `promote("0042")`, which resolved because
/// [`intentsvcs::model::normalise_id`] carried a width arm assigning every
/// untagged four-digit token to `Issue`. That arm came out on 2026-08-31 when
/// hv's resolution ladder landed: `THREAD_DIGITS` and `ISSUE_DIGITS` are both
/// 4, so the arm was deciding by width what it could not decide by form, and
/// `0042` is now [`AddressError::AmbiguousId`] -- asserted below, because the
/// refusal is the ruling.
///
/// **THE TAGGED SPELLING CARRIES THE REST OF THE TEST, WHICH IS THE POINT OF
/// CHANGING IT RATHER THAN DELETING IT.** What this test is FOR is the
/// addressable-versus-realisable split below, and that claim is about ISSUES
/// rather than about how one was spelled. `i42` names an issue with no
/// ambiguity to resolve, so the second assertion is tested exactly as before
/// and a store is still never consulted.
///
/// # An issue is still ADDRESSABLE and is no longer an ARTEFACT
///
/// This asserted `artefact()` returned `Some(("ISSUE", "0042"))`. hv ruled on
/// 2026-08-20 that issues are canon-and-store only, so it now answers `None`
/// -- **and the two halves of that are separate claims that this test used to
/// conflate.**
///
/// Promotion is about NAMING: a bare `0042` still resolves to this project's
/// issue, `intent://issue/0042` still means something, and `intent issues
/// show 0042` still answers. What ended is the claim that an issue has a
/// realised form on disk for `.intentfiles` to name. **Addressable and
/// realisable were one assertion here and they were never the same
/// property** -- an event and a whiteboard node have always been the first
/// without the second.
#[test]
fn a_bare_issue_id_becomes_this_project_s_issue() {
  // **THE UNTAGGED FORM IS REFUSED, AND SAYING SO HERE IS NOT DUPLICATION.**
  // `operator_id_spellings` proves the model refuses it; this proves the
  // refusal REACHES `promote` rather than being re-decided at this door -- the
  // second resolver that the whole address ruling exists to prevent.
  assert!(
    matches!(
      promote("0042"),
      Err(intentsvcs::address::AddressError::AmbiguousId { seq: 42, .. })
    ),
    "a bare four-digit token names an issue AND a thread, and this door is pure, so it reports \
     the ambiguity rather than picking"
  );

  let a = promote("i42").expect("a tagged issue id is addressable");
  assert_eq!(
    a.entity,
    Entity::Issue {
      id: "0042".to_string()
    },
    "promotion still names this project's issue -- the ruling took away its realised form, not \
     its identity"
  );
  assert_eq!(
    a.entity.artefact(),
    None,
    "an issue lives only in canon and the store, so it is not an artefact `.intentfiles` can \
     name and `hydrate` refuses it by name"
  );
}

/// **A FULL ADDRESS PASSES THROUGH UNTOUCHED, INCLUDING ITS AUTHORITY.**
///
/// This is the arm that proves promotion is a fallback and not a rewrite. A
/// `promote` that built an address from the id it found anywhere in the input
/// would quietly discard the authority -- turning a deliberate cross-project
/// reference into a local one, which is the failure `Facade::hydrate` refuses
/// by name and would never get the chance to.
#[test]
fn a_full_address_is_parsed_as_written_and_keeps_its_authority() {
  let a = promote("intent://other/threads/ST0057").expect("a URL parses");
  assert_eq!(
    a.authority.as_deref(),
    Some("other"),
    "the authority survives promotion -- otherwise a cross-project reference \n       \
     silently becomes a local one and hydrate's refusal never fires"
  );
}

/// **A VIEW IS STILL REFUSED**, so promotion did not open a second door into
/// the grammar with different rules.
#[test]
fn promotion_does_not_weaken_the_grammar() {
  let e = promote("intent:///threads/ST0057/info.md").expect_err("views have no address");
  assert!(
    matches!(e, AddressError::ViewAddressed { .. }),
    "a URL still goes through `parse` and meets every one of its refusals, \n       \
     got: {e}"
  );
}

/// **THE MALFORMED ARGUMENT, AND BOTH WRONG ANSWERS ARE ASSERTED AGAINST.**
///
/// **THE EXEMPLAR MOVED FROM `ST57` TO `ST5x` AND THE PROPERTY DID NOT.** `ST57`
/// was a typo when this door accepted only the two canonical forms; it now
/// resolves to `ST0057`, because the accepted set widened to v2's five spellings
/// plus the explicit tags. **A test whose subject becomes legal is not evidence
/// that the property lapsed** -- what it pins is that a spelling naming NOTHING
/// is a usage error rather than a not-found, and that survives the widening
/// untouched. Picking a new exemplar is the correct repair; deleting the test
/// because its input started passing would retire the property with it.
#[test]
fn a_typo_is_a_usage_error_naming_both_forms() {
  let e = promote("ST5x").expect_err("ST5x is not an id under any spelling");

  assert!(
    matches!(e, AddressError::NotAddressable { .. }),
    "not a not-found and not a URL complaint, got: {e}"
  );

  let said = format!("{e} -- {}", e.remedy());
  assert!(
    said.contains("ST0000") && said.contains("0042"),
    "the refusal must name BOTH accepted forms, since which one the caller \n       \
     meant is exactly what it cannot tell. said: {said}"
  );
  assert!(
    !said.contains("no such") && !said.contains("not found"),
    "and it must not read as a missing artefact: the estate was never \n       \
     consulted, so nothing was missing. said: {said}"
  );
}

/// **AN EMPTY ARGUMENT IS THE SAME REFUSAL**, not a panic and not a default.
#[test]
fn an_empty_argument_is_refused_by_the_same_door() {
  let e = promote("").expect_err("the empty string names nothing");
  assert!(matches!(e, AddressError::NotAddressable { .. }), "got: {e}");
}
