//! `intent ingest [PATH]` -- the WP-03 scaffolding for the markdown door that
//! WP-10's migrator plugs into.
//!
//! **What is testable now is the SHAPE, and it is worth pinning precisely
//! because the body is not here yet.** The parser lands in WP-10; until then
//! every path through this verb ends in a refusal, so the properties that can
//! go wrong are which project it resolved, and whether the refusal tells the
//! truth about what it did to that project.

use std::process::Command;

fn run(args: &[&str], cwd: &std::path::Path) -> (String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run the v3 binary");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
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

/// **The refusal says what did NOT happen, and says nothing about repairing
/// anything.**
///
/// This is the assertion with teeth, because the first wiring failed it in a
/// way that would have cost someone work. `ingest::from_md` expressed "not
/// built yet" as a `Refusal` carrying a `FindingClass::UnknownFileShape`
/// finding, so the estate-facing remedy for that class -- "move or rename it"
/// -- was printed at a user whose only mistake was running an unimplemented
/// command. It named `intent/st`.
#[test]
fn the_refusal_does_not_send_anyone_to_repair_a_project_that_is_fine() {
  let dir = tempfile::tempdir().expect("tempdir");
  project(dir.path());

  let (text, code) = run(&["ingest"], dir.path());
  assert_eq!(code, 1, "an unavailable operation fails: {text}");
  assert!(
    text.contains("not available in this build"),
    "it says what is unavailable: {text}"
  );
  assert!(
    text.contains("nothing was read and nothing was written"),
    "and leads its remedy with what did not happen, which is the reader's actual question: {text}"
  );
  for wrong in [
    "move or rename",
    "fix the artefacts",
    "could not read the committed canon",
    "intent doctor",
  ] {
    assert!(
      !text.contains(wrong),
      "an unbuilt feature must not be reported as a damaged estate -- {wrong:?} in: {text}"
    );
  }
}

/// `[PATH]` selects the project, and its absence selects the one you are in.
///
/// Asserted by the DIFFERENCE rather than by either alone: both cases refuse
/// with the same words today, so a test that only checked the message would
/// pass on an arm that ignored the argument entirely. What distinguishes them
/// is which project fails to resolve.
#[test]
fn a_named_path_is_the_project_and_an_absent_one_means_the_project_you_are_in() {
  let outside = tempfile::tempdir().expect("tempdir");
  let estate = tempfile::tempdir().expect("tempdir");
  project(estate.path());

  // Standing OUTSIDE any project, with no path: there is nothing to resolve.
  let (text, code) = run(&["ingest"], outside.path());
  assert_eq!(code, 1);
  assert!(
    text.contains("no Intent project") || text.contains("intent init"),
    "with no path and no project, the failure is about locating one: {text}"
  );

  // The same invocation, NAMING the estate: it resolves, and the refusal is
  // the operation's rather than the lookup's.
  let (text, code) = run(
    &["ingest", &estate.path().display().to_string()],
    outside.path(),
  );
  assert_eq!(code, 1);
  assert!(
    text.contains("not available in this build"),
    "a named path resolves from outside it -- the migrator's whole case: {text}"
  );

  // A path that is not a project names the path, not the working directory.
  let (text, code) = run(
    &["ingest", &outside.path().display().to_string()],
    estate.path(),
  );
  assert_eq!(code, 1);
  assert!(
    text.contains("config.json") && text.contains("the root of an Intent project"),
    "a bad path is reported as a bad path, even standing inside a good project: {text}"
  );
}
