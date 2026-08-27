//! ST0061 **AT-00.1, covering AC-00.1**: `intent st dehydrate <ID>` is wired
//! and is the INVERSE of `intent st hydrate` -- proven by ROUND TRIP.
//!
//! **THE ROUND TRIP IS THE CRITERION'S OWN CHOICE OF INSTRUMENT AND THE REASON
//! IS WORTH KEEPING IN FRONT OF A READER**, in AC-00.1's words: it "tests the
//! two verbs against each other rather than each against my expectations of
//! it." Two separate assertions -- one that dehydrate removes what I think it
//! should, one that hydrate writes what I think it should -- can BOTH pass
//! while the pair fails to compose, because each is measured against the same
//! author's idea of the answer. Byte-identity after `hydrate -> dehydrate ->
//! hydrate` is measured against the estate's own earlier state, and no
//! expectation of mine is in it.
//!
//! # The degenerate pass, and the control that is here to catch it
//!
//! **A `dehydrate` THAT REMOVES NOTHING ROUND-TRIPS PERFECTLY.** Do nothing in
//! the middle step and the tree is trivially byte-identical at the end -- a
//! flawless score for an implementation that does not exist. The criterion as
//! written cannot see that, so the middle state is asserted directly:
//! `dehydrate` must have taken the `STEELTHREAD:` entry out of the manifest AND
//! taken the thread's files off the disk, both checked between the two
//! hydrations. **The round trip proves composition; only the middle assertion
//! proves there was anything to compose.**
//!
//! This is the general rule ST0057's AC-11.3 taught, arriving one criterion
//! later: a criterion can be satisfied perfectly by a degenerate
//! implementation, and the test that proves the criterion will say so. What a
//! criterion asks for is a floor, never a ceiling.
//!
//! **AND IT IS MEASURED HERE, NOT INHERITED AS A MAXIM.** With `dehydrate`
//! made to remove nothing (`Mode::Apply` -> `Mode::Preview`) AND the middle
//! file assertion deleted from this file, `hydrate_dehydrate_hydrate_returns_
//! the_tree_byte_for_byte` PASSES. The round trip alone certifies an inverse
//! that was never performed.
//!
//! # Every assertion here has been seen to fail
//!
//! Green on the first run is not evidence. Each of these was applied to the
//! source in a detached worktree, one at a time, and reverted:
//!
//! | mutation | what it breaks | what goes red |
//! |---|---|---|
//! | `Mode::Apply` -> `Mode::Preview` in `Facade::dehydrate` | removes nothing, still delists | the middle file assertion -- **and the report test, independently** |
//! | `if was_listed` -> `if false` | never delists | the middle manifest assertion |
//! | `set.add(path, after)` -> `String::new()` | wipes the whole manifest | the ST0001-untouched assertion |
//! | `pin(.., None)` -> `pin(.., Some("mutation"))` in `Facade::hydrate` | restores different bytes | the final byte-for-byte comparison |
//! | `&done.removed` -> `done.removed.iter().skip(1)` in `render.rs` | omits a removed path from the report | the report/disk disagreement assertion |
//!
//! **The first row is the one worth reading twice, and its second half was not
//! designed.** A do-nothing `dehydrate` is caught here TWICE -- by the middle
//! assertion, which exists for it, and by the report test, which refuses to
//! check a report of nothing (`nothing was removed, so there is no report to
//! check`). The second net was a side effect of deriving the expected set from
//! the disk rather than from the report; it is recorded because it is true, not
//! because it was clever.
//!
//! # What "the tree" means here, and the ONE exclusion, named rather than left
//!
//! Every file in the project is compared by its bytes **except `intent/.cache/`,
//! which is the store.** The store is the SSOT and it RECORDS the trip: a
//! `disk.dehydrate` event and a `disk.hydrate` event, measured on this fixture.
//! Requiring it to come back byte-identical would be requiring the tool to
//! forget it did the work, which is the opposite of what this estate wants from
//! its history. **Canon is NOT excluded** -- `intent/.canon/` is compared like
//! any other file, and it holding still across the trip is part of the evidence
//! that dehydration is a disk operation and never a loss.
//!
//! # The ship gate is met, never bypassed, and the fixture pays for it
//!
//! `intent st dehydrate` refuses in a project that declares no dehydration
//! preconditions -- so this fixture DECLARES one and SATISFIES it, through the
//! CLI like everything else here. Nothing in this file reaches around the gate,
//! for the reason `intentsvcs/tests/common` states about its own helper: a test
//! that could hand itself a permitting verdict would make the gate enforced by
//! everyone remembering not to call it.
//!
//! # Driven through the BINARY
//!
//! AC-00.1 opens with "is WIRED" and asks for each removed path to be REPORTED
//! by name -- both are claims about a command's surface, and a facade-level
//! test can make neither. `intentsvcs/tests/facade_dehydrate.rs` already covers
//! the facade's refusals in depth and **this file deliberately does not restate
//! any of them**; what it adds is the composition with `hydrate`, which nothing
//! else asserts.
//!
//! # A harness note, because this file is the THIRD copy
//!
//! `tree()` now appears here, in `organize_default_declaration.rs` and in
//! `upgrade_command.rs`, and 37 of this crate's 39 test files spell their own
//! binary runner. Rust's integration-test model gives each file its own crate,
//! so sharing needs a `tests/common/mod.rs` that does not exist here. **The
//! duplication is real and it is reported rather than fixed in passing**:
//! introducing a common module for one new file means migrating the two
//! existing users, in a working tree three other nodes are committing to, mid-
//! cut. That is a decision to take deliberately, not a side effect of adding a
//! test.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Output};

