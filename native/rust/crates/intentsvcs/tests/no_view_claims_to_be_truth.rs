//! AT-03.17 / ST0056 AC-03.16: **no generated view names a generated artefact
//! as the source of truth.**
//!
//! # Why this is a work-loss defect and not a wording nit
//!
//! Under the ratified model the store is durable truth and the committed
//! extract is the interchange (D34). `acceptance.md` is a GENERATED VIEW. A
//! reader who believes a view that calls itself the single source of truth
//! authors a criterion row directly into it, and the next sync discards the
//! row without a word. **The document instructs the exact action the model
//! punishes**, and the person who loses the work is not the person who runs
//! the sync.
//!
//! The window is not theoretical and it is not long: vc measured a plain
//! `sync --to-store` -- an operation about the STORE -- rewriting both
//! `acceptance.md` files without their touching either. So the gap between
//! authoring into a view and losing it is one peer's unrelated commit.
//!
//! # Drive the generator, then read what it wrote
//!
//! The subject is [`views::render_all`] over the real estate, never the
//! generator's string literals. Reading `views.rs`'s own constants and
//! checking them against themselves is the tautology this thread keeps
//! meeting: **take the expected value from the subject and the test loses the
//! ability to disagree with it.**
//!
//! # What counts as "a generated artefact" is asked of the generator
//!
//! The roster of generated paths is `render_all`'s own output set, not a list
//! kept here. A hand-kept list is one someone must remember to extend on the
//! day they add a view, which is the day they are thinking about something
//! else -- the same argument `openness.rs` makes for enumerating tables from
//! the DDL.
//!
//! **Canon is deliberately NOT in that set, and the exclusion is the whole
//! distinction.** `intent/.canon/st/<ID>.json` is written by `--to-disk` too,
//! so "is it generated" is the wrong question. The right one is whether an
//! edit SURVIVES: a row hand-edited into canon round-trips through
//! `--to-store` and is how a criterion is minted at all, while a row authored
//! into a view is discarded. A view naming CANON as truth is correct and must
//! not be flagged.
//!
//! # The claim, never the word
//!
//! A sentence is a finding only when it BOTH carries an authority phrase AND
//! names a generated artefact. Keying on the word alone flags
//! `views.rs`'s own _storing it would be double truth_ -- a correct statement
//! about the model that names no path -- and AT-03.17 names that line as the
//! negative arm precisely so a fix cannot overshoot into it.
//!
//! # No count is asserted
//!
//! The estate measured 206 of 207 covers carrying the defect at `40d27ba2`.
//! That number is a measurement of a defect due to be repaired, and **a count
//! baked into a test goes green by drifting.** The assertion is the property;
//! the examined population is printed as the denominator.

//! # Attribution: the GENERATOR's words, not the author's
//!
//! A rendered view is a generated frame wrapped around authored prose, and
//! only the frame is this criterion's subject. **Scanning the whole view makes
//! reporting the defect an offence**: AC-03.16's own row quotes the historical
//! sentence verbatim, and ST0044 carries a v2-era criterion that says the same
//! thing truthfully about v2. The whiteboard header guard settled this shape
//! already -- it never scans prose, for exactly this reason.
//!
//! So a claim is attributed to the generator when it does NOT appear in the
//! canon the view was rendered FROM. Per thread, never estate-wide: because
//! AC-03.16 quotes the sentence, an estate-wide corpus would excuse every
//! generated occurrence of it in every other thread.
//!
//! **STATED REACH, because it is a real hole and not a rounding error.** The
//! one thread whose canon quotes the defect -- ST0056 -- cannot be caught by
//! the estate arm for that sentence, since the quotation legitimately excuses
//! it there. That is why
//! [`the_historical_claim_is_caught_and_the_model_statement_is_not`] asserts
//! the checker against the exact historical wording as a plain string: the
//! estate arm proves the estate is clean, and the string arm proves the
//! checker can still see the thing.

mod common;

use common::ctx;
use intentsvcs::model::Thread;
use intentsvcs::project::Project;
use intentsvcs::{ingest, views};
use std::collections::BTreeSet;
use testkit::repo_root;

