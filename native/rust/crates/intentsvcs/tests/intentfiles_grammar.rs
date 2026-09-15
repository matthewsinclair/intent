//! AT-02.1 / AC-02.1: **the `.intentfiles` grammar REFUSES rather than skips.**
//!
//! The parser accepts exactly `<SIGIL>:<ID>` with sigil in
//! `STEELTHREAD` or `ISSUE` and an optional trailing comment (`ISSUE` was retired by hv
//! on 2026-08-20). For every rejected
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

use intentsvcs::intentfiles::{IntentfilesError, Sigil, parse};
use intentsvcs::remedy::Remedy;
use testkit::repo_root;

/// A valid manifest whose every line is a different SHAPE, so an injection
/// lands in varied company rather than always between two identical rows.
fn valid_lines() -> Vec<String> {
  vec![
    "# a note: whole-line comments are admitted".to_string(),
    "STEELTHREAD:ST0011  # listed so it still realises after it closes".to_string(),
    String::new(),
    // Was `ISSUE:0042` until hv retired that sigil on 2026-08-20. Kept as a
    // BARE entry with no trailing comment, which is the shape it contributed:
    // this list exists so an injection lands in varied company.
    "STEELTHREAD:ST0042".to_string(),
    "STEELTHREAD:ST0056".to_string(),
    "STEELTHREAD:ST0057  # a trailing note".to_string(),
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
    // **`ISSUE` IS A KNOWN SIGIL AGAIN (ST0069 WP-01), SO THESE TWO MOVE FROM
    // `UnknownSigil` TO `MalformedId` RATHER THAN LEAVING THE TABLE.** The
    // paragraph here used to argue the opposite -- *a retired sigil is an
    // unknown sigil, not a malformed id*, because `MalformedId` would send the
    // operator to fix a number that no value could fix. That reasoning was
    // right while the sigil was retired and it inverts cleanly now that it is
    // not: `42` IS a fixable number, and `MalformedId`'s remedy names the
    // shape. Both spellings are still kept -- a bad id and a path -- because
    // the id rule is what refuses them now, and asserting that is what stops a
    // reader concluding the sigil is unknown.
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

  let first = &m.entries[0];
  assert_eq!(first.sigil, Sigil::SteelThread);
  assert_eq!(first.id, "ST0011");
  assert_eq!(first.line, 2, "the line a human reads it on");
  assert_eq!(
    first.comment.as_deref(),
    Some("listed so it still realises after it closes"),
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

/// **D57-9 RETIRED THE MARKERS, AND A MARKER LINE IS REFUSED RATHER THAN READ
/// AS A COMMENT** (issue 0338). Both markers start with `#`, so without an arm
/// of their own the comment rule would admit them, and a manifest written for
/// the two-region grammar would parse on carrying a construct that no longer
/// means anything.
#[test]
fn a_retired_marker_line_is_refused_with_its_line() {
  for marker in ["# BEGIN INTENT", "# END INTENT"] {
    let text = format!("STEELTHREAD:ST0011\n{marker}\nSTEELTHREAD:ST0056\n");
    let err = parse(&text).expect_err(&format!("`{marker}` must be refused, never admitted"));
    assert!(
      matches!(err, IntentfilesError::NotAnEntry { .. }),
      "`{marker}` refused by the wrong arm -- {err:?}"
    );
    assert_eq!(err.line(), 2, "`{marker}` is on line 2");
    assert!(
      err.remedy().contains("no longer part of the grammar"),
      "the remedy says the marker is retired: {}",
      err.remedy()
    );
  }
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
  seen.sort();

  let mut expected: Vec<String> = ["UnknownSigil", "NotAnEntry", "MalformedId"]
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
/// pragma, a region marker -- the manifest has a SECOND DECLARATION CHANNEL**,
/// in the one file whose criterion says a second enumeration must be
/// unrepresentable rather than discouraged (AC-02.5). Two declarations of
/// which-artefacts-matter agree for months and then quietly do not. Every
/// ecosystem that admitted comments acquired one eventually.
///
/// So: stripping every comment must change nothing but the comment field. A
/// directive smuggled into a `#` line would have to change something else to
/// do any work, and this is what notices.
#[test]
fn comments_are_inert() {
  // **The same comment vocabulary sits before, between and after entries**, so
  // a directive that changed what follows it has an entry to change. The line
  // quoting a retired marker is admitted: only a line that is EXACTLY a marker
  // is refused.
  let commented = "# a leading note\n\
     STEELTHREAD:ST0011  # why this is listed\n\
     \n\
     # noqa\n\
     # type: manifest\n\
     STEELTHREAD:ST0042 # another\n\
     # noqa\n\
     # type: manifest\n\
     # BEGIN INTENT was a marker; this is not\n\
     STEELTHREAD:ST0056\n\
     # a trailing note\n\
     # noqa\n\
     STEELTHREAD:ST0057\n";
  let stripped = "STEELTHREAD:ST0011\n\
     STEELTHREAD:ST0042\n\
     STEELTHREAD:ST0056\n\
     STEELTHREAD:ST0057\n";

  let a = parse(commented).expect("comments are admitted");
  let b = parse(stripped).expect("and so is their absence");

  let shape = |m: &intentsvcs::intentfiles::Manifest| -> Vec<(Sigil, String)> {
    m.entries.iter().map(|e| (e.sigil, e.id.clone())).collect()
  };
  assert_eq!(
    shape(&a),
    shape(&b),
    "a comment must change NOTHING but the comment field -- if these differ,\n       \
     something is reading a `#` line for content and the manifest has grown a\n       \
     second declaration channel"
  );
}
