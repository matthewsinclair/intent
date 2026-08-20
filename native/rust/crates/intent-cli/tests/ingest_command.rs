//! `intent ingest [PATH]` -- Phase A of the v2 migration (migration.md, WP-10).
//!
//! **This file used to assert a refusal, and the refusal is gone.** The verb
//! was scaffolding: every path through it ended in "not available in this
//! build", and these tests pinned the SHAPE of that -- which project it
//! resolved, and whether the refusal told the truth about what it had done to
//! it. The parser has since landed, so the two tests asserting the refusal were
//! asserting a defect, and they are replaced rather than adjusted. A test that
//! keeps passing by describing what a command used to do is worse than none: it
//! reads like coverage of the thing that replaced it.
//!
//! What Phase A promises is narrow and worth pinning exactly: it READS an
//! estate and writes nothing, it distinguishes residue that blocks from legacy
//! that carries, and it never reports an absent field as a wrong one.
//!
//! # This file is **AT-10.15**, AC-10.2's second covering row, and it carried no contract id
//! for three days
//!
//! Two of AC-10.2's four limbs -- BLOCKED, and a non-zero exit -- have been
//! driven here end to end through the shipped verb since 2026-08-17, while the
//! row read `to-write` and a second test was written in `intentsvcs` for the
//! same criterion. **Nothing linked the two**: this file named no `AC-` or
//! `AT-` id anywhere, so no instrument this estate owns could see that the
//! criterion was already half covered. `at lint`'s L2 catches a row citing an
//! absent file and L3 catches a cited file missing the row's id; **neither can
//! see a covering test that names nothing.**
//!
//! **The split is deliberate and is not one row widened** (vc, 2026-08-20).
//! `intentsvcs`' `migrate_refusal.rs` cites the FORMAT, because `Finding`'s
//! `Display` is where the format lives. This file cites the TERMINAL: that the
//! class survives out of the library, through the renderer, to the operator's
//! stdout. Two citations keep two assertions separately falsifiable; one wider
//! row makes a green mean less.

use std::process::Command;

