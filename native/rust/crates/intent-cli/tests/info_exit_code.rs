//! **`intent info` must not report success over a failure it printed.**
//!
//! Shipped 2026-08-17 as part of issue 0042's fix and found the same day by dc,
//! running a published-layout build: with `lib/templates/` absent the install
//! walk terminates, `info` prints `INTENT_HOME: <not set>`, names the reason on
//! stderr, and returns `Ok(())`. **dc's framing is the one this file is built
//! around: 0044 is `1` meaning five things; this was `0` meaning "I could not
//! do the thing you asked"** -- and it is the worse half, because a wrong
//! non-zero code stops a caller for the wrong reason and a zero on failure
//! stops nothing at all.
//!
//! **vc's 0044 sweep could not have found it, and the reason generalises to
//! this file's design.** That sweep's table classifies conditions BY exit code,
//! so a failure returning `0` lands in the success row by construction. The
//! question asked was "what code does each failure produce" and never "does any
//! failure produce success". **So every case here starts from a CONDITION and
//! asserts the code, never the other way round.**
//!
//! **The split under test, which is the whole subtlety: "never gate on PROJECT
//! state" is issue 0042's requirement and it is not "always exit 0".** An
//! unmigrated project is not a failure of `info`; an unresolvable install is.
//! Both directions are asserted here, because a fix in either direction alone
//! is a regression -- gating on project state re-opens 0042, and exiting 0 on a
//! broken install is what this file exists for.
//!
//! **The install is made unresolvable by MOVING THE BINARY, never by setting an
//! environment variable**, because `install::home()` reads no environment at
//! all -- it is a function of `current_exe()` and of nothing else. That is also
//! precisely dc's packaging hold reproduced: the Homebrew formula stages the
//! binaries and not `lib/templates/`, so a published build lives exactly where
//! this fixture puts it.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use testkit::workspace_root;

/// The Intent install root: the repository, which is the tree the real binary
/// is built inside and the one `lib/templates/` lives at the top of.
fn install_root() -> PathBuf {
  workspace_root()
    .parent()
    .and_then(Path::parent)
    .expect("the rust workspace sits two levels under the Intent install")
    .to_path_buf()
}

/// A copy of the binary with no Intent install above it, and a directory to run
/// it in. The temp root is the system temp dir, which has no `lib/templates/`
/// at any level -- and if it ever did, the first case below fails loudly rather
/// than passing vacuously.
fn staged_outside_any_install() -> (tempfile::TempDir, PathBuf) {
  let dir = tempfile::tempdir().expect("tempdir");
  let bin = dir.path().join("bin");
  std::fs::create_dir_all(&bin).expect("mkdir bin");
  let dest = bin.join("intent");
  std::fs::copy(env!("CARGO_BIN_EXE_intent"), &dest).expect("copy the binary out of its install");
  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).expect("chmod +x");
  }
  (dir, dest)
}

/// A v2 project: enough for `Project::discover` to find it and for the
/// migration to read as pending. Modelled on the estate this migration targets
/// rather than on a hypothetical.
fn unmigrated_project(at: &Path) -> PathBuf {
  let config = at.join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir .config");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"2.19.0\",\n  \"project_name\": \"Legacy\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"shell\"]\n}\n",
  )
  .expect("write config");
  at.to_path_buf()
}

