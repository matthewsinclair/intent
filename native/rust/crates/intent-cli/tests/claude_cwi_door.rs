//! `intent claude start` reaches the MAAC launcher, and the door is the one
//! check on the way.
//!
//! **The defect these arms first existed for was reachability, not a missing
//! feature.** `intent/plugins/claude/bin/intent_claude_cwi` implemented the
//! verb and was measured working standalone under a v3 binary; the v3
//! `claude()` dispatch simply had no arm for it, so it answered `2` -- "a known
//! command that is not implemented yet" -- while `intent claude --help` listed
//! it and `dispatch-table.json` filed `claude start` under `shipped`.
//!
//! # It had a sibling until ST0069 AC-14.12, and the arms that drove it are gone
//!
//! `intent claude ws <verb> [wsid]` managed the whiteboard as FILES. WP-14 makes
//! a node a ROW, so the row is retired in `dispatch-table.json` and the
//! spelling never reaches the launcher: it is refused at spelling-match time at
//! rc=2 naming `intent wb register`. **That refusal is driven by
//! `retirement_is_enumerable`**, so nothing here re-drives it.
//!
//! # Since issue 0609 the DOOR resolves and the launcher composes
//!
//! The door finds the project through the binary's own discovery, so a bare
//! repository is refused here as at every other verb, and reads the node with
//! the facade's `board`, the call `wb show` makes, so an absent node meets the
//! facade's own refusal and an unmigrated one is found. It then runs the
//! launcher FROM the project root as `start <ws> <display name>`. The launcher
//! used to walk up for the project, parse a board's header and carry a bash
//! copy of the absent-node refusal; each arm below drives one half of the
//! boundary that replaced them.
//!
//! # Driving this verb is a SAFETY question, and both hazards are sealed
//!
//! `claude start` is a declared member of `populations.not_probed`, whose
//! recorded reason is that **invoked bare it launches a real Claude Code
//! session**. So every arm here:
//!
//! - **puts a fixture `claude` first on PATH**, which records its working
//!   directory and its arguments and exits 0, so no session is ever launched
//!   and the arm can read exactly what the launcher handed over;
//! - **runs in a scratch project under the fixture HOME**, so the repository's
//!   real board is never read.
//!
//! **An arm here that forgets either is not a slow test, it is a live session
//! or a read of the real board**, which is why they are named at the top rather
//! than left as an incidental env var at the call site.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The launcher, as it sits in the repository this test runs from.
fn real_script() -> PathBuf {
  testkit::repo_root().join("intent/plugins/claude/bin/intent_claude_cwi")
}

/// A tree shaped like an Intent install, holding a real binary and the real
/// launcher.
///
/// **A test whose subject depends on the target directory is measuring
/// something other than the code.** [`intentsvcs::install::home`] walks up from
/// `current_exe()` to the `lib/templates` marker, so whether
/// `CARGO_BIN_EXE_intent` resolves to a real install is a fact about where
/// `CARGO_TARGET_DIR` points. Copying the binary into a tree this file controls
/// makes the install a property of the fixture rather than of the caller.
fn fixture_install() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  crate::common::fake_install(dir.path());
  crate::common::seed_plugin(
    dir.path(),
    "intent/plugins/claude/bin/intent_claude_cwi",
    &real_script(),
  );
  dir
}

/// A fixture `claude`: records its physical working directory and every
/// argument, NUL-separated because the composed context spans many lines, and
/// exits 0.
struct Claude {
  dir: tempfile::TempDir,
}

impl Claude {
  fn new() -> Self {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().expect("tempdir");
    let bin = dir.path().join("claude");
    std::fs::write(
      &bin,
      format!(
        "#!/bin/bash\nprintf '%s\\0' \"$(pwd -P)\" \"$@\" > '{}'\n",
        dir.path().join("argv").display()
      ),
    )
    .expect("write the fixture claude");
    std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    Self { dir }
  }

  /// PATH with this fixture first, so the launcher's `exec claude` finds it.
  fn path(&self) -> OsString {
    let mut path = OsString::from(self.dir.path());
    path.push(":");
    path.push(std::env::var_os("PATH").unwrap_or_default());
    path
  }

