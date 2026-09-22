//! AC-05.1 / D57-3: **`intent organize` PREVIEWS BY DEFAULT AND `--apply`
//! PERFORMS**, ruled by ic on 2026-08-19.
//!
//! **THE ROW IS NOT "A PREVIEW CHANGES NOTHING". IT IS "A PREVIEW AND A RUN
//! CANNOT DISAGREE".** The first is easy and nearly worthless: a preview that
//! silently reported fewer removals than the run performs would pass it, and
//! that is the failure with teeth -- an operator reads 3, types `--apply`, and
//! loses 544. So the central arm here runs the SAME plan against the SAME tree
//! in both modes and requires the two reports to be identical, which is only
//! possible if the preview both classified everything the run classifies AND
//! left the world alone for the run to find.
//!
//! **THE POSITIVE CONTROL IS LOAD-BEARING FOR THE SAME REASON IT IS IN THE SHIP
//! GATE.** "The file is still there after a preview" is produced equally by a
//! working preview and by a fixture that stopped producing a removal candidate.
//! Every quiet assertion below is paired with an arm that requires the file to
//! actually go.
//!
//! **AND THE PER-FILE GATE MUST RUN IN PREVIEW.** A preview that skipped it
//! would report every candidate as a removal, including the ones the run will
//! refuse -- the exact number the operator consults the preview to learn.

use std::cell::Cell;

