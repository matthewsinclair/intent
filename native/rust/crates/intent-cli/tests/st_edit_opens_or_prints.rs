//! **`st edit` OPENS THE FILE ON A TERMINAL AND PRINTS ITS PATH INTO A PIPE**,
//! with `--editor` and `--path` forcing either branch.
//!
//! hv ruled it on 2026-08-29 from three options, superseding two earlier
//! rulings on the same question. The overrides are the addition and they exist
//! for a stated cost: a bare TTY test makes behaviour depend on an invisible
//! property of the environment, so a wrapper, a CI job or an editor plugin gets
//! a different result with nothing in the command saying why.
//!
//! # The regression this file was asked to carry, and why it is not the one below
//!
//! The change was specified as owing one test: `$EDITOR "$(intent st edit
//! ST0001 info)"` must still return the path, because it is in
//! `docs/getting-started.md`. **That invocation does not work at HEAD and has
//! not since `c4709d3f`, for reasons that have nothing to do with this change.**
//! `info.md` is a GENERATED VIEW, and `Facade::edit` refuses a generated view
//! before it does anything else -- `intent/st/ST0001/info.md is generated from
//! the model, so an edit here is lost at the next render`. The register's
//! default for the `file` argument is `info`, so the bare `intent st edit
//! ST0001` refuses too, and `facade.rs` says so in passing inside a comment
//! about a different bug.
//!
//! **Written as specified, the regression test would have failed and read as
//! this change breaking it.** So the arm below drives the same PROPERTY -- a
//! captured stdout receives the path and nothing opens -- through `design`,
//! which is a file this verb can actually hand back. The documentation defect
//! is real and is filed separately; it is not this file's subject and it must
//! not be laundered into a green here.
//!
//! # What the pty proves that no ordinary test can
//!
//! The default branch is decided by `IsTerminal` on STDOUT, and every ordinary
//! test captures stdout -- so a test harness sees the pipe branch and can say
//! nothing at all about the terminal one. `common::pty_pair` puts a real
//! terminal there. **The reader is its own control:** the `--path`-on-a-terminal
//! arm asserts the path DOES come back through the same drain that the launch
//! arm asserts is empty, so an empty read cannot be mistaken for a broken
//! reader.

mod common;

use std::path::Path;
use std::process::{Command, Output, Stdio};

fn intent(dir: &Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .env("HOME", testkit::fixture_home())
    .env_remove("VISUAL")
    .env_remove("EDITOR")
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary")
}

/// A project whose only thread CARRIES `design.md` as an attachment.
///
/// **THE ATTACHMENT IS THE POINT OF THE FIXTURE, NOT SCENERY.** On a thread
/// with none, every one of the five declared `file` values refuses: `info` and
/// `acceptance` are generated views, and `design`, `impl` and `tasks` are not
/// carried, so `hydrate` never realises them. A fixture without this line
/// tests the refusal path five times and the subject zero times.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  intent(dir.path(), &["init", "editproj"]);
  intent(dir.path(), &["st", "new", "A thread"]);
  let body = dir.path().join("seed-design.md");
  std::fs::write(&body, "# design\n").expect("write the seed body");
  let out = intent(
    dir.path(),
    &[
      "st",
      "attach",
      "ST0001",
      "design.md",
      "--from",
      body.to_str().expect("utf8 path"),
    ],
  );
  assert!(
    out.status.success(),
    "the fixture attachment must exist, or every arm below tests a refusal: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  dir
}

fn expected(dir: &Path) -> String {
  dir
    .path_canonical()
    .join("intent/st/ST0001/design.md")
    .display()
    .to_string()
}

/// `tempfile` hands back `/var/...` on Darwin and the tool reports
/// `/private/var/...`; comparing the two as strings fails on a difference
/// neither party is making a claim about.
trait Canonical {
  fn path_canonical(&self) -> std::path::PathBuf;
}
impl Canonical for Path {
  fn path_canonical(&self) -> std::path::PathBuf {
    self.canonicalize().unwrap_or_else(|_| self.to_path_buf())
  }
}

/// A recorder standing in for an editor. It writes its whole argv to a file
/// and exits 0, so an arm can assert BOTH that it ran and what it was handed.
fn fake_editor(dir: &Path, log: &Path) -> String {
  let script = dir.join("fake-editor");
  std::fs::write(
    &script,
    format!("#!/bin/sh\nprintf '%s\\n' \"$@\" > {}\n", log.display()),
  )
  .expect("write the fake editor");
  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))
      .expect("make the fake editor executable");
  }
  script.display().to_string()
}

// ==========================================================================
// THE PIPE BRANCH -- what every script and every command substitution sees
// ==========================================================================

