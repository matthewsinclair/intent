//! **THE OP VOCABULARY HAS AN OPEN DESIGN QUESTION, AND THIS IS THE HALF OF IT
//! THAT CAN BE ANSWERED WITH A MEASUREMENT** (vc's Highlander finding F1;
//! built under vc's pen, 2026-08-27).
//!
//! Whether `op` should become a TYPE is hv's to rule on and nothing here
//! pre-empts it: there is no enum, no `FromStr`, and nothing refuses an op for
//! being unknown. The argument for a type turns on one factual question --
//! **would a parse ever meet a string it does not know** -- and that answer does
//! not live in the source. It lives in the event logs of real estates, so it
//! kept being re-derived by hand, once per person who asked.
//!
//! `doctor` now answers it on every run, on whatever estate is in front of you.
//!
//! # Three mechanisms, because each is blind where the others see
//!
//! | mechanism                                       | catches                                              | blind to                                    |
//! | ------------------------------------------------ | ----------------------------------------------------- | -------------------------------------------- |
//! | the live check (`doctor`)                        | an op in a real log this build does not declare       | anything never written on this machine      |
//! | `known_ops_are_spelled_in_the_source...`         | an op RETIRED or RENAMED and left in the roster       | a new op nobody added                       |
//! | `every_transition_op_is_in_the_roster`           | a new `st.*` edge nobody added to the roster          | a new op in a family with no state machine  |
//!
//! **THE UNCOVERED CORNER IS NAMED RATHER THAN LEFT TO BE FOUND:** a brand-new
//! `disk.*` or `issues.*` op, added and not rostered, is caught by none of the
//! three at the moment it is written. It is caught by the FIRST one the moment
//! it is USED -- the live check reports it -- which is loud, in the safe
//! direction, and one line to fix.
//!
//! # The retirement direction is the one that matters, and it is the quiet one
//!
//! An op that is renamed leaves history behind under the old name. The code
//! stops producing it; the log keeps carrying it; and **the roster goes on
//! listing a string nothing writes**, so the live check sees a match and says
//! nothing. That is precisely the state in which a future `FromStr` would start
//! failing on old rows, and it is invisible from the log side alone. Only the
//! source-reading test can see it.
//!
//! # Mutations, measured -- each revert re-run to a green baseline
//!
//! | mutation                                        | reds                                                    |
//! | ------------------------------------------------ | -------------------------------------------------------- |
//! | `st.start` dropped from the roster               | the silence test AND `every_transition_op_...`          |
//! | the check filters to nothing, so it never reports| the positive control AND the verdict test               |
//! | a retired op (`zz.retired`) left in the roster   | `known_ops_are_spelled_...` ONLY                        |
//! | the advisory becomes an actionable class         | the positive control AND the verdict test               |
//! | the source reader stops excluding `event.rs`     | **NOTHING**                                             |
//!
//! **THE LAST ROW IS NOT A FAILURE OF THE CONTROL, IT IS THE CONTROL'S RESULT,
//! AND IT NEEDED A SECOND MUTATION TO INTERPRET.** Removing the exclusion on
//! its own reds nothing, because with an honest roster every op is spelled at a
//! real call site anyway -- so the two readers agree on today's input and the
//! mutation is invisible. Run it TOGETHER with a retired op and the retirement
//! test goes GREEN on an op that is genuinely retired: the roster finds its own
//! entry in `KNOWN_OPS` and reports itself healthy.
//!
//! So the exclusion is load-bearing, and **no single-mutation table could have
//! shown that**. A one-at-a-time sweep would have recorded "reds nothing"
//! beside it and left a reader to conclude it could be deleted.
//!
//! # A note on two family counts that are not the same number
//!
//! The roster spans 11 families; the corpus vc measured is 22 ops in 8. **The
//! op count is deliberately not restated here** -- it was, in three places
//! across two files, and two new ops made all three wrong in one commit with
//! nothing to notice. `KNOWN_OPS` is the count.
//! **Neither is wrong and they answer different questions** -- how many families
//! this build CAN write, against how many have ever BEEN written on one machine.
//! A vocabulary is always at least as wide as its use. The corpus figure was
//! nearly written into the roster's own doc comment as though it described the
//! code.

mod common;

use common::Fixture;
use intentsvcs::doctor;
use intentsvcs::event::{Envelope, KNOWN_OPS, Subject};
use intentsvcs::finding::FindingClass;
use intentsvcs::transitions::{Disposition, find};
use std::collections::BTreeSet;

/// The undeclared-op advisories in a doctor run over this fixture's store.
fn undeclared(fx: &Fixture) -> Vec<String> {
  let facade = fx.facade_on_disk();
  let project = fx.project();
  let ctx = common::ctx();
  let report = doctor::diagnose(&project, &ctx, Some(facade.store()));
  report
    .findings
    .into_iter()
    .filter(|f| {
      f.class == FindingClass::Advisory && f.detail.contains("this build does not declare")
    })
    .map(|f| f.detail)
    .collect()
}

