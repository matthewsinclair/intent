//! **A BUILD GAP AND AN OUT-OF-MODEL DECLARATION ARE DIFFERENT SENTENCES, AND
//! THIS FILE EXISTS BECAUSE THEY WERE ABOUT TO BE THE SAME ONE.**
//!
//! Issue `0183` was filed proposing that `intent upgrade` name its out-of-model
//! FILE set, because `conservation_check.sh` reported 135 UNACCOUNTED files on
//! baize and 347 on the canary. **110 of baize's 135 -- 81% -- are
//! `intent/whiteboard/`, which `data-model.md` says LEFT the not-modelled set
//! at D30 and is modelled as `wb_node`/`wb_item`/`wb_message`, built in WP-14.**
//!
//! Implementing that issue as filed would have had the migrator declare 110
//! files out of a model an hv ruling had moved them INTO -- **the denominator
//! attack `sync::NOT_CARRIED` warns about in its own doc comment, committed by
//! the author who was quoting the warning.** Only classifying against the
//! document caught it. Care did not.
//!
//! So the two claims are kept apart HERE, mechanically:
//!
//! - `NOT_CARRIED`   -- the model does not cover this. Closes by fiat, forever.
//! - `NOT_YET_BUILT` -- the model covers this and no build carries it yet.
//!   Closes on its own when the work lands. **A gap that expires is worth more
//!   than a zero that never does** (vc, 2026-08-31).

mod common;

use common::{data_model_text, out_of_model_enumeration, out_of_model_section};

/// **THE ARM THAT WOULD HAVE CAUGHT THE NEAR-MISS.**
///
/// Every `NOT_YET_BUILT` member must be justified by a phrase the document
/// carries, and that phrase must NOT be inside the not-modelled section -- a
/// member in both places is the conflation this file exists to prevent, and it
/// would read perfectly well to a human.
#[test]
fn a_not_yet_built_class_is_inside_the_model_and_not_in_the_excluded_section() {
  let text = data_model_text();
  let excluded = out_of_model_section(&text);

  assert!(
    !intentsvcs::sync::NOT_YET_BUILT.is_empty(),
    "the population is empty, so every arm below passes over nothing -- if WP-14 has landed, \
     delete the member AND this assertion together rather than leaving a census of zero"
  );

  for member in intentsvcs::sync::NOT_YET_BUILT {
    assert!(
      text.contains(member.justified_by),
      "`{}` is reported as modelled-but-unbuilt on the strength of the phrase `{}`, and \
       data-model.md no longer carries it. Re-derive the claim from the document -- do not \
       adjust the phrase to match, which is the claim certifying itself",
      member.shown,
      member.justified_by
    );
    assert!(
      excluded.contains(member.justified_by),
      "`{}` is reported as modelled-but-unbuilt on a phrase outside `## What is deliberately not modelled`, \
       so nothing ties the claim to the section that would otherwise exclude it",
      member.shown
    );
  }
}

/// **THE ARM THAT ACTUALLY GUARDS THE NEAR-MISS, AND IT INDICTS THE OLDER TEST
/// AS WELL AS THIS ONE.**
///
/// The first draft asked whether a `NOT_YET_BUILT` justification appeared
/// INSIDE the not-modelled section and refused it if so. It fired immediately,
/// and it was wrong: `data-model.md` states the whiteboard's DEPARTURE inside
/// that very section -- *The whiteboard left this set at D30* -- so the section
/// contains the exception as well as the rule. **A mention is not an instance**,
/// which this estate has now met three times in one day.
///
/// **AND THE SAME HOLE IS IN `egest_estate.rs`'s PIN.** It accepts any
/// `NOT_CARRIED` justification the section CONTAINS, so a member justified by a
/// phrase drawn from the departure paragraph would pass -- the section mentions
/// the whiteboard while saying it left, and `contains` cannot tell the two
/// apart. That is the exact defect that nearly shipped here, and the older test
/// would not have caught it either.
///
/// So the section is SPLIT at its departure prose, and an out-of-model
/// justification must sit in the ENUMERATION half.
#[test]
fn an_out_of_model_justification_comes_from_the_enumeration_and_not_from_its_exceptions() {
  let text = data_model_text();
  let section = out_of_model_section(&text);

  let (enumeration, exceptions) = out_of_model_enumeration(section);

  assert!(
    !enumeration.is_empty() && !exceptions.is_empty(),
    "the split produced an empty half, so one of the two checks below is vacuous"
  );

  for member in intentsvcs::sync::NOT_CARRIED {
    assert!(
      enumeration.contains(member.justified_by),
      "`{}` is declared out-of-model on the phrase `{}`, which is not in the section's \
       ENUMERATION. If it came from the departure prose instead, the document is saying that \
       thing LEFT the excluded set and the declaration has it exactly backwards",
      member.shown,
      member.justified_by
    );
  }
}