/// **THE REGRESSION, DRIVEN ON A FILE THE VERB CAN ACTUALLY HAND BACK.**
/// Command substitution makes stdout a pipe, so the documented shape keeps
/// working: this is the reason the TTY form does not break the output contract
/// and the reason a `--editor`-only design was not needed.
#[test]
fn a_captured_stdout_still_receives_the_path_and_opens_nothing() {
  let dir = seeded();
  let log = dir.path().join("editor.log");
  let editor = fake_editor(dir.path(), &log);

  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "edit", "ST0001", "design"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .env_remove("VISUAL")
    .env("EDITOR", &editor)
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary");

  assert!(
    out.status.success(),
    "stderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert_eq!(
    String::from_utf8_lossy(&out.stdout).trim(),
    expected(dir.path()),
    "a captured stdout must receive the path, and it is what `$EDITOR \"$(intent st edit ...)\"` reads"
  );
  assert!(
    !log.exists(),
    "an editor was launched into a pipe. $EDITOR was set and reachable, so this is the branch \
     choosing wrongly rather than the launch failing"
  );
}

#[test]
fn path_forces_the_path_and_editor_forces_the_launch_off_a_terminal() {
  let dir = seeded();
  let log = dir.path().join("editor.log");
  let editor = fake_editor(dir.path(), &log);

  let out = intent(dir.path(), &["st", "edit", "ST0001", "design", "--path"]);
  assert!(out.status.success());
  assert_eq!(
    String::from_utf8_lossy(&out.stdout).trim(),
    expected(dir.path())
  );

  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "edit", "ST0001", "design", "--editor"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .env_remove("VISUAL")
    .env("EDITOR", &editor)
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary");
  assert!(
    out.status.success(),
    "stderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert_eq!(
    std::fs::read_to_string(&log)
      .expect("`--editor` did not launch anything off a terminal")
      .trim(),
    expected(dir.path()),
    "the editor was handed something other than the resolved path"
  );
}

// ==========================================================================
// THE TERMINAL BRANCH -- unreachable without a pty
// ==========================================================================

/// Run with a real terminal on STDOUT and return what the terminal received.
/// Run with a real terminal on STDOUT and return what the terminal received.
///
/// **THE TERMINAL IS READ CONCURRENTLY WITH THE RUN, AND READING IT AFTERWARDS
/// DOES NOT WORK.** A pty discards whatever is still unread in its buffer when
/// the last slave descriptor closes, so a drain that waits for the child to
/// exit returns an empty string for a run that printed perfectly. **The
/// `--path` arm below is what caught it** -- it asserts the path DOES come
/// back, so it failed loudly where the launch arm, which asserts the terminal
/// is EMPTY, had passed for the wrong reason. A control that cannot fail is
/// not a control, and this one earned its place by failing.
///
/// **THE `Command` IS SCOPED SO IT DROPS BEFORE THE READ, WHICH IS ALSO A
/// CORRECTNESS REQUIREMENT.** `Stdio::from(slave)` moves the descriptor into
/// the builder and the builder keeps it, so while the builder is alive the
/// parent still holds a slave open -- and a master whose slave is open never
/// reports end-of-stream. The first version of this file hung both terminal
/// arms with no process left running but the test binary itself.
fn on_a_terminal(dir: &Path, args: &[&str], editor: Option<&str>) -> (bool, String, String) {
  use std::io::Read;

  let (master, slave) = common::pty_pair();
  let mut child = {
    let mut command = Command::new(env!("CARGO_BIN_EXE_intent"));
    command
      .args(args)
      .current_dir(dir)
      .env("HOME", testkit::fixture_home())
      .env_remove("VISUAL")
      .env_remove("EDITOR")
      .stdin(Stdio::null())
      .stdout(Stdio::from(slave))
      .stderr(Stdio::piped());
    if let Some(editor) = editor {
      command.env("EDITOR", editor);
    }
    command
      .spawn()
      .expect("run the v3 binary against a terminal")
  };

  let reader = std::thread::spawn(move || common::drain(master));

  let mut stderr = String::new();
  if let Some(mut handle) = child.stderr.take() {
    handle
      .read_to_string(&mut stderr)
      .expect("read the child's stderr");
  }
  let status = child.wait().expect("wait for the child");
  let terminal = reader.join().expect("the terminal reader thread panicked");

  (status.success(), terminal, stderr)
}

/// **THE ARM THIS FILE EXISTS FOR.** No flag is passed; the terminal alone
/// decides, and the editor is launched with no `--editor` anywhere.
#[test]
fn a_terminal_on_stdout_opens_the_editor_with_no_flag_at_all() {
  let dir = seeded();
  let log = dir.path().join("editor.log");
  let editor = fake_editor(dir.path(), &log);

  let (ok, terminal, stderr) = on_a_terminal(
    dir.path(),
    &["st", "edit", "ST0001", "design"],
    Some(&editor),
  );

  assert!(ok, "stderr: {stderr}");
  assert_eq!(
    std::fs::read_to_string(&log)
      .expect(
        "nothing was launched with a terminal on stdout, which is the whole behaviour under test"
      )
      .trim(),
    expected(dir.path())
  );
  assert!(
    terminal.trim().is_empty(),
    "the path was printed to the terminal as well as opened: {terminal:?}"
  );
}

