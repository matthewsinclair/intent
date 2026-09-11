//! **The gate is two files, and canon used to write only one of them.**
//!
//! `canon::apply` has always written the chain block into `pre-commit`. That
//! block is:
//!
//! ```sh
//! _intent_chain="$(git rev-parse --git-path hooks 2>/dev/null)/pre-commit.intent"
//! if [ -x "$_intent_chain" ]; then
//!   "$_intent_chain" "$@" || exit $?
//! fi
//! ```
//!
//! **`if` with no `else`.** So until `install_carrier` existed, the one verb
//! whose job is wiring the gate produced a reference to a file that nothing in
//! either tree ever wrote -- and the result was not an error. It was a commit
//! at rc=0, in a project whose every report said the gate was wired.
//!
//! Baize is the measured instance: `intent_version` 3.0.0, canon present, fully
//! ported, four whiteboard nodes, and a gate running nothing.
//!
//! # Why these arms and not "does apply write the file"
//!
//! A presence test passes on an implementation that writes the wrong bytes from
//! the wrong tree with the wrong mode. Each arm below pins a property that a
//! plausible implementation gets wrong, and every one of them was reachable:
//!
//! - **Source.** dc measured eleven estates carrying the FROZEN v2 tree's gate
//!   byte for byte, ten stamped the same day. The last fleet-wide install read
//!   from a tree nobody develops in. **The defect was what the installer read**,
//!   so an installer that reads from anywhere but the resolved install root
//!   reproduces today's state exactly, and at rc=0.
//! - **Mode.** `[ -x ]` is the test the block applies. Right bytes plus wrong
//!   mode is the silent skip with every byte in place, and it is precisely the
//!   state a `write_if_changed` short-circuit leaves untouched forever.
//! - **The hook's own mode**, for the same reason one file up: a `pre-commit`
//!   git will not execute is a gate that never runs, and the "block already
//!   present" report is what made that invisible.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use intentsvcs::canon;

fn home() -> std::path::PathBuf {
  testkit::repo_root()
}

/// The shim as it exists in this install -- the bytes any correct implementation
/// must produce.
fn shim_template() -> String {
  let p = home().join("lib/templates/hooks/pre-commit-shim.sh");
  std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("no shim template at {}: {e}", p.display()))
}

fn mode(path: &Path) -> u32 {
  std::fs::metadata(path)
    .unwrap_or_else(|e| panic!("stat {}: {e}", path.display()))
    .permissions()
    .mode()
}

/// One `apply` against a fixture project, with the hooks directory handed in.
fn apply(fx: &crate::common::Fixture, hooks: &Path) -> canon::Applied {
  let project = fx.project();
  canon::apply(
    fx.root(),
    &home(),
    project.config(),
    &crate::common::ctx(),
    Some(hooks),
    canon::Options::default(),
  )
  .expect("canon apply")
}

fn hooks_dir(fx: &crate::common::Fixture) -> std::path::PathBuf {
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  hooks
}

/// **THE ARM THAT MAKES EVERY OTHER ARM MEAN SOMETHING.**
///
/// It drives the chain block as a shell program with no carrier beside it and
/// shows the outcome is success and silence. Without this, the rest of the file
/// reads as "we write one more file"; with it, the file being written is the
/// difference between a gate and a decoration.
///
/// **Deliberately independent of `apply`.** It executes the block canon emits,
/// so it stays true about the fail-open even if `apply` is rewritten -- and it
/// would still pass on the code as it stood before this change, which is what a
/// negative control is for.
#[test]
fn the_block_alone_passes_every_commit_in_silence() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);

  let hook = hooks.join("pre-commit");
  std::fs::write(
    &hook,
    canon::insert_chain_block("").expect("an empty hook is written whole"),
  )
  .expect("write hook");
  std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod");

  assert!(
    !hooks.join("pre-commit.intent").exists(),
    "the control requires the carrier to be absent"
  );

  let out = std::process::Command::new("bash")
    .arg(&hook)
    .current_dir(fx.root())
    .output()
    .expect("run the hook");

  assert!(
    out.status.success(),
    "the chain block with no carrier must be shown to PASS -- that is the defect \
     being closed, and if this ever fails the premise of this file has changed"
  );
  assert!(
    out.stderr.is_empty(),
    "and to pass SILENTLY: a warning would at least be a symptom. got: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// The carrier lands, executable, and its bytes are the install root's own.
#[test]
fn apply_installs_the_carrier_from_the_resolved_install_root() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);

  apply(&fx, &hooks);

  let carrier = hooks.join("pre-commit.intent");
  assert!(
    carrier.is_file(),
    "the block that sources this file is written by the same function"
  );
  assert_eq!(
    std::fs::read_to_string(&carrier).expect("read carrier"),
    shim_template(),
    "the carrier must be the template from the install root the binary resolved. \
     Eleven estates already carry a gate copied from the frozen v2 tree -- an \
     installer reading from anywhere else reproduces that, correctly and quietly"
  );
  assert!(
    mode(&carrier) & 0o111 != 0,
    "`[ -x ]` is what the chain block tests; mode {:o} makes the gate a no-op",
    mode(&carrier)
  );
}

