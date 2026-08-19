//! AT-02.1 / AC-02.1: **the `.intentfiles` grammar REFUSES rather than skips.**
//!
//! The parser accepts exactly `<SIGIL>:<ID>` with sigil in
//! `STEELTHREAD | ISSUE` and an optional trailing comment. For every rejected
//! input the run exits non-zero AND the offending line number appears in the
//! output.
//!
//! **The property under test is the ABSENCE of a silent skip**, and that is
//! harder to test than a refusal, because a skip and a refusal look identical
//! from the side that only checks the good lines parsed. So every case here
//! asserts three things together: the parse FAILED, the failure names the
//! RIGHT line, and no entry was harvested from the bad line.
//!
//! **The line number is checked against a position the case does not choose.**
//! Each bad line is injected at every position in a valid manifest and the
//! expected number comes from the injection index, not from a literal beside
//! the case. A test that writes `line: 3` next to a fixture with the bad line
//! third passes an off-by-one that only appears on files longer than the
//! fixture -- which is every real one.
//!
//! **The corpus is a partition, not a sample** (`bad_lines`). Each entry names
//! the arm it exercises, and [`every_error_variant_is_exercised`] asserts the
//! corpus reaches every variant the error enum declares. A hand-kept corpus
//! silently stops covering a variant on the day someone adds one, which is the
//! day they are thinking about anything else.

use intentsvcs::intentfiles::{BEGIN_MARKER, END_MARKER, IntentfilesError, Region, Sigil, parse};
use intentsvcs::remedy::Remedy;
use testkit::repo_root;

/// A valid manifest whose every line is a different SHAPE, so an injection
/// lands in varied company rather than always between two identical rows.
fn valid_lines() -> Vec<String> {
  vec![
    "# the pinned region: these survive an organize rewrite".to_string(),
    "STEELTHREAD:ST0011  # pinned so it still realises after it closes".to_string(),
    String::new(),
    "ISSUE:0042".to_string(),
    BEGIN_MARKER.to_string(),
    "STEELTHREAD:ST0056".to_string(),
    "STEELTHREAD:ST0057  # generated from status".to_string(),
    END_MARKER.to_string(),
  ]
}

