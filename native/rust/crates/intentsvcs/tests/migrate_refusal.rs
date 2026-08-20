//! **AT-10.2: Phase A residue BLOCKS, its report is CLASSED, and the estate a
//! refusal leaves behind is byte-identical to the one it found.**
//!
//! # Two of this criterion's four limbs were already covered, by a file its row
//! does not cite
//!
//! Measured before a line of this file was written, which is the only reason it
//! is not a duplicate. `intent-cli/tests/ingest_command.rs` already drives
//! `live_residue_blocks_and_closed_residue_carries` over a real v2 estate:
//! `rc == 1` -- limbs BLOCKED and exit-non-zero, end to end, through the
//! shipped verb -- with the SAME defect in a CLOSED thread carrying instead.
//! AT-10.2 cites none of it. So the row read `to-write` over coverage that
//! existed, which is the shape that cost this thread a day at `AT-10.14`.
//!
//! # What no test in this estate reads is the CLASS
//!
//! That file asserts the report names the thread (`ST0004`) and the value it
//! could not read (`Banana`). **Neither is the class.** Nine classes are
//! declared in `migration.md`, nine are emitted by `legacy.rs`, and
//! `parity/tools/residue_class_check.sh` compares those two lists **to each
//! other** -- so both ends of the existing check are inside the model, and
//! nothing asserts a class ever reaches the line an operator reads. **A
//! `Finding::body` that dropped `self.class.as_str()` passes every test that
//! exists today**, in both directions, at rc=0.
//!
//! The format has an OPTIONAL segment -- `residue: <file>[:<line>] -- <class>
//! -- <detail>` -- so one example proves one branch. Both are driven below,
//! from one estate, and the two classes are told apart rather than counted.
//!
//! # And atomicity is asserted only where it cannot fail
//!
//! `phase_a_reads_a_v2_estate_and_leaves_it_exactly_as_it_was` runs on a CLEAN
//! estate and reads back ONE file. `migrate.rs`'s
//! `a_blocked_plan_writes_nothing_because_it_cannot` **says so in its own
//! name**: `plan` is a pure planner holding no writer, so the assertion is
//! true of every possible implementation of it. **Neither covers the verb that
//! DOES write, refusing** -- which is the only place the word *atomic* has
//! anything to bite on.
//!
//! # The control, without which the atomicity arm proves nothing
//!
//! *The tree did not change* is also what you get from an `upgrade` that
//! refuses everything, or that is broken, or that was never reached. So the
//! same estate with its defect moved into a CLOSED thread must let `upgrade`
//! through **and the tree must change** -- the closed/live split is hv's ruling
//! and it is what makes an unchanged tree evidence rather than a tautology.

mod common;

use common::{Fixture, changed, facade_ctx, tree, v2_estate, v2_thread};
use intentsvcs::facade::Facade;
use intentsvcs::finding::Finding;
use intentsvcs::legacy;

/// A v2 work package carrying git conflict markers. Returns the 1-based line
/// the first marker is on.
///
/// **Chosen because its finding carries a LINE NUMBER**, which is the optional
/// half of the report format. `unknown-status` has none, so the two together
/// drive both branches of `body()` from one scan.
///
/// **The expected line is DERIVED from the bytes written, never typed.** My
/// first cut typed `9`, the scanner answered `10`, and the fix that suggests
/// itself -- change the 9 to a 10 -- fits the predicate to the observation,
/// which is the provenance rule's third limb. Counting the marker's position in
/// text this function itself authored is a fact about the FIXTURE; where the
/// scanner reports it is the thing under test, and the two stay independent.
fn v2_wp_with_conflict(fx: &Fixture, id: &str) -> u32 {
  let text = "---\nstatus: WIP\nscope: M\n---\n\n# WP-01: A package\n\n## Design\n\n<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> branch\n";
  fx.write_file(&format!("intent/st/{id}/WP/01/info.md"), text);
  let at = text
    .lines()
    .position(|l| l.starts_with("<<<<<<< "))
    .expect("the fixture must contain a conflict marker");
  (at + 1) as u32
}

/// The rendered `residue:` line for the first finding of `class`, or `None`.
fn line_for(findings: &[Finding], class: &str) -> Option<String> {
  findings
    .iter()
    .find(|f| f.class.as_str() == class)
    .map(|f| f.to_string())
}