/// The two sets must not overlap, in either direction.
#[test]
fn nothing_is_both_out_of_model_and_merely_unbuilt() {
  for built in intentsvcs::sync::NOT_YET_BUILT {
    for carried in intentsvcs::sync::NOT_CARRIED {
      assert!(
        !built.shown.eq_ignore_ascii_case(carried.shown),
        "`{}` appears in NOT_CARRIED and in NOT_YET_BUILT, which say opposite things about \
         whether the model covers it",
        built.shown
      );
    }
  }
}

/// **THE TWO SENTENCES MUST BE DISTINGUISHABLE BY A READER, NOT ONLY BY A
/// TYPE.** Both are one line on stderr; if they read alike, the split exists in
/// the code and not in the report, which is where it has to exist.
#[test]
fn the_two_reports_do_not_say_the_same_thing() {
  let not_carried = intentsvcs::sync::migration_not_carried();
  let not_yet = intentsvcs::sync::migration_not_yet_built()
    .expect("NOT_YET_BUILT is non-empty, so the line is emitted");

  assert!(
    not_carried.contains("not modelled"),
    "the out-of-model line must say the model does not cover these: {not_carried}"
  );
  assert!(
    not_yet.contains("the model claims these"),
    "the build-gap line must say the model DOES cover these: {not_yet}"
  );
  assert!(
    !not_yet.contains("not modelled"),
    "the build-gap line must not borrow the out-of-model wording -- that is the conflation \
     this file exists to prevent, arriving in the output: {not_yet}"
  );
  assert_ne!(not_carried, not_yet);
}

/// The out-of-model sentence names every member it is composed from.
///
/// **COMPOSED FROM `NOT_CARRIED` AND NOT FROM A SECOND LIST.** The migrator and
/// the extract decline the same three categories for the same reason, so a
/// second const naming them would agree on the day it was written and drift the
/// first time the document moved -- with the pinning test green on both while
/// they disagreed with each other.
#[test]
fn the_migration_line_names_every_class_it_declares() {
  let line = intentsvcs::sync::migration_not_carried();
  for member in intentsvcs::sync::NOT_CARRIED {
    assert!(
      line.contains(member.shown),
      "`{}` is declared not-carried and the emitted line does not name it: {line}",
      member.shown
    );
  }
}

/// The build-gap line names where the files still are, and who owes them.
///
/// **"still on disk" IS THE LOAD-BEARING HALF.** Without it the line reads as a
/// loss report, and the whole point of the distinction is that nothing was lost
/// -- the model's claim is unmet and the bytes are untouched.
#[test]
fn the_build_gap_line_says_where_the_files_are_and_who_owes_them() {
  let line = intentsvcs::sync::migration_not_yet_built().expect("non-empty");
  assert!(
    line.contains("still on disk"),
    "a build gap that does not say the files are intact reads as data loss: {line}"
  );
  for member in intentsvcs::sync::NOT_YET_BUILT {
    assert!(line.contains(member.at), "the line must name where: {line}");
    assert!(
      line.contains(member.owed_by),
      "a gap with no owed work package cannot be chased, and cannot be seen to expire: {line}"
    );
  }
}

/// **THE SPLIT IS DRIVEN TO BOTH SIDES, BECAUSE A PARTITION THAT NEVER
/// SEPARATES ANYTHING IS NOT A PARTITION.**
///
/// The arm above passes if every declared justification is in the enumeration
/// half -- which it also would if the split were degenerate and the
/// "enumeration" were the whole section. This pins a phrase that belongs only
/// to the exception prose and requires it to be on the far side.
#[test]
fn the_split_separates_the_enumeration_from_the_exception_prose() {
  let text = data_model_text();
  let section = out_of_model_section(&text);
  let (enumeration, exceptions) = out_of_model_enumeration(section);

  // `wb_node` appears ONLY where the document says the whiteboard is modelled.
  assert!(
    exceptions.contains("wb_node"),
    "the exception half no longer names the model the whiteboard moved into, so the split is \
     landing somewhere else in the section"
  );
  assert!(
    !enumeration.contains("wb_node"),
    "the enumeration half contains the whiteboard's model, so the split is degenerate and the \
     arm above is checking against the whole section -- which is what it exists to avoid"
  );

  // And the enumeration half must still hold the real members, or the split
  // has cut the section in the wrong place and everything above passes on air.
  assert!(
    enumeration.contains("wip.md / restart.md"),
    "the enumeration half lost a member the document plainly lists, so the cut is wrong"
  );
}
