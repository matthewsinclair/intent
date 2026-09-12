//! **WP-02: the v2 tree survives migration and disagrees with the store.**
//!
//! A migrated estate carries `intent/st/COMPLETED|CANCELLED|NOT-STARTED/<ID>/`
//! beside the flat v3 tree, and the v2 path answers greps first because it is
//! the one that existed for a year. The ruling of 2026-09-12 is ingest, then
//! prune, in that order, **and the prune refuses while the ingest is
//! incomplete** -- so these arms are mostly about the refusal, which is the
//! half that decides whether a bad day loses anybody's work.
//!
//! # Why the refusal is asserted on CONTENT and never on a row
//!
//! A row proves a path was seen. The half-migrated thread is the observed
//! shape, not a hypothetical: `acceptance.md` migrated and `design.md` did not,
//! on a real estate, at rc 0. So `bucket_verdict` compares the SHA of the bytes
//! on disk against the attachment canon carries, and
//! [`a_row_whose_bytes_never_arrived_is_not_held`] drives exactly that case --
//! an attachment present in canon under the right path, carrying different
//! bytes.

use crate::common::{Fixture, sample_issue, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::legacy;
use intentsvcs::model::Attachment;

/// A canon holding one thread, with whatever attachments the arm needs.
fn canon_with(attachments: Vec<Attachment>) -> Canon {
  let mut thread = sample_thread("ST0002");
  thread.attachments = attachments;
  Canon {
    threads: vec![thread],
    issues: Vec::new(),
    sections: Vec::new(),
    boards: Vec::new(),
  }
}

const DESIGN: &str = "# Design\n\nthe bytes only the bucket holds\n";

/// The bucket copy a migrated estate is left with: a thread directory under
/// `COMPLETED/` carrying the two source documents and one attachment.
fn bucketed(fx: &Fixture) {
  fx.write_file(
    "intent/st/COMPLETED/ST0002/info.md",
    "---\nstatus: Completed\n---\n\n# ST0002: A thread\n",
  );
  fx.write_file(
    "intent/st/COMPLETED/ST0002/acceptance.md",
    "# ST0002 -- Acceptance\n\n- AC-00.1 (non-test) x -- evidence: y -- satisfied: yes\n",
  );
  fx.write_file("intent/st/COMPLETED/ST0002/design.md", DESIGN);
}

/// AC-02.2. **The whole estate is the unit, and this is the arm that says so.**
#[test]
fn a_bucket_whose_content_the_store_holds_is_removable_whole() {
  let fx = Fixture::new();
  bucketed(&fx);
  let canon = canon_with(vec![Attachment::new("design.md", DESIGN)]);

  let found = legacy::leftovers(&fx.project(), &canon);
  assert!(
    !found.refuses(),
    "every bucket file's content is held, so nothing is withheld: {:?}",
    found.withheld
  );
  let names: Vec<String> = found
    .removable
    .iter()
    .map(|p| fx.project().relative(p))
    .collect();
  assert_eq!(
    names,
    vec![
      "intent/st/COMPLETED/ST0002/acceptance.md",
      "intent/st/COMPLETED/ST0002/design.md",
      "intent/st/COMPLETED/ST0002/info.md",
    ],
    "the two source documents are held as the model and the attachment as its bytes"
  );
}

/// AC-02.2, the half that matters. **A row is not content.**
#[test]
fn a_row_whose_bytes_never_arrived_is_not_held() {
  let fx = Fixture::new();
  bucketed(&fx);
  // The attachment is in canon, at the right path, with the wrong bytes --
  // exactly a half-migrated thread, which every row-counting check passes.
  let canon = canon_with(vec![Attachment::new(
    "design.md",
    "# Design\n\nsomething else\n",
  )]);

  let found = legacy::leftovers(&fx.project(), &canon);
  assert!(found.refuses(), "the bytes differ, so the prune refuses");
  let withheld: Vec<String> = found
    .withheld
    .iter()
    .map(|w| fx.project().relative(&w.path))
    .collect();
  assert_eq!(withheld, vec!["intent/st/COMPLETED/ST0002/design.md"]);
  assert!(
    found.withheld[0].reason.contains("differs"),
    "the refusal names the cause, not just the path: {}",
    found.withheld[0].reason
  );
}

/// AC-02.2. A thread the estate never migrated keeps every one of its files.
#[test]
fn a_bucket_thread_absent_from_canon_holds_its_whole_directory() {
  let fx = Fixture::new();
  bucketed(&fx);
  let canon = Canon {
    threads: Vec::new(),
    issues: Vec::new(),
    sections: Vec::new(),
    boards: Vec::new(),
  };

  let found = legacy::leftovers(&fx.project(), &canon);
  assert!(found.refuses());
  assert_eq!(
    found.withheld.len(),
    3,
    "all three, because nothing in canon claims any of them: {:?}",
    found.withheld
  );
  assert!(
    found
      .withheld
      .iter()
      .all(|w| w.reason.contains("ST0002 is not in canon")),
    "and each says which thread is missing: {:?}",
    found.withheld
  );
}

/// AC-02.2. v2's issue estate and the retired cache go with the buckets --
/// **and the issue is held on its BODY, because migration turns the
/// frontmatter into fields and a byte comparison would refuse every issue ever
/// migrated correctly.**
#[test]
fn the_v2_issue_estate_and_the_retired_cache_are_leftovers_too() {
  let fx = Fixture::new();
  let issue = sample_issue(21);
  fx.write_file(
    "intent/issues/CLOSED/0021/0021-credo-checks.md",
    &format!(
      "---\nid: 0021\ntitle: prune the dead mechanism\nstatus: CLOSED\n---\n\n{}",
      issue.body
    ),
  );
  fx.write_file("intent/.treeindex/cache.json", "{}\n");
  let canon = Canon {
    threads: Vec::new(),
    issues: vec![issue],
    sections: Vec::new(),
    boards: Vec::new(),
  };

  let found = legacy::leftovers(&fx.project(), &canon);
  assert!(
    !found.refuses(),
    "canon holds the body: {:?}",
    found.withheld
  );
  let names: Vec<String> = found
    .removable
    .iter()
    .map(|p| fx.project().relative(p))
    .collect();
  assert_eq!(
    names,
    vec![
      "intent/.treeindex/cache.json",
      "intent/issues/CLOSED/0021/0021-credo-checks.md",
    ]
  );
}

/// AC-02.3. **The discriminating case is this estate's own unclaimed tooling.**
///
/// `intent/st/ST0056/parity/tools/` lives under a FLAT thread directory, and
/// the population is rooted at the three bucket names -- so it is not a prune
/// candidate, and nothing here had to guess what leftover tooling looks like.
#[test]
fn a_tool_tree_beside_the_threads_is_neither_candidate_nor_pruned() {
  let fx = Fixture::new();
  bucketed(&fx);
  fx.write_file(
    "intent/st/ST0056/parity/tools/contract_check.sh",
    "#!/usr/bin/env bash\necho parity\n",
  );
  let canon = canon_with(vec![Attachment::new("design.md", DESIGN)]);

  let found = legacy::leftovers(&fx.project(), &canon);
  let touched: Vec<String> = found
    .removable
    .iter()
    .chain(found.withheld.iter().map(|w| &w.path))
    .map(|p| fx.project().relative(p))
    .collect();
  assert!(
    !touched.iter().any(|p| p.contains("parity/tools")),
    "a tree beside the threads is neither removable nor withheld -- it is not in the population at all: {touched:?}"
  );
}

/// AC-02.4. **Reported, and the file is not touched.**
#[test]
fn an_authored_file_naming_a_bucket_path_is_reported_and_left_alone() {
  let fx = Fixture::new();
  bucketed(&fx);
  let authored = "# Notes\n\nThe old design is at `intent/st/COMPLETED/ST0002/design.md`.\n";
  fx.write_file("intent/docs/notes.md", authored);
  let canon = canon_with(vec![Attachment::new("design.md", DESIGN)]);

  let found = legacy::leftovers(&fx.project(), &canon);
  let named: Vec<(String, u32)> = found
    .pointers
    .iter()
    .map(|p| (fx.project().relative(&p.path), p.line))
    .collect();
  assert_eq!(
    named,
    vec![("intent/docs/notes.md".to_string(), 3)],
    "the worklist is file and line, so a human can go straight to it"
  );
  assert_eq!(
    fx.read("intent/docs/notes.md"),
    authored,
    "and the file is not rewritten, byte for byte"
  );
}

// ---------------------------------------------------------------------------
// AC-02.2: the two doors, and the refusal that stands in front of both
// ---------------------------------------------------------------------------

/// Drive `organize` over a fixture and hand back its report.
fn organize(
  fx: &Fixture,
  canon: &Canon,
  mode: intentsvcs::organize::Mode,
) -> intentsvcs::organize::Report {
  let project = fx.project();
  let (tree, digest) = intentsvcs::organize::observe(&project, &[]).expect("observes the tree");
  // Declares nothing, which is somebody saying none -- so no view of the
  // thread is realised and every row below is about the v2 leftovers.
  let realised = intentsvcs::intentfiles::realised_for_action("# BEGIN INTENT\n# END INTENT\n")
    .expect("the manifest parses");
  let plan = intentsvcs::organize::plan(
    &project,
    canon,
    &realised,
    &crate::common::ctx(),
    &tree,
    digest.clone(),
  );
  plan
    .run(mode, &|| digest.clone())
    .expect("the run completes")
}

/// AC-02.2, the `organize --apply` door.
#[test]
fn organize_apply_removes_the_v2_tree_the_store_holds() {
  let fx = Fixture::new();
  bucketed(&fx);
  fx.write_file("intent/.treeindex/cache.json", "{}\n");
  let canon = canon_with(vec![Attachment::new("design.md", DESIGN)]);

  let preview = organize(&fx, &canon, intentsvcs::organize::Mode::Preview);
  assert_eq!(
    preview.pruned_legacy.len(),
    4,
    "the preview NAMES what an apply would remove, or the apply is a silent deletion: {:?}",
    preview.pruned_legacy
  );
  assert!(
    fx.path("intent/st/COMPLETED/ST0002/design.md").exists(),
    "and a preview removes nothing"
  );

  let done = organize(&fx, &canon, intentsvcs::organize::Mode::Apply);
  assert!(
    done.refused.is_empty(),
    "nothing to refuse: {:?}",
    done.refused
  );
  for rel in [
    "intent/st/COMPLETED/ST0002/design.md",
    "intent/st/COMPLETED/ST0002/info.md",
    "intent/st/COMPLETED/ST0002/acceptance.md",
    "intent/.treeindex/cache.json",
  ] {
    assert!(
      !fx.path(rel).exists(),
      "{rel} is still on disk after the prune"
    );
  }
}

/// AC-02.2, the refusal. **A prune that ingested nothing removes nothing.**
#[test]
fn one_unheld_file_refuses_the_whole_prune_and_names_it() {
  let fx = Fixture::new();
  bucketed(&fx);
  // In canon at the right path, with the wrong bytes: the half-migrated shape.
  let canon = canon_with(vec![Attachment::new(
    "design.md",
    "# Design\n\nsomething else\n",
  )]);

  let done = organize(&fx, &canon, intentsvcs::organize::Mode::Apply);
  assert!(
    done.pruned_legacy.is_empty(),
    "one unheld file refuses EVERY removal, not just its own: {:?}",
    done.pruned_legacy
  );
  for rel in [
    "intent/st/COMPLETED/ST0002/design.md",
    "intent/st/COMPLETED/ST0002/info.md",
  ] {
    assert!(
      fx.path(rel).exists(),
      "{rel} was removed despite the refusal"
    );
  }
  let named: Vec<String> = done.refused.iter().map(|r| r.to_string()).collect();
  assert!(
    named
      .iter()
      .any(|r| r.contains("design.md") && r.contains("differs")),
    "the refusal names the file and the cause: {named:?}"
  );
}