/// A MIGRATED project whose store cannot be opened: the config reads as v3, so
/// the migration is `Done` and `info` goes on to open the store, and
/// `intent/.cache` is a regular file where a directory is required.
fn migrated_project(at: &Path) -> PathBuf {
  let intent = at.join("intent");
  std::fs::create_dir_all(intent.join(".config")).expect("mkdir .config");
  std::fs::write(
    intent.join(".config").join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Broken\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  std::fs::write(intent.join(".cache"), "not a directory\n").expect("write the blocking file");
  at.to_path_buf()
}

/// Run `intent info` and return (code, stdout, stderr).
fn info(exe: &Path, cwd: &Path) -> (Option<i32>, String, String) {
  let out = Command::new(exe)
    .arg("info")
    .current_dir(cwd)
    .stdin(Stdio::null())
    .output()
    .unwrap_or_else(|e| panic!("run {} info in {}: {e}", exe.display(), cwd.display()));
  (
    out.status.code(),
    String::from_utf8_lossy(&out.stdout).to_string(),
    String::from_utf8_lossy(&out.stderr).to_string(),
  )
}

/// The value `info` prints when it could not resolve the install. v2's token,
/// kept so a consumer parsing the line sees something it already handles.
const NOT_SET: &str = "<not set>";

/// **The baseline, and it is not ceremony: without it every assertion below
/// passes on a command that fails unconditionally**, which is a worse defect
/// than the one being fixed -- `info` is on the pre-commit gate's path.
#[test]
fn a_resolvable_install_succeeds() {
  let root = install_root();
  let (code, stdout, stderr) = info(Path::new(env!("CARGO_BIN_EXE_intent")), &root);

  assert_eq!(
    code,
    Some(0),
    "`intent info` run from inside its own install must succeed. stdout:\n{stdout}\nstderr:\n{stderr}"
  );
  assert!(
    !stdout.contains(NOT_SET),
    "the install resolved to `{NOT_SET}` from inside the repository, so the fixture below proves nothing -- every case in this file would be measuring a broken \
     resolver rather than a moved binary:\n{stdout}"
  );
}

/// **The defect itself: a failure that printed must not report success.**
///
/// Run outside a project on purpose. That is the path the old code took a
/// mid-function `return Ok(())` on, so a fix that only checked at the bottom of
/// `info` would be correct for a project and unreachable here.
#[test]
fn an_unresolvable_install_fails_outside_a_project() {
  let (dir, exe) = staged_outside_any_install();
  let (code, stdout, stderr) = info(&exe, dir.path());

  assert!(
    stdout.contains(NOT_SET),
    "the fixture did not reproduce an unresolvable install -- something above {} is a valid Intent install, so this case is not testing what it claims:\n{stdout}",
    exe.display()
  );
  assert_ne!(
    code,
    Some(0),
    "`intent info` could not resolve its install, said so on stderr, and reported success. **A zero on failure stops no caller at all** -- and this path is the \
     one a published build takes, where `lib/templates/` is not staged beside the binary. stderr:\n{stderr}"
  );
}

/// **The other branch of the project block, and it is not redundant.**
///
/// The old shape returned early only when discovery FAILED, so these two cases
/// exercise structurally different paths to the exit. Covering one and not the
/// other is the shape D44's window hit in `views::render_all`: a rule enforced
/// on one of two paths is enforced on neither reliably, because the uncovered
/// one wins whenever it runs.
#[test]
fn an_unresolvable_install_fails_from_inside_a_project_too() {
  let (dir, exe) = staged_outside_any_install();
  let project = unmigrated_project(&dir.path().join("proj"));
  let (code, stdout, stderr) = info(&exe, &project);

  assert!(
    stdout.contains("Location:"),
    "the fixture is not being read as a project, so this case duplicates the one above instead of covering the other branch:\n{stdout}"
  );
  assert_ne!(
    code,
    Some(0),
    "an unresolvable install must fail on the path that runs to the end of the function, not only on the one that returned early. stdout:\n{stdout}\nstderr:\n{stderr}"
  );
}

/// **`2` is the one wrong non-zero answer**, and it is wrong for a stated
/// reason rather than by taste: every consumer that reads it treats it as
/// fail-open (0038's pre-commit gate) or as a refusal on the operator's behalf
/// (0043's `UserPromptSubmit` BLOCK). **A tool that cannot find its own install
/// is the case where a caller must NOT proceed**, so answering in the code that
/// means "carry on" is the defect with a different number.
///
/// Deliberately not asserting the exact value: 0044 may re-spell this whole
/// surface, and pinning `1` here would make that a test edit rather than a
/// decision. The safety property survives either way.
#[test]
fn the_failure_is_not_reported_in_the_code_consumers_read_as_fail_open() {
  let (dir, exe) = staged_outside_any_install();
  let (code, _stdout, stderr) = info(&exe, dir.path());

  assert_ne!(
    code,
    Some(2),
    "`intent info` answered an unresolvable install in the code the pre-commit gate reads as fail-open and `UserPromptSubmit` reads as BLOCK. Neither is what \
     'this build cannot find its own install' means. stderr:\n{stderr}"
  );
}

/// **The opposite regression, and it is the reason this fix is not simply
/// "return an error more often".**
///
/// Issue 0042: the whiteboard guards are resolved by parsing this command's
/// stdout, in projects that are unmigrated, half-migrated, or not projects at
/// all. **Project state must never reach the exit code.** All three project
/// conditions are driven with a RESOLVABLE install, so the only difference from
/// the failing cases above is the one thing that is allowed to gate.
///
/// **The third condition was added after the first two, and the gap it closed is
/// the reason to state the count.** `info_project` has three arms that report a
/// project-state failure -- no project, migration pending, and the store failing
/// to open -- and this test measured the first two while its NAME claimed all
/// project state. The unmeasured arm is the one most exposed: an unopenable store
/// surfaces a real error with a remedy, which reads exactly like something that
/// ought to be non-zero, and I started to make it non-zero before reading the
/// decision it would have reversed. A guard that names a universal property and
/// covers the two easy instances of it is worse than one that names its scope,
/// because the name is what the next reader trusts.
///
/// **Why the answer is still 0 there.** `info`'s exit code answers exactly one
/// question -- can this binary resolve its own installation -- and an unopenable
/// store is a fact about a project. Anything wanting "is this project usable"
/// should ask the command it actually wants, which gates on `Facade::open` and
/// answers 1. Recorded here because the tempting fix is local, defensible in
/// isolation, and would break the whiteboard guards in every project the day
/// `info` inherits the migration refusal.
#[test]
fn project_state_never_reaches_the_exit_code() {
  let exe = Path::new(env!("CARGO_BIN_EXE_intent"));
  let dir = tempfile::tempdir().expect("tempdir");

  let (code, stdout, stderr) = info(exe, dir.path());
  assert_eq!(
    code,
    Some(0),
    "not being in a project is not a failure of `info` -- v2 exits 0 here and the gate parses this output from arbitrary directories. stdout:\n{stdout}\nstderr:\n{stderr}"
  );
  assert!(
    stdout.contains("Not in an Intent project directory"),
    "and it must still say so:\n{stdout}"
  );

  let project = unmigrated_project(&dir.path().join("legacy"));
  let (code, stdout, stderr) = info(exe, &project);
  assert_eq!(
    code,
    Some(0),
    "an unmigrated project is the state the gate most needs `info` to work in -- it is what every consumer looks like the moment before it upgrades. \
     stdout:\n{stdout}\nstderr:\n{stderr}"
  );
  assert!(
    stdout.contains("Steel Threads:"),
    "and the migration is reported as content rather than as an exit code:\n{stdout}"
  );

  // **A MIGRATED project whose store cannot be opened.** Driven by putting a
  // regular FILE where `intent/.cache/` has to be a directory, so the failure is
  // ENOTDIR rather than a permission: chmod is ignored when the suite runs as
  // root, and a fixture that silently stops failing is how this arm would go
  // back to being unmeasured.
  let broken = migrated_project(&dir.path().join("nostore"));
  let (code, stdout, stderr) = info(exe, &broken);
  assert!(
    stdout.contains("unavailable"),
    "precondition: this fixture must actually reach the arm where the store fails to open, or the assertion below passes for the wrong reason. \
     stdout:\n{stdout}\nstderr:\n{stderr}"
  );
  assert_eq!(
    code,
    Some(0),
    "a store that will not open is PROJECT state, and `info`'s exit code answers only whether this binary can resolve its own install. It reads like something \
     that should be non-zero, which is exactly why it is measured. stdout:\n{stdout}\nstderr:\n{stderr}"
  );
}

/// **The gate's contract is on STDOUT and it survives the failure.**
///
/// `pre-commit.sh` parses `INTENT_HOME:` out of this command's stdout with
/// `sed`; it has no exit-code contract at all. So the fix must add a code
/// WITHOUT removing a line -- and the tempting shape (bail with `?` at the
/// point of failure) would have printed nothing at all, which is issue 0042
/// again with a better exit status.
#[test]
fn the_line_the_gate_parses_is_printed_even_when_the_command_fails() {
  let (dir, exe) = staged_outside_any_install();
  let out = Command::new("sh")
    .arg("-c")
    .arg(format!(
      "{} info 2>/dev/null | sed -n 's/^ *INTENT_HOME: *//p' | head -1",
      exe.display()
    ))
    .current_dir(dir.path())
    .output()
    .expect("run the gate's resolution expression");
  let resolved = String::from_utf8_lossy(&out.stdout).trim().to_string();

  assert_eq!(
    resolved, NOT_SET,
    "a failing `info` must still print the line the pre-commit gate builds the whiteboard guards from, carrying v2's `{NOT_SET}` token. An empty parse is what \
     0042 was, and the gate quotes this value back at the operator inside its fail-open message"
  );
}