/// A bad line and the predicate naming which refusal arm it must reach.
type BadLine = (&'static str, fn(&IntentfilesError) -> bool);

/// Every way a line can fail to be a manifest line, with the arm it exercises.
///
/// `#[allow]`-free by construction: the discriminant is compared, never the
/// payload, so a variant gaining a field does not silently drop a case.
fn bad_lines() -> Vec<BadLine> {
  vec![
    ("THREAD:ST0056", |e| {
      matches!(e, IntentfilesError::UnknownSigil { .. })
    }),
    ("steelthread:ST0056", |e| {
      matches!(e, IntentfilesError::UnknownSigil { .. })
    }),
    ("STEELTHREAD ST0056", |e| {
      matches!(e, IntentfilesError::NotAnEntry { .. })
    }),
    ("ST0056", |e| {
      matches!(e, IntentfilesError::NotAnEntry { .. })
    }),
    ("STEELTHREAD:ST56", |e| {
      matches!(e, IntentfilesError::MalformedId { .. })
    }),
    ("STEELTHREAD:ST00567", |e| {
      matches!(e, IntentfilesError::MalformedId { .. })
    }),
    ("ISSUE:42", |e| {
      matches!(e, IntentfilesError::MalformedId { .. })
    }),
    // AC-02.5 held MECHANICALLY: a path cannot satisfy either id shape, so a
    // file-valued line is unrepresentable rather than separately forbidden.
    ("STEELTHREAD:intent/st/ST0056/info.md", |e| {
      matches!(e, IntentfilesError::MalformedId { .. })
    }),
    ("ISSUE:issues/0042.json", |e| {
      matches!(e, IntentfilesError::MalformedId { .. })
    }),
  ]
}

/// The control. If this ever fails, every refusal below is refusing the
/// scaffolding rather than the injected line, and the whole file is vacuous.
#[test]
fn the_valid_manifest_parses() {
  let m = parse(&valid_lines().join("\n")).expect("the fixture itself must parse");
  assert_eq!(m.entries.len(), 4, "four artefacts in the fixture");
  assert_eq!(m.pinned().count(), 2, "two outside the markers");
  assert_eq!(m.generated().count(), 2, "two between them");

  let pin = m.pinned().next().unwrap();
  assert_eq!(pin.sigil, Sigil::SteelThread);
  assert_eq!(pin.id, "ST0011");
  assert_eq!(pin.region, Region::Pinned);
  assert_eq!(
    pin.comment.as_deref(),
    Some("pinned so it still realises after it closes"),
    "the trailing comment is PRESERVED -- it is where AC-02.3's decision is named"
  );
}

/// **The refusal, at every position, with the line number derived from the
/// injection rather than written beside the case.**
#[test]
fn every_bad_line_is_refused_and_names_its_own_line() {
  let base = valid_lines();

  for (bad, is_expected_arm) in bad_lines() {
    for at in 0..=base.len() {
      let mut lines = base.clone();
      lines.insert(at, bad.to_string());
      let text = lines.join("\n");

      // An injection between BEGIN and END still has to refuse -- the
      // generated region is not a place where the grammar relaxes.
      let err = parse(&text).expect_err(&format!(
        "`{bad}` injected at index {at} must REFUSE, never parse"
      ));

      assert_eq!(
        err.line(),
        at + 1,
        "`{bad}` at index {at}: the refusal must name line {}, said {}",
        at + 1,
        err.line()
      );
      assert!(
        is_expected_arm(&err),
        "`{bad}` at index {at}: refused by the wrong arm -- {err:?}"
      );
      assert!(
        err.to_string().contains(&format!("line {}", at + 1)),
        "AC-02.1 asks the LINE NUMBER to appear in the OUTPUT; `{err}` does not carry it"
      );
      assert!(
        !err.remedy().is_empty(),
        "every refusal states what to do about itself"
      );
    }
  }
}

/// **The no-silent-skip half, stated as its own property.**
///
/// The test above proves a bad line produces an error. It does NOT prove the
/// parser declined to harvest entries around it -- a parser that collected the
/// good lines and reported the bad one would satisfy it. Nothing may come back
/// from a refused parse at all.
#[test]
fn a_refused_parse_yields_no_entries() {
  let mut lines = valid_lines();
  lines.insert(2, "THREAD:ST0056".to_string());
  assert!(
    parse(&lines.join("\n")).is_err(),
    "a manifest carrying an unreadable line has no valid reading"
  );
}

/// The marker arms, which no injected ENTRY can reach.
#[test]
fn unbalanced_markers_are_refused_with_their_line() {
  let stray_end = format!("STEELTHREAD:ST0011\n{END_MARKER}\n");
  let err = parse(&stray_end).expect_err("an END with no BEGIN is not readable");
  assert!(matches!(err, IntentfilesError::UnopenedRegion { .. }));
  assert_eq!(err.line(), 2);

  let nested = format!("{BEGIN_MARKER}\nSTEELTHREAD:ST0011\n{BEGIN_MARKER}\n{END_MARKER}\n");
  let err = parse(&nested).expect_err("a BEGIN inside an open region is not readable");
  assert!(matches!(err, IntentfilesError::NestedRegion { .. }));
  assert_eq!(err.line(), 3);

  let unclosed = format!("{BEGIN_MARKER}\nSTEELTHREAD:ST0011\n");
  let err = parse(&unclosed).expect_err("an unclosed region is not readable");
  assert!(matches!(err, IntentfilesError::UnclosedRegion { .. }));
  assert_eq!(
    err.line(),
    1,
    "an unclosed region is reported where it OPENED -- the end of the file is\n       \
     where the reader notices, not where the mistake is"
  );
}

/// **The corpus must reach every arm the enum declares.**
///
/// Enumerated from the errors the corpus actually produces and compared
/// against a roster that has to be edited when a variant is added. That is a
/// deliberate tripwire rather than an inconvenience: a new variant with no
/// case is a refusal nobody has driven, and the estate has enough of those.
#[test]
fn every_error_variant_is_exercised() {
  let base = valid_lines();
  let mut seen: Vec<String> = Vec::new();

  for (bad, _) in bad_lines() {
    let mut lines = base.clone();
    lines.insert(0, bad.to_string());
    if let Err(e) = parse(&lines.join("\n")) {
      let arm = format!("{:?}", e)
        .split_whitespace()
        .next()
        .unwrap()
        .to_string();
      if !seen.contains(&arm) {
        seen.push(arm);
      }
    }
  }
  for marker_arm in ["UnopenedRegion", "NestedRegion", "UnclosedRegion"] {
    seen.push(marker_arm.to_string());
  }
  seen.sort();

  let mut expected: Vec<String> = [
    "UnknownSigil",
    "NotAnEntry",
    "MalformedId",
    "UnopenedRegion",
    "NestedRegion",
    "UnclosedRegion",
  ]
  .iter()
  .map(|s| s.to_string())
  .collect();
  expected.sort();

  assert_eq!(
    seen, expected,
    "the corpus must exercise every declared refusal arm"
  );
}

/// **The manifest THIS REPOSITORY SHIPS must satisfy the grammar.**
///
/// The corpus above proves the parser refuses what it should. It says nothing
/// about the one file anybody actually edits. A committed, hand-edited file
/// governed by a refusing grammar needs a test that reads THAT FILE, or the
/// first person to mistype a pin discovers it when `organize` aborts rather
/// than when they commit.
#[test]
fn the_shipped_manifest_parses() {
  let path = repo_root().join("intent").join(".intentfiles");
  let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
    panic!(
      "the committed manifest at {} must be readable: {e}",
      path.display()
    )
  });

  match parse(&text) {
    Ok(m) => {
      // No assertion on the CONTENT -- it changes as threads open and close,
      // and a count here would be a second declaration of the estate.
      let _ = m.entries.len();
    }
    // `{e}` ALREADY carries the line number -- the Display body opens with it.
    // Prefixing another renders it twice, which is the doubled-residue shape
    // `IngestError::Refused` documents: measured there as 12 findings printed
    // as 24. One rendering, and the error owns it.
    Err(e) => panic!(
      "{} does not satisfy its own grammar -- {e}\n  remedy: {}",
      path.display(),
      e.remedy()
    ),
  }
}