/// Put a row in the log under `op`, bypassing every verb.
///
/// **DELIBERATELY NOT THROUGH A VERB, BECAUSE NO VERB CAN PRODUCE THE SUBJECT
/// OF THIS TEST.** An op this build does not declare is, by definition, one no
/// code path here writes -- so the only honest way to stage it is to append the
/// envelope directly. That is also exactly how such a row arrives in real life:
/// written by a DIFFERENT build, some versions ago.
fn plant(fx: &Fixture, op: &str) {
  let facade = fx.facade_on_disk();
  let envelope = Envelope::minted(
    "local",
    common::PROJECT_ID,
    op,
    Subject {
      kind: "paths".to_string(),
      id: common::PROJECT_ID.to_string(),
    },
    serde_json::json!({}),
  );
  facade
    .store()
    .append_event(&envelope)
    .expect("append the planted envelope");
}

// ---------------------------------------------------------------------------
// THE LIVE CHECK, DRIVEN TO BOTH VERDICTS
// ---------------------------------------------------------------------------

/// **THE POSITIVE CONTROL, AND IT IS THE ARM THAT MAKES THE OTHER ONE MEAN
/// ANYTHING.**
///
/// A check that reports `0 undeclared` without ever having been shown it can
/// report a non-zero is decoration -- and this estate has already paid for that
/// once today, on an `events.jsonl` reading that was true, blind, and zero.
/// The invented op is what proves the instrument has a working needle.
#[test]
fn an_op_the_roster_does_not_declare_is_reported_with_its_count() {
  let fx = Fixture::new();
  plant(&fx, "st.invented");
  plant(&fx, "st.invented");

  let found = undeclared(&fx);
  assert_eq!(
    found.len(),
    1,
    "one advisory per distinct op, not per row: {found:?}"
  );
  assert!(
    found[0].contains("st.invented"),
    "the advisory must name the op, or the operator cannot act on it: {found:?}"
  );
  assert!(
    found[0].contains("2 row(s)"),
    "the advisory must carry the count, which separates one stray row from a \
     decade of them: {found:?}"
  );
}

/// **AND A LOG OF DECLARED OPS ONLY MUST BE SILENT.**
///
/// Driven through real verbs rather than planted, so the ops are the ones the
/// facade actually writes. Alone this test is vacuous -- it passes under a
/// check that reports nothing ever -- which is why the control above is first.
#[test]
fn a_log_of_declared_ops_only_says_nothing() {
  let fx = Fixture::new();
  {
    let mut facade = fx.facade_on_disk();
    facade.st_new("a thread").expect("st.new");
    facade.st_triage("ST0001").expect("st.triage");
    facade.st_start("ST0001").expect("st.start");
  }
  let census = fx.facade_on_disk().store().op_census().expect("census");
  assert!(
    census.len() >= 3,
    "the fixture must actually have written ops, or the silence below is about \
     an empty log: {census:?}"
  );
  assert_eq!(
    undeclared(&fx),
    Vec::<String>::new(),
    "every op these verbs write is in the roster, so nothing is owed: {census:?}"
  );
}

/// **THE ADVISORY MUST NOT MOVE THE VERDICT.**
///
/// Every estate with history would otherwise start failing `doctor` for holding
/// a true record of its own past -- the failure hv already ruled on when 66
/// advisory notes made a pristine doctor unreachable on Baize.
///
/// **THE CLAIM IS A DELTA, NOT AN ABSOLUTE, AND THE FIRST VERSION OF THIS TEST
/// GOT THAT WRONG.** It asserted the whole report exits 0, which failed -- a
/// bare fixture already carries three actionable findings of its own (a stale
/// backup and two missing views) that have nothing to do with ops. Asserting an
/// absolute made the test about the fixture's unrelated health; asserting the
/// DIFFERENCE makes it about the only thing this check contributes. Same
/// mistake, same day, as an assertion written about a refusal that belonged to
/// a neighbouring criterion.
#[test]
fn an_undeclared_op_adds_nothing_to_the_verdict() {
  let clean = Fixture::new();
  let before = {
    let facade = clean.facade_on_disk();
    doctor::diagnose(&clean.project(), &common::ctx(), Some(facade.store())).actionable()
  };

  let fx = Fixture::new();
  plant(&fx, "st.invented");
  let facade = fx.facade_on_disk();
  let report = doctor::diagnose(&fx.project(), &common::ctx(), Some(facade.store()));

  assert!(
    report
      .findings
      .iter()
      .any(|f| f.detail.contains("st.invented")),
    "the fixture must have produced the advisory, or this test is about nothing"
  );
  assert_eq!(
    report.actionable(),
    before,
    "an op nobody recognises is a state, not an obligation: nothing is broken, \
     there is no remedy to offer, and it must not be able to turn a healthy \
     estate red for keeping a true record of its own past"
  );
  assert!(
    report.advisories() > 0,
    "it must still be REPORTED -- silence and a clean bill of health are \
     indistinguishable to a reader, which is the whole reason this exists"
  );
}