/// Phrases that assert a thing is authoritative.
///
/// Deliberately NOT the bare word "truth": the preamble's _storing it would be
/// double truth_ is a true statement about double-storage, and AT-03.17 names
/// that line as the negative arm so a fix cannot overshoot into it.
const AUTHORITY_PHRASES: &[&str] = &[
  "source of truth",
  "canonical",
  "authoritative",
  "the truth about",
];

/// A char-boundary-safe window of `s` around byte offset `at`.
///
/// Used to compare a claim against the canon it may have come from. The
/// SENTENCE cannot be compared directly: a rendered AC row is
/// `- AC-03.5 ` + authored text, so the generated prefix would defeat a whole
/// sentence match and report every authored criterion as a generator claim.
fn window(s: &str, at: usize, radius: usize) -> &str {
  let start = s[..at]
    .char_indices()
    .rev()
    .take(radius)
    .last()
    .map(|(i, _)| i)
    .unwrap_or(0);
  let end = s[at..]
    .char_indices()
    .take(radius)
    .last()
    .map(|(i, c)| at + i + c.len_utf8())
    .unwrap_or(s.len());
  &s[start..end]
}

/// Claims in `content` that assert authority over a generated artefact AND
/// were written by the GENERATOR rather than carried from `authored`.
///
/// **A function of strings**, so the discriminating case is a standing
/// assertion rather than a mutation someone ran once by hand.
fn generator_truth_claims(
  content: &str,
  artefacts: &BTreeSet<String>,
  authored: &str,
) -> Vec<String> {
  let mut out = Vec::new();
  for line in content.lines() {
    for sentence in line.split(". ") {
      let lowered = sentence.to_lowercase();
      let Some(phrase) = AUTHORITY_PHRASES.iter().find(|p| lowered.contains(**p)) else {
        continue;
      };
      let named: Vec<&String> = artefacts.iter().filter(|a| sentence.contains(*a)).collect();
      if named.is_empty() {
        // An authority claim about something that is not a generated artefact:
        // the ratified boundary, or canon itself. Neither is the subject.
        continue;
      }
      let at = lowered.find(*phrase).unwrap_or(0);
      if authored.contains(window(sentence, at, 30)) {
        // Carried prose. The author said it; the generator only framed it.
        continue;
      }
      out.push(format!(
        "names {} as authoritative: {}",
        named
          .iter()
          .map(|s| s.as_str())
          .collect::<Vec<_>>()
          .join(", "),
        sentence.trim()
      ));
    }
  }
  out
}

/// Everything a human authored into one thread, as the canon JSON.
///
/// Serialised rather than enumerated field by field. A hand-kept roster of
/// authored fields is one someone must remember to extend on the day they add
/// a field, and canon is by construction every authored string there is.
fn authored_of(thread: &Thread) -> String {
  serde_json::to_string(thread).expect("a thread serialises")
}