fn run(args: &[&str], cwd: &std::path::Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn project(dir: &std::path::Path) {
  std::fs::create_dir_all(dir.join("intent/.config")).expect("mkdir");
  std::fs::write(
    dir.join("intent/.config/config.json"),
    "{\"intent_version\":\"3.0.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  )
  .expect("write config");
}

/// Write a v2 steel thread the way v2.19 writes one.
fn v2_thread(dir: &std::path::Path, id: &str, status: &str) {
  let st = dir.join("intent/st").join(id);
  std::fs::create_dir_all(&st).expect("mkdir st");
  std::fs::write(
    st.join("info.md"),
    format!(
      "---\nverblock: \"14 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: {status}\nslug: a-slug\ncreated: 20260814\ncompleted:\n---\n\n# {id}: A thread\n\n## Objective\n\nShip it.\n\n## Context\n\nBecause.\n"
    ),
  )
  .expect("write info.md");
}

/// **Phase A reads, reports, and writes NOTHING** -- including no database.
///
/// The last clause is the one worth a test: this runs on a project that has not
/// been migrated, so creating a store as a side effect of inspecting it would
/// leave a v3 artefact inside a v2 estate the operator never asked to change,
/// on the one command whose whole promise is that it is safe to run.
#[test]
fn phase_a_reads_a_v2_estate_and_leaves_it_exactly_as_it_was() {
  let dir = tempfile::tempdir().expect("tempdir");
  project(dir.path());
  v2_thread(dir.path(), "ST0001", "WIP");
  v2_thread(dir.path(), "ST0002", "Completed");

  let before = std::fs::read_to_string(dir.path().join("intent/st/ST0001/info.md")).expect("read");

  let (_, err, code) = run(&["ingest"], dir.path());
  assert_eq!(code, 0, "a clean v2 estate parses: {err}");
  assert!(
    err.contains("2 thread(s)"),
    "it reports what it read: {err}"
  );
  assert!(
    err.contains("nothing was read into a store and nothing was written"),
    "and says what it did NOT do, because `ok` on a migration command reads as `migrated`: {err}"
  );

  assert_eq!(
    std::fs::read_to_string(dir.path().join("intent/st/ST0001/info.md")).expect("read"),
    before,
    "Phase A rewrote the estate it was only supposed to read"
  );
  assert!(
    !dir.path().join("intent/.cache/intent.db").exists(),
    "Phase A created a store on an unmigrated project"
  );
}

/// **Residue in a LIVE thread blocks; the same defect in a CLOSED thread
/// carries.**
///
/// hv's ruling, and the reason the two are separate buckets rather than one
/// severity field. Asserted with the SAME defect in both kinds of thread, so
/// the test cannot pass by treating everything alike.
#[test]
fn live_residue_blocks_and_closed_residue_carries() {
  let dir = tempfile::tempdir().expect("tempdir");
  project(dir.path());
  for (id, status) in [("ST0003", "Completed"), ("ST0004", "WIP")] {
    v2_thread(dir.path(), id, status);
    let info = dir.path().join("intent/st").join(id).join("info.md");
    let text = std::fs::read_to_string(&info).expect("read");
    std::fs::write(
      &info,
      text.replace(&format!("status: {status}"), "status: Banana"),
    )
    .expect("write");
  }

  let (out, err, code) = run(&["ingest"], dir.path());
  assert_eq!(code, 1, "live residue blocks the migration: {err}{out}");
  assert!(
    out.contains("ST0004"),
    "the blocking finding names the live thread: {out}"
  );
  assert!(
    out.contains("Banana"),
    "and the value it could not read: {out}"
  );
  // **THE CLASS, AND THIS IS THE LIMB ONLY THIS CRATE CAN ASSERT.** That
  // `Finding::Display` renders the class is `migrate_refusal.rs`'s subject.
  // Whether it survives to a TERMINAL is a claim about the renderer, and a CLI
  // that reformatted findings for display -- dropping the class, keeping the
  // file and the detail -- would pass every assertion above and every test in
  // `intentsvcs`. The delimiters are in the pattern for the reason they are
  // there: `contains("unknown-status")` also passes on a build that prints the
  // class in the detail field, or twice, or before the file.
  assert!(
    out.contains(" -- unknown-status -- "),
    "the classed line must reach the operator, not just `Finding::Display`: {out}"
  );
  assert!(
    err.contains("blocking"),
    "the totals distinguish the two buckets: {err}"
  );
  assert!(
    err.contains("v2 tooling"),
    "and the remedy names the FIXING ENVIRONMENT, which is v2 rather than this binary: {err}"
  );
}

/// **An absent field is not a wrong one**, and conflating them invents work.
///
/// The first run of this parser against Intent's own estate reported 20
/// findings and **19 were fields that had never been authored**, described as
/// values "not in the v2 vocabulary". Three closed threads predate the
/// work-package frontmatter convention entirely. Reporting those as defects
/// sends an operator to repair files their own tooling was content with, which
/// is the confident-from-partial-evidence habit v3 exists to end.
#[test]
fn a_field_that_was_never_recorded_is_not_reported_as_a_wrong_value() {
  let dir = tempfile::tempdir().expect("tempdir");
  project(dir.path());
  v2_thread(dir.path(), "ST0001", "Completed");
  // A work package in the older shape: no frontmatter at all.
  let wp = dir.path().join("intent/st/ST0001/WP/01");
  std::fs::create_dir_all(&wp).expect("mkdir wp");
  std::fs::write(
    wp.join("info.md"),
    "# WP01: An old work package\n\n## Scope\n\nAll of it.\n",
  )
  .expect("write wp");

  let (out, err, code) = run(&["ingest"], dir.path());
  assert_eq!(code, 0, "a pre-convention artefact does not block: {err}");
  assert!(
    out.contains("predates the frontmatter convention"),
    "it is reported as what it is: {out}"
  );
  assert!(
    !out.contains("is not in the v2 vocabulary"),
    "an absent field must not be described as a value v2 would have rejected: {out}"
  );
  // **How the report says "nothing to do" changed, and this assertion was left
  // behind asserting the old way.** It required the remedy line `nothing to
  // fix` to appear; then a carried finding stopped carrying a remedy at all,
  // on the ruling that residue owes a remedy and a carry does not. The
  // requirement is unchanged -- the operator must not be sent to fix this --
  // so it is now stated against the mechanism that carries it, and stated as
  // BOTH halves rather than one, because the header alone is satisfied by a
  // report that prints a remedy underneath it.
  assert!(
    out.contains("converts as-is, no action"),
    "the carried bucket says so in its own header: {out}"
  );
  assert!(
    !out.contains("remedy:"),
    "and nothing here owes a remedy, so none is offered: {out}"
  );
}

/// `[PATH]` selects the project, and its absence selects the one you are in.
#[test]
fn a_named_path_is_the_project_and_an_absent_one_means_the_project_you_are_in() {
  let outside = tempfile::tempdir().expect("tempdir");
  let estate = tempfile::tempdir().expect("tempdir");
  project(estate.path());
  v2_thread(estate.path(), "ST0001", "Completed");

  let (_, err, code) = run(&["ingest"], outside.path());
  assert_eq!(code, 1);
  assert!(
    err.contains("no Intent project") || err.contains("intent init"),
    "with no path and no project, the failure is about locating one: {err}"
  );

  let (_, err, code) = run(
    &["ingest", &estate.path().display().to_string()],
    outside.path(),
  );
  assert_eq!(code, 0, "a named path resolves from outside it: {err}");
  assert!(err.contains("1 thread(s)"), "and reads THAT estate: {err}");

  let (_, err, code) = run(
    &["ingest", &outside.path().display().to_string()],
    estate.path(),
  );
  assert_eq!(code, 1);
  assert!(
    err.contains("config.json") && err.contains("the root of an Intent project"),
    "a bad path is reported as a bad path, even standing inside a good project: {err}"
  );
}