/// **THE MODE IS REPAIRED EVEN WHEN THE BYTES NEED NO WRITE.**
///
/// This is the arm that fails the moment `make_executable` is moved back inside
/// a `write_if_changed` short-circuit. The state it describes -- correct bytes,
/// mode 644 -- is self-perpetuating under that implementation: every future run
/// reports the carrier already canonical and never touches the bit that makes it
/// run.
#[test]
fn a_correct_carrier_that_is_not_executable_is_repaired() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let carrier = hooks.join("pre-commit.intent");

  std::fs::write(&carrier, shim_template()).expect("plant correct bytes");
  std::fs::set_permissions(&carrier, std::fs::Permissions::from_mode(0o644)).expect("chmod 644");

  let applied = apply(&fx, &hooks);

  assert!(
    applied.unchanged.iter().any(|p| p == &carrier),
    "the bytes matched, so this run must report it already canonical rather than \
     rewriting it: {applied:?}"
  );
  assert!(
    mode(&carrier) & 0o111 != 0,
    "and must still have fixed the mode -- bytes and mode are two properties and \
     only one of them was correct. got {:o}",
    mode(&carrier)
  );
}

/// The same defect one file up: a `pre-commit` already carrying the block, which
/// git will not execute.
///
/// `insert_chain_block` returns `None` here -- "already correct" -- and that
/// report is exactly what kept this state invisible.
#[test]
fn a_hook_already_carrying_the_block_is_made_executable() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let hook = hooks.join("pre-commit");

  std::fs::write(
    &hook,
    canon::insert_chain_block("").expect("a hook with the block"),
  )
  .expect("write hook");
  std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o644)).expect("chmod 644");

  apply(&fx, &hooks);

  assert!(
    mode(&hook) & 0o111 != 0,
    "a pre-commit git will not execute is a gate that never runs, whatever it \
     contains. got {:o}",
    mode(&hook)
  );
}

/// Run it twice, change nothing -- the property `claude upgrade`'s own doc
/// claims for the whole verb.
#[test]
fn a_second_run_reports_the_carrier_already_canonical() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let carrier = hooks.join("pre-commit.intent");

  let first = apply(&fx, &hooks);
  assert!(
    first.written.iter().any(|p| p == &carrier),
    "the first run writes it: {first:?}"
  );

  let second = apply(&fx, &hooks);
  assert!(
    second.unchanged.iter().any(|p| p == &carrier),
    "the second must report it unchanged, not write it again: {second:?}"
  );
  assert!(
    !second.written.iter().any(|p| p == &carrier),
    "and must not appear as written: {second:?}"
  );
}

/// A project with no repository gets no gate, and that is not a failure.
///
/// The caller passes `None`; canon must not invent a hooks directory, and must
/// not fail on the way past.
#[test]
fn no_repository_means_no_carrier_and_no_error() {
  let fx = crate::common::Fixture::new();
  let project = fx.project();
  let applied = canon::apply(
    fx.root(),
    &home(),
    project.config(),
    &crate::common::ctx(),
    None,
    canon::Options::default(),
  )
  .expect("a project without git is a supported shape, not an error");

  assert!(
    !applied
      .written
      .iter()
      .chain(applied.unchanged.iter())
      .any(|p| p.ends_with("pre-commit.intent")),
    "no repository, no carrier: {applied:?}"
  );
}