fn intent(dir: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    // stdin closed for the same reason `organize_default_declaration.rs` closes
    // it: inheriting the harness's stdin makes a result depend on whether cargo
    // was run from a terminal.
    .stdin(std::process::Stdio::null())
    .output()
    .expect("run the v3 binary")
}

fn ok(dir: &Path, args: &[&str]) -> String {
  let out = intent(dir, args);
  assert!(
    out.status.success(),
    "`intent {}` must succeed -- stdout: {}\nstderr: {}",
    args.join(" "),
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A project with the dehydration gate DECLARED and MET, and a second thread to
/// be the subject of the trip.
///
/// **The declaration is single, which the gate requires**: exactly one
/// criterion in the estate carries a `<<PRECONDITIONS ... PRECONDITIONS>>`
/// block, and two carriers anywhere would refuse.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "Fixture"]);
  ok(root, &["st", "new", "The gate declaration"]);
  ok(root, &["st", "new", "The round trip subject"]);
  ok(
    root,
    &[
      "ac",
      "new",
      "ST0001",
      "AC-00.1",
      "--text",
      "No dehydration path removes any file while any declared precondition is \
       unmet. <<PRECONDITIONS AC-00.9 PRECONDITIONS>>",
    ],
  );
  ok(
    root,
    &[
      "ac",
      "new",
      "ST0001",
      "AC-00.9",
      "--text",
      "a declared precondition",
    ],
  );
  ok(
    root,
    &[
      "ac",
      "satisfy",
      "ST0001",
      "AC-00.9",
      "--evidence",
      "met by construction in this fixture",
    ],
  );
  dir
}

/// Every file under the project with its bytes, **except the store**. See the
/// module note on the one exclusion.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
  fn walk(dir: &Path, root: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for e in entries.flatten() {
      let p = e.path();
      if p.is_dir() {
        walk(&p, root, out);
      } else if let Ok(bytes) = std::fs::read(&p) {
        let rel = p.strip_prefix(root).unwrap_or(&p).display().to_string();
        if rel.starts_with("intent/.cache") {
          continue;
        }
        out.insert(rel, bytes);
      }
    }
  }
  let mut out = BTreeMap::new();
  walk(root, root, &mut out);
  out
}

