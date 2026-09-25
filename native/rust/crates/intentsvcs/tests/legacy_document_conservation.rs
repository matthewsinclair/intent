//! **A DOCUMENT IS A SECTION LIST PLUS AN ORDER**, and the legacy parser was
//! conserving only the first half.
//!
//! D28's shape is two fields -- `objective` and `body` -- so that a section the
//! v2 template never named survives the migration instead of being dropped by a
//! model with a fixed set of headings. That much worked. What did not is that
//! `legacy::sections` returned a `BTreeMap`, and the consumer rebuilds `body` by
//! joining it, so **the reassembled document came back in ALPHABETICAL order**.
//! Measured across this repository's own estate: every work package differs
//! from what its author wrote.
//!
//! **The reason it survived is the reason this file exists rather than another
//! assertion in an existing one.** Every section was present, so every
//! per-section check passed -- and the census that found it hashes each section
//! body independently, which is exactly the shape that cannot see a reordering.
//! Only a comparison at the level of the WHOLE DOCUMENT can, because the defect
//! is not in any section, it is in the sequence. The same fixture that proves a
//! section survives will pass under both implementations; nothing short of
//! asserting the order can separate them.
//!
//! **And the comment on the code was true.** It said a section the template
//! never named survives, which it does. It was silent about the order, and a
//! reader auditing the function against its own documentation got agreement --
//! including one who was about to copy the line into the thread parser, where it
//! would have closed a genuine hole while propagating this defect,
//! and made the section counts reconcile on the way.

use crate::common::Fixture;
use intentsvcs::legacy;
use intentsvcs::model::to_canonical_json;

/// The authored heading order, chosen so that **sorted and authored cannot
/// coincide**: it is very nearly reverse-alphabetical.
///
/// A fixture whose authored order happens to be alphabetical passes under the
/// defect and under the fix, which makes it not a weak test but an unbuilt one.
/// `sorted_differs_from_authored` below asserts that this list actually
/// discriminates, so the day someone renames a heading here and quietly makes
/// the two orders agree, that is a failure rather than a silent downgrade.
const AUTHORED: &[&str] = &["Objective", "Zebra", "Middle", "Deliverables", "Acceptance"];

/// Everything after `Objective` -- what `body` is required to carry, in order.
fn authored_body_order() -> Vec<&'static str> {
  AUTHORED
    .iter()
    .copied()
    .filter(|h| *h != "Objective")
    .collect()
}

fn v2_estate(fixture: &Fixture) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nverblock: \"17 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: Completed\nslug: a-slug\ncreated: 20260817\ncompleted: 20260817\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );
  let mut wp = String::from(
    "---\ntitle: A work package\nscope: Small\nstatus: Done\n---\n\n# WP01: A work package\n",
  );
  for heading in AUTHORED {
    wp.push_str(&format!("\n## {heading}\n\nThe body of {heading}.\n"));
  }
  fixture.write_file("intent/st/ST0001/WP/01/info.md", &wp);
}

/// The anti-vacuity half, and it comes first because the test after it is
/// conditional on it: if the authored order and the sorted order agree, the
/// assertion below cannot fail under the defect it was written for.
#[test]
fn sorted_differs_from_authored() {
  let authored = authored_body_order();
  let mut sorted = authored.clone();
  sorted.sort_unstable();
  assert_ne!(
    authored, sorted,
    "the fixture's headings are in alphabetical order, so it cannot tell a parser that \
     preserves authored order from one that sorts -- pick headings that differ"
  );
}

/// **The sections come back in the order they were written.**
#[test]
fn a_work_packages_sections_come_back_in_the_order_they_were_written() {
  let fixture = Fixture::new();
  v2_estate(&fixture);
  let scan = legacy::scan(&fixture.project()).expect("scan the v2 estate");

  let wp = &scan.threads[0].wps[0];
  assert_eq!(
    wp.objective, "The body of Objective.",
    "the objective is still lifted out of the document"
  );

  // The order the headings actually appear in the reassembled body, read by
  // position rather than by presence -- presence is what already passed.
  let found: Vec<&str> = wp
    .body
    .lines()
    .filter_map(|l| l.strip_prefix("## "))
    .collect();

  assert_eq!(
    found,
    authored_body_order(),
    "the reassembled document is not the authored one. Every section is present and the \
     SEQUENCE is wrong, which is why a per-section comparison reports this estate clean"
  );
}