use crate::common::{Fixture, ctx, declaring_thread, gate_open, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::intentfiles;
use intentsvcs::model::{AcKind, AcState, Thread};
use intentsvcs::organize::{Action, Mode, OrganizeError, Plan, Report, TreeState, plan};

/// **ST0057 IS THE THREAD `declaring_thread` BUILDS, AND NAMING IT HERE IS WHAT
/// PUTS A REALISATION AND A REMOVAL IN ONE PLAN.** An earlier draft declared
/// ST0001, which is in no canon these fixtures build -- so the manifest pinned a
/// thread that does not exist, the plan carried removals and nothing else, and
/// `a_preview_writes_nothing_either` asserted over an empty set. Caught by that
/// test's own vacuity guard rather than by review.
const MANIFEST: &str = "\
STEELTHREAD:ST0057

";

fn canon_of(threads: Vec<Thread>) -> Canon {
  Canon {
    threads,
    ..Default::default()
  }
}

/// A tree with exactly one dehydration candidate, materialised from the plan's
/// own render so the per-file gate sees bytes it agrees with.
fn one_removal(fx: &Fixture, declaration: Thread) -> (std::path::PathBuf, Plan) {
  let project = fx.project();
  let canon = canon_of(vec![declaration, sample_thread("ST0002")]);
  let realised = intentfiles::realised_for_action(MANIFEST).expect("manifest parses");
  let doomed = project.info_view("ST0002");
  let tree = TreeState {
    present: [doomed.clone()].into_iter().collect(),
    ..Default::default()
  };
  let p = plan(
    &project,
    &canon,
    &realised,
    &ctx(),
    &tree,
    "digest".to_string(),
  );
  assert!(
    p.with(Action::Dehydrate).any(|s| s.path == doomed),
    "the fixture must produce exactly this dehydration candidate"
  );
  for step in p.with(Action::Dehydrate) {
    let content = step.content.as_ref().expect("a removal carries its render");
    if let Some(parent) = step.path.parent() {
      std::fs::create_dir_all(parent).expect("fixture dirs");
    }
    std::fs::write(&step.path, content).expect("fixture write");
  }
  (doomed, p)
}

/// Every field of a `Report`, rendered deterministically.
///
/// **`Report` cannot derive `PartialEq` -- it carries `Vec<OrganizeError>` --
/// and comparing only the path vectors would silently drop the refusals, which
/// are the half a drifting preview would get wrong.** Rendering the refusals to
/// their display strings compares exactly what the operator is shown.
fn fingerprint(r: &Report) -> String {
  let paths = |v: &Vec<std::path::PathBuf>| {
    let mut s: Vec<String> = v.iter().map(|p| p.display().to_string()).collect();
    s.sort();
    s.join("|")
  };
  let mut refused: Vec<String> = r.refused.iter().map(|e| e.to_string()).collect();
  refused.sort();
  format!(
    "hydrated=[{}]\nrewritten=[{}]\nunchanged=[{}]\ndehydrated=[{}]\nunclaimed=[{}]\ndiverged=[{}]\nrefused=[{}]",
    paths(&r.hydrated),
    paths(&r.rewritten),
    paths(&r.unchanged),
    paths(&r.dehydrated),
    paths(&r.unclaimed),
    paths(&r.diverged),
    refused.join("\n  ")
  )
}

#[test]
fn a_preview_does_not_remove_the_file_that_an_apply_removes() {
  let fx = Fixture::new();
  let (doomed, p) = one_removal(&fx, gate_open());

  let preview = p
    .run(Mode::Preview, &|| "digest".to_string())
    .expect("a preview returns");
  assert!(
    doomed.exists(),
    "PREVIEW REMOVED A FILE. This is the whole polarity ruling: the bare spelling of the verb must not cost anything."
  );
  assert_eq!(
    preview.dehydrated,
    vec![doomed.clone()],
    "the preview must still REPORT the removal it declined to perform -- a preview that reports nothing is indistinguishable from one with nothing to do"
  );

  // **THE CONTROL.** Without it, "the file survives a preview" is equally
  // produced by a fixture that never had a removal in it.
  p.run(Mode::Apply, &|| "digest".to_string())
    .expect("an apply returns");
  assert!(
    !doomed.exists(),
    "CONTROL FAILED: with the gate open, `--apply` must actually remove the file, or the preview arm above proves nothing"
  );
}

#[test]
fn the_preview_and_the_run_report_exactly_the_same_thing() {
  // **THE CENTRAL ROW.** Same plan, same tree, both modes, in order. The
  // reports must be identical -- which can only hold if the preview classified
  // everything the run classifies AND left the world untouched for the run to
  // find. A preview that under-reported removals, skipped the per-file gate, or
  // wrote so much as one file would move one of these seven fields.
  let fx = Fixture::new();
  let (_doomed, p) = one_removal(&fx, gate_open());

  let previewed = fingerprint(
    &p.run(Mode::Preview, &|| "digest".to_string())
      .expect("preview"),
  );
  let performed = fingerprint(&p.run(Mode::Apply, &|| "digest".to_string()).expect("apply"));

  assert_eq!(
    previewed, performed,
    "the preview and the run disagree. An operator reading the preview would act on a number the run does not honour."
  );
}

#[test]
fn a_preview_writes_nothing_either() {
  // The verb realises as well as removes, and a preview withholds BOTH. A
  // preview that quietly hydrated would be the safe-looking half of the split
  // doing exactly what the flag says it will not.
  let fx = Fixture::new();
  let (_doomed, p) = one_removal(&fx, gate_open());

  let hydrating: Vec<_> = p
    .with(Action::Hydrate)
    .chain(p.with(Action::HydrateAttachment))
    .map(|s| s.path.clone())
    .collect();
  assert!(
    !hydrating.is_empty(),
    "the fixture must produce at least one realisation, or this test asserts nothing"
  );

  p.run(Mode::Preview, &|| "digest".to_string())
    .expect("preview");
  for path in &hydrating {
    assert!(!path.exists(), "PREVIEW WROTE A FILE: {}", path.display());
  }

  p.run(Mode::Apply, &|| "digest".to_string()).expect("apply");
  for path in &hydrating {
    assert!(
      path.exists(),
      "CONTROL FAILED: `--apply` must realise {}, or the quiet arm above proves nothing",
      path.display()
    );
  }
}

#[test]
fn a_preview_never_asks_whether_the_tree_moved() {
  // **THE RE-OBSERVATION GUARD IS ABOUT THE MOMENT OF ACTING.** A preview takes
  // no irreversible step, so it has nothing to guard -- and refusing a preview
  // because a peer wrote a file elsewhere would deny the operator the one
  // reading that is always safe to take, on a shared tree where a peer writing
  // is the normal condition rather than the alarm.
  let fx = Fixture::new();
  let (_doomed, p) = one_removal(&fx, gate_open());

  let asked = Cell::new(0usize);
  let report = p
    .run(Mode::Preview, &|| {
      asked.set(asked.get() + 1);
      "A TREE THAT HAS MOVED".to_string()
    })
    .expect("a preview must not refuse over a moved tree");
  assert_eq!(
    asked.get(),
    0,
    "the preview consulted the digest guard; it has no irreversible step to guard"
  );
  assert!(
    !report.dehydrated.is_empty(),
    "and it must still have reported the removal it was previewing"
  );

  // Control: the same moved tree DOES stop the run that acts.
  let asked_on_apply = Cell::new(0usize);
  let err = p
    .run(Mode::Apply, &|| {
      asked_on_apply.set(asked_on_apply.get() + 1);
      "A TREE THAT HAS MOVED".to_string()
    })
    .expect_err("CONTROL FAILED: an apply against a moved tree must refuse");
  assert!(
    matches!(err, OrganizeError::TreeMoved { .. }),
    "got {err:?}"
  );
  assert_eq!(asked_on_apply.get(), 1, "the apply path asks exactly once");
}

#[test]
fn a_preview_reports_the_refusal_rather_than_counting_it_as_a_removal() {
  // **THE NUMBER THE PREVIEW EXISTS TO PRODUCE.** With a precondition unmet the
  // run removes nothing, so a preview claiming a removal would send an operator
  // to `--apply` expecting an effect that cannot happen -- and, in the other
  // direction, a preview that skipped the gate on a healthy estate would
  // promise removals the run will refuse file by file.
  let fx = Fixture::new();
  let (doomed, p) = one_removal(
    &fx,
    declaring_thread(&[
      (
        "AC-00.3",
        AcKind::NonTest,
        AcState::Unsatisfied { note: None },
      ),
      (
        "AC-00.4",
        AcKind::NonTest,
        AcState::Satisfied {
          evidence: "landed".to_string(),
        },
      ),
    ]),
  );

  let report = p
    .run(Mode::Preview, &|| "digest".to_string())
    .expect("preview");
  assert!(doomed.exists(), "the preview must not remove it either");
  assert!(
    report.dehydrated.is_empty(),
    "the preview must not PROMISE a removal the gate will refuse: {:?}",
    report.dehydrated
  );
  let refusal = report
    .refused
    .iter()
    .find(|e| matches!(e, OrganizeError::PreconditionsUnmet { .. }))
    .expect(
      "the preview must carry the refusal -- it is the answer to `what happens if I type --apply`",
    );
  assert!(
    refusal.to_string().contains("AC-00.3"),
    "and it must name the unmet precondition: {refusal}"
  );
}

#[test]
fn the_blocked_count_is_files_and_not_refusals() {
  // **THE DEFECT THIS ROW EXISTS FOR IS AN UNDERSTATEMENT, NOT AN ERROR.**
  // `PreconditionsUnmet` is deliberately ONE refusal for the whole run, so
  // `refused.len()` is 1 whether the gate is holding one removal or four
  // hundred. Every number in that sentence is true and the one a reader sees is
  // the wrong one: cc measured stdout `0 to remove, 1 refused` beside a stderr
  // refusal reading `would remove 423 file(s)`, same run, this estate.
  //
  // **SO THE ASSERTION IS THE INEQUALITY, NOT THE VALUE.** `blocked() == 3`
  // alone would pass against an implementation that returned `dehydrated.len()`,
  // or `refused.len()`, or any other quantity that happened to be 3 in a fixture
  // sized to make them agree. Requiring `blocked() > refused.len()` is the shape
  // of the bug, and it can only hold if the count reaches INSIDE the refusal.
  let fx = Fixture::new();
  let project = fx.project();
  let undeclared = ["ST0002", "ST0003", "ST0004"];
  let mut threads = vec![declaring_thread(&[(
    "AC-00.3",
    AcKind::NonTest,
    AcState::Unsatisfied { note: None },
  )])];
  threads.extend(undeclared.iter().map(|id| sample_thread(id)));

  let tree = TreeState {
    present: undeclared.iter().map(|id| project.info_view(id)).collect(),
    ..Default::default()
  };
  let p = plan(
    &project,
    &canon_of(threads),
    &intentfiles::realised_for_action(MANIFEST).expect("manifest parses"),
    &ctx(),
    &tree,
    "digest".to_string(),
  );
  // **POSITIVE CONTROL, and it is the one this fixture can most easily lose.**
  // A manifest that silently declared these, or a canon that did not carry them,
  // yields zero candidates -- and then `blocked() == 0 == refused.len()` fails
  // the inequality for a reason that has nothing to do with the count.
  assert_eq!(
    p.with(Action::Dehydrate).count(),
    undeclared.len(),
    "the fixture must produce one candidate per undeclared thread"
  );
  for step in p.with(Action::Dehydrate) {
    let content = step.content.as_ref().expect("a removal carries its render");
    if let Some(parent) = step.path.parent() {
      std::fs::create_dir_all(parent).expect("fixture dirs");
    }
    std::fs::write(&step.path, content).expect("fixture write");
  }

  let report = p
    .run(Mode::Preview, &|| "digest".to_string())
    .expect("preview");
  assert_eq!(
    report.refused.len(),
    1,
    "the gate speaks once for the whole run -- if this is 3, the refusal was made per-file and this row is measuring something else: {:?}",
    report.refused
  );
  assert_eq!(
    report.blocked(),
    undeclared.len(),
    "every gated removal must be counted, not the refusal carrying them"
  );
  assert!(
    report.blocked() > report.refused.len(),
    "the count a reader is shown must not collapse {} files into {} refusal",
    report.blocked(),
    report.refused.len()
  );
}

#[test]
fn a_run_the_gate_does_not_block_reports_nothing_blocked() {
  // **THE OTHER HALF, AND WITHOUT IT THE ROW ABOVE IS SATISFIED BY A CONSTANT.**
  // `blocked()` returning the candidate count unconditionally would pass every
  // assertion in the previous test. Here the preconditions are met, the same
  // shape of estate produces the same removals, and the figure must be zero --
  // so the number has to be reading the GATE and not the plan.
  let fx = Fixture::new();
  let (doomed, p) = one_removal(&fx, gate_open());
  let report = p
    .run(Mode::Preview, &|| "digest".to_string())
    .expect("preview");
  assert!(
    p.with(Action::Dehydrate).any(|s| s.path == doomed),
    "the fixture must still carry a removal, or zero is vacuous"
  );
  assert_eq!(
    report.blocked(),
    0,
    "nothing is held back when the gate permits: {:?}",
    report.refused
  );
}

#[test]
fn the_unclaimed_digest_moves_on_membership_and_not_on_order() {
  // **THE ROW EXISTS FOR THE CASE THE COUNT CANNOT SEE.** vc measured both arms
  // on the live estate: ADDING an unclaimed file moves `199 unclaimed` to `200`
  // and was already visible in the line the summary always printed; SWAPPING
  // one file for another inside a single directory left the whole output
  // byte-identical, with the changed entry at position 2 of 199. Grouping the
  // report by directory -- the first fix -- fails that swap for the same reason
  // the count does: same directory, same cardinality.
  // **ROOT-PREFIXED, BECAUSE THAT IS THE SHAPE THE WALK PRODUCES.** `Step.path`
  // is built from `self.root.join(..)`, so a fixture of bare relative literals
  // was testing a `Report` production never builds -- which is how it could
  // assert both halves below and still miss issue 0509 entirely.
  let root = std::path::PathBuf::from("/estate");
  let report = |paths: &[&str]| Report {
    unclaimed: paths.iter().map(|p| root.join(p)).collect(),
    ..Default::default()
  };

  let before = report(&["a/one.tap", "a/two.tap", "b/three.tap"]);
  let swapped = report(&["a/one.tap", "a/CHANGED.tap", "b/three.tap"]);
  let reordered = report(&["b/three.tap", "a/two.tap", "a/one.tap"]);

  // The swap is invisible to everything the summary carried before this.
  assert_eq!(
    before.unclaimed.len(),
    swapped.unclaimed.len(),
    "the fixture must hold cardinality constant, or it is testing the count"
  );
  assert_ne!(
    before.unclaimed_digest(&root),
    swapped.unclaimed_digest(&root),
    "a same-directory swap must move the digest -- it is the only thing that can see it"
  );

  // **AND THE OTHER HALF, without which the digest is a nuisance rather than an
  // instrument.** If walk order moved it, the detector would fire on the walk
  // instead of on the estate and a reader would learn to ignore it in a day.
  assert_eq!(
    before.unclaimed_digest(&root),
    reordered.unclaimed_digest(&root),
    "the same SET in a different order is the same set"
  );

  // Twelve hex characters, so the summary line stays readable.
  let d = before.unclaimed_digest(&root);
  assert_eq!(d.len(), 12, "digest should be 12 chars, got {d}");
  assert!(
    d.chars().all(|c| c.is_ascii_hexdigit()),
    "digest should be hex: {d}"
  );
}

/// A real `Report` from a real project at a real root, carrying exactly the
/// unclaimed files named.
///
/// **BUILT THROUGH `plan` AND `run` RATHER THAN CONSTRUCTED**, because the
/// question below is about what the WALK puts in the report, and a hand-built
/// `Report` is free to hold whatever shape the test wants it to.
fn unclaimed_report(fx: &Fixture, names: &[&str]) -> (std::path::PathBuf, Report) {
  let project = fx.project();
  let present: std::collections::BTreeSet<std::path::PathBuf> = names
    .iter()
    .map(|n| project.st_dir().join("ST0002").join(n))
    .collect();
  let p = plan(
    &project,
    &canon_of(vec![sample_thread("ST0002")]),
    &intentfiles::realised_for_action(MANIFEST).expect("manifest parses"),
    &ctx(),
    &TreeState {
      present,
      ..Default::default()
    },
    "digest".to_string(),
  );
  let report = p
    .run(Mode::Preview, &|| "digest".to_string())
    .expect("a preview returns");
  assert_eq!(
    report.unclaimed.len(),
    names.len(),
    "the fixture must put every named file in the unclaimed set, or the digest below is over the wrong population: {:?}",
    report.unclaimed
  );
  (project.root().to_path_buf(), report)
}

#[test]
fn the_unclaimed_digest_does_not_move_when_the_tree_does() {
  // **THIS IS THE ARM THAT WOULD HAVE FAILED BEFORE ISSUE 0509 WAS FIXED, and
  // its absence is why the defect survived.** The membership-and-order test
  // above builds its `Report` from string literals, so it never holds a project
  // root and cannot move a tree; it can assert both of its halves forever while
  // the digest silently answers a second question nobody asked it.
  //
  // The digest's job is "did this SET change". A project that is COPIED or
  // MOVED has the same set. Hashing the paths as stored -- root-prefixed --
  // made the digest change on the move, so a reader could not tell an estate
  // that had gained a file from one that had merely been relocated.
  let a = Fixture::new();
  let b = Fixture::new();
  let files = ["diagram.png", "notes.tap", "sub/extra.png"];

  let (root_a, report_a) = unclaimed_report(&a, &files);
  let (root_b, report_b) = unclaimed_report(&b, &files);

  // **THE VACUITY GUARD, and it is not ceremony here.** `Fixture::new` is a
  // `tempfile::tempdir()`, so two fixtures are two different roots -- but if
  // that ever stopped being true the assertion below would pass for the one
  // reason that proves nothing, and it would pass quietly.
  assert_ne!(
    root_a, root_b,
    "the two fixtures must sit at DIFFERENT roots, or 'the digest survived the move' is true because nothing moved"
  );
  assert_ne!(
    report_a.unclaimed, report_b.unclaimed,
    "the stored paths must differ between the two roots -- if they are equal the digest cannot distinguish the shapes and this test is measuring nothing"
  );
  // **AND THE DENOMINATOR, which is the guard the root check does not give**
  // (vc, 2026-09-22). Two digests over two EMPTY sets are also equal, so a
  // fixture whose `Unclaimed` rows stopped materialising -- for any reason
  // having nothing to do with this digest -- would satisfy the assertion below
  // for the one reason that proves nothing, and would do it quietly.
  assert!(
    !report_a.unclaimed.is_empty() && !report_b.unclaimed.is_empty(),
    "both reports must carry unclaimed files: a=[{:?}] b=[{:?}]",
    report_a.unclaimed,
    report_b.unclaimed
  );

  assert_eq!(
    report_a.unclaimed_digest(&root_a),
    report_b.unclaimed_digest(&root_b),
    "THE SAME SET AT A DIFFERENT ROOT IS THE SAME SET. A digest that moves when the project moves answers 'did this set change, or did the tree move', and nothing on the summary line distinguishes the two"
  );

  // **AND THE OTHER POLARITY, at a real root rather than a constructed one**,
  // so relativising cannot have been bought by flattening every path to
  // something that no longer discriminates -- a basename, say.
  let (root_c, report_c) = unclaimed_report(&b, &["diagram.png", "notes.tap", "sub/other.png"]);
  assert_ne!(
    report_a.unclaimed_digest(&root_a),
    report_c.unclaimed_digest(&root_c),
    "CONTROL FAILED: a changed member must still move the digest once the root is stripped"
  );
}
