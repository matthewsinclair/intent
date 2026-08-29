//! **`info.md` ROUND-TRIPS ITS TWO AUTHORED SECTIONS, AND REFUSES EVERY OTHER
//! EDIT RATHER THAN DISCARDING IT.**
//!
//! hv ruled the shape on 2026-08-29, first-hand: _I want to edit the ST and
//! then I want a sync to know that's been edited and update the db._ The
//! allow-list is `## Objective` and `## Context`; hv ruled the out-of-scope
//! policy separately and chose REFUSE-and-name over carry-and-warn, so the
//! store and the file are both untouched when anything else moved.
//!
//! # The arm vc made binding, and why it is not a duplicate of the others
//!
//! vc's condition on this build was an arm that plants a byte in a region
//! **neither authority owns** -- not the authored sections, not a section the
//! reader is meant to reject on sight, but the generated frame itself -- and
//! requires the round-trip to leave the rendered bytes IDENTICAL. Without it,
//! *ignores every other byte* is an intention rather than a property. All five
//! of the prose-damage issues (0124, 0126, 0127, 0129, 0133) are a reader that
//! was only supposed to be reading.
//!
//! # Why the reader compares against a render instead of parsing the file
//!
//! **The same heading can be authored or generated and the bytes do not say
//! which.** `views::info` emits `## Work Packages` itself UNLESS the thread's
//! `body` already carries one, in which case the authored copy renders
//! verbatim and the generated one defers. So any rule of the form "these
//! headings are generated" is wrong on some real thread, and the open
//! catch-all in `legacy::scan` -- everything that is not Objective or Context
//! -- swallows the generated sections into authored `body` when pointed at a
//! v3 view. That is the damage class arriving through the door built to stop
//! it, which is why this reader is a CLOSED allow-list.

mod common;

use common::{ctx, sample_thread};
use intentsvcs::views::{self, INFO_ROUND_TRIP_SECTIONS};

/// The control every other arm depends on: an UNEDITED render reads back to
/// exactly the values it was rendered from.
///
/// **If this fails, no refusal below means anything** -- a reader that cannot
/// recognise its own output would refuse every edit, and the refusal arms
/// would pass for the wrong reason.
#[test]
fn success_an_unedited_render_reads_back_unchanged() {
  let thread = sample_thread("ST0056");
  let rendered = views::info(&thread, &ctx());

  let got =
    views::info_read_back(&thread, &ctx(), &rendered).expect("an unedited render must read back");

  assert_eq!(got.objective, thread.objective);
  assert_eq!(got.context, thread.context);
}

#[test]
fn success_an_edit_to_objective_is_carried() {
  let thread = sample_thread("ST0056");
  let rendered = views::info(&thread, &ctx());
  let edited = rendered.replace(&thread.objective, "Ship the reified model, edited by hand.");

  assert_ne!(
    edited, rendered,
    "the fixture must actually change, or this arm proves nothing"
  );

  let got = views::info_read_back(&thread, &ctx(), &edited)
    .expect("an Objective edit is in the allow-list");

  assert_eq!(got.objective, "Ship the reified model, edited by hand.");
  assert_eq!(
    got.context, thread.context,
    "an untouched section must not move"
  );
}

#[test]
fn success_an_edit_to_context_is_carried() {
  let thread = sample_thread("ST0056");
  let rendered = views::info(&thread, &ctx());
  let edited = rendered.replace(
    &thread.context,
    "v2 bolted schema onto markdown three times, and we counted.",
  );

  assert_ne!(edited, rendered);

  let got =
    views::info_read_back(&thread, &ctx(), &edited).expect("a Context edit is in the allow-list");

  assert_eq!(
    got.context,
    "v2 bolted schema onto markdown three times, and we counted."
  );
  assert_eq!(got.objective, thread.objective);
}