fn manifest(root: &Path) -> String {
  std::fs::read_to_string(root.join("intent/.intentfiles")).expect("manifest")
}

fn declares(root: &Path, id: &str) -> bool {
  manifest(root)
    .lines()
    .any(|l| l.trim() == format!("STEELTHREAD:{id}"))
}

// ---------------------------------------------------------------------------

/// **AC-00.1's round trip**, with the middle state asserted so a do-nothing
/// `dehydrate` cannot pass it.
#[test]
fn hydrate_dehydrate_hydrate_returns_the_tree_byte_for_byte() {
  let dir = project();
  let root = dir.path();

  ok(root, &["st", "hydrate", "ST0002"]);
  let before = tree(root);
  assert!(
    declares(root, "ST0002"),
    "the subject must be listed before the trip starts, or the trip has no subject"
  );
  assert!(
    before.keys().any(|p| p.starts_with("intent/st/ST0002/")),
    "the subject must be realised before the trip starts, or there is nothing to remove"
  );

  ok(root, &["st", "dehydrate", "ST0002"]);

  // **THE MIDDLE STATE, AND IT IS THE WHOLE DEFENCE AGAINST A DEGENERATE
  // PASS.** Both halves of "is the inverse of hydrate" are checked here --
  // the manifest entry and the files -- because a `dehydrate` that did
  // neither would round-trip perfectly at the end of this test.
  assert!(
    !declares(root, "ST0002"),
    "dehydrate left the STEELTHREAD entry in the manifest, so it removed nothing to restore"
  );
  let middle = tree(root);
  assert!(
    !middle.keys().any(|p| p.starts_with("intent/st/ST0002/")),
    "dehydrate left the thread's files on disk, so the round trip below proves nothing"
  );

  // The other thread's declaration and files are untouched: `dehydrate` was
  // handed ONE id. Without this, wiping the manifest wholesale would still
  // round-trip if `hydrate` happened to rebuild it.
  assert!(
    declares(root, "ST0001"),
    "dehydrating ST0002 delisted ST0001 as well"
  );

  ok(root, &["st", "hydrate", "ST0002"]);
  let after = tree(root);

  let changed: std::collections::BTreeSet<&String> = before
    .keys()
    .chain(after.keys())
    .filter(|p| before.get(*p) != after.get(*p))
    .collect();
  assert!(
    changed.is_empty(),
    "hydrate -> dehydrate -> hydrate did not return the tree byte for byte; these differ: {changed:?}"
  );
}

/// **AC-00.1's report clause**: each removed path by name, and the manifest it
/// changed.
///
/// **THE EXPECTED SET IS DERIVED FROM THE DISK, NOT FROM THE REPORT.** What
/// vanished is measured by differencing the tree either side of the run, and
/// the report is then checked against that -- so a report naming a file it did
/// not remove, or silently removing one it did not name, both fail. A test that
/// read the paths out of the report and looked for them would be asking the
/// report to confirm itself.
#[test]
fn dehydrate_reports_every_removed_path_and_the_manifest_it_changed() {
  let dir = project();
  let root = dir.path();

  ok(root, &["st", "hydrate", "ST0002"]);
  let before = tree(root);
  let stdout = ok(root, &["st", "dehydrate", "ST0002"]);
  let after = tree(root);

  let vanished: Vec<String> = before
    .keys()
    .filter(|p| !after.contains_key(*p))
    .cloned()
    .collect();
  assert!(
    !vanished.is_empty(),
    "nothing was removed, so there is no report to check -- stdout: {stdout}"
  );

  let named: Vec<String> = stdout
    .lines()
    .filter_map(|l| l.trim().strip_prefix("removed: "))
    .map(|p| p.to_string())
    .collect();
  assert_eq!(
    named, vanished,
    "the removed: lines and the files that actually vanished disagree -- stdout: {stdout}"
  );

  assert!(
    stdout
      .lines()
      .any(|l| l.trim() == "delisted: intent/.intentfiles"),
    "the run changed the manifest and did not name it -- stdout: {stdout}"
  );
}