  /// What the launcher handed over, working directory first; `None` when
  /// `claude` never ran.
  fn invocation(&self) -> Option<Vec<String>> {
    let bytes = std::fs::read(self.dir.path().join("argv")).ok()?;
    let mut fields: Vec<String> = bytes
      .split(|b| *b == 0)
      .map(|f| String::from_utf8_lossy(f).into_owned())
      .collect();
    fields.pop();
    Some(fields)
  }
}

/// Run the fixture install's binary from `cwd`, under the fixture HOME, with
/// the fixture `claude` first on PATH.
fn run(exe: &Path, args: &[&str], cwd: &Path, claude: &Claude) -> (String, i32) {
  let out = testkit::fixtured_command(exe)
    .args(args)
    .current_dir(cwd)
    .env("PATH", claude.path())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the binary");
  let text = format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  (text, out.status.code().unwrap_or(-1))
}

/// A scratch project with `cc` registered through the verb, so its board is the
/// model's from the start.
fn project(exe: &Path, claude: &Claude) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (text, code) = run(exe, &["init", "cwi-fixture"], dir.path(), claude);
  assert_eq!(code, 0, "init: {text}");
  let (text, code) = run(
    exe,
    &[
      "wb",
      "register",
      "cc",
      "--name",
      "Control Claude",
      "--role",
      "control",
    ],
    dir.path(),
    claude,
  );
  assert_eq!(code, 0, "register: {text}");
  dir
}

/// The success path for a migrated node, launched from BELOW the project root.
///
/// The working directory the fixture `claude` records is the door's answer to
/// "where is the project": a launcher that still searched, or a door that
/// passed the caller's directory through, would leave it at `intent/`. The
/// context names the node by the display name the store holds, which only the
/// door's read can have supplied now that the launcher reads no board.
#[test]
fn start_runs_the_launcher_from_the_root_with_the_display_name() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let claude = Claude::new();
  let project = project(&exe, &claude);

  let (text, code) = run(
    &exe,
    &["claude", "start", "cc"],
    &project.path().join("intent"),
    &claude,
  );
  assert_eq!(code, 0, "start should launch: {text}");
  let argv = claude
    .invocation()
    .unwrap_or_else(|| panic!("the fixture claude never ran: {text}"));
  let root = std::fs::canonicalize(project.path()).expect("canonical root");
  assert_eq!(
    argv.first().map(String::as_str),
    root.to_str(),
    "claude did not run from the project root"
  );
  let session = format!(
    "{}-cc",
    root
      .file_name()
      .expect("a root has a name")
      .to_string_lossy()
      .to_lowercase()
  );
  assert!(
    argv.windows(2).any(|w| w[0] == "--name" && w[1] == session),
    "no session name `{session}` was composed: {argv:?}"
  );
  let context = argv
    .windows(2)
    .find(|w| w[0] == "--append-system-prompt")
    .map(|w| w[1].clone())
    .unwrap_or_else(|| panic!("no composed context was passed: {argv:?}"));
  assert!(
    context.contains("YOU ARE: cc (Control Claude)"),
    "the context does not carry the stored display name: {context}"
  );
  assert_eq!(argv.last().map(String::as_str), Some("/in-session"));
}