/// **THE CLASS REACHES THE LINE, AND TWO CLASSES ARE TOLD APART.**
///
/// The instance is drawn from the ESTATE, not handed to the formatter: this
/// scans real v2 markdown and renders whatever `legacy::scan` chose to emit.
/// Every existing residue test either hand-builds a `Scan { residue: vec![..] }`
/// or asserts only the substrings the fixture itself authored.
///
/// **The delimiters are in the assertion on purpose.** `contains("unknown-
/// status")` would pass on a formatter that printed the class in the detail
/// field, or twice, or before the file. ` -- unknown-status -- ` fails on all
/// three, and on dropping it.
#[test]
fn the_residue_report_carries_a_class_per_line_and_a_line_number_when_it_has_one() {
  let fx = v2_estate();
  v2_thread(&fx, "ST0001", "Banana");
  v2_thread(&fx, "ST0002", "WIP");
  let marker_line = v2_wp_with_conflict(&fx, "ST0002");

  let scan = legacy::scan(&fx.project()).expect("a v2 estate scans");

  assert!(
    scan.residue.len() >= 2,
    "the fixture must produce at least two live findings or the comparison below is vacuous: {:?}",
    scan.residue
  );

  let status = line_for(&scan.residue, "unknown-status")
    .expect("`status: Banana` in a LIVE thread is unknown-status residue");
  let conflict = line_for(&scan.residue, "conflict-markers")
    .expect("conflict markers in a LIVE thread's work package are conflict-markers residue");

  // migration.md: `residue: <file>:<line> -- <class> -- <detail>`.
  assert!(
    status.starts_with("residue: intent/st/ST0001/info.md -- unknown-status -- "),
    "the classed line, with no line number to print: {status:?}"
  );
  assert!(
    conflict.starts_with(&format!(
      "residue: intent/st/ST0002/WP/01/info.md:{marker_line} -- conflict-markers -- "
    )),
    "the classed line WITH its line number -- the optional segment of the format: {conflict:?}"
  );

  // Two findings, two different classes, and neither line is the other's.
  assert_ne!(
    status, conflict,
    "two classes rendered identically, so the class is not discriminating"
  );
}

/// **THE VERB THAT WRITES, REFUSING, LEAVES THE WHOLE TREE BYTE-IDENTICAL.**
///
/// `upgrade` is the writer. Run against live residue it must refuse, and the
/// estate afterwards must equal the estate before **including every file this
/// test did not author** -- the config it did not touch, the markdown it wrote
/// once, and anything a partially-applied migration would have left behind.
///
/// **The store is inside this comparison and that is deliberate.** It is
/// gitignored, so a diff-based check of the same claim is structurally blind to
/// exactly the artefact D01-reversed calls authoritative; `tree` reads the
/// filesystem and does not care what git tracks.
#[test]
fn upgrade_refuses_live_residue_and_writes_nothing_at_all() {
  let fx = v2_estate();
  v2_thread(&fx, "ST0001", "Banana");
  v2_thread(&fx, "ST0002", "WIP");

  let before = tree(fx.root());
  assert!(
    !before.is_empty(),
    "the estate must be non-empty before, or the equality below holds trivially"
  );

  let outcome = Facade::upgrade(&fx.project(), &facade_ctx());
  assert!(
    outcome.is_err(),
    "live residue must block the migration, not be migrated around"
  );

  let moved = changed(&before, &tree(fx.root()));

  assert!(
    moved.is_empty(),
    "a refused migration wrote to the estate -- {moved:?}"
  );
}

/// **THE CONTROL, AND WITHOUT IT THE ARM ABOVE IS A TAUTOLOGY.**
///
/// *The tree did not change* is equally what an `upgrade` that refuses
/// everything produces. The same defect, moved into a CLOSED thread, carries
/// under hv's ruling -- so the migration must run **and the tree must change**.
///
/// Asserted as a CHANGE rather than as `is_ok()`: a verb can return `Ok` having
/// done nothing, and this thread has already met one that returned `Ok` having
/// done far too much.
#[test]
fn the_same_defect_in_a_closed_thread_carries_and_the_migration_does_run() {
  let fx = v2_estate();
  v2_thread(&fx, "ST0001", "Banana");
  fx.write_file(
    "intent/st/ST0001/info.md",
    &fx
      .read("intent/st/ST0001/info.md")
      .replace("status: Banana", "status: Completed"),
  );
  v2_thread(&fx, "ST0002", "WIP");

  // The defect now lives in a thread the ruling says CONVERTS: a closed thread
  // whose work package carries conflict markers.
  v2_wp_with_conflict(&fx, "ST0001");

  let scan = legacy::scan(&fx.project()).expect("a v2 estate scans");
  assert!(
    line_for(&scan.carried, "conflict-markers").is_some(),
    "the closed thread's finding must land in the CARRIED bucket: residue={:?} carried={:?}",
    scan.residue,
    scan.carried
  );
  assert!(
    line_for(&scan.residue, "conflict-markers").is_none(),
    "and not in the blocking one, or the split is not being applied"
  );

  let before = tree(fx.root());
  Facade::upgrade(&fx.project(), &facade_ctx()).expect("carried findings do not block");
  let after = tree(fx.root());

  assert_ne!(
    before, after,
    "the migration reported success over a carried finding and changed nothing, so the \
     byte-identity asserted above is a property of this fixture rather than of a refusal"
  );
}