/// **vc's BINDING ARM: a byte planted in a region neither authority owns must
/// leave the model UNCORRUPTED, so the next render is byte-identical.**
///
/// vc's words: without this, *ignores every other byte* is an intention. All
/// five of the prose-damage issues (0124, 0126, 0127, 0129, 0133) are a reader
/// that was only supposed to be reading.
///
/// **THIS ARM USED TO ASSERT A REFUSAL AND THAT WAS MY OVER-READING OF hv's
/// RULING, NOT vc's CONDITION.** vc asked for byte-identity. Refusing on a
/// GENERATED region is not merely stricter, it is unworkable: a view the model
/// has moved past differs in exactly these regions through nobody's fault, so
/// the refusal fired on `sync_scope.rs` the moment a title changed in canon.
/// **hv's refuse ruling is about text the operator WROTE**, and it is asserted
/// on the authored regions below, where the loss it names can actually happen.
#[test]
fn invariant_a_byte_planted_in_a_generated_region_leaves_the_model_untouched() {
  let thread = sample_thread("ST0056");
  let rendered = views::info(&thread, &ctx());
  let edited = rendered.replace(
    "This cover never restates them.",
    "This cover never restates them. PLANTED.",
  );

  assert_ne!(edited, rendered, "the plant must land, or the arm is inert");

  let got = views::info_read_back(&thread, &ctx(), &edited)
    .expect("a generated region is regenerated, not carried and not refused");

  assert_eq!(
    got.objective, thread.objective,
    "the plant reached a field it does not own"
  );
  assert_eq!(
    got.context, thread.context,
    "the plant reached a field it does not own"
  );
  assert_eq!(
    views::info(&thread, &ctx()),
    rendered,
    "the round-trip must leave the rendered bytes byte-identical"
  );
}

/// **hv's REFUSE RULING, ON THE REGION WHERE IT BITES.** The preamble is
/// AUTHORED -- it is a modelled field carried verbatim -- and it is not in the
/// allow-list, so an edit to it is real writing this door cannot take.
/// Discarding it silently is the loss `IN-AG-NO-SILENT-001` names, and the
/// operator who loses the work is not the operator who runs the sync.
#[test]
fn failure_an_edit_to_the_authored_preamble_refuses() {
  let mut thread = sample_thread("ST0056");
  thread.preamble = "A line the author put above the first heading.".to_string();
  let rendered = views::info(&thread, &ctx());
  let edited = rendered.replace(&thread.preamble, "A line the author then CHANGED by hand.");

  assert_ne!(edited, rendered, "the edit must land, or the arm is inert");

  let refused = views::info_read_back(&thread, &ctx(), &edited)
    .expect_err("an edit to authored prose that cannot be carried must refuse");

  assert!(
    refused.iter().any(|s| s.contains("preamble")),
    "the refusal must name the preamble so the author can find their text: {refused:?}"
  );
}

/// The same ruling on an authored SECTION -- one the thread's `body` carries.
/// **`## Work Packages` is deliberately the subject**: it is a name the
/// renderer would generate for itself, so this arm also proves the reader
/// honours `carries_heading` and does not treat an authored section as
/// generated merely because it recognises the words.
#[test]
fn failure_an_edit_to_an_authored_section_the_body_carries_refuses() {
  let mut thread = sample_thread("ST0056");
  thread.body = "## Work Packages\n\nThe author's own table, not ours.\n".to_string();
  let rendered = views::info(&thread, &ctx());

  assert!(
    rendered.contains("The author's own table, not ours."),
    "the fixture must actually render the authored section, or this arm proves nothing"
  );

  let edited = rendered.replace("The author's own table, not ours.", "Edited by hand.");
  let refused = views::info_read_back(&thread, &ctx(), &edited)
    .expect_err("an authored section is the author's writing and cannot be silently dropped");

  assert!(
    refused.iter().any(|s| s.contains("Work Packages")),
    "the refusal must name the section: {refused:?}"
  );
}

