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

use crate::common::{data_model_text, out_of_model_enumeration, out_of_model_section};

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
    // **AND IT MUST NOT NAME A WORK PACKAGE -- D37**, which this arm asserted
    // the OPPOSITE of for an hour. `no_pm_state_in_output.rs` refused the
    // literal, not just the render: a shipped `WP-14` is one edit from a
    // terminal. The number is not carried here at all now -- `data-model.md`
    // says "built in WP-14" in the sentence `justified_by` pins to, so the
    // document is its one home.
    assert!(
      !line.contains("WP-"),
      "the emitted line names a work package of ours, which D37 forbids: {line}"
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

// ---------------------------------------------------------------------------
// THE PER-ARTEFACT HALF
//
// The arms above pin the two SUMMARY lines. These pin the enumeration that
// details the second one -- `sync::migration_not_yet_built_artefacts` -- and
// they exist because the estate already held the per-artefact standard and was
// applying it to the smaller class: `legacy.rs` names each oversized
// attachment individually, 8 of them here, while 1,386 whiteboard files on
// Lamplight reached the same report as one directory noun.
// ---------------------------------------------------------------------------

/// A fixture estate carrying one declared member's directory, with `n` files.
///
/// **Built under the member's OWN `at`, never under a path this test picks.**
/// A fixture that invents its own directory would pass while the shipped
/// constant pointed somewhere else entirely -- the shape of a test whose
/// subject cannot fail, and the reason the 0271 build's sixteen reds were the
/// fixtures rather than the guard.
fn estate_with_member_files(
  member: &intentsvcs::sync::NotYetBuilt,
  n: usize,
) -> crate::common::Fixture {
  let fx = crate::common::v2_estate();
  for i in 0..n {
    fx.write_file(&format!("{}node{i}/wip.md", member.at), "board\n");
  }
  fx
}

/// **EVERY FILE UNDER A DECLARED MEMBER GETS ITS OWN RECORD, CARRYING THE
/// MEMBER'S OWN JUSTIFICATION.**
///
/// The count is asserted against the WALK rather than against a literal, so
/// the arm keeps meaning the same thing as the fixture grows -- and the
/// justification arm is what makes the enumeration inherit the document
/// pinning above: a reason emitted per artefact cannot drift from the ruling
/// that authorises it without the first test in this file going red.
#[test]
fn every_file_under_a_declared_member_is_named_with_its_own_reason() {
  for member in intentsvcs::sync::NOT_YET_BUILT {
    let fx = estate_with_member_files(member, 3);
    let found = intentsvcs::sync::migration_not_yet_built_artefacts(&fx.project());
    let on_disk = intentsvcs::project::Project::files_in(&fx.root().join(member.at));

    assert_eq!(
      found.len(),
      on_disk.len(),
      "every file under `{}` is named or the enumeration is a directory noun again: {found:?}",
      member.at
    );
    assert_eq!(found.len(), 3, "the fixture's own files are the population");

    for f in &found {
      assert!(
        f.file.starts_with(member.at),
        "a record names a path outside the member it came from: {}",
        f.file
      );
      assert_eq!(f.class, intentsvcs::finding::FindingClass::ModelledNotBuilt);
      assert!(
        f.detail.contains(member.justified_by),
        "the record must quote the phrase that puts it inside the model, not a \
         sentence of the emitter's own: {}",
        f.detail
      );
      // **THE LOAD-BEARING HALF, PER LINE.** These are read one at a time, by
      // grep, far from any header -- and a line that does not say the file is
      // intact reads as a loss manifest. Same requirement the summary line
      // carries, for the same reason.
      assert!(
        f.detail.contains("on disk"),
        "a record that does not say the file is intact reads as data loss: {}",
        f.detail
      );
    }
  }
}

/// **NOTHING TO ENUMERATE IS THE ORDINARY ANSWER, AND IT IS SILENT.**
///
/// Most estates have no whiteboard. An empty vector lets the caller print no
/// section at all, rather than a heading over nothing -- which would be a
/// migration announcing a gap that is not there.
#[test]
fn a_member_directory_that_is_absent_enumerates_nothing() {
  let fx = crate::common::v2_estate();
  assert!(
    intentsvcs::sync::migration_not_yet_built_artefacts(&fx.project()).is_empty(),
    "an estate with none of the declared directories owes no records"
  );
}

/// **THE RECORDS MUST NEVER REACH `Scan`, AND THIS IS THE ARM THAT CATCHES IT.**
///
/// vc's ruling, 2026-09-05, under hv's pen: enumerate at the declaration, do
/// NOT widen `legacy::scan`'s population. The cost of getting it wrong is not
/// untidiness. `Scan::residue` BLOCKS (`migrate.rs`), so routing these through
/// `record` would refuse a migration to every estate that has a whiteboard --
/// permanently, on the ordinary shape of a project rather than on a defect --
/// and `carried` would print them under *converts as-is, no action*, which is
/// the one thing they do not do.
///
/// **It drives an estate that HAS the files**, so it is a partition that can
/// actually separate: an empty whiteboard would pass this arm under the very
/// wiring it exists to refuse.
#[test]
fn the_build_gap_records_never_enter_the_scan() {
  for member in intentsvcs::sync::NOT_YET_BUILT {
    let fx = estate_with_member_files(member, 3);
    assert!(
      !intentsvcs::sync::migration_not_yet_built_artefacts(&fx.project()).is_empty(),
      "precondition: the fixture must carry files, or this arm proves nothing"
    );

    let scan = intentsvcs::legacy::scan(&fx.project()).expect("the estate parses");
    for f in scan.residue.iter().chain(scan.carried.iter()) {
      assert_ne!(
        f.class,
        intentsvcs::finding::FindingClass::ModelledNotBuilt,
        "a build-gap record reached `Scan`, where residue BLOCKS and carried claims it \
         converts as-is -- neither is true of it: {f:?}"
      );
      assert!(
        !f.file.starts_with(member.at),
        "`legacy::scan` walked `{}`, whose files are not v2 thread material: {}",
        member.at,
        f.file
      );
    }
    assert!(
      scan.residue.is_empty(),
      "an estate whose only extra content is a declared member still migrates: {:?}",
      scan.residue
    );
  }
}

/// **NEITHER RENDERING LEADS `residue:`, AND THE TWO AGREE ON THE FACTS.**
///
/// `Display`'s lead was an `if/else` on `Advisory` until this class arrived,
/// which silently spells every later variant `residue:` -- here that would be
/// the report telling an operator to repair a file nothing is wrong with.
#[test]
fn a_build_gap_record_is_never_rendered_as_residue() {
  let member = intentsvcs::sync::NOT_YET_BUILT
    .first()
    .expect("the population is non-empty, asserted above");
  let fx = estate_with_member_files(member, 1);
  let found = intentsvcs::sync::migration_not_yet_built_artefacts(&fx.project());
  let one = found.first().expect("one file, one record");

  assert!(one.not_built_line().starts_with("not-yet-carried: "));
  assert!(
    !format!("{one}").starts_with("residue:"),
    "`Display` calls a build gap residue, which sends an operator to fix an intact file: {one}"
  );
  // The per-line form owes no remedy: the class remedy is a real sentence and
  // is printed once, not 1,386 times.
  assert!(
    !one.not_built_line().contains("remedy:"),
    "the per-artefact line must not carry the class remedy: {}",
    one.not_built_line()
  );
}
