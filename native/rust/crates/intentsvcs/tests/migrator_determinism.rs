//! AT-10.12 / AC-10.12: **a verb that reports an estate unchanged must not
//! have changed it.**
//!
//! **THE ROW WAS RE-CUT AND THE FILENAME WAS NOT, SO THE FIRST THING THIS
//! HEADER OWES A READER IS THAT MISMATCH.** `migrator_determinism` names the
//! property the row was MINTED with -- *migrate twice, require identical canon
//! bytes* -- and vc withdrew that wording on 2026-08-19. The live property is
//! about the REPORT: re-running a migrator over an estate may legitimately
//! overwrite what it produced, and **saying nothing changed while changing
//! forty files is the defect.** The arms below are named for the property; the
//! file is named for the citation, and moving a citation is vc's. **A home
//! whose name does not describe its contents is a defect this estate keeps
//! re-finding, so it is declared here rather than left for someone to notice.**
//!
//! **cc's OWN `migrate twice, require identical` IS DELIBERATELY NOT BUILT
//! HERE.** vc refused to mint it: it is unmeasured, and it does not catch the
//! observed defect, because two migrations from one v2 source agree with each
//! other while both disagree with the backfill that actually wrote the bytes.
//! An adjacent property is not a free one.
//!
//! # Why this fixture carries ISSUES, which is the whole reason it exists
//!
//! **`upgrade_command.rs`'s `running_it_twice_leaves_the_tree_byte_identical`
//! ALREADY DRIVES THE RE-RUN, IS GREEN, AND IS STRUCTURALLY BLIND TO THE
//! POPULATION THE DEFECT WAS MEASURED IN.** Its fixture is `v2_project` plus
//! two `v2_thread`s and **no `intent/issues/` at all** -- while AC-10.12's
//! measured instance is *all 40 issue bodies rewritten* under the unchanged
//! claim. A fixture with no issues cannot exhibit an issue-body defect, so
//! that test would have gone on passing through the entire episode.
//!
//! That is not a criticism of it: its subject is the re-run's idempotence and
//! for that subject its fixture is adequate. **The point is that a green there
//! was never evidence about here**, and nothing said so until this file.
//!
//! # The asymmetry that held this test back, and why it no longer does
//!
//! This instrument was blocked by cc on 2026-08-19 -- while authorised to
//! build it -- because threads and issues go through the same `frontmatter()`
//! and yet canon carried a trim on neither, while a re-run changed zero thread
//! canon and all forty issues. **A determinism test written against a
//! mechanism its author has half-understood passes for the wrong reason.**
//!
//! **THE HOLD IS DISCHARGED AND THE ANSWER IS ARCHITECTURAL RATHER THAN A
//! MISSING TRIM.** `legacy.rs:382` DECOMPOSES a thread body into
//! `(heading, text)` pairs and REASSEMBLES it, so the raw slice never survives
//! and leading or trailing whitespace **structurally cannot appear**;
//! `legacy.rs:685` is `body: body.to_string()`, the raw slice verbatim,
//! carrying bytes on purpose. Populations close both ways: 54 of 54 thread
//! bodies begin `## `, 40 of 40 issue bodies begin `# ` and zero begin `## `.
//! **Two mechanisms, one of which cannot express the defect.**
//!
//! # What is NOT claimed here, stated rather than discovered later
//!
//! **WHICH SIDE MOVED IS NOT DETERMINABLE BY ANY INSTRUMENT NOW AVAILABLE.**
//! The v2 binary refuses a v3 tree at exit 2, so no run of it can be compared
//! against a run of this one. **A green here says the verb and the estate
//! agree TODAY. It never says they always did**, and a reader who takes it for
//! the stronger claim is reading something this file cannot support.

mod common;

use std::collections::BTreeMap;

use common::{Fixture, facade_ctx, tree, v2_estate, v2_thread};
use intentsvcs::facade::Facade;

/// The issue numbers this fixture plants, in both v2 buckets.
///
/// **BOTH ARMS, because the migrator routes issue findings by bucket** -- block
/// in live work, carry in closed -- and a fixture populating one arm leaves the
/// other unexercised while every count reconciles perfectly against zero.
const ISSUES: &[(&str, &str, &str)] = &[("OPEN", "0001", "OPEN"), ("CLOSED", "0002", "CLOSED")];

/// The store is per-machine truth and legitimately differs after any run.
///
/// **D42 MAKES THIS A FIXED POINT RATHER THAN A CONVENIENCE.** The schema
/// defaults `created_at`/`updated_at` to `strftime`, so two runs of a perfect
/// migrator can never produce the same database -- measured at 117ms apart on
/// 705 rows. Comparing it would red every run for a reason that is not this
/// criterion's subject.
fn is_the_store(path: &str) -> bool {
  path.starts_with("intent/.cache/")
}