/// The success path for a node registered from its hand-authored header and not
/// yet migrated: `board` reads it as registered, so the door launches it.
#[test]
fn an_unmigrated_node_is_launched() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let claude = Claude::new();
  let project = project(&exe, &claude);
  let board = project.path().join("intent/whiteboard/dc");
  std::fs::create_dir_all(&board).expect("board dir");
  std::fs::write(
    board.join("wip.md"),
    "---\nnode: dc\nname: DevX Claude\nrole: worker\nsession_id: none\n\
     heartbeat_at: 2026-01-01 00:00Z\nstatus: active\nfocus: \"a fixture node\"\n\
     claims: []\n---\n# DevX Claude (dc)\n",
  )
  .expect("write the hand-authored board");
  let (text, code) = run(&exe, &["wb", "register"], project.path(), &claude);
  assert_eq!(code, 0, "register from the header: {text}");
  // The fixture is what it says: the node is on the roster and NOT migrated.
  let (shown, code) = run(
    &exe,
    &["wb", "show", "dc", "--json"],
    project.path(),
    &claude,
  );
  assert_eq!(code, 0, "show: {shown}");
  assert!(
    !shown.contains("migrated_at"),
    "the fixture node was migrated, so this arm would not test an unmigrated one: {shown}"
  );

  let (text, code) = run(&exe, &["claude", "start", "dc"], project.path(), &claude);
  assert_eq!(code, 0, "an unmigrated node should launch: {text}");
  let argv = claude
    .invocation()
    .unwrap_or_else(|| panic!("the fixture claude never ran: {text}"));
  assert!(
    argv.iter().any(|a| a.contains("YOU ARE: dc (DevX Claude)")),
    "the context does not carry the header's display name: {argv:?}"
  );
}

/// **An absent node is refused with the facade's own words, and they carry
/// issue 0413's ruling**: the command that registers one, an example of a
/// display name and a role so the session name is not the natural fill-in, and
/// the flag that repairs a node registered wrong. Since issue 0609 the launcher
/// carries no copy of this refusal, so these are the words every verb that
/// refuses an absent node prints.
#[test]
fn an_absent_node_is_refused_and_told_where_to_register_it() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let claude = Claude::new();
  let project = project(&exe, &claude);

  let (text, code) = run(&exe, &["claude", "start", "zz"], project.path(), &claude);
  assert_eq!(code, 1, "an absent node must be refused: {text}");
  assert!(
    text.contains("no node `zz` is registered"),
    "the refusal does not name the node that was asked for: {text}"
  );
  for shown in [
    intentsvcs::model::register_form("zz"),
    "--name \"Control Claude\" --role control".to_string(),
    "--correct".to_string(),
  ] {
    assert!(
      text.contains(&shown),
      "the remedy does not show `{shown}`: {text}"
    );
  }
  assert!(
    claude.invocation().is_none(),
    "an absent node reached the launch: {text}"
  );
}

/// Issue 0609's acceptance for this verb: in a bare repository holding an
/// `intent/` tree, `claude start` exits 1 before launching anything.
#[test]
fn a_bare_repository_is_refused_before_anything_launches() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let claude = Claude::new();
  let project = project(&exe, &claude);
  let status = std::process::Command::new("git")
    .args(["init", "-q", "--bare"])
    .arg(project.path())
    .status()
    .expect("run git");
  assert!(status.success(), "git init --bare failed");

  let (text, code) = run(&exe, &["claude", "start", "cc"], project.path(), &claude);
  assert_eq!(code, 1, "a bare repository must be refused: {text}");
  assert!(
    text.contains("is a bare git repository"),
    "the refusal does not say why: {text}"
  );
  assert!(
    claude.invocation().is_none(),
    "a bare repository reached the launch: {text}"
  );
}

/// An install missing its launcher refuses BY NAME rather than as an opaque
/// `127` from the shell, and says which subsystem is incomplete: `hook script`
/// and `whiteboard launcher` are different repairs.
#[test]
fn an_absent_launcher_refuses_by_name() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let launcher = install
    .path()
    .join("intent/plugins/claude/bin/intent_claude_cwi");
  let claude = Claude::new();
  let project = project(&exe, &claude);

  // Positive control FIRST: with the launcher present this same invocation
  // launches. Without it, a red arm below would be consistent with the fixture
  // simply being broken.
  let (before, code) = run(&exe, &["claude", "start", "cc"], project.path(), &claude);
  assert_eq!(code, 0, "the fixture install itself is broken: {before}");

  std::fs::remove_file(&launcher).expect("remove the launcher");
  let (text, code) = run(&exe, &["claude", "start", "cc"], project.path(), &claude);
  assert_ne!(code, 0, "an absent launcher must not succeed: {text}");
  assert!(
    text.contains("whiteboard launcher not found"),
    "the refusal does not name what is missing: {text}"
  );
  assert!(
    text.contains("intent_claude_cwi"),
    "the refusal does not name the path the operator must restore: {text}"
  );
}