/// A heading typed in by hand is refused BEFORE any body is compared, because
/// the region lists stop corresponding and every later comparison would be
/// against the wrong neighbour.
#[test]
fn failure_a_hand_added_heading_refuses_and_names_itself() {
  let thread = sample_thread("ST0056");
  let rendered = views::info(&thread, &ctx());
  let edited = rendered.replace(
    "## Context\n",
    "## Scope\n\nSomething the author wanted.\n\n## Context\n",
  );

  assert_ne!(edited, rendered);

  let refused =
    views::info_read_back(&thread, &ctx(), &edited).expect_err("a hand-added heading must refuse");

  assert!(
    refused.iter().any(|s| s.contains("## Scope")),
    "the refusal must name the heading that was added: {refused:?}"
  );
}

/// **THE PLACEHOLDER MUST NOT ROUND-TRIP.** `section_body` renders an empty
/// field as `_(not yet written)_`; reading that back verbatim would write the
/// placeholder into the model as though a human had typed it, and the next
/// render would show it as real prose that no longer says it is missing.
#[test]
fn invariant_the_not_yet_written_placeholder_reads_back_as_empty() {
  let mut thread = sample_thread("ST0056");
  thread.objective = String::new();
  let rendered = views::info(&thread, &ctx());

  assert!(
    rendered.contains("_(not yet written)_"),
    "the fixture must actually render the placeholder, or this arm tests nothing"
  );

  let got =
    views::info_read_back(&thread, &ctx(), &rendered).expect("an unedited render must read back");

  assert_eq!(
    got.objective, "",
    "the placeholder must not enter the model as prose"
  );
}

/// The renderer and the reader share ONE declaration. A heading renamed in one
/// and not the other stops round-tripping, and nothing mechanical would say so
/// -- so the declaration is asserted against the bytes the renderer emits.
#[test]
fn invariant_every_declared_round_trip_section_is_actually_rendered() {
  let thread = sample_thread("ST0056");
  let rendered = views::info(&thread, &ctx());

  for section in INFO_ROUND_TRIP_SECTIONS {
    assert!(
      rendered
        .lines()
        .any(|l| l.strip_prefix("## ").is_some_and(|h| h.trim() == *section)),
      "`{section}` is declared round-trippable but the renderer emits no such heading"
    );
  }
}

/// **A VERSION BUMP MUST NOT READ AS A HAND-EDIT.**
///
/// `finish` stamps `Generated by Intent v{version}` into every view, so a file
/// rendered by yesterday's binary differs from today's render in a byte no
/// human touched. **A reader comparing whole texts would refuse every thread in
/// the estate on the morning after an upgrade** -- and refuse them naming a
/// section the operator never edited, which is worse than refusing silently.
#[test]
fn invariant_a_version_bump_is_not_mistaken_for_an_edit() {
  let thread = sample_thread("ST0056");
  let older = views::RenderContext {
    version: "2.99.0-yesterday",
    todo_watermark: None,
  };
  let on_disk = views::info(&thread, &older);

  assert_ne!(
    on_disk,
    views::info(&thread, &ctx()),
    "the two renders must actually differ, or this arm proves nothing"
  );

  let got = views::info_read_back(&thread, &ctx(), &on_disk)
    .expect("a view rendered by an older binary is not a hand-edit");
  assert_eq!(got.objective, thread.objective);
  assert_eq!(got.context, thread.context);
}

/// **THE CONTROL ON THE VERSION FIX.** Rendering against the file's declared
/// version stops a version bump reading as an edit. The banner is generated
/// bytes -- `finish` appends it unconditionally and the sentence inside it says
/// so -- so a reworded banner is regenerated rather than refused, and the model
/// must come through it untouched.
#[test]
fn invariant_a_reworded_banner_does_not_reach_the_model() {
  let thread = sample_thread("ST0056");
  let rendered = views::info(&thread, &ctx());
  let edited = rendered.replace("Do not edit this file", "Feel free to edit this file");

  assert_ne!(
    edited, rendered,
    "the reword must land, or the arm is inert"
  );

  let got = views::info_read_back(&thread, &ctx(), &edited).expect("the banner is regenerated");
  assert_eq!(got.objective, thread.objective);
  assert_eq!(got.context, thread.context);
}
