//! `intent export --format <fmt>` at the surface (AC-06.6).
//!
//! The round-trip property is guarded where it lives, in
//! `intentsvcs/tests/export_round_trip.rs`. What is asserted HERE is what the
//! operator meets: which stream the artefact goes to, what a refusal costs
//! them, and that a refused format leaves them holding nothing rather than
//! something partial.
//!
//! **The last one is the reason this file exists rather than being folded into
//! the service test.** `intent export --format md > estate.md` creates the file
//! whatever we do; whether it ends up empty or half-written is decided by
//! whether a single byte reached stdout before the refusal, and that is only
//! observable from out here.

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

/// A project with one real thread in it, made through the CLI rather than by
/// writing canon by hand -- so the estate being exported is one the product
/// produced.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::create_dir_all(dir.path().join("intent/.config")).expect("mkdir");
  std::fs::write(
    dir.path().join("intent/.config/config.json"),
    "{\"intent_version\":\"3.0.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  )
  .expect("write config");
  let (_, err, code) = run(&["st", "new", "no · 12:30 · a hazardous title"], dir.path());
  assert_eq!(code, 0, "fixture setup failed: {err}");
  dir
}

/// The bare command exports, to stdout, in the roster's default format.
#[test]
fn the_bare_command_writes_the_whole_estate_to_stdout_as_json() {
  let dir = project();
  let (out, err, code) = run(&["export"], dir.path());
  assert_eq!(code, 0, "export failed: {err}");

  let bundle: serde_json::Value = serde_json::from_str(&out)
    .unwrap_or_else(|e| panic!("the default format must be readable JSON: {e}\n{out}"));
  assert_eq!(
    bundle["schema"], "intent/export@3.0",
    "the artefact names its own schema, so a stranger can tell what they have"
  );
  assert!(
    bundle["threads"].as_array().is_some_and(|t| t.len() == 1),
    "the estate's one thread is in the bundle: {out}"
  );
  // The hazard survives to the artefact as the string it is. This is the whole
  // point of the format choice, checked at the surface rather than inferred.
  assert_eq!(
    bundle["threads"][0]["title"], "no · 12:30 · a hazardous title",
    "a title that a YAML reader would mangle comes through as text"
  );
  // Events are in, because history is the half nothing else can reconstruct.
  assert!(
    bundle["events"].as_array().is_some_and(|e| !e.is_empty()),
    "the bundle carries the event log: {out}"
  );
}

/// **A refused format leaves stdout completely empty.**
///
/// Not "mostly empty" and not "an error on stdout": a redirect is the ordinary
/// way to use this command, so any byte written before the refusal becomes a
/// file the operator now has to distrust. Every refusal below is checked for
/// it, because they take different paths through the code and only one of them
/// gets near the emitter.
#[test]
fn every_refusal_writes_nothing_to_stdout_and_says_why_on_stderr() {
  let dir = project();

  // A format that cannot be read back at all -- the view.
  let (out, err, code) = run(&["export", "--format", "md"], dir.path());
  assert_eq!(code, 1);
  assert!(
    out.is_empty(),
    "a refusal wrote {} bytes to stdout",
    out.len()
  );
  assert!(
    err.contains("`md`") && err.contains("generated VIEW"),
    "it names the format and says why: {err}"
  );
  assert!(
    err.contains("--format json"),
    "and where to go instead: {err}"
  );

  // A format that reads back perfectly for us and not for anyone else.
  let (out, err, code) = run(&["export", "--format", "yaml"], dir.path());
  assert_eq!(code, 1);
  assert!(
    out.is_empty(),
    "a refusal wrote {} bytes to stdout",
    out.len()
  );
  assert!(
    err.contains("`yaml`"),
    "the format is named, so the operator knows the spelling was understood: {err}"
  );
  assert!(
    err.contains("PyYAML"),
    "and refused on a measurement rather than on a preference: {err}"
  );

  // A format that does not exist.
  let (out, err, code) = run(&["export", "--format", "xml"], dir.path());
  assert_eq!(code, 1);
  assert!(out.is_empty());
  assert!(
    err.contains("no export format named `xml`"),
    "an unknown format is a different answer from a refused one: {err}"
  );
  // **What is offered and what is merely mentioned are different things.** The
  // first version of this message said "one of: json, yaml, md" -- two of them
  // refuse, so the remedy for a refusal was two more refusals. Asserted on the
  // rendered line because that is where the defect was visible and nowhere
  // else: the roster was correct, the listing of it was not.
  let offer = err
    .lines()
    .find(|l| l.contains("one of:"))
    .unwrap_or_else(|| panic!("no offer line: {err}"));
  let offer = &offer[offer.find("one of:").expect("checked above")..];
  let offer = offer.split('.').next().expect("first sentence");
  assert!(
    offer.contains("json"),
    "the offer names what works: {offer}"
  );
  for refused in ["yaml", "md"] {
    assert!(
      !offer.contains(refused),
      "`{refused}` refuses and is offered as the remedy for a refusal: {offer}"
    );
  }
  assert!(
    err.contains("yaml") && err.contains("md") && err.contains("refused"),
    "and the declined names are still reported, so the next guess is not one of them: {err}"
  );
}

/// **`--format md` and `--format xml` are DIFFERENT answers**, and collapsing
/// them is the failure this checks for.
///
/// "There is no such format" invites the operator to go and find the right
/// spelling. `md` is the right spelling; the answer is still no. A single
/// unknown-format message for both would send someone hunting for a name that
/// does not exist to be found.
#[test]
fn a_refused_format_is_not_reported_as_an_unknown_one() {
  let dir = project();
  let (_, refused, _) = run(&["export", "--format", "md"], dir.path());
  let (_, unknown, _) = run(&["export", "--format", "xml"], dir.path());
  assert_ne!(
    refused, unknown,
    "a format the roster carries and declines is not the same as one it has never heard of"
  );
  assert!(
    !refused.contains("no export format named"),
    "`md` IS an export format -- reporting it as absent is a lie the operator will act on: {refused}"
  );
}

/// Outside a project the failure is about locating one, not about formats.
///
/// A command that reported "no such format" here would send the operator to fix
/// their spelling while standing in the wrong directory.
#[test]
fn outside_a_project_the_failure_is_the_missing_project() {
  let outside = tempfile::tempdir().expect("tempdir");
  let (out, err, code) = run(&["export"], outside.path());
  assert_eq!(code, 1);
  assert!(out.is_empty());
  assert!(
    err.contains("intent init") || err.contains("Intent project"),
    "the failure names the real problem: {err}"
  );
}