/// **THE OVERRIDE, AND THE CONTROL FOR THE ARM ABOVE IN ONE.** `--path` beats
/// the terminal -- and because the path comes back through the same reader the
/// previous arm asserts is empty, an empty read there cannot be a broken
/// reader quietly passing.
#[test]
fn path_beats_the_terminal_and_proves_the_reader_works() {
  let dir = seeded();
  let log = dir.path().join("editor.log");
  let editor = fake_editor(dir.path(), &log);

  let (ok, terminal, stderr) = on_a_terminal(
    dir.path(),
    &["st", "edit", "ST0001", "design", "--path"],
    Some(&editor),
  );

  assert!(ok, "stderr: {stderr}");
  assert_eq!(
    terminal.trim(),
    expected(dir.path()),
    "`--path` did not print to the terminal it was overriding"
  );
  assert!(
    !log.exists(),
    "`--path` opened an editor, so the override is inert on the branch it exists to override"
  );
}

// ==========================================================================
// REFUSALS
// ==========================================================================

#[test]
fn the_two_flags_together_are_refused_rather_than_ranked() {
  let dir = seeded();
  let out = intent(
    dir.path(),
    &["st", "edit", "ST0001", "design", "--editor", "--path"],
  );
  assert!(!out.status.success());
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(
    stderr.contains("--editor") && stderr.contains("--path"),
    "the refusal must name both flags, since which one would have won is exactly what the \
     caller is trying not to guess: {stderr:?}"
  );
  assert!(
    String::from_utf8_lossy(&out.stdout).trim().is_empty(),
    "a refused run printed a path anyway, which a command substitution would then use"
  );
}

#[test]
fn an_unset_editor_refuses_and_names_the_way_out() {
  let dir = seeded();
  let out = intent(dir.path(), &["st", "edit", "ST0001", "design", "--editor"]);
  assert!(!out.status.success());
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(
    stderr.contains("$VISUAL") && stderr.contains("$EDITOR"),
    "the refusal must name both variables it consulted: {stderr:?}"
  );
  assert!(
    stderr.contains("--path"),
    "a refusal that names no way forward is the thing this package exists not to build: {stderr:?}"
  );
}

/// **A SHELL ALIAS IS NOT EXECUTABLE BY ANY PROCESS**, and `EDITOR=e` is a
/// common shape. A bare not-found reads as a broken install; the refusal has
/// to say what kind of name is required or the operator has nowhere to go.
#[test]
fn an_editor_no_process_can_run_is_refused_by_name() {
  let dir = seeded();
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "edit", "ST0001", "design", "--editor"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .env_remove("VISUAL")
    .env("EDITOR", "an-editor-no-path-has")
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary");

  assert!(!out.status.success());
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(
    stderr.contains("an-editor-no-path-has"),
    "the refusal must name what it tried to run: {stderr:?}"
  );
  assert!(
    stderr.contains("alias"),
    "the refusal must name the case it is almost always caused by: {stderr:?}"
  );
}

/// `$VISUAL` before `$EDITOR` is the convention, and a caller who set both set
/// them meaning it. Driven with `$EDITOR` pointed at a program that FAILS, so
/// a run that consulted the wrong variable reds instead of passing quietly.
#[test]
fn visual_is_consulted_before_editor() {
  let dir = seeded();
  let log = dir.path().join("editor.log");
  let editor = fake_editor(dir.path(), &log);

  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "edit", "ST0001", "design", "--editor"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .env("VISUAL", &editor)
    .env("EDITOR", "/usr/bin/false")
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary");

  assert!(
    out.status.success(),
    "$EDITOR was consulted ahead of $VISUAL: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert_eq!(
    std::fs::read_to_string(&log)
      .expect("$VISUAL was not launched")
      .trim(),
    expected(dir.path())
  );
}

/// Several editors report an abandoned edit through the exit code, and a
/// caller chaining on this command would otherwise proceed as though the edit
/// had been made.
#[test]
fn a_failing_editor_is_reported_rather_than_swallowed() {
  let dir = seeded();
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "edit", "ST0001", "design", "--editor"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .env_remove("VISUAL")
    .env("EDITOR", "/usr/bin/false")
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary");

  assert!(!out.status.success());
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(
    stderr.contains("/usr/bin/false") && stderr.contains("exited with 1"),
    "the refusal must name the editor and its code: {stderr:?}"
  );
}
