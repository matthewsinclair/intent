//! AT-02.2 / AC-02.2: **lines outside the markers survive an `organize`
//! rewrite BYTE FOR BYTE, while the generated region is rewritten from
//! status.** Checked with a pin present and the generated region changing in
//! the same run.
//!
//! **The two halves must be checked TOGETHER or neither is checked.** A test
//! that only asserts the pins survived passes trivially against a writer that
//! does nothing at all; a test that only asserts the region changed passes
//! against one that rewrites the whole file. The criterion says "in the same
//! run" for that reason, and every case here asserts both.
//!
//! **Byte for byte is not the same as artefact for artefact**, and the
//! difference is the entire content of this row. A writer that parses the
//! pinned region to [`Entry`] and prints it back preserves every ARTEFACT and
//! silently normalises everything else -- a trailing comment, the two spaces
//! somebody put before it, a blank line left to group three related pins. That
//! writer passes any test written in terms of ids and fails this one, which is
//! why the fixture below is deliberately full of formatting nobody would
//! choose by accident.

use intentsvcs::intentfiles::{BEGIN_MARKER, END_MARKER, Generated, Region, Sigil, parse, render};

/// A manifest whose pinned region carries every kind of content a re-render
/// would quietly lose: an odd indent, doubled spaces before a comment, a bare
/// comment line, a blank line used as a separator, and trailing whitespace.
const PINNED_AND_AWKWARD: &str = "\
# the pins, and the formatting is the point
STEELTHREAD:ST0011      # kept after it closed -- see the 2026-08 ruling

  ISSUE:0042  #   spacing a re-render would tidy

# a bare comment between groups
STEELTHREAD:ST0000
# BEGIN INTENT
STEELTHREAD:ST0056
# END INTENT
";

/// Everything outside the markers, as raw bytes, in file order.
///
/// Derived from the text rather than from the parsed model on purpose: a
/// helper that went through `Entry` would compare two re-renderings and agree
/// with itself no matter what the writer did.
fn outside_markers(text: &str) -> Vec<&str> {
  let mut out = Vec::new();
  let mut inside = false;
  for line in text.lines() {
    match line.trim() {
      BEGIN_MARKER => inside = true,
      END_MARKER => inside = false,
      _ if !inside => out.push(line),
      _ => {}
    }
  }
  out
}

fn inside_markers(text: &str) -> Vec<&str> {
  let mut out = Vec::new();
  let mut inside = false;
  for line in text.lines() {
    match line.trim() {
      BEGIN_MARKER => inside = true,
      END_MARKER => inside = false,
      _ if inside => out.push(line),
      _ => {}
    }
  }
  out
}

#[test]
fn pins_survive_byte_for_byte_while_the_region_changes() {
  let before = PINNED_AND_AWKWARD;
  let after = render(
    before,
    &[
      Generated::new(Sigil::SteelThread, "ST0057"),
      Generated::new(Sigil::Issue, "0099"),
    ],
  )
  .expect("a readable manifest renders");

  // Half one: the region actually changed. Without this the test passes
  // against a writer that returns its input.
  assert_ne!(
    inside_markers(before),
    inside_markers(after.as_str()),
    "the generated region must be rewritten from status"
  );
  assert_eq!(
    inside_markers(after.as_str()),
    vec!["STEELTHREAD:ST0057", "ISSUE:0099"],
    "the region holds exactly what status supplied, in order"
  );

  // Half two: everything else is untouched, compared as BYTES.
  assert_eq!(
    outside_markers(before),
    outside_markers(after.as_str()),
    "every line outside the markers must survive byte for byte -- comments,\n       \
     indentation, blank lines and all"
  );
}

/// The pinned artefacts are still THERE and still PINNED after the rewrite --
/// the byte comparison above would also pass if the writer deleted the pinned
/// region and the helper returned two empty vectors.
#[test]
fn the_pinned_artefacts_survive_as_pins() {
  let after = render(
    PINNED_AND_AWKWARD,
    &[Generated::new(Sigil::SteelThread, "ST0057")],
  )
  .expect("renders");
  let m = parse(&after).expect("the rendered manifest parses");

  let pinned: Vec<&str> = m.pinned().map(|e| e.id.as_str()).collect();
  assert_eq!(pinned, vec!["ST0011", "0042", "ST0000"]);
  assert!(
    m.pinned().all(|e| e.region == Region::Pinned),
    "a pin does not change region across a rewrite"
  );

  let comment = m.pinned().next().unwrap().comment.as_deref();
  assert_eq!(
    comment,
    Some("kept after it closed -- see the 2026-08 ruling"),
    "the reason a pin exists survives the rewrite that would otherwise erase it"
  );
}

/// Emptying the region is a rewrite like any other. The pins are what must not
/// notice.
#[test]
fn an_empty_status_empties_the_region_and_keeps_the_pins() {
  let after = render(PINNED_AND_AWKWARD, &[]).expect("renders");
  assert!(
    inside_markers(after.as_str()).is_empty(),
    "no artefacts in status means no lines in the region"
  );
  assert_eq!(
    outside_markers(PINNED_AND_AWKWARD),
    outside_markers(after.as_str()),
    "an empty region must not disturb a single pinned byte"
  );
}

/// `organize`'s first run against a hand-written manifest that has never had a
/// generated region.
#[test]
fn a_manifest_with_no_markers_gains_them_without_losing_its_pins() {
  let hand_written = "# just pins so far\nSTEELTHREAD:ST0011\n";
  let after = render(
    hand_written,
    &[Generated::new(Sigil::SteelThread, "ST0056")],
  )
  .expect("renders");

  assert_eq!(
    outside_markers(after.as_str()),
    vec!["# just pins so far", "STEELTHREAD:ST0011", ""],
    "the hand-written lines survive; only a separator is added"
  );
  assert_eq!(inside_markers(after.as_str()), vec!["STEELTHREAD:ST0056"]);
  parse(&after).expect("what the writer produces, the parser reads");
}

/// **The round trip is stable.** Rendering the same status twice must produce
/// the same bytes, or `organize` writes a diff on every run and the manifest
/// becomes noise in every commit it appears in.
#[test]
fn rendering_is_idempotent() {
  let status = [Generated::new(Sigil::SteelThread, "ST0057")];
  let once = render(PINNED_AND_AWKWARD, &status).expect("renders");
  let twice = render(&once, &status).expect("renders again");
  assert_eq!(
    once, twice,
    "a second organize with unchanged status is a no-op"
  );
}

/// An unreadable manifest is not rewritten. There is no pinned region to
/// preserve if the file cannot be read, and guessing at one is how a pin is
/// lost -- the exact harm AC-02.3 exists to prevent, arriving through the
/// writer instead of through a status change.
#[test]
fn an_unreadable_manifest_is_refused_rather_than_rewritten() {
  let broken = "STEELTHREAD:ST0011\nTHREAD:ST0056\n";
  let err = render(broken, &[]).expect_err("a manifest that will not parse will not render");
  assert_eq!(err.line(), 2);
}