/// `--skip-settings` (issue `0143`): a project's own `.claude/settings.json` is
/// left byte-for-byte as it was, reported as skipped, and the rest of canon --
/// the gate included -- is still applied.
///
/// **THE FILE IS PLANTED WITH NON-CANON BYTES, SO THE ARM CAN FAIL.** Against
/// an absent or already-canonical file, "left alone" and "rewritten" would look
/// the same; the control run without the flag shows the same fixture DOES get
/// rewritten.
#[test]
fn skip_settings_leaves_the_settings_file_alone_and_says_so() {
  let project_settings = r#"{ "hooks": {}, "mine": true }"#;
  let run = |skip: bool| {
    let fx = crate::common::Fixture::new();
    fx.git_init();
    let hooks = hooks_dir(&fx);
    let settings = fx.root().join(".claude/settings.json");
    std::fs::create_dir_all(settings.parent().expect("parent")).expect("mkdir .claude");
    std::fs::write(&settings, project_settings).expect("plant settings");
    let project = fx.project();
    let applied = canon::apply(
      fx.root(),
      &home(),
      project.config(),
      &crate::common::ctx(),
      Some(&hooks),
      canon::Options {
        skip_settings: skip,
        ..Default::default()
      },
    )
    .expect("canon apply");
    let after = std::fs::read_to_string(&settings).expect("settings still readable");
    (applied, after, settings, hooks)
  };

  let (applied, after, settings, hooks) = run(true);
  assert_eq!(
    after, project_settings,
    "--skip-settings rewrote the project's settings.json"
  );
  assert_eq!(
    applied.skipped,
    vec![settings.clone()],
    "the skip must be reported: {applied:?}"
  );
  assert!(
    !applied
      .written
      .iter()
      .chain(applied.unchanged.iter())
      .any(|p| p == &settings),
    "a skipped file must not also report as written or unchanged: {applied:?}"
  );
  assert!(
    applied
      .written
      .iter()
      .any(|p| p == &hooks.join("pre-commit.intent")),
    "skipping the settings must not skip the rest of canon: {applied:?}"
  );

  let (control, after, settings, _) = run(false);
  assert_ne!(
    after, project_settings,
    "the control run must rewrite the planted file, or the arm above proves nothing"
  );
  assert!(
    control.skipped.is_empty(),
    "nothing is skipped without the flag: {control:?}"
  );
  assert!(
    control.written.iter().any(|p| p == &settings),
    "{control:?}"
  );
}

/// **REPORT MODE ANSWERS WHAT `--apply` WOULD DO, AND WRITES NOTHING** (issue
/// `0115`).
///
/// The dry run used to print canon's roster, identical on a stale estate and a
/// clean one. Three runs pin the replacement: a report on a fresh fixture names
/// what would be written and leaves the tree untouched; `--apply` then writes
/// EXACTLY that set; and a report on the now-canonical tree says nothing would
/// be written -- the answer the roster could never give.
#[test]
fn report_mode_answers_what_apply_would_do_and_writes_nothing() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let project = fx.project();
  let run = |report: bool| {
    canon::apply(
      fx.root(),
      &home(),
      project.config(),
      &crate::common::ctx(),
      Some(&hooks),
      canon::Options {
        report,
        ..Default::default()
      },
    )
    .expect("canon apply")
  };

  let report = run(true);
  assert!(
    !report.written.is_empty(),
    "a fresh fixture has canon to write, so the report must name it: {report:?}"
  );
  for path in &report.written {
    assert!(
      !path.exists(),
      "report mode wrote {} -- a dry run that writes is not a dry run",
      path.display()
    );
  }

  let applied = run(false);
  assert_eq!(
    applied.written, report.written,
    "the report and the write disagree about what changes"
  );

  let clean = run(true);
  assert!(
    clean.written.is_empty(),
    "on a canonical tree the report must say nothing would be written: {clean:?}"
  );
  // Seeded user-owned files read as `preserved` from here on, so the count
  // that matters is that the clean report EXAMINED the tree rather than
  // returning an empty verdict: an empty `written` from a report that looked
  // at nothing would pass the assertion above.
  assert!(
    !clean.unchanged.is_empty(),
    "a clean report must name what it found canonical, not return nothing: {clean:?}"
  );
}