/// **A repeated heading is two sections, not one.**
///
/// The map this replaced used `insert`, so a second `## Notes` overwrote the
/// first and the loss left no trace -- a separate conservation hole in the same
/// helper, found only because switching to a `Vec` for the ordering made it
/// impossible to reproduce.
#[test]
fn a_repeated_heading_is_not_swallowed_by_the_first_one() {
  let fixture = Fixture::new();
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: Completed\ncreated: 20260817\ncompleted: 20260817\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    "---\ntitle: A work package\nscope: Small\nstatus: Done\n---\n\n# WP01: A work package\n\n## Objective\n\nDo it.\n\n## Notes\n\nThe first note.\n\n## Notes\n\nThe second note.\n",
  );
  let scan = legacy::scan(&fixture.project()).expect("scan the v2 estate");

  let body = &scan.threads[0].wps[0].body;
  assert!(
    body.contains("The first note.") && body.contains("The second note."),
    "both bodies under a repeated heading survive; the map this replaced kept only the \
     last and said nothing: {body}"
  );
}

// ---------------------------------------------------------------------------
// The other direction: the parser must not read its own output back in
// ---------------------------------------------------------------------------

/// **A THREAD WITH COMMITTED CANON IS NOT RE-PARSED FROM MARKDOWN**, because the
/// markdown beside it is a generated view and parsing a view as a source is what
/// made the migration accrete without bound.
///
/// ic measured it against real v2 bytes: `ST0035/WP/01/info.md` went 8562 ->
/// 8840 -> 9190 -> 9540 over three runs, **monotonically, with no fixed point**,
/// while every run reported 0 blocking, 0 carried and 28 files planned. The D28
/// catch-all cannot tell the renderer's own `## Acceptance` and banner from
/// authored sections, so it absorbs them and the next render appends fresh ones
/// -- until the line reading "do not edit this file, it is rendered from the
/// model" is IN the model, in committed canon, several copies deep.
///
/// **This matters because of what hv ruled, not because it is untidy.** Big bang
/// and fix forward makes RE-RUNNING the recovery operation for a partial
/// migration, so the estate's only remedy was the thing that corrupted it.
///
/// **A heading-level discriminator cannot fix it and that is worth stating**:
/// `## Acceptance` is both a generated section and a legitimate authored v2 one
/// -- twelve threads in this repository carry the authored kind -- so there is no
/// list of names that separates them. The source of truth changes at migration,
/// and the parser has to follow it.
///
/// **The decoy is the load-bearing part of this test.** A matching thread count
/// proves nothing: the markdown could be re-parsed and happen to agree. So the
/// v2 markdown is REWRITTEN between the two scans to carry a marker that no
/// canon holds -- if the second scan's model contains it, the view was read.
#[test]
fn a_thread_with_canon_is_read_from_canon_and_its_view_is_not_parsed() {
  let fixture = Fixture::new();
  v2_estate(&fixture);

  let first = legacy::scan(&fixture.project()).expect("scan the v2 estate");
  assert_eq!(first.threads.len(), 1, "the v2 estate converts");
  assert!(
    first.already_migrated.is_empty(),
    "nothing is migrated yet: {:?}",
    first.already_migrated
  );

  // Phase B's write, reduced to the one artefact that matters here.
  let project = fixture.project();
  let canon = project.thread_json("ST0001");
  std::fs::create_dir_all(canon.parent().expect("parent")).expect("mkdir");
  std::fs::write(
    &canon,
    to_canonical_json(&first.threads[0]).expect("serialise"),
  )
  .expect("write canon");

  // The decoy: a generated-looking view where the v2 source used to be. It
  // carries a marker no canon holds, and the accreting sections that caused the
  // original defect.
  fixture.write_file(
    "intent/st/ST0001/WP/01/info.md",
    "---\ntitle: A work package\nscope: Small\nstatus: Done\n---\n\n# WP01: A work package\n\n## Objective\n\nThe body of Objective.\n\n## ZZ_DECOY_SECTION\n\nZZ_DECOY_MARKER\n\n## Acceptance\n\nAcceptance Criteria for this work package live in `ST0001/acceptance.md`.\n\n---\n\n_Generated by Intent. Do not edit this file._\n",
  );

  let second = legacy::scan(&fixture.project()).expect("re-scan the estate");

  assert_eq!(
    second.threads.len(),
    1,
    "**the thread is LOADED, not dropped** -- `steel_threads.md` and `todo.md` \
     render from the whole thread list, so a thread missing here is a thread \
     missing from the index, and the report would still read as success"
  );
  assert_eq!(
    second.already_migrated,
    vec!["ST0001".to_string()],
    "and the skip is NAMED, so `already_migrated + converted` still reconciles \
     against the estate"
  );

  let body = &second.threads[0].wps[0].body;
  assert!(
    !body.contains("ZZ_DECOY_MARKER"),
    "the generated view was parsed as a source -- this is the accretion, and one \
     more run would embed the banner too: {body}"
  );
  assert!(
    !body.contains("Do not edit this file"),
    "the banner that says the file is generated is now IN the model: {body}"
  );
}

