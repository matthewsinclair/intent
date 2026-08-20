//! AT-06.1 / AC-06.1 and AT-06.2 / AC-06.2: **the text realisation is COMPLETE
//! and says so with a denominator, and it is regenerable rather than
//! authoritative.**
//!
//! **THE DENOMINATOR IS THE SUBJECT, NOT THE WRITING.** A realisation that
//! writes some files passes any test that asks whether files appeared. The
//! failure this row exists to catch is a PARTIAL realisation that reads as
//! complete -- discovered, by construction, at the moment the tool is gone or
//! wrong and a human has nothing else to consult. So the arms here vary the
//! POPULATION and require the count to follow it.
//!
//! **AND `complete()` NEEDS A NEGATIVE CONTROL.** "It reported complete" is
//! produced equally by a working denominator and by one comparing a number to
//! itself. The tautology arm below is the whole reason `Counts::of` reads canon
//! rather than accumulating while writing.

mod common;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::model::Attachment;
use intentsvcs::realise::{self, Counts, RealiseError};

fn canon_of(threads: Vec<intentsvcs::model::Thread>) -> Canon {
  Canon {
    threads,
    ..Default::default()
  }
}

#[test]
fn every_artefact_canon_holds_reaches_the_realisation() {
  let fx = Fixture::new();
  let canon = canon_of(vec![sample_thread("ST0001"), sample_thread("ST0002")]);
  let root = fx.path("realisation");

  let r = realise::realise(&fx.project(), &canon, &ctx(), &root).expect("realises");

  assert!(
    r.complete(),
    "the realisation must be complete, and it names what fell short: {:?}",
    r.shortfall()
  );
  assert!(r.shortfall().is_empty());
  // **THE POPULATION IS NON-EMPTY, ASSERTED.** Every claim above is satisfied
  // by an estate of nothing, and `Counts::default() == Counts::default()`.
  assert_eq!(r.totals.threads, 2, "the fixture must carry two threads");
  assert!(r.totals.views > 0, "and the renderer must produce views");
  assert!(
    !r.written.is_empty(),
    "and files must actually be on disk -- `complete()` is a count, and a count agrees with an empty tree"
  );
}

#[test]
fn the_denominator_follows_the_estate_rather_than_the_writing() {
  // **THE TAUTOLOGY ARM.** If `Counts::of` accumulated while writing instead of
  // reading canon, every realisation would compare a number to itself and
  // report complete for every input -- including this one, where the two
  // populations differ by design.
  let fx = Fixture::new();
  let small = canon_of(vec![sample_thread("ST0001")]);
  let large = canon_of(vec![
    sample_thread("ST0001"),
    sample_thread("ST0002"),
    sample_thread("ST0003"),
  ]);

  let a = realise::realise(&fx.project(), &small, &ctx(), &fx.path("a")).expect("realises");
  let b = realise::realise(&fx.project(), &large, &ctx(), &fx.path("b")).expect("realises");

  assert_eq!(a.totals.threads, 1);
  assert_eq!(b.totals.threads, 3);
  assert!(
    b.totals.views > a.totals.views,
    "the view denominator must follow the estate too, not be a constant: {} vs {}",
    a.totals.views,
    b.totals.views
  );
  assert!(a.complete() && b.complete());
}

#[test]
fn an_attachment_canon_names_but_carries_no_bytes_is_refused_rather_than_skipped() {
  // **THE SHORTFALL THAT MUST NOT BE SILENT.** Skipping it would produce a
  // realisation counting short with no reason given; writing an empty file
  // would produce one counting COMPLETE over a file the human cannot use.
  // Refusing names the thread and the path.
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  // `Attachment::opaque` with no sidecar loaded: `text` is None and `blob` is
  // dropped by the parse, which is exactly the on-disk state.
  let mut att = Attachment::opaque("diagram.png", b"real bytes".to_vec());
  att.blob = None;
  thread.attachments.push(att);
  let canon = canon_of(vec![thread]);

  let err = realise::realise(&fx.project(), &canon, &ctx(), &fx.path("r"))
    .expect_err("canon naming bytes it does not carry must refuse");
  match &err {
    RealiseError::MissingBytes { thread, path } => {
      assert_eq!(thread, "ST0001");
      assert_eq!(path, "diagram.png");
    }
    other => panic!("wrong refusal: {other:?}"),
  }
  let rendered = err.to_string();
  assert!(rendered.contains("diagram.png"), "got: {rendered}");
}