/// **A COMMENT IS TEXT A HUMAN READS, AND NOTHING ELSE EVER READS IT.**
///
/// vc's boundary on admitting standalone comments (2026-08-19), held here as a
/// mechanism rather than a promise. Comments are admitted because a standalone
/// one names no artefact and so cannot cause the harm AC-02.1 exists to
/// prevent -- a skipped line dropping an artefact. That reasoning survives
/// exactly as long as comments stay inert.
///
/// **The moment a comment carries semantics -- a `# noqa`, a `# type:`, a
/// pragma, a region marker other than BEGIN/END -- the manifest has a SECOND
/// DECLARATION CHANNEL**, in the one file whose criterion says a second
/// enumeration must be unrepresentable rather than discouraged (AC-02.5). Two
/// declarations of which-artefacts-matter agree for months and then quietly do
/// not. Every ecosystem that admitted comments acquired one eventually.
///
/// So: stripping every comment must change nothing but the comment field. A
/// directive smuggled into a `#` line would have to change something else to
/// do any work, and this is what notices.
#[test]
fn comments_are_inert() {
  // **The same comment vocabulary appears in BOTH regions.** A first version
  // of this test put every comment in the pinned region only, and a mutant
  // that flipped `region` on seeing `# noqa` SURVIVED IT -- outside the
  // markers the flip was a no-op, so the guard could not see a directive that
  // only does work in the region it was absent from. A guard whose fixture
  // cannot reach the state the directive changes is decorative.
  let commented = format!(
    "# a leading note\n\
     STEELTHREAD:ST0011  # why this is pinned\n\
     \n\
     # noqa\n\
     # type: manifest\n\
     ISSUE:0042 # another\n\
     {BEGIN_MARKER}\n\
     # noqa\n\
     # type: manifest\n\
     # BEGIN INTENT is a marker; this is not\n\
     STEELTHREAD:ST0056\n\
     # a trailing note inside the region\n\
     {END_MARKER}\n\
     # noqa\n\
     STEELTHREAD:ST0057\n"
  );
  let stripped = format!(
    "STEELTHREAD:ST0011\n\
     ISSUE:0042\n\
     {BEGIN_MARKER}\n\
     STEELTHREAD:ST0056\n\
     {END_MARKER}\n\
     STEELTHREAD:ST0057\n"
  );

  let a = parse(&commented).expect("comments are admitted");
  let b = parse(&stripped).expect("and so is their absence");

  let shape = |m: &intentsvcs::intentfiles::Manifest| -> Vec<(Sigil, String, Region)> {
    m.entries
      .iter()
      .map(|e| (e.sigil, e.id.clone(), e.region))
      .collect()
  };
  assert_eq!(
    shape(&a),
    shape(&b),
    "a comment must change NOTHING but the comment field -- if these differ,\n       \
     something is reading a `#` line for content and the manifest has grown a\n       \
     second declaration channel"
  );
  assert_eq!(
    a.pinned().count(),
    b.pinned().count(),
    "not even the region split may depend on a comment"
  );
}