/// A v2 estate with threads AND issues, converted once.
fn migrated_estate_with_issues() -> Fixture {
  let fx = v2_estate();
  v2_thread(&fx, "ST0001", "WIP");
  v2_thread(&fx, "ST0002", "Completed");
  for (bucket, num, status) in ISSUES {
    fx.write_file(
      &format!("intent/issues/{bucket}/{num}/{num}-a-slug.md"),
      &format!(
        "---\nid: \"{num}\"\ntitle: a title\ndate: 2026-08-05\nreporter: matts\nstatus: \
         {status}\nseverity: medium\n---\n\n# {num}: a title\n\nBody.\n"
      ),
    );
  }
  Facade::upgrade(&fx.project(), &facade_ctx()).expect("a v2 estate with issues converts");
  fx
}

/// Paths whose bytes differ between two snapshots, the store excluded.
///
/// **PATHS, NEVER BYTES.** An `assert_eq!` over the byte vectors dumped 757KB
/// of sqlite into a failure message the first time this estate tried it.
fn differing(before: &BTreeMap<String, Vec<u8>>, after: &BTreeMap<String, Vec<u8>>) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();
  for (path, bytes) in before {
    if is_the_store(path) {
      continue;
    }
    match after.get(path) {
      Some(now) if now == bytes => {}
      Some(_) => out.push(format!("{path} (rewritten)")),
      None => out.push(format!("{path} (removed)")),
    }
  }
  for path in after.keys() {
    if !is_the_store(path) && !before.contains_key(path) {
      out.push(format!("{path} (added)"));
    }
  }
  out.sort();
  out
}

/// **THE PROPERTY: the claim and the estate agree.**
#[test]
fn a_run_that_reports_content_unchanged_leaves_the_content_unchanged() {
  let fx = migrated_estate_with_issues();
  let before = tree(fx.root());

  let again = Facade::upgrade(&fx.project(), &facade_ctx()).expect("a migrated estate re-runs");

  // **THE CLAIM'S PRECONDITION IS ASSERTED BEFORE THE CLAIM IS JUDGED.** The
  // renderer prints `their content is unchanged` only when `already_migrated`
  // is non-empty, so a re-run that populated nothing would make the arm below
  // a check on a sentence nobody printed -- green, and about nothing.
  assert!(
    !again.already_migrated.is_empty(),
    "the re-run reported no already-migrated threads, so the `content is unchanged` claim was \
     never made and this arm is judging a sentence that was not printed"
  );

  let after = tree(fx.root());
  let moved = differing(&before, &after);
  assert!(
    moved.is_empty(),
    "the verb reported {} thread(s) whose `content is unchanged` and then changed {} file(s) -- \
     the defect is the REPORT rather than the rewrite, and an operator who believes it will not \
     look:\n  {}",
    again.already_migrated.len(),
    moved.len(),
    moved.join("\n  ")
  );
}

/// **THE FIXTURE CARRIES THE POPULATION THE DEFECT WAS MEASURED IN**, which is
/// the only thing separating this file from the green one that already exists.
///
/// Without this arm the fixture could quietly lose its issues -- a renamed
/// bucket, a changed path -- and the arm above would go on passing over threads
/// alone, which is precisely the blind spot this file was written to close.
#[test]
fn the_fixture_carries_issues_because_a_thread_only_estate_cannot_exhibit_the_defect() {
  let fx = migrated_estate_with_issues();
  let store = intentsvcs::store::Store::open(&fx.project().db_path()).expect("the store opens");
  let (threads, issues) = store.load_canon().expect("the store loads");

  assert_eq!(
    issues.len(),
    ISSUES.len(),
    "the fixture planted {} issue(s) and the store holds {} -- an issue-body criterion measured \
     over an estate with no issue bodies passes for free",
    ISSUES.len(),
    issues.len()
  );
  assert!(
    !threads.is_empty(),
    "the fixture holds no threads, so `already_migrated` can never populate and the arm beside \
     this one would refuse rather than measure"
  );
}

/// **THE COMPARISON CAN SEE A CHANGE, DRIVEN RATHER THAN ASSUMED.**
///
/// The property arm passes by finding NOTHING. That is the shape that goes
/// green when the instrument breaks: a differ that reports an empty set for a
/// tree it cannot read is indistinguishable from one reporting a clean estate,
/// and both say the same word. **So a byte is planted and the differ must name
/// it** -- and the planted path is a canon file, because canon is what a
/// re-run rewrites and therefore what the arm above must be able to catch.
#[test]
fn the_comparison_names_a_file_the_estate_actually_changed() {
  let fx = migrated_estate_with_issues();
  let before = tree(fx.root());

  let canon = fx.canon_rel("ST0001");
  let mut bytes = fx.read(&canon);
  bytes.push_str("\n");
  fx.write_file(&canon, &bytes);

  let after = tree(fx.root());
  let moved = differing(&before, &after);
  assert!(
    moved.iter().any(|p| p.starts_with(&canon)),
    "a byte was appended to `{canon}` and the comparison did not name it, so the property arm \
     beside this one is passing against a differ that cannot see a rewrite. Reported: {moved:?}"
  );
}