#[test]
fn a_text_attachment_is_realised_with_its_bytes() {
  // The positive control for the arm above: without it, "refused" is produced
  // equally by a working guard and by a realiser that refuses every attachment.
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread
    .attachments
    .push(Attachment::new("notes.md", "the authored bytes"));
  let canon = canon_of(vec![thread]);

  // **A DELTA, NOT AN ABSOLUTE.** `sample_thread` already carries attachments,
  // so `== 1` asserted a property of the fixture rather than of the realiser --
  // and it would break the day the fixture grew a file, in a test about
  // something else entirely.
  let bare = realise::realise(
    &fx.project(),
    &canon_of(vec![sample_thread("ST0001")]),
    &ctx(),
    &fx.path("bare"),
  )
  .expect("realises");
  let r = realise::realise(&fx.project(), &canon, &ctx(), &fx.path("r")).expect("realises");
  assert_eq!(
    r.totals.attachments,
    bare.totals.attachments + 1,
    "the added attachment must move the denominator by exactly one"
  );
  assert_eq!(r.counts.attachments, r.totals.attachments);
  assert!(r.complete(), "shortfall: {:?}", r.shortfall());

  let written = r
    .written
    .iter()
    .find(|p| p.ends_with("notes.md"))
    .expect("the attachment reaches disk");
  assert_eq!(
    std::fs::read_to_string(written).expect("readable"),
    "the authored bytes"
  );
}

#[test]
fn the_realisation_is_never_seen_by_the_verb_that_reconciles_the_tree() {
  // **AC-06.2, ASKED OF THE SCANNER RATHER THAN OF `classify`.** An earlier
  // draft called `Project::classify` on each realised path and asserted
  // `Unattached`. That was a misuse and the failure taught me the contract:
  // `classify` takes a path relative to a THREAD directory, so handed a
  // project-relative one it saw `cmd-st.md` and correctly answered `Attachment`.
  // **The criterion is not about what `classify` would say if you misled it; it
  // is about whether anything ever hands it a path under `.backup/`.** So this
  // drives the real consumer: `organize` walks the tree, and a realised file
  // must not appear in ANY row of its plan -- not as a view, not as an
  // attachment, and not even as `Unclaimed`, because `Unclaimed` is still the
  // verb reporting on a file that is none of its business.
  let fx = Fixture::new();
  let canon = canon_of(vec![sample_thread("ST0001")]);
  let root = fx
    .project()
    .intent_dir()
    .join(".backup")
    .join("text")
    .join("STAMP");

  let r = realise::realise(&fx.project(), &canon, &ctx(), &root).expect("realises");
  assert!(
    !r.written.is_empty(),
    "the realisation must have written something, or this asserts nothing"
  );

  let (tree, digest) = intentsvcs::organize::observe(&fx.project(), &[]).expect("observes");
  let manifest = intentsvcs::intentfiles::parse("# BEGIN INTENT\n# END INTENT\n").expect("parses");
  let plan = intentsvcs::organize::plan(&fx.project(), &canon, &manifest, &ctx(), &tree, digest);

  for step in &plan.steps {
    assert!(
      !step.path.starts_with(&root),
      "`organize` reached a realised file -- the fallback has a read-back route into the estate: {} ({:?})",
      step.path.display(),
      step.action
    );
  }
  for path in &tree.present {
    assert!(
      !path.starts_with(&root),
      "the tree scan reached a realised file: {}",
      path.display()
    );
  }
}

#[test]
fn two_realisations_do_not_overwrite_each_other() {
  // The artefact is regenerable, and regenerable must not mean "the previous
  // one is gone". A human comparing two points in time is the ordinary use.
  let fx = Fixture::new();
  let canon = canon_of(vec![sample_thread("ST0001")]);

  let a = realise::realise(&fx.project(), &canon, &ctx(), &fx.path("one")).expect("first");
  let b = realise::realise(&fx.project(), &canon, &ctx(), &fx.path("two")).expect("second");

  assert_ne!(a.root, b.root);
  for path in &a.written {
    assert!(path.exists(), "the first realisation survives the second");
  }
  assert_eq!(a.counts, b.counts, "and they realise the same estate");
}

#[test]
fn counts_of_an_empty_estate_is_empty_rather_than_complete_by_accident() {
  // Guards the guard: `complete()` on an empty canon is true, and that is
  // correct -- but it must be visibly EMPTY, so a test asserting completeness
  // over a fixture that silently stopped producing threads cannot pass quietly.
  let empty = Canon::default();
  let c = Counts::of(&empty);
  assert_eq!(c.total(), 0);
}