// ---------------------------------------------------------------------------
// THE ROSTER'S TWO SOURCE-SIDE OBLIGATIONS
// ---------------------------------------------------------------------------

/// Every `.rs` file under this crate's `src`, EXCEPT the one holding the roster.
///
/// **THE EXCLUSION IS THE ENTIRE POINT OF THIS HELPER.** `KNOWN_OPS` is a list
/// of these strings, so a search that included `event.rs` would find every one
/// of them in the roster itself and pass unconditionally -- a test that proves
/// a list contains what the list contains.
fn call_site_source() -> String {
  fn walk(dir: &std::path::Path, out: &mut String) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for e in entries.flatten() {
      let p = e.path();
      if p.is_dir() {
        walk(&p, out);
      } else if p.extension().is_some_and(|x| x == "rs")
        && p.file_name().is_some_and(|n| n != "event.rs")
      {
        // **COMMENT LINES ARE DROPPED, BECAUSE A COMMENT IS NOT A CALL SITE.**
        // A retired op discussed in prose would otherwise keep its roster entry
        // alive forever, which is the exact failure this test exists to catch.
        for line in std::fs::read_to_string(&p).unwrap_or_default().lines() {
          if !line.trim_start().starts_with("//") {
            out.push_str(line);
            out.push('\n');
          }
        }
      }
    }
  }
  let mut out = String::new();
  walk(
    &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
    &mut out,
  );
  assert!(
    !out.is_empty(),
    "no source was read, so every assertion derived from it would be vacuous"
  );
  out
}

/// **AN OP RETIRED OR RENAMED AND LEFT IN THE ROSTER IS THE QUIET FAILURE, AND
/// THIS IS THE ONLY MECHANISM THAT SEES IT.**
///
/// From the log's side a stale roster entry looks exactly like a healthy one:
/// the op matches, the live check says nothing, and a future parse would fail
/// on rows nobody was warned about.
#[test]
fn known_ops_are_spelled_in_the_source_that_declares_them() {
  let source = call_site_source();
  let orphans: Vec<&str> = KNOWN_OPS
    .iter()
    .copied()
    // **BOTH QUOTE FORMS, AND THE SECOND ONE IS NOT PEDANTRY.** `todo.flush`
    // reaches its only call site inside a SQL string -- `WHERE op = 'todo.flush'`
    // in `store.rs` -- so a reader that knew about Rust literals alone reported
    // it as retired on this test's FIRST RUN. It was the instrument that was
    // too narrow, not the roster that was stale, and a plausible-looking red
    // that says "this op was renamed" is one somebody acts on.
    .filter(|op| !source.contains(&format!("\"{op}\"")) && !source.contains(&format!("'{op}'")))
    .collect();
  assert!(
    orphans.is_empty(),
    "{orphans:?} are in `event::KNOWN_OPS` and spelled at no call site outside \
     `event.rs`. Either the op was renamed or retired and the roster kept the old \
     name -- in which case history under it is now unreachable from the code and \
     the roster is the only thing still claiming otherwise -- or it never had a \
     call site at all. Remove it, or name the site."
  );
}

/// **AND EVERY OP THE STATE MACHINE DECLARES IS IN THE ROSTER.**
///
/// Read from the edge table at runtime, so it cannot drift from the machine it
/// describes -- the same derivation
/// `every_st_op_has_a_declared_list_answer.rs` uses one door over, and for the
/// same reason.
#[test]
fn every_transition_op_is_in_the_roster() {
  let Some(Disposition::State { edges, .. }) = find("Thread", "status").map(|f| &f.disposition)
  else {
    panic!("Thread.status is not a state machine in the transitions table");
  };
  let declared: BTreeSet<&str> = KNOWN_OPS.iter().copied().collect();
  let missing: Vec<&str> = edges
    .iter()
    .map(|e| e.verb)
    .filter(|v| !declared.contains(v))
    .collect();
  assert!(
    !edges.is_empty(),
    "the edge table yielded nothing, so this assertion is vacuous"
  );
  assert!(
    missing.is_empty(),
    "{missing:?} are transitions this build can perform and `event::KNOWN_OPS` \
     does not list, so `doctor` would report their own events as ops this build \
     does not declare"
  );
}