/// **Canon that exists and will not read is an ERROR, not residue.**
///
/// It was residue in the first version of this, and the classification is the
/// point rather than the mechanism. Every class in the residue report describes
/// something a v2 AUTHOR left behind: it has a fix environment, a carry
/// disposition, and a work list an operator can act on. This describes canon
/// **the migration itself wrote**, so there is no v2 author to attribute it to,
/// no carry policy it could fall under, and nothing for an operator to fix in
/// their estate. Reporting it beside `unknown-scope` would put a broken migrator
/// in a table about broken estates.
///
/// There is a second reason and it is about keeping an instrument honest.
/// `residue_class_check.sh` enumerates the emitted classes by grepping
/// `legacy.rs` for `FindingClass::`, so a class constructed inside `ingest` and
/// passed through here would be emitted by the migration and invisible to the
/// check that verifies the contract declares what the migration emits. **A
/// laundered finding is a false green in someone else's instrument**, and
/// refusing rather than re-reporting is what avoids it.
///
/// What must NOT happen is the third option: a silent skip, leaving a thread
/// with unreadable canon absent from the model and absent from any report.
#[test]
fn canon_that_does_not_read_is_an_error_rather_than_residue_or_a_silent_skip() {
  let fixture = Fixture::new();
  v2_estate(&fixture);
  let project = fixture.project();
  let canon = project.thread_json("ST0001");
  std::fs::create_dir_all(canon.parent().expect("parent")).expect("mkdir");
  std::fs::write(&canon, "{ not json at all").expect("write canon");

  let err = legacy::scan(&fixture.project()).expect_err(
    "unreadable canon must refuse, not be skipped and not be reported as estate residue",
  );
  let text = err.to_string();
  assert!(
    text.contains("does not read as canon"),
    "the refusal says what is wrong: {text}"
  );
  assert!(
    text.contains("ST0001"),
    "and names the artefact, so it is actionable rather than a bare failure: {text}"
  );
}