#[test]
fn no_generated_view_names_a_generated_artefact_as_truth() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let canon = ingest::read(&project).expect("canon reads from the real estate");
  let context = ctx();

  // The generated-artefact roster, asked of the generator rather than kept
  // here, and the denominator for the whole run.
  let rendered = views::render_all(&project, &canon, &context);
  let artefacts: BTreeSet<String> = rendered
    .iter()
    .filter_map(|v| v.path.file_name().map(|n| n.to_string_lossy().into_owned()))
    .collect();

  assert!(
    !rendered.is_empty() && !artefacts.is_empty(),
    "precondition: {} views over {} artefact names -- a probe whose population \
     is empty cannot contain the failure it tests for",
    rendered.len(),
    artefacts.len()
  );

  // Re-rendered per thread so each view is checked against the canon it came
  // FROM. `render_all` does not say which thread produced which view, and
  // guessing from the path would be deriving identity from a path -- the exact
  // thing that does not survive a relocation.
  let mut findings = Vec::new();
  let mut examined = 0usize;
  for thread in &canon.threads {
    let authored = authored_of(thread);
    let mut per_thread = vec![
      (project.info_view(&thread.id), views::info(thread, &context)),
      (
        project.acceptance_view(&thread.id),
        views::acceptance(thread, &context),
      ),
    ];
    for wp in &thread.wps {
      per_thread.push((
        project.wp_info_view(&thread.id, wp.seq),
        views::wp_info(thread, wp, &context),
      ));
    }
    for (path, content) in per_thread {
      examined += 1;
      for claim in generator_truth_claims(&content, &artefacts, &authored) {
        findings.push(format!("{}: {claim}", project.relative(&path)));
      }
    }
  }

  // The two estate-level views have no single authoring thread, so they are
  // checked against the whole canon.
  let estate_authored = serde_json::to_string(&canon.threads).expect("canon serialises");
  for (path, content) in [
    (
      project.steel_threads_view(),
      views::steel_threads(&canon.threads, &context),
    ),
    (project.todo_view(), views::todo(&canon.threads, &context)),
  ] {
    examined += 1;
    for claim in generator_truth_claims(&content, &artefacts, &estate_authored) {
      findings.push(format!("{}: {claim}", project.relative(&path)));
    }
  }

  assert_eq!(
    examined,
    rendered.len(),
    "the per-thread walk examined {examined} views where the generator renders {} -- \
     a partition that does not close means a view kind is going unchecked",
    rendered.len()
  );

  assert!(
    findings.is_empty(),
    "{} claims across {examined} rendered views name a generated artefact as the \
     source of truth (roster: {}):\n  {}",
    findings.len(),
    artefacts
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<_>>()
      .join(", "),
    findings.join("\n  ")
  );
}

/// **THE DISCRIMINATING CASE.** The historical sentence must still be caught.
///
/// Without this the estate arm is a probe whose population no longer contains
/// the failure: the estate was repaired in the same commit that added the
/// test, so a green proves the fix and says nothing about the checker. And for
/// ST0056 specifically the estate arm CANNOT see this sentence, because
/// AC-03.16 quotes it and the quotation legitimately excuses it there.
#[test]
fn the_historical_claim_is_caught_and_the_model_statement_is_not() {
  let artefacts: BTreeSet<String> = ["acceptance.md".to_string(), "info.md".to_string()]
    .into_iter()
    .collect();
  // Nothing authored, so anything found is attributed to the generator.
  let nothing_authored = "";

  let historical = "Acceptance Criteria and Acceptance Tests live in `acceptance.md` -- the single source of truth. This cover never restates them.";
  assert!(
    !generator_truth_claims(historical, &artefacts, nothing_authored).is_empty(),
    "the sentence this criterion exists about must be caught: {historical:?}"
  );

  let wp_historical = "Acceptance Criteria for this work package live in `ST0056/acceptance.md`, under the `WP-01` heading -- the single source of truth.";
  assert!(
    !generator_truth_claims(wp_historical, &artefacts, nothing_authored).is_empty(),
    "the WP-level twin must be caught too: {wp_historical:?}"
  );

  // NEGATIVE ARM, NAMED IN AT-03.17 SO A FIX CANNOT OVERSHOOT. A correct
  // statement about the model that happens to carry the word "truth".
  let model_statement = "Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth.";
  assert!(
    generator_truth_claims(model_statement, &artefacts, nothing_authored).is_empty(),
    "the model statement must NOT be flagged -- a check that fires here has keyed \
     on the word rather than the claim: {model_statement:?}"
  );

  // Naming CANON as the place to author is CORRECT: a hand edit there survives
  // the round trip, and that survival is the whole distinction being drawn.
  let canon_claim = "The contract is canon in this thread's model: mint a row in `.canon/st/ST0056.json`, then `intent sync --to-store`.";
  assert!(
    generator_truth_claims(canon_claim, &artefacts, nothing_authored).is_empty(),
    "naming canon as the place to author is correct and must not be flagged"
  );

  // ATTRIBUTION ITSELF, both ways. The same sentence is a finding when the
  // generator wrote it and not when the author did.
  let authored_corpus = format!("{{\"text\":\"{historical}\"}}");
  assert!(
    generator_truth_claims(historical, &artefacts, &authored_corpus).is_empty(),
    "a claim carried from canon is the AUTHOR's and must not be attributed to \
     the generator -- quoting the defect must never itself be the offence"
  );
}