/// **A migrated thread reached TWICE is loaded once, and the re-run does not
/// block on the ordinary shape of a re-run.**
///
/// The migration does not empty v2's status buckets, so afterwards a migrated
/// thread has a directory at `st/<ID>/` (written by the migration) and at
/// `st/COMPLETED/<ID>/` (left where v2 put it), and `thread_dirs` correctly
/// yields both. Both resolve the same canon.
///
/// **Without the guard, both pushed the same `Thread` and the 0011 duplicate-id
/// check then BLOCKED** -- so the first version of the accretion fix would have
/// made every re-run of a mostly-migrated estate refuse, which is precisely the
/// operation hv's fix-forward ruling depends on. Found by reading the walk rather
/// than by the suite, because no fixture had a thread in both places at once.
///
/// It does not weaken 0011: that class is two v2 artefacts claiming one id, and
/// neither of those has canon, so both still reach the markdown path and still
/// collide.
#[test]
fn a_migrated_thread_in_two_places_is_loaded_once_and_does_not_block() {
  let fixture = Fixture::new();
  v2_estate(&fixture);
  let project = fixture.project();

  // The same thread, also where v2's `st done` left it.
  fixture.write_file(
    "intent/st/COMPLETED/ST0001/info.md",
    "---\nstatus: Completed\ncreated: 20260817\ncompleted: 20260817\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );

  let first = legacy::scan(&project).expect("scan");
  assert_eq!(
    first.threads.len(),
    2,
    "before canon exists these ARE two artefacts claiming one id, and 0011 still sees them"
  );
  assert!(
    first
      .residue
      .iter()
      .any(|f| f.to_string().contains("appears 2 times")),
    "and it blocks, unmigrated: {:?}",
    first.residue
  );

  // Now migrate the flat one.
  let canon = project.thread_json("ST0001");
  std::fs::create_dir_all(canon.parent().expect("parent")).expect("mkdir");
  std::fs::write(
    &canon,
    to_canonical_json(&first.threads[0]).expect("serialise"),
  )
  .expect("write canon");

  let second = legacy::scan(&project).expect("re-scan");
  assert_eq!(
    second.threads.len(),
    1,
    "one id reached twice is ONE thread once canon exists"
  );
  assert_eq!(
    second.already_migrated,
    vec!["ST0001".to_string()],
    "named once, not twice -- `already_migrated + converted` has to reconcile"
  );
  assert!(
    second.residue.is_empty(),
    "**and the re-run does not block on the ordinary shape of a re-run**: {:?}",
    second.residue
  );
}

/// AT-02.2 / AC-02.1 -- ST0069 WP-02.
///
/// **The v2 `acceptance.md` preamble's AUTHORED lines survive,
/// and the template's boilerplate does not.**
///
/// Every line of that file which was not an `AC-`/`AT-` row fell on the floor
/// with no finding and no `Disposition` -- the LOST-PROSE shape
/// `Thread::preamble` was minted to close for `info.md`, still open one file
/// over. Measured on `ST0048/acceptance.md` at `baeae83a4` before it was built:
/// seven boilerplate lines, then one authored STATUS line INSIDE THE SAME
/// BLOCKQUOTE, nothing in the form separating them.
///
/// The fixture is that real shape: two lines byte-identical to the pinned
/// template, one line that DRIFTED from it, and one purely authored line. The
/// drifted line is carried, which is the ruled failure direction -- a stray
/// instruction sentence on a cover costs one deletion, a dropped authored line
/// is prose nobody knows is gone.
#[test]
fn the_acceptance_preambles_authored_lines_reach_the_thread_and_the_boilerplate_does_not() {
  // As v2 wrote it: `st new` put the thread's id where the template says
  // `ST####`, so the line to subtract carries the id.
  const BOILERPLATE_ONE: &str = "> Canonical acceptance contract for ST0001. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).";
  const BOILERPLATE_TWO: &str = "> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.";
  const DRIFTED: &str = "> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).";
  const AUTHORED: &str = "> STATUS: PROPOSED. ACs await hv ratification (the open-gate).";

  let fx = Fixture::new();
  fx.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fx.write_file(
    "intent/st/ST0001/info.md",
    "---\nstatus: Completed\nslug: a-slug\ncreated: 20260817\n---\n\n# ST0001: A thread\n\nThe cover's own preamble.\n\n## Objective\n\nShip it.\n",
  );
  fx.write_file(
    "intent/st/ST0001/acceptance.md",
    &format!(
      "---\nst_id: ST0001\n---\n\n# ST0001 A thread -- Acceptance\n\n{BOILERPLATE_ONE}\n>\n{BOILERPLATE_TWO}\n>\n{DRIFTED}\n>\n{AUTHORED}\n\n## Acceptance Criteria\n\n- AC-00.1 (non-test) A criterion. -- evidence: none -- satisfied: yes\n"
    ),
  );

  let scan = legacy::scan(&fx.project()).expect("the scan reads the estate");
  let thread = scan
    .threads
    .iter()
    .find(|t| t.id == "ST0001")
    .expect("the thread converted");

  assert!(
    thread.preamble.contains("The cover's own preamble."),
    "info.md's own preamble is still first: {:?}",
    thread.preamble
  );
  assert!(
    thread.preamble.contains(AUTHORED),
    "the authored line is carried: {:?}",
    thread.preamble
  );
  assert!(
    thread.preamble.contains(DRIFTED),
    "a boilerplate line that drifted from the pinned revision is CARRIED, not dropped: {:?}",
    thread.preamble
  );
  assert!(
    !thread.preamble.contains(BOILERPLATE_ONE) && !thread.preamble.contains(BOILERPLATE_TWO),
    "lines byte-identical to the pinned template are not the author's: {:?}",
    thread.preamble
  );

  let about: Vec<&legacy::Disposition> = scan
    .dispositions
    .iter()
    .filter(|d| d.heading == "the acceptance preamble")
    .collect();
  assert_eq!(
    about.len(),
    2,
    "both halves are recorded -- what was dropped and what was moved: {about:?}"
  );
  assert!(
    about.iter().any(|d| d.verdict == legacy::Verdict::Dropped)
      && about.iter().any(|d| d.verdict == legacy::Verdict::Refiled),
    "the drop is a judgement about the template; the carry is a MOVE into a file the author did not write it in: {about:?}"
  );
}
